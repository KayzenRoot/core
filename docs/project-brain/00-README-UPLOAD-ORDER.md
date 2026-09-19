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

## HIVE v1.0.0 compatibility

The following exact paths are mandatory because HIVE v1.0.0 Context Manager loads them as governance sources:

- `docs/project-brain/13-CHECKPOINT.md`
- `docs/project-brain/03-SCOPE.md`
- `docs/project-brain/15-DEFINITION-OF-DONE.md`
- `docs/project-brain/04-ARCHITECTURE.md`
- `docs/project-brain/16-DECISIONS-LEDGER.md`

Do not rename or relocate these files without a governed HIVE compatibility migration.
