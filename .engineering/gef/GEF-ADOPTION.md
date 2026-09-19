# GEF v1.0.0 Adoption - CORE

## Adoption

- Project: `KayzenRoot/core`
- Mode: `NEW_PROJECT`
- GEF: `v1.0.0`
- Stable upstream commit: `866fe3af8cccc65c929aaf6a47a924401fa448b3`
- Prompt mode: `GEF_V1_HIVE_FIRST`
- Review mode: `DELTA_EXACT_HEAD`
- Assurance: fail closed for missing required evidence

GEF is installed as CORE's engineering/governance layer. It does not replace CORE Project Brain truth or Git.

## Lifecycle

`ANALYZE -> SOURCE CHECK -> NEXT NECESSARY INCREMENT -> WORK ORDER -> CONTEXT LOCK -> PREFLIGHT -> EXECUTOR -> TESTS/EVIDENCE -> PR -> AUDIT -> VERDICT -> CHECKPOINT DELTA -> MERGE -> NEXT`

## New-project invariant

Planning precedes product implementation. CORE may create governance, discovery and planning artifacts now. Functional product code begins only after admitted Scope/Requirements/Architecture/DoD and a bounded Work Order exist.

## HIVE enhancement

When available, HIVE v1.0.0 is the default context/retrieval/checkpoint layer for executor work. HIVE does not receive authority to overwrite canonical Git sources.
