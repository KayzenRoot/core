# CORE Architecture

Status: `PRODUCT_DISCOVERY_ACTIVE`

This document contains the frozen foundation/runtime architecture plus accepted module-level product discovery architecture. Sections explicitly described as candidates remain unfrozen until their evidence gates are satisfied.

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


## M02 repository graph and trust-boundary architecture

M02 separates source paths from Git administration metadata.

```text
Workspace SOURCE_AUTHORITY
   |
   +-- Worktree A -----------+
   |                         |
   +-- Worktree B            | local source roots
   |                         |
   +-- Nested repo           |
                             v
                    Repository Graph
                             |
                             +--> GIT_METADATA_AUTHORITY
                             |      common-dir / worktree gitdir
                             |
                             +--> optional EXTERNAL_OBJECT_AUTHORITY
                                    alternates/shared objects
```

Only SOURCE_AUTHORITY is eligible to become downstream source-path input. Git metadata/object authorities exist solely to interpret repository state and do not transitively grant mutation capability.

HIVE association enters through a versioned `nexlabs.project-association@1`-style capability seam. M23 may later provide a deeper provider while M02 keeps the same consumer contract.


## M02 evidence/invalidation architecture

M02 separates optimization signals from correctness proof.

```text
watcher/Git hints
      |
      v
EIS Event Invalidation Spine
      |
      v
CIG Causal Invalidation Graph
      |
      v
required dirty basis mask
      |
      +--> GitInspector differential/provider layer
      +--> PAF/FSC physical proof
      +--> BHC bounded hashing
      +--> project-association refresh
      |
      v
Canonical Workspace Basis / Diff
      |
      +--> PEC L1 derived proof cache
      +--> compact evidence references
```

Correctness does not depend on watcher delivery or cache persistence. Action boundaries request a Basis Validity Matrix mask and revalidate every DIRTY/UNKNOWN or policy-required component.

Git backend architecture is provider-based. System Git serves as the semantic reference oracle in differential verification, while production provider selection remains governed by CORE-D-064 and benchmark/security evidence. A Rust-native or hybrid provider may be promoted only when its canonical evidence is equivalent for claimed capabilities.

Authority roots are explicit typed records rather than path prefixes. SOURCE, GIT_METADATA, EXTERNAL_OBJECT and INTERNAL_CORE_TEMP authority remain non-transitive.

The L1 proof cache is runtime-epoch scoped and disposable. Persistent proof caching remains optional and unapproved until corruption/recovery/security evidence exists.

M02 emits stable fingerprints, component masks and deltas so HIVE and later LLM-facing modules can reuse compact context rather than repeatedly embedding full path inventories or Git status payloads.


## M02 crate and contract architecture freeze

M02 V0.0 is one focused Rust crate: `core-workspace`.

It depends downward on core-contracts, core-identity and core-config, not on core-runtime. Runtime generation is contract data, avoiding a dependency cycle.

The initial GitInspector is hardened system Git behind a provider-neutral trait. Future Rust-native/hybrid providers remain swappable behind canonical GitEvidence and require separate evidence/admission.

PEC is L1/runtime-epoch only. EIS/CIG accept typed hints but correctness does not depend on an OS watcher adapter. These choices avoid a database, watcher framework and second Git parsing stack in the initial dependency graph.

Public M02 contracts are versioned, compact and canonical. Repository graphs, basis components and diffs use deterministic sorted serialization; durable receipts never become live capabilities.


## M02 Resource Calibration Gate architecture

Resource defaults are finalized by evidence inside the frozen M02 Work Order rather than fabricated during planning.

The implementation PR has a mandatory CALIBRATION_ONLY phase. After the frozen mechanisms exist, the executor runs deterministic local fixture/benchmark families, produces `docs/evidence/M02-CALIBRATION-REPORT.md`, applies the narrowly authorized numeric Calibration Delta, and reruns the entire exact-head validation suite.

The Calibration Delta cannot change architecture or dependencies. If evidence shows the architecture itself is insufficient, the Work Order stops and returns a normal governed Correction Delta.

DWS is part of the V0.0 architecture using component fingerprints and changed-set reconstruction. WMF is not part of the initial architecture and requires later evidence-backed admission.

Calibration is release/build evidence. M02 does not continuously self-tune in production.


## M03 Work Order Engine architecture discovery

M03 is the semantic compiler between M02 workspace truth and M04 runtime execution state.

```text
human / planner / API intent
          |
          v
 WorkOrderRequest
          |
          +--> canonical Project Brain / GEF source refs
          +--> M02 WorkspaceHandle / WorkspaceBasis refs
          +--> Context Lock / governance proof refs
          +--> optional HIVE compact context refs
          |
          v
     M03 Work Order Compiler
          |
          +--> schema/normalization
          +--> scope firewall
          +--> source/provenance binding
          +--> packet declarations
          +--> acceptance/evidence graph
          +--> correction policy
          +--> context budget envelope
          +--> deterministic fingerprint/lineage
          |
          v
   Frozen Work Order Revision
          |
      admission proof
          |
          v
    AdmittedWorkOrderV1
          |
          v
 M04 Run / Attempt / Step Engine
```

M03 never executes the Work Order it compiles. M04 owns mutable execution state. M05-M13 later own host/capability/agent/model/sandbox/tool/mutation behavior. M14-M16 later consume declared verification/evidence/review obligations.

The Work Order revision is immutable. A correction creates a new revision and a typed semantic delta. READY is only an eligibility state for M04 Run instantiation, not proof that execution succeeded.

### M03 canonical layers

1. **Intent layer**
   - non-executable authoring/request material.

2. **Semantic Work Order layer**
   - mission, scope, packets, acceptance, evidence, stop condition, correction policy.

3. **Binding layer**
   - canonical-source manifest, M02 basis requirements, Context Lock, governance proof and provenance.

4. **Admission layer**
   - exact frozen revision plus freshness/authorization evidence.

5. **Runtime handoff**
   - immutable AdmittedWorkOrderV1 consumed by M04.

### M03 Round 1 candidate mechanisms

- WOC Work Order Compiler;
- SDF Scope Delta Firewall;
- AEG Acceptance Evidence Graph;
- CBE Context Budget Envelope;
- WLG Work Order Lineage Graph;
- WSF Work Staleness Frontier;
- WPC Work Provenance Capsule.

These are discovery mechanisms, not implementation claims.

### Token-economy architecture

M03 does not duplicate HIVE retrieval intelligence. It packages canonical references and packet-specific context obligations so HIVE or other context providers can supply detail on demand.

Stable project context is represented by fingerprints/refs where possible; packet deltas carry only changed or specifically required inputs. Full raw source remains retrievable but is not duplicated into every compiled packet.

### Security architecture direction

Work Order compilation is treated as a security boundary because downstream execution trusts its scope.

Fail-closed checks include:
- source fingerprint substitution;
- stale Context Lock replay;
- superseded revision execution;
- semantic scope widening;
- dependency admission hidden inside small corrections;
- acceptance/evidence omission;
- packet cycles;
- oversized cardinality/resource abuse;
- secret-bearing provenance material;
- non-deterministic canonical serialization.

No M03 state may grant path authority beyond M02 or sandbox authority beyond future M11.
