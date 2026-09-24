# M04 — Run / Attempt / Step Engine

Status: `ROUNDS_1_3_PROMOTED_ROUND_4_PLANNING_GATE`
Implementation: `UNAUTHORIZED`
Assurance: `ELEVATED`

## Mission

M04 owns the deterministic execution-state model that turns one current, admitted M03 Work Order handoff into bounded Run, Attempt and Step lifecycle state. It records orchestration truth; it does not execute host commands, mutate repositories, choose models, grant capabilities or perform verification owned by later modules.

## Authority boundary

Inputs:
- M03 READY admission/handoff plus exact WorkOrderId/revision/fingerprint and required freshness bindings;
- M01 runtime/lifecycle cancellation and shutdown signals;
- explicit caller policy/evidence snapshots only.

M04 owns:
- RunId, AttemptId and StepId identities;
- lifecycle state machines and legal transitions;
- parent/child lineage and deterministic sequence/order;
- terminal-state semantics;
- bounded retry-attempt accounting as state, not retry policy;
- interruption/cancellation recording;
- resumable execution cursor/checkpoint contracts;
- deterministic event journal projection and replay validation.

M04 does not own:
- host/provider execution (M05+);
- capability negotiation/leases (M06/M11);
- specialist/model selection (M07-M09);
- execution policy decisions (M10);
- command/tool execution (M12);
- mutations (M13);
- verification/evidence/review decisions (M14-M17);
- recovery strategy (M18);
- quota/cost policy (M19);
- Git/GitHub/release actions (M20-M21);
- security-policy engines (M22);
- HIVE federation intelligence (M23);
- telemetry transport/observability spine (M24).

## Round 1 frozen direction

1. A Run can start only from a current M03 READY handoff whose recorded bindings are revalidated at the start boundary.
2. Run, Attempt and Step identities are immutable, typed and domain-separated.
3. State transitions are explicit, closed and fail typed on illegal/ambiguous transitions.
4. Attempts are append-only children of a Run. A retry creates a new Attempt; it never rewrites prior history.
5. Steps are append-only execution-state records within an Attempt. Later execution modules may attach outcomes/evidence through versioned references.
6. M04 records retry/correction eligibility facts but does not decide model/host/policy-specific retry strategy.
7. Cancellation is monotonic. Once cancellation is observed/accepted, no new child work may be admitted except bounded closeout/finalization transitions.
8. Terminal states are immutable historical facts. Resume creates a new continuation epoch/attempt rather than mutating terminal history.
9. Replay from canonical M04 events must reconstruct the same semantic state or fail closed.
10. Event ordering is explicit and deterministic; timestamps may be diagnostic metadata but never the sole semantic ordering authority.
11. Partial writes cannot expose a valid advanced state. Transition + journal/checkpoint publication requires atomic semantic commit at the storage boundary.
12. Core state-machine semantics remain zero-LLM and do not perform hidden filesystem/network/process/database/HIVE/GitHub I/O.
13. Persistent backend selection is not frozen in Round 1. Contracts must remain backend-neutral.
14. M04 consumes M03 contracts but M03 must never depend on M04.
15. UNKNOWN freshness, lineage, transition or replay evidence broadens to BLOCKED/STALE, never optimistic continuation.

## Candidate technologies

### RAS — Run Attempt Spine
Canonical typed hierarchy binding Run -> Attempt -> Step identities and immutable lineage.

### TLG — Transition Law Graph
Closed transition table with machine-checkable preconditions, terminal invariants and deterministic typed rejection.

### CER — Causal Execution Revision
Monotonic semantic sequence/epoch counters replace wall-clock ordering as execution truth.

### RJR — Replay Journal Root
Domain-separated rolling fingerprint over canonical state events, enabling corruption/reordering/truncation detection without making telemetry the authority.

### BRC — Boundary Revalidation Capsule
Compact proof captured at Run start and continuation boundaries tying M03 READY receipt, workspace basis, Context Lock/governance generation and execution epoch.

### ICF — Interruption Continuity Frame
Versioned continuation cursor that records the last semantically committed boundary without claiming M18 recovery strategy.

### ASF — Atomic State Fence
Storage-adapter contract requiring compare-and-set generation plus transition/journal publication as one semantic commit, preventing split-brain advancement.

## Initial lifecycle model

Run candidate states:
`CREATED -> ADMITTED -> ACTIVE -> {SUCCEEDED, FAILED, CANCELLED, BLOCKED, INTERRUPTED}`

Attempt candidate states:
`CREATED -> ACTIVE -> {SUCCEEDED, FAILED, CANCELLED, BLOCKED, INTERRUPTED}`

Step candidate states:
`DECLARED -> READY -> ACTIVE -> {SUCCEEDED, FAILED, CANCELLED, BLOCKED, SKIPPED, INTERRUPTED}`

Exact transition preconditions, SKIPPED semantics, closeout transitions and whether CREATED is durable are intentionally left for Round 2.

## Failure/threat model

Round 2+ must explicitly address:
- duplicate transition submission;
- stale compare-and-set generation;
- replayed or reordered events;
- journal truncation/corruption;
- concurrent attempt creation;
- cancellation racing child admission;
- stale M03 admission receipt;
- workspace/context drift between Run start and continuation;
- process crash between state transition and publication;
- oversized/deep histories and resource exhaustion;
- malicious external outcome payloads;
- secret leakage through diagnostics;
- cross-run/cross-attempt identity substitution.

## Evidence direction

Future freeze must require:
- transition-table exhaustive tests;
- illegal-transition/property tests;
- replay equivalence and corruption tests;
- concurrency/CAS race tests;
- cancellation-race tests;
- M03 handoff freshness/substitution tests;
- bounded-history/resource tests;
- deterministic canonicalization/fingerprint vectors;
- fuzz targets for transition/event/replay/identity/cursor surfaces;
- Windows + Ubuntu exact-head CI;
- no-hidden-I/O and zero-LLM proof;
- supply-chain/SBOM evidence;
- independent exact-head review with no unresolved HIGH/CRITICAL.

## Round 2 frozen semantics

### Durable lifecycle

`CREATED` is durable for Run and Attempt. `DECLARED` is durable for Step. No externally visible identity may exist without its initial durable event.

Run legal transitions:
- `CREATED -> ADMITTED | BLOCKED | CANCELLED`
- `ADMITTED -> ACTIVE | BLOCKED | CANCELLED | INTERRUPTED`
- `ACTIVE -> SUCCEEDED | FAILED | BLOCKED | CANCELLED | INTERRUPTED`

Attempt legal transitions:
- `CREATED -> ACTIVE | BLOCKED | CANCELLED`
- `ACTIVE -> SUCCEEDED | FAILED | BLOCKED | CANCELLED | INTERRUPTED`

Step legal transitions:
- `DECLARED -> READY | BLOCKED | CANCELLED | SKIPPED`
- `READY -> ACTIVE | BLOCKED | CANCELLED | SKIPPED`
- `ACTIVE -> SUCCEEDED | FAILED | BLOCKED | CANCELLED | INTERRUPTED`

All terminal states are immutable. There is no terminal-to-active transition. A continuation after `INTERRUPTED` creates a new Attempt/continuation epoch. `SKIPPED` is legal only before ACTIVE and requires an explicit non-execution reason code plus authority reference; it is never inferred from absence of execution.

### Parent/child closure laws

- Run `SUCCEEDED` requires every required Step in its selected completion projection to be `SUCCEEDED` or explicitly `SKIPPED` under an allowed authority.
- Run/Attempt terminalization prohibits creation of new descendants.
- Attempt `SUCCEEDED` requires all required Steps in that Attempt to satisfy their declared completion disposition.
- Child failure does not automatically choose parent retry/recovery policy; it exposes typed facts for later policy owners.
- A child can never be semantically newer than its parent generation fence.

### Event envelope

Every durable event carries: schema version, domain tag, RunId, optional AttemptId/StepId as required by event type, event sequence, expected prior generation, resulting generation, idempotency key, transition kind, canonical payload fingerprint, prior journal root and resulting journal root. Diagnostic timestamps are optional/non-authoritative.

Projection accepts only a contiguous event sequence beginning at the declared journal origin. Missing, reordered, cross-lineage, wrong-domain, wrong-generation or root-mismatched events fail closed.

### Idempotency

Idempotency key scope is `(RunId, operation-domain, caller-key)`. Repeating the exact semantic operation with the same key and identical canonical request fingerprint returns the recorded result without creating another event. Reusing the key with a different semantic fingerprint is `IDEMPOTENCY_CONFLICT`. Duplicate transport delivery is therefore harmless; semantic duplication is explicit.

### CER / generation and CAS laws

- Each semantic commit advances the Run generation exactly by one.
- Expected generation must equal the durable current generation.
- A stale or future generation returns typed `GENERATION_CONFLICT`; no partial mutation/event is valid.
- Attempt/Step local sequence numbers are monotonic within their typed parent but the Run generation is the serialization fence.
- No wall-clock timestamp resolves concurrent writers.
- ASF requires state projection, event append, journal-root update and generation advance to become visible atomically.

### Cancellation precedence

Cancellation has a monotonic Run cancellation epoch. Once a cancellation request is durably accepted at generation G, any child-admission/activation operation whose expected generation is G or later must reject unless it is an explicitly bounded closeout operation. A concurrently committed child operation strictly before the cancellation generation remains historical fact and is then driven toward cancellation/terminal closeout. Cancellation never erases a previously committed success/failure.

### Ordering

Attempt ordinal is a zero-based monotonic integer unique within Run and never reused. Step ordinal is a zero-based monotonic integer unique within Attempt and never reused. Stable semantic order is `RunId / AttemptOrdinal / StepOrdinal / EventSequence`, with typed IDs used for identity and ordinals used only for bounded ordering.

### ICF continuation cursor

An ICF contains schema/domain, RunId, source AttemptId, last committed Run generation, last event sequence, journal root, M03 BRC fingerprint, execution epoch and canonical cursor fingerprint. It contains no secret material and no host-specific recovery instruction. Resume validates all bindings and creates a new Attempt/epoch; mismatch or UNKNOWN yields `STALE_CONTINUATION`/BLOCKED.

### BRC compatibility

At Run admission and continuation, BRC must bind the exact M03 WorkOrderId, revision/fingerprint, workspace identity/basis, Context Lock fingerprint, governance/source generation and admission receipt fingerprint. Exact semantic equality is required unless a future explicitly versioned compatibility rule is frozen. Any changed or unverifiable binding is STALE and prevents activation.

### Resource bounds

The contract exposes explicit finite limits for attempts per Run, steps per Attempt, durable events per Run, canonical payload bytes per event, diagnostic bytes, continuation-cursor bytes and replay depth. Limit values are configuration/policy inputs and must be calibrated before execution freeze. Cap+1 fails typed without partial state advancement. Histories are never silently truncated to satisfy a cap.

### Durable versus derived

Durable: typed identities/ordinals, lifecycle events, generations, idempotency records, canonical payload fingerprints, journal roots, BRCs, ICFs, terminal reason codes and versioned external evidence references.

Derived: current projected state, counts, completion summaries, latest-child pointers and diagnostic views. Derived data is rebuildable from the bounded canonical journal and cannot supersede it.

### Round 2 threat closure

Round 2 explicitly closes duplicate delivery, stale/future CAS, event reorder/truncation/substitution, concurrent attempt creation, cancellation/admission races, stale M03 authority, continuation drift, split publication, unbounded history and cross-lineage identity substitution through TLG/CER/RJR/BRC/ICF/ASF invariants. Secret-redaction and malicious external payload validation remain mandatory evidence concerns.

## Round 3 frozen contract and evidence design

### Public V1 contract surface

The implementation freeze must expose versioned, deterministic contracts equivalent to:
- `RunAdmissionRequestV1 -> Result<RunAdmissionReceiptV1, M04ErrorV1>`
- `CreateAttemptRequestV1 -> Result<AttemptReceiptV1, M04ErrorV1>`
- `DeclareStepRequestV1 -> Result<StepReceiptV1, M04ErrorV1>`
- `TransitionRequestV1 -> Result<TransitionReceiptV1, M04ErrorV1>`
- `CancelRunRequestV1 -> Result<CancellationReceiptV1, M04ErrorV1>`
- `CreateContinuationRequestV1 -> Result<ContinuationReceiptV1, M04ErrorV1>`
- `ReplayRequestV1 -> Result<ReplayProjectionV1, M04ErrorV1>`
- `AttachReferenceRequestV1 -> Result<ReferenceReceiptV1, M04ErrorV1>`

Requests carry explicit expected generation and idempotency data where they mutate semantic state. Receipts identify the committed generation/event/root. No API accepts ambient repository, process, network or clock authority.

### Typed identity and envelope surface

V1 requires domain-separated `RunId`, `AttemptId`, `StepId`, `EventId`, `IdempotencyKey`, `JournalRoot`, `CanonicalFingerprint`, `ExecutionEpoch` and `RunGeneration`. Raw strings cannot substitute across domains.

All serialized V1 envelopes include schema version and kind. Unknown versions/kinds fail typed and never downgrade silently.

### Error taxonomy

Stable top-level classes:
- `INVALID_INPUT`
- `INVALID_TRANSITION`
- `LINEAGE_MISMATCH`
- `GENERATION_CONFLICT`
- `IDEMPOTENCY_CONFLICT`
- `STALE_AUTHORITY`
- `STALE_CONTINUATION`
- `REPLAY_INTEGRITY_FAILURE`
- `RESOURCE_LIMIT_EXCEEDED`
- `CANCELLED`
- `BLOCKED`
- `UNSUPPORTED_VERSION`
- `EXTERNAL_REFERENCE_REJECTED`
- `STORAGE_CONFLICT`
- `INTERNAL_INVARIANT_VIOLATION`

Errors are machine-readable, carry bounded diagnostics and retryability classification, but retryability is a fact, not M04 retry policy.

### Canonical serialization and fingerprints

Semantic fingerprints use a versioned canonical byte projection with explicit field order, length-prefixing for variable-width values, normalized enum discriminants, no map iteration order, no locale formatting, no diagnostic timestamps and no secret-bearing fields.

Domain separators are distinct for IDs, requests, events, journal chaining, BRC and ICF. Fingerprint inputs are schema-bound. Cross-domain reuse of identical bytes must produce different semantic fingerprints.

Hash algorithm selection remains dependency-admission work for the final freeze, but V1 requires a collision-resistant cryptographic digest with deterministic cross-platform vectors.

### Exact event-kind registry

V1 event kinds are closed:
`RUN_CREATED`, `RUN_ADMITTED`, `RUN_TRANSITIONED`, `RUN_CANCELLATION_ACCEPTED`,
`ATTEMPT_CREATED`, `ATTEMPT_TRANSITIONED`,
`STEP_DECLARED`, `STEP_TRANSITIONED`,
`CONTINUATION_CREATED`, `REFERENCE_ATTACHED`.

No generic arbitrary event kind is accepted. Each kind has a schema-specific canonical payload and legal parent/transition preconditions.

### Reason-code registry

Terminal/block/skip/interruption records use versioned reason-code namespaces. V1 reserves domain classes for caller cancellation, runtime shutdown, stale authority, invalid lineage, resource cap, external reference rejection, execution outcome, explicit authorized skip and invariant protection. Free-form diagnostics may supplement but never replace a machine reason.

### Projection, snapshots and compaction

The canonical journal remains authoritative. A snapshot is a derived acceleration artifact binding RunId, source generation, last event sequence and journal root plus a canonical projection fingerprint.

Loading a snapshot requires verification against the journal boundary. A snapshot cannot authorize state absent from the journal. Compaction may discard only independently reproducible derived/cache material; V1 does not permit destructive deletion of canonical events needed to prove the active bounded Run history. Archive/retention policy belongs outside M04.

### External outcome/evidence references

M04 stores only versioned bounded references and semantic attachment facts, not the external artifact body. Each reference declares owner module/domain, reference kind, immutable locator/identity, content fingerprint when available, producing Run/Attempt/Step lineage and schema version. M04 validates shape, lineage and bounds, not the truth of M14-M17 verification outcomes.

### Bounded configuration and calibration

`M04ResourceLimitsV1` contains finite positive caps for attempts/run, steps/attempt, events/run, event canonical bytes, diagnostic bytes, cursor bytes, reference count/bytes and replay depth. Zero/unbounded sentinels are forbidden for production admission.

Exact numeric defaults are not fabricated during planning. The eventual Work Order must include a Resource Calibration Gate producing evidence-backed finite defaults under representative and adversarial fixtures. Any calibration delta may change numeric limits only, never semantics/contracts/authority/dependencies.

### Adapter seams and dependency direction

M04 core depends only on admitted lower-level contracts required from M01/M03 and shared primitive utilities explicitly admitted at final freeze. M04 never calls M03 repository/HIVE/GitHub resolution itself; callers/adapters supply validated BRC inputs.

Storage is an external `M04StateStoreV1`-equivalent port implementing atomic ASF compare-and-set commit. No database/backend dependency is frozen here. External evidence/outcome integration is a reference-validation port, never a dependency from M14-M17 back into M04 core.

Dependency direction remains acyclic: M01/M03 -> M04 -> later consumers. Later modules may consume M04 contracts but M04 cannot import their policy/execution/verification implementations.

### Acceptance Evidence Graph candidate

Blocking evidence nodes for final M04 acceptance:
- EV-M04-001 public contract compile/API conformance;
- EV-M04-002 exhaustive legal transition matrix;
- EV-M04-003 illegal transition/property rejection;
- EV-M04-004 generation/CAS concurrency races;
- EV-M04-005 idempotency replay/conflict laws;
- EV-M04-006 cancellation precedence races;
- EV-M04-007 journal replay equivalence;
- EV-M04-008 reorder/truncation/substitution/root corruption rejection;
- EV-M04-009 BRC stale/substitution boundary tests;
- EV-M04-010 ICF continuation/epoch tests;
- EV-M04-011 typed identity cross-domain substitution tests;
- EV-M04-012 canonicalization/fingerprint golden vectors across Windows/Ubuntu;
- EV-M04-013 resource cap and cap+1 atomic rejection;
- EV-M04-014 snapshot verification/rebuild equivalence;
- EV-M04-015 external-reference bounds/lineage validation;
- EV-M04-016 no hidden filesystem/network/process/database/HIVE/GitHub I/O;
- EV-M04-017 zero-LLM core proof;
- EV-M04-018 fuzz campaigns for transition/event/replay/identity/cursor/reference surfaces;
- EV-M04-019 finite calibration report;
- EV-M04-020 dependency/supply-chain/SBOM evidence;
- EV-M04-021 exact-head Windows CI;
- EV-M04-022 exact-head Ubuntu CI;
- EV-M04-023 independent exact-head review with zero unresolved HIGH/CRITICAL.

All nodes are blocking unless a later frozen Work Order explicitly proves a node non-applicable without weakening a frozen requirement.

### Round 4 questions

Round 4 must freeze the implementation-addressable crate/file map, exact dependency admission, concrete Rust signatures/types, store/reference traits, property/fuzz target inventory, benchmark/calibration protocol, production DoD and disposition of all candidate technologies. It must also decide whether Round 4 is sufficient for final planning freeze or a separate final-freeze round is required.

## STOP CONDITION

Rounds 1-3 are promoted planning truth. M04 implementation remains unauthorized. The next legal increment is Round 4 implementation-addressable freeze design; do not create an execution Work Order, select a persistence backend, or authorize product execution.
