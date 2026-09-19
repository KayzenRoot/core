# CORE Requirements

Status: `BOOTSTRAP_BASELINE`

These are foundation requirements only. Product-functional requirements will be discovered and frozen during the planning phase.

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
