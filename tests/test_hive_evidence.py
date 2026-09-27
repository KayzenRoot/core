"""No live HIVE or Docker dependency: typed, redacted evidence unit tests."""

from __future__ import annotations

import json
import unittest
from pathlib import Path
from unittest.mock import patch

from scripts.hive_evidence import EvidenceBlocked, collect_evidence


CORE_SHA = "c" * 40
HIVE_SHA = "a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf"


def fake_api(_url: str, method: str, path: str) -> object:
    if method != "GET":
        raise AssertionError("evidence helper must be read only")
    if path == "/api/v1/health":
        return {
            "status": "ok",
            "version": "1.0.0",
            "data_root": "C:/Users/secret/private",
            "checks": {
                "postgres": {"status": "ok", "details": {"password": "do-not-emit"}},
                "redis": {"status": "ok"},
                "storage": {"status": "ok"},
            },
        }
    if path == "/api/v1/projects":
        return [{"name": "CORE", "relative_path": "core", "project_id": "secret-project-id"}]
    if path == "/api/v1/projects/secret-project-id":
        return {
            "relative_path": "core",
            "state": "READY",
            "git_head_sha": CORE_SHA,
            "absolute_path": "C:/Users/private/core",
        }
    if path == "/api/v1/projects/secret-project-id/index/status":
        return {
            "status": "COMPLETED",
            "repository_head_sha": CORE_SHA,
            "run_id": "secret-index-run",
        }
    if path == "/api/v1/projects/secret-project-id/retrieval/corpus":
        return {
            "state": "CURRENT",
            "latest_run": {
                "status": "COMPLETED",
                "repository_index_run_id": "secret-index-run",
            },
        }
    raise AssertionError(f"unexpected API route {path}")


class HiveLocalEvidenceTests(unittest.TestCase):
    def collect(self) -> dict[str, object]:
        with (
            patch("scripts.hive_evidence.git_revision", side_effect=[CORE_SHA, HIVE_SHA, HIVE_SHA]),
            patch("scripts.hive_evidence.request", side_effect=fake_api),
        ):
            return collect_evidence(
                base_url="http://localhost:8000",
                relative_path="core",
                core_repo=Path("/private/core"),
                hive_repo=Path("/private/hive"),
            )

    def test_valid_evidence_is_redacted_and_marks_mcp_pending(self) -> None:
        evidence = self.collect()
        wire = json.dumps(evidence)
        self.assertEqual(evidence["core_git_head"], CORE_SHA)
        self.assertEqual(evidence["hive_release_commit"], HIVE_SHA)
        self.assertEqual(evidence["retrieval_corpus"], "CURRENT_SAME_INDEX")
        self.assertEqual(evidence["mcp_client_proof"], "PENDING_REAL_CODEX_TOOL_INVOCATIONS")
        for secret in ("secret-project-id", "secret-index-run", "do-not-emit", "/private", "C:/Users"):
            self.assertNotIn(secret, wire)

    def test_wrong_index_head_is_fail_closed(self) -> None:
        def stale_api(url: str, method: str, path: str) -> object:
            value = fake_api(url, method, path)
            if path.endswith("/index/status") and isinstance(value, dict):
                return {**value, "repository_head_sha": "f" * 40}
            return value

        with (
            patch("scripts.hive_evidence.git_revision", side_effect=[CORE_SHA, HIVE_SHA, HIVE_SHA]),
            patch("scripts.hive_evidence.request", side_effect=stale_api),
        ):
            with self.assertRaisesRegex(EvidenceBlocked, "repository_index_not_current"):
                collect_evidence(
                    base_url="http://localhost:8000",
                    relative_path="core",
                    core_repo=Path("/private/core"),
                    hive_repo=Path("/private/hive"),
                )

    def test_wrong_release_tag_is_fail_closed(self) -> None:
        with patch(
            "scripts.hive_evidence.git_revision",
            side_effect=[CORE_SHA, HIVE_SHA, "d" * 40],
        ):
            with self.assertRaisesRegex(EvidenceBlocked, "hive_release_checkout_mismatch"):
                collect_evidence(
                    base_url="http://localhost:8000",
                    relative_path="core",
                    core_repo=Path("/private/core"),
                    hive_repo=Path("/private/hive"),
                )

    def test_remote_service_is_rejected_before_git_or_api(self) -> None:
        with (
            patch("scripts.hive_evidence.git_revision") as git,
            patch("scripts.hive_evidence.request") as api,
        ):
            with self.assertRaisesRegex(EvidenceBlocked, "api_must_be_local_loopback"):
                collect_evidence(
                    base_url="https://example.org",
                    relative_path="core",
                    core_repo=Path("/private/core"),
                    hive_repo=Path("/private/hive"),
                )
            git.assert_not_called()
            api.assert_not_called()

    def test_stale_corpus_is_fail_closed(self) -> None:
        def stale_api(url: str, method: str, path: str) -> object:
            value = fake_api(url, method, path)
            if path.endswith("/retrieval/corpus") and isinstance(value, dict):
                return {**value, "state": "STALE"}
            return value

        with (
            patch("scripts.hive_evidence.git_revision", side_effect=[CORE_SHA, HIVE_SHA, HIVE_SHA]),
            patch("scripts.hive_evidence.request", side_effect=stale_api),
        ):
            with self.assertRaisesRegex(EvidenceBlocked, "retrieval_corpus_not_current"):
                collect_evidence(
                    base_url="http://localhost:8000",
                    relative_path="core",
                    core_repo=Path("/private/core"),
                    hive_repo=Path("/private/hive"),
                )


if __name__ == "__main__":
    unittest.main()
