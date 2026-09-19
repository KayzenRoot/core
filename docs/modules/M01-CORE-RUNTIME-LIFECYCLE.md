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
