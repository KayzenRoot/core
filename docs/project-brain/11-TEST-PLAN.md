# CORE Test & Evidence Plan

Status: `PRODUCT_DISCOVERY_ACTIVE`

## Bootstrap proof ladder
1. Python syntax compilation for governance/HIVE bootstrap tooling.
2. Deterministic source/governance validation.
3. Unit tests for HIVE project identity resolution and fail-closed collision handling.
4. Hosted GitHub Actions evidence on the exact candidate head.
5. Semantic exact-head audit against Work Order, Scope, Architecture, Requirements and DoD.

## Current required bootstrap commands
```text
python -m py_compile scripts/validate_governance.py scripts/hive_bootstrap.py
python scripts/validate_governance.py
python -m unittest discover -s tests -p "test_*.py" -v
```

## Product validation
Product validation is frozen progressively by module and risk. M01 and M02 have promoted exact-head completion evidence; M03 obligations are being frozen during deep planning, with Rounds 1-3 promoted and Round 4 next; M04+ unit, integration, E2E, security, performance, recovery, compatibility and benchmark obligations remain `PENDING_DISCOVERY` until their owning plans are accepted.

Historical green evidence never automatically proves a changed head.


## M02 Round 4 validation obligations

Before Round 4 mechanisms can be promoted into an implementation Work Order, evidence must cover:
- watcher-loss/overflow and rename-storm behavior;
- action-boundary freshness with watcher disabled;
- CIG selective invalidation versus full required-mask validation;
- cache hit/miss/bypass/invalidation provenance;
- timestamp-preserving content rewrite;
- cache corruption/loss safety;
- bounded/coalesced hashing under cancellation and large files;
- symlink/junction swap during hash/path proof;
- deterministic repository-graph serialization and delta/full equivalence;
- hardened system-Git GitInspector fixtures against the Git semantic reference contract/oracle;
- differential fixtures for any alternative GitInspector provider only if that provider is proposed for admission;
- hostile Git config, secret canaries, no-network/no-mutation behavior;
- Windows and Unix path/filesystem cases;
- cold/warm/no-change/small-delta/large-delta benchmarks;
- memory/concurrency ceilings under large status/hash workloads.

Exact numeric resource defaults remain blocked until reproducible M02 calibration evidence exists. The Round 5 system-Git baseline must satisfy its semantic/security/resource gates; promotion of any alternative provider remains blocked until provider-specific differential evidence exists. Historical M01 evidence does not substitute for M02-specific proof.


## M02 Round 5 contract/file-map validation

Planning freeze review must verify:
- every v1 contract has explicit schema/version and canonical semantic fields;
- contract serialization order is deterministic;
- BVM profile masks match Round 5 definitions;
- file/crate map creates no dependency cycle;
- core-workspace initial dependency set is limited to the approved graph;
- system-Git provider is behind a trait and cannot leak provider-specific payload into canonical GitEvidence;
- L1 cache/watchers are optimizations only;
- persistent cache/Rust-native provider/watcher framework remain outside initial Work Order;
- M02 DoD is traceable to requirements and the module plan.

Implementation acceptance later must execute the fixture/fuzz/benchmark matrix described in the M02 module plan.


## M02 Round 6 calibration protocol

The M02 implementation Work Order must include a mandatory calibration stage before final acceptance.

Evidence rules:
- deterministic synthetic/local fixtures only;
- no network;
- at least five measured iterations per relevant scenario after warm-up where applicable;
- cold and warm results reported separately;
- semantic assertions run for every candidate resource profile;
- cancellation/overflow/failure cases included;
- unsupported top-scale fixture recorded as SKIPPED_RESOURCE_BOUND rather than inferred;
- selected values and rejected candidate values are both recorded;
- exact commands and environment versions are retained.

Required calibration dimensions are defined in the M02 module plan. The complete suite must be rerun after the bounded Calibration Delta establishes final numeric defaults.

DWS must prove delta/full recomputation equivalence. WMF is excluded from V0.0 benchmark/acceptance requirements unless separately admitted.


## M03 Round 1 validation obligations

Round 1 planning/review must prove the architecture is internally coherent before any product implementation is authorized.

Planning validation must cover:
- M03/M04 ownership separation;
- deterministic WorkOrderId/revision/fingerprint model;
- canonical source/provenance manifest semantics;
- scope envelope and forbidden-delta behavior;
- immutable revision/supersession semantics;
- stable packet identity/order semantics;
- acceptance criterion to evidence requirement coverage;
- Context Lock + M02 basis binding;
- HIVE advisory boundary;
- zero-LLM compiler/validator direction;
- compact context/token-economy semantics;
- initial threat/failure taxonomy.

Later implementation evidence MUST include at minimum:
- canonicalization determinism/property tests;
- semantically equivalent request permutations;
- invalid/unsupported schema rejection;
- source-fingerprint substitution/stale Context Lock tests;
- superseded revision rejection;
- packet cycle/order tests;
- scope-delta classifier adversarial fixtures;
- acceptance/evidence gap detection;
- secret redaction/provenance tests;
- bounded serialization/cardinality/resource tests;
- Windows + Ubuntu exact-head CI;
- fuzz/property testing for parser/canonicalizer/delta classifier;
- benchmarks for compile/diff/admission latency and serialized/context manifest size.

Exact file map, dependency graph, fuzz targets and numeric budgets remain pending later M03 rounds.


## M03 Round 2 validation obligations

Planning review must verify:
- v1 envelope/version rejection semantics;
- immutable revision law;
- semantic-vs-diagnostic fingerprint field classification;
- authoring/frozen/admission/runtime contract separation;
- packet DAG acyclicity and deterministic topological ordering;
- packet scope intersection;
- fresh M02 basis admission with UNKNOWN fail-closed;
- Context Lock stale/mismatch rejection;
- external governance proof binding and replay resistance;
- READY receipt non-evergreen semantics;
- ExecutionCorrectionProposal versus WorkOrderRevisionDiff behavior;
- deny-over-allow scope rules;
- AEG criterion/evidence completeness;
- machine-readable StopCondition requirement;
- context budget no-truncation law;
- WSF hints as non-authoritative invalidation input;
- secret-safe durable payload requirements.

Later implementation fixtures MUST include:
- equivalent semantic request permutations -> identical fingerprints;
- diagnostic-only metadata differences -> unchanged semantic fingerprint;
- schema/version downgrade/replay cases;
- packet-cycle and dangling dependency rejection;
- child packet scope widening rejection;
- stale M02 generation / incompatible WorkspaceId cases;
- stale Context Lock / wrong WorkOrder fingerprint cases;
- governance proof bound to wrong revision/head cases;
- admission receipt replay after source/workspace/policy change;
- same-revision allowed correction fixtures;
- forbidden dependency/scope/architecture/security correction fixtures;
- acceptance criterion missing required evidence edge;
- context-budget mandatory-source truncation attempt;
- secret canary in provenance/context payload;
- bounded large packet/source/criterion graphs.


## M03 Round 3 validation obligations

Planning review must verify:
- compiler/service operations contain no hidden I/O authority;
- stateless/no-internal-database disposition;
- deterministic logical WorkOrderId behavior;
- LineageSnapshot/LPC CAS semantics;
- canonical revision numbering/conflict handling;
- PacketContextPlan/PCM lossless reconstruction;
- compile memo correctness independence;
- resource dimension coverage;
- error category/reason/retryability mapping;
- no partial FROZEN/READY result on resource failure;
- one-crate/dependency direction has no M02 reverse cycle.

Later implementation evidence MUST include:
- two concurrent N+1 candidates from same lineage -> only one canonical persistence under LPC;
- stale lineage generation -> LINEAGE_CONFLICT;
- deterministic compile replay;
- compile cache hit/miss equivalence;
- PCM packet reconstruction equivalence;
- request/source/packet/criteria cardinality limit cases;
- compile/validate/diff/admission deadline/resource failures;
- no hidden network/Git/HIVE/process invocation from core-work-order;
- safe diagnostics secret canaries;
- fuzz/property tests for canonicalizer, packet DAG, delta classifier, lineage/LPC and admission inputs;
- benchmark scaling for sources/packets/edges/criteria/lineage/context/diffs.


## M03 Round 4 planning and implementation validation

Round 4 planning review checks the M03 contract, adapter and file/dependency map against the acceptance criteria. The planning review does not claim that future product tests or measurements have run.

The future implementation evidence must map each blocking acceptance criterion to an exact evidence obligation. At minimum:

| Area | Required exact-head proof |
| --- | --- |
| Contracts/schema | Public V1 envelope round-trip, unsupported schema/version/kind rejection, typed ID validation and no downgrade. |
| Fingerprints | Golden vectors; semantic-versus-diagnostic field projection; equivalent permutations and map-order independence; stable WorkOrderId/revision behavior. |
| Pure operations | Public API tests for parse, compile, validate, diff, classify, admission and handoff; replay-equivalent inputs yield equal semantic outputs. |
| Source/adapters | Resolver outputs bind requested source identity to observed fingerprint, authority, freshness and provenance; substitution/staleness/UNKNOWN do not become current. |
| M02 / Context Lock / GEF | Wrong schema, WorkspaceId/generation/basis/profile, lock fingerprint/base/source-set, governance verdict/head/scope/policy and replay fixtures all fail closed. |
| Packet/scope | Permuted valid DAGs produce one order; cycle/dangling/budget failures are typed; deny precedence and child-scope intersection prevent widening. |
| AEG/PCM | Missing blocking evidence, dangling edges and silent mandatory-source loss fail; reconstructed packet source sets equal independent requirements. |
| Lineage/correction | Stale LPC and competing N+1 CAS behavior; semantic deltas require a new revision; same-revision changes stay inside CorrectionPolicy. |
| Atomic resources | At-limit, over-limit and deadline cases return typed results; failure never returns partial FrozenWorkOrder, READY receipt or handoff. |
| Security | Secret canaries, bounded diagnostics, no hidden I/O, no secret-bearing durable fields, schema downgrade, stale receipt and authority-changing retry attempts. |

### M03 Round 4 property and fuzz targets

The implementation must cover the law matrix in M03-WORK-ORDER-ENGINE.md using deterministic generated property cases in the standard Rust test harness. Round 4 adds no property-testing dependency. Required laws include canonical permutation/replay, schema/version rejection, packet DAG order/acyclicity, scope non-widening, AEG completeness, PCM reconstruction, lineage/LPC compare-and-set, semantic-diff/correction classes, source substitution/staleness, Context Lock and governance replay, admission non-evergreen behavior, finite resource failure, no partial output and diagnostic redaction.

The existing separate fuzz package already uses libfuzzer-sys. Add bounded M03 fuzz targets for:

1. arbitrary V1 envelope/parser/canonicalizer input;
2. packet DAG identifiers and edge sets;
3. scope rules and semantic/correction deltas;
4. lineage snapshots and LPC preconditions;
5. source fingerprints, authority/provenance and freshness states;
6. workspace/lock/governance admission and receipt replay;
7. hostile diagnostic fields and secret canaries.

Fuzz targets operate only on in-memory bounded inputs, have no network/filesystem/process access, and fail on panic, hang, memory amplification, partial contract output or secret echo. Seeds are synthetic and contain no repository data.

### M03 Round 4 benchmark and resource calibration

No current M03 benchmark has run, so Round 4 reports no measured latency, memory or scale value. The later calibration gate varies source count and source bytes; packet count, DAG width/depth and edges; scope rules and semantic delta fields; criteria, evidence and AEG edges; shared context references and PCM reconstruction; lineage edges/revision diffs; admission source/workspace/lock/governance/policy inputs; and canonical output size.

Use deterministic local synthetic fixtures and assert semantics for every candidate. Run on Windows and Ubuntu, warm once, collect at least five measured iterations per scenario, and report median/min/max, exact commands, source SHA, OS, toolchain, CPU and fixture generator/version. Exercise candidate limits and cap-plus-one failures for every security-sensitive dimension. Cold/warm/cache equivalence applies only if an optional derived compile memo is separately admitted; the baseline is uncached.

Finite positive M03ResourceBudget defaults and any acceptance thresholds remain calibration-gated until implementation evidence exists. Unsupported scales are marked skipped/unsupported rather than extrapolated. Calibration failure, timeout, overflow or absent required scenario blocks production acceptance; no measurements or defaults may be invented in a planning document.

### M03 V0.0 production evidence gates

Production acceptance requires one exact final head with Windows and Ubuntu CI, contract/property/integration/adversarial/fuzz coverage, M03 calibration report, finite budget selection rationale, no-hidden-I/O and acyclic dependency proof, security/advisory/license/SBOM checks, deterministic serialization evidence, complete AEG-to-evidence traceability and independent exact-head review with no unresolved HIGH/CRITICAL finding. Any later commit invalidates the head-bound results and requires the applicable suite to rerun.
