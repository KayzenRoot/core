# CORE-M04-CONTRACT-DELTA-001 — exact replay authority and request-identity amendment

Status: **CANDIDATE / NOT YET PROMOTED / NOT YET IMPLEMENTATION AUTHORITY**. Governing decision proposed as CORE-D-204; prior one-file non-authoritative design ADR (PR #113) and three independent proof gaps documented in [Issue #111](https://github.com/KayzenRoot/core/issues/111).

## Selected candidate; subject to governed review

The unmerged M04 Pack A/B implementation and frozen V1 design lack (1) full genesis BRC, (2) full BRC for **every continuation epoch**, and (3) durable original idempotency request fingerprint/operation domain for populated replay maps. Neither a digest nor an ambient M03/HIVE lookup is acceptable provenance. This source-amendment candidate selects **caller-supplied journal-bound immutable full capsules for origin and continuations** plus **required original request fingerprint and operation domain on every committed semantic event**. One prepared commit produces exactly one event and one idempotency record; the closed event-kind/domain mapping is specified in the amended M04 module source.

A deliberately breaking *pre-release* V1 re-freeze is selected conditionally only if a recorded producer/consumer inventory confirms there is no previous-V1 durable journal data to migrate. The old M04 implementation remains unmerged in PR #106; that alone does **not** prove the absence of external consumers. If any durable old V1 consumer exists or cannot be ruled out, this candidate **STOPS**, and governance must choose explicit V2 event/request types with a complete migration policy before admission. Never silently downgrade, default a missing authority capsule, reuse a payload fingerprint as the original request fingerprint, or fabricate an unknown history item.

## Frozen shape once admitted

```rust
pub struct JournalBoundBrcV1 {
    pub event_sequence: EventSequenceV1,
    pub event_id: EventId,
    pub resulting_journal_root: JournalRoot,
    pub boundary: BoundaryRevalidationCapsuleV1,
}
// The full event has required operation_domain and original request_fingerprint.
pub struct ReplayRequestV1 {
    pub run_id: RunId,
    pub events: Vec<CanonicalEventV1>,
    pub boundary_capsules: Vec<JournalBoundBrcV1>,
    pub expected_boundary: Option<JournalBoundaryV1>,
}
```

Every `RunCreated` and `ContinuationCreated` must match **exactly one** full caller-supplied capsule including exact ID, sequence, resulting root, M03 Work Order, RunStartRevalidation, Context Lock/source and governance generation, workspace basis, domain-separated fingerprint and execution epoch. No capsule may match two events. An extra, missing, substituted or unbound capsule invalidates entire replay, with no partial publication. `RunProjectionV1.boundary` must be the most recent verified BRC. The host-side `M04StateStoreV1` atomically stores and retains the full capsule with the origin or continuation event and exposes it through explicit replay input. Pure M04 never fetches any hidden data.

All committed mutation events carry a closed `M04OperationDomainV1` and the original `CanonicalFingerprint` **request** digest alongside their existing payload digest, idempotency key, sequence, generation and journal link. This permits replay to reconstruct the *identical populated idempotency map* from authenticated canonical events, without inventing a separate authority ledger. `RunCreated` anchors `RunAdmission`; `RunAdmitted` anchors a **distinct** `Transition`, not a duplicate admission. Remaining event-domain pairs are frozen in the module contract and validated on constructor, deserialization, mutation-sensitive serialization and replay. Canonical framing, golden vectors, snapshots and budget proofs change accordingly.

## Scope/compatibility/safety

No persisted M04 V1 history is known in merged CORE, but external consumers must be inventoried before acceptance. The unchanged 23-node AC/EV map still blocks global acceptance. No new Rust production dependency, backend, hidden I/O, M03 authority fabrication or resource-default guess. New capsule count/combined-byte budgets require a separate evidence-backed Pack H numeric calibration. The original active Work Order/Context Lock is **suspended on promotion of this source delta**, and must be **separately re-admitted** against all changed canonical Git blob hashes before PR #106 changes code or Pack C starts. A green docs-only PR never authorizes implementation.

## Audit and promotion gate

Review exact 9-source/Work Order/evidence/Context Lock/GEF/checkpoint closure; no forced branch update, no cross-branch hash substitution; exact-head 10/10 Windows/Ubuntu/governance/fuzz/security/supply-chain and zero unresolved HIGH/CRITICAL. Solo reviewer `KayzenRoot` must record `OWNER_SELF_AUDIT / NOT INDEPENDENT` and never submit a native self-approval. Protected-main promotion is separate from the later re-admission, implementation pack audits and canonical checkpoint promotion.
