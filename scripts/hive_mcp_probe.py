"""Read-only, bounded HIVE v1.0.0 MCP server round-trip with redacted output.

This validates the local MCP server process directly, NOT that the separately
configured Codex client has registered or invoked the server.
"""

from __future__ import annotations

import argparse
import json
import os
import queue
import subprocess
import sys
import threading
import time
from pathlib import Path
from typing import Any
from uuid import UUID

if __package__:
    from .hive_bootstrap import request, resolve_registered_project
    from .hive_evidence import collect_evidence, EvidenceBlocked
    from .hive_mcp import resolve_hive_repo
else:
    from hive_bootstrap import request, resolve_registered_project
    from hive_evidence import collect_evidence, EvidenceBlocked
    from hive_mcp import resolve_hive_repo


EXPECTED_NAMES = frozenset({
    "project.list", "project.status", "context.build", "context.search",
    "memory.search", "memory.get", "checkpoint.read",
})
MAX_RPC_LINE_BYTES = 8_000_000
RPC_TIMEOUT_SECONDS = 20
MAX_SERVER_MESSAGES = 16


class MCPProbeBlocked(RuntimeError):
    """Typed, privacy-preserving failure, never containing server payloads."""


def _receive_messages(stream: Any, inbox: queue.Queue[object]) -> None:
    """Bound each incoming JSON line and surface failures without logging data."""
    try:
        while True:
            line = stream.readline(MAX_RPC_LINE_BYTES + 1)
            if not line:
                inbox.put(MCPProbeBlocked("mcp_server_stream_closed"))
                return
            if len(line) > MAX_RPC_LINE_BYTES:
                inbox.put(MCPProbeBlocked("mcp_server_message_too_large"))
                return
            try:
                payload = json.loads(line)
            except (UnicodeDecodeError, ValueError):
                inbox.put(MCPProbeBlocked("mcp_invalid_json_response"))
                return
            if not isinstance(payload, dict):
                inbox.put(MCPProbeBlocked("mcp_invalid_response_shape"))
                return
            inbox.put(payload)
    except (OSError, ValueError):
        inbox.put(MCPProbeBlocked("mcp_server_stream_failed"))


def rpc_request(
    stdin: Any,
    inbox: queue.Queue[object],
    *,
    request_id: int,
    method: str,
    params: dict[str, Any] | None = None,
    timeout: float = RPC_TIMEOUT_SECONDS,
) -> dict[str, Any]:
    payload: dict[str, Any] = {"jsonrpc": "2.0", "id": request_id, "method": method}
    if params is not None:
        payload["params"] = params
    try:
        stdin.write((json.dumps(payload, separators=(",", ":")) + "\n").encode("utf-8"))
        stdin.flush()
    except (BrokenPipeError, OSError, ValueError) as exc:
        raise MCPProbeBlocked("mcp_request_stream_unavailable") from exc
    deadline = time.monotonic() + timeout
    for _ in range(MAX_SERVER_MESSAGES):
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            break
        try:
            value = inbox.get(timeout=remaining)
        except queue.Empty:
            break
        if isinstance(value, MCPProbeBlocked):
            raise value
        if not isinstance(value, dict) or value.get("jsonrpc") != "2.0":
            raise MCPProbeBlocked("mcp_protocol_response_invalid")
        if value.get("id") == request_id:
            if "error" in value:
                # Never leak remote error data or a captured project UUID.
                raise MCPProbeBlocked("mcp_call_failed")
            result = value.get("result")
            if not isinstance(result, dict):
                raise MCPProbeBlocked("mcp_response_missing_result")
            return result
        if "id" in value:
            raise MCPProbeBlocked("mcp_unexpected_response_id")
        if not isinstance(value.get("method"), str):
            raise MCPProbeBlocked("mcp_unexpected_notification")
    raise MCPProbeBlocked("mcp_response_timeout_or_notification_flood")


def verify_tool_list(result: dict[str, Any]) -> None:
    tools = result.get("tools")
    if not isinstance(tools, list) or len(tools) != len(EXPECTED_NAMES):
        raise MCPProbeBlocked("mcp_tool_surface_mismatch")
    names: set[str] = set()
    for tool in tools:
        if not isinstance(tool, dict) or not isinstance(tool.get("name"), str):
            raise MCPProbeBlocked("mcp_tool_surface_mismatch")
        annotation = tool.get("annotations")
        if not isinstance(annotation, dict) or annotation.get("readOnlyHint") is not True:
            raise MCPProbeBlocked("mcp_tool_not_read_only")
        names.add(tool["name"])
    if names != EXPECTED_NAMES:
        raise MCPProbeBlocked("mcp_tool_surface_mismatch")


def verify_call_result(result: dict[str, Any], *, name: str, core_head: str, project_id: str) -> int:
    if result.get("isError") is not False:
        raise MCPProbeBlocked("mcp_tool_returned_error")
    payload = result.get("structuredContent")
    if not isinstance(payload, dict) or payload.get("project_id") != project_id:
        raise MCPProbeBlocked("mcp_tool_provenance_mismatch")
    if name == "checkpoint.read":
        item = payload.get("checkpoint")
        if (
            payload.get("version") != "mcp-checkpoint-read-v1"
            or not isinstance(item, dict)
            or item.get("path") != "docs/project-brain/13-CHECKPOINT.md"
            or item.get("git_head_sha") != core_head
            or item.get("registered_head_sha") != core_head
            or not isinstance(item.get("content"), str)
            or not item["content"]
            or not isinstance(item.get("section_count"), int)
            or item["section_count"] < 1
        ):
            raise MCPProbeBlocked("mcp_checkpoint_not_current")
        return item["section_count"]
    entries = payload.get("results")
    if (
        payload.get("version") != "mcp-context-search-v1"
        or payload.get("top_k") != 2
        or not isinstance(entries, list)
        or not 1 <= len(entries) <= 2
    ):
        raise MCPProbeBlocked("mcp_context_search_not_current")
    return len(entries)


def probe_mcp_server(*, base_url: str, relative_path: str, hive_repo: Path, core_repo: Path) -> dict[str, object]:
    local = collect_evidence(
        base_url=base_url,
        relative_path=relative_path,
        core_repo=core_repo,
        hive_repo=hive_repo,
    )
    projects = request(base_url, "GET", "/api/v1/projects")
    if not isinstance(projects, list):
        raise MCPProbeBlocked("mcp_core_registry_unavailable")
    try:
        project = resolve_registered_project(projects, name="CORE", relative_path=relative_path)
    except RuntimeError as exc:
        raise MCPProbeBlocked("mcp_core_registry_ambiguous") from exc
    if not isinstance(project, dict):
        raise MCPProbeBlocked("mcp_core_registration_missing")
    try:
        project_id = str(UUID(str(project["project_id"])))
    except (KeyError, TypeError, ValueError, AttributeError) as exc:
        raise MCPProbeBlocked("mcp_project_id_invalid") from exc

    command = ["docker", "compose", "exec", "-T", "api", "python", "-m", "app.mcp_server"]
    try:
        child = subprocess.Popen(
            command,
            cwd=hive_repo,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            bufsize=0,
        )
    except (OSError, ValueError) as exc:
        raise MCPProbeBlocked("mcp_local_server_launch_failed") from exc
    try:
        if child.stdin is None or child.stdout is None:
            raise MCPProbeBlocked("mcp_server_pipes_unavailable")
        inbox: queue.Queue[object] = queue.Queue(maxsize=MAX_SERVER_MESSAGES + 1)
        reader = threading.Thread(
            target=_receive_messages, args=(child.stdout, inbox), daemon=True
        )
        reader.start()
        init = rpc_request(
            child.stdin,
            inbox,
            request_id=1,
            method="initialize",
            params={
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "core-hive-local-proof", "version": "1.0"},
            },
        )
        info = init.get("serverInfo")
        if (
            not isinstance(info, dict)
            or info.get("name") != "hive-mcp"
            or info.get("version") != "mcp-core-surface-v1"
            or not isinstance(init.get("protocolVersion"), str)
        ):
            raise MCPProbeBlocked("mcp_server_version_mismatch")
        child.stdin.write(b'{"jsonrpc":"2.0","method":"notifications/initialized"}\n')
        child.stdin.flush()
        verify_tool_list(
            rpc_request(child.stdin, inbox, request_id=2, method="tools/list", params={})
        )
        checkpoint_sections = verify_call_result(
            rpc_request(
                child.stdin, inbox, request_id=3, method="tools/call",
                params={"name": "checkpoint.read", "arguments": {"project_id": project_id}},
            ),
            name="checkpoint.read", core_head=str(local["core_git_head"]),
            project_id=project_id,
        )
        found = verify_call_result(
            rpc_request(
                child.stdin, inbox, request_id=4, method="tools/call",
                params={
                    "name": "context.search",
                    "arguments": {"project_id": project_id, "query": "CORE", "top_k": 2},
                },
            ),
            name="context.search", core_head=str(local["core_git_head"]),
            project_id=project_id,
        )
    finally:
        if child.stdin is not None:
            try:
                child.stdin.close()
            except OSError:
                pass
        try:
            child.wait(timeout=2)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=3)
        if child.stdout is not None:
            child.stdout.close()

    # Never print original MCP content, search snippets, project UUID,
    # complete local paths, Docker environment or the server error payload.
    return {
        "schema": "CORE_HIVE_MCP_SERVER_EVIDENCE_V1",
        "evidence_origin": "EXECUTED_ON_LOCAL_MACHINE",
        "hive_release_commit": local["hive_release_commit"],
        "core_git_head": local["core_git_head"],
        "server_name": "hive-mcp",
        "server_version": "mcp-core-surface-v1",
        "read_only_tool_count": len(EXPECTED_NAMES),
        "checkpoint_same_head": True,
        "checkpoint_section_count": checkpoint_sections,
        "context_search_top_k": 2,
        "context_search_returned": found,
        "codex_client_proof": "PENDING_ACTUAL_CODEX_CLIENT_INVOCATIONS",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Safely probe the real local HIVE MCP server")
    parser.add_argument("--base-url", default=os.getenv("HIVE_API_URL", "http://localhost:8000"))
    parser.add_argument("--relative-path", default=os.getenv("HIVE_CORE_RELATIVE_PATH", "core"))
    parser.add_argument("--hive-repo", type=Path)
    args = parser.parse_args()
    try:
        proof = probe_mcp_server(
            base_url=args.base_url,
            relative_path=args.relative_path,
            hive_repo=args.hive_repo or resolve_hive_repo(),
            core_repo=Path(__file__).resolve().parents[1],
        )
    except (EvidenceBlocked, MCPProbeBlocked) as exc:
        print(json.dumps({"status": "BLOCKED", "reason": str(exc)}))
        return 1
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError):
        print(json.dumps({"status": "BLOCKED", "reason": "local_mcp_dependency_unavailable"}))
        return 1
    print(json.dumps(proof, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
