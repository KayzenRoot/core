"""Stateful local HIVE v1.0.0 project preparation with redacted CLI diagnostics.

Actual runtime and separately connected Codex tool invocations must be proven
with the dedicated read-only local evidence collectors, never this bootstrap.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any


class BootstrapBlocked(RuntimeError):
    """Static or machine-independent reason; no raw API data or local paths."""


def request(base_url: str, method: str, path: str, payload: dict[str, Any] | None = None) -> Any:
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        base_url.rstrip("/") + path,
        data=data,
        method=method,
        headers={"Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(req, timeout=15) as response:
            raw = response.read()
            return json.loads(raw.decode("utf-8")) if raw else None
    except urllib.error.HTTPError as exc:
        # The HTTP body can contain UUIDs, absolute paths or credentials.
        raise BootstrapBlocked(f"hive_http_status_{int(exc.code)}") from None
    except urllib.error.URLError:
        # Never include request URL, host/path or a provider-supplied reason.
        raise BootstrapBlocked("hive_api_unavailable") from None


def resolve_registered_project(
    projects: list[dict[str, Any]], *, name: str, relative_path: str
) -> dict[str, Any] | None:
    exact = next(
        (item for item in projects if item.get("relative_path") == relative_path),
        None,
    )
    if exact is not None:
        return exact

    name_collisions = [
        item
        for item in projects
        if item.get("name") == name and item.get("relative_path") != relative_path
    ]
    if name_collisions:
        # This is also used by the redacted evidence/probe collectors.
        raise BootstrapBlocked("project name exists at a different path; resolve identity explicitly")
    return None


def _required_sha(value: object) -> str:
    if not isinstance(value, str) or re.fullmatch(r"[0-9a-fA-F]{40}", value) is None:
        raise BootstrapBlocked("core_inspection_head_invalid")
    return value.lower()


def main() -> int:
    parser = argparse.ArgumentParser(description="Register and prepare CORE in HIVE v1.0.0")
    parser.add_argument("--base-url", default=os.getenv("HIVE_API_URL", "http://localhost:8000"))
    parser.add_argument("--name", default="CORE")
    parser.add_argument(
        "--relative-path",
        default=os.getenv("HIVE_CORE_RELATIVE_PATH") or Path.cwd().name,
        help="POSIX-relative path below HIVE_PROJECTS_ROOT",
    )
    args = parser.parse_args()

    health = request(args.base_url, "GET", "/api/v1/health")
    if (
        not isinstance(health, dict)
        or health.get("status") != "ok"
        or health.get("version") != "1.0.0"
    ):
        raise BootstrapBlocked("hive_release_or_health_unavailable")
    print("HIVE health: ok (release 1.0.0)")

    projects = request(args.base_url, "GET", "/api/v1/projects")
    if not isinstance(projects, list) or any(not isinstance(item, dict) for item in projects):
        raise BootstrapBlocked("invalid_project_registry_response")

    target = resolve_registered_project(
        projects,
        name=args.name,
        relative_path=args.relative_path,
    )
    if target is None:
        target = request(
            args.base_url,
            "POST",
            "/api/v1/projects",
            {"name": args.name, "relative_path": args.relative_path},
        )
        print("Registered CORE in HIVE.")
    else:
        print("Resolved existing CORE registration by exact relative path.")

    if not isinstance(target, dict) or not isinstance(target.get("project_id"), str) or not target["project_id"]:
        raise BootstrapBlocked("project_identity_unavailable")
    project_id = target["project_id"]

    inspected = request(args.base_url, "POST", f"/api/v1/projects/{project_id}/inspect")
    if not isinstance(inspected, dict) or inspected.get("state") != "READY":
        raise BootstrapBlocked("core_inspection_not_ready")
    if inspected.get("relative_path") != args.relative_path:
        raise BootstrapBlocked("core_inspection_identity_mismatch")
    core_head = _required_sha(inspected.get("git_head_sha"))
    print("Inspection: READY", core_head)

    index = request(args.base_url, "POST", f"/api/v1/projects/{project_id}/index")
    if not isinstance(index, dict) or index.get("status") != "COMPLETED":
        raise BootstrapBlocked("core_index_incomplete")
    print("Repository index: COMPLETED")

    corpus = request(
        args.base_url,
        "POST",
        f"/api/v1/projects/{project_id}/retrieval/corpus/sync",
    )
    if not isinstance(corpus, dict) or corpus.get("status") not in {"COMPLETED", "CURRENT"}:
        raise BootstrapBlocked("core_retrieval_corpus_incomplete")
    print("Retrieval corpus:", corpus["status"])

    # Include only a validated Git SHA and static status enum values; never
    # print project UUID, relative/absolute path, API payloads or private data.
    print(
        json.dumps(
            {
                "schema": "CORE_HIVE_BOOTSTRAP_STATUS_V1",
                "hive_release": "v1.0.0",
                "core_git_head": core_head,
                "state": "READY",
                "index_status": "COMPLETED",
                "corpus_status": corpus["status"],
                "local_read_only_evidence": "PENDING_SEPARATE_COLLECTORS",
                "codex_client_proof": "PENDING_ACTUAL_CODEX_CLIENT_INVOCATIONS",
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


def cli() -> int:
    try:
        return main()
    except BootstrapBlocked as exc:
        # Only our own static reasons are permitted here.
        print(json.dumps({"status": "BLOCKED", "reason": str(exc)}), file=sys.stderr)
    except Exception:
        # API, JSON and OS errors may contain sensitive environment data.
        print(json.dumps({"status": "BLOCKED", "reason": "local_bootstrap_unavailable"}), file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(cli())
