# CORE-WO-M03-001 Pack H calibration

## Binding and method

- Work Order: `CORE-WO-M03-001`; increment: `CORE-M03-FREEZE-001`.
- Authorized scope: `M03_WORK_ORDER_ENGINE`, Packs A–H. No M04 implementation, merge, release, checkpoint promotion, or self-review was performed.
- Canonical branch base: `6cae77e1d8814121df6646dec48bca1020119226`; Work Order authorized base: `ac90b1f48c5551e65ecadace95c59f7f0647062f`.
- Product and benchmark head measured: `de6a829bd2f9402214c371446615fc364431f515` (`fix(core): align M03 benchmark fixtures with calibrated caps`).
- HIVE v1.0.0 returned seven projects but did not resolve `KayzenRoot/core`; the Work Order explicitly permits degraded-safe `SOLO_GIT_CANONICAL`. This report does not claim CORE HIVE evidence.
- Benchmark command on both hosts: `cargo bench -p core-work-order --bench m03_work_order`.
- Harness: `synthetic-core-work-order-v1`; one warmup followed by five samples of ten operations each; synchronous, single-threaded, in-memory. Two runs were captured on each host. Timings are median nanoseconds per operation, converted to microseconds below. Raw logs also retain each sample and its min/median/max.
- Windows: Windows 11 Pro build 26200; AMD Ryzen 3 4300GE; `rustc 1.98.1 (48a229cea 2026-09-01)`, `cargo 1.98.1 (797e8a9bc 2026-08-05)`.
- Ubuntu: Ubuntu 26.04 under WSL2, kernel `6.18.33.2-microsoft-standard-WSL2`; same CPU and Rust/Cargo versions.

## Measurements

Values are median µs/op, shown as run 1 / repeat. Variation between runs is retained rather than presented as a production latency guarantee.

| Synthetic scenario | Windows | Ubuntu 26.04 / WSL2 |
| --- | ---: | ---: |
| Parse request, 2 sources / 2 packets | 459.9 / 227.6 | 195.9 / 398.6 |
| Compile, 2 sources / 2 packets | 1,276.6 / 817.7 | 666.5 / 973.2 |
| Compile, 32 sources | 3,027.8 / 2,688.5 | 2,104.1 / 2,416.7 |
| Compile, 32 packets / 32 AEG edges, wide | 7,549.1 / 8,547.9 | 5,463.9 / 5,561.1 |
| Compile, 32 packets / 32 AEG edges, depth 32 | 7,420.2 / 7,232.0 | 5,884.9 / 5,286.8 |
| Compile scope, 64 rules | 879.2 / 930.8 | 767.8 / 763.0 |
| Parse request with a 4,096-byte string | 210.5 / 240.8 | 215.9 / 214.9 |
| Compile request with a 4,096-byte string | 817.9 / 867.1 | 697.4 / 747.5 |
| Validate frozen Work Order, 2 packets | 253.8 / 244.4 | 238.2 / 228.5 |
| Canonical semantic bytes, 2 packets | 88.6 / 92.3 | 86.4 / 95.1 |
| Revision diff, 2 changed fields | 651.5 / 650.2 | 610.5 / 611.4 |
| Correction classification | 262.2 / 245.6 | 253.1 / 233.5 |
| Lineage compile, 32 edges | 1,162.8 / 1,134.0 | 1,033.5 / 1,060.3 |
| READY admission | 374.8 / 412.2 | 281.4 / 302.3 |
| READY handoff | 286.6 / 341.5 | 250.5 / 268.0 |

The harness observed host/run variance, including roughly 2x shifts in a few medians. These are bounded synthetic measurements, not production throughput or latency SLAs. The chosen limits are finite input/cardinality ceilings derived from accepted synthetic cases; the timing measurements do not justify larger ceilings.

## Selected calibrated profile and rejected cap-plus-one inputs

Every cap below is accepted by `M03ResourceBudgetV1::CALIBRATED_V1`; every cap-plus-one case was rejected by the H5 benchmark boundary checks. The prior values are limits in the earlier synthetic fixture generator, not production limits.

| Dimension | Earlier fixture limit | Selected cap | Rejected cap + 1 |
| --- | ---: | ---: | ---: |
| Request bytes | 128,000 | 78,333 | 78,334 |
| Frozen bytes | 128,000 | 78,898 | 78,899 |
| String bytes | 16,000 | 4,096 | 4,097 |
| Source references | 32 | 32 | 33 |
| Packets | 32 | 32 | 33 |
| Packet DAG edges | 128 | 31 | 32 |
| Scope rules | 256 | 64 | 65 |
| Acceptance criteria | 64 | 32 | 33 |
| Evidence requirements | 64 | 32 | 33 |
| AEG edges | 256 | 32 | 33 |
| Lineage edges | 128 | 32 | 33 |
| Context references | 256 | 32 | 33 |
| Correction rules | 64 | 64 | 65 |
| Diff entries | 256 | 3 | 4 |
| Diagnostic entries | 64 | 4 | 5 |
| JSON parse depth | 64 | 7 | 8 |

The finite profile is enforced in the product API. Uncalibrated budgets and attempts to raise any field above this profile return typed `InvalidContextBudget`; lower positive calibrated limits remain available. Boundary checks also verify failure is typed and no partial compilation is returned.

## Supported envelope and unsupported scales

The evidence supports the listed ceilings per dimension and the measured compound fixtures: 32 packets with up to 31 DAG edges and 32 AEG edges, 32 source references, 32 context references, 32 lineage edges, 64 scope rules, 64 correction rules, a 4,096-byte string, and the stated request/frozen byte limits. The harness measures the 32-packet width and depth cases with one shared source reference per packet, so context-edge count stays inside the calibrated 32-reference envelope.

The evidence does not establish every possible cross-product of all individual maxima, real repository payload distributions, parallel throughput, I/O costs, or production SLA. Scales above any cap are unsupported: request >78,333 bytes; frozen value >78,898 bytes; string >4,096 bytes; source refs, packets, criteria, evidence requirements, AEG edges, lineage edges, or context refs >32; packet DAG edges >31; scope or correction rules >64; diff entries >3; diagnostics >4; parse depth >7. Cap-plus-one rejection is an explicit safety ceiling, not an estimate of the largest future workload.

## Bounded delta from the earlier synthetic fixture

The calibration reduces the earlier fixture ceilings for request bytes by 49,667 (38.8%), frozen bytes by 49,102 (38.4%), string bytes by 11,904 (74.4%), packet edges by 97 (75.8%), scope rules by 192 (75%), criteria and evidence requirements by 32 each (50%), AEG edges by 224 (87.5%), lineage and context edges by 96 and 224 (75% and 87.5%), diff entries by 253 (98.8%), diagnostics by 60 (93.8%), and parse depth by 57 (89.1%). Source references, packet count, and correction rules retain their earlier fixture ceilings. No numeric production value was invented during planning; the selected profile is bounded to the synthetic measurements above.

## Supply chain and no-I/O evidence

- H5 `cargo audit --json`: 0 advisories across 69 locked dependencies; database update recorded as `2026-09-23T16:07:49+02:00` in [the audit result](M03-CARGO-AUDIT-H5.json).
- H5 `cargo deny check`: advisories, bans, licenses, and sources all pass in [the deny result](M03-CARGO-DENY-H5.txt).
- [SPDX 2.3 SBOM](M03-SBOM.spdx.json): 26 packages and 44 relationships. `Cargo.lock` SHA-256 is `ea0391226f57b7985d53b1c916042d5c99f1cb0d753ca4bf05574908bcfbcf97`; the lockfile is unchanged since the SBOM was generated.
- [Dependency graph and static no-I/O scan](M03-DEPENDENCY-NO-IO-H5.txt): the normal graph is local `core-identity`/`core-contracts`, serialization/error crates, and `sha2` plus its digest primitives. A source scan of the 19 Rust files in those crates and `core-work-order` found no filesystem, environment/cwd, process, network, clock, database, or LLM inference API call sites.

## Raw evidence and disposition

- Windows benchmark: [run 1](M03-BENCH-H5-WINDOWS.log), [repeat](M03-BENCH-H5-WINDOWS-REPEAT.log).
- Ubuntu benchmark: [run 1](M03-BENCH-H5-UBUNTU.log), [repeat](M03-BENCH-H5-UBUNTU-REPEAT.log).
- Boundary rejection tests are in both logs for all 16 dimensions. Fuzz and platform test logs are linked from [Pack G](M03-PACK-G.md).
- During H4 boundary bring-up, two attempts exposed an oversized context-reference count in the synthetic 32-packet fixture and an invalid budget in the source-count cap-plus-one fixture. The benchmark fixture was corrected to match the selected 32-reference profile; all final H5 measurements and boundaries pass. Those attempts were harness setup failures, not accepted H5 evidence.
- H5 Pack H calibration is locally complete. The full Ubuntu workspace still has three existing M02 `core-workspace/tests/service.rs` failures; all three were reproduced from an ext4 archive of base `6cae77e1d8814121df6646dec48bca1020119226` in [the exact-base log](M03-WORKSPACE-BASE-H5-UBUNTU.log). All 40 M03 package tests pass on both operating systems. Hosted checks and independent exact-head review remain pending. No checkpoint promotion or merge is claimed.
