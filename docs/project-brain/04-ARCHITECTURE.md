# CORE Architecture

Status: `BOOTSTRAP_BASELINE`

This document freezes only the foundation boundary. Product architecture remains pending discovery.

## Foundation planes

### HIVE intelligence plane
External HIVE v1.0.0 supplies project registry, repository intelligence, retrieval, memory, checkpoint-first context, token optimization and read-only MCP context capabilities.

### CORE product plane
This repository will contain the future CORE operational product. Its runtime architecture is not yet selected.

### GEF governance plane
GEF governs planning, Work Orders, context locks, preflight, evidence, exact-head review, checkpoints and release progression. GEF metadata is derived engineering state, not a replacement for Project Brain.

### Git/GitHub truth and transport plane
Git is canonical source history. GitHub provides hosting, PRs, CI evidence and review acceleration. Product runtime correctness must not silently depend on GitHub unless future Scope explicitly admits that dependency.

## Foundation flow

```text
Project intent
  -> Project Brain canonical truth
  -> HIVE context/retrieval
  -> GEF bounded Work Order
  -> executor
  -> tests/evidence
  -> exact-head audit
  -> checkpoint promotion
```

## Non-duplication invariant

CORE MUST consume stable HIVE capabilities instead of reimplementing HIVE context/memory/retrieval solely for local convenience.

## Product architecture gate

No runtime framework, database, queue, agent topology or deployment architecture is frozen by this bootstrap. Those decisions require discovery, requirements, threat/failure analysis and ADRs.
