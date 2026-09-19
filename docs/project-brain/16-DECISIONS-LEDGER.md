# CORE Decisions Ledger

Status: `ACTIVE`

## CORE-D-001 - Product naming
**Decision:** repository name is `core`; product family presentation is **HIVE CORE**. HIVE is the intelligence/context product and CORE is intended to become the operational nucleus.
**State:** ACCEPTED

## CORE-D-002 - GEF baseline
**Decision:** bootstrap CORE from the production-accepted GEF Bootstrap `v1.0.0`, upstream release commit `866fe3af8cccc65c929aaf6a47a924401fa448b3`.
**State:** ACCEPTED

## CORE-D-003 - HIVE baseline
**Decision:** target HIVE `v1.0.0`, release commit `a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf`, as the initial stable integration contract.
**State:** ACCEPTED

## CORE-D-004 - Canonical source compatibility
**Decision:** CORE canonical product/governance truth uses HIVE-compatible `docs/project-brain` paths. GEF operational artifacts reference those sources rather than creating competing product truth.
**State:** ACCEPTED

## CORE-D-005 - HIVE is external
**Decision:** do not vendor the HIVE runtime into CORE. CORE integrates with the separately installed HIVE instance through its stable API/MCP surfaces.
**State:** ACCEPTED

## CORE-D-006 - No premature product architecture
**Decision:** bootstrap freezes governance and integration boundaries only. Product runtime architecture is selected during discovery.
**State:** ACCEPTED

## CORE-D-007 - Evidence before progression
**Decision:** exact-state evidence and audit, not activity or confidence, close governed work.
**State:** ACCEPTED
