# CORE-D-204 PROPOSAL — Explicit genesis BRC for pure M04 replay

Status: **PROPOSED / NON-AUTHORITATIVE / NOT ADMITTED**. This document is a reviewable decision candidate only. It does not amend the canonical Decisions Ledger, the frozen M04 specification or Work Order, the ACTIVE Context Lock, acceptance criteria, implementation authorization, or current API. No M04 Pack C implementation is authorized by this proposal alone.

Tracking: [design gate #111](https://github.com/KayzenRoot/core/issues/111), [M04 implementation PR #106](https://github.com/KayzenRoot/core/pull/106), [Pack B audited subgate #110](https://github.com/KayzenRoot/core/issues/110). Candidate governance branch: `proposal/m04-pure-replay-genesis-brc`, based on canonical main `15179cf07d703f074cf50f793a5b1968ba356fc0`. No second GitHub identity is required; any later owner self-audit must say **NOT INDEPENDENT**.

## 1. Problem and conflict

The frozen `replay(request: &ReplayRequestV1, limits: &M04ResourceLimitsV1) -> Result<ReplayProjectionV1, M04ErrorV1>` must reconstruct a faithful `RunProjectionV1` using caller-supplied bounded values, with no hidden I/O. However:
- `ReplayRequestV1` currently supplies `run_id`, `events`, and `expected_boundary`, but not the complete origin BRC.
- The frozen `EventPayloadV1::RunCreated` supplies a Work Order identity reference and boundary fingerprint, not the BRC or its admitted M03 handoff/revalidation values.
- `RunProjectionV1.boundary` is a **complete** `BoundaryRevalidationCapsuleV1`; neither a fingerprint nor `JournalBoundaryV1` can be inverted to reconstruct it.
- Fabricating a BRC, fetching it implicitly from an ambient store/HIVE/M03, or dropping the field violates pure replay, causal provenance or the frozen output contract.

Scope of this design gate: reconstruct *historical state truth*, not choose a retry winner, activate a stale M03 authority, fetch external artifact bodies, invent a backend, or change the acceptance count.

## 2. Candidate resolution (subject to separate freeze)

**Propose caller-supplied genesis BRC as an explicit, required replay input.** The host retrieves the original durable BRC through an explicit `M04StateStoreV1` port, outside the pure state engine, and passes its immutable value together with the canonical journal. Replay validates the supplied capsule and its binding to `RunCreated` before it constructs `RunProjectionV1.boundary`. No internal M03/HIVE/network/file/process lookup is permitted.

Possible implementation-level shape, **illustrative only and not an approved API**:

```rust
pub struct ReplayRequestV2 {
    pub run_id: RunId,
    pub genesis_boundary: BoundaryRevalidationCapsuleV1,
    pub events: Vec<CanonicalEventV1>,
    pub expected_boundary: Option<JournalBoundaryV1>,
}
```

A versioned `ReplayRequestV2` is the safer compatibility candidate if existing V1 requests must remain decodable. Because CORE is pre-release, an explicitly governed in-place V1 breaking amendment is a second possible disposition, but **never add a required V1 field silently** or use `serde(default)` / implicit fallback. Decide version compatibility and wire migration as part of the *later binding Contract Delta*, before changing any code. The frozen pure replay signature and service construction map must be updated together under that delta, not left with an impossible V1 path.

This approach preserves the existing `RunCreated` canonical event payload shape and avoids embedding an entire BRC in a potentially repeated event envelope. Alternative for review: a full bounded BRC payload in genesis `RunCreated`, which would require changing canonical event vectors and event-size calibration; neither option is adopted by filing this proposal.

## 3. Required exact origin and lineage checks

A later accepted design and implementation must:
1. Reject a missing, unsupported, malformed or over-budget genesis BRC with a typed fail-closed result, not infer one from the journal.
2. Validate the BRC's schema/version; admitted `AdmittedWorkOrderV1` and `RunStartRevalidationV1` identity consistency; Work Order ID, revision and fingerprint; workspace basis; Context Lock and governance/source generation; and admission receipt binding against caller-supplied authorized records. External authority remains caller-owned and separately verified by BRC checks in Pack E; a syntactically matching digest alone is insufficient.
3. Require the first event to be the canonical Run origin. Bind `RunCreated.work_order` and `RunCreated.boundary_fingerprint` to the exact supplied genesis capsule using the admitted domain-separated BRC fingerprint algorithm. Reject wrong or absent event, a middle slice presented as genesis, or any attempt to reuse a BRC from another Run/epoch/Work Order.
4. Validate canonical event kind/payload/domain, typed Run/Attempt/Step lineage, zero-based sequence from declared origin, expected/resulting generation increments, previous/resulting journal-root chaining and any expected boundary. Never accept a fabricated matching final root as proof of missing history.
5. Derive a full `ReplayProjectionV1`, preserving the *caller-supplied verified* genesis BRC as `RunProjectionV1.boundary`. Build the remaining projection exclusively through validated journal events; reject contradictory parent/child state or historical reuse of attempt/step ordinals.
6. Treat snapshots as derived acceleration only: verify snapshot provenance and compare its derived reconstruction with full replay, never mint state or authority from a snapshot alone.

The caller's separate store contract must durably retain and atomically associate genesis BRC and the canonical Run journal for the history-retention period. A valid journal without its genesis capsule is insufficient for full replay and must yield an explicit integrity/stale-authority outcome, not an empty/default capsule.

## 4. Threat, budget, no-hidden-I/O and migration rules

- No direct M04 dependency on Tokio, core-runtime, core-workspace, host filesystem, Git/GitHub, HIVE, network SDK, database driver, cache or ambient time. Existing direct crate production dependencies stay within the frozen allow-list.
- Bound BRC size, replay depth, canonical event bytes, event count and diagnostic output using the later measured `M04ResourceLimitsV1`; cap+1 must fail atomically. Do not choose numeric defaults without Pack H calibration evidence.
- Any compatibility rule for old V1 serialized requests must be explicit and fail closed: unavailable genesis BRC is a real missing precondition, not a permissible downgrade. Document whether pre-release V1 can change or a new V2 contract/domain is required. Regenerate golden vectors **only after** a governing decision authorizes the affected exact shapes and schema.
- Keep the current 23 AC-M04 / EV-M04 nodes one-to-one. Proposed affected evidence is especially AC/EV-007 replay equivalence, 008 integrity rejection, 009 M03 BRC stale/substitution, 012 cross-platform canonical vectors, 013 resource bounds, 014 snapshots, 016 no hidden I/O and 021/022 exact-head platform CI. Do not mark any EV PASS in this proposal.
- The existing M04 Pack A/B historical subgate reviews remain historical facts at their exact heads; a contract/source amendment can invalidate *future* applicability and requires fresh source-lock binding and affected tests, not retroactive rewrite of those reviews.

## 5. Blocking validation matrix for any later implementation

Positive: valid admitted genesis BRC plus complete contiguous canonical journal deterministically recreates the same projection as incremental application across Windows and Ubuntu; repeated inputs and stable field order yield stable root and fingerprint; no hidden I/O is observed.

Negative: missing BRC; wrong schema/version; substituted or forged BRC fingerprint; foreign Run ID or execution epoch; substituted M03 Work Order ID/revision/fingerprint, workspace basis, Context Lock, source generation or admission receipt; missing/reordered/substituted genesis; truncated or middle-only event stream; duplicated event sequence/Attempt/Step ID/ordinal; stale/future generation, altered event domain, wrong prior or resulting root; unverified final boundary; cap+1 BRC/events/bytes/depth; invalid or substituted snapshot; incompatible legacy V1 replay request. Every negative must reject typed, without partial semantic publication or hidden repair.

## 6. Required governance route / STOP CONDITION

**This PR is only an ADR proposal**, not a frozen Contract Delta. Its safe review gate is documentation-only scope, no frozen-canonical fingerprint drift, no implementation or API change, exact-head hosted CI, owner-audit recorded **NOT INDEPENDENT**, and protected-main promotion only if governance permits.

To actually admit a changed replay contract:
1. Review the alternatives, choose a single compatibility/versioning disposition and record an **ACCEPTED** decision in `docs/project-brain/16-DECISIONS-LEDGER.md`.
2. In a separately governed source-amendment PR, update `docs/project-brain/02-REQUIREMENTS.md`, `04-ARCHITECTURE.md`, `11-TEST-PLAN.md`, `15-DEFINITION-OF-DONE.md`, `docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md`, the active M04 Work Order, source-fingerprint Context Lock, Evidence Bundle and planned checkpoint delta consistently; record any breaking-wire/API migration and its new authority basis. Check the full canonical source hierarchy rather than changing derived GEF alone.
3. Run exact-head governance and hosted CI, review complete diff, security and unresolved HIGH/CRITICAL, and record an explicit bounded owner self-audit (**NOT INDEPENDENT**) without native self-approval. Promote through protected main only after those conditions hold.
4. Rebind/re-admit the existing M04 implementation line to the new promoted frozen sources using the project's normal governed synchronization and fresh exact-head CI. Only then start full Pack C journal/replay/snapshot implementation and its STOP C acceptance.
5. Leave issue #111 **BLOCKED_DESIGN** if version policy, exact M03 trust binding, source amendment, lock re-admission or required CI remains unknown. Never present this proposal, Pack B approval or a green docs-only CI as implementation permission for Pack C.

**Current status: PROPOSED. No canonical decision, frozen contract amendment, Work Order amendment, active Context Lock update, full replay implementation, merge of PR #106 or checkpoint promotion is performed by this document.**
