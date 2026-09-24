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


## CORE-D-114 - M03 v1 uses explicit versioned envelopes
**Decision:** durable/external M03 payloads use `nexlabs.core.work-order` schema/version/kind envelopes and unsupported semantics fail typed.
**State:** ACCEPTED

## CORE-D-115 - Frozen Work Order and admission receipt are separate contracts
**Decision:** immutable Work Order semantics do not contain mutable/current admission truth. Admission is evaluated separately against current workspace, Context Lock, governance and policy evidence.
**State:** ACCEPTED

## CORE-D-116 - Frozen Work Orders declare workspace requirements, not live handles
**Decision:** runtime-bound M02 WorkspaceHandles are supplied at admission and are never persisted as semantic Work Order capability.
**State:** ACCEPTED

## CORE-D-117 - M03 v1 packet dependencies form a bounded DAG
**Decision:** packet prerequisites are acyclic semantic dependencies with deterministic topological order. The graph does not grant scheduler/concurrency authority.
**State:** ACCEPTED

## CORE-D-118 - READY admission is non-evergreen
**Decision:** a READY receipt is valid only for its recorded workspace/basis/lock/governance/policy generations. M04 must re-check required bindings before creating a new Run.
**State:** ACCEPTED

## CORE-D-119 - Scope deny overrides allow
**Decision:** explicit forbidden scope beats allow rules; ambiguity fails closed. Packet scope is intersected with parent Work Order scope.
**State:** ACCEPTED

## CORE-D-120 - Dependency admission is not implied by file/crate scope
**Decision:** permission to edit a path/package does not itself authorize adding a dependency.
**State:** ACCEPTED

## CORE-D-121 - Execution corrections and semantic Work Order revisions are distinct
**Decision:** implementation/test/evidence fixes that remain inside frozen semantics can use a typed ExecutionCorrectionProposal under the same Work Order revision. Semantic contract changes require a new immutable revision.
**State:** ACCEPTED

## CORE-D-122 - Same-revision correction classes are explicit
**Decision:** correction policy may allow bounded implementation/test/evidence/documentation/calibration/generated-artifact changes, but cannot silently allow scope/dependency/architecture/security changes or acceptance/stop weakening.
**State:** ACCEPTED

## CORE-D-123 - Acceptance Evidence Graph is structurally complete
**Decision:** each blocking criterion resolves to required evidence obligations or deterministic N/A semantics. Actual evidence artifacts remain outside M03 ownership.
**State:** ACCEPTED

## CORE-D-124 - Machine-readable stop conditions are mandatory
**Decision:** a Work Order without explicit terminal success/blocked/required-obligation semantics cannot reach READY.
**State:** ACCEPTED

## CORE-D-125 - Context budgets never justify silent semantic truncation
**Decision:** context limits can favor refs/progressive expansion but mandatory semantic sources cannot be silently dropped.
**State:** ACCEPTED

## CORE-D-126 - Work staleness hints are not truth
**Decision:** WSF may use hints to nominate revalidation but absence of a hint cannot prove freshness. UNKNOWN broadens to STALE/BLOCKED.
**State:** ACCEPTED

## CORE-D-127 - Durable M03 payloads are secret-free by construction
**Decision:** contracts store safe references/classifications/fingerprints, not raw credentials or secret-bearing provider payloads.
**State:** ACCEPTED

## CORE-D-128 - WOC/SDF/AEG/CBE/WLG/WSF/WPC are required M03 V0.0 semantic capabilities
**Decision:** the semantic behaviors of all seven Round 1 mechanisms are part of planned V0.0. Exact Rust implementation/file decomposition remains a later freeze.
**State:** ACCEPTED


## CORE-D-129 - Frozen Work Order objects have no mutable diagnostic subspace
**Decision:** non-semantic diagnostic/transport/rendering metadata may evolve only outside the immutable FrozenWorkOrder revision. No field stored inside a frozen revision is edited in place under a "diagnostic-only" label.
**State:** ACCEPTED


## CORE-D-130 - Context Lock concrete identity is admission evidence, not FrozenWorkOrder semantic input
**Decision:** FrozenWorkOrder fingerprints Context Lock constraints. The concrete lock fingerprint is recorded only after freeze in admission evidence, preventing a WorkOrderFingerprint <-> ContextLockFingerprint cycle.
**State:** ACCEPTED

## CORE-D-131 - Admission receipts are immutable historical proofs
**Decision:** a READY receipt never mutates into STALE. Later changed inputs cause a new admission evaluation/receipt, while M04 rejects reuse when recorded preconditions no longer match.
**State:** ACCEPTED


## CORE-D-132 - Reviews use reviewer-first correction
**Decision:** during governed reviews, the reviewer first applies any safe, bounded, causally understood correction that can be implemented and validated with currently available repository/GitHub tools. Codex or another executor is used only when the correction requires broader product implementation, unavailable local/runtime state, dependency/architecture/scope/security-policy change, or assurance that the review environment cannot provide. Every direct correction creates a new exact head and requires fresh applicable evidence.
**State:** ACCEPTED


## CORE-D-133 - M03 core is stateless by default
**Decision:** V0.0 compiler/admission logic owns no internal database or hidden canonical registry. It consumes explicit snapshots/evidence and emits deterministic contracts.
**State:** ACCEPTED

## CORE-D-134 - M03 core performs no hidden I/O
**Decision:** repository/Git/HIVE/GitHub/network/persistence work belongs to external adapters/domains; compiler semantics operate on resolved typed inputs.
**State:** ACCEPTED

## CORE-D-135 - M03 does not generate opaque random WorkOrderIds
**Decision:** logical IDs are caller-provided or deterministically derived from a versioned logical key through core-identity.
**State:** ACCEPTED

## CORE-D-136 - LPC provides lineage compare-and-set semantics
**Decision:** new semantic revisions emit a Lineage Precondition Capsule binding expected parent revision/fingerprint/store generation. Canonical persistence must reject stale preconditions.
**State:** ACCEPTED

## CORE-D-137 - M03 V0.0 has no internal persistence database
**Decision:** canonical Work Orders may remain repository/GEF persisted. M03 serializes and validates but does not commit/push/self-promote. Database storage is future/evidence-gated.
**State:** ACCEPTED

## CORE-D-138 - M03 reuses core-identity canonical fingerprinting
**Decision:** semantic projection is explicit and sorted; no second generic canonical/hash stack is introduced.
**State:** ACCEPTED

## CORE-D-139 - Compile memoization is derived and optional
**Decision:** any L1 compilation memo is disposable and correctness-independent. Persistent compilation cache is not part of V0.0.
**State:** ACCEPTED

## CORE-D-140 - M03 numeric resource defaults require calibration
**Decision:** Round 3 freezes dimensions/fixture directions; final finite numeric defaults come from reproducible implementation evidence before production acceptance.
**State:** ACCEPTED

## CORE-D-141 - M03 V0.0 core is synchronous unless evidence proves async necessity
**Decision:** no Tokio/async runtime dependency is admitted merely for compiler convenience.
**State:** ACCEPTED

## CORE-D-142 - M03 starts as one core-work-order crate
**Decision:** one focused crate is the initial direction. Split requires later evidence and governed change.
**State:** ACCEPTED

## CORE-D-143 - Dependency direction is M02 -> M03, never reverse
**Decision:** core-work-order may consume core-workspace public contracts; core-workspace does not depend on core-work-order, and M03 does not depend on M04+.
**State:** ACCEPTED

## CORE-D-144 - M03 errors expose category, reason and retryability
**Decision:** errors are machine-readable and bounded, separating invalid input, stale/conflict, policy, resource and internal failures.
**State:** ACCEPTED

## CORE-D-145 - M03 never performs hidden refresh-and-retry
**Decision:** lineage/workspace/governance/policy refresh can change authority/semantics and therefore remains explicit caller orchestration.
**State:** ACCEPTED

## CORE-D-146 - PCM deduplicates packet context references, not obligations
**Decision:** Packet Context Mesh may share canonical source nodes but each packet's reconstructed mandatory source set must be exact.
**State:** ACCEPTED

## CORE-D-147 - DCR is required compilation provenance
**Decision:** every compilation emits a deterministic receipt tying request/context/compiler/lineage-precondition/output fingerprints; it is evidence, not execution authority.
**State:** ACCEPTED

## CORE-D-148 - Resource failure cannot produce partial FROZEN/READY state
**Decision:** budget/deadline exhaustion is typed fail-closed and cannot yield partially accepted semantic/admission output.
**State:** ACCEPTED


## CORE-D-149 - GitHub main protection is enforced by an active no-bypass ruleset
**Decision:** CORE `main` uses the active repository ruleset `CORE main protection` (id `23769853`) to require pull-request entry, block deletion/non-fast-forward updates, require resolved review threads and enforce the current strict seven-context CI gate with no bypass actor. GitHub-native approving-review count remains zero for solo-maintainer compatibility; the independent GEF exact-head audit verdict and no-HIGH/CRITICAL promotion rule remain separate mandatory process gates, and auto-merge may be armed only after that governed verdict permits promotion.
**State:** ACCEPTED


## CORE-D-150 - M03 V0.0 public contracts are frozen as versioned V1 types
**Decision:** the public M03 schema identifier is nexlabs.core.work-order version 1. Durable root request, frozen, compilation, validation, revision-diff, correction-classification, admission and handoff payloads use a closed kind envelope and distinct typed IDs/revisions/fingerprints. Unsupported or ambiguous versions fail typed.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-151 - Semantic identity is an explicit canonical projection
**Decision:** WorkOrderFingerprint covers only explicitly enumerated immutable semantic fields. Diagnostics, timestamps, transport/rendering state, future runtime/evidence data and compiler/canonicalizer/policy/config/security implementation generations remain outside the frozen semantic projection. Unordered collections are explicitly sorted before reusing core-identity canonical_bytes/fingerprint; compiler implementation identity is bound separately by WorkOrderCompilationId.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-152 - M03 services consume resolved evidence and perform no hidden I/O
**Decision:** parse/compile/validate/diff/correction/admission/handoff are synchronous pure operations over explicit bounded values and deterministic budgets. Caller-owned resolvers and wall-clock deadline guards run outside the core; M03 functions never call adapters, read a host clock, refresh, retry, persist or access ambient state. A timed-out caller must discard any late result.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-153 - M02 proof crosses a narrow value-only adapter boundary
**Decision:** M03 binds versioned M02 workspace/basis identity, generation, fingerprint, required BVM profile/components and provenance through a bounded M03 evidence DTO. M03 does not duplicate M02 truth or directly depend on core-workspace; external host code maps the public M02 evidence into the DTO.
**Rationale:** base cargo metadata shows core-workspace enables Tokio filesystem/network/process features for M02 services, which are unnecessary in the compiler dependency closure.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-154 - Context Lock and governance evidence remain externally verified
**Decision:** M03 consumes versioned Context Lock evidence and externally verified governance proof bound to WorkOrderId/revision/fingerprint, source/base/head, scope and policy generation. M03 checks compatibility but never mints approval or implementation authority.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-155 - M03 V0.0 remains one crate with a minimal direct dependency set
**Decision:** one core-work-order crate contains contracts, identity, canonicalization, source/evidence DTOs, pure services, budgets and errors. Direct dependencies are core-identity plus existing serde, serde_json and thiserror. core-workspace/core-config/core-contracts/sha2 are not direct M03 dependencies; the optional M02 adapter is outside the crate. No direct Tokio, process/network, Git/HIVE/GitHub, database, cache, graph or regex dependency is admitted.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-156 - M03 errors are typed and retries remain caller-owned
**Decision:** closed error categories/codes cover schema, source/provenance, scope, packet graph, acceptance/evidence, lineage, admission/staleness, resources and internal invariants. Retryability names only an explicit caller action and never authorizes hidden refresh or retry.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-157 - Resource limits are finite and calibration-gated
**Decision:** all security-sensitive deterministic M03 core resource dimensions must have finite positive values before production acceptance. Round 4 freezes dimensions, fixtures, measurement protocol and fail-closed selection rules; it assigns no numeric defaults or measured performance claims. Wall-clock deadlines are caller-owned orchestration guards outside the pure core and cannot make semantic output depend on host scheduling.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-158 - M03 property and fuzz laws are explicit V0.0 obligations
**Decision:** deterministic property laws cover canonical replay/permutations, packet DAGs, scope, AEG/PCM, lineage/LPC, correction classes, provenance, admission freshness/replay, atomic resource failure and diagnostic redaction. Fuzzing reuses the existing libfuzzer-sys package with bounded targets; Round 4 adds no property-testing dependency.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-159 - M03 benchmarks use reproducible synthetic calibration
**Decision:** source/packet/edge/criteria/evidence/context/diff/admission scaling is measured on Windows and Ubuntu using local deterministic fixtures and the existing built-in bench harness. Cache comparisons apply only if a cache is separately admitted. Unsupported scales are reported, never extrapolated.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-160 - M03 production DoD is exact-head and evidence-traceable
**Decision:** completion requires versioned contracts, deterministic serialization, property/fuzz/security/resource/supply-chain and cross-platform evidence, no-hidden-I/O/dependency proof, complete AEG evidence and independent exact-head review with no unresolved HIGH/CRITICAL finding.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.

## CORE-D-161 - Round 4 checkpoint status remains unpromoted
**Decision:** this branch records Round 4 as a planning review candidate only. M03 implementation, checkpoint promotion, merge, release and the next planning increment remain unauthorized until the independent exact-head review and canonical governance gate permit progression.
**State:** ACCEPTED; Round 4 promoted by Review 004 / Issue #63 and PR #62; implementation remains unauthorized.


## CORE-D-162 - M03 Round 4 is promoted and final planning freeze is the next legal action
**Decision:** after Review 004 / Issue #63 approved exact head `8fd3f085f93b342373b06e4471088dc7b843fac4` and PR #62 was promoted as merge `78daa752760ba19b3c36c7e2a7574bb3cfd03501`, M03 Rounds 1-4 are canonical planning truth. The next legal increment is the Round 5 final planning freeze that compiles the implementation Work Order, pending Context Lock, acceptance/evidence mapping, Calibration Gate and executor handoff. Product implementation remains unauthorized until that freeze is independently reviewed/promoted and a separate execution-admission delta binds canonical main.
**State:** ACCEPTED


## CORE-D-163 - M03 Round 5 final planning freeze is a review candidate
**Decision:** CORE-M03-FREEZE-001 packages the future M03 implementation Work Order, pending Context Lock, Evidence Bundle skeleton, eight packets, Calibration Gate, 23-criterion AEG, and executor handoff on the exact planning base. This candidate does not authorize implementation. Independent exact-head review/promotion and a separate execution-admission delta binding canonical main are required before product work.
**State:** PROPOSED; pending independent exact-head review.


## CORE-D-164 - M03 final planning freeze is promoted and execution admission is the next legal action
**Decision:** Review 006 / Issue #67 APPROVED CORE-M03-FREEZE-001 at exact head `326eea936989ad2155ae6d1fb3fc965b8d1d25b9`; PR #66 was promoted as merge `ac90b1f48c5551e65ecadace95c59f7f0647062f`. The M03 Work Order, pending Context Lock, Packs A-H, Calibration Gate and 23-criterion AEG are now frozen planning truth. The next legal change is the bounded `CORE-M03-ADMIT-001` execution-admission delta. It may bind the promoted canonical base, activate the Context Lock and arm only `M03_WORK_ORDER_ENGINE`, but execution authority is effective only after that exact admission state is independently reviewed and promoted to canonical `origin/main`.
**State:** ACCEPTED

## CORE-D-165 - M03 execution admission is promoted; implementation is the next legal action
**Decision:** Review 007 / Issue #69 APPROVED CORE-M03-ADMIT-001 at exact head `36f18d1dc53faf9af9d3c06355ebe07384e4e1f7` with workflow run `35845550610` green across all seven required contexts. PR #68 was promoted as squash merge `abe21ed4564978d24b2f41bca13b6f052daa3b17`. The admission is therefore canonical. The next legal product action is governed execution of `CORE-WO-M03-001` from post-admission canonical `origin/main`, after recompiling the Context Lock/evidence bindings for this post-promotion state. No M03 product Pack A-H was implemented by this synchronization increment.
**State:** ACCEPTED


## CORE-D-166 - M03 implementation is promoted and M04 planning is the next legal product increment
**Decision:** M03-REVIEW-009 / Issue #73 APPROVED CORE-WO-M03-001 at exact head `b92f14514e4cf615ed76fa87915539a835b6a85b`. Workflow run `35905315953` completed all 10 jobs SUCCESS, including M03 Windows, M03 Ubuntu and the seven-target bounded fuzz campaign. PR #72 was squash-promoted as canonical merge `c63df6ad581c44bac66a8b1dab9a86522ce7fe3b`. EV-025 is satisfied by the independent exact-head review and no unresolved HIGH/CRITICAL finding remains. This closeout delta changes governance/project state only. After its independent review and promotion, M04 discovery/planning is the next legal product increment; M04 implementation remains unauthorized until its own planning freeze and execution admission.
**State:** ACCEPTED


## CORE-D-167 - M04 owns execution state, not execution side effects
**Decision:** M04 owns deterministic Run / Attempt / Step identities, lifecycle transitions, lineage, terminal-state semantics, replayable execution-state records and continuation boundaries. Host/model/tool execution, mutations, verification, recovery policy, delivery and telemetry transport remain owned by later modules. M04 implementation remains unauthorized.
**State:** ACCEPTED; Round 1 promoted by M04-REVIEW-001 / Issue #77 and PR #76; implementation remains unauthorized.

## CORE-D-168 - M04 starts only from revalidated M03 READY authority
**Decision:** creation/admission of a Run requires a current M03 READY handoff whose work-order, workspace/basis, Context Lock/governance and policy bindings are revalidated at the start boundary. UNKNOWN or stale authority fails closed.
**State:** ACCEPTED; Round 1 promoted by M04-REVIEW-001 / Issue #77 and PR #76; implementation remains unauthorized.

## CORE-D-169 - M04 history is append-only and retries create new Attempts
**Decision:** Attempts and Steps are immutable historical lineage records once terminal. Retry or continuation creates a new bounded child/epoch rather than rewriting prior execution history.
**State:** ACCEPTED; Round 1 promoted by M04-REVIEW-001 / Issue #77 and PR #76; implementation remains unauthorized.

## CORE-D-170 - Semantic ordering is causal, not wall-clock based
**Decision:** M04 uses explicit monotonic semantic generations/sequences for ordering and compare-and-set behavior. Timestamps may be diagnostic metadata but cannot alone establish execution truth.
**State:** ACCEPTED; Round 1 promoted by M04-REVIEW-001 / Issue #77 and PR #76; implementation remains unauthorized.

## CORE-D-171 - M04 replay and state publication fail closed
**Decision:** canonical event replay must reconstruct identical semantic state. Reordered, duplicated where non-idempotent, truncated, corrupt or stale-generation histories fail typed. Storage adapters must prevent valid partial advancement through an atomic semantic state fence.
**State:** ACCEPTED; Round 1 promoted by M04-REVIEW-001 / Issue #77 and PR #76; implementation remains unauthorized.

## CORE-D-172 - M04 core is backend-neutral, zero-LLM and hidden-I/O-free
**Decision:** Round 1 freezes no persistence backend. Core state-machine semantics consume explicit values/evidence and perform no hidden filesystem, process, network, database, HIVE/GitHub or LLM operation.
**State:** ACCEPTED; Round 1 promoted by M04-REVIEW-001 / Issue #77 and PR #76; implementation remains unauthorized.


## CORE-D-173 - M04 Round 2 freezes closed lifecycle matrices and immutable terminal history
**Decision:** Run, Attempt and Step use the explicit legal transition matrices in the canonical M04 module plan. Initial states are durable; terminal states never transition back to active. Continuation after interruption creates a new Attempt/epoch. SKIPPED is pre-ACTIVE only and requires explicit authority/reason.
**State:** ACCEPTED; Round 2 promoted by M04-REVIEW-002 / Issue #79 and PR #78; implementation remains unauthorized.

## CORE-D-174 - Run generation is M04's serialization fence
**Decision:** every semantic M04 commit advances the Run generation exactly once under compare-and-set. Attempt/Step ordinals and local sequences remain bounded lineage/order facts, but wall-clock time never resolves concurrency. State, journal, root and generation publication are one atomic semantic fence.
**State:** ACCEPTED; Round 2 promoted by M04-REVIEW-002 / Issue #79 and PR #78; implementation remains unauthorized.

## CORE-D-175 - Idempotency is fingerprint-bound and conflicting reuse fails closed
**Decision:** idempotency is scoped by Run, operation domain and caller key. Exact replay of the same canonical request returns its recorded result; reuse with a different semantic fingerprint is IDEMPOTENCY_CONFLICT and produces no state advancement.
**State:** ACCEPTED; Round 2 promoted by M04-REVIEW-002 / Issue #79 and PR #78; implementation remains unauthorized.

## CORE-D-176 - Cancellation is monotonic and generation-ordered
**Decision:** once cancellation is durably accepted, later child admission/activation is rejected except explicitly bounded closeout. Earlier committed child history remains immutable and is driven to terminal closeout rather than erased.
**State:** ACCEPTED; Round 2 promoted by M04-REVIEW-002 / Issue #79 and PR #78; implementation remains unauthorized.

## CORE-D-177 - M04 canonical journal is bounded, contiguous and replay-authoritative
**Decision:** durable events form a domain-separated contiguous sequence with prior/result journal roots and generation bindings. Reorder, truncation, substitution, wrong lineage/domain or root mismatch fails closed. Derived views never supersede the canonical bounded journal.
**State:** ACCEPTED; Round 2 promoted by M04-REVIEW-002 / Issue #79 and PR #78; implementation remains unauthorized.

## CORE-D-178 - Continuation revalidates exact M03 authority and creates a new epoch
**Decision:** ICF continuation binds the last semantic M04 boundary and BRC binds exact M03 admission/workspace/Context Lock/governance authority. Resume validates all bindings and creates a new Attempt/epoch. Changed, UNKNOWN or unverifiable authority becomes STALE/BLOCKED.
**State:** ACCEPTED; Round 2 promoted by M04-REVIEW-002 / Issue #79 and PR #78; implementation remains unauthorized.


## CORE-D-179 - M04 V1 exposes explicit versioned request/receipt contracts
**Decision:** M04 mutations and replay use explicit V1 request/receipt/result contracts with expected-generation and idempotency inputs where semantic state changes. No API receives ambient filesystem, repository, network, process or clock authority.
**State:** ACCEPTED; Round 3 promoted by M04-REVIEW-003 / Issue #83 and PR #80; implementation remains unauthorized.

## CORE-D-180 - M04 canonical fingerprints are schema-bound and domain-separated
**Decision:** semantic fingerprints use deterministic canonical bytes, explicit field ordering/lengths and distinct domain separators. Diagnostic time, locale, unordered map iteration and secret-bearing fields are excluded. Cross-platform golden vectors are blocking evidence.
**State:** ACCEPTED; Round 3 promoted by M04-REVIEW-003 / Issue #83 and PR #80; implementation remains unauthorized.

## CORE-D-181 - M04 V1 event kinds and error classes are closed registries
**Decision:** V1 accepts only frozen event kinds and machine-readable error/reason classes. Unknown schema/kind fails typed with no silent downgrade; free-form diagnostics never substitute for semantic reason codes.
**State:** ACCEPTED; Round 3 promoted by M04-REVIEW-003 / Issue #83 and PR #80; implementation remains unauthorized.

## CORE-D-182 - Snapshots accelerate projection but never replace journal authority
**Decision:** snapshots bind an exact journal boundary and are verified derived artifacts. Active canonical Run events required for proof are not destructively compacted by M04; archive/retention policy is external.
**State:** ACCEPTED; Round 3 promoted by M04-REVIEW-003 / Issue #83 and PR #80; implementation remains unauthorized.

## CORE-D-183 - External execution/verification data enters M04 only as bounded references
**Decision:** M04 records versioned lineage-bound references and attachment facts, not external artifact bodies and not later-module truth decisions. Reference shape/lineage/bounds are validated without importing M14-M17 policy.
**State:** ACCEPTED; Round 3 promoted by M04-REVIEW-003 / Issue #83 and PR #80; implementation remains unauthorized.

## CORE-D-184 - M04 final acceptance uses a blocking 23-node Evidence Graph
**Decision:** EV-M04-001 through EV-M04-023 cover contracts, transitions, concurrency, idempotency, cancellation, replay, BRC/ICF, identities, canonicalization, resource bounds, snapshots, references, hidden-I/O/zero-LLM, fuzz, calibration, supply chain, cross-platform CI and independent review. Numeric resource defaults require evidence-backed calibration and cannot mutate semantics.
**State:** ACCEPTED; Round 3 promoted by M04-REVIEW-003 / Issue #83 and PR #80; implementation remains unauthorized.


## CORE-D-185 - M04 Rounds 1-3 are promoted; Round 4 is the next legal planning increment
**Decision:** M04-REVIEW-003 / Issue #83 APPROVED CORE-M04-PLAN-003 at exact head `0a777b038e5436b72162ccd2185020d40128d809`; workflow `35940552631` completed all 10 hosted jobs SUCCESS and PR #80 was squash-promoted as merge `3b1fc86ee401153f109ee04d7b797b544b06a741`. Together with the prior Round 1 and Round 2 promotions, Rounds 1-3 are canonical planning truth. The next legal increment is Round 4 implementation-addressable freeze design. M04 product implementation remains unauthorized until a final planning freeze and a separate governed execution-admission delta are independently reviewed and promoted.
**State:** ACCEPTED
