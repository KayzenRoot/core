# CORE-D-205 — Standalone canonical source and M04 authority cutover

Date: 2026-09-29
Status: ADOPTED_ON_PROTECTED_MAIN_PROMOTION_ONLY
Work Order: #180; parent #172.

## Context
PRs #173/#176/#179 removed live LegacyProvider bootstrap and M01/M02/M03 LegacyProvider-specific coupling. Protected-main baseline 1e118f9763f90e9a1823786f9e399c55efcf3ef4 passed independent full 11/11 CI #36585621992. Nine M04 source files and old ACTIVE lock still carried historical pre-cutover architecture; external previous-V1 binary/API/journal consumers are UNKNOWN (#111).

## Decision
Exact tracked Git and local CORE-owned M01/M02/M03 V2 evidence are canonical. No LEGACY_PROVIDER install, MCP/API, Docker, registry, retrieval or context prerequisite. Optional generic advisory provider requires separate proof and never grants Git/path authority. Supersede only conflicting LEGACY_PROVIDER-dependent operative clauses in historic D-001/003/004/005/009/010/200/202 and old M04 admission. Preserve historical accepted evidence, unrelated GEF/security decisions, frozen M04 state-machine semantics and unadmitted D-204 replay proposal. Rebind new nine-source Git fingerprints to explicitly STALE M04 lock, disallow product execution, fail-close Work Order/evidence/GEF, and require separate standalone M04 compatibility/source review and NEW exact-source admission. Do not infer absence of previous V1 consumers or resume stale #106/#118.

## Review and stopping conditions
All nine SHA and Work Order/Context Lock/evidence bindings MUST match one exact candidate; negative authority regressions MUST fail. Exact-head full 11/11 Ubuntu/Windows, bounded fuzz, supply-chain/SBOM, PRB and soak, scoped owner audit NOT INDEPENDENT, protected guarded squash and independent new-main full 11/11 required before closure. No M04 code, no silent contract changes, no old-history rewrite and no extra module planning in this increment.
