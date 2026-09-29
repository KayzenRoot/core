# CORE

CORE is a standalone, headless Rust engineering runtime with GEF-governed Work Orders, local Git source authority, capability safety, review and delivery.

**No external context service, Docker project service, required repository MCP connector or external memory database is needed to start or develop CORE.** Owner-directed cutover is tracked in [Work Order #172](https://github.com/KayzenRoot/core/issues/172). Historic external context service evidence is preserved only as dated provenance, not as an executable prerequisite.

## Current state

- M01, M02 and M03: implemented and promoted.
- M04: external earlier-V1 consumer compatibility [#111](https://github.com/KayzenRoot/core/issues/111) remains an independent blocker. Do not merge blocked M04 contract/product PRs until fresh source/lock admission.
- M05 and M06: non-authoritative planning; rebaseline before executing any new product code.
- Standalone cutover: staged. Removal of retired MCP/bootstrap tooling is only phase 1; Rust and canonical source amendments require separate reviews.

## Work with the repository

Read `AGENTS.md`, `.engineering/SOURCE-HIERARCHY.md` and `docs/project-brain/13-CHECKPOINT.md`. Run `python scripts/validate_governance.py` for exact local Git/GEF source consistency. CI runs required Governance and module-specific Linux/Windows Rust, supply-chain and fuzz checks without a project server.
