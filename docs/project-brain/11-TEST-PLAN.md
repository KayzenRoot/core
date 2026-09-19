# CORE Test & Evidence Plan

Status: `BOOTSTRAP_BASELINE`

## Bootstrap proof ladder
1. Python syntax compilation for governance/HIVE bootstrap tooling.
2. Deterministic source/governance validation.
3. Unit tests for HIVE project identity resolution and fail-closed collision handling.
4. Hosted GitHub Actions evidence on the exact candidate head.
5. Semantic exact-head audit against Work Order, Scope, Architecture, Requirements and DoD.

## Current required bootstrap commands
```text
python -m py_compile scripts/validate_governance.py scripts/hive_bootstrap.py
python scripts/validate_governance.py
python -m unittest discover -s tests -p "test_*.py" -v
```

## Product validation
Unit, integration, E2E, security, performance, recovery, compatibility and benchmark obligations remain `PENDING_DISCOVERY` and will be frozen by product architecture/risk.

Historical green evidence never automatically proves a changed head.
