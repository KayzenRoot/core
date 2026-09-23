# M04 — Run / Attempt / Step Engine

Status: `ROUND_2_TRANSITION_SEMANTICS_CANDIDATE`
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

## Round 3 questions

Round 3 must freeze:
- public contract/type surface and error taxonomy;
- canonical serialization/domain separators and fingerprint inputs;
- exact event kinds and reason-code registry;
- projection/snapshot compaction rules without loss of journal authority;
- external outcome/evidence attachment contract;
- bounded configuration schema and calibration method;
- cross-module adapter seams and dependency direction;
- acceptance criteria/evidence graph for the eventual planning freeze.

## STOP CONDITION

Round 2 is planning only. Do not implement M04 product code, create an execution Work Order, select a persistence backend, or authorize execution. Stop for independent exact-head review and promotion before Round 3.
