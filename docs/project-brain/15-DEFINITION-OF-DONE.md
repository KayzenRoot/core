# CORE Definition of Done

Status: `M01_M02_COMPLETE_M03_DISCOVERY_ACTIVE`

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


## M03 - Work Order Engine DoD discovery baseline

M03 implementation is NOT authorized by Round 1.

The eventual M03 DoD must prove, on one governed exact head:
- versioned canonical Work Order contracts;
- deterministic compiler/fingerprint semantics;
- immutable revision and supersession safety;
- explicit source/provenance and M02 workspace-basis binding;
- explicit machine-readable scope envelope;
- correction-delta firewall;
- packet declarations separated from M04 runtime state;
- acceptance/evidence graph completeness;
- explicit stop conditions;
- Context Lock/governance proof admission;
- compact context budget manifests without mandatory-source omission;
- HIVE advisory/context enrichment without authority escalation;
- bounded resource/cardinality behavior;
- zero-LLM parser/compiler/admission path;
- property/adversarial/fuzz/Windows/Ubuntu/security/performance evidence;
- independent governed review with no unresolved HIGH/CRITICAL finding.

Exact M03 completion criteria remain `PENDING_DISCOVERY` until later rounds freeze contracts, file map, tests, benchmarks and Work Order.


### M03 Round 2 DoD refinements

The eventual M03 DoD additionally requires:
- explicit v1 envelope and contract-layer separation;
- immutable frozen revisions with semantic fingerprint law;
- bounded deterministic packet DAG;
- fresh M02 workspace admission rather than persisted live handles;
- Context Lock and external governance-proof compatibility checks;
- non-evergreen admission receipts with Run-start revalidation inputs;
- ScopeEnvelope deny precedence and dependency isolation;
- explicit same-revision CorrectionPolicy plus semantic revision diff;
- complete AEG and machine-readable StopCondition;
- no-silent-truncation context budget semantics;
- WSF fail-closed staleness behavior;
- secret-free durable contracts.

M03 remains discovery-only after Round 2.


### M03 Round 3 DoD refinements

Final M03 acceptance must additionally prove:
- stateless no-hidden-I/O compiler core;
- deterministic compile/validate/diff/classify/admit/handoff services;
- deterministic logical ID creation;
- external lineage snapshot + LPC race prevention;
- no internal database/persistence authority;
- canonical fingerprint-stack reuse;
- packet context mesh lossless reconstruction;
- DCR compiler provenance;
- explicit calibrated resource budgets;
- typed error/retryability behavior;
- dependency direction without M02 reverse cycle;
- no partial frozen/admitted result under budget/deadline failure.

Round 3 does not authorize implementation.


### M03 Round 4 production DoD direction

The future M03 V0.0 implementation is complete only when all of the following are proven together on one exact candidate head:

- all public V1 contracts, IDs, revision/fingerprint ownership, service inputs/outputs/errors and the frozen file/dependency map are implemented without scope drift;
- source, M02 workspace/basis, Context Lock, external governance and optional HIVE references follow the Round 4 adapter boundary and freshness/replay rules;
- canonical semantic projection, explicit collection sorting, stable serialization/fingerprint vectors and diagnostic exclusion pass deterministic permutation/replay tests;
- packet DAG, deny-overrides-allow scope intersection, AEG completeness, PCM reconstruction, correction classification and immutable revision behavior pass property and adversarial tests;
- external LineageSnapshot/LPC compare-and-set prevents stale or competing canonical revisions; M03 performs no persistence, hidden refresh, rebase, commit or promotion;
- parser, compile, validate, diff, correction, admission and handoff errors are typed and bounded; core resource failures return no partial FROZEN, READY or handoff output, and caller-owned wall-clock timeout discards any late result without ambient clock reads inside core-work-order;
- secret canaries and hostile adapter diagnostics prove durable contracts and errors are redacted and bounded;
- static/dependency evidence proves the compiler has no hidden filesystem, cwd, Git, HIVE, GitHub, network, process or database I/O and the dependency graph is acyclic and admitted;
- all finite positive resource defaults are supported by the mandatory committed M03 calibration report, with selected/rejected candidates, supported scales and no extrapolated or fabricated measurements;
- required unit, integration, deterministic property, adversarial, fuzz and benchmark/resource evidence passes on Windows and Ubuntu exact-head CI;
- advisory/license/supply-chain/SBOM evidence passes, the AEG covers every blocking DoD obligation, and an independent governed exact-head review finds no unresolved HIGH/CRITICAL defect.

M03 Round 4 is an independently reviewed and promoted planning baseline (Review 004 / Issue #63; PR #62; exact head `8fd3f085f93b342373b06e4471088dc7b843fac4`; workflow #147 / run `35805683331`; promotion merge `78daa752760ba19b3c36c7e2a7574bb3cfd03501`). Implementation remains unauthorized. The next gate is the M03 final planning freeze (Round 5 candidate), followed by its own review/promotion and a separate execution-admission delta.
