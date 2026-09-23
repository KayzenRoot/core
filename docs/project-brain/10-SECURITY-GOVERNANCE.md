# CORE Security & Governance

Status: `PRODUCT_DISCOVERY_ACTIVE / M02_THREAT_MODEL_ACTIVE`

## Bootstrap security invariants
- Secrets, credentials, tokens and private user data MUST NOT be committed.
- HIVE accesses CORE through the configured `HIVE_PROJECTS_ROOT` read-only project boundary.
- HIVE-derived memory/retrieval state is noncanonical and cannot overwrite Git truth.
- Ambiguous HIVE project identity fails closed.
- Missing/stale/conflicting canonical authority does not become ALLOW or DONE.
- Destructive Git/history operations require explicit governed authorization.
- Public issues/PRs must not contain exploit-sensitive private information.

## Product security

Cross-product authentication/authorization, privileged execution, secret lifecycle, abuse controls and final release-security gates remain pending their owning modules.

Product implementation cannot claim production security until the applicable module threat models and gates are frozen.

## M02 security and trust boundaries

M02 treats workspace paths, Git metadata/config, repository topology, watcher events, cache entries and HIVE association data as untrusted or stale-able inputs.

Required controls:
- lexical plus physical path validation; traversal/symlink/junction/reparse escape fails closed;
- SOURCE_AUTHORITY does not inherit from Git metadata/object/temp authority;
- Git inspection is read-only, argv/API based, no-network, non-interactive, bounded and cancellation-aware;
- hostile Git helpers/pagers/diff/textconv/fsmonitor behavior must not execute through the inspection path;
- credential-bearing remote/config material is redacted and excluded from proof/cache payloads;
- watcher events never grant freshness or authority;
- watcher overflow/loss broadens invalidation;
- proof caches are derived/disposable and cannot create BOUND after corruption/loss;
- mtime/stat alone is insufficient for correctness-relevant content proof;
- policy/security/filesystem-semantics/provider generation changes invalidate affected handles/cache;
- hashing is streaming, bounded and revalidates security-sensitive path chains;
- repository/submodule/nested graph traversal is depth/node/resource bounded with cycle detection;
- external Git object stores are deny-by-default unless explicitly admitted with provenance;
- HIVE association cannot grant local path authority or overwrite contradictory local checkout evidence;
- concurrent workspace drift requires action-boundary freshness checks; stale handles are never silently revived.

M02 threat model details and adversarial fixtures are maintained in `docs/modules/M02-PROJECT-WORKSPACE-ADAPTER.md`.


## M02 resource-calibration security rules

- security-sensitive resource budgets must become explicit finite values before M02 can be accepted;
- benchmark/calibration failure cannot degrade into partial BOUND success;
- calibration runs use local/synthetic fixtures and no network;
- unsupported scales are reported honestly rather than extrapolated;
- calibration output must redact secrets and user content;
- the bounded Calibration Delta cannot alter authority, dependency or security semantics;
- runtime overrides remain subject to hard validation and cannot create unlimited behavior.


## M03 Round 2 security rules

- a frozen Work Order MUST NOT embed live runtime capabilities;
- stale/superseded/admission-mismatched revisions fail closed;
- Work Order semantic downgrade via schema/version confusion is rejected;
- deny rules override scope allow rules;
- dependency admission is separately governed;
- source/provenance fingerprints are verified before READY where policy requires;
- governance proof replay against a different Work Order revision/fingerprint is rejected;
- Context Lock replay against changed canonical sources/base is rejected;
- READY receipts are basis-bound and non-evergreen;
- raw secrets are prohibited from durable M03 payloads;
- context expansion cannot bypass source secret classification;
- packet DAG cycles and oversized graph/cardinality are typed failures;
- semantic changes cannot be relabeled as documentation/evidence-only by text size;
- UNKNOWN freshness/authority/policy state never becomes ALLOW/READY.


## M03 Round 3 security rules

- no hidden I/O from compiler/admission core;
- no random opaque WorkOrderId allocation inside M03;
- canonical revision persistence MUST validate LPC parent/store-generation precondition;
- stale LPC cannot auto-rebase;
- no internal database is canonical in V0.0;
- compile memo/cache is disposable and cannot authorize READY;
- resource failure produces no partial frozen/admitted result;
- diagnostics are bounded and secret-safe;
- error retryability cannot trigger hidden authority-changing refresh;
- canonical projection must reject duplicate IDs/dangling refs/cycles before freeze;
- external adapter evidence is fingerprinted/provenanced before use;
- DCR proves compilation inputs/output identity but grants no execution authority.


## M03 Round 4 security and trust boundaries

- Every public durable M03 contract is explicitly schema/version/kind bound; unsupported versions and downgrade/replay attempts fail typed.
- The semantic fingerprint projection is field-explicit, sorted and secret-free. Diagnostics, timestamps, transport/rendering fields and raw source/context bodies cannot mutate a frozen revision or enter its fingerprint.
- The core compiler/admission crate receives bounded typed evidence only. It has no hidden filesystem, cwd, clock/timer, Git, GitHub, HIVE, network, process, database, refresh, retry or persistence authority.
- Resolver/adaptor implementations are outside the pure core. M03 never calls them; their evidence includes schema/version, source/workspace/lock/governance identity, fingerprint, provenance, freshness and bounded status.
- M02 snapshot mismatch, missing required basis components, stale generation, source substitution, Context Lock replay, governance proof replay, lineage CAS conflict or UNKNOWN state fails closed and cannot produce READY.
- M03 verifies external governance proof compatibility but cannot mint approval. HIVE references are advisory-only and cannot substitute for canonical Git, M02 or GEF proof.
- Scope deny rules override allow rules, child packet scope is intersected with parent scope, dependency admission is independent, and ambiguous or authority-changing retries fail closed.
- WorkOrderId allocation uses explicit caller identity or deterministic versioned logical keys; random values, clocks, branch names, cwd, host state and map iteration are excluded.
- M03ResourceBudget values must be finite and positive after calibration. Parser depth, serialized size, graph cardinality and diagnostics are deterministically bounded inside the core; exhaustion returns typed failure without partial FROZEN/READY/handoff state. Wall-clock deadlines are enforced only by caller-owned orchestration, which must discard late results and emit typed timeout evidence.
- Diagnostics expose only bounded safe codes, IDs/fingerprints and redaction classes. Raw prompts, source bodies, secrets, credentials, HIVE payloads and unbounded external/process error strings are forbidden.
- External lineage persistence uses LPC compare-and-set. M03 cannot write, auto-rebase, commit, push, update the checkpoint or turn stale evidence into current authority.
