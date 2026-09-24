# CORE-M04-SYNC-004 — Canonical Closeout

Status: `REVIEW_CANDIDATE`

Type: governance/project-state synchronization only

Canonical base: `b79891f489d8c7117aee15e1dca47abb9e23dea3`

Correction branch: `fix/m04-post-sync-canonical-closeout`

## Authority and preflight

- The complete CORE-M04-SYNC-004 work order was read before edits. Its scope is canonical state, provenance, fingerprints and handoff only. Pack A and all M04 product work remain stopped.
- `origin/main` was fetched and independently matched with `git ls-remote` at `b79891f489d8c7117aee15e1dca47abb9e23dea3`.
- The frozen authorized base `f6b422be5465d5a93d0b8fcf4c9507c205663072` is an ancestor of the canonical base.
- Work was isolated in a clean worktree created from the exact canonical base. The user's existing checkout at `D:/Projects/core` was left untouched; it remains on `fdb4dbe165e74b009c43df3874b6043c9b94710b` with pre-existing dirty state.
- Read-only HIVE v1.0.0 resolved project `2a5fb7e6-209a-4cd7-ac3b-137ca3b312bc`, but its project status pointed to the stale, dirty checkout above and `checkpoint.read` returned `stale/source_not_current`. No HIVE checkpoint content is used as canonical evidence; Git `origin/main` is the source of truth. Repeat HIVE/Git preflight after closeout promotion and before Pack A.
- Predecessor proof: CORE-M04-SYNC-003 was independently approved at PR #96 / Issue #97, exact reviewed head `f9a5a7847e268000a5249ae8e69c81ed22b924ad`, workflow `35994572596` (10/10 jobs successful), and promoted as the canonical base above. That review recorded zero unresolved HIGH/CRITICAL findings.

## Corrections in this closeout

- Synchronized Checkpoint and its derived bridge so the next legal action is conditional on this closeout's independent approval and promotion, then a fresh Pack A preflight.
- Reconciled status-only wording in Scope, Test Plan, the M04 module plan, Master Module Map and executor handoff with the already-promoted admission. Frozen M04 implementation semantics and authorized base are unchanged.
- Updated the active Context Lock execution rule and recorded the exact approved/promoted CORE-M04-SYNC-003 review facts.
- Refreshed the Work Order's nine canonical source blob bindings and the matching Context Lock/Evidence Bundle fingerprints to the candidate source state.
- Updated GEF-current promotion/next-action metadata and marked implementation as authorized but not started.
- No decision, architecture, product contract, dependency, Cargo file, product source, fuzz target, benchmark or implementation evidence was changed. No Pack A work began.

## Source binding audit

The active source set is the nine canonical files named by the CORE-WO-M04-001 Context Lock. The Work Order table, Context Lock and Evidence Bundle are bound to these candidate Git blob SHAs:

| Canonical source | Candidate Git blob SHA |
| --- | --- |
| `docs/project-brain/13-CHECKPOINT.md` | `a8b45c75f3f40fddb1762795fe4125fa7d860552` |
| `docs/project-brain/16-DECISIONS-LEDGER.md` | `6b7e873745e2ec6a81fedb6dbea5b0e0b271cde5` |
| `docs/project-brain/03-SCOPE.md` | `83a2509dab36c2b1676750d0e23542592255bc87` |
| `docs/project-brain/15-DEFINITION-OF-DONE.md` | `864f1974b340fca9a5a2d2e7e297c8f4fcfe5553` |
| `docs/project-brain/04-ARCHITECTURE.md` | `09f56adc576ee477fe61c5588fee201eac841bc2` |
| `docs/project-brain/02-REQUIREMENTS.md` | `b68bbdf41bbf06b1f1587261f39c31eee32e7cb9` |
| `docs/project-brain/10-SECURITY-GOVERNANCE.md` | `66a39179fb312eb229870aa5367e89708f6bfd66` |
| `docs/project-brain/11-TEST-PLAN.md` | `cc9e84fdbdea8a8d4e5a621756918ba361f5a031` |
| `docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md` | `51894d86ca39186eb5345881d457021ab90e34ed` |

The lock binds Work Order blob `2325764617a640da724cbec39f0edbe8e2dea8fb`; the Evidence Bundle binds Context Lock blob `327da2fb04ca2d363061e445730314f9652da92f` and handoff blob `a57ce41b292f7e6c487d62709a315d6bbe72a849`. These bindings are checked against the candidate worktree immediately before commit. GitHub PR metadata and hosted checks identify the final commit head and workflow run.

The three stale values present in the promoted Work Order table at the canonical base were Checkpoint `025045cc2269ee2bb4b043e024b00aa72fa713a3`, Decisions Ledger `a55a912576d8279dfe2bde89f43f19e4a1dd7b16`, and M04 module plan `882147c31c4dd2f5ed837474f4c914ba4c025c8f`. The active Context Lock already bound their then-current canonical blobs. This closeout refreshes the Work Order table and all dependent fingerprints after the status-only reconciliation.

## Validation and hosted review

- `py -3.12 scripts/validate_governance.py` — PASS; GEF v1.0.0 and HIVE compatibility v1.0.0 recognized; checkpoint/source bridges consistent; 27 required artifacts.
- JSON parsing of Context Lock, Evidence Bundle, GEF-current and Checkpoint bridge — PASS.
- Work Order table, Context Lock and Evidence Bundle match all nine canonical source blobs; Work Order, Context Lock and handoff blobs match their bound hashes — PASS.
- Active lock authority fields and exact `authorizedBase`, Checkpoint bridge parity, GEF next-action state and frozen-base ancestry — PASS.
- Changed-file inventory — PASS: 14 governance/evidence files; no product, Cargo, fuzz or benchmark paths.
- `git diff --check` — PASS.

Hosted required checks and independent review are separate GitHub gates and must be read against the exact final PR head. This candidate artifact does not assert their outcome. It is not approved or promoted.

Hosted exact-head required checks and independent review are pending until the PR exists. This candidate is not approved or promoted. Pack A remains unauthorized to start until this correction is independently approved and promoted and its separate preflight passes.

## Proposed Checkpoint Delta

After CORE-M04-SYNC-004 is independently approved and promoted, run fresh Git/HIVE/governance preflight for CORE-WO-M04-001 on canonical `origin/main`; verify the ACTIVE Context Lock, all nine source fingerprints, exact Work Order blob, frozen authorized base ancestry and governance-only intervening changes. Only if every check passes, create `feat/m04-run-state` from that exact canonical main and begin Pack A within the frozen `M04_RUN_ATTEMPT_STEP_ENGINE` scope. No M04 product implementation has started.
