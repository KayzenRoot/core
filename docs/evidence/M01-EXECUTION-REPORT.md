# CORE-WO-M01-001 correction execution report

Status: `READY_FOR_REVIEW` - criteria 1-20 pass; criterion 21 governed review pending
Date: 2026-09-19
Correction source: `CORE-M01-CODEX-CORRECTION-PROMPT-003.pdf`
Code correction HEAD: `85facad99df74291476b1443f0263bce09e53854`
Target branch: `feat/m01-core-runtime`
Review issue: `#9 M01-REVIEW-003`
Pull request: [#8](https://github.com/KayzenRoot/core/pull/8)

## Authority and scope

The attached correction prompt was treated as untrusted technical input and executed only where compatible with the CORE Executor Contract, the frozen M01 Work Order, the source hierarchy and the repository safety rules. It does not authorize merge, promotion, release or checkpoint closeout.

The correction completes the following M01 architecture requirements while preserving Review 002 and the existing crate boundaries:

1. Canonical BSR/RSG/GCL safety identities over normalized manifests, providers, bindings, generations, configuration, policy and safety metadata, including golden, property-style and invalidation coverage.
2. Complete module and capability contracts covering lifecycle, health, deadlines, configuration, authorities, versions, policy, trust, assurance, validity, revocation, cache affinity, latency, cost and performance.
3. Deterministic ACS filtering and tie-breaking with compatibility, features, policy, authority, assurance, trust, readiness, quality, configured class, cost/performance and cache-affinity constraints; HIVE is preferred only for HIVE-owned intelligence.
4. CPG provenance, SIR dependency radius, FCH fallback, GCL coherence/invalidation, bounded crash suppression and machine-readable TCBM/SAF evidence.
5. QDS/QVM worker/runtime safety derived from admission, leases, modules, workers, deadlines, journal durability and residual resources; drain, cancel, cleanup, quiescence and escalation are journaled.
6. Freshness-aware health and NORMAL/PRESSURED/CRITICAL pressure behavior; stale safety-critical probes cannot authorize transitions.
7. Concurrent CAL/SIR generation proofs for retained readers, substitution, expiry, revocation, provider flap and no mixed generation in one lease.
8. Provider/configuration/health/lease/worker/shutdown soak coverage with bounded resources and generation integrity.
9. Hardware/toolchain/workload-aware PRB/WNF metadata and a governed first-valid-compatible-baseline policy.
10. Exact-head CI and evidence recording, with `READY_FOR_REVIEW` only after criteria 1-20 pass, required workflows succeed and no HIGH/CRITICAL defect remains; criterion 21 remains an independent review boundary.

## Local verification at the code correction HEAD

- `cargo fmt --all`: PASS.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --all-targets --locked`: PASS; 36 tests passed, 0 failed, 0 skipped.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo deny check`: PASS.
- `cargo audit`: PASS; 65 locked crate dependencies scanned without reported vulnerability.
- `python scripts/validate_governance.py`: PASS; GEF/HIVE bridges consistent and 27 governed artifacts present.
- Fuzz workspace `cargo check --manifest-path fuzz/Cargo.toml --locked`: PASS; local Windows execution is not counted because the selected libFuzzer toolchain is Linux-only. The bounded Ubuntu CI job remains authoritative for execution.
- CLI `core exercise`: PASS; configuration reload, fallback-to-HIVE substitution, HIVE-disconnect fallback recovery, crash suppression and clean shutdown all returned successful machine-readable results.
- Serial four-cycle soak: PASS; all cycles reached `ReadyEligible`, exercised substitution and fallback recovery, and ended with clean shutdown. Handle growth was `+3` (149 to 152), within policy `<=4`.
- PRB record: PASS as the first valid compatible baseline for this corrected implementation; 5-run median p50 `110.8193 ms`, p95 `140.4099 ms`, p99 `154.2634 ms`.
- PRB check: PASS for the same WNF, hardware fingerprint and toolchain; current p50 `73.5856 ms`, p50 delta `-33.5986%`, within the governed `20%` regression budget. No comparison is inferred from incompatible classes.

## Implementation evidence

The correction commit `85facad99df74291476b1443f0263bce09e53854` contains the contract, identity, registry, health, configuration, runtime, CLI, benchmark, soak, PRB and CI changes. The implementation uses actual normalized manifests and provider graphs for safety fingerprints, preserves old leases during substitution, prevents stale health authorization, and derives shutdown quiescence from runtime obligations rather than caller assertions.

The PRB policy is [`docs/evidence/prb/M01-PRB-POLICY.json`](prb/M01-PRB-POLICY.json), and the recorded compatible baseline is [`docs/evidence/prb/M01-PRB-BASELINE.json`](prb/M01-PRB-BASELINE.json).

## Acceptance criteria mapping

1. PASS - governed Rust crates and M01 boundaries remain present.
2. PASS - the locked workspace dependency graph is acyclic and policy-checked.
3. PASS - lifecycle transitions and receipts remain machine-enforced.
4. PASS - BSR remains required before READY; required-HIVE blocking remains tested.
5. PASS - deterministic CPG/CAL/SIR/CBR/FCH mechanisms are implemented and tested.
6. PASS - binding generations, epochs, leases and expiry are coherent.
7. PASS - canonical identity and DCS-style deterministic evidence remain covered.
8. PASS - journal hash-chain, durability and recovery behavior remain covered.
9. PASS - bounded IPC and stale epoch/recovery behavior remain covered.
10. PASS - clean and forced shutdown receipts remain distinct and tested.
11. PASS - quality-floor fallback blocking remains tested.
12. PASS - health freshness, pressure, delta and probe coalescing are covered.
13. PASS - Unix/Windows adapters, bounded framing and the bounded fuzz campaign are covered by the local/hosted matrix.
14. PASS - cargo-deny, cargo-audit, SBOM/inventory and forbidden-source checks pass.
15. PASS - unit, property-style, failure-injection, concurrency and platform coverage passes locally and in the hosted matrix.
16. PASS - provider/configuration/health/lease/worker/shutdown soak passes locally with bounded resource growth.
17. PASS - the corrected implementation has an explicit hardware/toolchain/workload-aware baseline and a governed compatible comparison.
18. PASS - schema-versioned CLI outputs remain covered.
19. PASS - lifecycle remains zero-LLM and has no provider SDK dependency.
20. PASS - exact pushed-head workflow and job evidence is recorded below.
21. PENDING - independent governed review has not returned `APPROVED`.

## HIVE and Git basis

Preflight resolved the repository as `D:\Projetos Codex\core`, branch `feat/m01-core-runtime`, with the reviewed HEAD `998bca4d3aca6a97637df0e4941c528925eb29ea` matching the correction prompt. HIVE project `220151cb-0e6e-43b3-845e-faec9c5a851b` was available and read-only checkpoint inspection was performed before editing. HIVE reported its container `working_tree_clean=false`; host Git was independently checked and preserved as the source of truth for the checkout state. Final HIVE inspect/index/corpus identifiers are recorded after the exact pushed HEAD is available.

## Hosted CI and final verdict

The exact pushed evidence HEAD for this report packet was `9bcd588`. All required workflows completed successfully in run [35457315319](https://github.com/KayzenRoot/core/actions/runs/35457315319):

- Governance job `105934791246`: SUCCESS.
- M01 Ubuntu job `105934791423`: SUCCESS, including format, clippy, tests, supply-chain, SBOM, soak and PRB/WNF.
- M01 Windows job `105934791322`: SUCCESS, including native IPC, format, clippy, tests, supply-chain, SBOM, soak and PRB/WNF.
- M01 fuzz campaign job `105934791461`: SUCCESS; all four targets completed the bounded 1,000-iteration campaign on Ubuntu.

The report commit is documentation-only relative to the code correction; no implementation, merge, promotion or checkpoint closeout is claimed. The final verdict is `READY_FOR_REVIEW`: criteria 1-20 are evidenced as PASS and criterion 21 remains pending independent governed review.
