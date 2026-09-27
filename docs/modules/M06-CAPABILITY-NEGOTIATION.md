# M06 Capability Negotiation — Round 1 discovery candidate

Status: NON_AUTHORITATIVE_DISCOVERY_R1_CANDIDATE  
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
