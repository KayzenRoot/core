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


## M02 Round 6 scope classification

### NECESSARY
- RCG Resource Calibration Gate inside the frozen M02 Work Order;
- reproducible calibration evidence before final M02 acceptance;
- finite WorkspaceResourceBudget defaults derived from implementation evidence;
- DWS component/changed-set delta revalidation with full-recompute equivalence;
- bounded Calibration Delta limited to numeric defaults/thresholds/evidence.

### IMPORTANT
- benchmark fixture scaling sufficient to expose resource bottlenecks on supported CI platforms;
- explicit rejected-candidate evidence for budget selection.

### FUTURE
- WMF Workspace Merkle Forest;
- persistent runtime self-tuning;
- automatic background recalibration.

### OUT OF SCOPE FOR M02 V0.0
- guessed numeric resource defaults without evidence;
- unlimited security-sensitive budgets;
- architecture/dependency changes disguised as calibration;
- WMF implementation in the initial Work Order.


## M03 Round 2 scope classification

### NECESSARY
- versioned Work Order v1 envelope;
- immutable FrozenWorkOrder revision model;
- WorkOrderAdmissionRequest/Receipt separation;
- workspace requirement + fresh M02 admission binding;
- Context Lock binding;
- external governance proof verification;
- bounded packet DAG;
- ScopeEnvelope + deny precedence;
- ExecutionCorrectionProposal vs WorkOrderRevisionDiff separation;
- AEG v1;
- StopConditionV1;
- CBE dimensions/expansion semantics;
- WSF staleness dependency classes;
- compact provenance/lineage.

### IMPORTANT
- stable logical IDs across compatible revisions;
- packet/criterion/evidence split/merge lineage;
- compact AdmittedWorkOrder handoff for M04;
- admission status/reason taxonomy.

### FUTURE
- durable shared Work Order registry/database;
- organization-wide distributed lineage service;
- cryptographic signatures beyond available governance capability contracts;
- adaptive context budget auto-tuning.

### OUT OF SCOPE FOR M03 V0.0
- Run/Attempt/Step persistence;
- agent/model scheduling;
- command/tool execution;
- source mutation;
- Git delivery;
- final evidence artifact collection;
- review verdict generation.


## M03 Round 3 scope classification

### NECESSARY
- stateless compiler/service semantics;
- explicit compile/validate/diff/correction/admission/handoff operations;
- deterministic logical ID direction;
- LineageSnapshot + LPC CAS semantics;
- external persistence boundary;
- canonical projection/fingerprint reuse;
- PacketContextPlan + PCM;
- DCR compilation provenance;
- M03ResourceBudget dimensions;
- typed error/retryability taxonomy;
- one-crate/dependency direction.

### IMPORTANT
- optional bounded in-memory compile memoization if benchmarks justify it;
- resource calibration fixture families;
- safe diagnostics/provenance ergonomics.

### FUTURE
- persistent compile memo/cache;
- internal/shared Work Order database;
- distributed lock/sequence service;
- network Work Order registry;
- adaptive runtime resource tuning.

### OUT OF SCOPE
- hidden repository scans;
- Git commits/pushes;
- HIVE/network calls inside core compiler;
- Run scheduler/state;
- tool/source execution;
- final evidence/review engines.

## M03 Round 4 implementation-surface freeze

### NECESSARY for initial M03 V0.0 implementation
- add the single `crates/core-work-order` crate using the exact Round 4 file map;
- add root workspace membership/Cargo.lock integration;
- implement the frozen public WorkOrderService/data contracts;
- reuse core-identity canonical hashing and core-workspace public evidence types;
- add the seven frozen M03 fuzz targets;
- add the frozen M03 unit/integration/property/adversarial/benchmark coverage;
- extend GitHub CI/governance with M03 exact-head gates;
- produce M03 execution/calibration/evidence artifacts;
- perform the bounded M03 Calibration Delta before final acceptance.

### IMPORTANT but not automatically admitted
- disposable in-memory compile memoization, only if benchmark evidence justifies it;
- extra diagnostic CLI surfaces, only if required to satisfy a frozen acceptance obligation without expanding product authority.

### FUTURE / CONDITIONAL
- persistent M03 compilation cache;
- database-backed Work Order registry;
- async compiler/service runtime;
- alternate canonical serializer/hash stack;
- extra adapter crates;
- direct HIVE/GitHub/Git/filesystem resolver implementations inside M03;
- M03 crate split/micro-crates;
- additional third-party graph/regex/Git/cache/watcher/network libraries.

### OUT OF SCOPE for initial M03
- M04 Run/Attempt/Step execution;
- source mutation;
- Git commit/push/PR/merge;
- governance approval creation;
- evidence collection owned by M15;
- scheduling/parallel execution authority;
- secret storage;
- background benchmarking/autotuning;
- modification of M02 authority semantics.

Any need to modify lower product crates or expand these boundaries during execution requires a governed Correction Delta; green tests alone do not authorize the expansion.

