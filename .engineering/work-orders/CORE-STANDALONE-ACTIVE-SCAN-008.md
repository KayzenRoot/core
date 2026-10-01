# CORE-STANDALONE-ACTIVE-SCAN-008

Status: `EXECUTOR_PENDING`  
Issue: #189  
Parent: #172  
Branch: `governance/core-standalone-active-scan-008`  
Exact authorized base: `9d03bba3bf35b242bf0468f5fa7737980364c8e5`  
GEF release basis: `v1.1.1`  
Assurance: `ELEVATED`  
Product implementation authorized: `false`

## Objective

Create a deterministic, no-network, repository-wide audit of retired-provider references. Every Git-tracked match must be classified exactly once as `ACTIVE_BLOCKING`, `NEGATIVE_GUARD`, `HISTORICAL_PROVENANCE`, or `STALE_BLOCKED_CONTEXT`.

The increment inventories and enforces classification only. It does **not** remove discovered active blockers.

## Source-truth order

1. `docs/project-brain/13-CHECKPOINT.md`
2. `docs/project-brain/16-DECISIONS-LEDGER.md`
3. `docs/project-brain/03-SCOPE.md`
4. `docs/project-brain/15-DEFINITION-OF-DONE.md`
5. `docs/project-brain/04-ARCHITECTURE.md`
6. `docs/project-brain/02-REQUIREMENTS.md`
7. `.engineering/decisions/CORE-D-205-STANDALONE-CANONICAL-CUTOVER.md`
8. `.engineering/decisions/CORE-D-207-STANDALONE-SUPPORT-SURFACES.md`
9. `.engineering/decisions/CORE-D-208-STANDALONE-MODULE-DOC-DISPOSITION.md`
10. `.engineering/context-locks/CORE-WO-M04-001.json`
11. Existing standalone tests and `scripts/validate_governance.py`.

## Required executor deliverables

- `scripts/scan_retired_provider_refs.py`
- `tests/test_standalone_active_provider_scan.py`
- `.engineering/evidence/CORE-STANDALONE-ACTIVE-SCAN-008.json`
- `.engineering/context-locks/CORE-STANDALONE-ACTIVE-SCAN-008.json`
- Optional `.engineering/decisions/CORE-D-209-RETIRED-PROVIDER-REFERENCE-CLASSIFICATION.md` only when needed to make semantics explicit.
- Evidence section appended to this Work Order after RED/GREEN/FULL evidence exists.

## Core rules

- Enumerate with `git ls-files`; stable deterministic ordering; no network.
- Start from the validator's existing retired-provider identifier semantics, not naive substring matching.
- Treat approved archive markers as boundaries: active prefix and historical suffix are classified independently.
- Never classify a whole current path as historical merely because it contains history.
- `NEGATIVE_GUARD` must actively reject/prevent retired-provider behavior.
- `STALE_BLOCKED_CONTEXT` requires the M04 lock to remain `STALE`, `productImplementationAuthorized=false`, and `BLOCKED_RE_ADMISSION`.
- Unknown/ambiguous cases fail closed.
- Capture the exact current `ACTIVE_BLOCKING` baseline; regressions must fail on unexpected expansion.
- Do not mutate runtime/product/schema/M04 canonical sources in this increment.
- Preserve historical bytes.

## TDD gate

RED first. Observe only the intended scanner/evidence/classification failure. Then minimum GREEN. Run focused tests, existing governance regressions, and exact-head FULL.

## Acceptance / STOP

All ACs are canonical in issue #189. Do not merge without exact-head FULL 11/11, zero unresolved HIGH/CRITICAL, zero unresolved review threads, and `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT`. Merge only by protected expected-head squash. Do not close #189 until a separate actual new-main push FULL is 11/11 SUCCESS.

If `ACTIVE_BLOCKING` is non-empty, record the exact bounded set and leave remediation to the next Work Order. Parent #172 remains open.
