"""Deterministic, no-Docker MCP protocol, trust-boundary and redaction unit tests."""

from __future__ import annotations

import io
import json
import queue
import unittest

from scripts.hive_mcp_probe import (
    EXPECTED_NAMES,
    MCPProbeBlocked,
    rpc_request,
    verify_call_result,
    verify_tool_list,
)


CORE = "a" * 40
PID = "00000000-0000-0000-0000-000000000001"


def readonly_tools() -> dict[str, object]:
    return {
        "tools": [
            {"name": name, "annotations": {"readOnlyHint": True}}
            for name in sorted(EXPECTED_NAMES)
        ]
    }


class HiveMCPProbeTests(unittest.TestCase):
    def test_exact_seven_read_only_tool_surface(self) -> None:
        verify_tool_list(readonly_tools())
        for mutated in (
            {"tools": readonly_tools()["tools"][:-1]},
            {"tools": [
                *readonly_tools()["tools"][:-1],
                {"name": "unknown.write", "annotations": {"readOnlyHint": True}},
            ]},
            {"tools": [
                *readonly_tools()["tools"][:-1],
                {"name": "checkpoint.read", "annotations": {"readOnlyHint": False}},
            ]},
        ):
            with self.assertRaises(MCPProbeBlocked):
                verify_tool_list(mutated)

    def test_checkpoint_same_exact_head_without_emitting_content(self) -> None:
        checkpoint = {
            "isError": False,
            "structuredContent": {
                "version": "mcp-checkpoint-read-v1",
                "project_id": PID,
                "checkpoint": {
                    "path": "docs/project-brain/13-CHECKPOINT.md",
                    "git_head_sha": CORE,
                    "registered_head_sha": CORE,
                    "content": "# Private user data that must never be printed",
                    "section_count": 3,
                },
            },
        }
        self.assertEqual(verify_call_result(checkpoint, name="checkpoint.read", core_head=CORE, project_id=PID), 3)
        checkpoint["structuredContent"]["checkpoint"]["git_head_sha"] = "b" * 40
        with self.assertRaisesRegex(MCPProbeBlocked, "mcp_checkpoint_not_current"):
            verify_call_result(checkpoint, name="checkpoint.read", core_head=CORE, project_id=PID)

    def test_context_search_bounded_and_project_bound(self) -> None:
        search = {
            "isError": False,
            "structuredContent": {
                "version": "mcp-context-search-v1",
                "project_id": PID,
                "top_k": 2,
                "results": [{"snippet": "private context"}],
            },
        }
        self.assertEqual(verify_call_result(search, name="context.search", core_head=CORE, project_id=PID), 1)
        search["structuredContent"]["project_id"] = "00000000-0000-0000-0000-000000000002"
        with self.assertRaisesRegex(MCPProbeBlocked, "mcp_tool_provenance_mismatch"):
            verify_call_result(search, name="context.search", core_head=CORE, project_id=PID)

    def test_rpc_rejects_remote_error_without_disclosure(self) -> None:
        inbox: queue.Queue[object] = queue.Queue()
        inbox.put({
            "jsonrpc": "2.0", "id": 1,
            "error": {"code": 500, "message": "secret machine path and UUID"},
        })
        wire = io.BytesIO()
        with self.assertRaisesRegex(MCPProbeBlocked, "^mcp_call_failed$"):
            rpc_request(wire, inbox, request_id=1, method="tools/list", timeout=0.1)
        self.assertEqual(json.loads(wire.getvalue()), {
            "jsonrpc": "2.0", "id": 1, "method": "tools/list",
        })

    def test_rpc_rejects_wrong_response_id(self) -> None:
        inbox: queue.Queue[object] = queue.Queue()
        inbox.put({"jsonrpc": "2.0", "id": 8, "result": {}})
        with self.assertRaisesRegex(MCPProbeBlocked, "mcp_unexpected_response_id"):
            rpc_request(io.BytesIO(), inbox, request_id=1, method="initialize", timeout=0.1)

    def test_rpc_timeout_fails_closed(self) -> None:
        with self.assertRaisesRegex(MCPProbeBlocked, "mcp_response_timeout_or_notification_flood"):
            rpc_request(io.BytesIO(), queue.Queue(), request_id=1, method="initialize", timeout=0.005)

    def test_tool_error_result_is_never_counted_as_proof(self) -> None:
        with self.assertRaisesRegex(MCPProbeBlocked, "mcp_tool_returned_error"):
            verify_call_result(
                {"isError": True, "structuredContent": {"project_id": PID}},
                name="checkpoint.read", core_head=CORE, project_id=PID,
            )


if __name__ == "__main__":
    unittest.main()
