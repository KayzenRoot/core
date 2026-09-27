# CORE-M05-DISCOVERY-R2: source-backed transport/handshake review candidate

Status: NON_AUTHORITATIVE_R2_CANDIDATE / NO_IMPLEMENTATION
Work Order: https://github.com/KayzenRoot/core/issues/136
Exact initial protected-main base: eb231f57e8dffff0db811cf4b87c645e7022af59

## Actual inspected Git sources

- Promoted Round 1 plan: `docs/modules/M05-HOST-ADAPTER-FABRIC.md`, Git blob `0e287f3e2b3ac8cb4ee70092b2a335a3db11b43f`, after [PR #134](https://github.com/KayzenRoot/core/pull/134), [owner audit #135](https://github.com/KayzenRoot/core/issues/135) NOT INDEPENDENT, and full main-push [CI #36339748038](https://github.com/KayzenRoot/core/actions/runs/36339748038) 11/11 PASS.
- M01 actual `core-ipc` frame/handshake/Unix-socket/Windows-pipe Rust source: `crates/core-ipc/src/lib.rs`, Git blob `ff9e63800811f627ea02057daf122cfae6056db8`.
- M01 actual ModuleRegistry, CapabilityRegistry, provider health/quarantine and generation-aware lease validation: `crates/core-registry/src/lib.rs`, Git blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14`.
- M01 actual core DTOs (`RuntimeGeneration`, `CapabilityProviderDescriptor`, `CapabilityLease`, `ModuleManifest`): `crates/core-contracts/src/lib.rs`, Git blob `0cd1d56a4ac761ff1c8b761cde0e0d19dfa76bc9`.
- Round 1's original canonical M01 and M04 module planning/source hierarchy/Architecture/Requirements/Security fingerprints remain in `.engineering/evidence/CORE-M05-DISCOVERY-R1.md`.

## Review-only decisions and STOP

This PR appends an M05 Round 2 **candidate** distinguishing real M01 `CR1` framing and `negotiate` (same major+epoch, min minor/frame) from external endpoint authentication/feature-policy/lease authority. It identifies M01's current bounded read-body-before-full-header-verification sequence as a future hardening question, NOT a demonstrated exploit or authorization to change the M01 crate under this Work Order. External process, MCP/stdio/remote protocol admission and numeric budgets are UNFROZEN. No M05 production Rust, host integration, local device benchmark, M04 journal contract or HIVE runtime evidence is created.

M04 external previous-V1 durable consumers remain UNKNOWN under [issue #111](https://github.com/KayzenRoot/core/issues/111); source [PR #118](https://github.com/KayzenRoot/core/pull/118) is draft, product [PR #106](https://github.com/KayzenRoot/core/pull/106) unmerged, all 23 global EV-M04 pending. Real local HIVE/Codex proof [#4](https://github.com/KayzenRoot/core/issues/4) remains pending.

Require actual fresh exact-head CI status classification and scoped owner self-audit NOT INDEPENDENT, then protected squash merge and full main-push validation before closing the R2 discovery Work Order. No canonical checkpoint or ACTIVE M04 lock is modified.
