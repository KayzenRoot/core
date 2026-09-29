# GEF v1.0.0 Adoption - CORE

## Adoption

- Project: `KayzenRoot/core`
- Mode: `NEW_PROJECT`
- GEF: `v1.0.0`
- Stable upstream commit: `866fe3af8cccc65c929aaf6a47a924401fa448b3`
- Prompt mode: `GEF_V1_CORE_STANDALONE`
- Review mode: `DELTA_EXACT_HEAD`
- Assurance: fail closed for missing required evidence

GEF is installed as CORE's engineering/governance layer. It does not replace CORE Project Brain truth or Git.

## Lifecycle

`ANALYZE -> SOURCE CHECK -> NEXT NECESSARY INCREMENT -> WORK ORDER -> CONTEXT LOCK -> PREFLIGHT -> EXECUTOR -> TESTS/EVIDENCE -> PR -> AUDIT -> VERDICT -> CHECKPOINT DELTA -> MERGE -> NEXT`

## New-project invariant

Planning precedes product implementation. CORE may create governance, discovery and planning artifacts now. Functional product code begins only after admitted Scope/Requirements/Architecture/DoD and a bounded Work Order exist.

## Standalone execution

Exact checked-in Git sources, accepted Work Orders, Context Locks and GEF checkpoints are sufficient. A project context daemon is never required.
