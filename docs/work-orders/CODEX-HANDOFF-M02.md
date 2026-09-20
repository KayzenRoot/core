# CODEX HANDOFF — CORE-WO-M02-001

Status: PENDING FINAL PLANNING PROMOTION
Execution branch: `feat/m02-project-workspace-adapter`
Work Order: `.engineering/work-orders/CORE-WO-M02-001.md`
Context Lock: `.engineering/context-locks/CORE-WO-M02-001.json`
Module: M02 — Project / Workspace Adapter

## Hard gate

Do NOT begin product implementation unless the Context Lock says:
- `status=ACTIVE`;
- `authorizedBase` is a concrete SHA;
- `productImplementationAuthorized=true`.

If any of these are false, STOP and report STALE/NOT_AUTHORIZED.

## First action after authorization

1. verify remote `KayzenRoot/core`;
2. fetch origin;
3. checkout/create `feat/m02-project-workspace-adapter` exactly from Context Lock `authorizedBase`;
4. verify the authorized base is the current execution ancestor;
5. validate Context Lock fingerprints;
6. read canonical sources in Work Order order;
7. run governance validation;
8. check HIVE truthfully and write `docs/evidence/M02-PREFLIGHT.md`;
9. only then execute Pack A through H.

## Execution behavior

This is one comprehensive module Work Order.

- Do not redesign frozen M02 architecture.
- Do not add dependencies for convenience.
- Keep the stable planning prefix cached; use delta context per packet.
- Run each packet STOP gate before proceeding.
- Preserve valid independent evidence.
- Fix causal failures at the smallest surface.
- Do not skip the Pack H Resource Calibration Gate.
- The bounded Calibration Delta is the only pre-authorized post-benchmark semantic edit class, and it is numeric/evidence-only.

## Final return

Return in Brazilian Portuguese:
- exact authorized base and final head;
- Context Lock validation;
- commits Pack A-H;
- files created/changed;
- HIVE preflight truth;
- acceptance criteria 1-41 mapped to evidence;
- unit/integration/property/adversarial/fuzz results;
- Windows/Ubuntu CI;
- security/supply-chain/SBOM;
- zero-LLM proof;
- M02 calibration report + final finite budgets + rejected candidates;
- errors/corrections;
- residual risks;
- proposed Checkpoint Delta;
- `READY_FOR_REVIEW` or `BLOCKED`.

Never return APPROVED. That verdict belongs to the independent governed reviewer.
