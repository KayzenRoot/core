# CORE-WO-0001 - GEF + HIVE Foundation

Status: `COMPLETED_APPROVED`

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
- `python -m py_compile scripts/validate_governance.py scripts/hive_bootstrap.py scripts/hive_mcp.py`
- `python scripts/validate_governance.py`
- `python -m unittest discover -s tests -p "test_*.py" -v`
- GitHub Actions `Governance` on exact PR head.
- Structural/semantic inspection of PR diff.

## EVIDENCE
- Implementation PR: `#1`.
- Authorized base: `bcd688dd7376644f3c19dc04f6f7566b03281ec2`.
- Audited candidate: `342404099665f9302454f22b48241d3ec0a844fd`.
- Governance run: `35441163753` / run #9 / SUCCESS.
- Validator: PASS; 27 required artifacts; GEF checkpoint/source bridges CONSISTENT.
- HIVE bootstrap/MCP tests: 6/6 PASS.
- Semantic review: `5255641959` / APPROVED.
- Squash merge: `e669498558e6f129efd8a8f2e20679f28e8326a7`.
- CRITICAL: 0; HIGH: 0; blocking MEDIUM: 0.
- Local HIVE runtime execution remains separate operator evidence because the user's local machine is outside this GitHub execution environment.

## DELIVERABLES
Repository governance, complete bootstrap Source Pack, HIVE integration/tooling, project-scoped Codex HIVE MCP configuration, GitHub workflow/templates and governed evidence.

## REVIEW FORMAT PT-BR
Reviewer reports findings by severity and returns `APPROVED`, `CORRECTION REQUIRED` or `BLOCKED`.

## STOP CONDITION
Satisfied. CORE-WO-0001 is closed. Product implementation was not started. The next legal action is governed CORE product discovery.
