# CODEX HANDOFF — M04 Run / Attempt / Step Engine

Work Order: `CORE-WO-M04-001`  
Increment: `CORE-M04-FREEZE-001`  
Status: `PACK_A_CANDIDATE_OPEN / OWNER_SELF_AUDIT_PENDING`
Execution branch: `feat/m04-run-state`

## CURRENT EXECUTION STATUS

CORE-D-203 was promoted to canonical main by PR #108 (squash commit 15179cf07d703f074cf50f793a5b1968ba356fc0). The sole operational identity is KayzenRoot; any owner self-audit must say NOT INDEPENDENT, with no second account or native self-approval.

Pack A passed the restricted owner-audit subgate M04-REVIEW-013 / Issue #109 at exact base 15179cf07d703f074cf50f793a5b1968ba356fc0 and head fecdb4c7069a815a63146c58192575e80c592334. Exact-head workflow 36311844093 passed 10/10 required/current jobs. This subgate authorized starting Pack B only, not global M04 completion, EV-M04-023, merge or checkpoint promotion.

Pack B has a provisional implementation candidate in PR #106 / feat/m04-run-state: pure closed Run/Attempt/Step transitions, provisional deterministic projection, zero-based non-reused ordinals and parent/child completion guards. Current code-before-evidence-update head: 202740112758424569d2bbd016303e1b279aa37a; Pack B audit candidate M04-REVIEW-014 / Issue #110. Resolve the exact **new** PR head and new exact-head CI after every code/evidence update before declaring a verdict.

Step SKIPPED checks only the explicit reason and reference shape/lineage at this stage. Pack E still owns verifying the prior durable attachment and external reference evidence. Neither the Pack B helper nor a projected snapshot is an authoritative journal/store receipt. The pure transition and projection helpers never advance generation, append journal events, choose retry policy or perform I/O.

Pack B remains TECHNICAL_VALIDATION_PENDING until current-head Windows/Ubuntu fmt/Clippy/tests/security/supply-chain/fuzz evidence and owner self-audit pass. Pack C, PR merge, EV-M04-023 and canonical checkpoint promotion stay stopped until their respective later gates.

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

For each implementation-owned criterion record exact head, command/artifact, platform where applicable, result and evidence path/reference. Do not mark AC-M04-023/EV-M04-023 satisfied during execution. That criterion belongs to the separate owner-audit stage, which KayzenRoot may perform on the same account and must disclose as `NOT INDEPENDENT`.

## TERMINAL STATES

### READY_FOR_OWNER_AUDIT
Allowed only after Packs A-H, AC-M04-001..022, calibration/post-calibration reruns, required exact-head CI/security/supply-chain evidence and the implementation PR are complete with no unresolved HIGH/CRITICAL executor finding. It hands the exact base/head to KayzenRoot's separate logical audit stage; it is not independent review.

### BLOCKED
Use when any authorization, locked source, packet obligation, acceptance/evidence node, finite resource selection, CI/security gate or frozen contract is missing/stale/conflicting/failing or requires out-of-scope change.

The executor stage does not submit a GitHub review or merge. After executor handoff, the KayzenRoot owner-audit stage may record `OWNER_SELF_AUDIT_APPROVED`, explicitly `NOT INDEPENDENT`, and squash-merge only after the exact-head review is clean, every required check passes, scope/lock/source bindings are valid, review threads are resolved, and unresolved HIGH/CRITICAL findings are zero. No second account/session or native self-approval is required.


## Admission candidate note

CORE-M04-SYNC-003 was APPROVED by M04-REVIEW-009 / Issue #97 at exact head `f9a5a7847e268000a5249ae8e69c81ed22b924ad`; PR #96; workflow `35994572596` with 10/10 jobs SUCCESS; promotion merge `b79891f489d8c7117aee15e1dca47abb9e23dea3`; unresolved HIGH/CRITICAL: 0.

CORE-M04-SYNC-004 was independently APPROVED by M04-REVIEW-010 / Issue #100 at exact head `19123d50bebe1a13257d8e2768fca7a1ca1d3393`; PR #99; workflow `36013624792` completed 10/10 jobs SUCCESS; squash promotion merge `b2ac8e0db72a2145948e7773295b1b252c5e4eab`; unresolved HIGH/CRITICAL: 0.

CORE-M04-SYNC-005 was independently APPROVED by M04-REVIEW-011 / Issue #102 at exact head `e8b0548c2ecd7c22edbeb3f02de8a238f9c49ffc`; PR #101; workflow `36033275191` completed 10/10 jobs SUCCESS; promotion merge `251f15495b82df8270ebc12fa93807ffaa15fba4`; unresolved HIGH/CRITICAL: 0.

PR #106 is now the existing CORE-M04-SYNC-006 / Pack B candidate after restricted Pack A approval in Issue #109. The new Pack B audit candidate is Issue #110 and requires a complete exact-current-head CI result, valid canonical source/lock fingerprints, full diff review and zero unresolved HIGH/CRITICAL. Record NOT INDEPENDENT. Pack C, M04-wide EV-M04-023, merge and canonical checkpoint promotion remain stopped pending their governed gates.
