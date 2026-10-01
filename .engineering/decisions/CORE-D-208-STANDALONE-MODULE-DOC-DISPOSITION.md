# CORE-D-208 — Standalone M01/M02/M03 module-document authority disposition

Date: 2026-10-01
Status: ADOPTED_ON_PROTECTED_MAIN_PROMOTION_ONLY
Work Order: #187; parent #172.
Exact protected-main baseline: `5c2952223395747522576fa46e11063c493ca3a3`.
GEF release basis: latest published production release `v1.1.1`.

## Decision

M01, M02 and M03 are completed/promoted standalone modules, but their long accepted module records contain older provider-specific planning and contract language mixed with later standalone amendments. For current and future CORE execution, each module document receives a short authority overlay before the preserved record.

The overlay imports the already-promoted standalone revision in that same record as current authority. All **provider-neutral** safety, determinism, ownership, lifecycle/workspace/compiler and evidence requirements in the prior record remain retained unless explicitly superseded. All **retired-provider-specific** identities, preferences, services, context fields, preflights and runtime assumptions are non-operative for new execution wherever they conflict with the standalone revision. A later change may re-admit an external provider only through a separately governed provider-neutral contract and may never grant filesystem/Git/source authority implicitly.

## Exact immutable prior Git blob provenance

Each module now embeds its complete immediate predecessor source byte-for-byte after the marker `## Prior accepted module record (exact prior Git blob follows)`. No normalization, deletion or rewrite of the predecessor payload is permitted.

- `docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md`: prior Git blob `46851bb0dfd2f00ee36f790a64e3b6ba8971d2a9`; current revision imported by overlay: `Owner-directed standalone architecture amendment (2026-09-28)`.
- `docs/modules/M02-PROJECT-WORKSPACE-ADAPTER.md`: prior Git blob `3690e81e50f6fed1a28dea56bd22e0c83383b33d`; current revision imported by overlay: `Owner-directed standalone contract revision (2026-09-28)`; current serialized contract remains V2.
- `docs/modules/M03-WORK-ORDER-ENGINE.md`: prior Git blob `8e73e5fb7f80f7b695f957bbf0d0c12fa25974ed`; current revision imported by overlay: `Owner-directed standalone context contract revision (2026-09-28)` plus `Standalone V2 numeric calibration (2026-09-29)`.

`docs/modules/00-MASTER-MODULE-MAP.md` was inventoried at blob `32d65c726ee657ae75ccb4a23581e15d6844dce7` and required no provider-specific disposition in this increment.

## M04 and source-hierarchy boundary

This decision does not edit the Project Brain checkpoint, Decisions Ledger, Scope, DoD, Architecture, Requirements, Security, Test Plan or M04 module document. It does not edit or reinterpret `.engineering/context-locks/CORE-WO-M04-001.json`. That lock remains STALE with `productImplementationAuthorized=false`; #111 remains UNKNOWN/BLOCKING and PRs #106/#118 remain non-authoritative.

Because M01/M02/M03 module documents are not members of M04's nine-source stale lock, this bounded disposition does not re-admit M04.

## Assurance / STOP

Promotion requires the #187 RED→GREEN evidence, exact-head FULL 11/11, complete scoped patch audit, zero unresolved HIGH/CRITICAL findings or review threads, `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT`, expected-head protected squash with no bypass and a separate actual new-main push FULL 11/11 before #187 closes. Parent #172 remains open for a later whole-repository active-provider scan.
