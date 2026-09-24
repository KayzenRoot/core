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


## M03 Round 2 contract architecture

M03 v1 separates immutable semantics from current admission truth.

```text
WorkOrderRequestV1
        |
        v
      WOC
        |
        v
FrozenWorkOrderV1  --------------------+
 immutable semantic revision           |
        |                               |
        +--> WorkOrderRevisionDiffV1    |
        |                               |
        v                               |
WorkOrderAdmissionRequestV1             |
        |                               |
        +--> fresh M02 handle/basis     |
        +--> Context Lock proof         |
        +--> external governance proof  |
        +--> policy/security/config     |
        |                               |
        v                               |
WorkOrderAdmissionReceiptV1             |
 READY / STALE / BLOCKED / ...          |
        |                               |
        v                               |
AdmittedWorkOrderV1 <-------------------+
        |
        v
M04 revalidates admission bindings before Run creation
```

A frozen Work Order does not persist a live WorkspaceHandle. It carries `WorkspaceRequirementV1`; admission evaluates that requirement against current M02 runtime evidence.

A READY receipt is not a reusable capability forever. Its validity is parameterized by workspace generation/basis fingerprint, Context Lock fingerprint, governance proof, compiler/policy/security/config generations and relevant canonical-source identities.

### Packet graph architecture

M03 v1 models packets as a bounded DAG:
- acyclic;
- stable WorkPacketId;
- deterministic topological order with ID tie-break;
- packet scope intersected with parent ScopeEnvelope;
- dependencies fingerprinted as semantic input.

The DAG expresses prerequisite semantics only. It does not assign workers, agents, concurrency or scheduler policy.

### Revision/correction architecture

M03 separates:

```text
ExecutionCorrectionProposalV1
  -> validate against existing CorrectionPolicyV1
  -> same FrozenWorkOrder revision when still semantically inside contract

WorkOrderRevisionDiffV1
  -> semantic contract changed
  -> new immutable revision/fingerprint required
```

This prevents unnecessary revision churn while preserving strong governance for true semantic changes.

### Admission architecture

Admission verifies, but does not create, external authority:
- M02 provides workspace truth;
- Context Lock provides exact execution binding;
- governance system provides approval proof;
- M03 verifies compatibility and emits a receipt;
- M04 revalidates required bindings before new Run creation.

### Acceptance/evidence architecture

AEG is a declaration graph:
- M03 freezes criterion/evidence obligation identities and edges;
- M14 may later plan verification execution;
- M15 binds actual evidence artifacts;
- M16 issues independent review verdicts.

M03 cannot mark a criterion satisfied merely because its EvidenceRequirement exists.

### Context architecture

CBE carries finite dimensions and source expansion policy. Mandatory sources may be referenced/fingerprint-validated rather than inlined, but mandatory semantic content cannot disappear through truncation.

Round 2 freezes dimensions and behavior, not numeric defaults.


## M03 Round 3 service architecture

M03 core is a synchronous deterministic compiler/validator with explicit evidence inputs.

```text
external resolvers/adapters
  |  resolve source refs / M02 basis / Context Lock / governance / lineage
  v
CompilationContextV1 / AdmissionInputsV1
  |
  v
+---------------- core-work-order ----------------+
| compile            validate_frozen             |
| diff_revision      classify_correction         |
| evaluate_admission materialize_handoff         |
| canonical/identity scope/packets/AEG/context   |
| lineage/LPC        budget/errors               |
+------------------------------------------------+
  |
  +--> immutable contracts / receipts / fingerprints
  |
  v
external governed persistence / future M04
```

### Stateless persistence boundary

The repository/GEF domain may persist canonical Work Orders. M03 provides canonical bytes/contracts and a Lineage Precondition Capsule but does not perform the durable write itself.

LPC acts as compare-and-set evidence:
- expected parent revision/fingerprint/store generation;
- proposed new revision/fingerprint;
- deterministic precondition fingerprint.

A stale precondition cannot be silently rebased.

### Compiler purity boundary

No hidden repository/network/HIVE/GitHub access exists inside compiler semantics. Adapters resolve inputs first.

This allows:
- deterministic tests;
- replayable compilation;
- simpler fuzz/property models;
- low dependency surface;
- multiple future hosts without semantic drift.

### Context mesh architecture

```text
CanonicalSourceRef set
       |
       v
Packet Context Mesh (PCM)
  shared source nodes
  packet->source edges
       |
       +--> Packet A Context Plan
       +--> Packet B Context Plan
       +--> Packet C Context Plan
```

PCM deduplicates references, not obligations. Reconstructed packet mandatory-source sets must equal independently compiled requirements.

### Compilation provenance

DCR Deterministic Compilation Receipt records:
- request semantic fingerprint;
- compilation-context fingerprint;
- compiler schema/algorithm version;
- policy/config generation;
- lineage snapshot/precondition fingerprint;
- output WorkOrderFingerprint/CompilationId;
- bounded safe diagnostics.

DCR is evidence, not execution authority.

### Initial crate direction

One candidate crate: `core-work-order`.

The core is synchronous and does not require Tokio by default.

The Round 3 candidate dependency direction (`core-contracts + core-identity + core-config + core-workspace -> core-work-order -> future M04`) is refined by the Round 4 dependency disposition below after inspecting the actual workspace graph.

No database/network/Git/HIVE runtime dependency belongs in the compiler core.


## M03 Round 4 public-contract and adapter architecture

Round 4 freezes one versioned public boundary at schema nexlabs.core.work-order version 1. Authoring request, immutable FrozenWorkOrder, admission request/receipt and AdmittedWorkOrder handoff remain separate contracts. Distinct ID/revision/fingerprint wrappers make logical lineage, semantic revision, compiler result, packets, criteria, evidence and provenance non-interchangeable.

The pure synchronous core consumes already-resolved value evidence and emits deterministic contracts:

~~~text
caller-owned canonical source / M02 / Context Lock / GEF / optional HIVE adapters
        | bounded versioned evidence DTOs, fingerprints, provenance and freshness
        v
core-work-order pure compile / validate / diff / classify / admit / handoff
        | FrozenWorkOrder + DCR + LPC + immutable admission receipt
        v
external governed persistence / future M04
~~~

Resolver interfaces describe caller-owned source, M02 workspace/basis, Context Lock, governance, lineage and optional HIVE seams. No core-work-order service accepts or invokes a resolver. The host calls any needed adapter explicitly, then supplies its bounded evidence. M02 continues to own workspace/path/repository truth; the M03 DTO records only M02 schema/version, stable identities, generation, basis fingerprint, required profile/components and proof provenance. HIVE context remains advisory and cannot satisfy missing canonical, M02 or governance proof. Wall-clock deadlines are also host-owned: the pure core reads no clock; a timed-out invocation is discarded by the caller and cannot become accepted semantic/admission output.

FrozenWorkOrder semantic identity is an explicit projection. It includes schema/version/kind, semantic mission/scope/dependency policy, required canonical source identity/fingerprint/provenance/freshness, workspace/lock/governance requirements, packet DAG, acceptance/evidence graph, context obligations, correction policy, stop condition and lineage parent. It excludes diagnostic/transport/rendering data, timestamps, wall duration, PR URLs, runtime evidence and M04 state. Unordered collections are sorted before the existing core-identity canonical fingerprint primitive; diagnostics are stored outside the frozen revision.

### M03 Round 4 dependency disposition

Cargo metadata at the admitted base shows ten workspace crates and no M03 crate. Existing core-workspace directly enables Tokio filesystem, network and process features for M02 inspection. To keep M03 free of a transitive I/O-capable M02 service surface, V0.0 core-work-order does not directly depend on core-workspace or core-config. The outer adapter/host may depend on both core-workspace and core-work-order and map M02's public evidence to the narrow M03 DTO. This preserves M02 ownership without duplicating its workspace model.

The frozen M03 crate directly uses core-identity for canonical bytes/fingerprints and existing workspace serde, serde_json and thiserror dependencies for V1 contracts, bounded JSON and typed errors. It does not directly add core-contracts, core-config, core-workspace, sha2, Tokio, Git/network/process APIs, HIVE/GitHub SDKs, databases, graph libraries or caches. core-identity's observed transitive closure is core-contracts, serde, serde_json, sha2 and thiserror, with no process/network/database authority.

Dependency direction remains acyclic: existing M01/M02 crates are unchanged; core-work-order depends downward only on core-identity and serialization/error primitives; the outer host/adapters translate M02 evidence; future M04 consumes the immutable M03 handoff. Any direct M02 type coupling or additional dependency requires a separate architecture/dependency admission and fresh exact-head review.


## M04 Rounds 1-3 execution-state architecture candidate

M04 is the deterministic execution-state layer between the immutable M03 admitted handoff and later execution/policy/verification modules. It owns orchestration state, not host side effects.

~~~text
M03 AdmittedWorkOrder + READY admission evidence
        | exact BRC revalidation at Run boundary
        v
+---------------- core M04 state semantics ----------------+
| Run -> Attempt -> Step typed lineage                     |
| closed transition law graph (TLG)                       |
| Run generation / CAS serialization fence (CER)          |
| fingerprint-bound idempotency                            |
| monotonic cancellation                                   |
| append-only canonical journal + replay root (RJR)        |
| interruption continuation frame (ICF)                    |
| bounded references to later-module outcomes/evidence     |
+----------------------------------------------------------+
        | atomic state-fence commit port (ASF)
        v
external M04StateStoreV1 adapter

later M05-M24 consumers read/attach bounded versioned references;
M04 does not import their execution, policy or verification implementations.
~~~

### Authority and lifecycle

A Run can be admitted only after exact revalidation of the M03 READY handoff, WorkOrder identity/revision/fingerprint, workspace basis, Context Lock and governance/source generations. UNKNOWN or changed authority is STALE/BLOCKED.

Run, Attempt and Step identities are typed and immutable. Initial states are durable. Attempts and Steps are append-only children and their ordinals are never reused. Terminal states are historical facts and cannot reactivate. Retry/recovery policy remains outside M04; continuation after interruption creates a new Attempt/execution epoch.

### Concurrency and atomic publication

RunGeneration is the semantic serialization fence. Every mutating request supplies the expected generation. A successful semantic commit advances generation exactly once. State projection, canonical event append, journal-root update, idempotency record and resulting generation become visible atomically through the ASF storage port. Stale/future generation fails typed with no partial mutation.

No host wall clock establishes semantic order. Stable order is derived from Run generation, parent ordinals and event sequence.

### Idempotency and cancellation

Idempotency keys are scoped by Run and operation domain and are bound to a canonical request fingerprint. An exact duplicate returns the recorded result without a second event; conflicting key reuse fails closed.

Cancellation is monotonic. Once durably accepted, later child admission/activation is rejected except explicit bounded closeout. Earlier committed outcomes remain immutable history.

### Journal, replay and snapshots

The bounded canonical event journal is execution-state authority. Events are schema/version/domain separated and bind prior/result generations and prior/result journal roots. Replay requires a contiguous lineage-consistent sequence and fails closed on reorder, truncation, substitution or corruption.

Snapshots are derived acceleration artifacts bound to a verified journal boundary. They may rebuild/provide projection speed but cannot authorize state absent from the journal and cannot destructively replace active proof history.

### Public boundary

V1 exposes explicit request/receipt/result contracts for Run admission, Attempt creation, Step declaration, transitions, cancellation, continuation, replay and bounded reference attachment. Durable envelopes use closed schema/kind registries, typed IDs and machine-readable bounded errors/reason codes.

The deterministic core receives no ambient filesystem, repository, network, process, database, HIVE, GitHub or clock authority. Required external proof arrives as already-resolved bounded values/adapters. Core lifecycle/projection/replay semantics are zero-LLM.

### Canonical identity

M04 semantic fingerprints use schema-bound canonical bytes with explicit field ordering/lengths and domain separation. Diagnostic timestamps, locale formatting, unordered map order and secret-bearing fields are excluded. Cross-platform golden vectors are blocking evidence.

### External reference boundary

Later execution/verification modules may attach versioned bounded references carrying producer domain, immutable locator/identity, content fingerprint when available, and Run/Attempt/Step lineage. M04 validates shape, lineage and resource bounds only. It does not decide whether M14-M17 evidence is truthful or sufficient.

### Resource and calibration boundary

Finite production caps are required for attempts/run, steps/attempt, events/run, canonical event bytes, diagnostics, cursor bytes, references and replay depth. Cap+1 fails typed without partial advancement. Exact numeric defaults come from the later frozen Resource Calibration Gate and cannot change architecture or authority.

### Dependency direction

M04 may depend only on admitted lower-level M01/M03/shared primitive contracts and explicitly admitted storage/reference ports. M03 cannot depend back on M04, and M04 cannot import M05+ policy/execution/verification implementations. Persistent backend selection remains unfrozen in Round 3.

### Acceptance evidence architecture

The candidate M04 Acceptance Evidence Graph EV-M04-001..023 is blocking and covers public contracts, legal/illegal transitions, generation/CAS races, idempotency, cancellation races, journal replay/corruption, BRC/ICF, typed-identity substitution, canonicalization vectors, resource boundaries, snapshot equivalence, reference validation, hidden-I/O and zero-LLM proof, bounded fuzzing, calibration, supply-chain/SBOM, Windows/Ubuntu exact-head CI and independent exact-head review.

This is a planning candidate only. M04 product implementation remains unauthorized until the final planning freeze and a separate governed execution-admission delta are independently reviewed and promoted.


## M04 Round 4 implementation-addressable architecture candidate

### Crate boundary

M04 V0.0 is implemented as a single `core-run-state` crate. The crate contains only deterministic state semantics and versioned contracts. It does not own a database, runtime scheduler, executor, tool runner, Git client, HIVE client or external evidence service.

Direct dependency direction:

~~~text
core-contracts <- core-identity <- core-work-order <- core-run-state
                                      ^              |
                                      |              +--> later M05+ consumers
caller-owned M01/M02/GEF/HIVE adapters              |
                                      +--------------+
~~~

The direct M04 dependency set is `core-work-order`, `core-identity`, `serde` and `thiserror`. This keeps M03 handoff contracts available while avoiding direct Tokio/core-runtime/core-workspace or I/O-capable service dependencies.

### Pure semantic engine and host persistence

M04 uses a two-phase semantic/persistence model:

~~~text
host loads exact durable Run state
        |
        v
pure core-run-state prepare_* operation
  validate request + expected generation
  apply TLG/CER/idempotency/cancellation/BRC/ICF laws
  build canonical event + next projection + roots
  carry operation-specific receipt as PENDING result
        |
        v
PreparedCommitV1<R>
        |
        v
host-owned M04StateStoreV1.compare_and_commit(...)
  compare RunId + generation + prior root + idempotency state
  atomically persist event/projection/root/new generation
        |
        +--> DurableCommitReceiptV1
        +--> typed storage/generation conflict
        |
        v
pure finalize_commit(prepared, durable)
  exact-match durable proof
        |
        +--> authoritative Round 3 operation receipt R
~~~

The pure service never invokes the store. A receipt inside `PreparedCommitV1<R>` is explicitly pending and non-authoritative. After the host persists the exact prepared commit, pure `finalize_commit` validates the durable store receipt and only then releases the operation-specific Round 3 receipt as committed authority. This preserves deterministic in-memory testing and prevents persistence adapters from becoming hidden semantic authority.

### Store port contract

`M04StateStoreV1` is a host-facing port whose required capabilities are:
- load an exact Run durable record/snapshot boundary by RunId;
- atomically compare expected Run generation and prior journal root;
- atomically enforce the scoped idempotency record;
- persist exactly one prepared semantic commit or no semantic fragment;
- return a durable receipt containing committed generation, event sequence and journal root.

The port does not choose transitions, retries, cancellation behavior, identities, evidence truth or recovery policy. No concrete backend is selected in Round 4.

### Reference port contract

External outcome/evidence resolution is caller-owned. A reference adapter may validate or resolve an immutable locator and produce `ExternalReferenceEvidenceV1`. The pure `attach_reference` operation validates its version, lineage, fingerprint metadata and resource bounds before preparing `REFERENCE_ATTACHED`.

M04 never fetches artifact bodies and never imports M14-M17 verdict logic.

### Canonical framing

M04 semantic hashing does not duplicate the SHA-256 primitive already exposed by core-identity. Instead, M04 constructs a versioned binary frame:
- fixed domain/version prefix;
- typed field tag;
- fixed-width big-endian integer encoding;
- explicit presence marker for optional values;
- length prefix followed by bytes for variable-width strings/bytes;
- sorted canonical-key order for unordered collections.

The completed frame is passed to `core_identity::fingerprint_bytes`. Separate domain prefixes are required for IDs, requests, events, journal links, BRC, ICF, snapshots and prepared commits.

### File map

The V0.0 product crate is frozen to the file map in the canonical M04 module plan, including contracts, identity, canonical framing, transition laws, journal, projection, idempotency, cancellation, boundary/continuation, references, budgets, errors, store contract and pure services. Test, benchmark and fuzz surfaces are likewise named before implementation.

### Runtime cancellation seam

M01 cancellation/shutdown enters M04 as a bounded caller-owned value DTO containing only the execution-state facts required by the frozen cancellation law. `core-run-state` does not directly depend on the async runtime and does not subscribe to runtime events itself.

### Technology disposition

RAS, TLG, CER, RJR, BRC, ICF and ASF are required internal semantic mechanisms. They are implemented as normal Rust modules/types rather than independent services, plugins or feature-gated backends.

Persistent backend choice, destructive event retention, distributed state replication/consensus, persistent snapshot caching and hidden runtime autotuning remain deferred.

### Final-freeze boundary

Round 4 is not execution-ready. Round 5 must compile the implementation Work Order, pending Context Lock, evidence skeleton, construction packets, exact EV-M04 mapping, Resource Calibration Gate and executor handoff. A separate later admission delta must bind canonical main before any product implementation can begin.


## M04 Round 5 final-freeze execution architecture

The frozen implementation path is:

~~~text
promoted Round 5 planning truth
        |
        v
separate execution-admission delta
  exact canonical base + ACTIVE Context Lock
        |
        v
feat/m04-run-state
  Pack A -> B -> C -> D -> E -> F -> G -> H
        |
        v
implementation PR / READY_FOR_REVIEW
        |
        v
independent exact-head review
        |
        +--> APPROVED -> governed promotion
        +--> CORRECTION REQUIRED / BLOCKED
~~~

No planning artifact, prepared commit, executor claim or branch-local `ACTIVE` marker grants authority by itself. The later admission delta may bind state only; it cannot mutate the frozen M04 architecture.

The Work Order uses the Round 4 pure-core/host-store split unchanged. Packs A-F construct semantics, Pack G proves adversarial/security/fuzz/supply-chain obligations, and Pack H performs evidence-backed numeric calibration plus final exact-head validation. Any semantic change discovered during implementation requires the smallest governed Correction Delta rather than a widened Pack H.
