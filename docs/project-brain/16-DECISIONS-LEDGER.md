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


## CORE-D-069 - Watcher events are invalidation hints, not truth
**Decision:** filesystem/Git watcher events may mark WorkspaceBasis components dirty or unknown but cannot directly prove freshness or transition M02 to BOUND. Correctness survives total watcher loss through action-boundary revalidation.
**State:** ACCEPTED

## CORE-D-070 - Proof caches are derived and disposable
**Decision:** M02 may cache compact deterministic proofs, but cache contents never become canonical repository truth or filesystem authority. Cache loss/corruption must degrade performance, not correctness.
**State:** ACCEPTED

## CORE-D-071 - Cache identity includes policy and provider provenance
**Decision:** reusable proof identity includes relevant authority, filesystem semantics, policy/security/config generations and provider/backend version. Changed preconditions invalidate or bypass reuse.
**State:** ACCEPTED

## CORE-D-072 - mtime alone never proves content equality
**Decision:** timestamps/stat metadata can nominate a cache candidate but cannot alone prove unchanged correctness-relevant content. Stronger Git/content/file-identity evidence is required by policy.
**State:** ACCEPTED

## CORE-D-073 - Hashing is bounded and coalesced
**Decision:** content hashing uses streaming bounded buffers, bounded concurrency, cancellation/deadlines and safe in-flight request coalescing. Input size must not imply proportional RAM use.
**State:** ACCEPTED

## CORE-D-074 - Causal invalidation precedes selective revalidation
**Decision:** CIG deterministically maps evidence changes to affected WorkspaceBasis components. Selective revalidation is allowed only when it is no weaker than validating the full required BVM mask.
**State:** ACCEPTED

## CORE-D-075 - System Git is the semantic reference oracle, not the automatic production winner
**Decision:** provider differential tests use Git's own semantics as reference for Git repositories. CORE-D-064 remains in force: final production GitInspector selection requires comparative compatibility, security, resource and benchmark evidence.
**State:** ACCEPTED

## CORE-D-076 - Repository graph serialization is canonical
**Decision:** graph nodes/edges are deterministically sorted and schema-versioned before fingerprinting. Hash-map iteration order and diagnostic timestamps never enter semantic graph identity.
**State:** ACCEPTED

## CORE-D-077 - Authority roots are typed non-transitive records
**Decision:** authority roots carry class, stable/physical identity evidence, filesystem semantics, provenance and policy generation. Metadata/object/temp authority never expands SOURCE_AUTHORITY by transitivity.
**State:** ACCEPTED

## CORE-D-078 - Persistent proof cache is optional and not yet authorized
**Decision:** M02 requires only a bounded runtime-epoch L1 proof cache candidate. Persistent L2 caching cannot become required until corruption, recovery, invalidation and secret-safety evidence is frozen.
**State:** ACCEPTED

## CORE-D-079 - Compact workspace evidence is the downstream default
**Decision:** downstream HIVE/LLM-facing context should consume stable fingerprints, generations, component masks, deltas and evidence references instead of repeated raw path inventories/status output unless detail is explicitly required.
**State:** ACCEPTED

## CORE-D-080 - Resource numbers require benchmark calibration
**Decision:** M02 freezes typed resource-budget dimensions now, but exact numeric defaults must be calibrated from reproducible fixture/benchmark evidence rather than invented during architecture planning. Limit exhaustion fails typed and cannot yield partial BOUND.
**State:** ACCEPTED


## CORE-D-081 - M02 V0.0 uses one core-workspace crate
**Decision:** the initial module is implemented as a single focused core-workspace crate rather than a family of micro-crates. Split is permitted only by evidence-backed Correction Delta.
**State:** ACCEPTED

## CORE-D-082 - core-workspace does not depend on core-runtime
**Decision:** runtime generation/provenance crosses stable contracts; M02 avoids a reverse dependency cycle.
**State:** ACCEPTED

## CORE-D-083 - M02 reuses core-identity fingerprints
**Decision:** Workspace/Repository/Worktree/Basis semantic fingerprints use the existing canonical identity stack; no parallel serializer/hash framework is introduced.
**State:** ACCEPTED

## CORE-D-084 - M02 public contracts use explicit v1 envelopes
**Decision:** durable/external M02 payloads carry schema/kind/version and reject unsupported semantics. Evolution is versioned rather than inferred.
**State:** ACCEPTED

## CORE-D-085 - BVM v1 profiles are frozen freshness masks
**Decision:** READ_METADATA, READ_SOURCE, PLAN_WORK, EXECUTE_TOOL_READONLY, MUTATE_SOURCE and GIT_DELIVERY define required basis freshness; HIVE_RECONCILED is an assurance overlay. BVM never grants action authority.
**State:** ACCEPTED

## CORE-D-086 - FSC is conservative and read-only
**Decision:** M02 does not infer case behavior from OS family and does not write probe files into user source. Unknown security-sensitive semantics remain UNKNOWN or require stronger use-time proof.
**State:** ACCEPTED

## CORE-D-087 - System Git is the M02 V0.0 baseline provider
**Decision:** initial implementation includes a hardened system-Git GitInspector behind a provider-neutral trait. It must pass all evidence gates. Rust-native/hybrid alternatives remain separately evidence-gated and are not initial dependencies.
**State:** ACCEPTED

## CORE-D-088 - PEC persistent L2 is out of initial M02 scope
**Decision:** M02 V0.0 implements bounded runtime-epoch L1 proof reuse only. Persistent cache adds recovery/corruption/secret-lifecycle scope without being required for correctness.
**State:** ACCEPTED

## CORE-D-089 - OS watcher adapters are not required for M02 V0.0
**Decision:** EIS/CIG and typed hint ingestion are required, but baseline correctness must work with zero events. Platform watcher adapters are later optimizations.
**State:** ACCEPTED

## CORE-D-090 - New third-party dependencies require explicit admission
**Decision:** Round 5 freezes the initial dependency graph. Convenience alone is insufficient to add Git parsing, watcher, database, path-walk or platform-FFI crates.
**State:** ACCEPTED


## CORE-D-091 - Numeric M02 resource defaults are execution-evidence outputs
**Decision:** planning freezes resource dimensions, measurement protocol and safety semantics; exact WorkspaceResourceBudget numbers are produced from reproducible M02 implementation evidence inside the frozen Work Order before final acceptance.
**State:** ACCEPTED

## CORE-D-092 - M02 Work Order may authorize a bounded Calibration Delta
**Decision:** after benchmarks, the executor may change only numeric resource defaults, benchmark-derived thresholds and related evidence/documentation in the same Work Order/PR. Architecture/dependency/contract changes require a normal governed Correction Delta.
**State:** ACCEPTED

## CORE-D-093 - Calibration failure blocks BOUND/production acceptance
**Decision:** no partial benchmark, timeout, overflow or missing required calibration evidence may be converted into a successful production-ready M02 result.
**State:** ACCEPTED

## CORE-D-094 - Calibration is deterministic and zero-LLM
**Decision:** fixture generation, measurements, candidate selection and evidence processing do not use LLM inference.
**State:** ACCEPTED

## CORE-D-095 - DWS is required for M02 V0.0
**Decision:** component/changed-set delta revalidation is part of the initial module because it underpins bounded revalidation and downstream token/context economy. Delta correctness must equal full recomputation semantics.
**State:** ACCEPTED

## CORE-D-096 - WMF is deferred
**Decision:** Workspace Merkle Forest is not in the initial M02 Work Order. It may be admitted later only if evidence shows DWS/component fingerprints cannot satisfy scalability budgets.
**State:** ACCEPTED

## CORE-D-097 - V0.0 does not permanently self-tune resource defaults
**Decision:** calibration is build/release evidence. Runtime may consume governed configuration but does not silently benchmark itself or rewrite defaults in the background.
**State:** ACCEPTED

## CORE-D-098 - Unlimited security-sensitive resource sentinels are forbidden
**Decision:** resource budgets used to bound hostile repository/path/output behavior must have explicit finite values after calibration; zero/unlimited semantics cannot bypass those limits.
**State:** ACCEPTED


## CORE-D-099 - V0.0 system-Git baseline refines earlier provider-neutral evaluation decisions
**Decision:** CORE-D-087 is the V0.0 implementation refinement of CORE-D-064/075. The admitted system-Git baseline must prove its own semantic/security/resource/cross-platform obligations, but M02 V0.0 does not require adding an unadmitted alternative provider merely to create a comparison. Any future Rust-native/hybrid provider promotion requires differential comparison against the semantic reference and current baseline.
**State:** ACCEPTED


## CORE-D-100 - M03 is a Work Order compiler, not an executor
**Decision:** M03 compiles/validates/admit-checks immutable Work Order semantics. M04 owns Run/Attempt/Step execution state and no M03 API may execute commands or mutate source.
**State:** ACCEPTED

## CORE-D-101 - Human/LLM prose is never direct execution authority
**Decision:** free-form intent must compile into a versioned canonical Work Order revision before M04 may instantiate execution.
**State:** ACCEPTED

## CORE-D-102 - Frozen Work Order revisions are immutable
**Decision:** semantic correction creates a new WorkOrderRevision/fingerprint. Existing frozen/admitted revisions are never edited in place.
**State:** ACCEPTED

## CORE-D-103 - Work Order logical identity is distinct from revision/fingerprint
**Decision:** WorkOrderId names the logical lineage while WorkOrderRevision, WorkOrderFingerprint and WorkOrderCompilationId identify semantic/compiler states.
**State:** ACCEPTED

## CORE-D-104 - M03 consumes M02 workspace truth
**Decision:** M03 binds workspace requirements to M02 Workspace/Basis evidence and does not rediscover or redefine path/repository authority.
**State:** ACCEPTED

## CORE-D-105 - Governance authority remains external
**Decision:** M03 verifies governance/admission proof and Context Lock state but cannot self-approve a Work Order.
**State:** ACCEPTED

## CORE-D-106 - Acceptance and evidence are connected by stable IDs
**Decision:** M03 declares an Acceptance Evidence Graph. M15 later binds actual evidence artifacts; M03 does not treat command success alone as acceptance proof.
**State:** ACCEPTED

## CORE-D-107 - Scope changes are semantic, even when presented as small corrections
**Decision:** SDF classifies revision deltas. Unapproved scope/dependency/architecture/security-policy change fails closed rather than masquerading as evidence/test/documentation-only work.
**State:** ACCEPTED

## CORE-D-108 - HIVE enriches context but cannot rewrite frozen Work Order semantics
**Decision:** HIVE context references/provenance may be compiled into the manifest, but canonical source hierarchy and exact Git/filesystem evidence remain authoritative.
**State:** ACCEPTED

## CORE-D-109 - Context economy uses references plus packet deltas
**Decision:** compiled Work Orders prefer compact canonical refs/fingerprints/stable-prefix references and packet-specific deltas over repeated full-document copies, while mandatory sources remain retrievable.
**State:** ACCEPTED

## CORE-D-110 - M03 compiler/admission baseline is zero-LLM
**Decision:** parsing, validation, canonicalization, semantic diff classification, staleness and admission checks require no LLM inference.
**State:** ACCEPTED

## CORE-D-111 - Superseded or stale revisions cannot start new Runs
**Decision:** M04 may instantiate a new Run only from a current admitted Work Order revision. STALE/SUPERSEDED revisions fail typed.
**State:** ACCEPTED

## CORE-D-112 - Initial lineage is contract-first, not database-first
**Decision:** WLG lineage semantics are versioned contracts/edges in V0.0 discovery. No persistent database is selected merely to represent Work Order revisions.
**State:** ACCEPTED

## CORE-D-113 - Event/context hints never prove Work Order freshness by absence
**Decision:** WSF may use change hints to narrow revalidation, but uncertain/missing evidence broadens STALE and canonical bindings must be verified deterministically.
**State:** ACCEPTED
