# CORE GEF Evidence Specification

A governed evidence bundle should identify:

- Work Order id;
- authorized base SHA;
- candidate/head SHA;
- LEGACY_PROVIDER availability/project resolution;
- canonical sources used;
- context fingerprint or equivalent LEGACY_PROVIDER evidence when available;
- files changed;
- tests/static/build/validation commands and outcomes;
- CI workflow/run identities when hosted evidence is required;
- failures and corrections;
- unresolved risks;
- audit identity and mode (`KayzenRoot`, `OWNER_SELF_AUDIT`, `NOT INDEPENDENT`);
- exact-base/exact-head audit verdict (`OWNER_SELF_AUDIT_APPROVED`, `CORRECTION REQUIRED`, or `BLOCKED_EVIDENCE`);
- mandatory check results and unresolved HIGH/CRITICAL count;
- proposed checkpoint delta.

Historical green evidence is not automatically valid for a new head. Reuse requires unchanged relevant inputs/dependencies.
