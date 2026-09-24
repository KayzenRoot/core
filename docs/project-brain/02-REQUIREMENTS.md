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

- **CORE-R-171 Frozen diagnostic separation:** mutable diagnostic/transport/rendering metadata MUST live outside the immutable FrozenWorkOrder revision; no stored frozen field may change in place as a diagnostic-only edit.

- **CORE-R-172 Context Lock anti-circularity:** FrozenWorkOrder semantic identity MUST fingerprint Context Lock requirements/constraints, not a concrete Context Lock fingerprint that itself binds to the Work Order fingerprint; concrete lock identity is bound in admission evidence.
- **CORE-R-173 Immutable admission receipts:** WorkOrderAdmissionReceiptV1 MUST be immutable evidence of one evaluation; later staleness produces a new evaluation/receipt rather than mutating historical READY proof.


## M03 Round 3 requirements

- **CORE-R-174 Stateless compiler core:** M03 V0.0 core compilation/admission semantics MUST be stateless-by-default and MUST NOT require an internal database.
- **CORE-R-175 No hidden I/O:** core compiler/service operations MUST NOT scan repositories, invoke Git, call HIVE/GitHub/network services or persist Work Orders implicitly.
- **CORE-R-176 Explicit compile operation:** M03 MUST expose deterministic compile semantics from WorkOrderRequestV1 + explicit CompilationContextV1 to FrozenWorkOrderV1 + compilation proof.
- **CORE-R-177 Frozen validation operation:** M03 MUST support deterministic validation of an already frozen revision without mutating it.
- **CORE-R-178 Revision diff operation:** M03 MUST produce deterministic semantic WorkOrderRevisionDiffV1 results.
- **CORE-R-179 Correction classification operation:** M03 MUST classify ExecutionCorrectionProposalV1 against the frozen CorrectionPolicy without mutating the Work Order.
- **CORE-R-180 Admission evaluation operation:** M03 MUST evaluate admission only from explicit resolved workspace/lock/governance/source/policy inputs.
- **CORE-R-181 Handoff materialization operation:** AdmittedWorkOrderV1 MUST be materialized only from an exact FrozenWorkOrder + matching READY immutable admission receipt.
- **CORE-R-182 Deterministic WorkOrderId creation:** M03 MUST NOT generate opaque random WorkOrderIds internally; IDs are explicit or derived deterministically from a versioned logical key.
- **CORE-R-183 External lineage snapshot:** semantic revision compilation MUST consume an authoritative LineageSnapshotV1 rather than assume local latest revision state.
- **CORE-R-184 Lineage CAS precondition:** new revision compilation MUST emit LineagePreconditionCapsuleV1 and canonical persistence MUST verify it before promotion.
- **CORE-R-185 Lineage conflict fail-closed:** if the external lineage/store generation advanced, persistence MUST fail LINEAGE_CONFLICT and require re-resolution/recompile.
- **CORE-R-186 Revision numbering discipline:** canonical revisions start at 1, increase monotonically by one, are never reused, and revision number alone MUST NOT prove semantic identity.
- **CORE-R-187 No internal persistence authority:** M03 MUST NOT commit/push/update checkpoint/self-promote Work Orders; durable repository storage remains external.
- **CORE-R-188 Single canonical fingerprint stack:** M03 MUST reuse core-identity canonical fingerprint primitives rather than introduce a second generic hash/canonicalization framework.
- **CORE-R-189 Canonical projection independence:** semantic identity MUST be independent of incidental pretty-print JSON formatting and map iteration order.
- **CORE-R-190 Packet context plans:** M03 MUST produce packet-specific context source/ref plans without materializing executor prompt text.
- **CORE-R-191 Shared packet context deduplication:** repeated stable source refs across packets SHOULD be represented once through a canonical shared context mesh with lossless packet reconstruction.
- **CORE-R-192 Derived compile memo only:** any compilation cache MUST be derived/disposable, keyed by all correctness-relevant compiler/context generations, and MUST NOT become source truth.
- **CORE-R-193 No persistent compile cache initially:** persistent M03 compile caching is OUT OF SCOPE for V0.0 absent later evidence/admission.
- **CORE-R-194 Explicit M03 resource budget:** request/payload/cardinality/graph/context/diff/diagnostic/time dimensions MUST be typed and finite before production acceptance.
- **CORE-R-195 Evidence-derived M03 budget defaults:** numeric M03 resource defaults MUST come from reproducible implementation calibration rather than architecture guesses.
- **CORE-R-196 No runtime hidden autotuning:** M03 V0.0 MUST NOT silently benchmark/rewrite persistent resource defaults at runtime.
- **CORE-R-197 Typed error categories:** M03 errors MUST distinguish invalid input, stale/conflict, policy block, resource block and internal invariant categories with machine-readable reason codes.
- **CORE-R-198 Explicit retryability:** errors MUST carry deterministic retryability semantics; M03 MUST NOT silently refresh lineage/workspace/governance and retry behind the caller.
- **CORE-R-199 Bounded safe diagnostics:** errors/receipts MAY carry bounded safe IDs/fingerprints/diagnostics but MUST NOT leak raw secret material.
- **CORE-R-200 One-crate initial direction:** M03 V0.0 planning SHOULD target one focused `core-work-order` crate unless evidence later justifies a split.
- **CORE-R-201 Synchronous core baseline:** the core compiler/admission baseline SHOULD require no async runtime/Tokio dependency unless a later proof obligation demonstrates necessity.
- **CORE-R-202 Dependency direction:** core-work-order MAY depend downward on core-contracts/core-identity/core-config/core-workspace but MUST NOT depend on M04+; core-workspace MUST NOT depend back on core-work-order.
- **CORE-R-203 Compilation determinism proof:** repeated equivalent compile inputs MUST produce identical semantic outputs/fingerprints regardless of cache state or collection iteration order.
- **CORE-R-204 Admission determinism proof:** identical frozen revision + resolved admission inputs MUST produce identical semantic receipt/status/fingerprint; UNKNOWN MUST never become READY.
- **CORE-R-205 Deterministic Compilation Receipt:** compilation MUST emit bounded provenance tying request/context/compiler/lineage-precondition/output fingerprints together.
- **CORE-R-206 Packet Context Mesh reconstruction:** reconstructing each PacketContextPlan from the shared context mesh MUST yield exactly the independently required mandatory source set.
- **CORE-R-207 No partial output on resource failure:** resource/deadline failure MUST NOT yield a partially FROZEN or READY contract.


## M03 Round 4 requirements

- **CORE-R-208 Versioned public contracts:** every root M03 V0.0 payload crossing a durable/external boundary MUST use the exact schema identifier nexlabs.core.work-order, a supported explicit version, and a closed kind; nested DTOs inherit that root version or carry an explicit producer schema/version. Unsupported or ambiguous versions MUST fail typed without downgrade.
- **CORE-R-209 Typed semantic identities:** WorkOrderId, WorkOrderRevision, WorkOrderFingerprint, WorkOrderCompilationId, source, packet, criterion, evidence, context and lineage identities MUST be distinct validated types; IDs MUST be caller supplied or deterministically derived without random, time, branch, cwd, map-order or host-state input.
- **CORE-R-210 Explicit identity projection:** the WorkOrderFingerprint semantic projection MUST enumerate included semantic fields and excluded diagnostics; unordered collections MUST be explicitly sorted before reusing core-identity canonical_bytes/fingerprint. Compiler/canonicalizer/policy/config/security implementation generations MUST remain outside WorkOrderFingerprint and be bound by WorkOrderCompilationId.
- **CORE-R-211 Pure service boundary:** parse, compile, validate, diff, correction classification, admission and handoff MUST expose explicit typed inputs, outputs, deterministic budgets and errors; service functions MUST be synchronous/stateless and perform no hidden I/O, clock read, persistence, refresh or retry. Wall-clock deadlines MUST be caller-owned and any timed-out result MUST be discarded before it can become FROZEN/READY/handoff evidence.
- **CORE-R-212 Bounded source evidence:** canonical source resolution MUST occur in caller-owned adapters and return versioned bounded identity/fingerprint/authority/provenance/freshness evidence; raw content and secrets MUST NOT be required in durable M03 payloads.
- **CORE-R-213 M02 evidence binding:** M03 admission MUST consume versioned M02 workspace/basis proof values bound to WorkspaceId, generation, basis fingerprint, required BVM profile/components and provenance; mismatch, replay or UNKNOWN MUST NOT become READY, and M03 MUST NOT duplicate M02 path/repository truth.
- **CORE-R-214 External lock/governance proof:** Context Lock and governance evidence MUST bind the exact WorkOrderId/revision/fingerprint, canonical source/base/head, policy generation and implementation authorization as required; M03 MUST verify compatibility but MUST NOT mint approval or authority.
- **CORE-R-215 Packet DAG and scope firewall:** public packet contracts MUST form a bounded acyclic graph with deterministic topological order; deny MUST override allow and child packet scope MUST be a subset/intersection of parent scope.
- **CORE-R-216 AEG and context reconstruction:** every blocking acceptance criterion MUST resolve to required evidence edges or deterministic N/A semantics; reconstructing packet context from PCM MUST equal the independently required mandatory source set with no silent truncation.
- **CORE-R-217 External lineage CAS:** revision compilation MUST consume a versioned LineageSnapshot and emit an LPC precondition; stale parent/store generation MUST fail closed, and M03 MUST NOT persist, rebase or mutate lineage itself.
- **CORE-R-218 Typed failure/retry contract:** schema, source/provenance, scope, packet DAG, acceptance, lineage, admission/staleness, resource and internal invariant failures MUST have bounded machine-readable codes and explicit caller-only retryability; M03 MUST NOT refresh-and-retry.
- **CORE-R-219 Finite atomic resource limits:** each security-sensitive M03ResourceBudget dimension MUST be finite and positive after calibration; core limit failure MUST return a typed error with no partial FROZEN, READY or handoff result. Wall-clock deadlines are enforced by caller-owned orchestration outside the pure core; expiration MUST yield typed timeout evidence and MUST discard any late result. This refines CORE-R-194's typed finite time dimension by locating time enforcement at the caller boundary rather than inside M03ResourceBudgetV1.
- **CORE-R-220 Property and adversarial laws:** later implementation MUST prove deterministic replay/permutation, DAG ordering/cycle rejection, no scope widening, AEG/PCM completeness, lineage CAS, source substitution/staleness, Context Lock reconstruction, admission replay/staleness, atomic budget failure and diagnostic redaction laws.
- **CORE-R-221 Bounded fuzz surfaces:** implementation MUST provide bounded fuzz targets for envelope/parser/canonicalization, packet DAG, scope/delta, lineage/LPC, source provenance, admission/replay and diagnostic redaction.
- **CORE-R-222 Honest benchmark policy:** benchmark scenarios MUST scale sources, packets/edges, criteria/evidence, context refs, revision diffs and admission evidence; report reproducible platform/toolchain/fixture/command/results; cache cases apply only if a cache is separately admitted; planning MUST NOT claim measurements that do not exist.
- **CORE-R-223 Production evidence traceability:** M03 V0.0 production DoD MUST trace requirements to contracts, tests, security/property/fuzz/performance evidence, Windows/Ubuntu exact-head CI, supply-chain evidence and final acceptance.
- **CORE-R-224 No hidden authority dependency:** the pure core MUST NOT directly depend on filesystem, Git, HIVE, GitHub, network, process, database or host/runtime I/O APIs; external resolvers/adapters own those boundaries.
- **CORE-R-225 Durable secret safety:** raw credentials, prompt/source bodies, secret-bearing provider payloads and unbounded external diagnostics MUST NOT enter durable contracts or diagnostics; redaction must be proven with secret canaries.
- **CORE-R-226 HIVE remains advisory:** optional HIVE context MUST be represented only by bounded versioned references and provenance; unavailable or stale HIVE context MUST NOT fabricate proof, change canonical source authority or silently alter frozen semantics.
- **CORE-R-227 Acyclic minimal crate map:** the M03 V0.0 file/dependency map MUST remain one focused crate with explicit test/property/fuzz/benchmark targets and no M02 reverse dependency, M04+ dependency or unadmitted external authority.
- **CORE-R-228 Review candidate is not promotion:** Round 4 canonical state MUST identify planning as a review candidate/in progress, keep implementation unauthorized, and prohibit checkpoint promotion until independent exact-head review and applicable governance gates pass.


## M04 Rounds 1-3 requirements

- **CORE-R-229 M03-gated Run creation:** M04 MUST create a Run only from a current M03 READY handoff whose WorkOrder identity, revision, fingerprint and required freshness bindings are revalidated at the Run boundary.
- **CORE-R-230 Typed execution identities:** RunId, AttemptId, StepId, EventId, RunGeneration, ExecutionEpoch, IdempotencyKey, JournalRoot and CanonicalFingerprint MUST be distinct validated domain types; raw cross-domain substitution MUST fail typed.
- **CORE-R-231 Closed lifecycle transitions:** Run, Attempt and Step lifecycle transitions MUST use explicit closed legal-transition tables; illegal or ambiguous transitions MUST fail without semantic state advancement.
- **CORE-R-232 Durable initial and immutable terminal states:** Run CREATED, Attempt CREATED and Step DECLARED MUST be durable initial facts; terminal states MUST be immutable and MUST NOT transition back to active.
- **CORE-R-233 Append-only attempts and steps:** retries/continuations MUST create new Attempt/epoch records; prior Attempts/Steps MUST NOT be rewritten or ordinal-reused.
- **CORE-R-234 Parent-child closure:** parent terminalization MUST block new descendants; Run/Attempt success MUST require all required child dispositions to satisfy frozen completion rules.
- **CORE-R-235 Run generation serialization fence:** every semantic M04 commit MUST compare expected Run generation and atomically advance it exactly once; stale/future generations MUST return GENERATION_CONFLICT with no partial commit.
- **CORE-R-236 Atomic state fence:** projected state, canonical event append, journal-root update, idempotency record and resulting Run generation MUST become visible as one semantic commit at the storage adapter boundary.
- **CORE-R-237 Fingerprint-bound idempotency:** exact replay of the same semantic operation under the same scoped idempotency key MUST return the recorded result without a second event; conflicting semantic reuse MUST fail IDEMPOTENCY_CONFLICT.
- **CORE-R-238 Monotonic cancellation:** once Run cancellation is durably accepted, later child admission/activation MUST reject except explicitly bounded closeout; prior committed outcomes remain historical fact.
- **CORE-R-239 Causal ordering over wall clock:** semantic order MUST derive from generation/ordinals/event sequence, never diagnostic timestamps or host clock ordering.
- **CORE-R-240 Journal replay integrity:** replay MUST accept only a contiguous domain/lineage/generation/root-consistent canonical event sequence and MUST fail closed on reorder, truncation, substitution or corruption.
- **CORE-R-241 Continuation integrity:** ICF continuation MUST bind the last committed M04 generation/event/root and M03 BRC fingerprint; resume MUST revalidate all bindings and create a new Attempt/epoch, otherwise fail STALE_CONTINUATION/BLOCKED.
- **CORE-R-242 M03 BRC exactness:** Run admission/continuation MUST bind exact M03 WorkOrder, workspace basis, Context Lock, governance/source generation and admission receipt evidence; changed, UNKNOWN or unverifiable authority MUST block activation.
- **CORE-R-243 Finite M04 resource limits:** attempts/run, steps/attempt, events/run, canonical event bytes, diagnostics, cursor bytes, external references and replay depth MUST be finite positive production limits; cap+1 MUST fail typed with no partial advancement.
- **CORE-R-244 Durable journal authority:** lifecycle events, generations, identities/ordinals, idempotency records, fingerprints, roots, BRC/ICF and reason codes are durable authority; summaries/current state/latest-child views are derived and rebuildable.
- **CORE-R-245 Versioned public contracts:** M04 MUST expose explicit versioned request/receipt/result contracts for admission, attempt creation, step declaration, transition, cancellation, continuation, replay and bounded reference attachment.
- **CORE-R-246 No ambient authority in core API:** M04 core requests MUST NOT receive ambient filesystem, repository, network, process, database, HIVE, GitHub or clock authority; required evidence enters through explicit validated inputs/adapters.
- **CORE-R-247 Closed event and reason registries:** V1 durable event kinds and machine reason/error classes MUST be closed/versioned registries; unknown kind/version MUST fail typed without silent downgrade.
- **CORE-R-248 Deterministic canonicalization:** semantic fingerprints MUST use schema-bound canonical bytes with explicit field order/lengths, deterministic enum forms, domain separation and cross-platform golden vectors; unordered iteration, locale, diagnostics, timestamps and secrets MUST NOT influence semantic identity.
- **CORE-R-249 Snapshot non-authority:** snapshots MAY accelerate projection only when bound to a verified journal boundary; they MUST NOT authorize state absent from the canonical journal or destructively replace active proof history.
- **CORE-R-250 Bounded external references:** M04 MAY persist only versioned bounded lineage-bound references/attachment facts for later-module outcomes/evidence; it MUST NOT import external artifact bodies or later verification-policy truth.
- **CORE-R-251 Acyclic adapter direction:** M04 MAY consume admitted M01/M03/shared primitive contracts and external storage/reference ports, but MUST NOT depend on M05+ execution/policy/verification implementations or create reverse dependencies into M03.
- **CORE-R-252 Zero-LLM and hidden-I/O-free state core:** deterministic lifecycle/projection/replay semantics MUST require zero inference and perform no hidden filesystem/network/process/database/HIVE/GitHub I/O.
- **CORE-R-253 Blocking M04 Evidence Graph:** final M04 acceptance MUST satisfy EV-M04-001 through EV-M04-023, including contracts, transitions, concurrency/CAS, idempotency, cancellation, replay integrity, BRC/ICF, identity substitution, canonical vectors, resources, snapshots, references, no-hidden-I/O, zero-LLM, fuzz, calibration, supply chain, Windows/Ubuntu exact-head CI and independent review with zero unresolved HIGH/CRITICAL.
- **CORE-R-254 Evidence-derived calibration:** exact production M04 numeric resource defaults MUST come from reproducible bounded implementation calibration; an authorized calibration delta MAY change numeric limits only and MUST NOT mutate semantics, contracts, authority or dependencies.
- **CORE-R-255 Review candidate is not execution authority:** M04 discovery/planning candidates MUST keep product implementation unauthorized until final planning freeze and a separate governed execution-admission delta are independently reviewed and promoted.
