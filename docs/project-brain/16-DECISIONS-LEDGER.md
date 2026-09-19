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


## CORE-D-016 - LLM economics and cache are architecture concerns
**Decision:** token efficiency and LLM caching are cross-cutting architecture requirements, not late optimizations. Designs preserve stable prefixes, deterministic identities, delta reuse and correct invalidation.
**State:** ACCEPTED

## CORE-D-017 - Zero-LLM runtime lifecycle
**Decision:** bootstrap, lifecycle, module/capability resolution, configuration validation, base health and shutdown require zero LLM calls.
**State:** ACCEPTED

## CORE-D-018 - Cache cannot weaken correctness
**Decision:** exact/provider/semantic/evidence caches are derived optimization layers. Reuse requires provenance and correctness-relevant invalidation; a cache miss must not change functional correctness.
**State:** ACCEPTED


## CORE-D-019 - Rust-first CORE runtime
**Decision:** CORE runtime kernel and first-party runtime workers use Rust stable with Tokio. Python/TypeScript may be used at adapter/SDK edges when justified but are not mandatory runtime dependencies.
**State:** ACCEPTED

## CORE-D-020 - Selective process isolation
**Decision:** trusted modules may run in-process; risky/external/sandboxed work uses explicit process/provider boundaries. Same-machine first-party IPC uses local OS IPC rather than TCP by default.
**State:** ACCEPTED

## CORE-D-021 - Minimal M01 durability
**Decision:** M01 owns only an append-only runtime-safety journal. Durable Work/Run/Attempt/Step state belongs to M04.
**State:** ACCEPTED

## CORE-D-022 - Extensibility without arbitrary plugins
**Decision:** arbitrary dynamic-library plugin loading is rejected. Extensibility uses versioned process/API/MCP/stdio/IPC adapters and capability contracts.
**State:** ACCEPTED

## CORE-D-023 - TOML configuration
**Decision:** human-authored CORE repository configuration uses TOML with typed precedence; machine contracts/evidence use canonical schema-governed serialization with interoperable JSON representation.
**State:** ACCEPTED


## CORE-D-024 - Modules and capabilities are separate identities
**Decision:** consumers depend on versioned capability contracts, not concrete module/provider identities by default. Modules are runtime participants; capabilities are behavior contracts.
**State:** ACCEPTED

## CORE-D-025 - Atomic capability substitution
**Decision:** provider changes use prepared generation plus atomic binding publication and bounded leases. Active operations retain coherent provider identity until a safe boundary unless policy requires revocation.
**State:** ACCEPTED

## CORE-D-026 - No silent low-quality fallback
**Decision:** HIVE/provider loss may activate a standalone fallback only when that fallback satisfies the operation's declared quality/policy floor. Otherwise the affected operation is blocked/degraded explicitly.
**State:** ACCEPTED

## CORE-D-027 - Targeted invalidation on provider change
**Decision:** provider substitution does not globally invalidate caches/evidence. Only artifacts whose correctness identity depends on the changed provider/binding generation are invalidated.
**State:** ACCEPTED


## CORE-D-028 - Crash recovery never guesses success
**Decision:** after restart, ambiguous external effects are not assumed successful and are not blindly replayed. M01 reconciles runtime safety only; effect-owning modules must prove/reconcile their operations.
**State:** ACCEPTED

## CORE-D-029 - Epoch-bound runtime state
**Decision:** each supervisor lifetime has a boot epoch. Runtime leases, worker identities and safety receipts are epoch-bound so stale prior-process messages cannot mutate current state.
**State:** ACCEPTED

## CORE-D-030 - Quiescence-driven shutdown
**Decision:** graceful shutdown closes admission, drains leases, cancels cooperatively, verifies quiescence and records residuals before termination. Forced shutdown cannot masquerade as clean shutdown.
**State:** ACCEPTED

## CORE-D-031 - Deterministic-first crash diagnosis
**Decision:** restart/crash recovery uses journal/process/generation/fingerprint/evidence checks before any later LLM diagnosis. Equivalent crash bases may reuse verified diagnosis rather than repeatedly spending tokens.
**State:** ACCEPTED
