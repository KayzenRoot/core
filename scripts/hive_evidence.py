"""Read-only, redacted evidence collection against the user's actual local HIVE v1.0.0.

This is a local evidence helper, never a substitute for an actual Codex MCP
invocation or an authorized claim about data on machines it cannot inspect.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlsplit

if __package__:
    from .hive_bootstrap import request, resolve_registered_project
    from .hive_mcp import resolve_hive_repo
else:
    from hive_bootstrap import request, resolve_registered_project
    from hive_mcp import resolve_hive_repo


RELEASE_TAG = "v1.0.0"
RELEASE_COMMIT = "a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf"


class EvidenceBlocked(RuntimeError):
    """A typed, disclosure-safe reason why local evidence is not admissible."""


def git_revision(checkout: Path, revision: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(checkout), "rev-parse", revision],
        check=True,
        capture_output=True,
        text=True,
        timeout=10,
    )
    sha = completed.stdout.strip().lower()
    if re.fullmatch(r"[0-9a-f]{40}", sha) is None:
        raise EvidenceBlocked("invalid_local_git_revision")
    return sha


def collect_evidence(
    *, base_url: str, relative_path: str, core_repo: Path, hive_repo: Path
) -> dict[str, object]:
    """Verify actual read-only API and local Git facts without returning secrets."""
    url = urlsplit(base_url)
    if url.scheme not in {"http", "https"} or url.hostname not in {
        "localhost",
        "127.0.0.1",
        "::1",
    }:
        raise EvidenceBlocked("api_must_be_local_loopback")
    if not relative_path or relative_path.startswith(("/", "\\")) or ".." in relative_path.split("/"):
        raise EvidenceBlocked("invalid_relative_core_path")

    core_head = git_revision(core_repo, "HEAD")
    hive_head = git_revision(hive_repo, "HEAD")
    tag_commit = git_revision(hive_repo, "v1.0.0^{commit}")
    if hive_head != RELEASE_COMMIT or tag_commit != RELEASE_COMMIT:
        raise EvidenceBlocked("hive_release_checkout_mismatch")

    health = request(base_url, "GET", "/api/v1/health")
    if not isinstance(health, dict) or health.get("status") != "ok":
        raise EvidenceBlocked("hive_not_healthy")
    if health.get("version") != "1.0.0":
        raise EvidenceBlocked("hive_api_version_mismatch")
    checks = health.get("checks")
    if not isinstance(checks, dict) or any(
        not isinstance(checks.get(name), dict) or checks[name].get("status") != "ok"
        for name in ("postgres", "redis", "storage")
    ):
        raise EvidenceBlocked("hive_dependencies_not_healthy")

    projects = request(base_url, "GET", "/api/v1/projects")
    if not isinstance(projects, list):
        raise EvidenceBlocked("invalid_project_registry_response")
    try:
        target = resolve_registered_project(
            projects, name="CORE", relative_path=relative_path
        )
    except RuntimeError as exc:
        raise EvidenceBlocked("ambiguous_project_identity") from exc
    if not isinstance(target, dict) or not target.get("project_id"):
        raise EvidenceBlocked("core_not_registered_run_bootstrap")
    project_id = str(target["project_id"])
    detail = request(base_url, "GET", f"/api/v1/projects/{project_id}")
    if (
        not isinstance(detail, dict)
        or detail.get("relative_path") != relative_path
        or detail.get("state") != "READY"
        or detail.get("git_head_sha") != core_head
    ):
        raise EvidenceBlocked("core_project_not_ready_or_head_mismatch")

    index = request(base_url, "GET", f"/api/v1/projects/{project_id}/index/status")
    if (
        not isinstance(index, dict)
        or index.get("status") != "COMPLETED"
        or index.get("repository_head_sha") != core_head
        or not index.get("run_id")
    ):
        raise EvidenceBlocked("repository_index_not_current")
    corpus = request(base_url, "GET", f"/api/v1/projects/{project_id}/retrieval/corpus")
    latest = corpus.get("latest_run") if isinstance(corpus, dict) else None
    if (
        not isinstance(corpus, dict)
        or corpus.get("state") != "CURRENT"
        or not isinstance(latest, dict)
        or latest.get("status") != "COMPLETED"
        or latest.get("repository_index_run_id") != index["run_id"]
    ):
        raise EvidenceBlocked("retrieval_corpus_not_current")

    # Deliberately omit project UUID, absolute paths, API raw payloads,
    # database details, environment, document snippets and any credentials.
    return {
        "schema": "CORE_HIVE_LOCAL_EVIDENCE_V1",
        "evidence_origin": "EXECUTED_ON_LOCAL_MACHINE",
        "hive_release": RELEASE_TAG,
        "hive_release_commit": hive_head,
        "hive_health": "ok",
        "hive_dependencies": "ok",
        "core_relative_path_matches": True,
        "core_git_head": core_head,
        "core_hive_registry_state": "READY",
        "repository_index": "COMPLETED_SAME_HEAD",
        "retrieval_corpus": "CURRENT_SAME_INDEX",
        "mcp_client_proof": "PENDING_REAL_CODEX_TOOL_INVOCATIONS",
    }


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Emit redacted local CORE/HIVE release, API, index and corpus proof"
    )
    parser.add_argument(
        "--base-url", default=os.getenv("HIVE_API_URL", "http://localhost:8000")
    )
    parser.add_argument(
        "--relative-path", default=os.getenv("HIVE_CORE_RELATIVE_PATH", "core")
    )
    parser.add_argument(
        "--hive-repo",
        type=Path,
        help="Actual local HIVE v1.0.0 checkout (else HIVE_REPO_PATH or sibling)",
    )
    args = parser.parse_args()
    try:
        output = collect_evidence(
            base_url=args.base_url,
            relative_path=args.relative_path,
            core_repo=Path(__file__).resolve().parents[1],
            hive_repo=args.hive_repo or resolve_hive_repo(),
        )
    except EvidenceBlocked as exc:
        print(json.dumps({"status": "BLOCKED", "reason": str(exc)}))
        return 1
    except (OSError, ValueError, TypeError, RuntimeError, subprocess.SubprocessError):
        # Never print raw API errors; they can contain local paths or payloads.
        print(json.dumps({"status": "BLOCKED", "reason": "local_dependency_or_service_unavailable"}))
        return 1
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
