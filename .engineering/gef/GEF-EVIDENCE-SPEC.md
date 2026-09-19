# CORE GEF Evidence Specification

A governed evidence bundle should identify:

- Work Order id;
- authorized base SHA;
- candidate/head SHA;
- HIVE availability/project resolution;
- canonical sources used;
- context fingerprint or equivalent HIVE evidence when available;
- files changed;
- tests/static/build/validation commands and outcomes;
- CI workflow/run identities when hosted evidence is required;
- failures and corrections;
- unresolved risks;
- reviewer/audit verdict;
- proposed checkpoint delta.

Historical green evidence is not automatically valid for a new head. Reuse requires unchanged relevant inputs/dependencies.
