# CORE Project Brain: standalone source pack

Status: `CURRENT_STANDALONE_GIT_CANONICAL`  
Authority: CORE-D-205, CORE-D-206 and CORE-D-207 upon protected-main promotion of #184.

## Executor source authority

Bind exact local Git HEAD, tree and relevant source Git blob fingerprints before executing an admitted Work Order. Consult the current Project Brain in canonical order:

1. `docs/project-brain/13-CHECKPOINT.md`: current promoted state.
2. `docs/project-brain/16-DECISIONS-LEDGER.md`: effective dated decisions.
3. `docs/project-brain/03-SCOPE.md`: current permitted scope.
4. `docs/project-brain/15-DEFINITION-OF-DONE.md`: actual completion gates.
5. `docs/project-brain/04-ARCHITECTURE.md`: current architecture.
6. `docs/project-brain/02-REQUIREMENTS.md`: admitted requirements.

Retain governed 01 overview, 10 security, 11 tests, 12 deployment and 14 backlog as domain-specific references under `.engineering/SOURCE-HIERARCHY.md`. CORE owns these exact local paths. No external memory index, project registry, Docker stack, MCP endpoint or mandatory network is needed for CORE source authority. M01–M03 standalone V2 are implemented; M04 is blocked pending #111; M05/M06 are discovery only and M23 is future local context/evidence work.

The old upload-order and pinned-provider compatibility instructions are preserved below verbatim but are not operative.


## Historical discovery arclegacy_provider (non-operative; exact prior Git blob follows)

# CORE Project Brain

## Canonical authority order for executor startup

1. Latest approved `13-CHECKPOINT.md`
2. `16-DECISIONS-LEDGER.md`
3. `03-SCOPE.md`
4. `15-DEFINITION-OF-DONE.md`
5. `04-ARCHITECTURE.md`
6. `02-REQUIREMENTS.md`
7. Remaining governed Project Brain sources

This startup order does not replace domain-specific authority rules in `.engineering/SOURCE-HIERARCHY.md`.

## Source Pack inventory

- `01-PROJECT-OVERVIEW.md` — product identity and current stage
- `02-REQUIREMENTS.md` — admitted obligations
- `03-SCOPE.md` — scope classification
- `04-ARCHITECTURE.md` — approved architecture boundary
- `10-SECURITY-GOVERNANCE.md` — security/trust obligations
- `11-TEST-PLAN.md` — validation/evidence obligations
- `12-LOCAL-DEPLOYMENT.md` — deployment boundary
- `13-CHECKPOINT.md` — current promoted project state
- `14-BACKLOG.md` — future work
- `15-DEFINITION-OF-DONE.md` — completion semantics
- `16-DECISIONS-LEDGER.md` — governed decisions

## LEGACY_PROVIDER v1.0.0 compatibility

The following exact paths are mandatory because LEGACY_PROVIDER v1.0.0 Context Manager loads them as governance sources:

- `docs/project-brain/13-CHECKPOINT.md`
- `docs/project-brain/03-SCOPE.md`
- `docs/project-brain/15-DEFINITION-OF-DONE.md`
- `docs/project-brain/04-ARCHITECTURE.md`
- `docs/project-brain/16-DECISIONS-LEDGER.md`

Do not rename or relocate these files without a governed LEGACY_PROVIDER compatibility migration.
