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

`PROGRESSIVE_MODULE_DISCOVERY`

No product-completion percentage or production-readiness claim may be derived from the bootstrap DoD.


## M01 - Core Runtime & Lifecycle DoD

M01 completion requires the accepted Rust/Tokio runtime, versioned contracts, deterministic identity, typed config, module/capability registries, runtime journal, local IPC, health/degradation engine, lifecycle/supervisor and CLI to satisfy the module-specific DoD in `docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md`.

No M01 completion claim is valid without exact-head tests, security/supply-chain evidence, failure-injection/fuzz/property coverage, compatible performance-regression evidence, zero-LLM lifecycle proof and independent governed review. HIGH/CRITICAL unresolved defects block completion.


## M01 completion evidence
M01 satisfied its module DoD after the final corrective cycle. Review 011 recorded APPROVED at exact head `aac0f143ea576a11013e4346076b8b3b4bd24282`; workflow run #54 `35488781894` passed Governance, Ubuntu, Windows, fuzz, supply-chain/advisory, SBOM, soak and PRB/WNF; PR #17 was promoted as merge `d70b4296afbba93e8849ab6160e9b1caf5281e7d`. No unresolved HIGH/CRITICAL finding remained at promotion.


## M02 - Project / Workspace Adapter DoD

M02 is complete only when the frozen M02 plan and Work Order prove all of the following on the exact candidate head:
- explicit attach/revalidate/detach lifecycle with no ambient-CWD authority;
- deterministic ProjectBinding/Workspace/Repository/Worktree identities;
- versioned v1 contract envelopes and canonical fingerprints using core-identity;
- typed non-transitive authority roots and adversarial PAF/FSC path proof;
- deterministic RepositoryGraphV1 across normal, bare, linked-worktree, nested-repo and submodule fixtures;
- hardened system-Git GitInspector baseline with no shell, no network, no prompt, no repair/mutation, bounded output/deadline and secret redaction;
- standalone plus HIVE project-association reconciliation without HIVE granting local authority;
- WorkspaceBasis/BasisDiff generations, stale-handle invalidation and BVM action-boundary freshness;
- EIS/CIG correctness with watcher disabled and with overflow/loss hints;
- PEC L1 proof reuse that is disposable, observable and never mtime-only correctness;
- BHC bounded streaming/coalesced hashing and cancellation cleanup;
- explicit finite WorkspaceResourceBudget defaults backed by the mandatory M02 Resource Calibration Gate and committed calibration evidence;
- zero-LLM proof for all M02 discovery/binding/revalidation paths;
- unit/integration/property/adversarial/fuzz coverage on Windows and Ubuntu;
- supply-chain/advisory/SBOM checks;
- M02 benchmark/regression evidence, including selected and rejected calibration candidates;
- executor Evidence Bundle and proposed Checkpoint Delta;
- independent governed exact-head review with no unresolved HIGH/CRITICAL finding.

Persistent L2 proof cache, Rust-native/hybrid GitInspector providers and OS watcher adapters are not required for M02 V0.0 completion unless later admitted by governed evidence.


### M02 calibration completion gate

M02 cannot satisfy its DoD until:
- the implementation reaches CALIBRATION_ONLY state under the frozen Work Order;
- the required fixture/benchmark matrix runs;
- `docs/evidence/M02-CALIBRATION-REPORT.md` records environment, commands, measurements, selected/rejected candidates and final numeric budgets;
- the authorized Calibration Delta is applied without architecture/dependency/contract mutation;
- the full test/security/supply-chain/benchmark suite reruns on the new exact head.

WMF is not required for M02 V0.0 DoD. DWS equivalence to full recomputation is required.
