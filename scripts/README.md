# CORE scripts

- `validate_governance.py`: deterministic local canonical Git, GEF, Context Lock and evidence validation.
- `ci_impact.py`: fail-closed GitHub Actions impact classification.
- `m01_soak.py` and `m01_prb.py`: M01 bounded soak and performance checks.
- `m04_legacy_inventory.py`: offline owner-provided compatibility questionnaire; it never scans owner devices or a remote database.

No project-specific external daemon or MCP service is required.
