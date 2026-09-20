# CORE Definition of Done

Status: `M01_COMPLETE_PRODUCT_DISCOVERY_ACTIVE`

## Bootstrap DoD

Bootstrap is complete only when:

- required Project Brain files exist at HIVE v1.0.0 compatible paths;
- GEF v1.0.0 adoption/profile/policy/execution/review/evidence artifacts exist;
- the source hierarchy names canonical versus derived authority clearly;
- deterministic governance validation passes on the exact candidate head;
- GitHub PR and Work Order scaffolding exists;
- HIVE bootstrap tooling can health-check, register or resolve, inspect, index and retrieval-sync CORE without embedding secrets;
- bootstrap PR receives governed review;
- checkpoint is promoted only with accepted exact-head evidence.

## Product DoD

`PENDING DISCOVERY`

No product-completion percentage or production-readiness claim may be derived from the bootstrap DoD.


## M01 - Core Runtime & Lifecycle DoD

M01 completion requires the accepted Rust/Tokio runtime, versioned contracts, deterministic identity, typed config, module/capability registries, runtime journal, local IPC, health/degradation engine, lifecycle/supervisor and CLI to satisfy the module-specific DoD in `docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md`.

No M01 completion claim is valid without exact-head tests, security/supply-chain evidence, failure-injection/fuzz/property coverage, compatible performance-regression evidence, zero-LLM lifecycle proof and independent governed review. HIGH/CRITICAL unresolved defects block completion.


## M01 completion evidence
M01 satisfied its module DoD after the final corrective cycle. Review 011 recorded APPROVED at exact head `aac0f143ea576a11013e4346076b8b3b4bd24282`; workflow run #54 `35488781894` passed Governance, Ubuntu, Windows, fuzz, supply-chain/advisory, SBOM, soak and PRB/WNF; PR #17 was promoted as merge `d70b4296afbba93e8849ab6160e9b1caf5281e7d`. No unresolved HIGH/CRITICAL finding remained at promotion.
