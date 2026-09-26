#!/usr/bin/env python3
"""Deterministic CI impact classifier for CORE.

The classifier is deliberately fail-closed:
- non-PR events run full assurance;
- empty/unknown diffs run full assurance;
- shared foundations, CI/governance tooling, Cargo/workspace metadata and
  not-yet-specialized product modules run full assurance;
- docs/.engineering-only PRs keep Governance mandatory but avoid unrelated
  Rust/fuzz/supply-chain work;
- module-local code changes run only the affected module plus known reverse
  dependents.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


DOC_PREFIXES = ("docs/", ".engineering/")
DOC_FILES = {
    "README.md",
    "AGENTS.md",
    "LICENSE",
    "NOTICE",
    ".gitignore",
}

FULL_FILES = {
    "Cargo.toml",
    "Cargo.lock",
    "deny.toml",
    "rust-toolchain",
    "rust-toolchain.toml",
}
FULL_PREFIXES = (
    ".github/",
    ".cargo/",
    "scripts/",
    "tests/",
    "crates/core-contracts/",
    "crates/core-identity/",
    # Until M04 gets its own dedicated hosted jobs, any M04 product-code
    # change gets the existing full workspace matrix.
    "crates/core-run-state/",
)

M01_PREFIXES = (
    "crates/core-config/",
    "crates/core-registry/",
    "crates/core-journal/",
    "crates/core-ipc/",
    "crates/core-health/",
    "crates/core-runtime/",
    "crates/core-cli/",
)
M02_PREFIXES = ("crates/core-workspace/",)
M03_PREFIXES = ("crates/core-work-order/",)

M01_FUZZ = {
    "fuzz/fuzz_targets/ipc_frames.rs",
    "fuzz/fuzz_targets/config_toml.rs",
    "fuzz/fuzz_targets/compatibility.rs",
    "fuzz/fuzz_targets/journal_records.rs",
}
M02_FUZZ_PREFIX = "fuzz/fuzz_targets/m02_"
M03_FUZZ_PREFIX = "fuzz/fuzz_targets/m03_"
M04_FUZZ_PREFIX = "fuzz/fuzz_targets/m04_"


@dataclass(frozen=True)
class Impact:
    mode: str
    full: bool
    run_m01: bool
    run_m02: bool
    run_m03: bool
    run_m01_fuzz: bool
    run_m02_fuzz: bool
    run_m03_fuzz: bool
    run_supply_chain: bool
    rust_changed: bool
    changed_count: int
    reason: str

    def github_outputs(self) -> dict[str, str]:
        return {
            "mode": self.mode,
            "full": _b(self.full),
            "run_m01": _b(self.run_m01),
            "run_m02": _b(self.run_m02),
            "run_m03": _b(self.run_m03),
            "run_m01_fuzz": _b(self.run_m01_fuzz),
            "run_m02_fuzz": _b(self.run_m02_fuzz),
            "run_m03_fuzz": _b(self.run_m03_fuzz),
            "run_supply_chain": _b(self.run_supply_chain),
            "rust_changed": _b(self.rust_changed),
            "changed_count": str(self.changed_count),
            "reason": self.reason,
        }


def _b(value: bool) -> str:
    return "true" if value else "false"


def _norm(path: str) -> str:
    normalized = path.strip().replace("\\", "/")
    while normalized.startswith("./"):
        normalized = normalized[2:]
    return normalized


def _starts(path: str, prefixes: tuple[str, ...]) -> bool:
    return any(path.startswith(prefix) for prefix in prefixes)


def _is_doc_only(path: str) -> bool:
    return path in DOC_FILES or _starts(path, DOC_PREFIXES)


def _full(count: int, reason: str) -> Impact:
    return Impact(
        mode="full",
        full=True,
        run_m01=True,
        run_m02=True,
        run_m03=True,
        run_m01_fuzz=True,
        run_m02_fuzz=True,
        run_m03_fuzz=True,
        run_supply_chain=True,
        rust_changed=True,
        changed_count=count,
        reason=reason,
    )


def classify(paths: Iterable[str], event_name: str = "pull_request") -> Impact:
    changed = sorted({p for raw in paths if (p := _norm(raw))})
    count = len(changed)

    if event_name != "pull_request":
        return _full(count, f"{event_name or 'unknown'} event requires full assurance")
    if not changed:
        return _full(0, "empty diff is fail-closed to full assurance")

    m01 = m02 = m03 = False
    code_seen = False

    for path in changed:
        if _is_doc_only(path):
            continue

        if path in FULL_FILES or _starts(path, FULL_PREFIXES):
            return _full(count, f"full-assurance surface changed: {path}")

        if path == "fuzz/Cargo.toml" or path == "fuzz/Cargo.lock":
            return _full(count, f"shared fuzz manifest changed: {path}")

        if path in M01_FUZZ:
            m01 = code_seen = True
            continue
        if path.startswith(M02_FUZZ_PREFIX):
            m02 = code_seen = True
            continue
        if path.startswith(M03_FUZZ_PREFIX):
            m03 = code_seen = True
            continue
        if path.startswith(M04_FUZZ_PREFIX):
            return _full(count, f"M04 fuzz surface requires full assurance until dedicated M04 jobs exist: {path}")

        if path.startswith("benches/m01_"):
            m01 = code_seen = True
            continue
        if path.startswith("benches/m02_"):
            m02 = code_seen = True
            continue
        if path.startswith("benches/m03_"):
            m03 = code_seen = True
            continue
        if path.startswith("benches/m04_"):
            return _full(count, f"M04 benchmark surface requires full assurance until dedicated M04 jobs exist: {path}")

        if _starts(path, M01_PREFIXES):
            m01 = code_seen = True
            # core-config is a direct dependency of M02's core-workspace.
            if path.startswith("crates/core-config/"):
                m02 = True
            continue
        if _starts(path, M02_PREFIXES):
            m02 = code_seen = True
            continue
        if _starts(path, M03_PREFIXES):
            m03 = code_seen = True
            continue

        # Any unclassified product/repository surface is intentionally fail-closed.
        return _full(count, f"unclassified surface changed: {path}")

    if not code_seen:
        return Impact(
            mode="governance-only",
            full=False,
            run_m01=False,
            run_m02=False,
            run_m03=False,
            run_m01_fuzz=False,
            run_m02_fuzz=False,
            run_m03_fuzz=False,
            run_supply_chain=False,
            rust_changed=False,
            changed_count=count,
            reason="all changed paths are docs/governance-only",
        )

    selected = [name for name, enabled in (("m01", m01), ("m02", m02), ("m03", m03)) if enabled]
    return Impact(
        mode="+".join(selected),
        full=False,
        run_m01=m01,
        run_m02=m02,
        run_m03=m03,
        run_m01_fuzz=m01,
        run_m02_fuzz=m02,
        run_m03_fuzz=m03,
        run_supply_chain=True,
        rust_changed=True,
        changed_count=count,
        reason=f"module-local impact: {','.join(selected)}",
    )


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--event-name", required=True)
    parser.add_argument("--files-from", type=Path)
    parser.add_argument("--github-output", type=Path)
    parser.add_argument("paths", nargs="*")
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    paths = list(args.paths)
    if args.files_from:
        paths.extend(args.files_from.read_text(encoding="utf-8").splitlines())

    result = classify(paths, args.event_name)
    outputs = result.github_outputs()

    if args.github_output:
        with args.github_output.open("a", encoding="utf-8") as fh:
            for key, value in outputs.items():
                fh.write(f"{key}={value}\n")

    for key, value in outputs.items():
        print(f"{key}={value}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
