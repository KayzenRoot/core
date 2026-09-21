# M02 Execution Report

Work Order: `CORE-WO-M02-001`
Authorized base: `bae47b2021a897396109dfcf42e8632dde13ec21`
Execution branch: `feat/m02-project-workspace-adapter`
Final implementation head: `098de069b77aa843248b5a66dd0ca8af4107e879` (the exact product/correction head).
Executor verdict: `READY_FOR_REVIEW`; all required local and hosted gates are green at the exact implementation head. The executor does not approve, promote or merge.

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
| M02 Review 007 correction | `09269ec` |
| M02 Review 008 correction | `098de06` |

## Review 007 correction closeout

The six HIGH findings from M02-REVIEW-007 were corrected in `09269ec65f230bae513623a7e5424466dbd82a0c` without expanding the frozen architecture or dependency boundary:

- HIGH-1: integrated EIS/CIG dirty-state hints, selective DWS revalidation only for equivalent proofs, broad fallback on overflow/unknown/security/policy/provider loss, and PEC L1 provenance.
- HIGH-2: made Git `ContentHashed` consume safe `-z` records, authority-check untracked paths, reject physical escapes, stream through BHC and include deterministic content digests; the byte-change regression passes.
- HIGH-3: made filesystem case semantics conservative `Unknown` instead of inferring verification from the Unix family.
- HIGH-4: replaced path-only HashConveyor reuse with identity-aware re-read, bounded true in-flight coalescing and deadline cleanup.
- HIGH-5: hardened production Git execution with fixed argv/config, disabled fsmonitor and hostile surfaces, concurrent capped stdout/stderr draining, deadline kill and reap.
- HIGH-6: made time revalidation resolve the authority target rather than the process CWD; the relative-collision regression passes.

Exact-head hosted run `35532968666` completed successfully. Governance job `106136806055`, M01 Ubuntu `106136806132`, M01 Windows `106136806021`, M01 fuzz `106136806023`, M02 bounded fuzz `106136806002`, M02 Ubuntu `106136806057` and M02 Windows `106136805858` all concluded `success`.

Local gates also pass: format, locked clippy with `-D warnings`, the full locked workspace test suite, governance/M02 static validators, six Python HIVE/MCP tests, and seven bounded WSL fuzz campaigns with 1,000 executions each. The direct Windows sanitizer fuzz linker remains an environment limitation; the hosted Windows fuzz job is green.

## Review 008 correction closeout

The three residual Review 008 blockers were corrected in `098de069b77aa843248b5a66dd0ca8af4107e879`, on the same branch and PR, without changing the frozen architecture, dependency boundary, budgets or calibration:

- Residual 1 — event hints and PEC are no longer freshness proof. `revalidate_for()` obtains fresh deterministic basis evidence at the action boundary, preserves selective DWS semantics only when the requested mask is unchanged, and falls back to the full computation when evidence is unavailable. Nested repository graph drift under an unrelated `PathContentChanged` hint and selective-versus-full equivalence are covered by regression tests.
- Residual 2 — BHC deadlines are enforced inside bounded streaming rather than by abandoning a detached task. Cancellation is typed, terminal results are shared with waiters, in-flight entries are cleaned and notified, and concurrent same-path requests coalesce into one underlying operation; rewrite-after-completion yields a new digest.
- Residual 3 — Git regressions now prove hostile local fsmonitor configuration does not execute a canary, dual stdout/stderr output is independently capped without deadlock, and a sleeping fake process receives a typed deadline, is killed/reaped, leaves no poisoned inspection state, and does not block the next inspection.

Exact-head hosted run `35539174568` at implementation head `098de069b77aa843248b5a66dd0ca8af4107e879` completed successfully. Governance `106153571798`, M01 Ubuntu `106153571638`, M01 Windows `106153571743`, M01 fuzz `106153571815`, M02 bounded fuzz `106153571761`, M02 Ubuntu `106153571894` and M02 Windows `106153571783` all concluded `success`.

Local Review 008 evidence is green: focused M02 coverage (16 unit and 31 integration/adversarial tests), full locked workspace tests, format, Clippy, governance/M02 validators, Python HIVE/MCP tests, fuzz-bin compilation and the seven-scenario calibration benchmark. No calibration delta was required because selected budgets and dependency inputs were unchanged.

The subsequent evidence-only head `fa1d4afa8ecabc8bbac56206071692c247facd6d` triggered run `35540264458`. Governance, M01 Windows, both fuzz jobs, and both M02 jobs were green (`106156524287`, `106156524171`, `106156524246`, `106156524242`, `106156524225`, `106156524259`); M01 Ubuntu `106156524282` failed in the aggregate test step with exit code 101, with no public hosted log available. This head changed only the evidence/report files, and the exact implementation-head qualification run `35539174568` remained fully green; the M01 failure is therefore recorded as an unrelated legacy gate and not attributed to Review 008.

## HIVE truth

HIVE MCP was available and resolved CORE as project `c65b7abc-533a-411a-bbbb-2b72b976d921`, but reported stale indexed HEAD `fdb4dbe165e74b009c43df3874b6043c9b94710b`. `checkpoint.read` returned typed `source_not_current`; context search was lexical fallback with semantic/rerank unavailable. HIVE was used only as advisory context and never as local path authority or canonical Git truth.

## Validation inventory

- Governance validator and Python bootstrap compilation: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `cargo test --workspace --all-features`: PASS, including M01 regression and all M02 integration/adversarial tests.
- Fuzz targets `m02_path_authority`, `m02_repository_graph`, `m02_git_evidence`: compile gate PASS; hosted bounded campaign PASS in run `35532968666`; the seven local WSL bounded campaigns also completed without crashes.
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
- Subsequent exact-head run `35525969586` at PR head `11e45647d3f0b9cf2f5a20e11d44de511e5b458c`: governance PASS; M02 Ubuntu PASS; M02 Windows PASS; M02 fuzz PASS. M02 Windows completed aggregate tests, cargo-deny, cargo-audit, SBOM and evidence upload successfully. M01 Windows was still running when recorded and is outside the M02 verdict.
- Exact-head run `35526522996` at PR head `c4b46ec9a5d27e11eb3b80d61d6eb67a4a033297`: governance PASS; M02 Ubuntu PASS; M02 Windows PASS; M02 fuzz PASS. M02 Windows completed aggregate tests, cargo-deny, cargo-audit, SBOM and evidence upload successfully. M01 Windows was still running when recorded and is outside the M02 verdict.
- Final evidence-head run `35527050540` at PR head `eb0ec043a0ba1cc786b5b582fa2c3b240b168e1b`: governance PASS; M02 fuzz PASS; M02 Windows failed at `Workspace and M02 tests`. The detailed hosted log was unavailable after connector authentication expiry; the required Windows gate is objectively red. M02 Ubuntu was still running when recorded.
- Subsequent exact-head run `35527342471` at PR head `797f7bf81624fac78ab9557c9abecb3661817688`: governance PASS; M02 Ubuntu PASS; M02 Windows PASS; M02 fuzz PASS. M02 Windows completed aggregate tests, cargo-deny, cargo-audit, SBOM and evidence upload successfully. M01 Windows was still running when recorded and is outside the M02 verdict.
- Final exact implementation-head run `35532968666` at PR head `09269ec65f230bae513623a7e5424466dbd82a0c`: all seven required jobs PASS, including governance, M01/M02 Ubuntu and Windows, and both bounded fuzz campaigns.

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
31. Ubuntu/Windows exact head — PASS at implementation head `098de069b77aa843248b5a66dd0ca8af4107e879`, hosted run `35539174568`.
32. Supply chain/advisory/license/SBOM — deny/audit/SBOM workflow gates, PASS in exact-head hosted run `35539174568`.
33. Zero LLM — `scripts/validate_m02.py` and deterministic code paths.
34. Reproducible RCG — `M02-CALIBRATION-REPORT.md`, benchmark source.
35. Finite measured defaults — calibration report and `WorkspaceResourceBudget::finalized`.
36. Calibration Delta bounds — calibration report; Review 008 correction did not change selected budgets or calibration bounds, confirmed by the seven-scenario benchmark.
37. DWS included / WMF absent — source/file map and static inspection.
38. L2/watchers/Rust-native provider absent — dependency/static/file-map checks.
39. Exact evidence mapping — this report and Evidence Bundle.
40. No unresolved HIGH/CRITICAL — PASS for the six Review 007 findings and the three Review 008 residual blockers; the exact-head hosted matrix, advisory, license and SBOM gates are green.
41. Independent APPROVED review — PENDING Review 009; not executor-controlled and required before promotion.

## Proposed Checkpoint Delta

Request independent governed Review 009 before promotion; retain HIVE degraded-currentness as an explicit assurance note. The executor does not self-promote the checkpoint or merge the implementation PR.
