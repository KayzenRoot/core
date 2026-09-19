# CORE-WO-M01-001 execution report

Status: `READY_FOR_REVIEW` (local implementation/evidence complete; independent governed review and promotion remain separate gates)
Date: 2026-09-19
Implementation HEAD before the report commits: `ba76569` (`feat(m01): add CLI evidence and project execution rules`). The first report commit was `1df00f0`; this refresh records the final observed integration state.

## Packet commits

| Packet | Commit | Scope |
| --- | --- | --- |
| A | `a4f7b0f` | workspace, contracts, schema versions, DCS/DIF/RSG/SAF and content handles |
| B | `5b81870` | TOML/env/CLI precedence, provenance, validation and redaction |
| C | `0791ae8` | module graph, capability resolution, CPG/CAL/SIR/CBR/FCH hooks |
| D | `2bfa6d8` | append-only hash-chain journal, recovery classification and compaction |
| E | `299dcf1` | bounded versioned local IPC framing, handshake and epoch barrier |
| F | `c76eb9e` | DCM/QFC/HCC/ODF/PHC health and degradation primitives |
| G | `0e81087` | Tokio supervisor, RLC/BSR/QDS/QVM, cancellation and zero-LLM lifecycle |
| H | `ba76569` | CLI JSON contracts, project-wide attachment rule, fuzz corpus and preflight evidence |

## Verification

- `python scripts/validate_governance.py`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo test --workspace --all-targets --locked`: PASS, 22 unit tests, 0 failed, 0 skipped.
- CLI `version`, `validate`, `doctor`, `start`: PASS with schema-versioned JSON; `start` produced `ReadyEligible` and no LLM calls.
- Release BOOT baseline: 25 samples, WNF `3e70c0176fa97ef4c43e95976b5c95b13fa205e8cceb4740bf361f74fa666127`, p50 62.80 ms, p95 85.23 ms, p99 94.35 ms on the current Windows host. PRB policy is baseline-relative and hardware/WNF-bound.
- Adversarial/failure injection: bounded IPC allocation-bomb/truncation/epoch tests, config unknown-key/secret-canary tests, journal truncation/hash-chain/compaction tests, stale lease tests, forced shutdown residual tests and zero-LLM lifecycle tests pass.
- `cargo tree --workspace --locked` and `cargo metadata --locked --no-deps`: PASS; dependency graph is acyclic and lockfile is committed.
- Unsafe inventory: no first-party `unsafe` blocks. `cargo-audit` and `cargo-deny` are not installed on this host; vulnerability/license/advisory scanning is therefore an explicit follow-up gate, not claimed as passed.

## HIVE evidence

Observed HIVE v1.0.0 API health `status=ok`; CORE was inspected as `READY` at branch `feat/m01-core-runtime` and final observed HEAD `1df00f05aa8f2022f122ad42f425ef50140c2db4`. Final index run `969ef2a7-f536-4c6d-9c92-a39048705515` completed at that HEAD with 92 discovered files, 27 indexed files, 65 reused files, 25 added files and 2 changed files. Final corpus run `7913b5dd-a2fb-4321-928d-5004b19c47bc` completed `CURRENT` with 177 references (108 repository sources). HIVE reported `working_tree_clean=false` inside its Linux container while the host Git command with `--untracked-files=no` was clean; this is retained as a truthful integration residual, not suppressed. This is integration/preflight evidence only; M01 runtime code has no HIVE source or database dependency.

## Acceptance criteria mapping

1. PASS - nine governed Rust crates and required M01 contract/mechanism boundaries exist.
2. PASS - workspace dependency graph is acyclic; forbidden runtime dependencies are absent.
3. PASS - closed RLC transitions and typed transition receipts are machine-enforced.
4. PASS - BSR is produced before `READY`; blocked required-HIVE bootstrap has no success receipt.
5. PASS - deterministic module/capability registries expose leasing, impact and binding receipt primitives.
6. PASS - binding generations and boot epochs are checked; stale leases are rejected.
7. PASS - DCS canonical ordering and SHA-256 golden vector pass.
8. PASS - journal is append-only, checksummed/hash-chained, corruption-detecting and bounded-compaction tested.
9. PASS - IPC and capability lease paths reject stale epochs; journal recovery never assumes ambiguous success.
10. PASS - clean and forced shutdown receipts distinguish quiescence from residual termination.
11. PASS - quality-floor continuity blocks an unsafe fallback.
12. PASS - multi-dimensional health, delta snapshots and single-flight probes are tested.
13. PARTIAL - bounded/versioned/fuzz-defended protocol logic and adversarial mutation tests pass; a long-running cargo-fuzz campaign and cross-platform native transport run remain review/CI work.
14. PARTIAL - lockfile, minimal dependency graph and empty unsafe inventory pass; cargo-audit/cargo-deny tooling is unavailable locally.
15. PARTIAL - unit/property-style/adversarial/failure-injection coverage passes; full hosted integration/fuzz campaign remains an independent gate.
16. PARTIAL - local 25-sample BOOT baseline passes; long soak/resource-growth evidence remains a release gate.
17. PASS - reproducible WNF/PRB metadata and baseline-relative policy are emitted.
18. PASS - CLI version/status/validate/doctor/start outputs are schema-versioned JSON.
19. PASS - runtime lifecycle reports zero LLM calls and has no provider SDK dependency.
20. PASS - exact commit/toolchain/host/lockfile/test/benchmark evidence is recorded in this report and preflight report.
21. PENDING - independent governed review approval is intentionally not self-issued; no merge, promotion or checkpoint closeout was performed.

## Residual risks and next governed gates

- Install/run the approved advisory, license and SBOM tooling in CI.
- Run hosted Windows/Unix IPC, cargo-fuzz and deep failure-injection/soak tiers.
- Perform independent exact-head review and record the verdict before any merge or checkpoint promotion.
