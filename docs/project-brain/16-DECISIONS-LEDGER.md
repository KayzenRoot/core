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


## CORE-D-032 - M01 Rust workspace boundaries
**Decision:** M01 separates contracts, identity, config, registry, journal, IPC, health, runtime and CLI into acyclic Rust crate boundaries.
**State:** ACCEPTED

## CORE-D-033 - Contracts before orchestration
**Decision:** M01 implementation starts with versioned contracts and canonical identity vectors before supervisor behavior.
**State:** ACCEPTED

## CORE-D-034 - Canonical identity is explicit
**Decision:** safety/cache/evidence fingerprints use CORE DCS rules rather than serializer-default byte output.
**State:** ACCEPTED


## CORE-D-035 - Performance is promotion evidence
**Decision:** CORE performance claims require reproducible benchmark evidence. Material regression beyond accepted PRB policy blocks promotion unless explicitly governed by an evidence-backed exception.
**State:** ACCEPTED

## CORE-D-036 - LLM/cache economics are measured
**Decision:** LLM-facing modules must expose token/cache efficiency metrics sufficient to identify repeated uncached work, reuse and invalidation. Token savings never override quality/safety floors.
**State:** ACCEPTED

## CORE-D-037 - Safe Rust default
**Decision:** first-party CORE code defaults to safe Rust. Unsafe code requires a localized documented safety invariant, tests and evidence that a safe alternative would not meet accepted requirements.
**State:** ACCEPTED

## CORE-D-038 - Security-sensitive changes affect evidence validity
**Decision:** security-policy, TCB and relevant dependency changes participate in evidence/cache validity even when public functional APIs are unchanged.
**State:** ACCEPTED


## CORE-D-039 - M01 technology consolidation
**Decision:** M01 technology candidates are consolidated into required M01 mechanisms, cross-module primitives seeded by M01, and later engines with required M01 seams. Overlapping implementations are forbidden where one canonical primitive can serve multiple technologies.
**State:** ACCEPTED

## CORE-D-040 - Technology names do not override evidence
**Decision:** NexLabs technology branding does not protect a mechanism from narrowing, merging or rejection. Independent measurable value and production evidence govern final retention.
**State:** ACCEPTED


## CORE-D-041 - M02 is an adapter, not a second Project Registry
**Decision:** M02 owns local action-plane workspace binding and reconciliation. HIVE retains canonical Project Registry intelligence.
**State:** ACCEPTED

## CORE-D-042 - Workspace action requires explicit basis
**Decision:** later execution may not rely on ambient current-directory assumptions. A validated WorkspaceHandle/basis is required before action.
**State:** ACCEPTED

## CORE-D-043 - Project, workspace, repository and worktree identities remain separate
**Decision:** M02 models these as distinct typed identities linked by explicit relations rather than collapsing them into an absolute path.
**State:** ACCEPTED

## CORE-D-044 - Git/filesystem state and HIVE identity are different authorities
**Decision:** HIVE may authoritatively identify the registered project; local Git/filesystem evidence authoritatively describes the concrete checkout. Conflict is explicit and blocks unsafe attachment rather than being silently merged.
**State:** ACCEPTED

## CORE-D-045 - M02 is read-only with respect to source and Git mutation
**Decision:** M02 may inspect Git/filesystem state for basis establishment but mutation belongs to M13 and Git/GitHub delivery belongs to M20/M21.
**State:** ACCEPTED

## CORE-D-046 - Path authority precedes sandboxing
**Decision:** M02 performs deterministic path-within-authority validation and escape detection. M11 later adds runtime sandbox/capability enforcement; M02 must not pretend path validation is a complete sandbox.
**State:** ACCEPTED

## CORE-D-047 - Workspace drift is a correctness event
**Decision:** correctness-relevant changes to repository/worktree/config/security basis invalidate or require revalidation of affected workspace handles before action.
**State:** ACCEPTED

## CORE-D-048 - M02 workspace discovery is zero-LLM
**Decision:** identity, Git basis, path normalization, boundary resolution, HIVE/local reconciliation and drift detection are deterministic-first and require no inference.
**State:** ACCEPTED


## CORE-D-049 - M02 state machine keeps ambiguity as typed evidence
**Decision:** M02 top-level binding states are UNBOUND, DISCOVERING, VALIDATING, BOUND, DRIFTED, REVALIDATING, BLOCKED and DETACHING. Ambiguity/not-found/conflict are typed reasons, not hidden lifecycle states.
**State:** ACCEPTED

## CORE-D-050 - Live workspace handles are runtime-bound snapshots
**Decision:** WorkspaceHandle is immutable, runtime-epoch-bound and non-authoritative outside freshness validation. Durable WorkspaceBindingReceipt is evidence, not a filesystem capability.
**State:** ACCEPTED

## CORE-D-051 - Workspace basis is componentized
**Decision:** M02 represents workspace identity, authority, repository graph, Git state, config/security and project association as explicit basis components. Correctness fingerprints exclude diagnostic-only volatility.
**State:** ACCEPTED

## CORE-D-052 - Local RepositoryId is not a remote URL
**Decision:** repository identity uses local Git/common-dir and boundary evidence. Remote URLs are mutable association hints and never the sole repository identity.
**State:** ACCEPTED

## CORE-D-053 - HIVE availability does not churn local workspace identity
**Decision:** temporary HIVE availability/provider changes do not rewrite WorkspaceId. Association evidence and assurance may change the binding generation or block HIVE-required operations.
**State:** ACCEPTED

## CORE-D-054 - Compatible drift still invalidates the old handle
**Decision:** successful revalidation after correctness-relevant drift emits a new WorkspaceGeneration/WorkspaceHandle; the stale handle is never silently revived.
**State:** ACCEPTED

## CORE-D-055 - Untracked-file policy is part of correctness evidence
**Decision:** treatment of untracked files is explicit in WorkspaceBasis. CONTENT_HASHED is the safe general execution default unless a governed downstream profile proves a narrower policy sufficient.
**State:** ACCEPTED

## CORE-D-056 - Path proof uses lexical plus physical validation
**Decision:** M02 validates both lexical containment and physical symlink/junction/reparse resolution. Non-existing targets bind the nearest existing ancestor and require use-time revalidation.
**State:** ACCEPTED

## CORE-D-057 - M02 cannot eliminate TOCTOU alone
**Decision:** path/workspace validation receipts record staleness/use-time requirements. M11/M13/M20 must revalidate or enforce at their actual action boundary.
**State:** ACCEPTED

## CORE-D-058 - Delta revalidation requires equivalence proof
**Decision:** DWS/BVM optimizations may replace full rescans only where tests/benchmarks prove equivalent correctness for the affected basis components.
**State:** ACCEPTED


## CORE-D-059 - Source and Git metadata authority are separate
**Decision:** linked-worktree/common-dir Git metadata may exist outside workspace source roots. M02 may inspect that metadata read-only but it never becomes source authority by transitivity.
**State:** ACCEPTED

## CORE-D-060 - Submodules are local evidence only in M02
**Decision:** M02 records submodule declarations/materialization but never initializes, fetches, updates or clones them.
**State:** ACCEPTED

## CORE-D-061 - Nested repositories require explicit policy
**Decision:** an independent nested repository is represented as its own graph node and must be admitted, ignored or conflicted explicitly.
**State:** ACCEPTED

## CORE-D-062 - Bare repositories do not imply source worktrees
**Decision:** a bare repository may satisfy metadata-only operation profiles but cannot satisfy a worktree/source operation.
**State:** ACCEPTED

## CORE-D-063 - HIVE association enters through a versioned capability
**Decision:** M02 consumes project association through a provider/capability contract. HIVE remains external; M23 may later replace/deepen the provider without changing M02 ownership.
**State:** ACCEPTED

## CORE-D-064 - GitInspector backend stays swappable until evidence
**Decision:** M02 freezes the security/semantic contract before selecting system Git versus Rust-native Git inspection. Backend selection is benchmark/security evidence-driven.
**State:** ACCEPTED

## CORE-D-065 - Unknown filesystem semantics are explicit
**Decision:** M02 does not guess case/alias behavior. Security-sensitive ambiguity yields UNKNOWN/blocked or requires stronger use-time proof.
**State:** ACCEPTED

## CORE-D-066 - External Git object stores require explicit admission
**Decision:** Git alternates/shared object roots outside admitted metadata authority are denied by default for execution-ready binding unless policy explicitly admits them with provenance.
**State:** ACCEPTED

## CORE-D-067 - M02 never repairs repositories
**Decision:** malformed/unsupported repository state is reported typed. Reset, checkout, index repair, submodule update and similar repair/mutation are outside M02.
**State:** ACCEPTED

## CORE-D-068 - Large-workspace optimization must preserve full-basis semantics
**Decision:** WMF/DWS component hashing may optimize revalidation only when canonical root/basis semantics and delta/full equivalence are proven.
**State:** ACCEPTED
