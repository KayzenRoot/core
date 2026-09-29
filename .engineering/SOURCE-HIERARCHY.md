# CORE Source Hierarchy

Status: `FROZEN_BOOTSTRAP`

Authority is domain-specific. Derived summaries and GEF metadata help navigate work but never replace exact canonical tracked Git source truth.

## Domains

- **REPOSITORY_STATE:** Git files, commits, diffs and executable facts at an exact SHA.
- **PROJECT_STATE:** `docs/project-brain/13-CHECKPOINT.md`.
- **DECISION:** `docs/project-brain/16-DECISIONS-LEDGER.md` and future ADRs.
- **SCOPE:** `docs/project-brain/03-SCOPE.md`.
- **REQUIREMENT:** `docs/project-brain/02-REQUIREMENTS.md`.
- **ARCHITECTURE:** `docs/project-brain/04-ARCHITECTURE.md`.
- **SECURITY:** `docs/project-brain/10-SECURITY-GOVERNANCE.md`.
- **COMPLETION:** `docs/project-brain/15-DEFINITION-OF-DONE.md`.
- **EXECUTION:** active admitted Work Order under `.engineering/work-orders/`.
- **VALIDATION:** `docs/project-brain/11-TEST-PLAN.md` plus exact-head tests, CI, evidence bundles and audit results.
- **DEPLOYMENT:** `docs/project-brain/12-LOCAL-DEPLOYMENT.md`.
- **FUTURE_WORK:** `docs/project-brain/14-BACKLOG.md`.
- **CONVERSATION:** transient input only.

## Startup order

Checkpoint -> Decisions -> Scope -> DoD -> Architecture -> Requirements -> other applicable sources.

This order is a context-loading rule, not permission for one domain to overwrite another.

## Conflict behavior

If applicable authoritative sources conflict or a required source is stale/missing, stop the affected progression with a truthful conflict/block state. Never turn UNKNOWN into ALLOW or DONE.

## Standalone source rule

Use exact canonical Git commits, local tracked source and approved evidence. No installed project context service is needed for source authority.
