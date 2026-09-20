# CODEX HANDOFF — CORE-WO-M01-001

Execution branch: `feat/m01-core-runtime`
Frozen base: `fdb4dbe165e74b009c43df3874b6043c9b94710b`
Work Order: `docs/work-orders/CORE-WO-M01-001.md`
Module: M01 — Core Runtime & Lifecycle

## First action: synchronization and preflight

Before writing product code:

1. Verify the repository remote is `KayzenRoot/core`.
2. Fetch origin and checkout `feat/m01-core-runtime`.
3. Verify the frozen base is an ancestor of HEAD.
4. Read, in canonical order:
   - `docs/project-brain/13-CHECKPOINT.md`
   - `docs/project-brain/16-DECISIONS-LEDGER.md`
   - `docs/project-brain/03-SCOPE.md`
   - `docs/project-brain/15-DEFINITION-OF-DONE.md`
   - `docs/project-brain/04-ARCHITECTURE.md`
   - `docs/project-brain/02-REQUIREMENTS.md`
   - `docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md`
   - `docs/work-orders/CORE-WO-M01-001.md`
5. Run existing governance validation before implementation.
6. If local HIVE v1.0.0 is available, use its existing integration/bootstrap path for context. If unavailable, continue in bounded standalone mode and record that fact. Never fabricate HIVE evidence.
7. Produce a short preflight report: exact HEAD, toolchain availability, HIVE availability, governance result, blockers.
8. If no blocker, execute Packets A through H in order.

## Execution behavior

This is one comprehensive Work Order, not eight independent redesign prompts. Keep accepted architecture stable. At each packet:
- implement only its owned boundaries;
- run packet STOP tests;
- preserve passing evidence;
- correct failures before advancing;
- commit coherent increments;
- do not broaden scope;
- do not reread/re-emit large stable documents when their fingerprint/basis has not changed.

Use deterministic tools/tests before LLM reasoning. Reuse valid build/test/evidence artifacts. Prefer delta context.

## Required final report

Return:
- exact final HEAD;
- commits by Packet A-H;
- files created/changed;
- tests/property/fuzz/failure-injection results;
- benchmark baseline and WNF/PRB evidence;
- supply-chain/security/SBOM/unsafe evidence;
- zero-LLM lifecycle proof;
- HIVE integration status/evidence if actually available;
- residual risks;
- explicit mapping of all 21 acceptance criteria to evidence;
- final verdict: READY_FOR_REVIEW or BLOCKED.

## STOP

Do not stop because the workspace compiles.
Do not stop after a subset of packets.
Do not claim success with skipped required evidence.
Stop only under the Work Order FINAL STOP CONDITION.
