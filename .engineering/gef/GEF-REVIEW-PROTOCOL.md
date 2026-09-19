# CORE GEF Review Protocol

Review the exact candidate/head against Scope, Architecture, Requirements, acceptance criteria and DoD.

## Review order

1. Confirm authorized base and candidate head.
2. Confirm Context Lock validity.
3. Inspect semantic delta, not only file count.
4. Verify required tests/evidence are exact-head and successful.
5. Check scope drift, architectural drift, security/integrity regressions and hidden assumptions.
6. Classify findings by severity.
7. Return one verdict: `APPROVED`, `CORRECTION REQUIRED`, or `BLOCKED`.

No checkpoint promotion with unresolved HIGH/CRITICAL findings.

Review output is in Brazilian Portuguese unless a Work Order explicitly requires another language.
