# CORE GEF Policy

## Core rules

1. Canonical source truth outranks chat memory and derived caches.
2. Product work requires an admitted Work Order.
3. Every Work Order binds an execution base and expected evidence.
4. Context locks become stale when material authority/base changes.
5. Deterministic proof precedes LLM inference when practical.
6. HIGH/CRITICAL known defects block promotion.
7. Evidence must bind the exact candidate/head.
8. Canonical checkpoint promotion occurs only after the exact-head owner-audit verdict and all required technical gates.
9. Scope discoveries are classified as NECESSARY, IMPORTANT, FUTURE or OUT OF SCOPE before admission.
10. Destructive Git/history operations require explicit governed authorization.
11. Reviews follow reviewer-first correction: safe bounded findings are corrected directly by the auditor when current tools can implement and validate them; delegation to Codex/another executor is reserved for corrections that genuinely require broader execution capabilities or governance.
12. CORE has one operational GitHub identity, `KayzenRoot`. The owner self-audit is a separate logical stage, is explicitly `NOT INDEPENDENT`, and never requires a second account or a native self-approval.

## Local source and context

Canonical local Git and checked-in Project Brain remain source truth. No external context server is a dependency or prerequisite. Missing required evidence still blocks affected changes under the active Work Order.


## Reviewer-first correction

During review, the smallest safe causal fix should be applied by the reviewer when it is within frozen scope and can be validated with available exact-head evidence.

Direct review fixes MUST NOT:
- expand scope;
- add dependencies without admission;
- alter architecture/security policy without governance;
- bypass required local/runtime evidence;
- weaken tests, security, DoD or acceptance criteria.

Any direct fix creates a new exact head and invalidates prior head-bound evidence for changed inputs.

Codex/another executor is used only when the correction cannot be completed and validated safely in the review environment.

## Single-account audit

The owner self-audit must inspect the exact base/head and full delta, confirm required technical evidence and checks, report findings/severity and zero unresolved HIGH/CRITICAL before returning `OWNER_SELF_AUDIT_APPROVED`. Missing another reviewer identity is not evidence failure. Missing or failed technical gates remain fail-closed.
