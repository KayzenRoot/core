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

## Reviewer-first Correction Delta (2026-09-27, SAME PR #98)

The exact historical head `d15806f023c867aca1e6cc0c096862479f54b762` failed hosted workflow [#36001862823](https://github.com/KayzenRoot/core/actions/runs/36001862823) only in the M01 Ubuntu Advisory scan: raw job logs show bare `cargo-audit` printed Cargo subcommand usage and exited 2, even though the pinned `cargo-audit 0.22.2` installation succeeded and `cargo-deny check` passed. Replace the single incorrect bare binary invocation with `cargo audit` so Cargo dispatches the installed plugin with the required `audit` subcommand.

Protect critical M04 source/lock/governance amendment paths by fail-closed FULL classification even when file extension is Markdown/JSON; retain normal low-risk docs-only fast path. Add deterministic tests for the critical canonical paths. No frozen M04 source or product code change.

Re-sync non-force against latest protected main using a two-parent merge tree with only the FOUR originally scoped AIG paths over the exact main tree, preserving all unrelated main fixes. The earlier 10+ job failure is HISTORICAL; fresh exact merge-head CI, actual required job conclusions and owner self-audit `NOT INDEPENDENT` are PENDING and required before any PR #98 promotion. No performance claim is accepted until hosted evidence exists.

## CI correction delta #2: stale test-fixture expectation

New exact head `7ed992b0cc0766985d8986a0b4921d36e634d32c` [workflow #36330552740](https://github.com/KayzenRoot/core/actions/runs/36330552740) Governance executed 34 unit tests and identified one reviewer-introduced stale test expectation: original `test_docs_only_is_governance_only` still used canonical `docs/project-brain/13-CHECKPOINT.md` after the intended fail-closed critical-source rule. Update that fixture to an actually noncritical `docs/HIVE-INTEGRATION.md` and retain separate explicit canonical FULL assertions. Zero product/contract/scope changes; run evidence at this previous head is historical and new CI is required. No merge until the fresh exact head is all green and audited.

## C03 audit hardening: derived M04 authority surfaces

The initial C01 critical-source classifier covered primary M04 canonical source and lock paths. A follow-up inspection of the actual 17-path draft Contract Delta #118 and historical M04 synchronization PRs showed additional authority-carrying derived artifacts: `.engineering/gef/`, `.engineering/evidence/CORE-WO-M04-*`, `.engineering/evidence/CORE-M04-*` and `docs/work-orders/CORE-M04-*`/`CODEX-HANDOFF-M04.md`. Require FULL for these also, with explicit negative tests, because a derived Evidence↔Context Lock or GEF-authority-only PR must not masquerade as a generic docs-only change under protected-main assurance. Keep ordinary noncritical docs Governance-only. Previous CI heads remain historical; re-run fresh exact-head full gates after C03.

## C04 security correction: eliminate untrusted filename shell interpolation

Source audit identified direct `${{ needs.impact.outputs.reason }}` interpolation inside a `run:` shell body. `reason` can embed a Git PR changed filename; hostile path text (including shell substitution tokens) must not be inserted into executable shell source. Suppress the raw value in downstream `run:` and point users to the classifier job, which already logs the reason as ordinary Python stdout. Add a regression test that asserts the vulnerable echo pattern cannot recur. Preserve predefined Boolean/mode outputs only. This is a security correction, not a completed exploit or claim about a remote adversary. Prior head CI is historical, fresh exact-head full gates required.
