# CODEX HANDOFF — M04 Run / Attempt / Step Engine

Work Order: `CORE-WO-M04-001`  
Increment: `CORE-M04-FREEZE-001`  
Status: `EXECUTION_AUTHORIZED_NOT_STARTED / CORE_M04_SYNC_004_REVIEW_PENDING`
Future execution branch: `feat/m04-run-state`

## STOP BEFORE EXECUTION

Do **not** modify product code, Cargo manifests/lockfiles, fuzz targets, benchmark code or runtime crates unless all of the following are true on canonical `origin/main`:

1. CORE-M04-FREEZE-001 has been independently exact-head reviewed and promoted as canonical base `f6b422be5465d5a93d0b8fcf4c9507c205663072`.
2. CORE-M04-ADMIT-001 has been independently reviewed and promoted to canonical `origin/main` as merge `bb6f631284361fae29479c66f62ca88bebf3d79c`.
3. `.engineering/context-locks/CORE-WO-M04-001.json` on canonical main is `ACTIVE`.
4. The lock contains a concrete `authorizedBase`.
5. `productImplementationAuthorized = true`.
6. The lock binds the exact current `CORE-WO-M04-001` blob and the exact locked canonical source fingerprints.
7. The execution branch is exactly `feat/m04-run-state` and is created from the admitted post-promotion canonical main.
8. Git/governance preflight passes and optional HIVE preflight is recorded honestly if available.

If any condition is absent, stale, UNKNOWN, conflicting or exists only on a PR branch, stop with `BLOCKED / NOT_AUTHORIZED`.

## CANONICAL READ ORDER

Read:
1. `docs/project-brain/13-CHECKPOINT.md`
2. `docs/project-brain/16-DECISIONS-LEDGER.md`
3. `docs/project-brain/03-SCOPE.md`
4. `docs/project-brain/15-DEFINITION-OF-DONE.md`
5. `docs/project-brain/04-ARCHITECTURE.md`
6. `docs/project-brain/02-REQUIREMENTS.md`
7. `docs/project-brain/10-SECURITY-GOVERNANCE.md`
8. `docs/project-brain/11-TEST-PLAN.md`
9. `docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md`
10. `.engineering/SOURCE-HIERARCHY.md`
11. `.engineering/gef/GEF-CURRENT.json`
12. `AGENTS.md`
13. `.engineering/work-orders/CORE-WO-M04-001.md`
14. the exact ACTIVE M04 Context Lock.

Consult M03 implementation contracts only to satisfy the frozen M03→M04 handoff boundary. Do not reinterpret M03 as M04 policy.

## EXECUTION MODEL

Execute Packs A-H in order:
- A contracts / typed identity / canonical framing;
- B lifecycle / projection;
- C journal / replay / snapshots;
- D generation-CAS / idempotency / cancellation;
- E BRC / ICF / external references;
- F resource limits / prepared-finalized receipt boundary / store conformance / no hidden I/O;
- G properties / adversarial tests / six fuzz targets / security / supply chain;
- H deterministic calibration / bounded numeric delta / evidence / exact-head CI / PR handoff.

Do not advance a dependent pack while an upstream blocking obligation is unresolved.

## FROZEN IMPLEMENTATION BOUNDARY

Implement one crate: `crates/core-run-state`.

Allowed direct production dependencies:
- `core-work-order`
- `core-identity`
- `serde`
- `thiserror`

No direct production dependency on Tokio, `core-runtime`, `core-workspace`, Git/HIVE/GitHub SDKs, filesystem/network/process/database/time APIs, Criterion, proptest, graph/cache frameworks or a new cryptography stack.

`serde_json` is test/tooling-only unless a separate governed proof admits production use.

No production persistence backend is selected by this Work Order.

## RECEIPT AUTHORITY RULE

`prepare_*` returns `PreparedCommitV1<R>` with a pending operation-specific receipt. That receipt is **not committed authority**.

The host persists the prepared commit using the `M04StateStoreV1` equivalent atomic compare-and-commit port. Only after a matching `DurableCommitReceiptV1` is passed to pure `finalize_commit` may `R` be released as authoritative.

Any RunId/fingerprint/generation/event-sequence/journal-root mismatch fails closed.

## CALIBRATION

Do not invent resource defaults.

Pack H may run deterministic supported-platform measurements and apply the one authorized Calibration Delta only to:
- finite numeric `M04ResourceLimitsV1` defaults/thresholds;
- calibration fixtures/results/report;
- numeric test expectations that necessarily follow the selected limits.

If implementation evidence suggests a semantic, dependency, authority, backend or security change, stop and request a governed Correction Delta.

## EVIDENCE

Maintain `.engineering/evidence/CORE-WO-M04-001.json`.

AC-M04-001..023 map exactly one-to-one to EV-M04-001..023.

For each implementation-owned criterion record exact head, command/artifact, platform where applicable, result and evidence path/reference. Do not mark AC-M04-023/EV-M04-023 satisfied. That criterion belongs to the independent reviewer.

## TERMINAL STATES

### READY_FOR_REVIEW
Allowed only after Packs A-H, AC-M04-001..022, calibration/post-calibration reruns, required exact-head CI/security/supply-chain evidence and the implementation PR are complete with no unresolved HIGH/CRITICAL executor finding.

### BLOCKED
Use when any authorization, locked source, packet obligation, acceptance/evidence node, finite resource selection, CI/security gate or frozen contract is missing/stale/conflicting/failing or requires out-of-scope change.

Never return `APPROVED`. Never merge your own implementation.


## Admission candidate note

CORE-M04-SYNC-003 was APPROVED by M04-REVIEW-009 / Issue #97 at exact head `f9a5a7847e268000a5249ae8e69c81ed22b924ad`; PR #96; workflow `35994572596` with 10/10 jobs SUCCESS; promotion merge `b79891f489d8c7117aee15e1dca47abb9e23dea3`; unresolved HIGH/CRITICAL: 0.

CORE-M04-SYNC-004 is a governance-only closeout candidate. It does not start Pack A. After this correction is independently reviewed, APPROVED and promoted, the next legal action is the preflight for CORE-WO-M04-001 Pack A. Only after that preflight passes may `feat/m04-run-state` be created from the resulting canonical main and Pack A begin.
