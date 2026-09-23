# M04 — Run / Attempt / Step Engine

Status: `ROUND_1_DISCOVERY_CANDIDATE`
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

## Round 2 questions

Round 2 must freeze:
- exact Run/Attempt/Step transition matrices;
- event envelope and semantic projection;
- idempotency keys and duplicate semantics;
- generation/CAS laws;
- cancellation precedence;
- attempt/step numbering and ordering;
- continuation cursor semantics;
- M03 BRC compatibility rules;
- resource dimensions and bounded-history policy;
- durable vs derived fields.

## STOP CONDITION

Round 1 is planning only. Do not implement M04 product code, create an execution Work Order, select a persistence backend, or authorize execution. Stop for independent exact-head review and promotion of this discovery candidate.
