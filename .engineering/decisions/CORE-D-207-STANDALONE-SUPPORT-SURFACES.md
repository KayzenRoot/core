# CORE-D-207 — Auxiliary standalone source and PR-preflight retirement

Date: 2026-09-29
Status: ADOPTED_ON_PROTECTED_MAIN_PROMOTION_ONLY
Work Order: #184; parent #172; predecessor #182.
Original authorized protected-main base: `f27e3a3f1be64d2dfedc762dea30dd1dce7cab17`.
Reconciled protected-main base after GEF v1.1.1 adoption: `7593644c04725ad6a75fcda5cf2e59fad04c1a81`.
GEF adoption PR #186 changed generated `.gef` adoption metadata only; it did not reauthorize M04 or rewrite the canonical M04 source lock.
Prior independent main-push FULL #36595275630: 11/11 SUCCESS on the original predecessor base.

## Decision

Current Project Brain upload order and deployment, modular delivery process, technology research and PR template must use exact local Git-canonical standalone authority without mandatory external context-server/preflight, Docker, project registry, federation, or Codex-only execution. M01–M03 standalone V2 remain implemented; M04's old execution authority remains STALE/BLOCKED pending #111 previous-V1 external binary/API/journal evidence; M05/M06 remain discovery, M23 remains FUTURE local context/evidence only. Generic optional external providers require separate version/identity/security and never grant Git, Work Order, path or execution authority. Retired provider-specific named protocol/federation research is NOT_PLANNED, not product code.

## Original byte-exact source provenance

All four original documents are preserved verbatim after `## Historical discovery arclegacy_provider (non-operative; exact prior Git blob follows)` and tested against these *original* exact Git blobs:

- `docs/project-brain/00-README-UPLOAD-ORDER.md` = `b25433e68e5d209c3767f1e727e0bffe9c0a6fc9`
- `docs/project-brain/12-LOCAL-DEPLOYMENT.md` = `3988ac80814fba9b48569cfdb59f46218094ebf3`
- `docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md` = `0265293a529f9850cc63c72e8aedbe617951cafd`
- `docs/research/CORE-TECHNOLOGY-CANDIDATES.md` = `7e6e33ecf635baf74990a361aa01b3072e13b34d`

The active PR template must not preserve obsolete mandatory preflight instructions; its exact original blob is `ff569cdd20be8bab3579e595905930d8277fe650` at `.github/pull_request_template.md` on the accepted base.

## Scope and STOP

Only four auxiliary documents, PR template, this dated decision, narrow validator and no-network regressions may change. Keep all nine current M04 canonical blobs/Work Order/Context Lock/Evidence/GEF untouched; no Rust, Cargo, CI/ruleset or old PR #106/#118 changes. Historical M01/M02/M03 module documentation requires a separate bounded disposition before parent #172 may close.

Require raw-byte arclegacy_provider fingerprints and negative CRLF tests, new exact-head real FULL 11/11 Linux/Windows M01–M03 with bounded fuzz, 16/16 M01 soak on both OS, honest PRB, cargo deny/audit and SBOM, scoped owner self-audit NOT INDEPENDENT and no unresolved HIGH/CRITICAL/threads, strict protected expected-head squash, then a **separate ACTUAL new-main full 11/11 push** before #184 closes. #111 previous-V1 external consumers UNKNOWN/BLOCKING.
