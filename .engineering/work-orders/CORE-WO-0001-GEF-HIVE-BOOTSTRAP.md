# CORE-WO-0001 - GEF + HIVE Foundation

Status: `EXECUTED_AWAITING_AUDIT`

## OBJECTIVE
Bootstrap KayzenRoot/core as a new GEF v1.0.0 project and make it structurally compatible with the stable HIVE v1.0.0 context/retrieval/MCP contract before product planning begins.

## HIVE PREFLIGHT
Upstream HIVE v1.0.0 contract inspected from KayzenRoot/hive release commit `a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf`.
Local user HIVE runtime is outside GitHub and is not falsely claimed as executed by this Work Order.

## CANONICAL BASIS
- User authorization to create CORE and install GEF + HIVE integration.
- GEF Bootstrap v1.0.0 release commit `866fe3af8cccc65c929aaf6a47a924401fa448b3`.
- HIVE v1.0.0 release commit `a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf`.
- CORE seed base `bcd688dd7376644f3c19dc04f6f7566b03281ec2`.

## CONTEXT BUDGET
Read only the GEF installation/quickstart/source-authority surfaces and HIVE Project Registry, Context Manager governance paths, stable MCP contract and registration/index/retrieval seams necessary for bootstrap.

## RISK / ASSURANCE
`STANDARD_PLUS`

## SCOPE
- Materialize canonical Project Brain compatible with HIVE v1.0.0.
- Materialize GEF target-project governance and execution/review/evidence contracts.
- Add deterministic governance validation.
- Add HIVE local registration/index/corpus preparation tooling.
- Add GitHub governance scaffolding.

## OUT OF SCOPE
- CORE product implementation.
- Installing/running the user's local HIVE Docker runtime from GitHub.
- Choosing CORE runtime stack.
- Vendoring HIVE or GEF source workspaces into CORE.
- Repository admin/ruleset mutations unavailable through the connected write surface.

## ACCEPTANCE CRITERIA
- Required HIVE governance paths exist.
- GEF/HIVE versions and release commits are pinned.
- Source hierarchy prevents competing canonical truth.
- Validator passes in hosted CI.
- HIVE bootstrap script performs health -> resolve/register -> inspect -> index -> corpus sync.
- Product implementation remains gated.
- Exact candidate head receives audit before checkpoint promotion/merge.

## TESTS
- `python scripts/validate_governance.py`
- GitHub Actions `Governance` on exact PR head.
- Structural inspection of PR diff.

## EVIDENCE
PR exact-head CI and review are the acceptance evidence. Local HIVE runtime execution remains a post-merge operator action because the runtime is on the user's machine.

## DELIVERABLES
Repository governance, Project Brain, HIVE integration documentation/tooling, GitHub workflow/templates and this Work Order.

## REVIEW FORMAT PT-BR
Reviewer reports findings by severity and returns `APPROVED`, `CORRECTION REQUIRED` or `BLOCKED`.

## STOP CONDITION
Stop after bootstrap PR is created and exact-head evidence/audit is available. Do not begin CORE product discovery or implementation inside this Work Order.
