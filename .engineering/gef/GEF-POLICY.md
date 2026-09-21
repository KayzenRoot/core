# CORE GEF Policy

## Core rules

1. Canonical source truth outranks chat memory and derived caches.
2. Product work requires an admitted Work Order.
3. Every Work Order binds an execution base and expected evidence.
4. Context locks become stale when material authority/base changes.
5. Deterministic proof precedes LLM inference when practical.
6. HIGH/CRITICAL known defects block promotion.
7. Evidence must bind the exact candidate/head.
8. Canonical checkpoint promotion occurs only after audit.
9. Scope discoveries are classified as NECESSARY, IMPORTANT, FUTURE or OUT OF SCOPE before admission.
10. Destructive Git/history operations require explicit governed authorization.
11. Reviews follow reviewer-first correction: safe bounded findings are corrected directly by the reviewer when current tools can implement and validate them; delegation to Codex/another executor is reserved for corrections that genuinely require broader execution capabilities or governance.

## HIVE

HIVE is the preferred intelligence layer, not canonical-write authority. If HIVE is unavailable, execution reports the degraded state and follows the Work Order's degradation policy.


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
