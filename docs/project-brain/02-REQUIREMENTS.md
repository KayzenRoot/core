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
