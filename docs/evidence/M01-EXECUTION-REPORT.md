# CORE-WO-M01-001 execution report — Prompt 007

Status: `READY_FOR_REVIEW` — criteria 1–20 pass; criterion 21 governed review pending
Date: 2026-09-19
Correction source: `CORE-M01-CODEX-CORRECTION-PROMPT-007.pdf`
Implementation HEAD: `ad4ea2658a828b4fe3b40619e57047f3d15522fe`
Target branch: `feat/m01-core-runtime`
Review issue: `#13 M01-REVIEW-007`
Pull request: [#8](https://github.com/KayzenRoot/core/pull/8)

## Authority and scope

Prompt 007 was executed in document order within the frozen M01 Work Order, the CORE Executor Contract and repository rules. The correction was limited to the bounded event-driven CAL drain residuals:

1. `CapabilityRegistry` now owns a clonable Tokio watch-backed `LeaseChangeSubscription` with serialized monotonic revisions.
2. Acquire, release, revoke and expiry cleanup publish lease revisions; QDS also waits for the next monotonic lease expiry without polling.
3. `Supervisor::shutdown` selects lease revision, expiry timer or the existing Tokio shutdown deadline, with no timed 1 ms CAL drain loop.
4. Mandatory separate-task, timeout, multi-lease ordering and subscription-boundary tests preserve generation coherence, independent release and QVM residual semantics.
5. Criterion 17 now records the current validated PRB/WNF value rather than the stale prior value.
6. This report maps the exact canonical criteria 1–21 from `docs/work-orders/CORE-WO-M01-001.md`.

No merge, promotion, release or independent approval is claimed.

## Exact-head local evidence

- Toolchain: stable Rust `cargo/rustc 1.98.1`; system Python 3.12.10.
- `cargo fmt --all -- --check`: PASS.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --all-targets --locked`: PASS; 52 tests, 0 failed; the core-registry suite passed 13/13 and core-runtime 18/18.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo check --manifest-path fuzz/Cargo.toml --locked`: PASS.
- `python scripts/validate_governance.py`: PASS; GEF `v1.0.0` and HIVE compatibility `v1.0.0` bridges consistent; 27 governed artifacts.
- Python compilation for governance, HIVE, PRB, soak and SBOM scripts: PASS.
- M01 soak, 16 iterations: PASS; every cycle reached `ReadyEligible`, preserved fallback, generation coherence, probe coalescing and quarantine behavior; process handles `150 -> 153` (`+3`, policy `<=4`).
- PRB/WNF: PASS; baseline median p50 `110.8193 ms`, current median p50 `126.7303 ms`, regression `14.357607384273305%`, allowed `20%`.
- Local SBOM SHA-256: `1c32c13097b7889c606f5af6d62fe20b3d6fa437ac055d973ca526cece4d61c7`.
- Focused production-path validation: separate-task lease wake, unreleased-lease deadline, multi-lease release ordering, subscription-boundary notifications and existing worker-completion tests all passed; lease-focused stress passed 5/5.

## HIVE and Git basis

Git is canonical for this correction. The target checkout is `D:\Projeto Codexx\core-prompt004`, branch `feat/m01-core-runtime`, at the implementation HEAD above. Read-only HIVE preflight found the separate registered `core` project on `main` (`c65b7abc-533a-411a-bbbb-2b72b976d921`) at `fdb4dbe165e74b009c43df3874b6043c9b94710b`; the target branch is not registered there and its checkpoint read returned `source_not_current`. No target-branch HIVE evidence is claimed.

## Hosted exact-head evidence

Run [35481048830](https://github.com/KayzenRoot/core/actions/runs/35481048830) completed `success` at implementation HEAD `ad4ea2658a828b4fe3b40619e57047f3d15522fe`:

- [Governance job 105998800559](https://github.com/KayzenRoot/core/actions/runs/35481048830/job/105998800559): SUCCESS.
- [M01 Ubuntu job 105998800666](https://github.com/KayzenRoot/core/actions/runs/35481048830/job/105998800666): SUCCESS, including format, clippy, tests, supply-chain, SBOM, soak and PRB.
- [M01 Windows job 105998800447](https://github.com/KayzenRoot/core/actions/runs/35481048830/job/105998800447): SUCCESS, including native IPC, supply-chain/advisory, SBOM, soak and PRB.
- [M01 fuzz campaign 105998800581](https://github.com/KayzenRoot/core/actions/runs/35481048830/job/105998800581): SUCCESS.

The implementation evidence is bound to `ad4ea26`; this documentation-only update is published separately and receives its own exact-head workflow validation. No self-referential evidence loop is claimed.

## Canonical acceptance criteria

1. **All M01-required crates/contracts/mechanisms exist at governed boundaries — PASS.** Evidence: `crates/core-contracts`, `core-config`, `core-registry`, `core-runtime`, `core-journal`, `core-ipc`, `core-health`, `core-identity` and CLI sources; workspace check/tests and Governance job passed.
2. **Workspace dependency graph is acyclic and forbidden dependencies are absent — PASS.** Evidence: locked workspace checks, `Cargo.lock`, Governance dependency checks and hosted supply-chain/advisory gates.
3. **RLC state transitions are machine-enforced with receipts — PASS.** Evidence: typed lifecycle decisions/context in `crates/core-runtime/src/lib.rs`, journal-before-mutate paths, structured `TransitionDenied`, and runtime tests.
4. **BSR is required before READY/DEGRADED eligibility — PASS.** Evidence: Supervisor bootstrap/eligibility paths and runtime bootstrap tests; hosted platform jobs passed.
5. **Module/capability registries are deterministic and support CAL/SIR/CBR/FCH — PASS.** Evidence: registry mechanisms/tests for deterministic admission, dependency traversal, capability binding and fallback.
6. **Capability substitution is atomic and generation/epoch coherent — PASS.** Evidence: registry substitution and generation/lease tests; exact-head workspace tests passed.
7. **DCS golden vectors prove stable canonical identity — PASS.** Evidence: `core-identity` golden-vector tests and hosted test matrix.
8. **Runtime Journal integrity/recovery/corruption handling passes — PASS.** Evidence: journal integrity, recovery, truncation and corruption tests; exact-head workspace tests passed.
9. **SBR/EEB reject stale epoch mutation and blind replay — PASS.** Evidence: stale epoch lease/IPC tests and replay rejection paths in registry/runtime/IPC.
10. **QDS/QVM distinguish clean shutdown from residual forced termination — PASS.** Evidence: async Tokio-monotonic QDS now selects clonable watch-backed CAL revisions, next lease expiry and the shutdown deadline; independent lease release, generation coherence, worker completion, timeout and forced-residual tests all pass.
11. **DCM/QFC enforce safe degradation and quality-floor continuity — PASS.** Evidence: degradation matrix, capability pressure and quality-floor tests; Supervisor context carries live degradation state.
12. **PHC/HCC/ODF health behavior passes concurrency/delta tests — PASS.** Evidence: health concurrency, probe coalescing, freshness/delta and stale authorization tests; soak passed.
13. **Local IPC is bounded, versioned, fuzzed and platform-adapted — PASS.** Evidence: `core-ipc` tests, platform IPC job and hosted fuzz campaign.
14. **Security/supply-chain gates and unsafe inventory pass — PASS.** Evidence: Governance, hosted cargo-deny/audit/advisory/SBOM and forbidden-source checks; no HIGH/CRITICAL gate finding.
15. **Unit/property/integration/fuzz/failure-injection suites pass — PASS.** Evidence: 52 local workspace tests including `lease_released_from_separate_task_wakes_qds_before_deadline`, `unreleased_lease_times_out_only_after_shutdown_deadline`, `multiple_leases_release_in_different_order_without_lost_notifications`, `lease_change_notifications_cover_lifecycle_and_boundaries` and the existing worker-completion tests; clippy, fuzz manifest check, failure/deadline tests and all hosted jobs passed.
16. **Soak tests show no unbounded growth — PASS.** Evidence: 16-cycle soak, `+3` process handles within policy `<=4`, clean shutdown on every cycle.
17. **PRB/WNF benchmark evidence is reproducible and no unapproved material regression exists — PASS.** Evidence: compatible baseline/current WNF and hardware/toolchain; current p50 `126.7303 ms` versus baseline `110.8193 ms`, regression `14.357607384273305%` within `20%`; source artifact is `artifacts/m01-prb-prompt007.json`.
18. **CLI machine outputs are versioned/stable — PASS.** Evidence: CLI schema/benchmark output tests and hosted platform jobs.
19. **Zero-LLM M01 proof passes — PASS.** Evidence: runtime bootstrap/lifecycle/shutdown uses no LLM/provider SDK path; Governance and workspace tests passed.
20. **Exact-head evidence bundle identifies commit/toolchain/platform/baselines — PASS.** Evidence: this report identifies implementation SHA, toolchain, local baselines, SBOM hash, HIVE boundary and hosted run/job IDs at that exact pushed SHA.
21. **Independent governed review verdict is APPROVED — PENDING.** Review 008 is required; no independent approval is claimed and PR #8 remains open.

## Final verdict

`READY_FOR_REVIEW` is justified for implementation HEAD `ad4ea2658a828b4fe3b40619e57047f3d15522fe`: criteria 1–20 have exact-head local/hosted evidence, the CAL drain is event-driven without the 1 ms polling loop, the corrected PRB evidence is internally consistent, and no HIGH/CRITICAL gate finding remains. Criterion 21 is the STOP boundary for independent governed Review 008. No merge, promotion or approval is claimed.
