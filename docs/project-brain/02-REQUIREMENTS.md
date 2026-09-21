# CORE Requirements

Status: `PRODUCT_DISCOVERY_ACTIVE`

This file contains the frozen foundation requirements plus accepted module-level product discovery requirements. Later-module requirements remain pending until governed discovery/freeze.

## Foundation requirements

- **CORE-R-001 GEF governance:** CORE MUST use GEF v1 lifecycle semantics for planning, bounded execution, evidence, review and checkpoint promotion.
- **CORE-R-002 HIVE canonical paths:** CORE MUST preserve the five exact HIVE v1.0.0 governance paths defined in `00-README-UPLOAD-ORDER.md`.
- **CORE-R-003 HIVE-first execution:** implementation Work Orders MUST include HIVE preflight whenever HIVE can materially assist the work.
- **CORE-R-004 Deterministic first:** Git, hashes, static inspection, AST/symbol data and tests MUST precede model inference when they can prove the fact.
- **CORE-R-005 Exact-state evidence:** tests, audits and promotion evidence MUST identify the exact candidate/head they validate.
- **CORE-R-006 No duplicate canonical truth:** derived GEF/HIVE metadata MUST NOT silently supersede Project Brain or Git.
- **CORE-R-007 Safe degradation:** unavailable HIVE/provider capabilities MUST be reported truthfully and MUST NOT be represented as successful evidence.
- **CORE-R-008 Public repository hygiene:** secrets, credentials, private tokens and private user data MUST NOT be committed.
- **CORE-R-009 Product planning gate:** product implementation MUST NOT begin until Scope, Architecture, Requirements and DoD for the first implementation increment are explicitly frozen.
- **CORE-R-010 External HIVE runtime:** HIVE runtime MUST remain independently deployable and MUST NOT be vendored into CORE merely for convenience.

## Product requirements

- **CORE-R-011 Headless:** no dashboard/cockpit/web UI.
- **CORE-R-012 Standalone:** CORE remains safely usable without HIVE.
- **CORE-R-013 HIVE substitution:** compatible HIVE capabilities replace bounded fallbacks through contracts.
- **CORE-R-014 Complete-product commitment:** ACCEPTED_REQUIRED capabilities must be built; there is no MVP tier.
- **CORE-R-015 LLM economics:** LLM-facing modules optimize tokens/retries/reusable evidence without lowering quality.
- **CORE-R-016 Cache-first:** LLM-facing contracts preserve stable material, deterministic identity and explicit invalidation.
- **CORE-R-017 Zero-LLM lifecycle:** M01 bootstrap/lifecycle/health/shutdown uses no inference.
- **CORE-R-018 Cache evidence:** reuse exposes class, hit/miss/bypass reason, identity/provenance and invalidation basis where applicable.



## M01 production requirements

- **CORE-R-019 Crash ambiguity:** restart MUST NOT infer success for an ambiguous external side effect.
- **CORE-R-020 Epoch safety:** stale prior-epoch runtime messages/leases MUST NOT mutate current runtime state.
- **CORE-R-021 Graceful shutdown:** shutdown MUST verify quiescence or explicitly record residual/incomplete obligations.
- **CORE-R-022 Quality-floor failover:** provider/fallback substitution MUST NOT reduce an operation below its declared quality/policy floor.
- **CORE-R-023 Multi-dimensional health:** health MUST expose capability/impact degradation rather than a single boolean.
- **CORE-R-024 Probe coalescing:** equivalent concurrent external health probes SHOULD be coalesced when correctness/freshness permit.
- **CORE-R-025 Delta observability:** repeated runtime/health telemetry SHOULD support stable-baseline + delta representation to reduce redundant storage/context.

- **CORE-R-026 Performance evidence:** promotion MUST detect material regressions against compatible benchmark baselines.
- **CORE-R-027 Bounded resources:** runtime queues, retries, frames and safety-relevant allocations MUST be bounded by policy.
- **CORE-R-028 Supply-chain evidence:** release evidence MUST include dependency/advisory/license/provenance checks and SBOM.
- **CORE-R-029 Unsafe Rust:** first-party unsafe code MUST be exceptional, localized, documented and independently reviewable.
- **CORE-R-030 Cache economics:** future LLM-facing execution MUST expose stable/cache-eligible versus uncached/retried token economics.


## M02 Project / Workspace Adapter requirements

- **CORE-R-031 Explicit workspace basis:** execution-capable modules MUST NOT act without a validated WorkspaceHandle and WorkspaceBasisFingerprint.
- **CORE-R-032 Identity separation:** project binding, workspace, repository and worktree identities MUST remain distinct typed identities.
- **CORE-R-033 Git/local truth:** local filesystem/Git state is canonical for the attached checkout; HIVE project identity MUST NOT overwrite contradictory local checkout evidence.
- **CORE-R-034 HIVE reconciliation:** HIVE association MUST be represented with provenance and explicit match/conflict/unknown state; no fabricated HIVE identity is allowed.
- **CORE-R-035 Standalone binding:** M02 MUST support bounded deterministic workspace binding without HIVE.
- **CORE-R-036 Path authority:** every path exposed for later execution MUST be validated against declared workspace authority roots.
- **CORE-R-037 Escape resistance:** traversal, symlink/junction/reparse escape and ambiguous normalization MUST fail closed when security-relevant.
- **CORE-R-038 Read-only M02:** M02 MUST NOT own source mutation, Git commit/branch/PR mutation or delivery behavior.
- **CORE-R-039 Workspace drift:** correctness-relevant basis drift MUST invalidate or revalidate affected WorkspaceHandles before later action.
- **CORE-R-040 Worktree awareness:** linked worktrees, detached HEAD, submodules, sparse checkout and nested repositories MUST be represented explicitly rather than flattened into one path.
- **CORE-R-041 Zero-LLM workspace identity:** discovery, binding, path validation, Git basis and drift detection MUST require zero inference.
- **CORE-R-042 Deterministic workspace fingerprints:** M02 MUST reuse M01 canonical serialization/fingerprint primitives for workspace correctness identity.
- **CORE-R-043 Delta revalidation:** repeated workspace validation SHOULD recompute only correctness-relevant deltas when equivalence to full recomputation is provable.
- **CORE-R-044 Secret-safe Git metadata:** remote URLs, config and process output MUST be redacted so credentials/tokens cannot enter receipts/evidence.
- **CORE-R-045 Proof-carrying binding:** successful workspace admission MUST emit a compact versioned binding receipt sufficient for downstream validation without embedding repository contents.


## M02 Round 2 requirements

- **CORE-R-046 Runtime-bound handles:** live WorkspaceHandle objects MUST be bound to the current runtime epoch/generation; durable receipts MUST NOT become implicit live authority after restart.
- **CORE-R-047 Semantic Git basis:** raw Git metadata bytes/stat-cache churn MUST NOT alter workspace correctness identity unless repository semantics changed.
- **CORE-R-048 Untracked policy is explicit:** untracked-file treatment MUST be recorded in the basis; narrower policies MUST NOT be silently selected for performance.
- **CORE-R-049 Revalidation emits new generation:** correctness-relevant compatible drift MUST produce a new WorkspaceGeneration/handle rather than reviving the stale handle.
- **CORE-R-050 No global evidence override:** user intent, local checkout facts and HIVE project association are separate authority domains and MUST be reconciled rather than ranked into one overwrite hierarchy.
- **CORE-R-051 Path validation is not sandbox authority:** M02 receipts MUST state when use-time revalidation is required and MUST NOT claim M11-level enforcement.
- **CORE-R-052 Non-existing path safety:** validation for non-existing targets MUST bind the nearest existing physical ancestor and require use-time revalidation before later mutation.
- **CORE-R-053 Git inspection is bounded/read-only:** M02 Git inspection MUST use explicit non-shell commands/APIs, bounded output/deadlines, no interactive credentials and no network side effects.
- **CORE-R-054 Basis deltas are reconstructable:** any incremental WorkspaceBasisDiff path admitted for correctness MUST be provably equivalent to full recomputation for the affected basis.
- **CORE-R-055 Security drift invalidates:** authority/security/filesystem-semantics changes MUST invalidate affected handles regardless of performance/cache cost.


## M02 Round 3 requirements

- **CORE-R-056 Authority classes:** SOURCE_AUTHORITY, GIT_METADATA_AUTHORITY and any external object-store authority MUST remain distinct and non-transitive.
- **CORE-R-057 Git metadata indirection:** linked-worktree/common-dir metadata outside the source root MAY be inspected read-only but MUST NOT become source authority.
- **CORE-R-058 No automatic submodule/network mutation:** M02 MUST NOT init/update/fetch/clone submodules or contact remotes during workspace binding.
- **CORE-R-059 Repository graph bounds:** nested/submodule/worktree discovery MUST have explicit depth/node/resource bounds and cycle detection.
- **CORE-R-060 Bare repository semantics:** a bare repository MUST NOT satisfy an operation that requires source-worktree authority.
- **CORE-R-061 HIVE association capability:** HIVE project association MUST enter M02 through a versioned external capability/provenance contract, not a HIVE source-code dependency.
- **CORE-R-062 External object stores:** Git alternates/shared object roots outside admitted metadata authority MUST be blocked or explicitly policy-admitted with provenance.
- **CORE-R-063 Git inspection hardening:** every GitInspector backend MUST be read-only, no-network, non-interactive, bounded and cancellation-aware.
- **CORE-R-064 Filesystem semantics honesty:** case/alias/path semantics MUST expose UNKNOWN where not reliably provable; M02 MUST NOT guess security-sensitive normalization.
- **CORE-R-065 Streaming resource safety:** attacker-controlled path/content/output cardinality MUST NOT cause unbounded memory allocation.
- **CORE-R-066 Association disconnect semantics:** temporary HIVE loss MUST NOT rewrite local workspace/repository identity; assurance degradation MUST be explicit.
- **CORE-R-067 Submodule declaration separation:** declared and materialized submodules MUST be represented separately.
- **CORE-R-068 Nested repository explicitness:** nested independent repositories MUST be explicitly admitted/ignored/conflicted by policy rather than silently merged.
- **CORE-R-069 No repository repair:** M02 MUST report malformed/unsupported Git state rather than repairing, resetting or normalizing the repository.
- **CORE-R-070 Backend equivalence:** any system-Git or Rust-native GitInspector implementation MUST satisfy the same canonical fixtures/security contract.


## M02 Round 4 requirements

- **CORE-R-071 Watchers are hints:** filesystem/Git watcher events MAY narrow revalidation work but MUST NOT be treated as freshness proof or directly produce BOUND.
- **CORE-R-072 Action-boundary proof:** downstream action admission MUST re-check the required WorkspaceBasis validity mask even when no watcher change was observed.
- **CORE-R-073 Derived proof cache:** M02 cache entries are disposable derived evidence and MUST NOT become canonical workspace truth or filesystem authority.
- **CORE-R-074 Cache observability:** proof reuse MUST expose hit/miss/bypass/invalidation reason, provider/version, policy generation and provenance.
- **CORE-R-075 No mtime-only correctness:** timestamps/stat metadata MAY nominate reuse candidates but MUST NOT alone prove unchanged correctness-relevant file content.
- **CORE-R-076 Bounded hashing:** content hashing MUST be streaming, cancellation-aware, concurrency-bounded and coalesce equivalent in-flight work when safe.
- **CORE-R-077 Causal invalidation:** evidence changes MUST map deterministically to affected WorkspaceBasis component masks; selective invalidation MUST be no weaker than full required-mask validation.
- **CORE-R-078 Provider differential proof:** alternative GitInspector providers MUST be compared after canonicalization against a Git semantic reference oracle for all claimed capabilities.
- **CORE-R-079 Backend evidence gate:** production GitInspector provider selection MUST remain evidence-driven across semantic compatibility, security, resource use, cross-platform behavior and benchmark results.
- **CORE-R-080 Canonical graph serialization:** repository/worktree graph identity MUST use deterministic sorted serialization independent of map/hash iteration order and diagnostic timestamps.
- **CORE-R-081 Authority root contract:** each authority root MUST carry typed class, stable identity, physical/canonical evidence, filesystem semantics, provenance and policy generation; authority classes are non-transitive.
- **CORE-R-082 Security-sensitive cache bypass:** unknown filesystem semantics, changed authority/security generation or insufficient proof MUST bypass/invalidate cached evidence rather than assume freshness.
- **CORE-R-083 Event loss safety:** watcher overflow/loss MUST broaden invalidation and trigger revalidation; it MUST NOT be interpreted as no change.
- **CORE-R-084 Compact downstream evidence:** M02 SHOULD expose stable fingerprints, component masks, deltas and evidence references instead of raw path/status inventories to reduce downstream context/token cost.
- **CORE-R-085 Resource-budget contract:** Git inspection, graph traversal, hashing, cache and watcher processing MUST operate under explicit typed resource budgets; limit breach MUST fail typed without partial BOUND success.


## M02 Round 5 requirements

- **CORE-R-086 Versioned workspace contracts:** durable/external M02 payloads MUST use explicit schema/version envelopes and reject unsupported semantics.
- **CORE-R-087 Single fingerprint stack:** M02 identities/fingerprints MUST reuse core-identity canonical primitives.
- **CORE-R-088 Canonical collection ordering:** unordered graph/basis collections MUST be explicitly sorted before fingerprinting.
- **CORE-R-089 Receipt/handle separation:** a durable receipt MUST NOT directly become a live handle without fresh validation.
- **CORE-R-090 Fixed BVM profiles:** deterministic READ_METADATA, READ_SOURCE, PLAN_WORK, EXECUTE_TOOL_READONLY, MUTATE_SOURCE and GIT_DELIVERY freshness profiles plus assurance overlays are required.
- **CORE-R-091 Conservative FSC:** case/alias semantics remain UNKNOWN unless reliably proved; OS family alone is insufficient.
- **CORE-R-092 No source-tree probe writes:** filesystem-semantics discovery MUST NOT create probe files in user source authority.
- **CORE-R-093 Initial provider baseline:** V0.0 MUST implement hardened system Git behind a provider-neutral trait; alternatives require separate evidence/admission.
- **CORE-R-094 L1 cache only:** persistent proof caching is OUT OF SCOPE for initial M02.
- **CORE-R-095 Watcher-independent baseline:** M02 MUST remain correct with zero watcher events.
- **CORE-R-096 Minimal dependency graph:** core-workspace MUST use only the frozen dependency set unless a governed dependency-admission delta proves necessity.
- **CORE-R-097 No runtime dependency cycle:** core-workspace MUST NOT depend on core-runtime.
- **CORE-R-098 Exact file-map discipline:** implementation MUST remain inside the frozen Round 5 file map except generated evidence or audited Correction Delta.


## M02 Round 6 requirements

- **CORE-R-099 Calibration gate:** M02 implementation MUST pass a mandatory Resource Calibration Gate before production-ready acceptance.
- **CORE-R-100 Evidence-derived numeric defaults:** WorkspaceResourceBudget numeric defaults MUST be derived from reproducible M02 implementation/fixture evidence, never guessed during planning.
- **CORE-R-101 Bounded calibration delta:** the Work Order MAY authorize a post-benchmark delta limited to numeric budget defaults, benchmark thresholds and evidence references.
- **CORE-R-102 No architecture mutation through calibration:** calibration MUST NOT alter ownership, contract semantics, authority classes, dependency graph, provider class, BVM semantics or security invariants.
- **CORE-R-103 Calibration report:** exact candidate evidence MUST include a committed M02 calibration report with environment, fixtures, commands, measurements, selections and rejected candidates.
- **CORE-R-104 No extrapolated success:** unsupported fixture scales MUST be recorded as bounded/skipped rather than extrapolated.
- **CORE-R-105 Deterministic concurrency calibration:** Git/hash concurrency candidates MUST be bounded by host available parallelism and selected from measured valid candidates.
- **CORE-R-106 No unlimited security bounds:** resource configuration MUST NOT use zero/unlimited sentinels for security-sensitive budgets.
- **CORE-R-107 DWS required:** M02 V0.0 MUST implement component/changed-set delta revalidation with proven equivalence to full recomputation.
- **CORE-R-108 WMF deferred:** Workspace Merkle Forest MUST NOT enter the initial Work Order absent a later evidence-backed scalability decision.
- **CORE-R-109 No runtime hidden autotuning:** V0.0 MUST NOT perform background/permanent self-benchmarking or silently rewrite budget defaults at runtime.
- **CORE-R-110 Calibration preserves zero-LLM:** calibration, budget selection and benchmark evaluation MUST require zero LLM inference.

- **CORE-R-111 Baseline provider proof without forced alternative dependency:** the system-Git V0.0 baseline MUST prove its own contract/security/resource obligations; an alternative provider comparison is required only when an alternative is proposed for admission.


## M03 Round 1 requirements

- **CORE-R-112 Machine-verifiable Work Orders:** executable work intent MUST be compiled into a versioned canonical Work Order contract before M04 can instantiate a Run.
- **CORE-R-113 No direct prose execution:** free-form human/LLM prose MUST NOT itself grant execution authority.
- **CORE-R-114 Deterministic Work Order compilation:** equivalent semantic inputs MUST compile to the same canonical Work Order payload/fingerprint under the same compiler/policy generation.
- **CORE-R-115 Immutable revisions:** a frozen Work Order revision MUST be immutable; semantic corrections produce a new revision/fingerprint.
- **CORE-R-116 Distinct Work Order identities:** WorkOrderId, WorkOrderRevision, WorkOrderFingerprint and WorkOrderCompilationId MUST remain distinct concepts.
- **CORE-R-117 Explicit source manifest:** canonical inputs MUST be referenced with authority/provenance/fingerprint/freshness semantics.
- **CORE-R-118 Explicit scope envelope:** allowed and forbidden modules/paths/artifact classes/dependency changes MUST be machine-readable rather than inferred only from prose.
- **CORE-R-119 M02 basis binding:** M03 MUST consume M02 workspace/basis evidence and MUST NOT rediscover or redefine local workspace authority.
- **CORE-R-120 Context Lock binding:** admission MUST bind the Work Order revision to the exact applicable Context Lock/fingerprint or fail stale.
- **CORE-R-121 Governance proof verification:** M03 may verify external governance/admission evidence but MUST NOT invent or self-approve governance authority.
- **CORE-R-122 Stable packet identities:** declared work packets MUST have stable IDs and deterministic ordering/dependency semantics.
- **CORE-R-123 M03/M04 state separation:** Work Order declarations MUST NOT contain mutable Run/Attempt/Step execution state.
- **CORE-R-124 Acceptance-evidence graph:** acceptance criteria MUST map deterministically to explicit evidence requirements or typed evidence-not-applicable rationale.
- **CORE-R-125 Explicit STOP CONDITION:** every executable Work Order MUST compile a machine-readable final stop condition.
- **CORE-R-126 Correction delta classification:** semantic revision changes MUST be classified into governed delta classes such as evidence-only, test-only, calibration, implementation, dependency, scope, architecture and security-policy changes.
- **CORE-R-127 Fail-closed scope delta:** a requested delta outside the revision's allowed correction policy MUST fail typed and require broader governance/new revision.
- **CORE-R-128 Supersession safety:** superseded/stale revisions MUST NOT be eligible for new Run instantiation.
- **CORE-R-129 Event/source staleness safety:** uncertain upstream source/workspace/policy change MUST broaden stale classification rather than preserve READY.
- **CORE-R-130 Compact context manifests:** downstream execution context SHOULD use stable source IDs/fingerprints/packet deltas rather than duplicate complete project documents by default.
- **CORE-R-131 Context budget completeness:** token/context budgets MAY reduce payload size but MUST NOT omit sources marked mandatory by scope/risk/policy.
- **CORE-R-132 HIVE advisory boundary:** HIVE may enrich context and provenance but MUST NOT silently rewrite a frozen Work Order or override newer canonical Git/filesystem evidence.
- **CORE-R-133 Zero-LLM compiler baseline:** canonical parsing, validation, fingerprinting, diff classification and admission checks MUST require zero LLM inference.
- **CORE-R-134 Bounded Work Order resources:** Work Order cardinality/serialized size/packet graph/context refs MUST operate under explicit finite resource budgets before production acceptance.
- **CORE-R-135 Canonical collection ordering:** unordered Work Order collections MUST be explicitly sorted before fingerprinting.
- **CORE-R-136 Secret-safe provenance:** provenance/context manifests MUST reference secrets safely and MUST NOT require durable embedding of raw credentials or secret-bearing payloads.
- **CORE-R-137 Typed failure taxonomy:** invalid/stale/ambiguous/superseded/forbidden Work Orders MUST fail with machine-readable error classes and MUST NOT silently become READY.


## M03 Round 2 requirements

- **CORE-R-138 M03 v1 envelopes:** durable/external M03 payloads MUST carry explicit `nexlabs.core.work-order` schema/version/kind and reject unsupported semantics.
- **CORE-R-139 Contract-layer separation:** authoring request, frozen semantic revision, admission request/receipt and M04 runtime state MUST remain distinct contracts.
- **CORE-R-140 Frozen revision immutability:** `FrozenWorkOrderV1` MUST never be mutated in place after freeze.
- **CORE-R-141 Semantic revision identity:** semantic field changes MUST create a new WorkOrderRevision and WorkOrderFingerprint.
- **CORE-R-142 Non-semantic diagnostics exclusion:** timestamps/wall durations/UI rendering/transport identifiers MUST NOT alter Work Order semantic identity unless explicitly promoted to semantic policy.
- **CORE-R-143 Packet DAG:** WorkPacket declarations MUST form a bounded acyclic graph with deterministic topological order and stable packet IDs.
- **CORE-R-144 Packet scope intersection:** each packet's allowed mutation surface MUST be no broader than the parent Work Order ScopeEnvelope.
- **CORE-R-145 No scheduling authority in packet graph:** packet dependencies MUST NOT themselves grant parallel/concurrent execution authority.
- **CORE-R-146 Workspace requirement not live capability:** frozen Work Order semantics MUST declare workspace/basis requirements rather than persist a runtime-bound M02 WorkspaceHandle.
- **CORE-R-147 Fresh workspace admission:** M03 admission MUST validate workspace requirements against current M02 handle/basis evidence.
- **CORE-R-148 Explicit basis compatibility:** workspace admission MUST distinguish EXACT_MATCH, COMPATIBLE_REFRESH, INCOMPATIBLE and UNKNOWN; UNKNOWN MUST NOT become READY.
- **CORE-R-149 Context Lock exact binding:** execution-capable admission MUST validate required Context Lock schema, Work Order binding, authorized base/source fingerprints and implementation authorization.
- **CORE-R-150 External governance proof:** M03 MAY validate governance admission proof but MUST NOT mint its own governance approval.
- **CORE-R-151 Non-evergreen admission receipt:** READY admission is proof at one exact basis/generation/policy state and MUST NOT be treated as a perpetual capability.
- **CORE-R-152 Run-start revalidation:** M04 handoff MUST carry enough admission fingerprints/generations for M04 to re-check required freshness before creating a new Run.
- **CORE-R-153 Scope deny precedence:** explicit deny MUST override allow and ambiguous scope classification MUST fail closed.
- **CORE-R-154 Dependency permission isolation:** crate/path permission MUST NOT imply authority to add a dependency.
- **CORE-R-155 Execution correction separation:** implementation/test/evidence corrections that remain inside frozen semantics MAY use an ExecutionCorrectionProposal without changing WorkOrderRevision.
- **CORE-R-156 Semantic revision diff:** changes to scope, dependency policy, architecture/security rules, packet graph, acceptance, evidence obligations, context requirements, stop conditions, governance requirements or workspace requirements MUST be semantic revision changes.
- **CORE-R-157 Same-revision correction policy:** same-revision correction classes MUST be explicit and MUST NOT include silent scope expansion, dependency admission, architecture change, security-policy change, acceptance weakening or stop-condition weakening.
- **CORE-R-158 Acceptance graph completeness:** every blocking acceptance criterion MUST resolve to required EvidenceRequirement IDs or deterministic N/A semantics.
- **CORE-R-159 Evidence declaration vs artifact separation:** M03 MUST declare evidence obligations while actual runtime evidence artifacts remain owned by later evidence modules.
- **CORE-R-160 Mandatory machine-readable stop condition:** Work Orders without explicit success/blocked/required-criterion terminal semantics MUST NOT become READY.
- **CORE-R-161 Context budget no-silent-truncation:** context budgets MUST NOT silently drop mandatory semantic sources/fields.
- **CORE-R-162 Governed context expansion:** context budget overflow MUST fail typed or use an explicitly governed expansion reason/policy.
- **CORE-R-163 Source expansion policy:** canonical sources MUST declare expansion behavior such as ALWAYS_LOAD, PACKET_ON_DEMAND, VALIDATE_FINGERPRINT_ONLY or OPTIONAL_DIAGNOSTIC.
- **CORE-R-164 Staleness hints non-authoritative:** change/event/context hints MAY narrow candidate revalidation work but MUST NOT prove freshness by absence.
- **CORE-R-165 Unknown staleness broadens:** unknown source/workspace/lock/governance/compiler/security state MUST broaden to STALE/BLOCKED.
- **CORE-R-166 Secret-free durable Work Orders:** durable Work Order/provenance/context payloads MUST NOT embed raw credentials or secret-bearing provider payloads.
- **CORE-R-167 Stable logical obligation IDs:** packet/criterion/evidence IDs MAY persist across revisions only when they retain the same logical responsibility/meaning.
- **CORE-R-168 Split/merge lineage:** packet/criterion/evidence splits or merges MUST use new IDs plus explicit lineage rather than silently reusing one old ID for changed semantics.
- **CORE-R-169 Contract-first lineage:** M03 V0.0 lineage MUST be representable with versioned contracts and MUST NOT require a database merely for revision relationships.
- **CORE-R-170 Required semantic mechanisms:** WOC, SDF, AEG, CBE, WLG, WSF and WPC semantic capabilities are REQUIRED in the planned M03 V0.0 contract, while implementation shape remains pending.
