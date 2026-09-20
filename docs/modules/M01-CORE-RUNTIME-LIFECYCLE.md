# M01 - Core Runtime & Lifecycle

Status: `COMPLETED_APPROVED`

## Mission

Define the durable headless runtime substrate that all later CORE modules can depend on without embedding HIVE-owned intelligence. CORE is planned as one complete product rather than an MVP ladder; accepted M01 capabilities are construction commitments.

## Ownership

M01 owns:
- CORE process/runtime identity;
- runtime bootstrap and shutdown;
- lifecycle state machine;
- dependency/capability registration seam;
- configuration loading contract;
- module registration/health contract;
- cancellation and graceful termination primitives;
- runtime compatibility/version identity;
- fail-closed startup semantics.

M01 does NOT own:
- project intelligence;
- RAG/retrieval;
- durable organizational memory;
- agent orchestration;
- model routing;
- work execution;
- verification;
- GitHub delivery;
- UI.

## Initial lifecycle candidate

```text
CREATED
 -> BOOTSTRAPPING
 -> READY
 -> DEGRADED
 -> DRAINING
 -> STOPPED

BOOTSTRAPPING -> BLOCKED
READY/DEGRADED -> FAILED
```

Exact transitions remain to be frozen during M01 planning.

## HIVE relationship

M01 must not require HIVE to start.

It exposes a provider/capability seam so M02/M23 can later attach compatible HIVE capabilities.

HIVE absence is not itself runtime failure. A module that explicitly requires a HIVE-owned capability may become unavailable/degraded while standalone-safe modules remain operational.

## ACS implications

M01 should establish the neutral capability registry/resolution primitive used later by Adaptive Capability Substitution, but MUST NOT implement HIVE-specific providers in M01.

## Candidate file map

Paths are planning targets, not implementation claims:

```text
src/
  runtime/
    index.*
    runtime.*
    lifecycle.*
    module-registry.*
    capability-registry.*
    config.*
    health.*
    shutdown.*
    errors.*
  contracts/
    runtime.*
    module.*
    capability.*
    health.*
tests/
  runtime/
    lifecycle.*
    module-registry.*
    capability-registry.*
    config.*
    shutdown.*
    failure-injection.*
docs/
  modules/
    M01-CORE-RUNTIME-LIFECYCLE.md
```

File extensions and language are deliberately unfrozen until stack selection is justified.

## File rules

### runtime
Own composition/root lifecycle only. No business feature logic.

### lifecycle
Closed transition table. Illegal transitions fail explicitly. No prose-only state semantics.

### module-registry
Registers CORE modules by stable ID/version/capabilities/dependencies. Duplicate IDs and dependency cycles fail closed.

### capability-registry
Resolves capability contracts and provenance. It must distinguish native CORE, standalone fallback and external/HIVE provider origins.

### config
Typed configuration with precedence and provenance. Secrets must not be serialized into diagnostics/events.

### health
Machine-readable liveness/readiness/degraded reasons. Health is evidence, not a dashboard.

### shutdown
Bounded drain/cancel/cleanup contract. No silent abandonment of future durable work.

### errors
Stable machine-readable error taxonomy with causal wrapping and redaction.

## Initial invariants

- headless only;
- no HIVE dependency for base startup;
- no shared HIVE database;
- deterministic lifecycle transitions;
- explicit degraded capability state;
- stable machine-readable errors;
- configuration is validated before READY;
- secrets are redacted;
- modules cannot silently replace another module's capability;
- external providers carry provenance/version;
- startup must not claim READY with missing NECESSARY runtime invariants.

## Planning questions still open

- implementation language/runtime for CORE;
- sync versus async module lifecycle interface;
- process model: single process first versus supervisor/worker split;
- persistence needed by M01 versus deferred to M04;
- configuration formats and environment precedence;
- compatibility/version negotiation representation;
- cancellation primitive;
- health aggregation semantics;
- module dependency-cycle strategy;
- whether plugin loading is necessary in V1 or should remain FUTURE.

## Preliminary tests

- legal/illegal lifecycle transition matrix;
- duplicate module registration;
- dependency cycle;
- missing required capability;
- optional capability absence -> degraded, not false failure;
- config corruption;
- secret redaction;
- startup exception;
- cancellation during bootstrap;
- graceful drain;
- shutdown timeout;
- external provider disconnect;
- deterministic repeated bootstrap.

## STOP CONDITION for M01 planning

M01 planning is not frozen until stack/runtime choices, lifecycle transitions, contracts, exact file map, failure model, tests, benchmarks, DoD and OUT OF SCOPE are accepted and reflected in canonical sources. No M01 product implementation before that point.


## Product completeness rule
No MVP tier. ACCEPTED_REQUIRED is a construction commitment. RESEARCH_CANDIDATE requires proof before promotion; REJECTED is deliberately excluded. FUTURE is not a parking lot for accepted requirements.

## Runtime architecture direction - Round 1

### Async-first control plane
M01 uses async-first lifecycle/control contracts for long-running tools, streams, cancellation, timeouts, agents and verification. Sync adapters may exist only at boundaries.

### Supervisor + selectively isolatable workers
A coherent local-first supervisor owns lifecycle, module/capability registries, configuration and health. Safe modules may run in-process; risky/resource-heavy/untrusted work can cross an isolated worker boundary. This is not a microservice mandate.

### No distributed-system tax by default
M01 does not require Kubernetes, service mesh, distributed consensus or RPC between every module.

## New M01 technology candidates

### RLC - Runtime Lifecycle Calculus
Machine-verifiable transition rules combine current state, transition intent, blocking invariants, capability availability, active leases/work and shutdown budget. Every accepted transition emits a typed receipt.

### CPG - Capability Provenance Graph
Tracks contract/version, provider, origin (CORE_NATIVE | CORE_FALLBACK | HIVE_EXTERNAL | OTHER_EXTERNAL), compatibility, health, trust, activation generation and supersession.

### RSG - Runtime Safety Genome
Secret-free deterministic fingerprint over runtime version, module manifests, capability contracts, configuration schema and safety-critical policy versions.

### QDS - Quiescence-Driven Shutdown
Computes safe drain/cancel boundaries before shutdown. Forced termination emits explicit incomplete-work evidence.

### DCM - Degraded Capability Matrix
Represents partial safe operation per capability rather than one healthy/unhealthy bit.

### BSR - Bootstrap Safety Receipt
Machine-readable proof of configuration generation, module graph, capability resolution and safety checks used to enter READY/DEGRADED. No receipt, no READY claim.

## Lifecycle refinement candidate
```text
CREATED -> VALIDATING -> BOOTSTRAPPING -> SYNCHRONIZING -> READY
READY <-> DEGRADED
startup stages -> BLOCKED
READY/DEGRADED -> DRAINING -> STOPPED
non-recoverable invariant violation -> FAILED
```
SYNCHRONIZING does not make HIVE mandatory. BLOCKED exposes diagnostics. DEGRADED enumerates capability loss through DCM.

## Configuration direction
Precedence: compiled safe defaults < repository config < machine/user config < environment < explicit process arguments.
Typed schema, value provenance, immutable generation ID, secret tagging/redaction, fail-closed unknown safety keys, explicit reloadability, no silent coercion of security/resource values.

## Module manifest candidate
Stable module ID; contract version; implementation version; required/optional/provided capabilities; lifecycle hooks; isolation class; criticality; startup/shutdown timeouts; health probes; config namespace; event schemas; compatibility constraints.

## Failure taxonomy candidate
CONFIGURATION_INVALID; MODULE_GRAPH_INVALID; CAPABILITY_UNAVAILABLE; CAPABILITY_INCOMPATIBLE; PROVIDER_DISCONNECTED; STARTUP_TIMEOUT; STARTUP_INVARIANT_FAILED; MODULE_START_FAILED; MODULE_CRASHED; CANCELLATION_FAILED; DRAIN_TIMEOUT; SHUTDOWN_FORCED; INTERNAL_INVARIANT_VIOLATION.

Every error carries stable code, causal chain, retryability, severity, affected module/capability, redacted detail and correlation identity.

## Robustness requirements to freeze
- deterministic dependency-graph startup;
- cycle detection before activation;
- idempotent lifecycle hooks where feasible;
- bounded startup/shutdown;
- no zombie worker after STOPPED;
- isolated-worker crash containment;
- provider disconnect cannot corrupt supervisor state;
- monotonic config/capability generation IDs;
- structured cancellation propagation;
- backpressure seam;
- replayable bootstrap diagnostics;
- failure injection hooks in tests.

## Performance/evaluation plan
Measure cold/warm bootstrap, graph validation at scale, capability resolution, health aggregation, config validation/reload, cancellation propagation, graceful shutdown, worker crash detection, idle CPU/RAM and event/control seam throughput. Requirements will be set from recorded baselines rather than invented numbers.

## Security direction
Least authority; no implicit tool/network/filesystem authority from registration; secrets excluded from BSR/RSG/events; mandatory external-provider provenance; validated config injection; unsafe dynamic loading disabled by default; immutable module identity; worker boundary prepared for M11 sandboxing.

## Expanded tests
Lifecycle property matrix; dependency-graph fuzzing; capability substitution races; config precedence properties; malformed config fuzzing; crash injection at every bootstrap stage; provider disconnect/reconnect; concurrent health races; cancellation storms; hung shutdown; forced worker kill; secret canaries; deterministic RSG; reproducible BSR; start/stop soak; resource leak detection.

## Round 1 open decisions
1. Primary language/runtime.
2. Same versus different worker runtime.
3. Exact async/cancellation primitive.
4. Minimal M01 journal versus all durable run state in M04.
5. IPC transport.
6. Config serialization.
7. Dynamic module/plugin policy.
8. M01/M24 event boundary.
9. Contract-version compatibility.
10. Benchmark corpus and reference hardware classes.


## Round 2 - Production, performance and LLM economics

Every technology/module decision is scored on reliability, quality, latency/throughput, CPU/RAM/I/O, input-token reduction, output/retry reduction, provider prompt-cache friendliness, semantic/evidence reuse, deterministic work avoided before inference, and invalidation correctness. Saving tokens may never lower the accepted quality floor.

### Cache-first runtime principle
M01 makes later LLM work cacheable by construction: stable versus volatile separation, canonical serialization, content fingerprints, stable ordering, immutable generations, delta propagation, complete cache keys and explainable invalidation/hit/miss/bypass evidence. M01 provides primitives, not the full provider cache engine.

### SCP - Stable Context Partitioning
Classifies reusable data as STABLE_PREFIX, SEMI_STABLE, DELTA or EPHEMERAL so later prompt compilers preserve byte-stable prefixes.

### CAG - Cache Affinity Graph
Tracks correctness-relevant dependencies of reusable results for targeted invalidation instead of global eviction.

### DIF - Deterministic Input Fingerprint
Canonical serialization plus cryptographic identity for correctness-relevant LLM/tool inputs, excluding irrelevant volatility.

### LCR - LLM Call Reuse Gate
Deterministic pre-call seam: exact reuse? verified evidence reuse? semantic reuse? delta-only call? invalid because authority changed?

### PSM - Prompt Stability Meter
Measures stable-prefix ratio, delta ratio, cache-key churn, avoidable volatile tokens and provider-cache-eligible prefix size.

### TEB - Token Economics Budget
Portable envelope for context/input, output, retry/escalation, cache preference, quality floor and evidence-reuse policy.

## Runtime implications for cache/performance
- canonical deterministic serialization;
- immutable config/module/capability generations;
- no LLM or repository scan on hot lifecycle/capability/health paths;
- lazy expensive enrichment;
- zero-LLM bootstrap/lifecycle/shutdown.

## Cache correctness invariants
- no hit without identity/provenance;
- no reuse across incompatible authority generations;
- failed/negative results have explicit invalidation/TTL semantics;
- security/policy changes invalidate affected reuse;
- provider cache artifacts are not provider-neutral evidence;
- cache is never canonical truth;
- cache miss cannot change correctness;
- metrics distinguish exact, provider-prefix, semantic, evidence and bypass.

## Added performance/evaluation tests
Canonical serialization determinism; irrelevant-metadata fingerprint stability; relevant-change fingerprint mutation; targeted CAG invalidation; stable-prefix benchmark; generation-churn benchmark; capability lookup p50/p95/p99; bootstrap at 10/100/1000 synthetic modules; idle CPU/RAM; cancellation under load; cache-receipt overhead; proof that lifecycle performs zero LLM calls.

## Stack evaluation matrix
Rust, Python and TypeScript/Node will be scored for supervision/async cancellation, predictable performance/memory, safe concurrency, startup overhead, cross-platform packaging, IPC/streaming, schemas/serialization, adapter ergonomics, observability, fuzz/property testing, HIVE integration, Codex/CLI integration, implementation velocity, maintainability and deterministic/cache-stable representations.

Architectures to compare:
A. TypeScript/Node supervisor + workers;
B. Python supervisor + workers;
C. Rust supervisor + adapters/workers;
D. Rust kernel + TypeScript orchestration;
E. TypeScript runtime + selective Rust performance/sandbox components.

Hybrid complexity must prove its operational value.


## Round 3 - Runtime stack decision

### Decision
CORE runtime kernel and first-party workers use **Rust stable**.

Primary async runtime: **Tokio**.
Serialization/contracts: **Serde** with canonical encoding rules owned by CORE.
Error boundaries: typed domain errors; no panic-as-control-flow.
CLI/headless entrypoint: Rust binary.
Python/TypeScript are adapter/SDK languages only when an external integration justifies them; they are not required runtime dependencies.

### Why Rust
- predictable low idle/runtime overhead;
- memory safety without GC pauses;
- strong concurrency/process supervision primitives;
- excellent fit for long-lived headless daemon/runtime;
- static single-binary-oriented distribution;
- strong type system for lifecycle/capability/evidence invariants;
- mature property/fuzz testing ecosystem;
- deterministic data structures/serialization can be enforced;
- isolates HIVE language choice from CORE runtime;
- avoids requiring Node/Python merely to keep CORE alive.

### Why not Python as CORE kernel
Python remains excellent for HIVE/data/AI integration but would duplicate HIVE's language/runtime coupling, has weaker isolation from blocking work and less predictable CPU/concurrency behavior for the action plane.

### Why not Node/TypeScript as CORE kernel
TypeScript has excellent orchestration ergonomics, but CORE's long-lived supervisor, process control, resource governance, sandbox preparation and low-overhead headless operation benefit more from Rust. TypeScript remains suitable for future SDKs/adapters.

### Why not Rust + TypeScript mandatory split
A mandatory dual-runtime architecture adds packaging, IPC, versioning, crash and observability tax before evidence shows a need. CORE will be Rust-first with polyglot edges, not polyglot at the center.

## Process model

```text
core binary
  |
  +-- supervisor
  |    +-- lifecycle
  |    +-- config generations
  |    +-- module graph
  |    +-- capability graph
  |    +-- health/DCM
  |    +-- cancellation tree
  |
  +-- trusted in-process modules
  |
  +-- worker supervisor
       +-- isolated first-party worker
       +-- external host adapter process
       +-- sandbox boundary (M11)
```

Isolation class is declared by module manifest:
- IN_PROCESS_TRUSTED;
- CHILD_PROCESS;
- SANDBOX_REQUIRED;
- EXTERNAL_PROVIDER.

No module may self-upgrade its isolation/authority class at runtime.

## Async and cancellation semantics

Tokio task cancellation is wrapped by a CORE-owned hierarchical cancellation contract.

Cancellation scopes:
- RUNTIME;
- MODULE;
- WORKER;
- future RUN;
- future ATTEMPT;
- future STEP.

Parent cancellation propagates downward. Child failure does not automatically cancel siblings unless policy marks the child critical.

Every cancellable operation declares:
- cooperative cancellation support;
- hard deadline;
- cleanup deadline;
- escalation action.

## IPC decision

First-party isolated local workers use **length-delimited framed messages over local OS IPC** with a versioned CORE protocol. Transport adapter:
- Unix domain sockets on Unix-like systems;
- named pipes on Windows.

No localhost TCP by default for same-machine first-party workers.

Payload contracts are versioned and canonicalized. Large binary/artifact payloads are referenced by content identity/path-safe artifact handles rather than copied repeatedly through control messages.

## Persistence boundary

M01 owns only a **small append-only Runtime Journal** for lifecycle safety, generation activation and incomplete shutdown/recovery markers.

M01 MUST NOT become the durable Work/Run database. M04 owns durable execution state.

Journal properties:
- append-only records;
- monotonic sequence;
- checksum/integrity;
- bounded retention/compaction;
- crash-safe write discipline;
- no secrets;
- replay only for runtime safety reconciliation.

## Configuration decision

Human-authored repository configuration: **TOML**.
Machine contracts/evidence: canonical structured serialization defined by schema; JSON representation is available for interoperability.
Environment variables and CLI overrides are supported under explicit typed precedence.

Config is divided into:
- STATIC_RESTART_REQUIRED;
- DYNAMIC_SAFE_RELOAD;
- SECRET_REFERENCE.

Secret values are resolved at runtime and never written back to config/evidence.

## Plugin policy

Arbitrary dynamic library loading is **REJECTED** for the product boundary.

Extensibility uses:
- versioned process adapters;
- MCP/API/stdio/IPC host adapters;
- capability contracts.

Reason: process boundaries provide better crash/security/version isolation than loading unknown code into the CORE supervisor.

## Compatibility policy

Contracts use semantic compatibility ranges plus explicit capability feature flags.

Rules:
- major contract mismatch -> incompatible/fail closed;
- required feature absent -> capability unavailable;
- optional feature absent -> degraded path allowed;
- unknown security-critical field/feature -> fail closed;
- provider implementation version and contract version are distinct.

## M01 / M24 event boundary

M01 owns only the minimal internal control-event primitive required for lifecycle and supervision.
M24 owns the full durable/observable Execution Nervous System event spine.

M01 events are typed and designed so M24 can bridge them without changing M01 semantics.

## Cache-aware Rust rules

- stable prompt/cache material MUST NOT use nondeterministic HashMap iteration;
- canonical maps use deterministic key ordering;
- volatile runtime IDs stay outside stable-cache payload sections;
- timestamps are metadata, not semantic identity unless a contract explicitly says otherwise;
- fingerprints operate on canonical bytes;
- serde schemas distinguish cache identity fields from diagnostic-only fields;
- avoid cloning/serializing large stable payloads when content handles suffice.

## New technology candidate - ZCP

### ZCP - Zero-Copy Context Handles
Large stable context/evidence artifacts can be addressed by immutable content handles and passed between local CORE components without repeatedly embedding/serializing the full payload.

Goals:
- lower RAM copies;
- lower IPC volume;
- preserve exact content identity;
- improve LLM prompt assembly reuse;
- make cache identity cheap.

ZCP is not shared mutable memory. Handles reference immutable content with verified identity.

## New technology candidate - GCL

### GCL - Generation Coherence Layer
Config, module graph, capability graph and policy bases carry generation IDs. A future run records the coherent generation tuple it used.

If a safety-relevant generation changes mid-operation, policy can continue, revalidate or cancel based on dependency impact rather than globally restarting everything.

This supports targeted cache invalidation and low-disruption production updates.

## New technology candidate - DCS

### DCS - Deterministic Canonical Serialization
CORE-defined canonical byte representation for fingerprints, receipts, cache keys and cross-module identities.

Properties:
- deterministic field/key ordering;
- explicit schema/version;
- normalized numeric/string rules;
- exclusion rules for diagnostic volatility;
- golden-vector tests across versions/languages.

DCS is foundational for DIF, RSG, BSR, TSS and cache correctness.

## M01 accepted architecture after Round 3

ACCEPTED_REQUIRED:
- Rust stable kernel;
- Tokio async runtime;
- supervisor + selective worker isolation;
- hierarchical cancellation;
- local OS IPC for first-party workers;
- minimal append-only runtime journal;
- TOML human config;
- typed/canonical machine contracts;
- no arbitrary in-process plugins;
- semantic contract compatibility;
- minimal M01 control events;
- cache-stable deterministic serialization discipline.

RESEARCH_CANDIDATE:
- RLC, CPG, RSG, QDS, DCM, BSR;
- SCP, CAG, DIF, LCR, PSM, TEB;
- ZCP, GCL, DCS.

REJECTED:
- Python as CORE kernel;
- Node/TypeScript as CORE kernel;
- mandatory dual-runtime kernel;
- arbitrary dynamic-library plugins;
- microservices-by-default;
- network TCP for same-machine first-party IPC by default.


## Round 4 - Module/Capability Fabric

### Separation invariant
A **module** is a deployable/runtime participant. A **capability** is a versioned behavior contract. Consumers depend on capabilities, not provider module identities, unless an explicit policy requires a named provider.

This separation is foundational for ACS, HIVE substitution, testing and cache-safe provider changes.

## Module Registry

Module identity tuple:
- module_id;
- contract_version;
- implementation_version;
- build_identity;
- isolation_class.

Module manifest additionally declares:
- required capabilities + version/features;
- optional capabilities;
- provided capabilities;
- startup dependencies;
- criticality;
- lifecycle hooks;
- health contract;
- startup/shutdown deadlines;
- configuration namespace/schema;
- authority requirements;
- event contract versions.

Registration phases:
```text
DISCOVERED -> VALIDATED -> ADMITTED -> ACTIVATING -> ACTIVE
                                      -> BLOCKED
ACTIVE -> DRAINING -> INACTIVE
ACTIVE -> QUARANTINED
```

A discovered module has no authority merely because it exists on disk.

## Capability Registry

Capability identity is provider-independent:
```text
namespace.capability-name@contract-major
```

Provider binding contains:
- provider_id/module_id;
- origin;
- contract version;
- feature set;
- health;
- trust/assurance class;
- authority requirements;
- latency/cost metadata when applicable;
- activation generation;
- deterministic provider fingerprint.

Consumers request a CapabilityRequirement, not a concrete provider.

## ACS resolution algorithm candidate

Deterministic ordering:
1. filter contract compatibility;
2. filter required features;
3. filter policy/authority eligibility;
4. filter health/readiness;
5. apply quality floor;
6. prefer configured provider class;
7. apply deterministic cost/performance/cache-affinity tie-break;
8. bind provider and emit CapabilityBindingReceipt.

No LLM chooses a provider in M01.

For HIVE-owned intelligence capability, default preference when HIVE is compatible and healthy:
```text
HIVE_EXTERNAL > CORE_FALLBACK
```
unless an explicit safety/policy constraint requires otherwise.

## Atomic capability substitution

Provider substitution uses prepare/commit semantics:
```text
candidate discovered
 -> validate compatibility
 -> health/provenance check
 -> compute impact
 -> PREPARED generation
 -> atomically publish new binding generation
 -> notify affected consumers
 -> retire old binding after leases drain
```

Consumers already holding a capability lease keep a coherent binding until their safe boundary unless policy requires immediate revocation.

## New technology - CAL

### CAL - Capability Atomic Leasing
Consumers receive a bounded immutable capability lease containing:
- capability identity;
- provider fingerprint;
- binding generation;
- validity/revocation policy;
- relevant authority;
- cache-affinity identity.

This prevents a long operation from unknowingly using provider A for half the work and provider B for the rest.

Expected benefits:
- deterministic execution;
- safer HIVE connect/disconnect;
- cleaner cache identity;
- reproducible evidence.

## New technology - SIR

### SIR - Substitution Impact Radius
Before changing a capability binding, CORE computes the dependency radius:
- directly bound consumers;
- derived capabilities;
- active leases;
- cache/evidence dependencies;
- safety-critical operations.

The impact determines whether substitution can be live, requires revalidation, waits for quiescence or is blocked.

SIR integrates with CAG and GCL.

## New technology - CBR

### CBR - Capability Binding Receipt
Every binding/substitution produces a canonical receipt containing decision inputs and selected provider, without secrets.

This gives later execution/evidence systems proof of which implementation actually supplied a behavior.

## New technology - FCH

### FCH - Fallback Capability Harness
Standalone fallbacks are explicitly constrained implementations with:
- declared quality/feature ceiling;
- no hidden durable organizational memory;
- no silent expansion into HIVE-owned responsibility;
- test vectors shared with the full capability contract;
- visible fallback provenance.

Purpose: CORE remains useful standalone without accidentally rebuilding HIVE.

## Cache-aware provider substitution

Provider change does NOT automatically invalidate every cache.

Cache identity distinguishes:
- PROVIDER_INDEPENDENT evidence;
- PROVIDER_BOUND output;
- MODEL_BOUND output;
- AUTHORITY_GENERATION_BOUND output.

CAG + SIR determine affected entries. A provider substitution only invalidates dependencies whose correctness identity includes that provider/binding generation.

## HIVE disconnect behavior

Unexpected HIVE loss:
1. mark affected provider bindings UNAVAILABLE;
2. revoke or drain leases according to capability policy;
3. compute SIR;
4. bind eligible fallback atomically when quality/policy floor allows;
5. mark DCM degradation;
6. invalidate only affected cache/evidence;
7. emit binding/degradation receipts.

No silent fallback is allowed when fallback quality is below the operation's declared floor.

## Anti-flapping policy

Provider health changes must not cause rapid HIVE<->fallback oscillation.

Candidate rules:
- health hysteresis;
- minimum healthy stabilization window before promotion;
- exponential reconnect backoff;
- substitution cooldown;
- immediate failover only for capabilities whose policy permits it;
- manual/policy pin supported for diagnosis.

## Registry performance requirements

- resolution is deterministic and local;
- hot capability lookup requires no network and no LLM;
- active binding reads use immutable snapshots/generations;
- graph recomputation is incremental;
- substitutions compute affected subgraph, not full system, when safe;
- registry reads remain available during preparation of next generation.

## Registry security requirements

- module discovery grants zero authority;
- provider cannot self-assert higher trust;
- capability contract cannot grant tool/filesystem/network authority implicitly;
- external/HIVE provider identity must be authenticated by later integration layer;
- authority escalation requires policy decision;
- quarantined providers cannot receive new leases;
- manifest/config provenance included in admission evidence.

## Registry test program

Property/fuzz:
- dependency cycles;
- conflicting providers;
- version-range boundaries;
- feature-set combinations;
- deterministic tie-break;
- substitution races;
- lease/revocation races;
- HIVE disconnect during active lease;
- provider flapping;
- fallback below quality floor;
- targeted cache invalidation;
- generation monotonicity;
- quarantine behavior;
- 10/100/1,000/10,000 capability synthetic graphs.

## Round 4 promotion proposal

Promote to ACCEPTED_REQUIRED after implementation evidence:
- CPG;
- GCL;
- DCS;
- CAL;
- SIR;
- CBR;
- FCH.

Keep CAG implementation depth coordinated with later cache/evidence modules, while M01 defines the dependency/invalidation hooks.


## Round 5 - Lifecycle safety, crash recovery and shutdown

## RLC formalization

Runtime states:
- CREATED;
- VALIDATING;
- BOOTSTRAPPING;
- SYNCHRONIZING;
- READY;
- DEGRADED;
- BLOCKED;
- DRAINING;
- STOPPED;
- FAILED.

Every transition is evaluated as:
```text
TransitionDecision =
  current_state
  + requested_transition
  + invariant_set
  + generation_tuple
  + capability_matrix
  + active_lease_summary
  + deadline_budget
  -> ALLOW | DENY | DEFER
```

ALLOW produces a TransitionReceipt. DENY produces a typed reason. DEFER identifies the condition/deadline required before reevaluation.

No state mutation occurs before its journal intent is durably recorded when the transition is safety-relevant.

## Transition classes

### Pure transitions
No external side effect; can be recomputed safely.

### Reconciled transitions
Depend on external/provider state and require current generation/fingerprint validation.

### Destructive/terminal transitions
Drain, forced cancellation, quarantine or shutdown. Require explicit receipt and journal record.

## Runtime Journal record families

M01 journal records only:
- runtime boot epoch opened/closed;
- transition intent/commit;
- configuration generation activated;
- module graph generation activated;
- capability binding generation activated;
- worker spawned/observed terminated;
- drain started/completed;
- forced termination marker;
- incomplete prior shutdown marker;
- recovery reconciliation result.

It MUST NOT contain prompts, model outputs, source code payloads, Work/Run/Step business state or secrets.

## Journal durability model

Records are append-only, checksummed and sequence-numbered.

Safety-relevant commit protocol:
1. canonicalize record;
2. checksum;
3. append;
4. flush according to durability class;
5. only then expose committed transition/binding generation.

Durability classes:
- MEMORY_ONLY_DIAGNOSTIC;
- FLUSH_REQUIRED;
- SYNC_REQUIRED.

Only state whose loss could cause unsafe replay/false readiness uses SYNC_REQUIRED.

## New technology - SBR

### SBR - Safe Boot Reconciliation
On restart, CORE does not blindly replay previous actions. It reconstructs only runtime safety facts, inspects live OS/process/provider state, compares generations, and classifies prior state:
- CLEAN_STOP;
- RECOVERABLE_INTERRUPTION;
- ORPHANED_RESOURCE;
- STALE_EXTERNAL_STATE;
- AMBIGUOUS_EFFECT;
- CORRUPT_JOURNAL.

Ambiguous external effects are never assumed successful. Later execution modules must reconcile them with their own evidence/idempotency contracts.

## New technology - EEB

### EEB - Execution Epoch Barrier
Every supervisor lifetime receives a monotonic boot epoch. Runtime-owned leases, worker identities and transition receipts are epoch-bound.

Artifacts from an older epoch cannot silently mutate current runtime state. They may be inspected as evidence but require explicit reconciliation before adoption.

Benefits:
- rejects late worker messages after restart;
- prevents stale cancellation/health updates;
- strengthens cache/evidence provenance;
- simplifies crash recovery.

## New technology - IES

### IES - Idempotency Envelope Standard
M01 defines a generic envelope for later side-effecting modules:
- operation identity;
- intent fingerprint;
- idempotency key;
- generation tuple;
- precondition fingerprint;
- expected postcondition;
- reconciliation method.

M01 does not execute business side effects. It standardizes the safety carrier so M12/M13/M20/M21 can avoid blind duplicate execution after crashes.

## QDS formalization

Shutdown phases:
```text
DRAIN_REQUESTED
 -> ADMISSION_CLOSED
 -> LEASE_DRAIN
 -> COOPERATIVE_CANCEL
 -> CLEANUP
 -> QUIESCENCE_CHECK
 -> STOP_COMMIT
```

If deadlines expire:
```text
... -> ESCALATE
    -> FORCE_TERMINATE
    -> INCOMPLETE_SHUTDOWN_RECEIPT
    -> STOPPED_WITH_RESIDUALS
```

`STOPPED_WITH_RESIDUALS` is represented in the final shutdown receipt/recovery marker; externally the process is stopped, but the next boot must reconcile residuals before READY.

## Quiescence definition

A runtime is quiescent only when:
- no new work/admission is accepted;
- no active critical capability lease remains;
- all in-process modules reached their declared safe boundary;
- isolated workers exited or were explicitly force-terminated;
- journal safety records are committed;
- required cleanup obligations are either satisfied or recorded as residuals.

## New technology - QVM

### QVM - Quiescence Verification Matrix
Machine-readable matrix of shutdown obligations per module/worker/capability. QDS computes shutdown completion from the matrix instead of relying on best-effort hooks.

## BSR formalization

Bootstrap Safety Receipt includes:
- boot epoch;
- runtime/build identity;
- config generation/fingerprint;
- module graph generation/fingerprint;
- capability graph generation/fingerprint;
- DCM summary;
- journal recovery classification;
- required invariant results;
- blocked/degraded reasons;
- timestamp as metadata only;
- RSG fingerprint;
- receipt schema version.

READY requires BSR verdict READY_ELIGIBLE.
DEGRADED requires BSR verdict DEGRADED_ELIGIBLE plus explicit unavailable capability set.
BLOCKED never produces a success receipt.

## Crash containment

### In-process module panic
Panic is caught at supervision boundaries where technically safe. A panic cannot be treated as a normal error. Critical module panic can transition runtime to FAILED/DRAINING by policy.

### Child worker crash
Supervisor records observed exit, revokes new leases, computes SIR, updates DCM, and either restarts under bounded policy or quarantines.

### Crash-loop protection
Worker restart policy has:
- bounded retry window;
- exponential backoff;
- jitter only in scheduling metadata, never cache identity;
- crash fingerprint grouping;
- quarantine threshold;
- manual/policy reset path.

## New technology - CFS

### CFS - Crash Fingerprint Suppression
Repeated equivalent crashes are grouped by deterministic crash fingerprint. CORE avoids generating repetitive expensive diagnostics/LLM analysis for the same verified failure basis.

A changed code/config/provider generation can invalidate the suppression key.

Expected benefit: fewer repeated model calls, logs and diagnosis tokens during crash loops.

## Recovery and LLM economics

Recovery path is deterministic-first:
1. journal integrity;
2. OS/process inspection;
3. generation comparison;
4. known crash fingerprint lookup;
5. existing evidence/cache lookup;
6. targeted deterministic diagnostics;
7. only later modules may request LLM diagnosis if uncertainty remains.

No LLM call is permitted merely because CORE restarted after a crash.

## Failure injection scenarios

Required:
- kill -9/TerminateProcess during each startup phase;
- crash after journal intent but before transition commit;
- crash after commit before external observer receives receipt;
- truncated final journal record;
- checksum corruption;
- stale worker message from previous epoch;
- worker crash loop;
- HIVE disconnect during SYNCHRONIZING;
- HIVE disconnect during active capability lease;
- config generation changes during bootstrap;
- shutdown with hung in-process task;
- shutdown with unresponsive child worker;
- process termination during QDS cleanup;
- disk full/read-only journal path;
- clock jumps forward/backward;
- duplicate IPC frame delivery.

## Time semantics

Correctness uses monotonic clocks for durations/deadlines. Wall clock is diagnostic metadata only unless a higher module explicitly requires calendar semantics.

No cache/fingerprint identity depends on wall-clock time by default.

## M01 durability boundary reaffirmed

M01 can determine whether runtime safety state is clean, interrupted or ambiguous. It MUST NOT infer that a higher-level Codex/Git/GitHub/tool operation succeeded after a crash. That proof belongs to the module that owns the effect.


## Round 6 - Health, degradation and production observability

## Health model

M01 health is multi-dimensional, not boolean.

Dimensions:
- PROCESS_LIVENESS;
- RUNTIME_READINESS;
- MODULE_GRAPH_HEALTH;
- CAPABILITY_HEALTH;
- JOURNAL_INTEGRITY;
- RESOURCE_PRESSURE;
- EXTERNAL_DEPENDENCY_HEALTH;
- SHUTDOWN_SAFETY.

Each dimension reports:
- state: HEALTHY | DEGRADED | UNAVAILABLE | UNKNOWN;
- reason code;
- affected generation;
- observed_at metadata;
- evidence/reference;
- impact class.

## DCM formalization

DCM maps unavailable/degraded capabilities to allowed operation classes.

It answers:
- what still works?
- what is blocked?
- what can use fallback?
- what requires revalidation?
- what quality floor is now achievable?

Runtime-wide DEGRADED is derived from capability impact, not arbitrary warning count.

## New technology - QFC

### QFC - Quality Floor Continuity
Every capability/provider advertises an assurance/quality envelope. Operations declare a minimum quality floor.

Failover is permitted only if:
```text
fallback_effective_quality >= operation_required_quality
```

Quality includes contract features, assurance class and policy constraints, not a subjective model score.

This prevents token/cost optimizers from selecting a cheaper fallback that invalidates correctness requirements.

## New technology - HCC

### HCC - Health Causality Compression
Health aggregation stores a causal root plus affected dependents instead of repeating the same verbose diagnostic across every module.

Example:
```text
ROOT: HIVE provider unavailable
AFFECTS: context.full, memory.verified, decision.fabric
DERIVED: M14 verification enrichment degraded
```

Benefits:
- smaller telemetry/evidence;
- smaller future LLM diagnostic context;
- clearer root-cause analysis;
- less duplicate token input.

## New technology - ODF

### ODF - Observability Delta Frames
Health/runtime snapshots are emitted as a stable baseline fingerprint plus changed fields/events rather than full repeated snapshots.

Full snapshot remains available on demand.

Expected effect: lower event volume, storage and LLM/console context size.

## Production resource pressure

M01 monitors process-level pressure signals needed for runtime safety:
- memory pressure;
- CPU saturation trend;
- file descriptor/handle exhaustion;
- disk/journal write failure;
- worker count;
- event/control queue pressure.

M19 will own full resource/cost governance. M01 only protects runtime safety.

Pressure policy uses bounded states:
NORMAL -> PRESSURED -> CRITICAL.

CRITICAL can close new admission or trigger controlled degradation; it must not silently kill correctness-critical work.

## Health cache policy

Health probes have explicit freshness windows. Cached health is tagged with age/generation.

A stale health result cannot authorize a safety-critical transition.

Probe deduplication/coalescing prevents many consumers from simultaneously hitting the same external provider.

## New technology - PHC

### PHC - Probe Herd Coalescing
Concurrent equivalent health/capability probes share one in-flight deterministic probe and fan out the result with provenance.

Expected effect:
- less network/CPU;
- fewer HIVE/provider calls;
- lower cascading load during incidents;
- lower duplicated diagnostic work.

## Observability privacy

Diagnostics default to identifiers, fingerprints, codes and bounded metadata.
Prompt bodies, secrets, source payloads and model outputs are not included in M01 health events.

## Production tests

- root failure affecting 1/100/1000 dependents;
- DCM derivation correctness;
- QFC failover acceptance/rejection;
- stale health cannot authorize transition;
- probe herd of 1/10/1000 concurrent consumers;
- ODF delta reconstruction;
- pressure transition hysteresis;
- disk failure during health/journal operation;
- memory pressure while draining;
- health query during generation substitution;
- diagnostic redaction/property tests.

## Round 6 status

M01 now has sufficient architectural direction for runtime, lifecycle, capability substitution, crash recovery, shutdown and health. Remaining deep-planning areas before freeze:
1. exact Rust workspace/file map;
2. concrete schemas/contracts;
3. benchmark thresholds and regression policy;
4. dependency/supply-chain policy;
5. complete test/eval matrix;
6. M01 Definition of Done;
7. final technology promotion/rejection;
8. executor Work Order compilation.


## Round 7 - Construction boundaries

M01 Rust workspace is frozen into nine responsibility boundaries: core-contracts, core-identity, core-config, core-registry, core-journal, core-ipc, core-health, core-runtime and core-cli. The dependency graph MUST remain acyclic, with contracts/identity below orchestration. No M01 crate may depend on HIVE source, LLM provider SDKs, GitHub SDKs, Redis/database servers, UI frameworks, Kubernetes or arbitrary dynamic plugin loaders.

Public M01 CLI: `core start`, `core status --json`, `core validate --json`, `core doctor --json`, `core version --json`. Machine output is schema-versioned; human output is never canonical evidence.

## Round 8 - Contract-first construction

The executor implements versioned contracts before orchestration: RuntimeGeneration, ModuleManifest, CapabilityRequirement, CapabilityProviderDescriptor, CapabilityLease, TransitionReceipt, BootstrapSafetyReceipt, HealthSnapshot, RuntimeJournalRecord and IdempotencyEnvelope.

Identity rules: DCS owns canonical bytes; identity maps are sorted; wall-clock metadata is excluded from fingerprints by default; durations use integer units; floating point is forbidden from safety/cache identity unless explicitly normalized; unknown security-critical fields fail closed. Persisted/exchanged records carry schema versions. Required semantic changes require major contract bumps.


## Round 9 - Performance, cache and regression governance

### Benchmark philosophy
M01 performance is a governed product property. Benchmarks produce versioned evidence bound to commit/build/hardware class/configuration. Absolute targets are calibrated from reproducible baselines; relative regressions can block promotion before absolute targets are finalized.

### Benchmark suites
- BOOT: cold/warm bootstrap, validation, synchronization-disabled startup;
- REG: module graph admission, capability lookup, binding/substitution, SIR computation;
- ID: DCS serialization, DIF/RSG/BSR fingerprint generation, content-handle operations;
- JRN: append/flush/sync, recovery scan, corruption detection, compaction;
- IPC: frame encode/decode, local round trip, throughput, large-handle reference path;
- HLT: health aggregation, DCM derivation, PHC coalescing, ODF generation;
- CAN: cancellation propagation and deadline enforcement;
- SDN: clean drain, forced shutdown and QVM evaluation;
- MEM: idle memory, per-module/per-capability growth, leak/soak slope;
- CPU: idle CPU and sustained control-plane overhead.

### Synthetic scale classes
Benchmark graphs at 10, 100, 1,000 and 10,000 modules/capabilities where the tested structure supports that scale. Results report p50/p95/p99 where latency distributions matter.

### Regression policy
A benchmark result is compared only with a compatible baseline fingerprint. Material regression thresholds are stored as policy, not hard-coded into test logic. A regression above the accepted budget blocks promotion unless accompanied by an approved evidence-backed exception/ADR.

### New technology - PRB
**PRB - Performance Regression Budget** binds each hot path to a baseline, allowed regression envelope and hardware class. Improvements can accumulate performance credit, but credit cannot justify violating safety/quality constraints.

### New technology - WNF
**WNF - Work Normalization Fingerprint** fingerprints benchmark workload shape so two measurements are compared only when dataset/graph/config/workload semantics are equivalent.

### New technology - CEE
**CEE - Cache Efficiency Envelope** captures cache economics for later LLM-facing modules: stable-prefix ratio, exact/evidence/semantic/provider-cache hit classes, invalidation rate, avoidable token repetition, lookup overhead and stale-reuse rejection. M01 defines the metric carrier and identity hooks; later modules populate LLM-specific measurements.

### Token/cache regression rules
For LLM-facing future modules, promotion evidence must eventually report at least:
- total input tokens;
- stable/cache-eligible input tokens;
- uncached delta tokens;
- output tokens;
- retry/escalation tokens;
- tokens avoided by exact/evidence/semantic reuse;
- cache hit/miss/bypass/invalidation counts;
- PSM stable-prefix ratio.
A change that materially increases repeated uncached tokens without a documented quality/correctness benefit is a regression candidate.

### Benchmark correctness
Benchmarks MUST NOT disable integrity/security checks merely to obtain attractive numbers unless the benchmark is explicitly labeled microbenchmark-only. Production-path benchmark profiles preserve production safety semantics.

### CI tiers
- FAST: deterministic unit/property smoke + selected microbench regression signals;
- STANDARD: integration + representative benchmarks + leak checks;
- DEEP: large graphs, fuzz campaigns, soak, crash/failure injection, cross-platform IPC;
- RELEASE: exact-head evidence, supply-chain checks, reproducible benchmark corpus and accepted PRB status.

### Hardware normalization
Evidence records CPU architecture/model class, cores, RAM, OS/kernel, storage class, Rust/toolchain and power mode when available. Results from incompatible hardware are not compared as if equivalent.

### Performance anti-patterns forbidden
- LLM calls in M01 hot paths;
- full graph recomputation for local changes when incremental correctness is possible;
- full snapshot serialization when a stable handle/delta suffices;
- polling loops when event/notification semantics are available;
- unbounded queues;
- unbounded retries;
- copying large immutable payloads across components without need;
- synchronous blocking I/O on Tokio executor threads.

## Round 10 - Supply-chain and runtime security baseline

### Rust dependency policy
Dependencies require an explicit purpose and must minimize feature sets. Direct dependencies are preferred only when they remove meaningful implementation/security risk. No dependency is added solely to avoid writing a small deterministic primitive.

### Supply-chain gates
- lockfile committed;
- dependency license policy;
- vulnerability/advisory scan;
- banned/duplicate dependency policy where justified;
- source/provenance review for critical crates;
- feature minimization;
- unsafe-code inventory;
- reproducible toolchain pin;
- SBOM generation at release;
- artifact checksum/provenance evidence.

### Unsafe Rust policy
Safe Rust is default. Any `unsafe` block in first-party M01 code requires:
- localized boundary;
- written safety invariant;
- dedicated tests;
- reviewer-visible evidence;
- no equivalent safe implementation with acceptable measured cost.

### New technology - TCBM
**TCBM - Trusted Computing Base Map** produces a machine-readable map of crates/modules that can affect lifecycle safety, capability authority, canonical identity, journal integrity or IPC trust boundaries. Security review focuses deeply on this smaller TCB rather than treating every file equally.

### New technology - SAF
**SAF - Security-Aware Fingerprint** extends dependency/cache impact metadata so a security-policy or TCB-relevant dependency change invalidates affected assurance/evidence even when functional APIs appear unchanged.

### Runtime hardening requirements
- deny-by-default authority;
- bounded frame/message sizes;
- parser/resource limits before allocation;
- no shell interpolation for process execution contracts;
- path normalization and traversal defenses at future file boundaries;
- child environment allowlisting/redaction;
- secret zeroization where practical for owned buffers;
- crash/error messages must not leak secrets;
- IPC peer identity/authentication hooks;
- journal permissions/integrity validation;
- symlink/race-aware file operations where safety-relevant;
- no network listener in M01 by default.

### Security testing
- fuzz all untrusted parsers/frames/config records;
- malformed length-prefix and allocation-bomb tests;
- path/config injection tests;
- environment secret-canary tests;
- stale/forged epoch and binding receipt tests;
- journal tamper/truncation/reordering tests;
- IPC peer spoofing tests where platform permits;
- dependency/feature drift checks;
- unsafe-code audit gate;
- privilege/authority escalation property tests.


## Round 11 - Complete verification matrix

### Static/compile
- rustfmt check;
- clippy with governed deny policy;
- warnings-as-errors for first-party M01 targets;
- dependency graph cycle/forbidden-dependency validation;
- schema/golden-vector compatibility checks;
- unsafe inventory.

### Unit/property
- exhaustive legal/illegal RLC transitions;
- DCS canonical equivalence/non-equivalence;
- config precedence/provenance;
- generation monotonicity;
- semver/feature compatibility;
- CPG/CAL/SIR binding properties;
- QFC fallback rules;
- DCM derivation;
- journal hash chain/integrity;
- QVM completeness.

### Integration
- clean bootstrap to READY;
- valid degraded bootstrap;
- blocked bootstrap;
- HIVE absent standalone path;
- external provider attach/detach simulation;
- atomic substitution under concurrent readers;
- active lease across provider change;
- worker crash/restart/quarantine;
- clean and forced shutdown;
- journal restart reconciliation;
- local IPC on Windows and Unix families.

### Cache/token architecture
- stable fields produce byte-identical canonical material;
- volatile diagnostic changes do not churn semantic identity;
- correctness-relevant changes always alter affected identity;
- SIR/CAG hook invalidation scope is minimal and sufficient;
- PSM/CEE carrier schemas are deterministic;
- bootstrap/lifecycle execute zero LLM calls.

### Adversarial/fuzz
- config, manifest, IPC, journal, compatibility inputs;
- malformed/truncated/oversized frames;
- crash at transition boundaries;
- stale epoch messages;
- forged receipts;
- concurrent substitution/cancellation/shutdown;
- disk and permission failures.

### Soak
Repeated start/stop, worker churn, provider flap, config safe reload and health probing must show no unbounded resource growth or generation corruption.

## Round 12 - M01 Definition of Done candidate

M01 is complete only when:
1. all accepted M01 crates/contracts are implemented;
2. exact lifecycle/RLC transitions are machine-enforced;
3. BSR prevents false READY;
4. module/capability registries support deterministic atomic bindings and leases;
5. standalone fallback/HIVE-external provenance is represented without HIVE source dependency;
6. DCS/generation/fingerprint golden vectors pass;
7. Runtime Journal survives tested interruption/corruption scenarios or fails closed;
8. SBR/EEB prevent stale prior-epoch mutation and blind replay;
9. QDS/QVM distinguish clean versus residual shutdown;
10. DCM/QFC health degradation blocks unsafe low-quality fallback;
11. M01 hot paths contain zero LLM inference;
12. cache/token identity hooks are deterministic and observable;
13. IPC is bounded/versioned and cross-platform tested;
14. security/supply-chain gates pass with no unresolved HIGH/CRITICAL finding;
15. fuzz/property/integration/failure-injection suites pass at accepted thresholds;
16. PRB-compatible benchmarks show no unapproved material regression;
17. idle/soak tests show no unbounded leak/growth;
18. CLI machine outputs are versioned and tested;
19. exact-head evidence bundle is complete;
20. independent governed review returns APPROVED;
21. canonical checkpoint is promoted only after accepted evidence.

## M01 explicit OUT OF SCOPE
- agent orchestration and specialist selection;
- model/provider routing;
- actual LLM invocation/cache provider implementation;
- full context compiler/RAG/memory;
- durable Run/Attempt/Step execution database;
- tool/command business execution;
- Git/GitHub delivery;
- full ENS durable telemetry spine;
- dashboard/TUI/web UI;
- distributed cluster control plane;
- arbitrary in-process plugin ecosystem.

## M01 planning status after Round 12
The architecture is close to planning freeze. Remaining tasks are final technology disposition, exact benchmark-policy seed values after first baseline, executor file-by-file implementation notes, and Work Order/STOP CONDITION compilation.


## Round 13 - Technology disposition and consolidation

### PROMOTED_TO_M01_REQUIRED
These mechanisms are distinct, measurable and belong in M01:
- RLC — Runtime Lifecycle Calculus;
- CPG — Capability Provenance Graph;
- RSG — Runtime Safety Genome;
- QDS — Quiescence-Driven Shutdown;
- DCM — Degraded Capability Matrix;
- BSR — Bootstrap Safety Receipt;
- GCL — Generation Coherence Layer;
- DCS — Deterministic Canonical Serialization;
- CAL — Capability Atomic Leasing;
- SIR — Substitution Impact Radius;
- CBR — Capability Binding Receipt;
- FCH — Fallback Capability Harness;
- SBR — Safe Boot Reconciliation;
- EEB — Execution Epoch Barrier;
- IES — Idempotency Envelope Standard (contract only in M01; effect execution later);
- QVM — Quiescence Verification Matrix;
- CFS — Crash Fingerprint Suppression;
- QFC — Quality Floor Continuity;
- HCC — Health Causality Compression;
- ODF — Observability Delta Frames (M01 control/health subset);
- PHC — Probe Herd Coalescing;
- PRB — Performance Regression Budget;
- WNF — Work Normalization Fingerprint;
- TCBM — Trusted Computing Base Map;
- SAF — Security-Aware Fingerprint.

### PROMOTED_AS_CROSS_MODULE_PRIMITIVE
M01 implements the primitive/contract seam; later modules own full product behavior:
- SCP — Stable Context Partitioning;
- CAG — Cache Affinity Graph hooks/dependency identity;
- DIF — Deterministic Input Fingerprint;
- TEB — Token Economics Budget carrier;
- ZCP — immutable content-handle contract/local primitive;
- CEE — Cache Efficiency Envelope carrier/metrics schema.

### DEFERRED_IMPLEMENTATION_WITH_REQUIRED_SEAM
These remain accepted CORE technologies but their active engine does not belong in M01:
- LCR — LLM Call Reuse Gate: active decision engine belongs with LLM execution/routing/cache modules;
- PSM — Prompt Stability Meter: active measurement belongs with prompt/context compilation.

M01 MUST preserve the stable partition/fingerprint/generation/metric hooks they require.

### CONSOLIDATION RULES
- DIF uses DCS; it is not a second serializer.
- RSG uses DCS + SAF-relevant inputs; it is not a second dependency graph.
- BSR references RSG/generations/health; it does not duplicate them.
- CBR is the receipt produced by capability binding; CAL is the lease consumed afterward.
- SIR consumes CPG/GCL/CAG hooks; it does not maintain a parallel graph.
- ODF compresses observation transport; HCC compresses causal representation. They remain distinct.
- QDS consumes QVM; QVM is the proof matrix, QDS is the shutdown algorithm.
- SBR consumes journal + EEB + generations; it is not a second journal.
- CEE measures cache economics; TEB constrains budget. They are not interchangeable.

### Rejection rule
No candidate is kept merely because its acronym exists. If implementation evidence shows a promoted mechanism has no independent measurable value, it MUST be merged/narrowed/rejected through an ADR rather than preserved for branding.

## M01 technology count after consolidation
- 25 M01-required mechanisms;
- 6 cross-module primitives seeded by M01;
- 2 accepted later engines with mandatory M01 seams.

This is a responsibility map, not a claim of patents or external novelty.


## Round 14 - Executor construction packets

To minimize Codex rediscovery, M01 implementation is internally partitioned into construction packets while remaining ONE Work Order and ONE module delivery.

### Packet A - Contracts and deterministic identity
Owns: core-contracts + core-identity.
Implement contract types, schema versions, DCS, DIF, generations, RSG/SAF inputs, ZCP handle primitive, canonical golden vectors.
STOP-A: canonical identity/property tests pass.

### Packet B - Configuration and admission
Owns: core-config + module admission contracts.
Implement TOML/env/CLI precedence, provenance, redaction, static/dynamic/secret-reference classification, generation creation, manifest validation.
STOP-B: invalid/ambiguous safety config fails closed; secret canaries pass.

### Packet C - Registries and substitution fabric
Owns: core-registry.
Implement dependency graph, CPG, compatibility, deterministic resolution, CAL, SIR, CBR, FCH and CAG hooks.
STOP-C: concurrent atomic substitution/lease/property tests pass.

### Packet D - Journal and recovery
Owns: core-journal + SBR/EEB persistence primitives.
Implement hash-chain records, durability classes, recovery classification, corruption handling, bounded compaction.
STOP-D: interruption/corruption matrix passes.

### Packet E - IPC and worker boundary
Owns: core-ipc + worker protocol contracts.
Implement bounded framed local IPC, handshake/version negotiation, epoch binding, platform transports and malformed-frame defenses.
STOP-E: cross-platform protocol tests and fuzz targets pass.

### Packet F - Health/degradation
Owns: core-health.
Implement DCM, QFC, HCC, PHC, ODF-M01, pressure state and freshness semantics.
STOP-F: degradation/fallback/probe-herd/delta reconstruction tests pass.

### Packet G - Runtime/lifecycle
Owns: core-runtime.
Implement RLC, supervisor, bootstrap, BSR, synchronization seam, cancellation tree, worker supervision, QDS/QVM, CFS and control events.
STOP-G: bootstrap/crash/shutdown/failure-injection suites pass and lifecycle has zero LLM calls.

### Packet H - CLI and production evidence
Owns: core-cli + benches/fuzz/integration/release evidence wiring.
Implement stable exit codes/JSON schemas, doctor/validate/status/version/start, PRB/WNF benchmark metadata, TCBM/supply-chain evidence hooks.
STOP-H: full M01 DoD evidence is generated at exact head.

## Executor rules
- packets are sequencing aids, not separate architecture decisions;
- one packet may not redesign an accepted earlier packet silently;
- failures feed correction into the owning packet;
- Codex should read only canonical M01 sources + required dependency contracts for each packet where possible;
- stable packet instructions should remain byte-stable across corrections; corrections are appended as deltas;
- no broad repository rediscovery between packets unless a fingerprint/generation proves the basis changed;
- reuse test/build evidence when validity fingerprints still match.

## Token-efficient executor context strategy
Each packet context should be assembled as:
```text
STABLE PREFIX
  product invariants
  M01 accepted decisions
  packet contract/file rules
  security/quality floor

DELTA
  current packet task
  changed files
  failing evidence
  correction requirements

REFERENCES/HANDLES
  large canonical docs/evidence not needed inline
```

This structure is mandatory input to future HIVE/CORE prompt-cache optimization.

## M01 planning freeze readiness
The module can enter PLANNING_FREEZE after:
- exact file-level notes are attached to packets where needed;
- initial benchmark policy is expressed as baseline-relative gates rather than fabricated absolute numbers;
- Work Order acceptance/STOP is compiled;
- canonical checkpoint points to the frozen M01 basis.


## Completion record
M01 was implemented under `CORE-WO-M01-001`, passed the final governed Review 011 at exact head `aac0f143ea576a11013e4346076b8b3b4bd24282`, and was promoted by PR #17 as merge `d70b4296afbba93e8849ab6160e9b1caf5281e7d`. The final evidence bundle is `docs/evidence/M01-EXECUTION-REPORT.md`. M01 is now a frozen upstream contract for later modules unless changed through a governed ADR/correction process.
