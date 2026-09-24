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
Product validation is frozen progressively by module and risk. M01 and M02 have promoted exact-head completion evidence; M03 Rounds 1-5 planning are promoted. CORE-M03-ADMIT-001 was approved and promoted as a bounded authorization-only change; it did not claim M03 product test, benchmark, calibration, or implementation results. M03 implementation has separate exact-head completion evidence. M04 planning and execution admission are promoted; M04 implementation validation obligations remain `PENDING_IMPLEMENTATION` until exact-head evidence is produced. M05+ obligations remain `PENDING_DISCOVERY` until their owning plans are accepted.

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
| Atomic resources | At-limit and over-limit core cases return typed results; failure never returns partial FrozenWorkOrder, READY receipt or handoff. A caller-owned deadline test must prove a timed-out/late service result is discarded and cannot be admitted. |
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

Finite positive M03ResourceBudget defaults and any acceptance thresholds remain calibration-gated until implementation evidence exists. Unsupported scales are marked skipped/unsupported rather than extrapolated. Calibration failure, caller-owned timeout, overflow or absent required scenario blocks production acceptance; no measurements or defaults may be invented in a planning document. Deadline tests must verify the core itself performs no ambient clock read and the caller discards any late result.

### M03 V0.0 production evidence gates

Production acceptance requires one exact final head with Windows and Ubuntu CI, contract/property/integration/adversarial/fuzz coverage, M03 calibration report, finite budget selection rationale, no-hidden-I/O and acyclic dependency proof, security/advisory/license/SBOM checks, deterministic serialization evidence, complete AEG-to-evidence traceability and independent exact-head review with no unresolved HIGH/CRITICAL finding. Any later commit invalidates the head-bound results and requires the applicable suite to rerun.


## M03 Round 5 final-freeze promoted / execution-admission validation status

CORE-M03-FREEZE-001 froze the future implementation Work Order, Context Lock form, Evidence Bundle skeleton, executor handoff, eight construction packets, rule-only Calibration Gate, and a 23-criterion machine-readable AEG. Review 006 / Issue #67 approved exact head `326eea936989ad2155ae6d1fb3fc965b8d1d25b9`; PR #66 was promoted as merge `ac90b1f48c5551e65ecadace95c59f7f0647062f` with workflow `35813591063` green in all seven required contexts. All implementation Evidence Requirements remain PENDING_IMPLEMENTATION. CORE-M03-ADMIT-001 must validate exact source/Work Order/lock fingerprints, canonical-main-only authorization, M03-only scope and unchanged architecture/dependency/acceptance/calibration/security semantics before promotion. The admission delta itself claims no product implementation success.


## M04 Rounds 1-3 planning and future implementation validation

Round 3 planning review must verify consistency among canonical Scope, Requirements, Architecture, Security, DoD, module plan, decisions and checkpoint/GEF state. Planning evidence does not claim product implementation tests have run.

Future implementation evidence must map the blocking EV-M04-001..023 graph to exact-head artifacts. Required proof includes:
- public V1 contract/API conformance and unsupported version/kind rejection;
- exhaustive legal transition matrices and illegal-transition properties;
- generation/CAS stale/future/conflicting writer races;
- fingerprint-bound idempotency exact replay and conflicting reuse;
- cancellation versus child-admission/activation race ordering;
- journal projection/replay equivalence and reorder/truncation/substitution/root-corruption rejection;
- BRC stale/substitution and ICF continuation/epoch fixtures;
- typed identity cross-domain substitution rejection;
- canonicalization/fingerprint golden vectors on Windows and Ubuntu;
- resource at-limit and cap+1 atomic rejection for every finite dimension;
- snapshot verification/rebuild equivalence without journal authority loss;
- external reference lineage/shape/bounds validation;
- static/runtime no-hidden-I/O and zero-LLM proof;
- bounded fuzz targets for transition, event, replay, identity, cursor and external-reference surfaces;
- reproducible finite resource calibration with selected/rejected candidates;
- advisory/license/supply-chain/SBOM evidence;
- exact-head Windows and Ubuntu CI;
- independent exact-head review with zero unresolved HIGH/CRITICAL findings.

### M04 future fuzz/property surfaces

At minimum, bounded generated/fuzz cases must exercise transition envelopes, typed identities, event sequencing/root chaining, CAS/idempotency, BRC/ICF payloads, snapshot boundaries and external-reference schemas. Fuzzing must remain in-memory, resource bounded and free of network/process/repository side effects.

### M04 future calibration protocol

Planning freezes dimensions, not guessed numeric values. The implementation Work Order must run deterministic representative/adversarial fixtures on supported platforms, record environment/toolchain/commands and measured results, choose finite positive defaults, document rejected candidates and rerun exact-head validation after the narrowly authorized numeric Calibration Delta. Unsupported scales are reported, not extrapolated.


## M04 Round 4 implementation-addressable validation plan

Planning review must verify the frozen crate/file/dependency map against the current workspace graph and confirm that no new production dependency or backend has been silently admitted.

Future implementation must provide exact-head evidence for:
- compilation/public API conformance of all frozen V1 types and service signatures;
- exhaustive TLG transition table and parent/terminal invariants;
- prepared-commit purity independent of store implementation;
- pending operation receipts cannot become authority before a matching durable commit receipt is successfully finalized;
- finalization rejects RunId/operation-fingerprint/generation/event-sequence/journal-root mismatch;
- store-port conformance with adversarial partial-write/conflict fakes;
- same-generation two-writer races with at most one successful durable commit;
- exact idempotency duplicate/conflict behavior;
- cancellation precedence under adversarial operation orderings;
- replay/projection equivalence across generated histories;
- root-chain corruption, reorder, truncation, substitution and cross-lineage rejection;
- BRC/ICF stale/UNKNOWN/mismatch fixtures;
- canonical binary-frame golden vectors and domain-separation vectors on Windows and Ubuntu;
- deterministic/caller-supplied ID behavior with no random/clock input;
- snapshot-boundary verification and journal rebuild equivalence;
- reference-adapter evidence shape/lineage/resource validation with secret canaries;
- at-limit/cap+1 behavior for every M04ResourceLimitsV1 dimension;
- static dependency/no-hidden-I/O/zero-LLM proof;
- six named bounded fuzz campaigns;
- supply-chain/advisory/license/SBOM gates;
- cross-platform exact-head CI;
- complete EV-M04-001..023 traceability and final independent review.

### M04 benchmark/calibration matrix

The frozen `m04_run_state` harness scales attempts/run, steps/attempt, events/run, same-generation concurrent preparation, event payload bytes, replay length, snapshot interval/rebuild, reference cardinality/bytes and continuation cursor bytes. Each supported scenario uses deterministic synthetic fixtures, one warm-up and at least five measured iterations per platform. Reports include exact source SHA, OS, Rust toolchain, CPU, fixture generator/version, command and median/min/max.

Candidate finite limits and cap+1 are exercised explicitly. Unsupported scales are reported as unsupported. Planning documents contain no claimed latency/memory/default values before these measurements exist.


## M04 Round 5 final-freeze validation contract

The future implementation must execute Packs A-H and satisfy AC-M04-001..023 exactly one-to-one with EV-M04-001..023.

Each AC record must identify:
- exact candidate head;
- test/property/fuzz/benchmark/static command or artifact;
- platform where relevant;
- result;
- evidence path/reference;
- whether a Calibration Delta affected the criterion;
- unresolved finding count.

Pack H must rerun every criterion affected by the selected numeric resource limits after the Calibration Delta. A pre-calibration green result cannot substitute for a required post-calibration exact-head result.

The final implementation PR may report only `READY_FOR_REVIEW` or `BLOCKED`. It may not report APPROVED. Independent exact-head review is AC-M04-023 and remains unsatisfied until performed by the reviewer after executor handoff.
