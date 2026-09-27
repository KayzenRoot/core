# M06 Capability Negotiation — Round 1 discovery candidate

Status: R1_R3_DOCUMENTED / R4_NON_AUTHORITATIVE_LIMITED_V0_CANDIDATE  
Initial protected-main base: 7c68b829541ddbbc97a1a957f64883e23c26a94a  
Work Order: https://github.com/KayzenRoot/core/issues/154  
Public M06 API frozen: NO | Product code authorized: NO | M05 host API admitted: NO | M04 V1/V2 contract accepted: NO

This document is **discovery-only** under the canonical Master Module Map's M05–M24 planning allowance. It is not an admission to implement M06, a new canonical decision, a replacement for the existing M01 registry, or evidence that an actual HIVE/third-party host is available. The M04 external prior-V1 consumer gate #111 and HIVE owner-local gate #4 remain separate.

## 1. Mission: the gap above an already capable M01 registry

M06 may eventually normalize **independently corroborated provider capability/transport observations** into bounded, policy-constrained negotiation *proposals* for a caller with an already-admitted task and explicit runtime context. M06 must make missing requirements, incompatible versions, unavailable features, degraded providers and cross-capability constraints explicit to the caller **before** requesting an actual M01 binding. Its output is an observation/decision proposal with exact input and policy provenance, never a credential, sandbox permit, M04 journal mutation or assertion that a remote service has performed the requested action.

The existing **M01 CapabilityRegistry already performs deterministic provider eligibility, preference, binding, substitution and leases**. Any later M06 addition must have a source-demonstrated integration need, especially externally verified protocol-to-capability translation and multi-capability plan feasibility; a wrapper that re-implements M01's existing selection is an unnecessary module and should be rejected at final planning review.

## 2. Exact implemented baseline, not speculative M06 functionality

The inspected existing code is `crates/core-registry/src/lib.rs` Git blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14` plus `crates/core-contracts/src/lib.rs` Git blob `0cd1d56a4ac761ff1c8b761cde0e0d19dfa76bc9`.

- M01's `CapabilityRequirement` already models semantic contract/version, mandatory features, quality floor, policy, required authorities, minimum assurance/trust, required and preferred provider origins/class, ownership, cache affinity and optional finite latency/cost ceilings. `SemVer::compatible_with` requires equal major and actual version >= the requested version, **not** equal major alone.
- M01's `CapabilityProviderDescriptor` already carries declared origin, feature set, quality, health, trust, activation generation, fingerprint, authority requirements, readiness/quarantine, dependencies and evidence dependencies. **A provider-generated descriptor and its digest prove only equality of the declared bytes, never independently verified identity, authority or truthful quality.**
- M01's `resolve_from_state` filters all the above required dimensions, chooses a deterministic preference order, and fails with `QualityFloor` or `NoProvider` instead of silently lowering a floor. `origin_rank` already gives `HiveExternal` precedence for genuinely HIVE-owned intelligence and `CoreNative/CoreFallback` precedence for CORE-owned action.
- M01 already exposes `resolve`, `bind_with_generation`, `substitute_with_generation`, `acquire_lease_with_generation`, `validate_lease`, lease expiration/revocation/change notification, and `generation_coherence`. Its binding and per-capability leases are authoritative once correctly admitted, not a provisional M06 proposal. M01 does **not** expose a documented all-or-nothing multi-capability binding/lease transaction; M06 must not claim group atomicity from sequential calls.

M05's non-authoritative R1–R4 source at `docs/modules/M05-HOST-ADAPTER-FABRIC.md`, Git blob `222dace08091637e0192eb0259ba13109e418ba7`, proposes bounded observed host handshakes, protocol version, epoch/generation, peer-proof *reference*, session health and tainted claimed features. M05 does not authenticate an arbitrary host, grant process/network authority, negotiate a provider policy or supply a final published API. M06 must not hard-code its proposed DTOs until M05 receives its separate final planning freeze.

## 3. Authority matrix: no second registry, policy engine or model router

| Source/owner | Existing or future responsibility | M06 permitted candidate contribution |
| --- | --- | --- |
| M01 runtime/registry | Supervisor/runtime epoch, provider registry/health/quarantine, deterministic eligibility and origin preference, authoritative bindings/leases/revocation | Read caller-authorized snapshots and delegate actual resolve/bind/lease/coherence calls; **never duplicate or override** the M01 registry. |
| M02 workspace/M03 Work Order | Exact workspace and admitted scope/Context Lock/Work Order provenance | Consume only caller-owned exact references; no ambient directory, prompt or memory as authorization. |
| M04 execution state | Durable Run/Attempt/Step and journal/idempotency history | Candidate proposal uses an opaque authorized caller operation reference, **not** draft #118's unaccepted durable V1 bytes. No M06 -> M04 journal mutation. |
| M05 Host Adapter Fabric | Independently owned transport/session and honest external observations, future admission pending | Map *observed* transport/protocol/feature claims to typed negotiation inputs only after verification by actual trust owners. |
| M06 candidate | Cross-boundary semantic capability/protocol proposal, dependency feasibility and explicit unsatisfied requirement reporting | No provider trust minting, actual atomic group binding, actual tool/model selection or side effects. |
| M07–M09 | Specialist binding, agent orchestration and model/effort choices | M06 may report compatibility/availability, but must not silently pick an agent, prompt, model or effort tier. |
| M10/M11/M12/M22 | Execution policy, independent host origin/security proof and sandbox, actual tool/command execution and security enforcement | Require externally admitted proof/authority receipts. A host's MCP tool list or HIVE search result does not become permission. |
| M18/M19/M23/M24 | Recovery decisions, budget/cost policy, HIVE federation, telemetry/event transport | M06 reports bounded failures, resource/provenance observations only; no retry/recovery, fabricated HIVE health, automatic provider spend or duplicate telemetry spine. |

## 4. SOLO and HIVE-connected candidate behavior

**SOLO:** the native CORE registry and any independently admitted minimum deterministic local fallback remain usable with no Docker, MCP server, model, remote network or credential. If a nonfallback critical capability is missing, report it unavailable and **block that dependent action**, not invent a less-trusted substitute. CORE must not reproduce HIVE-owned memory/RAG/HUE/Decision Fabric as its own second intelligence stack.

**HIVE-connected:** HIVE owns durable memory, retrieval and context intelligence. A HIVE capability may be proposed only if the actual caller supplied current *verified* HIVE release/transport/feature/identity proof and the existing M01 descriptor/binding meets the requested version, trust, quality and policy floor. The CORE runtime is pinned to HIVE v1.0.0 in current integration docs; an MCP **protocol** identifier such as `mcp-core-surface-v1` is not a software release or proof that the owner's local HIVE Docker/Codex client exists. Separate real evidence issue #4 must not be auto-closed by planning.

**Fallback:** M01 already prefers HIVE origins for HIVE-owned intelligence when an eligible, admitted provider exists. If HIVE is unproven/absent, a minimal native fallback is allowed only within the caller's exact admitted requirements and source/capability ownership. No silent downgrade of accuracy, required features, minimum assurance, policy, latency ceiling or allowed authority. A missing required feature returns typed BLOCKED even if a nominally preferred provider has a higher advertised score.

## 5. Round 1 provisional negotiation lifecycle, no final API

Candidate stage names are explanatory, **not** frozen enums:

1. **REQUIREMENTS_BOUND:** caller supplies one or more canonical existing `CapabilityRequirement` values, source/policy/Work Order reference and M01 runtime generation. If source/policy is stale, deny without discovery.
2. **OBSERVATIONS_COLLECTED:** read an already authorized M01 provider snapshot and, where a later admitted M05 exists, its bounded observed endpoint/schema/feature claims. Missing/remote host observations are `UNKNOWN`, not trusted capability.
3. **PROVENANCE_CORROBORATED:** separately admitted M11/M22 peer/process identity evidence and M10 policy authorization bind the observations to current M01 descriptor origin/fingerprint/activation generation. A valid protocol handshake or descriptor fingerprint **alone fails** this gate.
4. **FEASIBILITY_PROPOSED:** delegate each authoritative eligibility/quality/version check to M01's existing resolver using explicit caller policy. For a multi-capability plan, report which requirements are independently satisfiable and which provider dependencies might conflict; **do not assert global atomic feasibility from sequential per-capability resolves**.
5. **BINDING_REQUESTED:** on an independently admitted caller action, request authoritative M01 binding(s) with exact runtime generation. Verify/return actual immutable binding/lease receipts, and revalidate if a provider changed between proposal and bind. Never forge a lease or pretend M06 writes to the registry.
6. **OBSERVE_CHANGE / REFUSE:** M01 revocation, health/quarantine, policy/config/module/capability graph changes or lost M05 host session make a previous proposal **STALE**. A safety-critical active lease cannot be silently rerouted: the owning policy/recovery layer must decide a governed substitute/abort under the actual M01 APIs.

A future negotiated proposal should contain only bounded requirement fingerprints, observed vs verified feature/provenance references, exact runtime/graph/policy generation, explicit unsatisfied reasons, candidate selection constraints and a short expiry/change guard. **Final field names, serialization, provider sorting and cache TTL are UNDECIDED**; if current M01 already supplies identical facts, M06 should simply reuse its result rather than create a second source of truth.

## 6. Failure, threat and resource hypotheses

- **Bad-version/feature:** unsupported required SemVer major/minor/patch, unknown security-critical protocol extension, unverified feature list or provider manifest spoof. Return `INCOMPATIBLE`/`PROVENANCE_UNVERIFIED` rather than weakening requirements or confusing MCP protocol with HIVE release.
- **Policy/quality floors:** absent required authority, below-floored trust/quality/assurance, incompatible sandbox/isolation, unknown local process owner, stale Context Lock or ambiguous Work Order -> `AUTHORITY_BLOCKED` or `NO_ELIGIBLE_PROVIDER`.
- **Lifecycle:** unhealthy/quarantined/disconnected provider, revoked/expired lease, changed M01 epoch/config/capability graph/binding generation, stale observed M05 generation, late host response -> `STALE` and explicit caller re-evaluation without blind continuation.
- **Cross-capability coupling:** dependency cycle, contradictory provider-origin constraint or capability graph drift between multiple binds requires explicit `PLAN_NOT_ATOMICALLY_ADMITTED` / BLOCKED, never partial application advertised as a complete plan.
- **Hostile data:** untrusted MCP tool metadata, provider-supplied `performance_score`/`trust`/identity, unknown provider claims, secret-bearing errors and prompt injection may not rewrite policy, owner trust or real verification status.
- **Costs/resource:** only compare metrics if measured, normalized and admitted by M19; `max_cost_milli` and `max_latency_micros` in M01 are caller policy inputs, not performance measurements. Proposed finite snapshot/observation bytes, provider/requirement cardinalities, negotiation attempts, timeout and redacted diagnostic budgets need later calibration; **no guessed numeric caps**.
- **Telemetry/evidence:** future M24 typed status may report requirement/provenance fingerprints, generation, rejection code and coarse bounded counts; never raw prompts, host response, credentials, machine path, project UUID or HIVE retrieved snippet. An eligibility observation is not a tool-result proof or an M04 event commit.

## 7. Round 1 candidate negative/evidence matrix: all PENDING

These IDs are discovery hypotheses, **not** a final accepted M06 Acceptance-Evidence Graph and not executed product tests.

| ID | Future evidence obligation |
| --- | --- |
| EV-M06-D01 | Existing M01 `resolve/bind/lease` reuse and proof no competing provider registry or contradictory eligibility algorithm. |
| EV-M06-D02 | Same-major/required-semver-version and mandatory feature downgrade rejection on verified normalized host observations. |
| EV-M06-D03 | Self-attested provider origin/trust/assurance/fingerprint and forged MCP tool metadata cannot authorize a capability. |
| EV-M06-D04 | Source/Work Order/Context Lock/policy provenance and M01 runtime generation fail-closed under stale input. |
| EV-M06-D05 | Exact caller quality/trust/assurance/authority/origin/class, finite cost/latency requirements never silently weakened. |
| EV-M06-D06 | SOLO/no-HIVE minimum local fallback only when exact required floors pass; missing critical feature blocks. |
| EV-M06-D07 | HIVE-owned eligible origin preference only with actual separately verified compatible running context, never Git-tag/protocol conjecture. |
| EV-M06-D08 | Provider degradation/quarantine, binding-generation/epoch drift, active lease/revocation and late host observation rejection. |
| EV-M06-D09 | Multi-capability dependency-cycle/plan drift, partial binding and unsupported atomic-group claim fail closed. |
| EV-M06-D10 | No specialist/agent/model selection, shell/tool execution, repo mutation, M04 journal write, network/HIVE memory clone or hidden I/O. |
| EV-M06-D11 | Untrusted metadata/prompt injection/credential/path/log redaction and bounded external diagnostic handling. |
| EV-M06-D12 | Host-version/manifest fuzz or property fixtures, stable typed decision/failure outcomes and zero-LLM deterministic core. |
| EV-M06-D13 | Finite provider/requirement/snapshot budgets calibrated against actual M01 registry and selected host transport, reproducible Linux/Windows behavior. |
| EV-M06-D14 | Future exact-head source/lock, full Ubuntu/Windows/CI/security/SBOM/fuzz evidence and logical owner review explicitly NOT INDEPENDENT after separate product admission. |

## 8. Nineteen planning dimensions and deferred decisions

Round 1 proposes mission/ownership, HIVE overlap and SOLO/HIVE behavior, high-level input/output provenance, reused M01 data/bindings, lifecycle and fail-closed decisions, threat taxonomy, resource/telemetry dimensions, tests and STOP. **Still UNFROZEN:** executable M06 public/internal DTO contracts and versioned APIs; actual crate/file map and dependency graph; M05's admitted host observation interface; M10/M11/M22 trust-policy receipt fields; any multi-capability atomic admission extension to M01; concrete provider onboarding and accepted transport/dependency types; measured finite budgets; migration/version compatibility; complete acceptance/DoD; owner-audited execution Work Order and ACTIVE Context Lock. A later Round 2 must first test whether the proposed M06 boundary adds real value beyond already-implemented M01; redundant wrapper proposals must be rejected.

**OUT OF SCOPE:** Rust code, SDK/dependency addition, remote provider setup, actual Codex/HIVE local evidence, modifying M01 eligibility or lease semantics, selecting M05's public API, frozen M04 replay/BRC fields, active M04 source/lock, M07–M09 agents/models, M10/M11/M12 policy/tool execution, GitHub delivery, numeric speedup percentages or mandatory network availability.

## 9. Round 1 STOP

Promote only this non-authoritative discovery document plus its separately bound source evidence after exact-head applicable Governance and required status contexts, zero unresolved HIGH/CRITICAL, scoped logical owner self-audit `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT`, protected squash merge, then **new real FULL 11/11 main-push CI**. That closes a planning increment only: the M06 public contract and implementation still require distinct final planning freeze and executable Work Order admission. It cannot resolve external M04 prior-V1 issue #111, owner-local HIVE issue #4 or any EV-M04/M05/M06 product evidence.


---

## Round 2 — dependency graph feasibility, authority-safe group proposals and gap assessment

Status: R2_NON_AUTHORITATIVE_GROUP_FEASIBILITY_CANDIDATE  
Work Order: https://github.com/KayzenRoot/core/issues/157  
Exact initial protected-main basis: c2e8c0a307408e77028b5a1c8f99326afee03927  
Existing M01 registry remains the sole authoritative eligibility, binding and lease owner. **No public M06 DTO, group transaction, numeric budget or implementation is frozen.**

### R2.1 — Specific M01 capability and the genuinely missing property

Actual accepted `core-registry` blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14` contains four mechanisms relevant to M06 planning:

1. `ModuleRegistry.validate_graph` detects cycles in **module startup** dependencies (`ModuleManifest.startup_dependencies`), not an already-demonstrated admission of provider `dependency_capabilities`.
2. `CapabilityRegistry.graph_snapshot` returns the **currently registered** provider descriptors and active binding identities/provenance within one registry read lock. This is a bounded observation if its cardinality and source access are later admitted, not a frozen transaction or independently authenticated descriptor.
3. `CapabilityRegistry.substitution_impact_details` **already traverses transitive dependent capabilities** starting from an *active binding*, reports active leases, evidence dependencies, cache affinities and safety-critical dependent capabilities, and whether revalidation is needed. M06 must reuse its result when considering replacement, rather than implement an inconsistent second impact walker. It is an impact view, **not** proof that a new group of candidates can be admitted.
4. `resolve` checks an individual requirement's contract, required features/authorities, assurance/trust/quality, origin/class, policy/cache, latency/cost and health/readiness/quarantine, then applies the accepted preference order. `bind_with_generation` / `acquire_lease_with_generation` and `validate_lease` are **per capability**, and `generation_coherence` compares the accepted runtime/activation/binding basis. `resolve_from_state` does not traverse each eligible provider's `dependency_capabilities` before returning a selected candidate. A `graph_snapshot` followed by separate `resolve` calls does **not** share one atomic lock/transaction.

`core-contracts` blob `0cd1d56a4ac761ff1c8b761cde0e0d19dfa76bc9` exposes provider `dependency_capabilities` and `evidence_dependencies` as different sets. The latter cannot be silently treated as a capability provider, authenticated artifact or loaded file because the current type provides no general verification protocol for arbitrary evidence string identifiers.

**Only proposed net-new value for M06:** explain graph-level feasibility of a *caller-authorized set* without promoting individual eligibility to a false group admission. It may compose **read-only** M01 observations and policy-proof references. It must not create a competing provider registry, copy M01's requirement filter or modify frozen M01 semantics without a distinct reviewed change.

### R2.2 — Candidate pure, bounded graph preflight, all findings provisional

The following stages describe a potential **pure, side-effect-free, zero-LLM** M06 planning function. Its final public signature, traits, limits and failure encoding remain unfrozen.

1. **Bind caller input.** Accept an explicit finite set of caller-owned `CapabilityRequirement` values and references to the *already admitted* M03 scope/Work Order and M01 `RuntimeGeneration`; no free-form model prompt, ambient workspace or remote manifest can add requirements. Reject empty/duplicate conflicting requirements rather than silently choose one version/quality policy. A repeated capability name with a different required major, feature, origin, policy, security floor or budget is a conflict until the caller supplies one authoritative merged requirement; no OR merge or floor downgrade.
2. **Acquire one observation.** Request a bounded M01 `graph_snapshot` in a single read operation and capture the caller-supplied current runtime/graph/policy generation and verified provenance **references**. Snapshot consistency applies to the descriptor/binding collection *at the time of that call*; no existing API guarantees a later independent `resolve` sees the same registry contents or proves the runtime generation was globally locked.
3. **Corroborate input classes.** Separate trusted caller/M11/M22 evidence of provider/process/source identity from the descriptors' claimed `origin`, `features`, `trust`, `health`, `performance_score` and `fingerprint`. Provider claims remain tainted until an independently admitted port authenticates them. If independent proof for a required external provider is missing, report `PROVENANCE_UNVERIFIED`, not eligible. No HIVE protocol identifier, release tag or signed-looking digest alone is a verified owner-local runtime.
4. **Use M01, never reimplement eligibility.** For each **already-authorized canonical requirement**, obtain an M01 `resolve` result for a best eligible candidate under the accepted single-capability ordering, and bind its descriptor identity back to the snapshot. If resolution returned no provider or selected a fingerprint/activation generation missing or changed in the observation, report BLOCKED/STALE. A proposed graph-wide search for alternative combinations would need **a separately admitted M01 read-only eligible-set API** with the *identical* accepted selector semantics; M06 cannot secretly fork that algorithm and claim equivalent security.
5. **Examine dependency closure.** For each provisionally selected descriptor, inspect `dependency_capabilities` transitively using only bounded snapshot data. A dependency must either be explicitly present in the authorized requirement set or be independently shown to have an **existing, caller-authorized, still-current M01 binding/lease**, with its relevant trust/policy/version proof. A named dependency cannot silently enroll a new capability or grant authority. Missing dependency, self-edge, cycle, name collision, incompatible per-edge requirement or provider-choice contradiction produce a typed **provisional plan BLOCKED** result. Provider dependency names alone do not encode version, security floor or a verified binding; those must be separately supplied by a future governed capability-contract source.
6. **Keep evidence dependencies separate.** For every `evidence_dependencies` reference, require the respective source/security/evidence owner to specify an authoritative verification port and the caller's scope. Merely finding a same-named capability, cache affinity or textual evidence reference in `graph_snapshot` is insufficient. Absent or ambiguous references yield `EVIDENCE_NOT_VERIFIED`; no content fetch or M04 journal mutation occurs in the pure function.
7. **Compute conservative result.** A dependency-complete, conflict-free set may be described only as `PROVISIONAL_FEASIBILITY` at the observed generation, with exact referenced M01 descriptors, unsatisfied/UNKNOWN conditions and the missing independent proof gates. Any conflict, inability to enumerate needed alternatives, stale provider, unverified external authority or inability to guarantee group atomicity yields `BLOCKED` or `NOT_ATOMICALLY_ADMITTED`, **never** `READY`, a live lease, a tool permit or an M04 Run state. The caller may request separately governed M01 per-capability bindings only when no actual side effect can occur until all independent execution gates are met.

A future final plan fingerprint should be derived from canonical, bounded caller requirements plus exact source/graph/policy/runtime/provenance basis with domain separation, but R2 **does not** invent an encoding, hashing domain or implicit persisted cache.

### R2.3 — Capability conflicts, cycles and partial-admission examples

These are hypothetical negative-fixture descriptions, not observed bugs or executable tests.

| Fixture | Expected conservative interpretation |
| --- | --- |
| A requires B; B provider unregistered | `MISSING_DEPENDENCY` even if M01 resolves A individually. |
| A requires B; B requires A, or A requires itself | `DEPENDENCY_CYCLE`, with a bounded canonical cycle reference, no recursive stack exhaustion. |
| Duplicate caller requirement X major 1 and X major 2 | `CONFLICTING_REQUIREMENTS`, no silent version substitution. |
| X's provider declares B but caller did not admit B | `DEPENDENCY_NOT_AUTHORIZED` unless separately proven pre-existing caller-owned binding. |
| X and Y individually resolve, but selected providers require incompatible source/realm/authority versions | `CROSS_CAPABILITY_CONFLICT`, do not treat both `resolve` successes as a group. |
| Provider alternatives could make a group feasible, but accepted M01 exposes only its chosen candidate | `ALTERNATIVE_SEARCH_NOT_ADMITTED`; no independent M06 algorithm, unproven plan remains blocked. |
| Snapshot says provider P, later M01 resolution/binding changes its fingerprint or activation generation | `PLAN_STALE`; reread and revalidate, never silently reuse old permission. |
| Owner-supplied HIVE provider announces features but peer proof is absent | `PROVENANCE_UNVERIFIED`; HIVE remains optional with truthful SOLO behavior. |
| `evidence_dependencies` contains opaque E with no separately verified source | `EVIDENCE_NOT_VERIFIED`; do not map E to a provider or claim artifact authenticity. |
| Provider P replaced while transitive dependents have active leases or safety-critical state | Use **M01 `substitution_impact_details`** and current change notifications; `REVALIDATION_REQUIRED`, no automatic live reroute. |
| Two individual binds succeed, third fails, or M01 generation changes mid-sequence | `GROUP_PARTIAL_OR_STALE`; do not label success, undo unknown external effects, infer rollback from lease release or write durable M04 history. |

Do not conflate M01 module startup graph validation with this prospective *capability* graph. Do not conflate M01's impact graph (what could break when an already bound provider changes) with selecting a new multi-provider dependency-complete plan.

### R2.4 — Consistency, safety-critical leases and the transaction boundary

The observed set has a time-of-check/time-of-use gap across `graph_snapshot`, individual M01 `resolve`, individual `bind_with_generation`, per-capability `acquire_lease_with_generation` and separate host-session validation. The accepted M01 `generation_coherence` compares one binding's runtime/activation basis; it is **not** a transaction that freezes the entire provider graph while multiple capabilities bind. M01 `substitution_impact_details` reports a transitive impact but does not atomically close external effects.

**Safe R2 conclusion:** M06 may draft a group-feasibility proposal and reject provable inconsistencies, but **cannot guarantee atomic multi-provider admission with existing APIs**. A future implementation must pick one of two separately reviewed designs:

- **Option A, reduced scope:** M06 remains a read-only preflight/diagnostic module. Any multi-provider side-effectful execution is denied until an external admitted orchestrator independently verifies the final per-capability M01 leases, policy/sandbox/peer proofs, generation and effect boundaries. Partial leases remain revocable, but their release does not undo remote side effects.
- **Option B, separately governed M01 extension:** after independent architecture/security review, M01 may expose a true snapshot-bound batch eligibility and atomic group binding/lease admission contract, with exact all-or-nothing semantics, fairness, shutdown/race and cross-platform tests. That is a **different M01 source/Work Order/lock admission**, not a hidden M06 wrapper. R2 authorizes no change to current M01.

These options are **alternatives for future decision**; neither is accepted by R2. No source-independent performance or low-latency claim follows from either.

### R2.5 — SOLO, HIVE, external transport and owner boundaries

SOLO may propose an admitted local CORE-owned capability graph without HIVE. A HIVE-owned intelligence requirement may use a minimal admitted CORE fallback **only** when its required version/features/quality/trust/assurance and explicit caller policy already permit it; absence of owner-local HIVE #4 cannot be used to advertise a verified HIVE binding. No HIVE RAG/cache/decision fabric is reproduced inside M06.

A future M05 R1–R4 host observation (source blob `222dace08091637e0192eb0259ba13109e418ba7`) is non-authoritative: its declared protocol features, process ID and MCP manifest do not establish process identity, permission, filesystem scope, sandbox or remote TLS/auth. M11/M22 must admit independently verified peer/provenance/security references; M10/M12 independently authorize actual effects. M07–M09 select specialists/models/effort, M18 handles policy-governed recovery and M19 budgets actual measured resources.

M04 remains the sole durable Run/Attempt/Step owner. The prior-V1 external-consumer inventory and replay BRC/idempotency amendment are still blocked [#111](https://github.com/KayzenRoot/core/issues/111); do not serialize draft PR #118 fields, infer a V2 migration or close M04's 23 pending EVs based on a provisional M06 proposal.

### R2.6 — Future negative tests, measured budgets and redacted evidence

All of the following **future R2 discovery IDs are PENDING** and are not an implementation AEG or proof from current M01 CI:

| ID | Future required negative/positive fixture or evidence |
| --- | --- |
| EV-M06-D15 | Existing M01 `ModuleRegistry.validate_graph` startup-cycle proof distinguished from provider dependency-closure preflight. |
| EV-M06-D16 | M01 `graph_snapshot` and `substitution_impact_details` reused with no competing registry, impact algorithm or selection reimplementation. |
| EV-M06-D17 | Selected candidate with missing, self, circular or unadmitted transitive `dependency_capabilities` blocked deterministically. |
| EV-M06-D18 | Duplicate/conflicting caller version, security and provider-origin constraints rejected without silent merges/downgrades. |
| EV-M06-D19 | Individually eligible providers with incompatible group policy/authority/realm or no admitted alternative enumeration remain BLOCKED. |
| EV-M06-D20 | Unverified or substituted `evidence_dependencies` and host-declared peer/feature/quality claims do not grant new capability trust. |
| EV-M06-D21 | Repeated/stale `graph_snapshot`, delayed resolver, changed M01 epoch/generation/provider fingerprint and late M05 session fail closed. |
| EV-M06-D22 | `substitution_impact_details` active lease, critical dependent, evidence/cache-affinity and revalidation cases preserve existing accepted semantics. |
| EV-M06-D23 | Partial group bind/lease outcomes do not claim atomic success or compensate unknown external effects with lease release. |
| EV-M06-D24 | SOLO/fallback and HIVE-owned optionality tested at exact required quality/trust/version floors, independent of owner-local #4. |
| EV-M06-D25 | Empty, oversized, deep/cyclic, duplicate, malicious/tainted provider/evidence graphs bounded, redacted and zero-LLM. |
| EV-M06-D26 | Calibrate real candidate provider/requirement/edge/evidence counts, graph depth and change-storm/latency/memory budgets against actual M01 snapshots on Linux/Windows; **no invented numeric cap or speedup**. |
| EV-M06-D27 | Pure preflight has no Git/HIVE/M04/host process/network/tool side effects; actual action execution remains independently gated. |
| EV-M06-D28 | After separately accepted final contracts, exact-head Linux/Windows, governance, supply-chain, bounded fuzz, source/lock and logical owner review NOT INDEPENDENT. |

A final frozen M06 acceptance graph must later assign each accepted requirement a concrete fixture, source owner, platform, expected result, invariant/quality floor, reproducible selected/rejected budget and review gate. R1 `EV-M06-D01..D14` and R2 `D15..D28` remain **28 discovery hypotheses, all pending**, distinct from M04's still-pending 23 global EVs and M05's 28 future candidate EVs.

### R2.7 — Round 3 entry, all 19 planning dimensions and STOP

R1 established mission/owner split, SOLO/HIVE boundary, provisional negotiation lifecycle, threat model and a 14-node discovery matrix. R2 provides source-backed differential analysis of M01's implemented graph/impact facilities, proposed group-feasibility laws, race/partial-admission alternatives, bounded negative fixtures and source-specific missing fields. **Still unfrozen:** final M06 public/internal DTOs and proof-port contracts, executable crate/file map, M05 actual admitted host observation API, M11/M22 independently verified identity/provenance schema, M10/M12 policy/action admission, exact dependency metadata semantics, group transaction decision, measured finite budgets and selected transport/SDK dependencies, migration compatibility, complete DoD and an audited executable Work Order/Context Lock. These are not silently declared solved.

**Round 3 may draft a candidate file/port map only after re-reading the actual promoted M01 and R2 sources**, and must either keep read-only M06 scope or explicitly flag a separately governed M01 batch-admission prerequisite. Final M06 Work Order freeze is later and also blocked from binding unadmitted M05/M04 contracts or claiming HIVE #4 proof.

**Round 2 STOP:** scope-limited append to this planning document plus new source-bound evidence; exact-head docs-only Governance and required successful status contexts with truthful Rust/fuzz no-op; scoped logical owner self-audit `NOT INDEPENDENT`, zero unresolved HIGH/CRITICAL/threads, protected squash merge and **new real FULL 11/11 main-push validation**. That proves only this non-authoritative planning increment was reviewed. No M06 API, M01 change, atomic multi-provider admission, remote trust, public implementation, new numeric floor or M04/HIVE external acceptance results from Round 2.


---

## Round 3 — minimal candidate crate map, proof ports and synthetic test harness

Status: R3_NON_AUTHORITATIVE_FILE_PORT_CANDIDATE  
Work Order: https://github.com/KayzenRoot/core/issues/160  
Exact initial promoted R1+R2 main: 2236eb1799f08974ce23ae79ee113e46d3fc441d  
Candidate mode for file-map exploration: **read-only M06 graph feasibility only**. The separately governed M01 batch-transaction alternative stays UNSELECTED, not quietly rejected or implemented.

This round specifies a *possible construction map* and executable-test requirements to reduce later rediscovery; it creates **no actual crate, Rust file, new dependency, public versioned DTO, Work Order execution admission, calibrated numeric budget or product-test evidence**. The M04 prior-V1 external compatibility issue #111 and real HIVE/Codex issue #4 are independently blocked. Prior R1 and R2 remain non-authoritative planning history.

### R3.1 — Existing Cargo/registry constraints and a one-crate candidate

The current protected root `Cargo.toml` (Git blob `3d9870aff7a8ee5f99f63aecfd295408f683d00e`) is workspace resolver 2, Rust edition 2021, rust-version 1.82, with **11 existing crates including** `core-contracts`, `core-registry`, `core-runtime`, `core-ipc`, `core-workspace` and `core-work-order`. It currently contains **no M06 crate**. Existing `crates/core-registry/Cargo.toml` (blob `13c3eaaf6f9f4185f0e4242bea96c7cc7800739a`) already depends on `core-contracts`, `core-identity`, `serde`, `thiserror` and `tokio`; `crates/core-contracts/Cargo.toml` (blob `0f5a6a8c5d81df2b864d09b39c7aa5635870ab76`) depends only on the declared serde/JSON/error workspace libraries. The source of actual registry methods remains accepted `core-registry/src/lib.rs` blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14`.

**Candidate for later review, not a real directory:** a single future `crates/core-capability-negotiation/` crate for pure read-only group preflight. It may consume existing `core-contracts` and, **if later dependency review admits it**, the real `core-registry` read-only snapshot/resolver/impact surface. A `core-registry` dependency has a transitively available Tokio dependency; describing M06's own core as deterministic does **not** pretend the complete dependency tree is Tokio-free. M01 never depends on M06. No speculative M05/M04/HIVE SDK dependency is allowed while those contracts are unadmitted. A strictly pure `core-contracts`-only M06 plus a separate caller-owned M01 adapter is an **alternate dependency option**, not something to add simultaneously or assume is already supported by the existing `CapabilityGraphSnapshot` type location.

Proposed dependency arrows if the first option is later accepted:

```text
future caller-owned M06 integration (not yet defined)
    -> future core-capability-negotiation
        -> existing core-registry (read-only, separately admitted use)
            -> existing core-contracts / core-identity / workspace libraries
        -> existing core-contracts
future caller-owned host/security proof adapters (UNFROZEN M05/M10/M11/M22 ports)
    -> bounded, verified references supplied TO M06
core-registry -X-> core-capability-negotiation
M04 durable journal -X-> M06
future core-capability-negotiation -X-> unadmitted M05/M04 SDK and HIVE memory DB
```

`-X->` denotes a forbidden dependency in this candidate, not an implemented compiler rule. Concrete crate features, direct dependencies, public exports and actual workspace membership require a later reviewed final planning/Work Order delta.

### R3.2 — Candidate file-to-authority and prohibited dependency map

These are **future file-map entries, not newly created files**. Each must be deleted, consolidated or revised at final source freeze if a specific capability is already adequately served by accepted M01; unused abstraction layers must not survive just to make the planned module appear larger.

| Future candidate path | Sole candidate responsibility | Mandatory owner/forbidden coupling |
| --- | --- | --- |
| `crates/core-capability-negotiation/src/lib.rs` | Thin, dependency-light surface and version/scope boundary only. | Exports nothing publicly until final source freeze; no re-export of an unauthorized host tool action. |
| `src/requirements.rs` | Pure finite caller-requirement canonicalization and conflict checks; never silently union incompatible policies. | Use existing `core-contracts::CapabilityRequirement` and `SemVer`; M03 exact admission/Context Lock supplied by caller, not fetched. |
| `src/observation.rs` | Immutable snapshot/generation input validation, claimed versus externally verified provenance-reference separation and change-guard basis. | Prefer actual M01 `graph_snapshot` types if admitted; a provider descriptor's self-authored fingerprint is not its peer proof. |
| `src/ports.rs` | **Read-only** M01 snapshot/resolve/substitution impact and independent owner-provided proof-read contracts, if necessary. | No new registry, lease writer, host process launcher, direct OS query or M11/M22 policy decision engine; don't invent fields from unadmitted M05 DTOs. |
| `src/graph.rs` | Bounded deterministic dependency closure on selected snapshot descriptors, duplicate/missing/self/cycle and unadmitted-edge refusal. | Reuse accepted M01 single-capability eligibility instead of re-ranking provider alternatives; capability edges not evidence authenticity. |
| `src/evidence.rs` | Require externally verified, exact-scope **opaque references** for non-capability `evidence_dependencies`, preserve UNKNOWN. | Actual cryptographic/trust proof remains M11/M22/M15 and authorized owner ports, never host-text string equality. |
| `src/feasibility.rs` | Pure composition of independently obtained M01 results, dependency/evidence proof status, source/runtimes and provisional blocked/refusal outcome. | Never calls M01 binding, lease mutation or M04 journal commit; never advertises atomic group permission. |
| `src/outcome.rs` | Finite typed refusal families, immutable basis/proof reference and redacted/capped deterministic counter summary. | No raw provider host text, HIVE snippets, credentials, filesystem paths, project UUIDs or new authorization tokens. |
| `tests/pure_group_graph.rs` | Positive dependency-complete synthetic cases and negative/tainted/conflicting/deep graphs. | Fake snapshot/provider descriptors only; such a test is **not** external peer or live M05 evidence. |
| `tests/races_and_integration.rs` | Future real M01 read-only graph/resolve/impact integration and scripted generation-change/partial-group race tests. | Exact-head Linux/Windows later; a fake transaction may prove refusal but cannot validate group atomicity. |
| `tests/solo_hive_boundaries.rs` | CORE-owned versus HIVE-owned preference/fallback under explicit, verified fixtures. | No actual owner-local HIVE #4 assertion or cloned HIVE retrieval logic. |
| `tests/hostile_provenance.rs` | Forged manifest, mismatched feature/major, evidence substitution, tainted text, unverified host negative fixtures. | No synthetic self-attestation accepted as genuine peer proof. |

**Consolidation check:** A separate `src/graph.rs` must not copy `core-registry::substitution_impact_details`; it may inspect candidate **new** dependency closure only. `ports.rs` cannot wrap `resolve_from_state` with subtly different filtering. A `src/feasibility.rs` cannot introduce a second binding cache/source of truth. If the admitted M01 interfaces cannot supply a consistent candidate basis, emit a bounded **NOT_ADMITTED / STALE / ALTERNATIVE_SEARCH_UNAVAILABLE** response and defer the M01 API change to a separately governed Work Order.

### R3.3 — One immutable candidate input/output boundary, no public schema freeze

The possible *pure* input groups are: (a) explicit caller-owned canonical requirement set and admitted scope/reference; (b) exact M01 runtime+policy+graph/binding generation and a **single** observed graph snapshot; (c) independently resolved M01 eligible result per canonical requirement with an exact matching descriptor/provenance basis; (d) independently verified peer/evidence references supplied by later admitted owner ports; (e) finite caller-provided validation/calibration limits. If any mandatory field is missing, foreign, stale or invented by an untrusted provider, no provisional group result is eligible.

The possible *pure* output distinguishes `BLOCKED`, `UNVERIFIED`, `STALE`, `NOT_ATOMICALLY_ADMITTED` and `PROVISIONAL_FEASIBILITY`; every branch includes only typed bounded reasons and immutable **non-authoritative** observation/source basis. `PROVISIONAL_FEASIBILITY` does not itself carry a lease, action admission, group transaction, session trust elevation, replay receipt or actual effect status. External authorization is checked again by actual owner ports at effect time, even if a cached prior feasibility proposal still has matching labels. All these are **candidate conceptual labels**, not accepted serialized enum variants.

An M05 host observation can be connected to this shape **only later**, behind a separately accepted versioned port for the specific transport. An M04 Run/Attempt/Step reference must remain opaque until #111's actual compatibility disposition and a fresh admitted source/lock; no draft #118 fields can be treated as frozen. A real HIVE context provider may be represented only by admitted M01 provenance+separate actual running proof, not a GitHub tag or `mcp-core-surface-v1` protocol string.

### R3.4 — Synthetic harness ownership and exact negative fixtures

Build the **future** pure harness around deterministic immutable fake M01 snapshot+resolve responses and separately controlled peer/evidence owner proof results. Use an injected monotonically increasing test epoch, explicit change-event sequence and operation/provenance references. The harness must never request a real provider API key, spawn an arbitrary program, make a network request or alter a repository. It should report fixture identities and bounded generic reasons without recording canary host text.

| Future harness group | Test obligations already tracked in R1/R2 discovery |
| --- | --- |
| Accepted M01 interop and no second selector | D01, D05, D15, D16: replay source-grounded single-resolve ordering, startup dependency versus capability dependency distinction, real impact/graph reuse. |
| SemVer/features and duplicate canonicalization | D02, D17, D18, D19: major/minor/patch floors, self/foreign/circular graph, conflicting duplicate requirement and group alternatives not enumerated by M01. |
| Independently verified identity/evidence | D03, D04, D11, D20: fake M05/MCP/provider manifest and poisoned evidence dependency cannot mint trust or scope. |
| Generation/change and group races | D08, D09, D21, D22, D23: epoch, provider binding/activation, lease revoke, dependent impact and partial group attempts fail closed. |
| SOLO/HIVE bounded fallback | D06, D07, D24: only exact caller-permitted capability floors, no HIVE cache/memory implementation or false owner-local runtime assumption. |
| No hidden effects, zero-LLM, bounded hostile data | D10, D12, D25, D27: all fixture ports deny network/process/Git/M04 mutations and prevent raw prompt/path/credential logs. |
| Calibration and independent assurance | D13, D14, D26, D28: proposed reproducible resource/cross-platform methodology, eventual exact-head CI/evidence and NOT INDEPENDENT audit **only after implementation exists**. |

**Test-vs-trust honesty:** a fake valid M05 observation, fake HIVE peer receipt or synthetic M11 security reference proves only the pure M06 refusal/branch logic for *that supplied input*. It cannot establish that an actual host identity, tool authority or HIVE v1.0.0 deployment is verified. No test on an uncreated crate is marked PASS in R3.

### R3.5 — Resource measurement, token-efficient fault isolation and evidence protocol

Candidate calibration dimensions: number of canonical requirements, M01 registered providers, graph nodes/edges, maximum transitive depth, independent evidence refs, duplicate count, dirty/late generations and change-notification bursts; for each evaluate valid, cap-1/cap/cap+1, empty, malformed and cyclic hostile fixtures with exact reproducible seed and hash. **No numeric cap is admitted by this round.** The later executor must measure same-platform baseline `core-registry` snapshot+single resolver and M06 pure wrapper **separately**, and report the extra latency/CPU/RSS/allocations attributable to M06, cold/warm, pathological bounded graphs and Ubuntu-versus-Windows results.

Future stable harness boundaries should isolate tests/failures by `requirements`, `observation/ports`, `graph`, `evidence`, `feasibility`, `outcome` and host-provider fake fixture. On a failure, run only the affected harness first and its downstream integration/negative cases after the correction; retain an exact source SHA and deterministic fixture seed so a later executor need not re-read every module or re-run unrelated full tests for each local iteration. **Protected promotion still requires the complete, non-skippable exact-head CI/acceptance matrix.** This local optimization is not permission to weaken final quality checks.

Recorded test evidence should include source/Work Order/Context Lock bindings where applicable, fixture ID and seed, platform/compiler/toolchain, accepted vs rejected limits, actual latency/memory, typed outcomes, negative denial coverage, zero secret-bearing raw logs and source-specific failure owner. All 28 D01..D28 are **planning discovery** only; a final AEG/Definition of Done must map accepted requirements to actual executable fixtures after the future owner-governed freeze.

### R3.6 — Nineteen planning dimensions and bounded STOP

R1 documented mission, HIVE/CORE ownership, SOLO/HIVE behavior, preliminary states/failure/security/telemetry and evidence. R2 established the genuinely missing conservative graph preflight and missing atomic group transaction against accepted M01. R3 adds a tentative *single-crate* file/port map, fake harness responsibilities, acyclic dependency possibilities, provenance/identity trust separation, calibration protocol and checkpointable fault isolation.

**Still blocked/unfrozen for a later Round 4 and final execution packet:** choose pure-only dependency/port option versus separately governed M01 batch change; actually admit M05 versioned host DTO after its own final freeze; independently admit M10/M11/M12/M22 action/peer-proof contracts; govern external M04 previous-V1/V2 compatibility #111; prove local HIVE #4 only if deployment is claimed; settle public versioned M06 API, exact Cargo direct/transitive dependencies and real crate paths, truthful HIVE/host migration and compatibility, measured positive finite numeric budgets, final traceable AEG/DoD, separately reviewed executable Work Order and ACTIVE Context Lock. No status shown here indicates those gates passed, and no extra discovery IDs are invented merely because the file map got longer.

**Round 3 STOP:** accept only this R3 appendix plus a new bounded source-evidence Markdown file following exact-head docs-only Governance and mandatory successful status contexts, a logical owner self-audit `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT` with zero unresolved HIGH/CRITICAL/threads, protected non-force squash merge and independently verified real **FULL 11/11 protected-main push**. Closure remains strictly R3 discovery and does not authorize code, new public schema, provider authority, a multi-provider atomic transaction or premature M05/M04/HIVE promotion.


---

## Round 4 — limited read-only v0 direction and remaining execution-freeze gates

Status: R4_NON_AUTHORITATIVE_LIMITED_V0_DIRECTION_CANDIDATE  
Work Order: https://github.com/KayzenRoot/core/issues/169  
Exact initial reviewed main: bac70cbb2f083322088e92694b5218c785bc68cf  
**This is a proposed limited planning choice, not a frozen public API, code permission, production readiness or a completed AEG.**

### R4.1 — Scope choice for v0: independent pure diagnostic preflight

For the first **candidate** M06 increment, adopt R2's **Option A at the planning level**: a pure, bounded, caller-supplied, **read-only group-feasibility preflight**. It can prove that some selected provider configurations are *not* suitable and can provide provisional dependency observations for a trusted caller, but **never** calls `bind`, mints a lease, mutates `core-registry`, authorizes a tool/model/process or claims an all-or-nothing group transaction.

The optional R2 **Option B**, a true snapshot-bound M01 atomic batch eligible-set/bind/lease admission API, is *not included in this v0 candidate*. It requires its **own** M01 Work Order, contract/source/security review and exact-head Rust/Windows/Linux race/property tests if a future actual execution requirement justifies it. Current accepted M01 supports `graph_snapshot`, per-capability `resolve`, `substitution_impact_details`, `bind_with_generation` and individual generation-aware leases; **there is no accepted multi-provider atomic group admission API**. R4 therefore does not pretend that orchestrating several single-capability calls creates atomicity.

This limited scope leaves the provisional M06 interface independent of unfrozen M04 replay/BRC/idempotency, M05 host-observation DTOs and local HIVE configuration. Even pure v0 **does not become executable without a separate final freeze, accepted proof boundaries, calibrated finite limits and reviewed Work Order**. R1–R3 architecture and 28 future discovery test obligations remain source history, not completed product features.

### R4.2 — Future pure function contracts: candidate groups, no serialized schema

The conceptual **input** is an immutable bounded group of existing `core_contracts::CapabilityRequirement` values from an independently admitted caller plus the caller's exact immutable scope/Work Order source reference, selected **accepted** M01 `RuntimeGeneration`, a separately obtained read-only M01 `CapabilityGraphSnapshot`, exact M01 `resolve` results (or typed rejection) and a bounded set of *independently verified* evidence/peer-proof **references** whose verification semantics are owned by later admitted policy/security ports. These field groups are not accepted Rust public DTO types, a published hash format or a guarantee that separate M01 API reads were atomic.

The pure **output** is one bounded typed decision family and only relevant immutable scope/generation/provenance fingerprints and redacted failure reasons: `BLOCKED` for deterministically disproven compatibility, `UNVERIFIED` for missing independently confirmed external/evidence requirements, `STALE_OR_INCONSISTENT_OBSERVATION` when snapshot and separate `resolve`/generation bases cannot be reconciled, or at most `PROVISIONAL_FEASIBILITY` when all *observed* requirements and graph edges are consistent. Every non-BLOCKED result must still declare `GROUP_NOT_ATOMICALLY_ADMITTED` and **no permission to invoke**. An unsupported unknown, absence of independent proof or cardinality breach is a denial rather than an optimistic fallback.

No one may treat `CapabilityGraphSnapshot` as an atomic snapshot of a later independent `resolve`; its accepted implementation captures descriptors/bindings at one registry read-lock instant only. The snapshot type currently does **not** contain a globally locked runtime policy/graph epoch covering later M01 resolver calls. If fresh verification of that gap requires a new M01 batch-read API, that would be a separate governance request; v0 must remain conservative and refuse to upgrade a provisional observation into admission.

### R4.3 — Exact source boundary and no duplicate algorithms

Accepted M01 `core-registry/src/lib.rs` blob `dfe326c90ef4051813f0b2da26fdcad5522fd6c8` now includes both distinct reviewed safeguards: [PR #164](https://github.com/KayzenRoot/core/pull/164) checks the **current** provider ID/fingerprint/activation/health/readiness/quarantine before a new lease; [PR #167](https://github.com/KayzenRoot/core/pull/167) propagates health/quarantine for one logical provider ID across **all** registered capability descriptors. They were protected promoted and the latter independently FULL validated by [CI #36349930954](https://github.com/KayzenRoot/core/actions/runs/36349930954), **11/11 SUCCESS**. This is accepted M01 behavior; M06 v0 does not reimplement live admission, duplicate the registry or treat an old snapshot as a current lease.

The accepted M01 `resolve_from_state` already enforces mandatory per-capability SemVer, features, authorities, assurance, trust, origin/class, policy, quality, health/readiness/quarantine, latency/cost. M06 must delegate rather than re-rank these fields, invent a second fallback route or separately honor self-attested provider identity. The *only new pure v0 analysis* is conservative caller-authorized **cross-capability** dependency closure and contradiction reporting: current M01 individual resolution does not prove complete `dependency_capabilities` closure, opaque `evidence_dependencies` authenticity or atomic group admission.

`ModuleRegistry.validate_graph` checks module **startup** graph cycles; `substitution_impact_details` already computes **active binding** dependent/cache/evidence/critical impacts. M06's tentative `graph.rs` must not clone either accepted algorithm. Actual provider descriptor `origin`, `fingerprint`, `performance_score` or an MCP tool name are *claims*, not independent peer proof. No HIVE user-memory/RAG logic, host process launcher, network listener, M04 journal writer or second authorization layer belongs in the pure crate.

### R4.4 — Limited dependency/port layout and STOP-first execution boundary

R3's candidate `crates/core-capability-negotiation` single-crate file map remains *proposed*. The limited v0 could expose only pure `requirements`, `observation`, `graph`, `evidence`, `feasibility` and `outcome`, plus a bounded **read-only** M01 snapshot/resolve/impact adapter owned by the trusted caller. A direct `core-registry` dependency versus a `core-contracts`-only pure crate with a caller-supplied adapter is **still a final-freeze choice**: current `Cargo.toml` blob `3d9870aff7a8ee5f99f63aecfd295408f683d00e` has no M06 member and existing M01 registry depends transitively on Tokio. R4 creates neither option.

The permitted conceptual data flow is:

```text
trusted caller with explicit existing scope and M01 read-only access
  -> independently admitted snapshot/resolver/proof ports (NO M06 writes)
  -> pure bounded v0 conflict + provider dependency-closure preflight
  -> typed BLOCKED | UNVERIFIED | STALE | PROVISIONAL_ONLY
  -> caller must separately revalidate M01 + policy/host/actor/effect gates
NO: M06 -> binding/lease writes, M04 journal, direct M05 process,
    arbitrary HIVE/RAG/LLM, M01 reverse dependency or tool execution
```

No ABI, public DTO name, serde schema/version, hash domain, host-proof receipt format, max graph depth, performance baseline or crate file actually exists. A future executor must receive these precise choices after a new source-bound freeze; an unapproved placeholder must never be confused with production code.

### R4.5 — Exact fail-closed invariants and regression ownership

A later implementation must prove the following with deterministic fake and real-M01 integration evidence, **not** with diagrams alone:

- **No implicit capability acquisition:** every dependency is already in the caller's authorized requirement set or is a separately *actually validated* prior binding within current scope; graph traversal cannot add a capability, trust floor, action scope or evidence owner.
- **No optimistic duplicate merge:** repeated capability requirements with contradictory major/minor/patch, source/origin, trust, assurance, quality, mandatory features or policy must block rather than intersect/union silently.
- **Provider dependency distinct from evidence:** `dependency_capabilities` are capability edges, `evidence_dependencies` are opaque references to separately verified evidence owners, and module startup dependencies are a third independent graph. Same-named strings are not sufficient proof.
- **No hidden alternative enumeration:** when accepted M01's chosen providers yield an incompatible group, a different combination must **not** be searched with copied M01 ranking logic or marked feasible without a separately governed read-only eligible-set API.
- **No stable-snapshot fiction:** a different descriptor fingerprint, activation generation, runtime policy/capability graph epoch, quarantine/health or prior lease between observations and actual effect means STALE/revalidate. Single-procedure testing across a cloned registry is not proof of cross-process or global atomicity.
- **No fake group transaction:** multiple valid M01 `resolve` outputs or individually acquired leases yield, at most, provisional diagnostics. Failures after earlier binds require actual caller/policy/M18 decisions. Dropping a lease does not undo a possible external side effect.
- **No unverified external trust:** unadmitted M05 host session, remote MCP tool metadata or a claimed HIVE release/protocol identifier cannot mint independently authenticated M11/M22 peer/security or M10/M12 effect authority.
- **SOLO first:** absent local HIVE #4 evidence, native CORE-only logic still reports deterministic unverified HIVE availability and only uses a CORE fallback if accepted M01 requirements explicitly permit exact compatible floors.
- **Finite, deterministic, redacted:** empty/duplicate/deep/cyclic/malformed/oversize/tainted graph fixture handling, bounded memory/CPU/resource/cancellation checks and no live network, external subprocess, Git mutation, HIVE persistence, M04 journal or raw secret/path/prompt output.

No numeric performance advantage or exact production limit is asserted by this round.

### R4.6 — Mapping the existing 28 future discoveries to v0 evidence owners

All `EV-M06-D01..D28` remain **PENDING discovery hypotheses**; this classification does not create a final AEG, execute tests or waive blocked external evidence.

| Future fixture owner | Existing discovery hypotheses | Evidence required before executable v0 acceptance |
| --- | --- | --- |
| Accepted M01 interop/eligibility | D01, D02, D05, D08, D15, D16, D22 | Assert actual single-capability resolver semantics, runtime coherence and unchanged M01 health/quarantine/transitive substitution behavior through real read-only integration. |
| Pure requirement canonicalization | D04, D17, D18 | Strict finite caller authorization basis; duplicate/version/floor/ref conflict and empty-scope negative cases. |
| Pure dependency graph | D09, D17, D19, D23 | Missing/self/cyclic/mutually incompatible graphs, alternative enumeration refusal and no false atomic group success. |
| Separate peer/evidence verification | D03, D07, D11, D20, D21 | Fake forged manifests, unverified evidence/provenance and stale generation observations must remain blocked; **live external security proof not available** through fake ports. |
| SOLO/HIVE and side effects | D06, D10, D24, D27 | Deterministic no-HIVE operation, truthful fallback floors, no process/network/Git/tool/M04 effects in pure code. |
| Platform/hostile/property resources | D12, D13, D14, D25, D26, D28 | Reproducible synthetic graph seeds, cap-1/cap/cap+1 measurements and full source-bound Windows/Ubuntu/supply-chain/fuzz and reviewer evidence when code actually exists. |

Any unlisted possible future claim must be linked to an admitted requirement before a final implementation AEG is frozen; this table only **groups** previously defined hypotheses, it does not mark their realization.

### R4.7 — Nineteen-dimension readiness and next separate freeze

R1–R4 now document a limited mission, CORE/HIVE overlap boundaries, SOLO/connected behavior, candidate read-only role, M01 reuse, candidate file/port map, ephemeral observation states, fail-closed reason families, side-effect/owner prohibitions, future telemetry and reproducible test categories. **Still NOT frozen:** exact public/internal versioned contracts; direct Cargo/trait dependency decision; what constitutes independently admitted M11/M22 evidence-port verification; exact bounded scope/provenance receipt from M03 and M10; handling a genuinely admitted M05 host observation after its separate freeze; real M04 V1/V2 disposition before *any* durable invocation mapping; measured and reviewed finite numeric resource budgets and regression floors; compatibility/migration proof; finalized AEG with fixture owners and actual test results; complete acceptance/DoD; separate GEF reviewer-admitted executable Work Order and ACTIVE module Context Lock.

A later planning/freeze round must resolve *all applicable* 19 canonical dimensions with exact accepted source bindings and reproducible measured baseline. HIVE Docker and separate Codex client [#4](https://github.com/KayzenRoot/core/issues/4) and M04 prior-V1 external consumer [#111](https://github.com/KayzenRoot/core/issues/111) are **not** automatically settled by limiting v0's pure scope. No new M04 V1 schema may be inferred from draft PR #118, and no product PR #106 or M05 R5 freeze is admitted.

**Round 4 STOP:** only this non-authoritative R4 appendix plus new source-bound evidence, fresh exact-head docs-only Governance/required successful status contexts with intentional Rust/fuzz no-op identified, a scoped logical owner review `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT` and zero unresolved HIGH/CRITICAL/threads, protected squash merge and independent **real FULL 11/11 post-merge main-push**. Closure proves a bounded v0 planning direction, not any product M06 readiness, group transaction, host trust, local HIVE or external M04 compatibility.
