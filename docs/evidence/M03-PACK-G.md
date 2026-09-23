# CORE-WO-M03-001 Pack G local qualification

## Scope and binding

- Work Order: `CORE-WO-M03-001`; authorized scope: `M03_WORK_ORDER_ENGINE`.
- Execution branch: `feat/m03-work-order-engine`; implementation is based on canonical main `6cae77e1d8814121df6646dec48bca1020119226` and the active Context Lock.
- This report records local worktree evidence. A final implementation commit, hosted exact-head checks, and independent review are separate gates.
- HIVE v1.0.0 read-only MCP returned seven projects but did not resolve `KayzenRoot/core`; degraded-safe `SOLO_GIT_CANONICAL` execution was explicitly permitted. No CORE HIVE checkpoint result is claimed.

## Local checks

| Check | Windows | Ubuntu 26.04 / WSL2 |
| --- | --- | --- |
| `cargo test -p core-work-order` | 39 passed | 39 passed |
| `cargo clippy -p core-work-order --all-targets -- -D warnings` | PASS | PASS |
| M03 Rust formatting and benchmark compilation | PASS | PASS |
| `cargo test --workspace` | 142 passed, 0 failed | 139 passed, 3 failed in existing `core-workspace/tests/service.rs` cases |

The Ubuntu workspace failures are `attach_path_revalidate_and_detach_are_explicit`, `no_git_content_change_invalidates_standalone_binding`, and `unrelated_event_hint_cannot_hide_nested_repository_graph_drift`. All three also fail when the same tests run from an archive of the authorized base commit `6cae77e1d8814121df6646dec48bca1020119226`; M03 does not modify `crates/core-workspace`. The exact-base reproduction is recorded separately from the passing M03 package suite.

Workspace logs: [Windows](M03-WORKSPACE-WINDOWS.log), [Ubuntu](M03-WORKSPACE-UBUNTU.log).

## Fuzz and security evidence

Seven Ubuntu libFuzzer targets each completed 1,000 executions with maximum input length 32,768 bytes, timeout 10 seconds, and RSS limit 1,024 MiB. Final execution counts and peak RSS are in [the fuzz logs](fuzz/).

The redaction target found a real unsafe-subject gap for the synthetic input `cpayloadary_api_`. The subject validator now rejects `prompt` and `payload` markers; a named regression seed is retained at `fuzz/corpus/m03_diagnostics_redaction/payload-subject-regression.txt`. The fixed code passed the redaction regression tests on both platforms and its bounded fuzz campaign.

`cargo tree -p core-work-order --edges normal` shows only the local `core-identity` crate and the declared serialization/error dependencies. A source scan found no filesystem, process, network, environment/cwd, database, ambient-clock, async-runtime, or LLM API calls in the M03 core or its local identity/contracts dependencies. Compilation and admission remain synchronous and value-only; the caller-timeout discard contract has a deterministic admission-boundary test.

Supply-chain artifacts: [SPDX 2.3 SBOM](M03-SBOM.spdx.json) (26 components, 44 relationships), [cargo-audit JSON](M03-CARGO-AUDIT.json) (0 advisories across 69 locked dependencies), and [cargo-deny results](M03-CARGO-DENY.txt) (advisories, bans, licenses, and sources all pass).

## Pack G disposition

M03-specific local tests, static checks, bounded fuzz, redaction, dependency, advisory, license, and SBOM evidence pass on the recorded worktree. The three Ubuntu M02 workspace failures remain an explicitly reproduced base-commit limitation; they were not changed under the M03-only scope. Hosted checks at the final commit and the independent exact-head review remain pending. Pack H calibration is recorded separately.
