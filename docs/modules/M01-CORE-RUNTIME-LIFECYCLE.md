# M01 - Core Runtime & Lifecycle

Status: `DISCOVERY_IN_PROGRESS`

## Mission

Define the smallest durable headless runtime that all later CORE modules can depend on without embedding HIVE-owned intelligence or choosing unnecessary infrastructure early.

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
