# CORE Scope

Status: `PRODUCT_DISCOVERY_ACTIVE`

## NECESSARY - bootstrap

- Materialize GEF v1.0.0 new-project governance.
- Materialize a HIVE v1.0.0 compatible Project Brain.
- Provide deterministic governance validation.
- Provide local scripts to register, inspect, index and retrieval-sync CORE in HIVE.
- Provide GitHub PR, Work Order and evidence scaffolding.
- Keep product planning separate from bootstrap installation.

## IMPORTANT - planning phase

- Define the operational responsibilities of CORE relative to HIVE.
- Define executor/orchestrator, review, automation, policy and integration responsibilities.
- Define interfaces with Codex/IDEs/GitHub and other NexLabs systems only after discovery.
- Define observability, security, reliability and performance targets with evidence.

## FUTURE

To be classified during governed discovery. No future capability is admitted merely because it is discussed.

## OUT OF SCOPE - bootstrap

- Product runtime implementation.
- Duplicating HIVE context, memory or retrieval runtime.
- Replacing Git as canonical source history.
- Claiming autonomous production behavior before architecture and verification are frozen.
- Selecting a final application stack before product architecture requires it.


## ACCEPTED DISCOVERY CONSTRAINTS

- CORE is a headless action/execution plane; dashboard/cockpit/web UI are OUT OF SCOPE.
- CORE must remain usable without HIVE.
- When compatible HIVE is available, CORE consumes HIVE-owned intelligence through contracts instead of duplicating HIVE subsystems.
- Default delivery granularity is one complete planned module per bounded Codex Work Order when safe.
- Product planning must provide executor-ready file maps, contracts, invariants and tests before implementation.


## M02 Project / Workspace Adapter scope

### NECESSARY
- deterministic explicit workspace attachment and identity;
- distinct ProjectBinding / Workspace / Repository / Worktree identities;
- read-only Git/filesystem basis inspection;
- typed SOURCE/GIT_METADATA/EXTERNAL_OBJECT/TEMP authority roots;
- path containment/escape proof before later action modules;
- workspace drift detection and action-boundary freshness validation;
- standalone operation with explicit HIVE reconciliation when available;
- canonical WorkspaceBasis fingerprints/diffs and compact binding receipts;
- bounded zero-LLM evidence, hashing and revalidation;
- deterministic invalidation and proof-cache semantics.

### IMPORTANT
- watcher-driven invalidation hints for lower revalidation cost;
- L1 proof reuse/coalescing;
- differential GitInspector provider evaluation;
- WMF/DWS acceleration where equivalence is proven;
- compact evidence surfaces that reduce HIVE/LLM downstream context.

### FUTURE / CONDITIONAL
- persistent L2 proof cache after recovery/corruption/secret-safety design;
- hybrid or Rust-native GitInspector acceleration after semantic/security/benchmark proof;
- additional platform watcher providers outside supported CI targets.

### OUT OF SCOPE FOR M02
- source mutation or patch application;
- Git checkout/reset/branch/commit/merge/push/fetch;
- repository repair;
- network access to Git remotes;
- HIVE RAG/memory/repository-intelligence duplication;
- runtime sandbox enforcement owned by M11;
- Git/GitHub delivery owned by M20/M21;
- semantic AST/repository analysis;
- visual UI/dashboard.
