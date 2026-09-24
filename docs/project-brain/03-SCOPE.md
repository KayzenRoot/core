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


## M04 Rounds 1-3 scope classification

### NECESSARY
- deterministic Run / Attempt / Step lifecycle state and typed lineage;
- M03 READY/BRC revalidation at Run and continuation boundaries;
- closed transition tables with immutable terminal history;
- Run-generation CAS serialization and atomic state-fence publication;
- fingerprint-bound idempotency and monotonic cancellation;
- append-only canonical event journal, replay validation and derived snapshots;
- ICF continuation records without recovery-policy ownership;
- bounded external outcome/evidence references;
- finite resource dimensions and evidence-backed calibration gate;
- explicit versioned public request/receipt/result contracts;
- blocking EV-M04-001..023 acceptance evidence graph.

### IMPORTANT
- compact deterministic projection/snapshot acceleration;
- backend-neutral storage/reference ports;
- bounded safe diagnostics and machine reason codes;
- cross-platform canonicalization vectors and replay fixtures.

### FUTURE / CONDITIONAL
- persistent storage backend selection/optimization;
- archive/retention/compaction policy beyond active proof history;
- distributed multi-writer state stores or replication;
- performance accelerators that preserve ASF/RJR equivalence.

### OUT OF SCOPE FOR M04
- host/provider execution;
- capability negotiation/leases;
- agent/model selection or execution policy;
- command/tool execution and source mutation;
- recovery strategy ownership;
- quota/cost policy;
- Git/GitHub delivery/release actions;
- security-policy engines;
- HIVE federation intelligence;
- verification/evidence/review verdict ownership;
- telemetry transport/observability spine.

M04 implementation remains unauthorized until final planning freeze and separate governed execution admission.


## M04 Round 4 scope classification

### NECESSARY
- one `core-run-state` V0.0 crate and frozen file/test/fuzz/bench map;
- minimal direct dependency set and acyclic M03 -> M04 direction;
- pure prepared-commit service layer;
- backend-neutral atomic state-store port contract;
- caller-owned external-reference evidence seam;
- concrete V1 request/result/type groups;
- M04 canonical framing over the shared core-identity digest primitive;
- exact property/adversarial law inventory;
- six named bounded fuzz targets;
- deterministic Windows/Ubuntu benchmark and Resource Calibration Gate;
- required disposition of RAS/TLG/CER/RJR/BRC/ICF/ASF;
- separate Round 5 final planning freeze.

### IMPORTANT
- concise store/reference adapter ergonomics without semantic authority leakage;
- snapshot/replay benchmark scenarios that expose large-history cost honestly;
- exact file-level traceability from EV-M04 nodes to future implementation evidence.

### FUTURE / CONDITIONAL
- concrete persistent backend;
- event archive/retention service;
- distributed store replication/consensus;
- persistent snapshot/projection cache;
- runtime adaptive tuning.

### OUT OF SCOPE FOR ROUND 4
- product implementation;
- execution Work Order/active Context Lock;
- database/backend selection;
- M05+ executor/policy/review implementation;
- numeric resource defaults without implementation measurements.


## M04 Round 5 final-freeze scope

### IN SCOPE FOR THE PLANNING FREEZE
- freeze `CORE-WO-M04-001`, pending Context Lock, Evidence Bundle skeleton and executor handoff;
- bind the promoted Round 1-4 semantics to one exact construction packet graph A-H;
- preserve the exact `core-run-state` file/dependency map;
- freeze AC-M04-001..023 one-to-one against EV-M04-001..023;
- freeze the numeric/evidence-only Resource Calibration Delta;
- freeze executor STOP states and independent review ownership;
- record exact canonical source fingerprints for later admission staleness checks.

### OUT OF SCOPE
- all M04 product implementation;
- active execution authority or execution-branch creation;
- Cargo/workspace/fuzz/bench source changes;
- production persistence/backend choice;
- M05+ implementation;
- architecture/contract redesign disguised as calibration or admission.

Round 5 promotion freezes planning truth only. Execution still requires a separate governed admission delta.
