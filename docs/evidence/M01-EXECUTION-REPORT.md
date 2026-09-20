# CORE-WO-M01-001 execution report — Prompt 008

Status: `READY_FOR_REVIEW` — criteria 1–20 pass; criterion 21 governed review pending
Date: 2026-09-19
Correction source: `CORE-M01-CODEX-CORRECTION-PROMPT-008.pdf`
Repository: `KayzenRoot/core`
Target branch: `fix/m01-cal-admission-close`
Exact PDF base: `8f0820188130613caa6890c82d43225d9a317947`
Implementation HEAD: `895f68d410f964063e95f062b32ff2755461bb8d`
Review source: Issue [#16](https://github.com/KayzenRoot/core/issues/16)
Pull request: [#17](https://github.com/KayzenRoot/core/pull/17)

## Authority and bounded scope

Prompt 008 was executed in document order under the CORE Executor Contract, the frozen M01 Work Order and repository rules. The correction is limited to the CAL/QDS admission race:

1. Capability lease admission is stored in the existing synchronized `RegistryState` and is closed by `close_lease_admission()` without a reopen path for the runtime epoch.
2. `RegistryError::AdmissionClosed` is returned by `acquire_lease_with_generation()` while holding the same write lock used for lease insertion.
3. `Supervisor::shutdown()` closes capability admission before LeaseDrain; runtime status exposes both admission gates.
4. QVM proves both gates, re-reads active leases immediately before the quiescence matrix, and cannot produce clean STOP with a residual lease.
5. Deterministic clone, race, QDS integration, release-wakeup, timeout and residual tests preserve generation, revision, expiry, substitution and release/revoke behavior.

The second commit only stabilizes pre-existing lifecycle test timing under loaded hosted Windows runners; it does not alter production behavior. No merge, promotion, release or M01 completion is claimed.

## Changed files

- `crates/core-registry/src/lib.rs`
- `crates/core-runtime/src/lib.rs`

## Exact-head local evidence

- Toolchain: stable Rust `cargo/rustc 1.98.1`; system Python `3.12.10`.
- `cargo fmt --all -- --check`: PASS.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --all-targets --locked`: PASS; 50 tests, 0 failed; `core-registry` 15/15 and `core-runtime` 19/19.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo check --manifest-path fuzz/Cargo.toml --locked`: PASS.
- `python scripts/validate_governance.py`: PASS; GEF `v1.0.0`, HIVE compatibility `v1.0.0`, bridges consistent and 27 governed artifacts.
- Python compilation for all six scripts: PASS.
- HIVE bootstrap/MCP unit tests: PASS; 6/6.
- Admission race test repeated five times: PASS; each run completed 1/1.
- M01 soak, 16 iterations: PASS; every cycle reached `ReadyEligible`, process handles `151 -> 154` (`+3`, policy `<=4`), record fingerprint `fce220351520b9717d0db917fd6f33a6568c65c9c4ef0fe441356dd0852eb4ba`.
- PRB/WNF: PASS; compatible baseline p50 `110.8193 ms`, current median p50 `125.5707 ms`, regression `13.31121925512975%`, allowed `20%`.
- Local SBOM SHA-256: `1c32c13097b7889c606f5af6d62fe20b3d6fa437ac055d973ca526cece4d61c7`.
- Local `cargo-deny` and `cargo-audit` executables were unavailable; hosted Ubuntu/Windows supply-chain and advisory steps passed.
- Local artifacts: `artifacts/m01-soak-prompt008-16.json`, `artifacts/m01-prb-prompt008.json`, `artifacts/m01-sbom-prompt008.cdx.json` and its SHA-256 sidecar.

## HIVE and Git basis

Read-only HIVE preflight resolved the registered CORE project as `c65b7abc-533a-411a-bbbb-2b72b976d921` on `main` at `fdb4dbe165e74b009c43df3874b6043c9b94710b`, READY and clean in HIVE. The correction checkout is the exact PDF branch and is not HIVE-registered; checkpoint read returned `source_not_current`. Git is therefore canonical for this target branch and no target-branch HIVE evidence is claimed.

The branch was created from the remote PDF base `8f0820188130613caa6890c82d43225d9a317947`. The implementation commits are:

- `e471eb495b9561fbbe0ccfed6140d8bd6af1e99c` — linearize capability lease admission close.
- `895f68d410f964063e95f062b32ff2755461bb8d` — stabilize lifecycle timing assertions for the hosted Windows gate.

## Hosted exact-head evidence

Workflow [35484136893](https://github.com/KayzenRoot/core/actions/runs/35484136893) completed `success` at exact implementation HEAD `895f68d410f964063e95f062b32ff2755461bb8d`:

- [Governance job 106007237663](https://github.com/KayzenRoot/core/actions/runs/35484136893/job/106007237663): SUCCESS.
- [M01 Ubuntu job 106007237655](https://github.com/KayzenRoot/core/actions/runs/35484136893/job/106007237655): SUCCESS, including format, clippy, tests, supply-chain, advisory, SBOM, soak and PRB/WNF.
- [M01 Windows job 106007237632](https://github.com/KayzenRoot/core/actions/runs/35484136893/job/106007237632): SUCCESS, including native IPC, supply-chain, advisory, SBOM, soak and PRB/WNF.
- [M01 fuzz campaign 106007237534](https://github.com/KayzenRoot/core/actions/runs/35484136893/job/106007237534): SUCCESS.

The predecessor run at `e471eb495b9561fbbe0ccfed6140d8bd6af1e99c` was superseded after loaded Windows timing assertions failed; the final head has deterministic timing tests and all hosted jobs are green. No unresolved HIGH/CRITICAL finding is reported by the required hosted gates.

## Canonical acceptance criteria

1. **All M01-required crates/contracts/mechanisms exist at governed boundaries — PASS.** Existing M01 workspace and hosted Governance/platform gates pass.
2. **Workspace dependency graph is acyclic and forbidden dependencies are absent — PASS.** Locked checks and hosted supply-chain gates pass.
3. **RLC state transitions are machine-enforced with receipts — PASS.** Existing lifecycle receipts and runtime tests pass.
4. **BSR is required before READY/DEGRADED eligibility — PASS.** Bootstrap tests and hosted platform jobs pass.
5. **Module/capability registries are deterministic and support CAL/SIR/CBR/FCH — PASS.** New `cloned_registries_cannot_acquire_after_admission_closes` and `concurrent_acquire_and_close_have_one_registry_linearization_point` prove typed admission closure and counted pre-close successes; existing registry suite remains green.
6. **Capability substitution is atomic and generation/epoch coherent — PASS.** The new gate is in `RegistryState` beside lease insertion; existing substitution, generation coherence and lease-release tests pass.
7. **DCS golden vectors prove stable canonical identity — PASS.** Existing identity suite and hosted jobs pass.
8. **Runtime Journal integrity/recovery/corruption handling passes — PASS.** Existing journal suite and hosted jobs pass.
9. **SBR/EEB reject stale epoch mutation and blind replay — PASS.** Existing stale lease/IPC paths and hosted jobs pass.
10. **QDS/QVM distinguish clean shutdown from residual forced termination — PASS.** Shutdown closes capability admission before LeaseDrain, QVM has separate general/capability gate items, active leases are re-read immediately before QVM, and `qvm_detects_active_lease_without_caller_claim` proves a nonzero lease cannot produce clean STOP.
11. **DCM/QFC enforce safe degradation and quality-floor continuity — PASS.** Existing degradation and fallback tests pass.
12. **PHC/HCC/ODF health behavior passes concurrency/delta tests — PASS.** Existing health suite and 16-cycle soak pass.
13. **Local IPC is bounded, versioned, fuzzed and platform-adapted — PASS.** Existing IPC tests, hosted platform jobs and fuzz campaign pass.
14. **Security/supply-chain gates and unsafe inventory pass — PASS.** Hosted cargo-deny/audit, Governance, SBOM and advisory gates pass; no HIGH/CRITICAL gate finding remains.
15. **Unit/property/integration/fuzz/failure-injection suites pass — PASS.** Workspace tests are 50/50; the new clone/race/QDS tests, five repeated race runs, residual/timeout/release-wakeup tests and hosted fuzz campaign pass.
16. **Soak tests show no unbounded growth — PASS.** 16-cycle soak passed with handle growth `+3 <= 4`.
17. **PRB/WNF benchmark evidence is reproducible and no unapproved material regression exists — PASS.** Current p50 regression is `13.31121925512975%`, within the `20%` policy.
18. **CLI machine outputs are versioned/stable — PASS.** Existing CLI/benchmark output tests and hosted jobs pass.
19. **Zero-LLM M01 proof passes — PASS.** Existing zero-LLM bootstrap and soak evidence pass.
20. **Exact-head evidence bundle identifies commit/toolchain/platform/baselines — PASS.** This report binds branch, PDF base, final implementation HEAD, local evidence, SBOM, PRB baseline and exact workflow/job IDs.
21. **Independent governed review verdict is APPROVED — PENDING.** Review 009 is required; PR #17 remains open and no independent approval or merge is claimed.

## Final verdict

`READY_FOR_REVIEW` is justified for implementation HEAD `895f68d410f964063e95f062b32ff2755461bb8d`: the CAL admission close/acquire operations share one registry write-lock linearization point; QDS closes that gate before LeaseDrain; QVM proves both gates and re-reads active leases before STOP; all mandatory local and hosted gates pass; and no HIGH/CRITICAL gate finding remains. The STOP condition is reached for handoff to Review 009. M01 is not declared complete and no closeout merge was created.
