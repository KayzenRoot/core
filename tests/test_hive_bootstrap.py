"""Deterministic HIVE bootstrap API/success/failure privacy tests; no Docker."""

from __future__ import annotations

import contextlib
import io
import json
import sys
import unittest
import urllib.error
from unittest.mock import patch

from scripts.hive_bootstrap import BootstrapBlocked, cli, main, request, resolve_registered_project

CORE_SHA = "c" * 40
PROJECT_ID = "00000000-0000-0000-0000-000000000001"
CANARY = "C:/Users/private/secret-auth-token"


def fake_request(_url: str, method: str, path: str, payload: dict | None = None) -> object:
    if path == "/api/v1/health":
        return {
            "status": "ok", "version": "1.0.0", "details": {"secret": CANARY},
            "checks": {"postgres": {"password": "do-not-print"}},
        }
    if path == "/api/v1/projects":
        return [{"name": "CORE", "relative_path": "core", "project_id": PROJECT_ID,
                 "private_path": CANARY}]
    if path == f"/api/v1/projects/{PROJECT_ID}/inspect":
        return {"relative_path": "core", "state": "READY", "git_head_sha": CORE_SHA,
                "private_path": CANARY}
    if path == f"/api/v1/projects/{PROJECT_ID}/index":
        return {"status": "COMPLETED", "private_path": CANARY}
    if path == f"/api/v1/projects/{PROJECT_ID}/retrieval/corpus/sync":
        return {"status": "CURRENT", "secret": CANARY}
    raise AssertionError("unexpected_api_route")


class HiveProjectResolutionTests(unittest.TestCase):
    def test_exact_relative_path_wins(self) -> None:
        projects = [
            {"project_id": "wrong", "name": "CORE", "relative_path": "archive/core"},
            {"project_id": "right", "name": "Renamed CORE", "relative_path": "core"},
        ]
        result = resolve_registered_project(projects, name="CORE", relative_path="core")
        self.assertIsNotNone(result)
        self.assertEqual(result["project_id"], "right")

    def test_same_name_different_path_fails_closed_and_does_not_leak(self) -> None:
        projects = [{"project_id": "other", "name": "CORE", "relative_path": CANARY}]
        with self.assertRaises(BootstrapBlocked) as ctx:
            resolve_registered_project(projects, name="CORE", relative_path="core")
        self.assertIn("different path", str(ctx.exception))
        self.assertNotIn(CANARY, str(ctx.exception))

    def test_missing_project_returns_none(self) -> None:
        projects = [{"project_id": "other", "name": "Widget", "relative_path": "widget"}]
        self.assertIsNone(resolve_registered_project(projects, name="CORE", relative_path="core"))

    def test_success_never_prints_health_paths_project_uuid_or_api_payloads(self) -> None:
        stdout = io.StringIO()
        calls: list[tuple[str, str]] = []
        def collect(url: str, method: str, path: str, payload: dict | None = None) -> object:
            calls.append((method, path))
            return fake_request(url, method, path, payload)
        with (
            patch.object(sys, "argv", ["hive_bootstrap.py", "--relative-path", "core"]),
            patch("scripts.hive_bootstrap.request", side_effect=collect),
            contextlib.redirect_stdout(stdout),
        ):
            self.assertEqual(main(), 0)
        output = stdout.getvalue()
        for secret in (CANARY, PROJECT_ID, "do-not-print", "private_path", "secret-auth-token"):
            self.assertNotIn(secret, output)
        self.assertIn(CORE_SHA, output)
        self.assertIn("PENDING_ACTUAL_CODEX_CLIENT_INVOCATIONS", output)
        self.assertEqual(
            calls,
            [
                ("GET", "/api/v1/health"),
                ("GET", "/api/v1/projects"),
                ("POST", f"/api/v1/projects/{PROJECT_ID}/inspect"),
                ("POST", f"/api/v1/projects/{PROJECT_ID}/index"),
                ("POST", f"/api/v1/projects/{PROJECT_ID}/retrieval/corpus/sync"),
            ],
        )

    def test_missing_project_preserves_stateful_registration_sequence(self) -> None:
        calls: list[tuple[str, str, object]] = []
        def new_project(url: str, method: str, path: str, payload: dict | None = None) -> object:
            calls.append((method, path, payload))
            if path == "/api/v1/projects" and method == "GET":
                return []
            if path == "/api/v1/projects" and method == "POST":
                return {"project_id": PROJECT_ID, "relative_path": "core"}
            return fake_request(url, method, path, payload)
        with (
            patch.object(sys, "argv", ["hive_bootstrap.py", "--relative-path", "core"]),
            patch("scripts.hive_bootstrap.request", side_effect=new_project),
            contextlib.redirect_stdout(io.StringIO()) as stdout,
        ):
            self.assertEqual(main(), 0)
        self.assertIn(("POST", "/api/v1/projects", {"name": "CORE", "relative_path": "core"}), calls)
        self.assertNotIn(PROJECT_ID, stdout.getvalue())

    def test_http_api_errors_never_emit_raw_body_or_endpoint(self) -> None:
        exc = urllib.error.HTTPError(
            "http://localhost:8000/api/v1/projects/private-id",
            403, "forbidden " + CANARY,
            hdrs=None, fp=io.BytesIO(("private-id " + CANARY).encode()),
        )
        with patch("scripts.hive_bootstrap.urllib.request.urlopen", side_effect=exc):
            with self.assertRaisesRegex(BootstrapBlocked, "^hive_http_status_403$") as ctx:
                request("http://localhost:8000", "GET", "/api/v1/projects/private-id")
        self.assertNotIn(CANARY, str(ctx.exception))
        self.assertNotIn("private-id", str(ctx.exception))

    def test_network_error_does_not_leak_server_reason(self) -> None:
        with patch(
            "scripts.hive_bootstrap.urllib.request.urlopen",
            side_effect=urllib.error.URLError("token=" + CANARY),
        ):
            with self.assertRaisesRegex(BootstrapBlocked, "^hive_api_unavailable$"):
                request("http://localhost:8000", "GET", "/api/v1/health")

    def test_cli_failure_is_typed_and_redacted(self) -> None:
        def error_api(url: str, method: str, path: str, payload: dict | None = None) -> object:
            if path.endswith("/inspect"):
                return {"state": "BLOCKED", "private": CANARY}
            return fake_request(url, method, path, payload)
        stderr, stdout = io.StringIO(), io.StringIO()
        with (
            patch.object(sys, "argv", ["hive_bootstrap.py", "--relative-path", "core"]),
            patch("scripts.hive_bootstrap.request", side_effect=error_api),
            contextlib.redirect_stdout(stdout),
            contextlib.redirect_stderr(stderr),
        ):
            self.assertEqual(cli(), 1)
        self.assertEqual(json.loads(stderr.getvalue())["reason"], "core_inspection_not_ready")
        self.assertNotIn(CANARY, stderr.getvalue() + stdout.getvalue())
        self.assertNotIn(PROJECT_ID, stderr.getvalue() + stdout.getvalue())

    def test_unexpected_exceptions_cannot_print_secret(self) -> None:
        stderr = io.StringIO()
        with (
            patch("scripts.hive_bootstrap.main", side_effect=RuntimeError(CANARY)),
            contextlib.redirect_stderr(stderr),
        ):
            self.assertEqual(cli(), 1)
        self.assertEqual(json.loads(stderr.getvalue())["reason"], "local_bootstrap_unavailable")
        self.assertNotIn(CANARY, stderr.getvalue())

    def test_invalid_cli_arguments_never_echo_user_private_paths(self) -> None:
        stderr, stdout = io.StringIO(), io.StringIO()
        with (
            patch.object(sys, "argv", ["hive_bootstrap.py", "--unknown-private-path", CANARY]),
            contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr),
        ):
            self.assertEqual(cli(), 1)
        self.assertEqual(json.loads(stderr.getvalue())["reason"], "invalid_cli_arguments")
        self.assertNotIn(CANARY, stderr.getvalue() + stdout.getvalue())
        self.assertNotIn("--unknown-private-path", stderr.getvalue() + stdout.getvalue())

    def test_invalid_api_head_is_never_echoed(self) -> None:
        def bad_head(url: str, method: str, path: str, payload: dict | None = None) -> object:
            if path.endswith("/inspect"):
                return {"relative_path": "core", "state": "READY", "git_head_sha": CANARY}
            return fake_request(url, method, path, payload)
        stderr, stdout = io.StringIO(), io.StringIO()
        with (
            patch.object(sys, "argv", ["hive_bootstrap.py", "--relative-path", "core"]),
            patch("scripts.hive_bootstrap.request", side_effect=bad_head),
            contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr),
        ):
            self.assertEqual(cli(), 1)
        self.assertEqual(json.loads(stderr.getvalue())["reason"], "core_inspection_head_invalid")
        self.assertNotIn(CANARY, stderr.getvalue() + stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
