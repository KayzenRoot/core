# M01 - Core Runtime & Lifecycle

Status: `DISCOVERY_IN_PROGRESS`

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
