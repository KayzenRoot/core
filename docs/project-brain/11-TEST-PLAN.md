# CORE Test & Evidence Plan

Status: `PRODUCT_DISCOVERY_ACTIVE`

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
Product validation is frozen progressively by module and risk. M01 has promoted exact-head evidence; M02 obligations are being frozen during discovery; later-module unit, integration, E2E, security, performance, recovery, compatibility and benchmark obligations remain `PENDING_DISCOVERY` until their owning plans are accepted.

Historical green evidence never automatically proves a changed head.


## M02 Round 4 validation obligations

Before Round 4 mechanisms can be promoted into an implementation Work Order, evidence must cover:
- watcher-loss/overflow and rename-storm behavior;
- action-boundary freshness with watcher disabled;
- CIG selective invalidation versus full required-mask validation;
- cache hit/miss/bypass/invalidation provenance;
- timestamp-preserving content rewrite;
- cache corruption/loss safety;
- bounded/coalesced hashing under cancellation and large files;
- symlink/junction swap during hash/path proof;
- deterministic repository-graph serialization and delta/full equivalence;
- GitInspector differential fixtures against the Git semantic reference oracle;
- hostile Git config, secret canaries, no-network/no-mutation behavior;
- Windows and Unix path/filesystem cases;
- cold/warm/no-change/small-delta/large-delta benchmarks;
- memory/concurrency ceilings under large status/hash workloads.

Exact numeric resource defaults and production GitInspector provider selection remain blocked until this evidence exists. Historical M01 evidence does not substitute for M02-specific proof.
