# CORE-WO-M03-001 Pack G local qualification (H5)

## Scope and binding

- Work Order: `CORE-WO-M03-001`; authorized scope: `M03_WORK_ORDER_ENGINE`.
- Execution branch: `feat/m03-work-order-engine`; implementation is based on canonical main `6cae77e1d8814121df6646dec48bca1020119226` and the active Context Lock.
- Final measured product/benchmark head: `de6a829bd2f9402214c371446615fc364431f515`.
- This report records local worktree evidence. A final implementation commit, hosted exact-head checks, and independent review are separate gates.
- HIVE v1.0.0 read-only MCP returned seven projects but did not resolve `KayzenRoot/core`; degraded-safe `SOLO_GIT_CANONICAL` execution was explicitly permitted. No CORE HIVE checkpoint result is claimed.

## Local checks

| Check | Windows | Ubuntu 26.04 / WSL2 |
| --- | --- | --- |
| `cargo test -p core-work-order` | 40 passed | 40 passed |
| `cargo clippy -p core-work-order --all-targets -- -D warnings` | PASS | PASS |
| M03 Rust formatting and benchmark compilation | PASS | PASS |
| `cargo test --workspace` | 143 passed, 0 failed | 140 passed, 3 failed in existing `core-workspace/tests/service.rs` cases |
| `python scripts/validate_governance.py` | PASS; 27 required artifacts | N/A (host-local governance validator) |

The Ubuntu workspace failures are `attach_path_revalidate_and_detach_are_explicit`, `no_git_content_change_invalidates_standalone_binding`, and `unrelated_event_hint_cannot_hide_nested_repository_graph_drift`. All three also fail when the same tests run from an ext4 archive of the authorized canonical branch base `6cae77e1d8814121df6646dec48bca1020119226`; the [exact-base result](M03-WORKSPACE-BASE-H5-UBUNTU.log) records all three. M03 does not modify `crates/core-workspace`. The exact-base reproduction is separate from the passing M03 package suite.

Exact H5 workspace logs: [Windows](M03-WORKSPACE-H5-WINDOWS.log), [Ubuntu](M03-WORKSPACE-H5-UBUNTU.log). Exact H5 M03 package logs: [Windows](M03-TEST-H5-WINDOWS.log), [Ubuntu](M03-TEST-H5-UBUNTU.log).

## Fuzz and security evidence

Seven Ubuntu libFuzzer targets each completed 1,000 executions at H5 with maximum input length 32,768 bytes, timeout 10 seconds, and RSS limit 1,024 MiB. Per-target execution counts and peak RSS are recorded for [admission replay](fuzz/m03_admission_replay_h5_ubuntu.log), [diagnostic redaction](fuzz/m03_diagnostics_redaction_h5_ubuntu.log), [canonical envelope](fuzz/m03_envelope_canonical_h5_ubuntu.log), [lineage](fuzz/m03_lineage_lpc_h5_ubuntu.log), [packet DAG](fuzz/m03_packet_dag_h5_ubuntu.log), [scope delta](fuzz/m03_scope_delta_h5_ubuntu.log), and [source provenance](fuzz/m03_source_provenance_h5_ubuntu.log).

The redaction target found a real unsafe-subject gap for the synthetic input `cpayloadary_api_`. The subject validator now rejects `prompt` and `payload` markers; a named regression seed is retained at `fuzz/corpus/m03_diagnostics_redaction/payload-subject-regression.txt`. The fixed code passed the redaction regression tests on both platforms and its bounded fuzz campaign.

The exact H5 `cargo tree -p core-work-order --edges normal` includes local `core-identity` and `core-contracts`, `serde`, `serde_json`, `thiserror`, and `sha2` with digest primitives. A static scan of all 19 Rust source files in those crates and `core-work-order` found no filesystem, process, network, environment/cwd, database, ambient-clock, or LLM inference API call sites. Compilation and admission remain synchronous and value-only; the caller-timeout discard contract has a deterministic admission-boundary test. See [the H5 dependency/no-I/O record](M03-DEPENDENCY-NO-IO-H5.txt).

H5 supply-chain artifacts: [SPDX 2.3 SBOM](M03-SBOM.spdx.json) (26 packages, 44 relationships; `Cargo.lock` unchanged since SBOM generation), [cargo-audit JSON](M03-CARGO-AUDIT-H5.json) (0 advisories across 69 locked dependencies), and [cargo-deny results](M03-CARGO-DENY-H5.txt) (advisories, bans, licenses, and sources all pass).

## Pack G disposition

M03-specific local tests, static checks, bounded fuzz, redaction, dependency, advisory, license, and SBOM evidence pass at H5. The three Ubuntu M02 workspace failures remain an explicitly reproduced base-commit limitation; they were not changed under the M03-only scope. Hosted checks at the final commit and the independent exact-head review remain pending. Pack H calibration is recorded in [the H5 calibration report](M03-PACK-H-CALIBRATION.md).
