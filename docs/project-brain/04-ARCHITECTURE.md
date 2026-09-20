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


## Discovery architecture direction

```text
HIVE = intelligence plane
CORE = action plane
GEF  = governance protocol
Git/GitHub = source history + governed delivery transport
```

CORE is headless. CLI, APIs, MCP and structured events may be admitted by module planning; visual control surfaces are not part of CORE.

CORE standalone behavior uses only bounded fallback capabilities needed for safe operation. When HIVE is compatible/available, Adaptive Capability Substitution is the candidate pattern for replacing those fallbacks with HIVE-owned intelligence.

HIVE and CORE must not share canonical database tables. Candidate synchronization uses versioned envelopes, fingerprints and events.

Canonical discovery map: `docs/modules/00-MASTER-MODULE-MAP.md`.
Planning/execution protocol: `docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md`.


## Runtime technology baseline

CORE product runtime is Rust-first:
- Rust stable;
- Tokio async runtime;
- one headless supervisor;
- trusted in-process modules plus selectively isolated workers;
- local OS IPC for first-party local worker boundaries;
- hierarchical cancellation;
- minimal append-only M01 runtime journal;
- TOML human configuration;
- canonical typed machine contracts;
- no arbitrary dynamic-library plugins.

HIVE remains independently implemented/deployed. CORE communicates with HIVE through versioned external contracts rather than sharing language/runtime/database internals.


## M02 Project / Workspace Adapter architecture direction

M02 sits directly above M01 runtime/contracts and below every later module that needs a concrete project checkout.

```text
HIVE Project Registry (external intelligence authority)
          |
          | optional project identity/provenance
          v
M02 Basis Reconciliation Layer
          ^
          | local deterministic evidence
          |
Git / filesystem / explicit config
          |
          v
Workspace Identity + Authority + Basis
          |
          v
WorkspaceHandle / Workspace Binding Receipt
          |
          +--> M03 Work Order Engine
          +--> M04 Run Engine
          +--> M05 Host Adapters
          +--> M11 Sandbox/Leases
          +--> M13 Mutation
          +--> M20 Git Delivery
```

M02 is read-oriented. It may invoke Git only for bounded inspection/discovery needed to establish the current basis. It does not commit, checkout, reset, branch, push, merge or otherwise mutate repository state.

M02 consumes M01 deterministic canonical serialization, generations, typed errors, health/degradation and capability provenance rather than creating parallel primitives.

Candidate M02 technologies under evaluation:
- WIL Workspace Identity Lattice;
- CWB Canonical Workspace Basis;
- PAF Path Authority Firewall;
- BRL Basis Reconciliation Layer;
- WDG Workspace Drift Guard;
- RBR Repository Boundary Resolver;
- WBR Workspace Binding Receipt;
- DWS Delta Workspace Snapshot.

These are discovery candidates until M02 technology disposition is governed by measurable tests/benchmarks.


## M02 validity architecture

M02 separates durable evidence from live action state.

```text
WorkspaceAttachRequest
        |
        v
DISCOVERING -> VALIDATING
        |          |
        |          +---- local Git/filesystem evidence
        |          +---- explicit intent/config
        |          +---- optional HIVE association evidence
        v
WorkspaceBindingReceipt  (durable proof)
        |
        +--> WorkspaceHandle (runtime-epoch-bound)
                  |
                  +--> WorkspaceBasisFingerprint
                  +--> WorkspaceGeneration
                  +--> authority roots
                  +--> repository/worktree graph
                  +--> reconciliation/assurance
                  |
                  v
          downstream action boundary
                  |
             freshness check
                  |
         BOUND / DRIFTED / BLOCKED
```

Workspace basis is componentized so later modules can declare required validity masks without giving M02 permission to perform their actions. M02 may optimize revalidation with deltas only when the result is provably equivalent to full correctness evaluation.

Path validation is a proof layer, not an OS sandbox. Mutation/sandbox modules must revalidate security-sensitive receipts at use time.
