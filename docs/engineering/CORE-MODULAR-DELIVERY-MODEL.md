# CORE Modular Planning and Delivery Model

Status: `CANONICAL_DISCOVERY_RULE`

## Purpose

CORE is planned by ChatGPT as architecture/orchestration work and implemented later by Codex as bounded heavy execution. Planning must reduce executor rediscovery, context waste and unnecessary reasoning.

## Authority split

### ChatGPT / architecture orchestrator
Owns:
- source reconciliation;
- HIVE/CORE responsibility boundaries;
- module decomposition and dependency order;
- architecture decisions and ADR proposals;
- file map and file-level construction instructions;
- contracts, schemas, invariants and failure modes;
- security, performance, reliability and test strategy;
- Work Order compilation;
- acceptance criteria, DoD and STOP CONDITION;
- post-execution review and correction planning.

### Codex / implementation executor
Owns:
- implementation of the frozen module Work Order;
- creating/modifying the preplanned files;
- tests and deterministic evidence;
- minimal implementation-time decisions needed to satisfy frozen contracts;
- reporting deviations or blockers instead of silently redesigning the system.

Codex MUST NOT rediscover settled product architecture when canonical planning already answers the question.

## Prompt granularity

Default implementation unit: **one complete module per Codex prompt / Work Order**.

A module may be split only when:
- context size would make execution unsafe;
- independent security/migration boundaries require separate proof;
- implementation cannot be reviewed coherently in one candidate;
- a hard dependency must be promoted first.

Tiny prompt chains are discouraged when one bounded module can be safely implemented and verified in one Work Order.

## Planning-before-code rule

For every module, freeze before implementation:
1. mission and ownership;
2. HIVE overlap/non-duplication disposition;
3. SOLO mode behavior;
4. HIVE-connected behavior;
5. public/internal contracts;
6. file map;
7. file responsibilities and key rules;
8. data/state ownership;
9. lifecycle/state machine;
10. failure taxonomy;
11. security boundaries;
12. performance/resource budgets;
13. telemetry/events;
14. test/eval matrix;
15. migration/compatibility implications;
16. acceptance criteria;
17. Definition of Done;
18. STOP CONDITION;
19. explicit OUT OF SCOPE.

## Executor-ready file map

Planning SHOULD create placeholder/spec files or canonical file-map entries early enough that the future executor knows:
- exact target path;
- purpose;
- exports/interfaces;
- dependencies;
- forbidden dependencies;
- invariants;
- expected tests;
- relevant canonical source.

A placeholder is guidance, not a false implementation claim.

## Review loop

```text
canonical module plan
  -> large bounded Codex Work Order
  -> implementation + tests + evidence
  -> exact-head review by ChatGPT
  -> APPROVED | CORRECTION REQUIRED | BLOCKED
  -> correction Work Order when necessary
  -> checkpoint promotion
  -> next module
```

## HIVE-first execution

When HIVE is available, executor prompts MUST use HIVE preflight and bounded HIVE context rather than re-ingesting the whole repository.

When HIVE is unavailable, CORE planning/evidence remains usable and the executor uses canonical Project Brain + Work Order sources directly.

## Token/time optimization rules

- deterministic discovery before LLM discovery;
- reuse canonical decisions instead of restating them;
- provide exact paths instead of asking Codex to invent structure;
- include acceptance tests in the initial Work Order;
- avoid repeated whole-repository scans;
- use HIVE delta/context capabilities when available;
- fail closed on stale planning basis;
- one module, one coherent evidence bundle whenever safe.
