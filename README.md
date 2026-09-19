# HIVE CORE

**CORE** is the operational nucleus of the NexLabs HIVE ecosystem.

HIVE provides project intelligence, context, memory, retrieval and governed context delivery. CORE is being designed as the operational engineering layer that will work with HIVE under the Governed Engineering Framework (GEF).

## Current state

- Repository mode: new project
- Product stage: bootstrap complete, product discovery/planning not started
- GEF baseline: v1.0.0
- HIVE compatibility baseline: v1.0.0
- Canonical project truth: `docs/project-brain/`
- Engineering governance: `.engineering/`
- Product code: not started

## Governing workflow

`ANALYZE -> SOURCE CHECK -> NEXT NECESSARY INCREMENT -> WORK ORDER -> CONTEXT LOCK -> PREFLIGHT -> EXECUTOR -> TESTS/EVIDENCE -> PR -> EXACT-HEAD AUDIT -> CHECKPOINT -> NEXT`

Read `AGENTS.md`, `.engineering/SOURCE-HIERARCHY.md` and `docs/project-brain/13-CHECKPOINT.md` before changing governed state.

## HIVE

CORE is HIVE-native. The canonical files required by HIVE v1.0.0 are materialized at the exact paths expected by the stable Context Manager and MCP surface.

See `docs/HIVE-INTEGRATION.md`.

## GEF

GEF Bootstrap is not vendored into this repository. The stable GEF v1.0.0 target-project contract is materialized here and pinned to its accepted upstream release.

See `docs/GEF-BOOTSTRAP.md`.
