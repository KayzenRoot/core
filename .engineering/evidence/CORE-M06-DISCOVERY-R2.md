# CORE-M06-DISCOVERY-R2: source-bound group-feasibility planning evidence

Status: NON_AUTHORITATIVE_R2_CANDIDATE / NO_PRODUCT_IMPLEMENTATION  
Work Order: https://github.com/KayzenRoot/core/issues/157  
Initial protected-main: c2e8c0a307408e77028b5a1c8f99326afee03927

## Exact accepted sources and before-state

- R1 `docs/modules/M06-CAPABILITY-NEGOTIATION.md` source Git blob `5932113e3abe874184ac3995a2e85afc53bca0ff`, promoted [PR #155](https://github.com/KayzenRoot/core/pull/155) with bounded logical NOT INDEPENDENT owner audit [#156](https://github.com/KayzenRoot/core/issues/156). New **real FULL** protected-main [CI #36345587516](https://github.com/KayzenRoot/core/actions/runs/36345587516) exact `c2e8c0a307408e77028b5a1c8f99326afee03927` completed **11/11 SUCCESS** including actual Linux/Windows M01–M03 and all three bounded fuzz campaigns/supply-chain. This is historical baseline evidence, not current R2 PR proof.
- Accepted M01 `crates/core-registry/src/lib.rs` Git blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14`, `core-contracts/src/lib.rs` `0cd1d56a4ac761ff1c8b761cde0e0d19dfa76bc9`. `graph_snapshot` produces a point-in-time provider/binding observation; `substitution_impact_details` already traverses active transitive dependent and evidence/cache/safety-critical relationships; `ModuleRegistry.validate_graph` tests startup dependencies. Accepted per-capability `resolve` does not itself enforce provider `dependency_capabilities` closure, and neither per-capability `bind_with_generation` nor lease APIs claim atomic **group** admission. Group safety must not be inferred from several one-capability successful results.
- M05's **non-authoritative** `docs/modules/M05-HOST-ADAPTER-FABRIC.md` blob `222dace08091637e0192eb0259ba13109e418ba7` remains its R1–R4 planning source only, not a frozen M06 DTO or proof of authenticated external host.
- Canonical planning model `docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md` blob `0265293a529f9850cc63c72e8aedbe617951cafd`. ACTIVE frozen M04 Context Lock Git blob `7c62aad48f84040d68f7fc70958e851a42f0e1d0` must remain unchanged.

## What is and is not being reviewed

R2 is a **candidate pure graph-feasibility model** over caller-authorized requirements and observed M01 graph/verified *external evidence references*. Distinguishes provider capability dependencies from opaque evidence dependencies and from module startup edges, preserves the existing M01 filter/selection and substitution-impact algorithms, refuses missing/circular/conflicting/stale/unverified group claims and explicitly disallows declaring sequential per-capability binding **atomic**. It lists Option A (read-only provisional M06) and Option B (separately governed M01 atomic batch extension) as **unselected** alternatives, with negative fixtures EV-M06-D15..D28 **ALL PENDING** and no invented benchmark or numeric production resource cap.

**Not done:** new Rust crate or M01 API, eligible alternative enumeration, group transaction, provider security policy, remote SDK/client, live external context service/Codex call, M05 executable host interface, versioned M04 BRC/Run DTO or executable M06 Work Order. External M04 legacy-build/journal/client inventory [#111](https://github.com/KayzenRoot/core/issues/111) remains UNKNOWN, draft contract source PR #118 and product PR #106 remain unmerged, 23 EV-M04 pending. Real local external context service/Docker/Codex proof [#4](https://github.com/KayzenRoot/core/issues/4) remains OPEN.

## Fresh R2 STOP to document after exact-head CI

Two-file Markdown-only scope: `docs/modules/M06-CAPABILITY-NEGOTIATION.md` (append R2, advance candidate header) and this source-bound evidence. Require exact-head Governance and all mandatory successful status jobs; note docs-only classifier and intentional Rust/fuzz no-op; distinct scoped logical owner audit `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT` with zero unresolved HIGH/CRITICAL/review threads; protected squash merge; and a **separate actual new FULL 11/11 main-push CI** before closing. All 28 M06 **discovery** IDs (R1 14 + R2 14) are PENDING; no frozen AEG/product implementation is admitted.
