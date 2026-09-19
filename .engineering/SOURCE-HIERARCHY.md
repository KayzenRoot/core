# CORE Source Hierarchy

Status: `FROZEN_BOOTSTRAP`

Authority is domain-specific. Derived summaries, HIVE indexes, memories and GEF metadata accelerate work but do not silently replace canonical source truth.

## Domains

- **REPOSITORY_STATE:** Git files, commits, diffs and executable facts at an exact SHA.
- **PROJECT_STATE:** `docs/project-brain/13-CHECKPOINT.md`.
- **DECISION:** `docs/project-brain/16-DECISIONS-LEDGER.md` and future ADRs.
- **SCOPE:** `docs/project-brain/03-SCOPE.md`.
- **REQUIREMENT:** `docs/project-brain/02-REQUIREMENTS.md`.
- **ARCHITECTURE:** `docs/project-brain/04-ARCHITECTURE.md`.
- **COMPLETION:** `docs/project-brain/15-DEFINITION-OF-DONE.md`.
- **EXECUTION:** active admitted Work Order under `.engineering/work-orders/`.
- **VALIDATION:** exact-head tests, CI, evidence bundles and audit results.
- **FUTURE_WORK:** `docs/project-brain/14-BACKLOG.md`.
- **CONVERSATION:** transient input only.

## Startup order

Checkpoint -> Decisions -> Scope -> DoD -> Architecture -> Requirements -> other applicable sources.

This order is a context-loading rule, not permission for one domain to overwrite another.

## Conflict behavior

If applicable authoritative sources conflict or a required source is stale/missing, stop the affected progression with a truthful conflict/block state. Never turn UNKNOWN into ALLOW or DONE.

## HIVE rule

HIVE retrieval, cache, memory, fingerprints and context capsules are derived. They may accelerate source resolution but cannot supersede tracked canonical Git content.
