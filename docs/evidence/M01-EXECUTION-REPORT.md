# CORE-WO-M01-001 execution report — Prompt 009

Status: `READY_FOR_REVIEW` — criteria 1–20 pass; criterion 21 governed review pending
Date: 2026-09-20
Correction source: `CORE-M01-CODEX-CORRECTION-PROMPT-009.pdf`
Repository: `KayzenRoot/core`
Target branch: `fix/m01-cal-admission-close`
Exact PDF base: `8f0820188130613caa6890c82d43225d9a317947`
Audited basis: `fa4e0687a27c79c3564b56ebee02a56f42e8aacf`
Implementation HEAD: `31b08c87c350c745762a1ac06ca28f43f7a9514a`
Review source: Issue [#18](https://github.com/KayzenRoot/core/issues/18)
Pull request: [#17](https://github.com/KayzenRoot/core/pull/17)

## Authority and bounded scope

Prompt 009 was executed in document order under the CORE Executor Contract, the frozen M01 Work Order and repository rules. The correction is limited to the missing in-drain CAL admission proof:

1. Capability lease admission is stored in the existing synchronized `RegistryState` and is closed by `close_lease_admission()` without a reopen path for the runtime epoch.
2. `RegistryError::AdmissionClosed` is returned by `acquire_lease_with_generation()` while holding the same write lock used for lease insertion.
3. `Supervisor::shutdown()` closes capability admission before LeaseDrain; runtime status exposes both admission gates.
4. QVM proves both gates, re-reads active leases immediately before the quiescence matrix, and cannot produce clean STOP with a residual lease.
5. The new `in_drain_clone_acquire_is_rejected_during_repeated_shutdowns` integration test repeats the full spawned-shutdown scenario 16 times and proves rejection while the pre-close lease remains active.

The existing production implementation is unchanged by Prompt 009. No merge, promotion, release or M01 completion is claimed.

## Changed files

- `crates/core-registry/src/lib.rs`
- `crates/core-runtime/src/lib.rs`
- `docs/evidence/M01-EXECUTION-REPORT.md`

## Exact-head local evidence

- Toolchain: stable Rust `cargo/rustc 1.98.1`; system Python `3.12.10`.
- `cargo fmt --all -- --check`: PASS.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --all-targets --locked`: PASS; 56 tests, 0 failed; `core-registry` 15/15 and `core-runtime` 20/20, including the new 16-cycle in-drain integration proof.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo check --manifest-path fuzz/Cargo.toml --locked`: PASS.
- `python scripts/validate_governance.py`: PASS; GEF `v1.0.0`, HIVE compatibility `v1.0.0`, bridges consistent and 27 governed artifacts.
- Python compilation for all six scripts: PASS.
- HIVE bootstrap/MCP unit tests: PASS; 6/6.
- New in-drain integration proof: PASS; `in_drain_clone_acquire_is_rejected_during_repeated_shutdowns`, 16 repetitions, bounded gate observation, typed `AdmissionClosed` rejection, unchanged active-lease count, clean `StopCommit` and all QVM items satisfied.
- M01 soak, 16 iterations: PASS; every cycle reached `ReadyEligible`, process handles `150 -> 153` (`+3`, policy `<=4`), record fingerprint `615d338c2f4d28a093a0c0c77400b23801c3ae36cf88063130eeb1ef94e1db09`.
- PRB/WNF: PASS; compatible baseline p50 `110.8193 ms`, current median p50 `127.0076 ms`, regression `14.607834555894145%`, allowed `20%`.
- Local SBOM SHA-256: `1c32c13097b7889c606f5af6d62fe20b3d6fa437ac055d973ca526cece4d61c7`.
- Local bounded fuzz campaign was unavailable because nightly Rust and `cargo-fuzz` are not installed; hosted fuzz job is required evidence.
- Local `cargo-deny` and `cargo-audit` executables were unavailable; hosted Ubuntu/Windows supply-chain and advisory steps passed.
- Local artifacts: `artifacts/m01-soak-prompt009.json`, `artifacts/m01-prb-prompt009.json`, `artifacts/m01-sbom-prompt009.cdx.json` and the prior prompt artifacts.

## HIVE and Git basis

Read-only HIVE preflight resolved the registered CORE project as `c65b7abc-533a-411a-bbbb-2b72b976d921` on `main` at `fdb4dbe165e74b009c43df3874b6043c9b94710b`, READY and clean in HIVE. The correction checkout is the exact PDF branch and is not HIVE-registered; checkpoint read returned `source_not_current`. Git is therefore canonical for this target branch and no target-branch HIVE evidence is claimed.

The branch was created from the remote PDF base `8f0820188130613caa6890c82d43225d9a317947`. The implementation commits are:

- `e471eb495b9561fbbe0ccfed6140d8bd6af1e99c` — linearize capability lease admission close.
- `895f68d410f964063e95f062b32ff2755461bb8d` — stabilize lifecycle timing assertions for the hosted Windows gate.
- `31b08c87c350c745762a1ac06ca28f43f7a9514a` — add the repeated in-drain clone admission integration proof.

## Hosted exact-head evidence

Workflow [35486894184](https://github.com/KayzenRoot/core/actions/runs/35486894184) completed `success` as the exact-head matrix for implementation HEAD `31b08c87c350c745762a1ac06ca28f43f7a9514a`:

- [Governance job 106014811918](https://github.com/KayzenRoot/core/actions/runs/35486894184/job/106014811918): SUCCESS.
- [M01 Ubuntu job 106014812038](https://github.com/KayzenRoot/core/actions/runs/35486894184/job/106014812038): SUCCESS, including format, clippy, tests, supply-chain, advisory, SBOM, soak and PRB/WNF.
- [M01 Windows job 106014812031](https://github.com/KayzenRoot/core/actions/runs/35486894184/job/106014812031): SUCCESS, including native IPC, supply-chain, advisory, SBOM, soak and PRB/WNF.
- [M01 fuzz campaign 106014812071](https://github.com/KayzenRoot/core/actions/runs/35486894184/job/106014812071): SUCCESS.

The predecessor run at `e471eb495b9561fbbe0ccfed6140d8bd6af1e99c` was superseded after loaded Windows timing assertions failed; the final implementation head has the new in-drain proof. No unresolved HIGH/CRITICAL finding is reported by the required hosted gates.

## Canonical acceptance criteria

1. **All M01-required crates/contracts/mechanisms exist at governed boundaries — PASS.** Existing M01 workspace and hosted Governance/platform gates pass.
2. **Workspace dependency graph is acyclic and forbidden dependencies are absent — PASS.** Locked checks and hosted supply-chain gates pass.
3. **RLC state transitions are machine-enforced with receipts — PASS.** Existing lifecycle receipts and runtime tests pass.
4. **BSR is required before READY/DEGRADED eligibility — PASS.** Bootstrap tests and hosted platform jobs pass.
5. **Module/capability registries are deterministic and support CAL/SIR/CBR/FCH — PASS.** New `cloned_registries_cannot_acquire_after_admission_closes`, `concurrent_acquire_and_close_have_one_registry_linearization_point` and the repeated in-drain integration proof prove typed admission closure, counted pre-close successes and no post-close lease increase; existing registry suite remains green.
6. **Capability substitution is atomic and generation/epoch coherent — PASS.** The new gate is in `RegistryState` beside lease insertion; existing substitution, generation coherence and lease-release tests pass.
7. **DCS golden vectors prove stable canonical identity — PASS.** Existing identity suite and hosted jobs pass.
8. **Runtime Journal integrity/recovery/corruption handling passes — PASS.** Existing journal suite and hosted jobs pass.
9. **SBR/EEB reject stale epoch mutation and blind replay — PASS.** Existing stale lease/IPC paths and hosted jobs pass.
10. **QDS/QVM distinguish clean shutdown from residual forced termination — PASS.** Shutdown closes capability admission before LeaseDrain, QVM has separate general/capability gate items, active leases are re-read immediately before QVM, and `qvm_detects_active_lease_without_caller_claim` proves a nonzero lease cannot produce clean STOP.
11. **DCM/QFC enforce safe degradation and quality-floor continuity — PASS.** Existing degradation and fallback tests pass.
12. **PHC/HCC/ODF health behavior passes concurrency/delta tests — PASS.** Existing health suite and 16-cycle soak pass.
13. **Local IPC is bounded, versioned, fuzzed and platform-adapted — PASS.** Existing IPC tests, hosted platform jobs and fuzz campaign pass.
14. **Security/supply-chain gates and unsafe inventory pass — PASS.** Hosted cargo-deny/audit, Governance, SBOM and advisory gates pass; no HIGH/CRITICAL gate finding remains.
15. **Unit/property/integration/fuzz/failure-injection suites pass — PASS.** Workspace tests are 56/56; the new clone/race/QDS tests, 16-cycle in-drain integration proof, residual/timeout/release-wakeup tests and hosted fuzz campaign pass.
16. **Soak tests show no unbounded growth — PASS.** 16-cycle soak passed with handle growth `+3 <= 4`.
17. **PRB/WNF benchmark evidence is reproducible and no unapproved material regression exists — PASS.** Current p50 regression is `13.31121925512975%`, within the `20%` policy.
18. **CLI machine outputs are versioned/stable — PASS.** Existing CLI/benchmark output tests and hosted jobs pass.
19. **Zero-LLM M01 proof passes — PASS.** Existing zero-LLM bootstrap and soak evidence pass.
20. **Exact-head evidence bundle identifies commit/toolchain/platform/baselines — PASS.** This report binds branch, PDF base, final implementation HEAD, local evidence, SBOM, PRB baseline and the exact successful workflow/job IDs.
21. **Independent governed review verdict is APPROVED — PENDING.** Review 010 is required; PR #17 remains open and no independent approval or merge is claimed.

## Final verdict

`READY_FOR_REVIEW` is justified for implementation HEAD `31b08c87c350c745762a1ac06ca28f43f7a9514a`: workflow `35486894184` jobs `106014811918`, `106014812031`, `106014812038` and `106014812071` are all successful; the repeated in-drain test proves the old clone is rejected while the pre-close lease remains active, the lease count does not increase, release wakes QDS, and QVM reaches clean `StopCommit`; all mandatory local and hosted gates pass; and no HIGH/CRITICAL gate finding remains. The STOP condition is reached for handoff to Review 010. M01 is not declared complete and no closeout merge was created.
