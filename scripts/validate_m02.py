"""Deterministic static gates for CORE-WO-M02-001."""

from __future__ import annotations

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "crates" / "core-workspace"
ALLOWED_DEPENDENCIES = {
    "core-config",
    "core-contracts",
    "core-identity",
    "serde",
    "serde_json",
    "sha2",
    "thiserror",
    "tokio",
}
FORBIDDEN_TERMS = (
    "core-runtime",
    "core-health",
    "gix",
    "git2",
    "libgit2",
    "llm",
    "openai",
    "anthropic",
    "inference",
    "watcher-framework",
)


def main() -> int:
    cargo = (WORKSPACE / "Cargo.toml").read_text(encoding="utf-8")
    dependency_names = set(re.findall(r"^([A-Za-z0-9_-]+)(?:\.workspace)?\s*=", cargo, re.MULTILINE))
    dependencies = sorted(dependency_names - {"name", "version", "edition", "license", "rust-version"})
    forbidden_dependencies = sorted(set(dependencies) - ALLOWED_DEPENDENCIES)
    source_files = sorted(WORKSPACE.rglob("*.rs"))
    source = "\n".join(path.read_text(encoding="utf-8") for path in source_files)
    lower_source = source.lower()
    forbidden_terms = sorted(term for term in FORBIDDEN_TERMS if term in lower_source)
    unsafe_count = len(re.findall(r"\bunsafe\s+(?:\{|fn|impl|trait)", source))
    result = {
        "workspace": "core-workspace",
        "allowed_dependencies": dependencies,
        "forbidden_dependencies": forbidden_dependencies,
        "forbidden_runtime_terms": forbidden_terms,
        "unsafe_keyword_count": unsafe_count,
        "zero_llm": not any(term in lower_source for term in ("llm", "openai", "anthropic", "inference")),
        "status": "PASS" if not forbidden_dependencies and not forbidden_terms and unsafe_count == 0 else "FAIL",
    }
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
