# CORE GEF Review Protocol

Review the exact candidate/head against Scope, Architecture, Requirements, acceptance criteria and DoD.

## Review order

1. Confirm authorized base and candidate head.
2. Confirm Context Lock validity.
3. Inspect semantic delta, not only file count.
4. Verify required tests/evidence are exact-head and successful.
5. Check scope drift, architectural drift, security/integrity regressions and hidden assumptions.
6. Classify findings by severity.
7. Return one verdict: `OWNER_SELF_AUDIT_APPROVED`, `CORRECTION REQUIRED`, or `BLOCKED_EVIDENCE`.

No checkpoint promotion with unresolved HIGH/CRITICAL findings.

Review output is in Brazilian Portuguese unless a Work Order explicitly requires another language.


## Reviewer-first correction protocol

After classifying findings, the reviewer MUST attempt bounded direct correction before returning work to an executor.

### DIRECT_FIX_ELIGIBLE

A finding is direct-fix eligible when the smallest causal correction:
- remains within the admitted Work Order/scope;
- is narrow and low-risk;
- adds no unapproved dependency or architecture;
- can be performed with currently available repository/GitHub tools;
- can be revalidated with exact-head evidence from the current review environment.

For such findings, the reviewer SHOULD:
1. patch the same review branch/PR directly;
2. document the correction;
3. discard historical head evidence for changed inputs;
4. rerun the required exact-head validation;
5. continue the review on the new head.

### EXECUTOR_REQUIRED

Delegate back to Codex/another executor only when the correction requires substantial product work, unavailable local/runtime state, dependency/architecture/scope changes, or validation that the reviewer cannot perform to the required assurance level.

A corrective executor prompt MUST describe only the residual executor-required findings and MUST preserve already-closed findings.

The reviewer MUST NOT use delegation as a convenience substitute for a safe direct fix.

## Single-account owner self-audit

CORE has one operational GitHub identity: `KayzenRoot`. Executor and auditor are separate logical lifecycle stages, not separate accounts or reviewer sessions. The exact-head audit may be performed by KayzenRoot after executor handoff and must record the exact base/head, full changed surface, source/lock bindings, required checks, findings and severity, unresolved HIGH/CRITICAL count, and verdict.

Use `OWNER_SELF_AUDIT_APPROVED` only when every required exact-head CI/security/evidence gate passes, scope and source bindings are valid, and unresolved HIGH/CRITICAL findings are zero. State `NOT INDEPENDENT` in the audit record. Never submit a native GitHub `APPROVE` review from the PR author account. The absence of another identity alone is never `BLOCKED_EVIDENCE`; missing/failed technical evidence, invalid scope/source bindings, or unresolved HIGH/CRITICAL findings remain blockers.

An owner self-audit verdict does not itself merge code, promote a checkpoint, release, or waive the protected-main ruleset.
