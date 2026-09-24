# CORE-CI-AIG-001 — Adaptive Integrity Gate

Status: `REVIEW_CANDIDATE`  
Type: CI reliability/performance optimization  
Canonical base: `b79891f489d8c7117aee15e1dca47abb9e23dea3`  
Product semantics changed: `NO`

## Problem

The legacy Governance workflow executes the complete M01/M02/M03 Rust, fuzz and supply-chain matrix for every pull request, including governance/docs-only deltas. It also repeatedly compiles `cargo-deny`, `cargo-audit` and `cargo-fuzz` in separate jobs.

Observed recent hosted logs show one Linux M02 job spending roughly 4m45s installing `cargo-deny v0.20.2` and `cargo-audit v0.22.2` before the actual policy scans. The same tools were installed again in other jobs. This is duplicated assurance cost, not additional evidence quality.

## Adaptive Integrity Gate (AIG)

AIG is deterministic and fail-closed.

For pull requests:
- docs/`.engineering`-only deltas run mandatory Governance while required M01/M02 status contexts complete as lightweight no-op proofs;
- module-local M01/M02/M03 code changes run the affected module and known reverse dependents;
- shared foundations, Cargo/workspace metadata, workflow/tooling changes, M04 product surfaces and every unknown surface run full assurance.

For push to `main`:
- full assurance always runs.

The existing required status context names are preserved:
- Governance
- M01 (ubuntu-latest)
- M01 (windows-latest)
- M01 fuzz campaign
- M02 workspace adapter (ubuntu-latest)
- M02 workspace adapter (windows-latest)
- M02 bounded fuzz campaign

M03 hosted contexts remain present.

## Performance changes without assurance reduction

- cancel superseded runs for the same PR via workflow concurrency;
- checkout the exact PR head explicitly;
- run Rust formatting once on Linux for Rust-impacting deltas;
- stop repeating full-workspace Clippy/tests in M02;
- centralize dependency/advisory scanning in the required M01 Linux context;
- pin `cargo-deny 0.20.2`, `cargo-audit 0.22.2` and `cargo-fuzz 0.13.2` to versions observed in the prior successful workflow;
- cache Cargo build/download state and pinned CI tool binaries;
- keep module-specific fuzzing only for impacted modules on PRs;
- keep full assurance on every canonical main push.

## Fail-closed classifier rules

`scripts/ci_impact.py` returns full assurance for:
- non-PR events;
- empty diffs;
- `.github/`, `.cargo/`, `scripts/`, `tests/`;
- Cargo manifests/lock, deny policy and Rust toolchain files;
- shared `core-contracts` / `core-identity`;
- M04 product/fuzz/benchmark surfaces until dedicated M04 hosted jobs exist;
- any unclassified repository surface.

The classifier has deterministic unit coverage in `tests/test_ci_impact.py`.

## M04 authority

This delta does not modify any canonical source fingerprint bound by the active CORE-WO-M04-001 Context Lock, the M04 Work Order, GEF policy, M04 contracts, Packs A-H, AC/EV semantics or product implementation scope.

## Expected effect

Governance/docs-only PRs should complete the merge gate near the duration of the Governance classifier/validation path instead of waiting for unrelated Rust/fuzz/supply-chain jobs.

Module-local PRs should avoid unrelated module matrices. Full-risk deltas still run the full matrix.

Exact performance must be measured from hosted runs after promotion; no unmeasured runtime reduction is treated as evidence.

## STOP CONDITION

Promote only after:
- exact-head workflow syntax is accepted by GitHub;
- classifier unit tests pass;
- Governance passes;
- because this PR changes the workflow and classifier tooling, the classifier intentionally selects FULL assurance for its own validation;
- all hosted jobs on the exact head succeed;
- independent review finds zero unresolved HIGH/CRITICAL.
