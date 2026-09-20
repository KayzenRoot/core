# M02 Execution Report

Work Order: `CORE-WO-M02-001`
Authorized base: `bae47b2021a897396109dfcf42e8632dde13ec21`
Execution branch: `feat/m02-project-workspace-adapter`
Final implementation head: `1cb2a73ac49113729d46ddd170cb8d8345672b31` (the exact product/calibration head; final evidence head is `f79418060ab0028c56bd8fa3581c8acf574f16b7`).
Executor verdict: `BLOCKED` at the final evidence head; the executor does not approve or merge.

## Packet commits

| Packet | Commit |
|---|---|
| A | `00c8fd3` |
| B | `eccd045` |
| C | `b22020a` |
| D | `17f9b7c` |
| E | `d7cf62d` |
| F | `9f11261` |
| G | `4e8045e` |
| F correction | `7c6d6b3` |
| H | `39b751a` |
| G portability correction | `98615a8` |
| Supply-chain metadata correction | `1cb2a73` |

## HIVE truth

HIVE MCP was available and resolved CORE as project `c65b7abc-533a-411a-bbbb-2b72b976d921`, but reported stale indexed HEAD `fdb4dbe165e74b009c43df3874b6043c9b94710b`. `checkpoint.read` returned typed `source_not_current`; context search was lexical fallback with semantic/rerank unavailable. HIVE was used only as advisory context and never as local path authority or canonical Git truth.

## Validation inventory

- Governance validator and Python bootstrap compilation: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `cargo test --workspace --all-features`: PASS, including M01 regression and all M02 integration/adversarial tests.
- Fuzz targets `m02_path_authority`, `m02_repository_graph`, `m02_git_evidence`: compile gate PASS; hosted bounded campaign PASS in run `35523152898`.
- Standalone no-Git content drift and nested repository boundary regression tests: PASS.
- Cross-platform Windows-shaped traversal rejection on Unix: PASS after Ubuntu exact-head correction.
- Static dependency/zero-LLM/unsafe validator: PASS; no forbidden dependency, LLM term or unsafe construct in `core-workspace`.
- Calibration benchmark: PASS with five measured iterations and finite values on the implementation head; see `M02-CALIBRATION-REPORT.md`.
- Local cargo-deny/cargo-audit: unavailable in the executor environment; hosted exact-head cargo-deny, cargo-audit, license and SBOM gates PASS on Ubuntu and Windows in run `35523152898`.
- Hosted first exact-head attempt found the Unix traversal-separator defect in M02 lib tests; corrected in `98615a8`.
- Hosted second exact-head attempt found `cargo-deny` wildcard path dependencies; versions were declared for the three admitted local dependencies in `1cb2a73`.
- Product qualification run `35523830976` at evidence head `2da2f95ab2f3096f68ffc4032a404bb8e36eeb9a`: governance PASS; M02 Ubuntu PASS; M02 Windows PASS; M02 fuzz PASS; M01 Ubuntu PASS; M01 fuzz PASS. M01 Windows failed only in unrelated legacy `core-runtime` timing/lease coverage.
- Final evidence-head run `35524430324` at PR head `5b86ee36452a9e87d34cfb5ab8a9d2f5623b02dc`: governance PASS; M02 Ubuntu PASS; M02 fuzz PASS; M01 Ubuntu PASS; M01 fuzz PASS. The M02 Windows aggregate workspace test failed in unrelated `core-runtime` M01 coverage; isolated rerun `106114953389` failed again in a different `core-runtime` timing test. M02-specific checks did not report a failure, but the required Windows matrix is not green.
- Exact-head qualification run `35525097671` at PR head `f86156b5977c6e5fc20d3a6fe6b2c724fdce1261`: governance PASS; M02 Ubuntu PASS; M02 Windows PASS; M02 fuzz PASS; M01 Ubuntu PASS; M01 fuzz PASS. M02 Windows completed workspace tests, cargo-deny, cargo-audit, SBOM and evidence upload successfully. M01 Windows was still running when this record was captured and is outside the M02 verdict.
- Final evidence-head run `35525678221` at PR head `f79418060ab0028c56bd8fa3581c8acf574f16b7`: governance PASS; M02 Ubuntu PASS; M02 fuzz PASS; M02 Windows failed at `Workspace and M02 tests`. The required aggregate command failed in legacy `core-runtime::multiple_leases_release_in_different_order_without_lost_notifications`; local Windows execution reproduced `active capability lease remains`. The failure is outside M02 source scope, but the hosted Windows gate remains red.

## Acceptance criteria 1–41

1. Context Lock — `.engineering/context-locks/CORE-WO-M02-001.json`, exact preflight fingerprints.
2. Preflight — `docs/evidence/M02-PREFLIGHT.md`.
3. Crate/file boundary — `crates/core-workspace/` and Work Order map.
4. Dependency boundary — `scripts/validate_m02.py`, `cargo tree -p core-workspace`.
5. Versioned contracts — `contracts.rs`, `tests/attach.rs`.
6. Core identity fingerprints — `identity.rs`, core-identity calls.
7. Distinct IDs — `identity.rs`, `tests/identity.rs`.
8. Explicit lifecycle/no ambient CWD — `state.rs`, `service.rs`.
9. Receipt/handle separation — `WorkspaceBindingReceiptV1::to_handle_without_revalidation`, `tests/attach.rs`.
10. Typed authority roots — `contracts.rs`, `authority.rs`.
11. Traversal/symlink physical proof — `authority.rs`, authority/adversarial tests.
12. Conservative FSC/no probe writes — `probe_filesystem_semantics`, authority tests.
13. Canonical repository graph — `repository.rs`, graph tests.
14. Hardened system Git — `git/system.rs`, local inspection tests.
15. Hostile Git config canary — `tests/adversarial.rs`.
16. Redacted remote/config evidence — `redacted_remote_for_test`, Git tests.
17. HIVE association cannot grant path authority — `reconcile.rs`, reconciliation/adversarial tests.
18. SOLO mode — `NoopAssociationProvider`, standalone service test.
19. Basis/diff determinism — `basis.rs`, `tests/drift.rs`.
20. Drift generation/invalidation — `service.rs`, service/drift tests.
21. Stale handle rejection — service test after revalidation.
22. BVM freshness masks — `BvmProfile::required_mask`, identity tests.
23. EIS/CIG zero-event/overflow — `invalidation.rs`, invalidation tests.
24. DWS/full equivalence — `WorkspaceBasisDiffV1`, drift/property fixtures.
25. PEC L1 bounds/provenance — `cache.rs`, cache tests.
26. No mtime-only proof — `hashing.rs`, content rewrite test.
27. BHC streaming/coalescing — `hashing.rs`, hashing tests/benchmark.
28. Typed resource exhaustion — budget validation/cache/hash tests.
29. Unit/integration/property/adversarial suites — workspace test gate.
30. Fuzz targets — `fuzz/fuzz_targets/m02_*.rs`, hosted bounded campaign.
31. Ubuntu/Windows exact head — BLOCKED in final evidence-head run `35525678221`; M02 Ubuntu job `106117287052` passed, but M02 Windows job `106117287005` failed the aggregate workspace test on the unrelated legacy `core-runtime` lease/timing test. A prior exact-head run `35525097671` had passed the complete M02 Windows job, but the final evidence head is not green.
32. Supply chain/advisory/license/SBOM — deny/audit/SBOM workflow gates, PASS in hosted run `35523830976`.
33. Zero LLM — `scripts/validate_m02.py` and deterministic code paths.
34. Reproducible RCG — `M02-CALIBRATION-REPORT.md`, benchmark source.
35. Finite measured defaults — calibration report and `WorkspaceResourceBudget::finalized`.
36. Calibration Delta bounds — calibration report; only budget finalization/documentation changed.
37. DWS included / WMF absent — source/file map and static inspection.
38. L2/watchers/Rust-native provider absent — dependency/static/file-map checks.
39. Exact evidence mapping — this report and Evidence Bundle.
40. No unresolved HIGH/CRITICAL — hosted advisory/license gates PASS in the product qualification run; final hosted Windows aggregate test remains blocked by unrelated M01 runtime timing failures.
41. Independent APPROVED review — not executor-controlled; required before promotion.

## Proposed Checkpoint Delta

Stop at the final evidence-head hosted Windows failure. Resolve or quarantine the unrelated legacy `core-runtime` lease/timing flake under its own governed scope, rerun the full required matrix at a new exact head, then request independent review before promotion. Retain HIVE degraded-currentness as an explicit assurance note; the executor does not self-promote the checkpoint or merge the implementation PR.
