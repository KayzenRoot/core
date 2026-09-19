# CORE Executor Contract

CORE is governed by GEF and is HIVE-first.

## Authority

1. Resolve authority through `.engineering/SOURCE-HIERARCHY.md`.
2. Read `docs/project-brain/13-CHECKPOINT.md` for current promoted project state.
3. Read `docs/project-brain/16-DECISIONS-LEDGER.md`, `03-SCOPE.md`, `15-DEFINITION-OF-DONE.md`, `04-ARCHITECTURE.md`, then `02-REQUIREMENTS.md` as required by the active Work Order.
4. For implementation, obey the admitted Work Order and exact Git base/head bindings.
5. Chat discussion is input, not durable canonical truth.

## HIVE-first preflight

Before product edits:
- resolve repository root, branch, HEAD and cleanliness with Git;
- verify HIVE v1.0.0 availability;
- resolve CORE with the HIVE read-only MCP surface;
- read the checkpoint through HIVE when available;
- retrieve only the minimum sufficient context;
- use Git/static/AST evidence before LLM inference where deterministic proof exists;
- record HIVE and Git basis in execution evidence.

Stable HIVE v1.0.0 MCP tools:
`project.list`, `project.status`, `context.build`, `context.search`, `memory.search`, `memory.get`, `checkpoint.read`.

If HIVE is unavailable, never fabricate HIVE evidence. Continue only when the Work Order explicitly allows degraded-safe execution.

## GEF lifecycle

`ANALYZE -> SOURCE CHECK -> NEXT NECESSARY INCREMENT -> WORK ORDER -> CONTEXT LOCK -> PREFLIGHT -> EXECUTOR -> TESTS/EVIDENCE -> PR -> AUDIT -> VERDICT -> CHECKPOINT DELTA -> MERGE -> NEXT`

Verdicts are `APPROVED`, `CORRECTION REQUIRED`, or `BLOCKED`.

No HIGH or CRITICAL known defect may be promoted.

## Execution

- Execute the admitted scope, not a speculative redesign.
- Prefer minimum sufficient context and progressive disclosure.
- Reuse exact-state evidence only while its dependencies remain valid.
- Validate cheap/static checks before broader checks.
- Do not force push, rewrite history or destructively clean without explicit governed authorization.
- Never expose secrets in code, logs, prompts or evidence.

## Completion

Green tests are evidence, not completion. Merge is evidence, not completion. Canonical checkpoint promotion follows independent audit and the project DoD.
