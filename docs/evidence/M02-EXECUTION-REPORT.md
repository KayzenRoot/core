# M02 Execution Report

Work Order: `CORE-WO-M02-001`
Authorized base: `bae47b2021a897396109dfcf42e8632dde13ec21`
Execution branch: `feat/m02-project-workspace-adapter`
Final implementation head: `39b751a2c21462be9061eb850327c3fd930a9d39` (the exact product/calibration head; evidence-only closure may follow).
Executor verdict: `READY_FOR_REVIEW` only if the exact-head hosted gates and independent review conditions below are green; otherwise `BLOCKED`.

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

## HIVE truth

HIVE MCP was available and resolved CORE as project `c65b7abc-533a-411a-bbbb-2b72b976d921`, but reported stale indexed HEAD `fdb4dbe165e74b009c43df3874b6043c9b94710b`. `checkpoint.read` returned typed `source_not_current`; context search was lexical fallback with semantic/rerank unavailable. HIVE was used only as advisory context and never as local path authority or canonical Git truth.

## Validation inventory

- Governance validator and Python bootstrap compilation: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `cargo test --workspace --all-features`: PASS, including M01 regression and all M02 integration/adversarial tests.
- Fuzz targets `m02_path_authority`, `m02_repository_graph`, `m02_git_evidence`: compile gate PASS; bounded hosted campaign is required on the final head.
- Standalone no-Git content drift and nested repository boundary regression tests: PASS.
- Static dependency/zero-LLM/unsafe validator: PASS; no forbidden dependency, LLM term or unsafe construct in `core-workspace`.
- Calibration benchmark: PASS with five measured iterations and finite values on the implementation head; see `M02-CALIBRATION-REPORT.md`.
- Local cargo-deny/cargo-audit: pending availability of those external tools; hosted exact-head jobs are required and cannot be replaced by historical results.

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
31. Ubuntu/Windows exact head — `m02` workflow matrix, pending final hosted run.
32. Supply chain/advisory/license/SBOM — deny/audit/SBOM workflow gates, pending final hosted run.
33. Zero LLM — `scripts/validate_m02.py` and deterministic code paths.
34. Reproducible RCG — `M02-CALIBRATION-REPORT.md`, benchmark source.
35. Finite measured defaults — calibration report and `WorkspaceResourceBudget::finalized`.
36. Calibration Delta bounds — calibration report; only budget finalization/documentation changed.
37. DWS included / WMF absent — source/file map and static inspection.
38. L2/watchers/Rust-native provider absent — dependency/static/file-map checks.
39. Exact evidence mapping — this report and Evidence Bundle.
40. No unresolved HIGH/CRITICAL — final security/advisory/independent review gates required.
41. Independent APPROVED review — not executor-controlled; required before promotion.

## Proposed Checkpoint Delta

Promote only after independent governed review: mark `CORE-WO-M02-001` complete at the exact reviewed implementation head, retain HIVE degraded-currentness as an explicit assurance note, record the final calibration report and hosted Ubuntu/Windows/security/SBOM/fuzz identities, and open M03 discovery only after the M02 promotion decision. The executor does not self-promote the checkpoint or merge the implementation PR.
