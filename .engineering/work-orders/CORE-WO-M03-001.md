# CORE-WO-M03-001 — M03 Work Order Engine

Status: FINAL_FREEZE_CANDIDATE / IMPLEMENTATION_UNAUTHORIZED
Increment: CORE-M03-FREEZE-001
Module: M03 — Work Order Engine
Repository: KayzenRoot/core
Planning base: 786ad33a27d45eb435bc6e63f22174de74b0bb71
Planning branch: planning/m03-final-freeze-round5
Future execution branch: feat/m03-work-order-engine
Risk / assurance: ELEVATED
Execution model: one comprehensive implementation Work Order, eight ordered construction packets
Product implementation authorization: FALSE
Context Lock: .engineering/context-locks/CORE-WO-M03-001.json — PENDING_PROMOTION
Evidence skeleton: .engineering/evidence/CORE-WO-M03-001.json
Codex handoff: docs/work-orders/CODEX-HANDOFF-M03.md

This candidate freezes the future implementation contract only. It creates no M03 product code and grants no authority to begin implementation. Independent exact-head review/promotion and a later execution-admission delta are separate mandatory gates.

## OBJECTIVE

Implement the production-grade M03 Work Order Engine exactly from promoted M03 Rounds 1–4 and this final freeze.

M03 compiles governed work intent into a deterministic, versioned, machine-verifiable Work Order and evaluates admission from explicit, fresh, bounded evidence. M03 does not execute the compiled work. M04 owns Run/Attempt/Step state.

The future implementation must provide the frozen V1 public contracts, pure synchronous services, canonical semantic identity, source and workspace evidence bindings, scope/correction firewalls, packet DAG, AEG/PCM, external lineage/LPC semantics, deterministic admission/handoff, finite calibrated resource limits, and exact-head evidence required below.

Do not reopen settled architecture, contract meanings, dependency direction, security boundaries, or M03/M04 ownership. If frozen semantics prove insufficient, stop and request a governed Correction Delta.

## CONTEXT AND PROMOTED ROUND 1–4 BASIS

Rounds 1–4 are accepted planning truth on the required planning base. Round 4 froze public V1 contracts, the pure service boundary, caller-owned resolver seams, the exact one-crate file/dependency map, property/adversarial/fuzz laws, benchmark protocol, and production DoD direction.

The implementation is a semantic compiler/admission boundary, not an executor. Its service operations are synchronous, stateless-by-default, value-driven, deterministic, and zero-LLM. They have no hidden filesystem, cwd, clock, Git, HIVE, GitHub, network, process, database, refresh, retry, or persistence authority. Caller-owned timeout guards discard late results.

WorkOrderFingerprint represents frozen semantic identity. WorkOrderCompilationId binds compiler/canonicalizer/policy/config/security implementation identity and compilation context separately. Frozen Work Orders declare workspace requirements, not live M02 handles. M02 owns workspace/repository/path truth. External Context Lock and governance evidence may be verified for compatibility but M03 cannot mint approval or implementation authority. External LineageSnapshot/LPC supports compare-and-set; M03 never persists, rebases, commits, pushes, promotes a checkpoint, or self-approves.

Scope deny overrides allow; packet scope is intersected with parent scope; ambiguity blocks. Blocking acceptance criteria map to explicit evidence obligations. PCM may deduplicate source references but never obligations, and mandatory context sources cannot be truncated. HIVE references are advisory only. V0.0 has no persistent compile cache; any later L1 memo is optional, derived, disposable, and correctness-independent.

The planning history is Round 1 promotion Review 001 / Issue #47 / PR #46; Round 2 Review 002 / Issue #49 / PR #48; Round 3 Review 003 / Issue #53 / PR #52; Round 4 Review 004 / Issue #63 / PR #62. Round 4 exact reviewed head was 8fd3f085f93b342373b06e4471088dc7b843fac4, workflow run 35805683331, and promotion merge 78daa752760ba19b3c36c7e2a7574bb3cfd03501. These are historical planning evidence, not M03 implementation evidence.

## HIVE PREFLIGHT

Before future product-code changes, the executor MUST:

1. Resolve the exact repository remote, canonical main, branch, HEAD, and cleanliness.
2. Verify HIVE v1.0.0 read-only MCP availability and attempt to resolve KayzenRoot/core and its checkpoint.
3. Record only observed HIVE project, context, and checkpoint results in docs/evidence/M03-PREFLIGHT.md and the Evidence Bundle.
4. If CORE resolves, use the returned context as an accelerator and record exact provenance/freshness. Canonical Git remains authoritative.
5. If CORE is absent or HIVE cannot provide the required context, use SOLO mode from the exact locked Git sources below. This Work Order explicitly permits that degraded-safe path; mark HIVE context unresolved/unavailable, never PASS.
6. Do not treat another HIVE project's status, a stale index, or an unverified memory result as CORE evidence. Never fabricate HIVE registration, checkpoint, freshness, or health.

Planning preflight for this freeze found the HIVE v1.0.0 read-only MCP surface reachable. project.list returned seven projects and did not resolve KayzenRoot/core; the available READY project status was not CORE, and a CORE checkpoint could not be read. This planning candidate therefore used exact Git sources in SOLO mode. The future implementation executor must repeat the check; this observation is not evergreen execution evidence.

HIVE is an intelligence/context capability only. It cannot grant workspace/path authority, replace canonical source fingerprints, satisfy missing M02 or governance proof, or rewrite frozen Work Order semantics.

## CANONICAL BASIS AND SOURCE HIERARCHY

The required canonical planning basis is origin/main at 786ad33a27d45eb435bc6e63f22174de74b0bb71. The Git blob IDs below bind the nine prescribed source files as read at that base. They are Git object blob IDs, not claims that candidate-branch versions remain unchanged.

| Canonical source | Planning-base blob |
| --- | --- |
| docs/project-brain/13-CHECKPOINT.md | 8963d1bd2f081e99ed57942cca05b51133362971 |
| docs/project-brain/16-DECISIONS-LEDGER.md | 6bbae722236b5040613855d5746488010be4006a |
| docs/project-brain/03-SCOPE.md | f2df9200f88bd0fcc77d9b4a02ad7a039d2f429d |
| docs/project-brain/15-DEFINITION-OF-DONE.md | 20d0825c6164b0f8e5d1912bbdf670b5e4217b05 |
| docs/project-brain/04-ARCHITECTURE.md | d9bd9609e81e57862715ea18fa176e9531287e35 |
| docs/project-brain/02-REQUIREMENTS.md | 2148eb95b408827a1d7c12a6ca059eba48062ce1 |
| docs/project-brain/10-SECURITY-GOVERNANCE.md | 4c7a102a6229de12d4721507db5cfc18a914b59b |
| docs/project-brain/11-TEST-PLAN.md | 61be29929f1edc4ca274dd651c23791320be8d5e |
| docs/modules/M03-WORK-ORDER-ENGINE.md | ceb68fa1c780b6e777d08f4ec16ead4a9e059f3b |

Authority remains governed by .engineering/SOURCE-HIERARCHY.md: Git is repository truth; Checkpoint is project-state authority; Decisions Ledger/ADRs govern decisions; Scope governs scope; Requirements and Architecture govern product contracts; DoD governs completion; this admitted Work Order governs future execution only after admission; Test Plan and exact-head evidence govern validation. GEF/HIVE bridges and summaries are derived views.

The future executor must read in this order: Checkpoint, Decisions Ledger, Scope, DoD, Architecture, Requirements, Security, Test Plan, M03 module plan, CORE Modular Planning and Delivery Model, SOURCE-HIERARCHY, GEF-CURRENT, AGENTS, this Work Order, and the exact active Context Lock. M02 Work Order, Context Lock, and handoff may be consulted only for structure, never as M03 requirements or authority.

At implementation start, compare current canonical sources with the admitted lock. A material change to a frozen source, policy, base, module plan, Work Order, or lock makes the work stale. Stop the affected progression and request re-admission; do not silently recompile, rebase, or reinterpret the design.

## CONTEXT LOCK / STALENESS

The current Context Lock is a planning artifact with status PENDING_PROMOTION, planningBase 786ad33a27d45eb435bc6e63f22174de74b0bb71, authorizedBase null, and productImplementationAuthorized false. It does not activate this Work Order.

Implementation is forbidden unless all of the following are true after independent final-freeze review and promotion:

- a separate governed execution-admission delta is present on canonical origin/main;
- the exact Context Lock on canonical origin/main is ACTIVE and binds a concrete authorizedBase;
- the lock binds this Work Order and current canonical source fingerprints;
- the active lock declares the execution branch feat/m03-work-order-engine and ELEVATED assurance;
- productImplementationAuthorized is true only in that later admitted canonical lock;
- the execution branch is created from the post-admission canonical main and the admitted base is its ancestor with only governance/admission metadata between them;
- Git, HIVE and governance preflight are repeated and recorded.

If any gate is missing, stale, unknown, conflicting, present only on a PR branch, or fails exact-base/source validation, stop and report NOT_AUTHORIZED / STALE. Never alter the pending lock to make execution appear admitted. Any implementation change to source files, Cargo manifests/lockfiles, fuzz manifests, product CI, or runtime crates before those gates is prohibited.

## CONTEXT BUDGET

Use the canonical source set once as a stable prefix. Preserve settled Round 1–4 contracts as the stable semantic base. Then disclose packet-specific source sections, exact changed files, current fingerprints, and failing evidence as delta context.

Use canonical paths and fingerprints rather than copying whole repositories or source bodies into every packet. Fetch additional source detail only when an explicit Work Order obligation requires it. HIVE may provide compact references only when actually available and current. Deterministic Git/hash/static/test evidence precedes model inference. Context limits may reduce duplicated transport, never a mandatory semantic obligation; no silent truncation is allowed.

## RISK / ASSURANCE

Classification: ELEVATED.

The Work Order compiler governs future execution scope and admission. Defects could widen authority, accept stale or substituted sources/workspaces/locks/governance, create nondeterministic identity, omit acceptance evidence, leak secrets, bypass lineage checks, or produce partial READY output under resource exhaustion.

Required assurance includes the frozen contract/API tests, canonical golden/property laws, integration and adversarial fixtures, seven bounded fuzz targets, secret canaries, static no-hidden-I/O and dependency proof, zero-LLM proof, finite-resource calibration, Windows and Ubuntu exact-head evidence, dependency/advisory/license/SBOM checks, and an independent exact-head review with no unresolved HIGH/CRITICAL finding.

## SCOPE

### Necessary future implementation

Implement only CORE-WO-M03-001:

- one focused crates/core-work-order crate and the exact Round 4 path map below;
- V1 public contracts, strict envelopes, typed IDs, validation and deterministic logical ID allocation;
- source manifest and bounded provenance/freshness evidence;
- narrow M02, Context Lock, governance and optional HIVE value-only evidence boundaries;
- synchronous pure parse, compile, validation, semantic diff, correction classification, admission and handoff functions;
- explicit sorted canonical semantic projection, DCR and separate WorkOrderCompilationId;
- ScopeEnvelope deny precedence and packet-scope intersection;
- bounded deterministic packet DAG;
- complete AEG, machine-readable StopCondition, CBE and lossless PCM reconstruction;
- external LineageSnapshot/LPC validation and caller-owned external CAS classification;
- immutable non-evergreen admission receipts and M04 run-start revalidation fields;
- finite deterministic M03ResourceBudget limits selected only in the bounded Pack H calibration delta;
- typed closed error/retryability and bounded redacted diagnostics;
- exact prescribed tests, properties, adversarial fixtures, fuzz targets, bench harness, evidence, security, supply-chain and cross-platform obligations.

### Out of scope

Do not implement or add:

- any M03 Rust source, crate, Cargo workspace membership, Cargo.lock or fuzz manifest/target in this planning increment;
- any M04 or later product module;
- a database, canonical registry, persistence adapter, commit/push/checkpoint update, auto-rebase or self-promotion;
- direct dependency on core-workspace, core-config, core-contracts, sha2, Tokio, Git libraries, filesystem/network/process/clock APIs, HIVE/GitHub SDKs, databases, graph/regex frameworks, a cache store, proptest, Criterion, or a new cryptography stack;
- hidden resolver invocation, refresh/retry, ambient cwd/clock/config/client access, or runtime scheduling authority;
- numeric production resource defaults, measured benchmark claims, or acceptance thresholds before the implementation calibration evidence exists;
- changes to Round 1–4 architecture/contracts, authority, ownership, dependencies, security invariants, file topology, acceptance meaning, or M02/M04 boundaries;
- lock activation, authorizedBase binding, execution-branch creation, implementation authorization, product merge, release/tag, checkpoint promotion, or any claim that M03 is implemented/complete.

## FILES / SOURCES TO READ

Read the canonical sources in the exact order under CANONICAL BASIS. Then inspect only the interfaces needed from:

- Cargo.toml and Cargo.lock, to verify the frozen workspace/dependency graph before implementation;
- crates/core-identity, to reuse canonical_bytes/fingerprint;
- crates/core-contracts only as the existing transitive identity contract reference;
- crates/core-workspace only outside the M03 core, to map public M02 evidence through a caller-owned value adapter;
- the separate fuzz package and its existing libfuzzer-sys configuration;
- existing workflow/static supply-chain configuration to identify required exact-head gates.

Future paths listed below are the complete M03 V0.0 product file map. No additional product path is implicitly admitted. Evidence/report outputs are separately listed under DELIVERABLES and do not widen the Rust file map.

## FROZEN FUTURE FILE MAP — EXACTLY ROUND 4

This is the future implementation map only. None of these M03 product paths may be created by CORE-M03-FREEZE-001. Paths under src/ and tests/ are relative to crates/core-work-order; fuzz/ and benches/ are repository-relative.

| Future path | Frozen responsibility |
| --- | --- |
| Cargo.toml | Add crates/core-work-order as one root workspace member; preserve lockfile discipline. |
| Cargo.lock | Regenerate through Cargo only if adding the workspace member changes the resolved workspace package entries; never hand-edit. |
| crates/core-work-order/Cargo.toml | Declare the minimal direct dependencies below. |
| src/lib.rs | Public V1 exports, schema constants and pure service boundary. |
| src/contracts.rs | Envelopes, request/frozen/admission/handoff, source/scope/context/packet/AEG/lineage/evidence DTOs. |
| src/identity.rs | Typed IDs, validation and deterministic logical-key allocation. |
| src/canonical.rs | Semantic projection, explicit ordering, canonical bytes and fingerprint wrappers. |
| src/source.rs | Source manifest, bounded provenance and freshness rules. |
| src/adapters.rs | External resolver trait and adapter evidence interfaces; no adapter implementation or I/O. |
| src/compiler.rs | Parse/compile/freeze and validation orchestration over explicit values. |
| src/scope.rs | Allow/deny, scope intersection and correction firewall. |
| src/packets.rs | Bounded DAG validation and deterministic topological ordering. |
| src/acceptance.rs | AEG completeness, edge and N/A validation. |
| src/context.rs | Context budgets, source expansion and lossless PCM reconstruction. |
| src/lineage.rs | Snapshot/LPC validation and external CAS result classification. |
| src/delta.rs | Semantic revision diff and same-revision correction classification. |
| src/admission.rs | Explicit source/M02/lock/governance freshness admission and handoff checks. |
| src/budget.rs | Finite M03ResourceBudgetV1 validation and budget checks. |
| src/errors.rs | Closed error categories, codes, safe diagnostics and retryability. |
| src/service.rs | The pure public functions listed above. |
| tests/public_contracts.rs, tests/canonical_identity.rs | Public API/version round-trip, semantic projection, stable IDs and canonical vectors. |
| tests/source_workspace.rs, tests/adapters_no_io.rs | Source provenance, M02 snapshot bindings, freshness and external adapter boundary. |
| tests/scope_packets.rs, tests/acceptance_context.rs | Deny/intersection, DAG order, AEG completeness and PCM reconstruction. |
| tests/lineage_corrections.rs, tests/admission_replay.rs | LPC CAS, semantic deltas, correction policy, stale/replayed admission and handoff. |
| tests/resources_redaction.rs | Finite limits, no partial output and secret-safe bounded diagnostics. |
| fuzz/Cargo.toml | Add one core-work-order path dependency and explicit target declarations to the existing separate cargo-fuzz package; reuse libfuzzer-sys. |
| fuzz/Cargo.lock | Regenerate through Cargo only if the separate fuzz package resolution changes; never hand-edit. |
| fuzz/fuzz_targets/m03_*.rs | Bounded parser/canonicalizer, packet DAG, scope/delta, lineage/LPC, source provenance and admission/replay harnesses. |
| benches/m03_work_order.rs | Stable built-in bench harness for deterministic synthetic fixtures; no Criterion dependency. |

## DEPENDENCY RULES

Allowed direct internal dependency: core-identity only.

Allowed existing third-party dependencies: serde with derive, serde_json, thiserror. Fuzz-only: existing libfuzzer-sys in the separate fuzz package.

The M03 core MUST NOT directly depend on core-workspace, core-config, core-contracts, sha2, Tokio, Git, HIVE/GitHub SDKs, network/process/filesystem/time APIs, databases, persistent caches, graph or regex frameworks, proptest, Criterion, or a new cryptography crate. Reuse core-identity canonical fingerprinting. A caller-owned outer host may depend on M02 and M03 to translate evidence, but that adapter is outside core-work-order. Any dependency change needs a separate governed admission and exact-head review.

## REQUIREMENTS

All accepted M03 requirement IDs in CORE-R-112 through CORE-R-228, inclusive, are in scope for the future implementation. This is a contiguous 117-requirement set; none may be silently omitted. Their exact canonical wording is pinned by the Requirements and M03 plan blob IDs above. Coverage is operationalized by Packs A–H and acceptance criteria AC-001 through AC-023.

- Round 1, CORE-R-112 through CORE-R-137: machine-verifiable, deterministic immutable Work Orders; distinct identities; canonical source/provenance and explicit scope; M02/Context Lock/governance bindings; stable packets separate from M04 state; complete AEG and machine-readable stop; correction/staleness safety; compact complete context; advisory HIVE; zero-LLM; finite resource and secret-safe typed failures.
- Round 2, CORE-R-138 through CORE-R-173: strict V1 envelopes/layer separation; immutable semantic revisions and identity; bounded DAG and intersected packet scope; fresh M02 evidence; exact Context Lock and external governance compatibility; immutable non-evergreen receipts and M04 revalidation; deny/dependency isolation; correction-vs-revision separation; complete AEG/StopCondition; no-truncation and governed context expansion; fail-closed staleness; secret-free durable payload; stable obligation IDs and lineage; required WOC/SDF/AEG/CBE/WLG/WSF/WPC; Context Lock anti-circularity.
- Round 3, CORE-R-174 through CORE-R-207: stateless pure service operations; no hidden I/O; deterministic IDs and external lineage/LPC CAS; monotonic revision; no internal persistence; canonical identity reuse; PCM and DCR; finite resource dimensions with evidence-derived defaults; typed bounded errors/caller-only retry; one-crate acyclic dependency direction; deterministic compile/admission; atomic failure with no partial output.
- Round 4, CORE-R-208 through CORE-R-228: exact public schema/version/kind and typed IDs; explicit semantic projection vs compilation identity; pure no-I/O/no-clock functions and caller timeout discard; bounded source/M02/lock/governance evidence; DAG/scope and complete AEG/PCM; external lineage CAS; closed error/retry domains; calibrated finite resource limits; property/fuzz law matrix; honest reproducible Windows/Ubuntu calibration; exact-head production evidence traceability; no hidden authority dependencies; secret-safe durable payloads; advisory HIVE; exact one-crate dependency map; review candidate distinct from promotion.

No requirement grants permission beyond the scope and later Context Lock activation rules below.

## ARCHITECTURE RULES

- Preserve the accepted Round 1–4 M03/M02/M04 boundaries and the exact public V1 types, fields, enum domains, service functions, adapter DTOs, error classes, and dependency/file map in docs/modules/M03-WORK-ORDER-ENGINE.md.
- Implement parse_request, compile, validate_frozen, diff_revision, classify_correction, evaluate_admission, materialize_handoff, and canonical_semantic_bytes as synchronous pure operations over explicit typed values and M03ResourceBudgetV1.
- Frozen semantic projection includes only the specified immutable Work Order semantics. Sort every unordered collection before core-identity canonical_bytes/fingerprint. WorkOrderFingerprint excludes compiler implementation generations; WorkOrderCompilationId binds compiler/algorithm/policy/config/security and resolved compilation context.
- No service reads a repository/path/cwd/environment/global client or clock, invokes a resolver, Git/HIVE/GitHub/network/process, persists, refreshes, retries, or mutates a frozen revision. Caller-owned orchestration enforces deadlines and discards late results.
- Source, M02, Context Lock, governance, HIVE, lineage/LPC, correction, error/retry, diagnostics and admission fields obey the exact Round 4 DTO and freshness rules. UNKNOWN, mismatch, replay, substitution or missing required proof never becomes READY.
- Packet graphs are bounded DAGs with deterministic topological ordering; deny overrides allow; packet scope is a subset/intersection of parent scope; no scheduling authority is inferred.
- Every blocking criterion has evidence edges or deterministic N/A semantics; mandatory source reconstruction through PCM equals the independently required source set; no partial output is returned on a core resource error.
- READY receipts are immutable historical evidence, not evergreen capabilities; M04 must revalidate carried freshness bindings before Run creation.
- No runtime persistence, database, self-tuning or hidden compile cache. Any separately admitted disposable L1 memo cannot affect correctness.
- HIVE remains advisory. Zero LLM is mandatory for parsing, compilation, validation, fingerprinting, diff, correction classification and admission.

## CONSTRAINTS

- Exact Work Order identity: CORE-WO-M03-001. Future implementation branch: feat/m03-work-order-engine.
- Future execution base remains null until a distinct admission delta after final-freeze review/promotion.
- Do not create any frozen product file-map path during this planning increment.
- Do not change allowed direct dependency set, file map, contracts, architecture, acceptance, security, ownership or M02/M04 boundary through calibration.
- Do not put prompts, source bodies, credentials, user repository data, raw provider payloads or unbounded diagnostics in durable evidence.
- No numeric production values or performance claims before the Pack H evidence exists.
- Historical Round 1–4 evidence is planning provenance only and cannot satisfy future implementation acceptance.
- Any later commit invalidates exact-head evidence that depends on the prior candidate head.

## CONSTRUCTION PACKETS

### Pack A — Contracts and identity

Implement the exact V1 envelope/kinds, closed serialized enums, typed IDs/revisions/fingerprints, strict parsing/version rejection, deterministic caller/logical-key WorkOrderId, typed errors and deterministic budget validation. Frozen/admission/handoff contracts have private fields/read-only accessors as frozen by Round 4.

STOP A: public contracts round-trip at V1; unknown schema/version/kind/enum and malformed/duplicate IDs fail typed; IDs and revisions obey deterministic laws; no product dependency outside the admitted set.

### Pack B — Canonical sources and external evidence seams

Implement versioned canonical source refs and bounded resolution evidence, freshness/provenance/substitution checks, and value-only M02/Context Lock/governance/HIVE evidence DTOs. Declare caller-owned resolver traits only; the pure core never calls them. HIVE references require an advisory-only marker.

STOP B: mismatched, stale, unknown or substituted source evidence is non-current; wrong M02/lock/governance schemas or bindings fail closed; no raw source body, path inventory, secret or live handle is persisted.

### Pack C — Compiler and canonical identity

Implement parse_request, compile, validate_frozen, canonical semantic projection, canonical bytes, deterministic compilation ID and Deterministic Compilation Receipt. Separate WorkOrderFingerprint from compiler/policy/config/security identity. Equivalent semantic permutations compile to identical semantic output; diagnostic-only differences do not change WorkOrderFingerprint.

STOP C: golden vectors and deterministic replay/permutation evidence pass; compilation emits no partial frozen result; fingerprints use core-identity and explicitly sorted collections.

### Pack D — Scope, packets, AEG and context

Implement ScopeEnvelope allow/deny and correction scope rules; packet DAG validity and deterministic topological order; complete criterion/evidence edges and N/A semantics; machine-readable StopCondition; ContextBudgetEnvelope and lossless Packet Context Mesh reconstruction.

STOP D: deny wins, ambiguity blocks, child scope cannot widen, invalid DAG/AEG fails typed, every blocking criterion has a resolvable evidence obligation, and PCM reconstruction preserves each packet's mandatory source set without truncation.

### Pack E — Lineage, revision diff and correction policy

Implement authoritative LineageSnapshotV1/LPC validation and external CAS result classification, monotonic revision semantics, semantic WorkOrderRevisionDiff, stable obligation-ID lineage rules, and same-revision ExecutionCorrectionPolicy classification. The external writer alone performs compare-and-set persistence.

STOP E: stale store generation is LINEAGE_CONFLICT; two competing N+1 proposals cannot both become canonical under one snapshot; semantic/scope/dependency/security/acceptance/stop changes require a new revision; same-revision corrections remain inside explicit policy.

### Pack F — Admission and handoff

Implement deterministic source/workspace/lock/governance/policy admission from already-resolved evidence; immutable non-evergreen WorkOrderAdmissionReceipt; stale/UNKNOWN/replay/supersession rejection; exact READY-receipt matching for AdmittedWorkOrder; M04 run-start revalidation bindings. Do not mint approval or authority.

STOP F: identical input evidence yields deterministic semantic receipt; any mismatch/replay/staleness blocks READY; no receipt mutates; handoff requires the exact frozen revision and matching READY receipt and carries revalidation inputs for M04.

### Pack G — Assurance matrix

Complete unit, public API, integration, deterministic property, adversarial, bounded fuzz, security/redaction, zero-LLM, no-hidden-I/O, dependency, advisory/license/SBOM and Windows/Ubuntu exact-head evidence. Add only the seven frozen M03 fuzz targets to the existing fuzz package, reusing libfuzzer-sys.

Required bounded fuzz surfaces: envelope/canonicalizer; packet DAG; scope/delta; lineage/LPC; source provenance; admission/replay; diagnostics/redaction. Fuzz input, nesting, cardinality and execution work are bounded; seeds are synthetic and carry no repository data.

STOP G: all non-calibration tests and static/security/supply-chain checks pass at the exact candidate head on Windows and Ubuntu; all seven targets satisfy bounded campaign requirements; no unresolved HIGH/CRITICAL finding; candidate is eligible to enter Pack H CALIBRATION_ONLY. Do not claim numeric calibration here.

### Pack H — M03 Resource Calibration Gate and final evidence

Enter CALIBRATION_ONLY only after Packs A–G pass. Run the deterministic synthetic benchmark/resource matrix under CALIBRATION PROTOCOL. Produce docs/evidence/M03-CALIBRATION-REPORT.md with exact candidate SHA, commands, environment/toolchain/CPU, fixture generator/version, selected and rejected candidates, supported/unsupported scales, results, semantic assertions and final finite budgets.

The only pre-authorized Calibration Delta may change numeric M03 resource defaults, benchmark-derived thresholds, fixture metadata/evidence references, and explanatory documentation/comments. It cannot change architecture, dependencies, contracts, ownership, security invariants, file topology, acceptance meaning, or M02/M04 boundaries. A need for any excluded change stops with CORRECTION REQUIRED/BLOCKED and a governed correction path.

After the bounded Calibration Delta, rerun all exact-head format/lint/build/test/property/adversarial/fuzz/security/supply-chain/SBOM and Windows/Ubuntu gates plus calibration/regression benchmarks. Update the evidence bundle and docs/evidence/M03-EXECUTION-REPORT.md. Do not carry forward head-bound results after any later commit without rerunning the applicable gates.

STOP H: all required scenarios are recorded, final resource values are finite, positive and evidence-backed, selected/rejected candidates and unsupported scales are explicit, no architecture change was smuggled through calibration, the full post-delta suite is exact-head green, and the future execution candidate is ready for independent review. If any required scenario fails, times out, overflows, is missing, or cannot be supported, stop BLOCKED; never extrapolate or fabricate success.

## CALIBRATION GATE

The future implementation MUST include a mandatory CALIBRATION_ONLY stage specific to M03 compiler/admission work. This section freezes the rule, not production values.

Calibrate every security-sensitive dimension in M03ResourceBudgetV1: request and frozen serialized bytes; string bytes; canonical source references; packet nodes and DAG edges; scope rules; acceptance criteria; evidence requirements and AEG edges; lineage edges; context references; correction rules; semantic diff entries; diagnostics; parser depth; and caller-owned wall-clock deadline policy at the host boundary. Deadlines are not fields read by the pure core.

Use deterministic local synthetic fixtures only. No network, HIVE calls, LLM, real repository contents, secrets or source bodies. Warm once; collect at least five measured iterations per relevant scenario; report median/min/max, exact command and candidate SHA, OS, toolchain, CPU and fixture generator/version. Run relevant scenarios on Windows and Ubuntu. Assert semantic correctness at every candidate value. Exercise measured candidate caps and cap-plus-one failures for each security-sensitive dimension. Mark unsupported scales as skipped/unsupported and give the observed limiting reason; never extrapolate.

Record fixture families for minimal valid Work Order; source count/bytes/nesting; packet count/width/depth/edges; scope allow/deny and delta fields; criteria/evidence/AEG edges; shared references and PCM reconstruction; lineage edges/revision diffs; admission source/workspace/lock/governance/policy inputs; canonical output size; malformed cycles/dangling references; repeated IDs; near-limit strings; stale admission; and timeout/late-result discard.

No numeric production budget, default, candidate ceiling, latency threshold, memory threshold, or performance claim is supplied by this planning Work Order. Values must be derived from implementation evidence. Select finite positive caps that satisfy semantic/security assertions and the project acceptance policy; retain selected and rejected candidates with rationale. Unsupported scales remain unsupported.

The bounded Calibration Delta may only record the measured numeric defaults/thresholds, fixture metadata/evidence references and explanatory comments described in Pack H. If safe finite values cannot be selected or an architecture/security/contract/dependency change is needed, stop and seek a normal governed correction.

## ACCEPTANCE CRITERIA AND AEG MAPPING

Every criterion below is blocking and MUST bind to the listed future Evidence Requirement IDs. The mapping is also serialized in .engineering/evidence/CORE-WO-M03-001.json. Criteria are declarations, not evidence that implementation has passed.

| Criterion | Blocking acceptance obligation | Required future evidence |
| --- | --- | --- |
| AC-001 | Public V1 envelopes round-trip; exact schema/version/kind and strict unsupported/downgrade rejection. | EV-001, EV-019, EV-021, EV-022 |
| AC-002 | Typed IDs/revisions/fingerprints validate; WorkOrderId is caller supplied or deterministically derived without random/clock/host state. | EV-002, EV-004, EV-019 |
| AC-003 | WorkOrderFingerprint uses the explicit semantic projection; WorkOrderCompilationId binds compiler/policy/config/security identity separately. | EV-003, EV-004, EV-019 |
| AC-004 | Canonical sorting/permutation laws and golden vectors prove stable bytes/fingerprints; diagnostic-only changes are excluded. | EV-004, EV-019 |
| AC-005 | All pure services have no hidden I/O or ambient clock; caller-owned timeout discards late results and no timed-out result is accepted. | EV-005, EV-018, EV-019 |
| AC-006 | Source authority/provenance/freshness and substitution rules reject stale, unknown, missing or changed sources. | EV-006, EV-019 |
| AC-007 | M02 workspace identity/generation/basis/profile/components are matched through bounded value evidence; mismatch/UNKNOWN blocks. | EV-007, EV-019 |
| AC-008 | Context Lock and external governance proof bind exact WorkOrder/revision/fingerprint/base/head/scope/policy and reject replay; M03 never mints authority. | EV-008, EV-019 |
| AC-009 | Packet DAG is bounded, acyclic and deterministically ordered; deny precedence and child-scope intersection prevent widening. | EV-009, EV-019 |
| AC-010 | Every blocking AEG criterion has required evidence edges; PCM reconstruction equals the independent mandatory-source set without silent truncation. | EV-010, EV-019 |
| AC-011 | External LineageSnapshot/LPC rejects stale store generation; competing N+1 candidates cannot both pass external CAS. | EV-011, EV-019 |
| AC-012 | Revision diff and correction classification are deterministic; semantic changes require a new revision and same-revision changes obey CorrectionPolicy. | EV-012, EV-019 |
| AC-013 | Admission is deterministic; stale/UNKNOWN/substituted/replayed evidence never becomes READY; receipts are immutable and non-evergreen. | EV-013, EV-019 |
| AC-014 | Handoff requires the exact READY receipt and carries all M04 run-start freshness/revalidation inputs. | EV-014, EV-019 |
| AC-015 | All resource dimensions are finite/positive after calibration; cap failures are typed and atomic; caller timeout discards late output. | EV-015, EV-024, EV-019 |
| AC-016 | Diagnostics are bounded, typed and safe; secret canaries/raw source/prompt/provider payloads are not echoed or persisted. | EV-016, EV-019 |
| AC-017 | Parsing, compilation, validation, canonicalization, diff, correction and admission use zero LLM inference. | EV-017, EV-018 |
| AC-018 | Static evidence proves the admitted acyclic dependency set and absence of filesystem/cwd/Git/HIVE/GitHub/network/process/database authority in the pure core. | EV-018 |
| AC-019 | Required public API/unit/integration/property/adversarial tests and all seven bounded fuzz surfaces pass without panic, hang, amplification, partial output or secret echo. | EV-019, EV-020 |
| AC-020 | Relevant exact-head implementation checks pass on Windows and Ubuntu. | EV-021, EV-022 |
| AC-021 | Advisory, license, dependency provenance and SBOM evidence pass on the exact final head. | EV-023 |
| AC-022 | Calibration report records selected/rejected finite budgets, supported/unsupported scales and reproducible Windows/Ubuntu measurements. | EV-024 |
| AC-023 | Independent final exact-head review records no unresolved HIGH/CRITICAL finding. | EV-025 |

The acceptance graph is complete only when each AC ID resolves to at least one EV ID, every EV ID has a named producer/evidence class, and actual exact-head artifacts are bound later. A prose statement, plan, green historical run, or reviewer opinion alone does not satisfy an implementation evidence obligation.

## TESTS / PROPERTY / ADVERSARIAL / FUZZ / SECURITY / SUPPLY-CHAIN / CROSS-PLATFORM

Future implementation evidence MUST include at least:

- public V1 envelope round-trip; unsupported schema/version/kind, downgrade, invalid enum/required field and duplicate-ID rejection;
- deterministic identity/revision laws, semantic versus diagnostic projection, golden vectors and equivalent collection/map permutations;
- public API behavior for parse, compile, validate, diff, correction classification, admission, handoff and canonical semantic bytes;
- source resolver evidence binding requested identity to observed fingerprint, authority, provenance and freshness; substitution/stale/UNKNOWN rejection;
- wrong M02 schema/workspace/generation/basis/profile/components; missing required BVM components; wrong/stale/replayed Context Lock and governance exact base/head/scope/policy; all fail closed;
- packet self-edge/cycle/dangling/duplicate ID/budget rejection, deterministic topological order, deny precedence, ambiguous scope and packet non-widening;
- missing/dangling/orphan AEG evidence rejection and exact PCM mandatory-source reconstruction;
- two competing N+1 LPC candidates under one snapshot, stale store-generation LINEAGE_CONFLICT, monotonic revisions and no M03 persistence/rebase;
- semantic revision diff, correction classes, forbidden scope/dependency/architecture/security/acceptance/stop changes and same-revision allowed correction;
- admission replay after source/workspace/lock/governance/policy changes; immutable receipt and no-evergreen READY;
- exact READY handoff matching and M04 revalidation fields;
- every calibrated dimension at limit and cap-plus-one; typed no-partial-output failures; timeout evidence at caller boundary and late-result discard with no core clock read;
- hostile diagnostics, provider strings and synthetic secret canaries prove bounded redaction and no raw echo;
- static dependency/I/O audit, zero-LLM proof and acyclic crate graph;
- seven bounded fuzz targets as listed in Pack G, with synthetic seeds and no filesystem/network/process access;
- dependency advisory/license/provenance checks and SBOM;
- relevant suites and benchmark/calibration evidence on Windows and Ubuntu exact final head.

Use the existing Rust harness and existing separate libfuzzer-sys package. Round 4 admits no property-testing dependency and no Criterion. Historical M01/M02 evidence is not M03 implementation evidence. The current main-protection ruleset CORE main protection (id 23769853) requires these exact seven statuses on this planning PR: Governance; M01 (ubuntu-latest); M01 (windows-latest); M01 fuzz campaign; M02 workspace adapter (ubuntu-latest); M02 workspace adapter (windows-latest); M02 bounded fuzz campaign. Do not weaken or bypass the ruleset. The existing M01/M02 workspace tests run the Cargo workspace; the two current fuzz campaigns enumerate only their existing M01/M02 targets. Run all seven M03 fuzz targets as separate bounded exact-head evidence unless a separately governed CI change admits them. This Work Order does not authorize a workflow/ruleset change.

Hosted results must identify the exact final implementation head; any later commit invalidates affected head-bound evidence and requires applicable reruns.

## DELIVERABLES

The future executor must produce:

1. The M03 crate and only the frozen product file map.
2. Required unit/integration/property/adversarial/fuzz/bench code and synthetic fixtures.
3. docs/evidence/M03-PREFLIGHT.md with truthful Git/HIVE/governance preflight.
4. docs/evidence/M03-CALIBRATION-REPORT.md with the measured matrix, selected/rejected candidates, finite defaults and environment.
5. docs/evidence/M03-EXECUTION-REPORT.md mapping AC-001 through AC-023 to exact-head evidence and residual risks.
6. Updated .engineering/evidence/CORE-WO-M03-001.json with exact authorized base, implementation head, changed files, tests, property/fuzz/security, calibration, CI, risks, corrections and proposed Checkpoint Delta.
7. The exact Context Lock and its source binding as activated only through the separate admission delta on canonical main.
8. One governed implementation PR for CORE-WO-M03-001 and an independent review handoff. Do not merge or self-promote.

## EVIDENCE BUNDLE

Use .engineering/evidence/CORE-WO-M03-001.json. Record actual:
- Work Order/increment ID; repository/remote; planning base; later authorized base; final implementation head; all relevant source and Context Lock fingerprints;
- HIVE availability, CORE project resolution, checkpoint/context evidence or explicit SOLO/unavailable result;
- changed files, packet progress, tests/commands/results, property/adversarial/fuzz/security/zero-LLM results;
- dependency graph, advisory/license/provenance and SBOM results;
- calibration runs, exact commands/environment/fixtures/measurements, candidate selection/rejection, final finite values and unsupported scales;
- exact hosted workflow/run/context names, conclusions and candidate SHA;
- risks, failures, corrections and evidence invalidation after later commits;
- independent review issue/PR/verdict and unresolved HIGH/CRITICAL count;
- proposed Checkpoint Delta and final executor status READY_FOR_REVIEW or BLOCKED.

Never prefill future implementation evidence with PASS, green CI, measured values, an authorized base, an independent verdict, or completion. The current skeleton's future implementation fields remain pending/null until observed.

## REVIEW FORMAT PT-BR

A futura resposta executora deve ser em português brasileiro e incluir:

- increment, Work Order, branch de implementação e status do Context Lock;
- base autorizado e head final exatos, fingerprints de fonte/lock;
- arquivos alterados e resumo dos Packs A–H;
- contagem de critérios e cobertura AC/EV;
- preflight HIVE com resultado real ou SOLO/unavailable;
- testes, propriedades, fuzz, segurança, dependências/SBOM e CI por head;
- relatório de calibração, candidatos selecionados/rejeitados e confirmação de que valores vieram de medições;
- falhas/correções, riscos residuais e proposta de Checkpoint Delta;
- PR e gate de revisão independente;
- veredito executor READY_FOR_REVIEW ou BLOCKED, nunca APPROVED.

## EXECUTOR PERMISSIONS

This planning Work Order grants no implementation permission.

After a separate reviewed/promotion and execution-admission delta activates the exact lock on canonical main, the future executor may implement, test, apply only the bounded Pack H Calibration Delta, update evidence, commit, push, and open/update one governed PR within this scope. It may not widen scope, add dependencies, modify frozen contracts/architecture/security/authority, activate its own lock, merge, self-approve, promote a checkpoint, release, or tag.

The current task may edit only the planning/governance/evidence artifacts named by CORE-M03-FREEZE-001. It may not create any future product path in the frozen map.

## BLOCKER / CORRECTION PROTOCOL

If the future executor finds a reproducible defect or impossibility in the frozen contract:

1. Stop the affected packet and all dependent packets.
2. Preserve valid independent evidence.
3. Record exact source/head, failing obligation, reproduction and impact without exposing secrets.
4. Apply only a correction already allowed by the current frozen scope or Pack H Calibration Delta.
5. Otherwise request the smallest governed Correction Delta; do not redesign adjacent contracts or continue as if the criterion passed.
6. Rerun affected exact-head gates after every correction.

Unknown, missing or stale proof remains BLOCKED/STALE; it never defaults to allow or READY.

## FINAL STOP CONDITION

Stop only in one of these executor states:

### READY_FOR_REVIEW

Only after Packs A–H satisfy the frozen STOP gates, AC-001 through AC-023 are bound to actual exact-head evidence, Pack H calibration and post-delta reruns are complete, all required CI/security/supply-chain evidence is exact-head green, the Evidence Bundle is complete, the implementation PR is open, and no unresolved HIGH/CRITICAL finding remains. This status is a handoff to an independent reviewer, not approval or merge authorization.

### BLOCKED

If any required criterion, evidence, source binding, gate, bounded resource selection or authorization precondition is missing, stale, conflicting, failing, unsupported without an allowed disposition, or requires a change outside frozen scope, stop and state the exact gap. Do not work around governance gates.

The executor MUST NEVER return APPROVED. Approval belongs to the independent governed reviewer. This Work Order cannot be executed until the separate admission delta activates its Context Lock on canonical main.
