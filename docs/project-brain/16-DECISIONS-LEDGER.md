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


## CORE-D-008 - Headless CORE
**Decision:** CORE contains no dashboard, cockpit or web UI. A future visual NexLabs product may consume HIVE + CORE externally.
**State:** ACCEPTED

## CORE-D-009 - Intelligence/action ownership
**Decision:** HIVE owns durable intelligence/context/memory/retrieval/knowledge; CORE owns execution/orchestration/verification/delivery. Shared needs use contracts, not duplicate canonical engines.
**State:** ACCEPTED

## CORE-D-010 - Standalone plus HIVE-enhanced operation
**Decision:** CORE operates safely without HIVE. Compatible HIVE substitutes HIVE-owned intelligence providers; fallback capability stays bounded and must not evolve into a second HIVE.
**State:** ACCEPTED

## CORE-D-011 - Modular implementation cadence
**Decision:** ChatGPT performs architecture/orchestration and compiles executor-ready module plans. Codex performs heavy implementation. Default unit is one complete module per large bounded Work Order/prompt, followed by exact-head review and correction as needed.
**State:** ACCEPTED

## CORE-D-012 - Planning reduces executor rediscovery
**Decision:** Before Codex implementation, planning records target files, responsibilities, contracts, invariants, forbidden dependencies, tests, acceptance criteria, DoD and stop conditions.
**State:** ACCEPTED


## CORE-D-013 - Single complete product planning
**Decision:** CORE is not planned as an MVP ladder. ACCEPTED_REQUIRED capabilities are construction commitments. Research candidates require evidence before promotion and may be rejected rather than silently deferred.
**State:** ACCEPTED

## CORE-D-014 - Innovation must be falsifiable
**Decision:** proprietary technology candidates require an explicit problem, mechanism, expected benefit, risks, benchmark/evaluation and promotion criterion. A coined name alone is not innovation evidence.
**State:** ACCEPTED

## CORE-D-015 - M01 async-first, selective isolation direction
**Decision:** M01 planning proceeds with an async-first headless supervisor and selectively isolatable worker architecture. This does not imply microservices or distributed deployment. Final stack/transport remain pending M01 evidence.
**State:** ACCEPTED
