# M06 Capability Negotiation — Round 1 discovery candidate

Status: R1_DOCUMENTED / R2_NON_AUTHORITATIVE_DISCOVERY_CANDIDATE  
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
