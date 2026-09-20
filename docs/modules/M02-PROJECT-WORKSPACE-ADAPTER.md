# M02 - Project / Workspace Adapter

Status: `DISCOVERY_IN_PROGRESS`

## Mission

M02 binds the CORE action plane to a concrete local project/workspace safely and deterministically.

It converts ambiguous human notions such as "this project", "this checkout" or "this repository" into typed, versioned, machine-verifiable workspace handles that later CORE modules can execute against without guessing paths, repository identity, Git basis, worktree state or HIVE project association.

M02 is an adapter and authority boundary. It is not a second HIVE Project Registry, not a repository-intelligence engine and not a mutation/delivery engine.

## Ownership boundary

### M02 owns

- local workspace discovery and explicit workspace attachment;
- deterministic project/workspace/repository identity for the CORE action plane;
- repository/worktree/submodule boundary discovery;
- canonical path normalization and path-within-authority checks;
- read-only Git/worktree inspection needed to establish execution basis;
- workspace generation/fingerprint calculation;
- detection of relevant workspace drift after a basis was established;
- standalone project binding when HIVE is unavailable;
- HIVE project-binding reconciliation when HIVE is available;
- machine-readable workspace readiness/degradation receipts;
- workspace handles consumed by later execution modules;
- deterministic workspace metadata needed for cache/evidence invalidation.

### M02 does NOT own

- HIVE Project Registry canonical project truth;
- semantic repository intelligence, AST/symbol indexing, RAG or retrieval;
- durable organizational memory;
- arbitrary filesystem sandboxing or capability enforcement, which belongs to M11;
- source mutation/change application, which belongs to M13;
- Git commit/branch/PR/release delivery, which belongs to M20/M21;
- Work Order semantics, which belong to M03;
- durable Run/Attempt/Step execution state, which belongs to M04;
- host/IDE integration, which belongs to M05;
- visual workspace UI.

## Core invariant

CORE MUST never execute against an implicit or stale workspace basis.

Before an execution-capable module can act, it must hold a valid `WorkspaceHandle` whose identity, authority roots, repository/worktree basis and generation are explicit.

## Operating modes

### Standalone mode

M02 derives the local project/workspace binding from deterministic local evidence:
- explicit CLI/config path;
- filesystem identity;
- Git repository/worktree metadata when present;
- repository remotes only as non-secret metadata;
- canonical configuration;
- stable path identity.

Standalone mode must remain useful without HIVE and must not synthesize HIVE identifiers.

### HIVE-enhanced mode

When HIVE is compatible and available:
- M02 resolves or receives the HIVE project identity through the existing external HIVE contract;
- local workspace evidence is reconciled against that project association;
- HIVE project identity may enrich the binding;
- HIVE remains canonical for its Project Registry identity;
- Git/filesystem state remains canonical for the actual local checkout;
- disagreement is explicit and cannot be silently merged.

## Initial state model

```text
UNBOUND
  -> DISCOVERING
  -> CANDIDATE
  -> VALIDATING
  -> BOUND

CANDIDATE/VALIDATING
  -> AMBIGUOUS
  -> BLOCKED

BOUND
  -> DRIFTED
  -> REVALIDATING
  -> BOUND

BOUND/DRIFTED
  -> DETACHED
```

State names are discovery candidates until the M02 state contract is frozen.

## Identity model

M02 must distinguish these identities rather than collapse them:

1. `ProjectBindingId`
   - CORE action-plane binding identity.
   - May reference a HIVE project ID but is not itself HIVE's canonical registry.

2. `WorkspaceId`
   - stable identity for the attached local workspace authority root.

3. `RepositoryId`
   - stable identity for a discovered Git repository boundary.

4. `WorktreeId`
   - identity for a concrete Git worktree/checkout.

5. `WorkspaceGeneration`
   - monotonic/derived generation representing the validated execution basis.

6. `WorkspaceBasisFingerprint`
   - deterministic fingerprint over correctness-relevant workspace basis fields.

Physical absolute paths are attributes, not the only identity.

## Candidate canonical basis fields

A workspace basis may include:
- schema version;
- ProjectBindingId;
- optional HIVE project reference + provenance;
- workspace root canonical path identity;
- repository set and boundary graph;
- primary repository/worktree identity;
- Git object-format identity where applicable;
- HEAD object/commit where applicable;
- branch/ref name as diagnostic metadata, not sole identity;
- index fingerprint;
- tracked worktree delta fingerprint;
- untracked-set fingerprint according to policy;
- submodule/gitlink basis;
- sparse-checkout declaration when applicable;
- case-sensitivity/filesystem normalization capability;
- authority-root set;
- configuration generation;
- M01 runtime generation;
- relevant security/policy fingerprint;
- timestamp only as diagnostic metadata, excluded from semantic identity by default.

## Round 1 technology candidates

### WIL - Workspace Identity Lattice

**Problem:** path, repository, worktree and project identity are often conflated.

**Mechanism:** model ProjectBinding, Workspace, Repository and Worktree as separate typed identities with explicit relations.

**Expected benefit:** prevents accidental execution in the wrong checkout, supports worktrees/monorepos/nested repositories and improves evidence/cache identity.

**Primary risk:** identity model can become too complex.

**Promotion criterion:** deterministic tests must prove stable identity across path aliases while differentiating semantically distinct worktrees/bindings.

### CWB - Canonical Workspace Basis

**Problem:** later modules need a reproducible "what exactly am I acting on?" basis.

**Mechanism:** canonical schema + deterministic fingerprint for correctness-relevant workspace state.

**Expected benefit:** exact-head-like semantics extend from Git commits to dirty local workspaces and multi-repository workspaces.

**Primary risk:** fingerprint churn from volatile fields.

**Promotion criterion:** correctness-relevant changes must alter the basis while irrelevant metadata does not.

### PAF - Path Authority Firewall

**Problem:** path traversal, symlink/junction escape and ambiguous relative roots can send CORE outside the intended workspace.

**Mechanism:** normalize requested paths against declared authority roots, resolve escape-sensitive components under governed policy and return typed allow/deny receipts.

**Expected benefit:** safe path addressing before later M11 sandboxing.

**Primary risk:** cross-platform path semantics and TOCTOU races.

**Promotion criterion:** adversarial Windows/Unix path tests, traversal/symlink/junction cases and no false authorization outside declared roots.

### BRL - Basis Reconciliation Layer

**Problem:** HIVE project identity, explicit config, Git roots and current checkout can disagree.

**Mechanism:** deterministic evidence precedence and reconciliation receipts with explicit MATCH / PARTIAL / CONFLICT / UNKNOWN states.

**Expected benefit:** no silent attachment to the wrong project.

**Primary risk:** over-blocking legitimate detached/local work.

**Promotion criterion:** conflict matrix tests and bounded standalone fallback semantics.

### WDG - Workspace Drift Guard

**Problem:** execution basis can change after preflight.

**Mechanism:** generation/fingerprint validation at execution boundaries with targeted delta recomputation.

**Expected benefit:** later Work Orders can fail closed on stale basis rather than act on a changed checkout.

**Primary risk:** excessive rescans.

**Promotion criterion:** targeted invalidation benchmarks show drift detection cost scales with changed basis rather than full-tree rescans where platform/Git evidence permits.

### RBR - Repository Boundary Resolver

**Problem:** monorepos, nested repositories, linked worktrees, submodules and bare repositories create ambiguous boundaries.

**Mechanism:** deterministic repository/worktree boundary graph built from Git-native metadata rather than filename heuristics alone.

**Expected benefit:** correct authority and routing across complex workspaces.

**Primary risk:** unusual Git layouts.

**Promotion criterion:** fixture matrix for normal repo, bare repo, linked worktree, nested repo, submodule, sparse checkout and detached HEAD.

### WBR - Workspace Binding Receipt

**Problem:** later modules need proof that workspace admission happened safely.

**Mechanism:** typed receipt containing identities, generation, basis fingerprint, authority roots, binding provenance, degraded/conflict state and validation evidence refs.

**Expected benefit:** proof-carrying workspace attachment.

**Primary risk:** oversized receipts.

**Promotion criterion:** canonical compact receipt is sufficient for downstream validation without embedding full repository content.

### DWS - Delta Workspace Snapshot

**Problem:** repeated full workspace scans waste CPU/I/O and later LLM context.

**Mechanism:** stable basis + deterministic delta set for changed repository/workspace facts.

**Expected benefit:** lower I/O, faster revalidation, cache-friendly HIVE/CORE context and less repeated evidence.

**Primary risk:** missed invalidation.

**Promotion criterion:** delta reconstruction must equal full recomputation in property/fixture tests.

## Security baseline

M02 MUST:
- treat path inputs and repository metadata as untrusted;
- reject traversal outside authority roots;
- handle symlink/junction/reparse-point escape explicitly;
- avoid shell-string Git invocation;
- use bounded command output if Git subprocesses are needed;
- redact credentials embedded in remote URLs;
- never serialize secrets from Git credential helpers/environment;
- distinguish read-only repository inspection from later mutation authority;
- not grant filesystem authority merely because a path exists;
- protect against repository path confusion, nested .git indirection and hostile config values;
- fail closed on ambiguous security-sensitive normalization.

## Performance and token-economy baseline

M02 is zero-LLM for discovery, identity, path validation, Git basis and drift detection.

Design priorities:
1. Git-native metadata before recursive filesystem scanning.
2. Stable canonical identity with volatile diagnostics excluded.
3. Content/delta handles rather than embedding large file lists.
4. Incremental/delta basis recomputation where correctness allows.
5. No semantic repository analysis that belongs to HIVE.
6. Machine-readable reasons for cache hit/miss/bypass/invalidation.
7. Cross-platform normalization without converting every path to expensive content reads.
8. Reuse M01 DCS/DIF/generation primitives instead of inventing a second fingerprint stack.

## Failure taxonomy candidates

- WORKSPACE_NOT_FOUND
- WORKSPACE_AMBIGUOUS
- WORKSPACE_AUTHORITY_VIOLATION
- REPOSITORY_NOT_FOUND
- REPOSITORY_BOUNDARY_CONFLICT
- GIT_METADATA_INVALID
- HIVE_PROJECT_BINDING_CONFLICT
- WORKSPACE_BASIS_STALE
- WORKSPACE_DRIFT_DETECTED
- PATH_NORMALIZATION_FAILED
- SYMLINK_ESCAPE_DETECTED
- UNSUPPORTED_FILESYSTEM_SEMANTICS
- WORKTREE_STATE_UNREADABLE
- CONFIGURATION_CONFLICT

Errors must remain typed, stable and redacted.

## Initial test matrix

### Identity / basis
- same workspace reached via harmless path alias -> stable identity;
- distinct linked worktrees -> distinct WorktreeId;
- dirty index/worktree changes alter required basis;
- volatile timestamp does not alter semantic basis;
- detached HEAD is represented without false failure.

### Repository layouts
- normal repository;
- bare repository;
- linked worktree;
- monorepo;
- nested independent repository;
- submodule;
- sparse checkout;
- no-Git workspace;
- multiple repository roots under one workspace.

### Path authority
- .. traversal;
- symlink escape;
- Windows junction/reparse path;
- drive-letter/case normalization;
- UNC path policy;
- reserved/device path rejection where applicable;
- race-aware revalidation at use boundary.

### HIVE reconciliation
- HIVE unavailable -> bounded standalone binding;
- HIVE matches local evidence -> enriched binding;
- HIVE project conflicts with local repo evidence -> explicit conflict;
- stale HIVE project/workspace association -> no silent success.

### Drift
- HEAD changed;
- index changed;
- worktree changed;
- untracked set changed under policy;
- submodule pointer changed;
- config/security generation changed;
- irrelevant diagnostic metadata changed;
- delta recomputation equals full recomputation.

## Benchmark families

- workspace attach cold/warm;
- Git basis calculation clean/dirty;
- path validation throughput;
- linked-worktree discovery;
- nested/submodule boundary resolution;
- basis revalidation no-change;
- basis revalidation small delta;
- 10 / 100 / 1,000 / 10,000 changed-path metadata cases where relevant;
- memory growth across repeated bind/unbind/revalidate;
- Windows and Unix filesystem/path cases.

First implementation establishes compatible hardware/platform baselines. Do not fabricate absolute performance promises.

## M02 planning STOP CONDITION

M02 planning is not frozen until all of the following are explicit:
- final ownership vs HIVE/M11/M13/M20;
- standalone and HIVE-enhanced behavior;
- exact state model;
- identity and canonical basis contracts;
- path-authority semantics;
- repository/worktree boundary model;
- drift and revalidation semantics;
- failure taxonomy;
- security/threat model;
- performance/resource/token-economy requirements;
- exact crate/file map and dependency rules;
- test/fuzz/fixture/benchmark plan;
- M02 DoD;
- explicit OUT OF SCOPE;
- executor packets;
- frozen Work Order and FINAL STOP CONDITION.

No M02 product implementation is authorized before that freeze.


## Round 2 - State machine, contracts and validity semantics

### M02 binding state machine

M02 uses a small explicit state machine. Discovery candidates and error reasons are data, not hidden pseudo-states.

```text
UNBOUND
  -> DISCOVERING
  -> VALIDATING
  -> BOUND

DISCOVERING
  -> BLOCKED

VALIDATING
  -> BLOCKED

BOUND
  -> DRIFTED
  -> DETACHING

DRIFTED
  -> REVALIDATING
  -> DETACHING

REVALIDATING
  -> BOUND
  -> BLOCKED

BLOCKED
  -> DISCOVERING
  -> VALIDATING
  -> DETACHING

DETACHING
  -> UNBOUND
```

Rules:
- `AMBIGUOUS`, `NOT_FOUND`, `CONFLICT` and `UNSUPPORTED` are typed reasons for BLOCKED, not durable top-level states.
- a candidate path/repository set is data emitted by DISCOVERING;
- only VALIDATING may produce a binding receipt;
- only BOUND may issue a current WorkspaceHandle;
- DRIFTED immediately makes the prior handle stale for correctness-relevant action;
- REVALIDATING never silently revives an old handle; success emits a new generation/handle;
- DETACHING revokes the local attachment view but does not mutate repositories or HIVE.

### Handle versus durable receipt

`WorkspaceHandle` is an ephemeral runtime object:
- bound to the current M01 runtime epoch/generation;
- immutable after issue;
- carries WorkspaceGeneration + WorkspaceBasisFingerprint;
- is NOT a filesystem capability and is NOT a sandbox token;
- must be freshness-validated at downstream action boundaries.

`WorkspaceBindingReceipt` is durable evidence:
- schema-versioned;
- records the binding identities, basis fingerprint, authority roots, association status and validation evidence refs;
- may be retained after process exit;
- cannot by itself authorize future action after restart;
- must be revalidated to mint a new WorkspaceHandle.

### Contract set

#### WorkspaceAttachRequest
Required fields:
- schema_version;
- explicit workspace locator/root;
- optional expected ProjectBindingId;
- optional expected repository/worktree selector;
- optional expected HIVE project reference;
- authority policy;
- repository discovery policy;
- untracked-file policy;
- symlink/reparse policy;
- required binding assurance;
- configuration generation.

#### ProjectBindingId
CORE-local action-plane binding identity. It remains stable across temporary HIVE availability changes for the same local workspace binding. A different HIVE association changes binding generation/reconciliation evidence but does not rewrite WorkspaceId.

#### WorkspaceId
Identity of the local workspace authority root after platform-aware canonicalization. Harmless aliases resolving to the same governed root must converge; distinct governed roots must not.

#### RepositoryId
Local repository identity derived from Git repository/common-dir evidence and local boundary identity. Remote URLs are association hints only and MUST NOT be the sole RepositoryId.

#### WorktreeId
Identity for one concrete Git checkout/worktree. Linked worktrees of one repository share RepositoryId but have distinct WorktreeId values.

#### WorkspaceGeneration
Monotonic generation for accepted binding/basis changes within a runtime epoch. Revalidation that changes any correctness-relevant basis component produces a new generation.

#### WorkspaceBasis
Canonical structured representation of the execution basis.

Correctness component classes:
- WORKSPACE_IDENTITY;
- FILESYSTEM_SEMANTICS;
- AUTHORITY_ROOTS;
- REPOSITORY_GRAPH;
- WORKTREE_IDENTITY;
- HEAD_STATE;
- INDEX_STATE;
- TRACKED_WORKTREE_STATE;
- UNTRACKED_WORKTREE_STATE;
- SUBMODULE_STATE;
- SPARSE_CHECKOUT_STATE;
- CONFIG_GENERATION;
- SECURITY_POLICY;
- PROJECT_ASSOCIATION.

Diagnostic metadata such as observation timestamps is excluded from semantic identity by default.

#### WorkspaceBasisFingerprint
DCS/DIF-derived fingerprint over correctness-relevant WorkspaceBasis material. M02 MUST reuse M01 canonical identity primitives.

#### WorkspaceBasisDiff
Typed old/new delta:
- changed component mask;
- old/new basis fingerprints;
- identity/security hard-invalidations;
- state invalidations;
- association changes;
- diagnostic-only changes;
- evidence refs.

#### ProjectAssociationEvidence
External project-association statement with:
- provider origin;
- HIVE project reference when present;
- provider/contract version;
- evidence freshness/generation;
- asserted repository/workspace hints;
- provenance;
- no local authority grant.

#### ReconciliationReceipt
Result of comparing user/config intent, local filesystem/Git evidence and optional HIVE evidence.

Candidate statuses:
- CONSISTENT;
- STANDALONE_VERIFIED;
- PARTIAL;
- CONFLICT;
- INSUFFICIENT.

HIVE association has an orthogonal status:
- MATCH;
- STALE;
- CONFLICT;
- UNAVAILABLE;
- NOT_REQUESTED.

No authority domain may overwrite another domain's contradictory evidence.

#### PathValidationRequest
Contains:
- WorkspaceHandle reference;
- requested logical path;
- operation class;
- required authority root;
- existence expectation;
- path policy generation.

#### ValidatedPathReceipt
Contains:
- logical requested path;
- lexical normalized path;
- authority root identity;
- nearest existing physical ancestor;
- resolved physical target when it exists;
- symlink/junction/reparse chain summary;
- filesystem semantics fingerprint;
- use-time revalidation requirement;
- allow/deny reason.

A ValidatedPathReceipt proves M02 validation. It is not a sandbox capability.

### Canonical Git basis

M02 must prefer semantic Git state over raw mutable files.

Candidate read-only basis:
- repository/common-dir/worktree identities;
- object format;
- HEAD object ID;
- attached/detached mode;
- symbolic HEAD ref when present;
- index semantic fingerprint from staged entries rather than raw index bytes;
- tracked dirty-set + content fingerprints for changed tracked files;
- untracked-set/content basis according to explicit policy;
- submodule/gitlink state;
- sparse-checkout declaration;
- repository config/security facts needed for safe interpretation.

Raw index mtimes/stat-cache bytes MUST NOT churn the semantic basis unless they change repository semantics.

### Untracked-file policy

M02 supports explicit policies:
- EXCLUDED_BY_POLICY;
- NAMES_ONLY;
- CONTENT_HASHED.

For an execution-ready general workspace binding, CONTENT_HASHED is the safe default for unignored untracked files unless a governed downstream profile proves a narrower policy is sufficient.

A narrower policy must be visible in WorkspaceBasis and therefore in evidence/cache validity. It may not be silently substituted to improve performance.

### Drift classes

#### HARD_IDENTITY_DRIFT
Examples:
- workspace authority root identity changed;
- repository/common-dir identity changed;
- worktree identity changed;
- filesystem semantics changed incompatibly.

Effect: prior handle is invalid; explicit rebind normally required.

#### SECURITY_DRIFT
Examples:
- authority policy changed;
- path security policy changed;
- security-relevant config generation changed.

Effect: fail closed until revalidated.

#### GIT_BASIS_DRIFT
Examples:
- HEAD/ref mode;
- index;
- tracked content;
- untracked content under policy;
- submodule;
- sparse checkout.

Effect: old handle stale; revalidation emits new generation if still bindable.

#### ASSOCIATION_DRIFT
HIVE/external project association changed, became stale or conflicts.

Effect:
- HIVE-required assurance -> block/revalidate;
- standalone-safe operation -> local binding may remain valid but association assurance degrades explicitly.

#### DIAGNOSTIC_DRIFT
Timestamp/logging/observation metadata only.

Effect: no semantic fingerprint churn.

### Revalidation outcomes

- STILL_VALID: full correctness fingerprint unchanged.
- UPDATED_COMPATIBLE: binding remains valid but new generation/handle required.
- REBIND_REQUIRED: identity/authority relationship changed.
- BLOCKED_CONFLICT: contradictory or unsafe evidence.
- INSUFFICIENT_EVIDENCE: required assurance cannot be proved.

### Reconciliation law

M02 does not use one global "source priority" list.

Instead:
- explicit user/config input expresses intended target;
- filesystem/Git evidence proves the concrete local checkout;
- HIVE proves its own registered project association;
- security policy decides minimum assurance.

A local checkout cannot be transformed into another checkout because HIVE says so.
A HIVE project association cannot be fabricated because local paths look similar.
A user-supplied path cannot override a physical authority escape.

### Path authority algorithm

Path validation has two phases.

1. Lexical phase
- parse using native platform rules;
- reject empty/invalid/device-sensitive forms according to policy;
- normalize separators/dot segments without blindly lowercasing;
- reject lexical escape outside the declared logical root.

2. Physical phase
- resolve existing components;
- inspect symlink/junction/reparse transitions;
- prove the resolved target/nearest existing ancestor remains within an accepted physical authority root;
- record filesystem/case semantics;
- fail closed on unknown security-sensitive resolution.

For a non-existing target:
- validate the nearest existing ancestor physically;
- validate the remaining lexical suffix;
- mark the receipt `use_time_revalidation_required=true`;
- later mutation/sandbox modules MUST revalidate at the use boundary because M02 cannot eliminate TOCTOU.

### Cross-platform path rules

Unix:
- symlink loops fail typed;
- mount/device boundary metadata is observable where available;
- M02 path validation is not a mount namespace sandbox.

Windows:
- drive-relative paths such as `C:foo` are rejected by default;
- UNC paths require an explicitly admitted UNC authority root;
- device/NT namespace escapes are denied unless a future governed policy explicitly supports them;
- junctions/reparse points are resolved under policy;
- case folding is based on detected filesystem/root semantics, never unconditional lowercase;
- alternate data stream/device-name handling is fail-closed unless explicitly supported.

### Git Safe Inspection candidate profile

M02 read-only Git inspection SHOULD use an allowlisted argv-based adapter, never shell command strings.

Hardening candidates:
- no network commands;
- `GIT_TERMINAL_PROMPT=0`;
- optional locks disabled where correctness permits;
- external diff disabled;
- fsmonitor hooks disabled for deterministic inspection;
- bounded stdout/stderr;
- bounded command deadlines;
- explicit repository/worktree arguments;
- credential-bearing remote URLs redacted before evidence;
- no broad environment inheritance beyond an allowlist.

The implementation backend (system Git adapter versus a Rust-native Git library) remains evidence-driven until file-map/benchmark freeze.

### Round 2 technology candidates

#### BVM - Basis Validity Matrix
Maps downstream operation classes to the WorkspaceBasis component masks they require.

Expected benefit:
- targeted stale detection and cache/evidence invalidation;
- prevents irrelevant workspace metadata from forcing broad recomputation.

Risk:
- an incomplete requirement mask could create false freshness.

Promotion criterion:
- property/fixture tests show BVM-selective invalidation is never weaker than full correctness validation for registered operation classes.

#### GSI - Git Safe Inspection
A hardened read-only Git inspection profile/adapter with explicit commands, bounded outputs and no interactive/network behavior.

Expected benefit:
- deterministic Git semantics without giving M02 mutation or network authority.

Risk:
- unusual Git configurations/platform behavior.

Promotion criterion:
- fixture/adversarial matrix proves required repository/worktree facts without modifying index/worktree or leaking credentials.

#### ACR - Authority Chain Receipt
Compact proof of the logical-to-physical path validation chain.

Expected benefit:
- downstream modules can reason about why a path was accepted/denied without repeating large path-resolution logs.

Risk:
- receipt could become stale after filesystem changes.

Promotion criterion:
- path-swap/symlink/junction tests prove stale receipts are detected by use-time revalidation.

#### FSC - Filesystem Semantics Capsule
Captures correctness-relevant root semantics such as case behavior, path namespace class, volume/device identity hints and reparse/symlink support.

Expected benefit:
- stable cross-platform path identity and fewer false alias/collision decisions.

Risk:
- platform probing complexity.

Promotion criterion:
- Windows/Unix fixture tests demonstrate stable alias handling and explicit unsupported/unknown states.

### Round 2 required tests

- state transition matrix and illegal transitions;
- WorkspaceHandle cannot survive epoch mismatch;
- durable receipt cannot be used as live authority;
- linked worktrees share RepositoryId and differ in WorktreeId;
- remote URL change alone does not silently rewrite RepositoryId;
- raw index stat-only churn does not change semantic index fingerprint;
- staged-content change does change index fingerprint;
- untracked content change invalidates CONTENT_HASHED basis;
- HIVE outage does not fabricate/change WorkspaceId;
- HIVE conflict blocks HIVE-required assurance;
- standalone binding stays explicit when HIVE unavailable;
- lexical path escape rejected;
- physical symlink/junction escape rejected;
- non-existing target requires use-time revalidation;
- path target swapped after receipt is detected;
- WorkspaceBasisDiff reconstruction equals full basis recomputation;
- diagnostic-only changes do not churn basis identity;
- security policy changes always invalidate relevant handles.

### Round 2 unresolved items

Still to freeze:
- exact Rust crate/file boundaries;
- system Git versus Rust-native Git implementation backend;
- repository graph representation details;
- submodule recursion/depth policy;
- large/untracked-file hashing resource budgets;
- file watcher/event optimization versus on-demand revalidation;
- exact BVM operation classes;
- HIVE association adapter contract placement relative to M23;
- final threat model/fuzz corpus;
- benchmark seed policy and M02 DoD.


## Round 3 - Repository graph, threat model and external association seam

### Repository/worktree graph model

M02 represents repository topology as a typed graph instead of assuming one workspace equals one Git root.

Node classes:
- WORKSPACE_ROOT;
- REPOSITORY;
- WORKTREE;
- SUBMODULE_DECLARATION;
- SUBMODULE_WORKTREE;
- NESTED_REPOSITORY;
- GIT_METADATA_ROOT;
- EXTERNAL_OBJECT_STORE.

Edge classes:
- CONTAINS;
- WORKTREE_OF;
- DECLARES_SUBMODULE;
- MATERIALIZES_SUBMODULE;
- NESTED_IN;
- USES_GIT_METADATA;
- USES_OBJECT_STORE;
- PRIMARY_BINDING.

Graph invariants:
- every worktree references exactly one RepositoryId;
- linked worktrees may share RepositoryId while retaining unique WorktreeId;
- nested independent repositories are not silently converted into submodules;
- submodule declarations and initialized submodule worktrees are distinct facts;
- source authority and Git metadata authority are separate;
- graph cycles caused by malformed metadata/path indirection fail typed;
- graph traversal is depth/size bounded by policy.

### Authority classes

M02 distinguishes authority classes so discovery does not accidentally grant execution rights.

#### SOURCE_AUTHORITY
Paths downstream source operations may refer to after successful PAF validation.

#### GIT_METADATA_AUTHORITY
Read-only Git administration paths required to interpret a repository/worktree, including linked-worktree common metadata that may physically live outside SOURCE_AUTHORITY.

This authority:
- permits only M02's bounded metadata inspection;
- is never inherited automatically as source/mutation authority;
- must be recorded in the binding receipt.

#### EXTERNAL_OBJECT_AUTHORITY
Optional read-only object-store paths used by Git alternates/shared object databases.

Default execution-ready policy is DENY unless explicitly admitted with provenance. If allowed, external object roots participate in the security/basis fingerprint.

#### INTERNAL_CORE_TEMP
Ephemeral CORE-owned scratch/state outside the workspace, when needed for bounded streaming/sorting/fixtures. It may never be represented as project source authority.

### Worktree and .git indirection

M02 must support:
- normal .git directory;
- .git file pointing to a worktree gitdir;
- git-common-dir outside the worktree root;
- bare repository;
- detached HEAD;
- linked worktrees.

A .git indirection path is treated as untrusted metadata:
1. parse without shell;
2. resolve physically;
3. classify as GIT_METADATA_AUTHORITY;
4. validate against Git-reported/common-dir facts;
5. never promote it into SOURCE_AUTHORITY merely because Git uses it.

### Submodule policy

M02 never auto-initializes, fetches, updates or recursively clones submodules.

It records:
- gitlink entries from the parent repository basis;
- declared submodule path/name;
- declared URL only as redacted association metadata;
- initialized/uninitialized status;
- initialized child repository/worktree identity where locally present.

Submodule paths are passed through PAF.

Recursion is bounded by:
- maximum configured depth;
- maximum repository/node count;
- cycle detection;
- cancellation/deadline.

An uninitialized submodule is valid repository state, not an automatic error. Downstream operation requirements decide whether it is sufficient.

### Nested repository policy

An independent repository physically under another workspace path is represented as NESTED_REPOSITORY, not merged into the parent's Git state.

Workspace policy decides whether it is:
- admitted as a secondary repository;
- ignored as outside the requested repository set;
- a boundary conflict.

The decision and evidence appear in WorkspaceBindingReceipt.

### Bare repository policy

A bare repository may be bound for operations whose Basis Validity Matrix does not require a worktree.

It cannot satisfy a source-worktree operation merely because Git metadata exists.

### HIVE project association capability seam

M02 consumes project association through a versioned capability rather than importing HIVE code.

Candidate capability identity:
`nexlabs.project-association@1`

Request fields:
- local WorkspaceId;
- repository/worktree evidence summary;
- optional configured HIVE project reference;
- local basis/provenance fingerprint;
- required freshness/assurance.

Response fields:
- provider origin/version;
- HIVE project reference when available;
- asserted repository/workspace hints;
- association generation/fingerprint;
- freshness metadata;
- provenance;
- status/evidence refs.

Rules:
- provider unavailable -> explicit UNAVAILABLE;
- response cannot grant local path authority;
- response cannot mutate local WorkspaceId;
- conflicting HIVE/local evidence produces reconciliation conflict under policy;
- M23 may later provide a deeper federation implementation without changing the M02 consumer contract.

### HIVE disconnect/reconnect semantics

Temporary HIVE provider loss does not change WorkspaceId or RepositoryId.

If the current handle only requires standalone assurance:
- local binding may remain BOUND;
- project association health becomes UNAVAILABLE/STALE;
- the binding receipt/health view reflects degradation.

If an operation requires HIVE_RECONCILED assurance:
- existing local handle may remain structurally valid;
- operation admission is blocked until fresh association evidence returns.

Reconnect:
- new association evidence is reconciled;
- identical association may refresh freshness without local identity churn;
- changed/conflicting association produces ASSOCIATION_DRIFT and revalidation.

### Threat model

M02 treats the workspace, repository metadata and external association data as potentially hostile inputs.

#### T1 Path traversal / namespace escape
Attack:
- .. segments, alternate separators, device namespaces, UNC tricks.

Control:
- PAF lexical + physical validation;
- platform-specific namespace classification;
- fail closed on unknown security-sensitive forms.

#### T2 Symlink/junction/reparse swap
Attack:
- accepted path is swapped after validation.

Control:
- ACR records chain/ancestor evidence;
- use-time revalidation flag;
- downstream action boundary must revalidate/enforce.

#### T3 Malicious .git indirection
Attack:
- .git file points outside expected metadata roots or to malformed location.

Control:
- separate GIT_METADATA_AUTHORITY;
- physical resolution/provenance;
- no source authority inheritance.

#### T4 Hostile Git config/helpers
Attack:
- fsmonitor hook, external diff/textconv, pager/editor, credential prompt or helper side effect.

Control target:
- GSI command profile;
- no shell;
- allowlisted read-only commands;
- disable interactive/network behavior;
- disable external diff/textconv/fsmonitor for inspection paths where applicable;
- bounded environment and output.

#### T5 Credential-bearing repository metadata
Attack:
- remote URLs or config contain credentials/tokens.

Control:
- redact URL userinfo/secrets;
- avoid broad config serialization;
- secret canary tests.

#### T6 Output/resource bomb
Attack:
- huge status, pathological path counts, oversized config/output.

Control:
- streaming bounded readers;
- output/entry/depth budgets;
- cancellation/deadlines;
- no unbounded Vec of attacker-controlled records.

#### T7 Submodule recursion/network surprise
Attack:
- malicious submodule graph triggers fetch/recursive explosion.

Control:
- no fetch/init/update;
- bounded local-only traversal;
- path/URL treated as untrusted metadata.

#### T8 Case/normalization collision
Attack:
- two logical paths alias on case-insensitive or normalization-sensitive filesystems.

Control:
- FSC records known/unknown semantics;
- no unconditional lowercase;
- collision tests;
- fail closed when safe identity cannot be proved.

#### T9 External object-store escape
Attack:
- Git alternates/shared object DB points outside admitted metadata authority.

Control:
- clear untrusted environment alternates;
- detect repository alternates where possible;
- explicit EXTERNAL_OBJECT_AUTHORITY policy/provenance.

#### T10 HIVE stale/conflicting association
Attack/failure:
- stale HIVE record points at another project/repository.

Control:
- BRL reconciliation;
- freshness generation;
- no HIVE overwrite of local facts.

#### T11 Concurrent workspace drift
Attack/failure:
- local user/process changes HEAD/index/files after preflight.

Control:
- WDG generation/fingerprint checks;
- action-boundary freshness validation;
- old handle never revived.

#### T12 Repository graph confusion
Attack:
- nested repo/submodule/worktree is misclassified, broadening authority.

Control:
- RBR typed graph;
- explicit boundary policy;
- fixture/fuzz matrix.

### Git inspection security contract

M02 defines a `GitInspector` interface. Backend choice remains evidence-driven, but every backend must satisfy the same contract.

Required properties:
- read-only;
- no network;
- no shell command strings;
- no interactive prompt;
- cancellation/deadline aware;
- bounded stdout/stderr/record count;
- explicit repository/worktree target;
- stable machine-readable parsing;
- credential redaction;
- no index/worktree mutation;
- version/provenance recorded.

System-Git backend hardening candidates:
- explicit argv only;
- `GIT_TERMINAL_PROMPT=0`;
- pager/editor/askpass disabled;
- optional locks disabled when safe;
- external diff/textconv disabled;
- fsmonitor disabled for deterministic status inspection;
- literal pathspec behavior where paths are supplied;
- no fetch/pull/remote contact;
- bounded process output with kill on policy breach;
- environment allowlist/clearing of alternate-object environment variables.

Rust-native backend candidates must meet identical fixtures and security tests. Backend selection is frozen only after benchmark/security evidence.

### Filesystem Semantics Capsule states

FSC must avoid pretending every platform/root has known case behavior.

Candidate states:
- CASE_SENSITIVE_VERIFIED;
- CASE_INSENSITIVE_VERIFIED;
- CASE_PRESERVING_UNKNOWN;
- UNKNOWN_UNSAFE_FOR_ALIAS_DECISION.

FSC may also record:
- volume/device identity hints;
- root path namespace class;
- symlink/reparse support;
- canonical path evidence source;
- normalization policy version.

For existing paths, physical/file identity evidence may disambiguate aliases even when general filesystem behavior is unknown.
For non-existing security-sensitive paths, unknown alias semantics may require blocking or downstream use-time proof.

### Large-workspace algorithm direction

M02 MUST avoid full content rescans when Git/local delta evidence can safely narrow work.

Candidate pipeline:
1. establish stable repository/worktree identity;
2. collect semantic HEAD/index state;
3. obtain tracked/untracked changed sets;
4. hash only content whose correctness identity requires it;
5. update component-level basis;
6. emit WorkspaceBasisDiff;
7. compare against full recomputation in verification/soak fixtures.

Content hashing:
- streaming buffers;
- bounded concurrency;
- cancellation-aware;
- content size does not translate to equivalent RAM allocation.

### New technology candidate: WMF - Workspace Merkle Forest

**Problem:** large workspaces can make monolithic basis hashing expensive and cause broad invalidation.

**Mechanism:** component/repository/path-partitioned Merkle-style fingerprints whose root forms part of CWB while subroots identify targeted change radius.

**Expected benefit:**
- faster DWS recomputation;
- targeted cache/evidence invalidation;
- compact handles for large path sets;
- less repeated context material.

**Primary risks:**
- complexity;
- canonical ordering mistakes;
- accidental false freshness if partitions are incomplete.

**Promotion criterion:**
- root equality with canonical full-basis fingerprint semantics;
- property tests for insert/delete/rename/content change;
- delta vs full reconstruction equivalence;
- benchmark advantage on large changed-path fixtures without unacceptable memory cost.

### Basis Validity Matrix initial operation classes

Discovery-only initial classes:
- READ_METADATA;
- READ_SOURCE;
- PLAN_WORK;
- EXECUTE_TOOL_READONLY;
- MUTATE_SOURCE;
- GIT_DELIVERY;
- HIVE_RECONCILED_OPERATION.

M02 does not own these actions. BVM only defines which basis components later modules must prove fresh before those actions.

Example direction:
- READ_METADATA may not require untracked content hashes;
- READ_SOURCE requires source/path/security basis;
- MUTATE_SOURCE requires authority + Git/source basis + use-time path revalidation;
- GIT_DELIVERY additionally requires HEAD/index/worktree/ref basis;
- HIVE_RECONCILED_OPERATION additionally requires fresh project association evidence.

Masks remain discovery candidates until downstream M03/M11/M13/M20 contract review.

### Round 3 resource invariants

- repository/submodule graph node count bounded;
- recursion depth bounded;
- process output bounded;
- command duration bounded;
- content hashing memory bounded;
- concurrency bounded;
- no network request by M02 Git inspection;
- no automatic repository repair;
- no mutation to index/worktree/config;
- no LLM inference;
- no repository-wide semantic AST/RAG analysis.

### Round 3 verification additions

Fixtures:
- linked worktree with common gitdir outside source root;
- .git file with malformed/escaping target;
- nested independent repo;
- initialized and uninitialized submodule;
- submodule path traversal attempt;
- bare repo;
- alternate object store inside/outside admitted policy;
- hostile remote URL with embedded canary credential;
- repo config with fsmonitor/external diff canaries;
- huge command output / path-count cap;
- case-collision fixtures on supported platforms;
- HIVE association reconnect same/different/conflict.

Properties:
- source authority never expands from metadata authority;
- HIVE evidence never grants path authority;
- no Git backend command has network/mutation classification;
- bounded parser rejects over-policy output without partial success;
- WMF/DWS root equals full canonical basis semantics;
- cancellation leaves no false BOUND receipt.

### Round 3 unresolved items

Still to freeze:
- GitInspector production backend;
- exact metadata commands/API set;
- exact authority-root structures/contracts;
- repository graph serialization;
- object-store policy defaults;
- filesystem-semantics probing implementation;
- watcher/event strategy;
- hashing/cache storage strategy;
- exact resource limits after benchmarks;
- final crate/file map;
- final technology disposition;
- DoD and Work Order.
