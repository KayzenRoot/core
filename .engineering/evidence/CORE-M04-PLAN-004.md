# CORE-M04-PLAN-004 — Round 4 Implementation-Addressable Planning Evidence

Status: `REVIEW_CANDIDATE`  
Type: planning only  
Canonical base: `19275cf485123b938f4f6fd32f941178afde7f82`  
Assurance: `ELEVATED`

## Source check

Round 4 was derived from canonical M04 Rounds 1-3 plus the current workspace graph. The observed workspace already contains `core-work-order`, `core-identity`, the shared workspace dependency set and the existing fuzz package.

Observed direct inputs:
- `core-work-order` currently depends on `core-identity`, serde, serde_json and thiserror;
- `core-identity` exposes canonical/fingerprint primitives backed by SHA-256;
- M03 exposes admitted Work Order/receipt/handoff evidence required at M04 boundaries;
- the existing CI has dedicated M01/M02/M03 Windows, Ubuntu and fuzz jobs.

## Frozen Round 4 candidate

Round 4 freezes:
- one `core-run-state` crate;
- exact product/test/fuzz/benchmark file map;
- direct dependency set: `core-work-order`, `core-identity`, serde, thiserror;
- no direct Tokio/core-runtime/core-workspace or external I/O/backend dependency;
- pure prepared-commit service signatures;
- host-owned `M04StateStoreV1` atomic compare-and-commit contract;
- caller-owned external-reference evidence seam;
- versioned M04 binary domain framing over `core_identity::fingerprint_bytes`;
- required V1 type groups;
- exact property/adversarial law inventory;
- six named bounded fuzz targets;
- deterministic `m04_run_state` benchmark/calibration matrix;
- RAS/TLG/CER/RJR/BRC/ICF/ASF as required V0.0 semantics;
- a separate Round 5 final planning freeze.

## Dependency disposition

No new external crate is admitted by this planning increment. A future implementation may add the new workspace member `crates/core-run-state`, but product code remains unauthorized until Round 5 and separate execution admission.

## Resource honesty

No numeric M04 resource defaults or performance measurements are claimed. Exact values remain implementation-calibration outputs under a numeric/evidence-only Calibration Delta.

## Non-effects

This planning increment does not:
- implement product code;
- create an execution Work Order or active Context Lock;
- select a persistence backend;
- authorize M04 execution;
- alter M03 or M01 product semantics.

## STOP CONDITION

Independent exact-head review and successful hosted CI are required before promotion. If approved, Round 5 final planning freeze is the next legal increment.
