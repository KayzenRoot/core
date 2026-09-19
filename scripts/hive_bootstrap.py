from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any


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
        body = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"{method} {path} -> HTTP {exc.code}: {body}") from exc
    except urllib.error.URLError as exc:
        raise RuntimeError(f"HIVE unavailable at {base_url}: {exc.reason}") from exc


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
        collision_paths = ", ".join(
            sorted(str(item.get("relative_path")) for item in name_collisions)
        )
        raise RuntimeError(
            f'HIVE already has project name "{name}" at a different path: '
            f"{collision_paths}. Resolve the identity explicitly instead of guessing."
        )
    return None


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
    print("HIVE health:", health)

    projects = request(args.base_url, "GET", "/api/v1/projects")
    if not isinstance(projects, list):
        raise RuntimeError("HIVE project list returned an unexpected payload")

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

    project_id = target.get("project_id")
    if not project_id:
        raise RuntimeError("HIVE did not return project_id")

    inspected = request(args.base_url, "POST", f"/api/v1/projects/{project_id}/inspect")
    if inspected.get("state") != "READY":
        raise RuntimeError(f"CORE is not READY in HIVE: {inspected}")
    if inspected.get("relative_path") != args.relative_path:
        raise RuntimeError(
            "HIVE inspection resolved a different canonical path; "
            "refuse to prepare an ambiguous project identity."
        )
    print("Inspection: READY", inspected.get("git_head_sha"))

    index = request(args.base_url, "POST", f"/api/v1/projects/{project_id}/index")
    if index.get("status") != "COMPLETED":
        raise RuntimeError(f"CORE indexing did not complete: {index}")
    print("Repository index: COMPLETED")

    corpus = request(
        args.base_url,
        "POST",
        f"/api/v1/projects/{project_id}/retrieval/corpus/sync",
    )
    if corpus.get("status") not in {"COMPLETED", "CURRENT"}:
        raise RuntimeError(f"CORE retrieval corpus is not current: {corpus}")
    print("Retrieval corpus:", corpus.get("status"))

    print(
        json.dumps(
            {
                "project_id": project_id,
                "relative_path": inspected.get("relative_path"),
                "git_head_sha": inspected.get("git_head_sha"),
                "state": inspected.get("state"),
                "index_status": index.get("status"),
                "corpus_status": corpus.get("status"),
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"HIVE bootstrap failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
