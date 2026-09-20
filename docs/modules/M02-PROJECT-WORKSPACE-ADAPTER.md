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


## Round 4 - Evidence engine, invalidation, cache and backend evaluation

### Round 4 objective

Round 4 freezes how M02 observes workspace change efficiently without weakening correctness.

The central rule is:

> events and caches may tell M02 what to re-check, but only deterministic revalidation may prove what is currently true.

This round deliberately does **not** select the final `GitInspector` production backend. CORE-D-064 remains binding: backend selection requires comparative security/compatibility/performance evidence.

### Evidence pipeline

M02 uses a four-stage evidence pipeline:

```text
OS/Git change hints
      |
      v
EIS Event Invalidation Spine
      |
      v
CIG Causal Invalidation Graph
      |
      +--> dirty WorkspaceBasis component mask
      |
      v
deterministic revalidation
      |
      +--> GitInspector
      +--> Path/FSC proof
      +--> bounded content hashing
      +--> optional HIVE association refresh
      |
      v
CWB / WorkspaceBasisDiff
      |
      +--> PEC proof cache
      +--> new WorkspaceHandle generation when required
```

No watcher event, cache entry or timestamp alone can transition a workspace to BOUND.

### EIS - Event Invalidation Spine

**Problem:** full workspace rescans on every action waste I/O and CPU, while filesystem watchers can drop, coalesce or reorder events.

**Mechanism:** platform watcher/Git metadata events are normalized into non-authoritative invalidation hints.

Candidate event classes:
- PATH_CREATED;
- PATH_REMOVED;
- PATH_RENAMED;
- PATH_CONTENT_CHANGED;
- PATH_METADATA_CHANGED;
- GIT_HEAD_HINT;
- GIT_INDEX_HINT;
- GIT_REFS_HINT;
- GIT_METADATA_HINT;
- AUTHORITY_ROOT_HINT;
- FILESYSTEM_SEMANTICS_HINT;
- ASSOCIATION_HINT;
- OVERFLOW_OR_LOSS.

Each hint carries:
- WorkspaceId;
- optional RepositoryId/WorktreeId;
- normalized path handle when safe;
- event class;
- provider identity/version;
- monotonic local sequence when available;
- observation time as diagnostic only;
- overflow/loss indicator.

Rules:
- events only mark basis components DIRTY/UNKNOWN;
- event loss/overflow broadens invalidation rather than pretending no change;
- duplicate/coalesced events are safe;
- event ordering is not a correctness dependency;
- a watcher provider may be disabled with no loss of correctness, only performance.

### CIG - Causal Invalidation Graph

**Problem:** one changed fact can invalidate several basis components and downstream cache classes.

**Mechanism:** a deterministic graph maps evidence classes to affected WorkspaceBasis components and operation validity masks.

Example edges:
- HEAD/ref hint -> HEAD_STATE + REPOSITORY_GRAPH checks as policy requires;
- index hint -> INDEX_STATE;
- tracked source hint -> TRACKED_WORKTREE_STATE;
- untracked source hint -> UNTRACKED_WORKTREE_STATE under policy;
- .git/common-dir hint -> REPOSITORY_GRAPH + WORKTREE_IDENTITY + GIT_METADATA authority;
- authority-root hint -> AUTHORITY_ROOTS + FILESYSTEM_SEMANTICS + security hard invalidation;
- configuration generation -> CONFIG_GENERATION;
- security policy generation -> SECURITY_POLICY;
- HIVE provider generation -> PROJECT_ASSOCIATION.

CIG output is a component mask plus reason/provenance. It never produces ALLOW directly.

### PEC - Proof Economy Cache

**Problem:** repeated deterministic proofs still cost CPU/I/O and generate large repeated evidence/context.

**Mechanism:** cache compact proof objects, not mutable source truth.

Minimum cache identity:
- schema/algorithm version;
- WorkspaceId;
- runtime epoch where relevant;
- WorkspaceGeneration/basis component generation;
- authority-root identity;
- provider/backend identity + version;
- policy/config/security generation;
- filesystem semantics capsule fingerprint;
- normalized path/repository/worktree identity;
- evidence-specific semantic identity;
- hash algorithm/version when content-derived.

Cache entry result classes:
- HIT_VALIDATED;
- MISS_NOT_FOUND;
- BYPASS_SECURITY;
- BYPASS_UNKNOWN_SEMANTICS;
- BYPASS_POLICY;
- INVALIDATED_EVENT;
- INVALIDATED_GENERATION;
- INVALIDATED_PROVIDER;
- INVALIDATED_AUTHORITY;
- INVALIDATED_SECURITY;
- EXPIRED_DIAGNOSTIC_ONLY.

Rules:
- cache state is derived and disposable;
- cache corruption/loss cannot change canonical repository truth;
- a cache hit is usable only after its identity preconditions are rechecked;
- mtime alone is never sufficient proof of unchanged correctness-relevant content;
- unknown security-sensitive filesystem semantics force bypass or stronger proof;
- no credential-bearing Git/config material enters cache keys or payloads;
- cache hit/miss/bypass reasons are machine-readable for later performance/token accounting.

### Cache tiers

#### L1 epoch proof cache

Required candidate:
- in-memory;
- bounded;
- process/runtime-epoch scoped;
- deterministic eviction policy not required for correctness;
- coalesces identical concurrent proof requests.

#### L2 persistent proof cache

Deferred candidate:
- optional optimization only;
- never required for correctness;
- must be schema/version/provenance keyed;
- must support safe invalidation after dependency/security/provider changes;
- cannot be promoted until recovery, corruption and secret-leak tests exist.

Round 4 does not authorize a persistent cache database.

### BHC - Bounded Hash Conveyor

**Problem:** CONTENT_HASHED untracked/worktree policies can create expensive duplicate hashing and memory pressure.

**Mechanism:** a bounded asynchronous hashing conveyor:
- streams file content in fixed bounded buffers;
- globally caps active hash tasks by policy;
- coalesces identical in-flight hash requests;
- supports cancellation/deadline;
- emits content digest + algorithm/version + path/file-identity proof;
- never allocates proportional RAM to input size;
- does not follow a changed symlink/reparse chain without fresh PAF proof.

Hash reuse candidates:
- Git object IDs may satisfy content identity only for facts Git semantically proves, such as committed blobs/index entries;
- dirty worktree/untracked content requires local content proof according to policy;
- filesystem metadata may nominate a cache candidate but cannot alone prove equivalence.

### AuthorityRoot contract freeze direction

Candidate canonical structure:

```text
AuthorityRoot {
  authority_root_id
  class
  logical_root
  canonical_existing_root
  physical_root_identity
  filesystem_semantics_fingerprint
  provenance
  policy_generation
  externality
  allowed_inspection_profile
}
```

Authority classes remain:
- SOURCE_AUTHORITY;
- GIT_METADATA_AUTHORITY;
- EXTERNAL_OBJECT_AUTHORITY;
- INTERNAL_CORE_TEMP.

Invariants:
- authority is explicit and non-transitive;
- an authority edge describes topology, never grants a broader class;
- SOURCE_AUTHORITY cannot be inferred from GIT_METADATA_AUTHORITY;
- EXTERNAL_OBJECT_AUTHORITY is deny-by-default for execution-ready binding;
- INTERNAL_CORE_TEMP cannot appear as project source;
- policy/security generation changes invalidate all affected authority receipts.

### Deterministic repository graph serialization

Repository graph evidence is serialized canonically:
- nodes sorted by typed stable identity;
- edges sorted by (edge class, source id, target id);
- no map/hash iteration order enters DCS;
- diagnostic timestamps excluded;
- redacted remote association hints excluded from local RepositoryId;
- schema version participates in the fingerprint.

Graph deltas must reconstruct to the same canonical root as full graph recomputation.

### GitInspector backend evaluation contract

Round 4 freezes a differential evaluation harness rather than choosing a winner.

#### Semantic reference oracle

The installed Git executable is the **test/reference semantic oracle** for Git behavior because downstream repositories are Git repositories and compatibility must match Git semantics.

This does not automatically make the system-Git adapter the production backend.

Reference facts include, where supported:
- repository/worktree/common-dir discovery;
- bare/worktree status;
- object format;
- HEAD/ref/object identity;
- index semantic entries;
- porcelain-v2 worktree status;
- submodule/gitlink state;
- sparse-checkout declarations;
- local config facts explicitly admitted by policy.

Reference invocation rules:
- argv only, no shell;
- no network operations;
- no interactive prompts;
- bounded output and deadline;
- sanitized environment;
- explicit repository/worktree target;
- no mutation/repair commands.

#### Candidate production providers

At minimum:
1. hardened system-Git provider;
2. Rust-native provider candidate (for example a gix-class implementation) where feature coverage is sufficient.

Every provider must emit the same canonical `GitEvidence` contract.

Provider promotion requires:
- semantic equivalence fixture pass;
- hostile config/path safety pass;
- no-network/no-mutation proof;
- cancellation/resource-bound proof;
- Windows + Unix coverage;
- cold/warm/large-workspace benchmarks;
- dependency/supply-chain review.

A hybrid provider is permitted only if its boundary is explicit and differential tests prove equivalent canonical evidence.

### SPO - Semantic Provider Oracle

**Problem:** provider-specific behavior can silently change workspace fingerprints.

**Mechanism:** differential fixture runner compares provider output after canonicalization against the Git semantic reference oracle.

Results:
- EQUIVALENT;
- EQUIVALENT_WITH_DIAGNOSTIC_DELTA;
- UNSUPPORTED_CAPABILITY;
- SEMANTIC_MISMATCH;
- SECURITY_VIOLATION;
- RESOURCE_POLICY_VIOLATION.

Any semantic/security mismatch blocks provider promotion for the affected capability.

### Watcher strategy

Watcher/event providers are optimization-only.

Candidate adapters:
- Linux inotify-class;
- Windows ReadDirectoryChangesW-class;
- macOS FSEvents-class when macOS support is admitted;
- portable polling fallback only as an optional hint provider, never as the freshness proof.

Correctness path:
1. action requests validity;
2. CIG determines required basis mask;
3. dirty/unknown components are revalidated;
4. even when no watcher event exists, generation/provider/policy preconditions are checked;
5. security-sensitive action boundaries may force targeted physical/path revalidation regardless of clean hints.

Overflow/loss:
- marks affected scope UNKNOWN;
- invalidates relevant PEC entries;
- triggers bounded targeted/full recomputation according to scope;
- never reports a clean workspace solely because the watcher recovered.

### Token/context economy

M02 does not call an LLM, but its outputs are designed to reduce later LLM context cost.

Downstream context should prefer:
- WorkspaceId;
- WorkspaceGeneration;
- WorkspaceBasisFingerprint;
- compact component mask;
- WorkspaceBasisDiff summary;
- repository graph root/subroot IDs;
- cache/invalidation reason;
- evidence references.

Large path inventories, Git status payloads and raw watcher streams stay out of normal model context unless explicitly requested.

This creates a stable cacheable prefix for later HIVE/CORE planning and lets downstream systems request only changed evidence.

### ResourceBudget contract

Round 4 freezes resource dimensions, not fabricated numeric limits.

`WorkspaceResourceBudget` must include:
- max repository graph nodes;
- max recursion depth;
- max Git process duration;
- max stdout bytes;
- max stderr bytes;
- max parsed records;
- max concurrent Git inspectors;
- max concurrent hash tasks;
- max single-file hash bytes before explicit policy decision;
- max aggregate hash bytes per validation attempt;
- max cache entries/bytes;
- max watcher backlog/hints;
- max revalidation wall-clock budget.

Exact defaults are frozen only after the M02 benchmark fixture suite establishes compatible baselines. A timeout/limit breach is typed and never converted into partial BOUND success.

### Round 4 technology disposition

Promote to REQUIRED DESIGN MECHANISM:
- EIS Event Invalidation Spine;
- CIG Causal Invalidation Graph;
- PEC Proof Economy Cache L1;
- BHC Bounded Hash Conveyor;
- SPO Semantic Provider Oracle/differential harness.

Remain EXPERIMENTAL/CONDITIONAL:
- persistent PEC L2;
- WMF acceleration;
- Rust-native Git provider;
- hybrid Git provider;
- platform watcher implementations beyond supported CI platforms.

Technology branding never bypasses evidence requirements.

### Round 4 verification additions

Properties:
- no event sequence can directly produce BOUND;
- dropping all watcher events preserves correctness after action-boundary revalidation;
- watcher overflow broadens invalidation;
- cache loss/corruption cannot create a valid handle;
- mtime-only changes/reuse never falsely prove content equality;
- content change with preserved timestamp is detected when content proof is required;
- identical concurrent hash/proof requests coalesce without changing output;
- cancellation releases permits and emits no partial proof;
- CIG selective invalidation is never weaker than full required-mask validation;
- graph delta reconstruction equals full canonical graph root;
- provider canonical evidence equals reference oracle for supported fixtures;
- provider mismatch cannot be hidden by cache;
- security/policy/provider version change invalidates affected cached proof;
- compact evidence references reconstruct to canonical source/evidence without embedding raw repository contents.

Adversarial fixtures:
- watcher event loss/overflow;
- rename storms;
- timestamp-preserving content rewrite;
- case-only rename;
- symlink/junction swap during hash request;
- Git index replacement;
- hostile Git config with helpers/pagers/diff/textconv;
- oversized porcelain output;
- cache entry with old policy/provider generation;
- corrupted persistent-cache candidate record;
- provider semantic disagreement.

Benchmarks:
- cold attach with cache empty;
- warm no-change revalidation;
- 1-file / 10-file / 1,000-file delta;
- untracked CONTENT_HASHED sets;
- concurrent duplicate hash requests;
- watcher-hinted vs no-watcher revalidation;
- system-Git vs Rust-native candidate equivalence/performance;
- memory ceiling under large status/hash fixtures.

### Round 4 unresolved items

Still to freeze before M02 planning STOP CONDITION:
- final GitInspector production provider disposition from evidence;
- exact numeric resource defaults;
- exact Rust crate/file map and dependency graph;
- repository graph wire/schema representation;
- exact FSC platform probing implementation;
- persistent cache disposition;
- final BVM operation masks after downstream contract review;
- final technology disposition after benchmarks;
- M02-specific DoD;
- frozen Work Order, executor packet and FINAL STOP CONDITION.

M02 implementation remains unauthorized.


## Round 5 - Contract freeze, file map and dependency boundary

### Round 5 objective

Round 5 turns the M02 architecture into executor-addressable contracts without authorizing implementation.

It freezes:
- the v1 public data model;
- canonical repository-graph wire semantics;
- Basis Validity Matrix profiles;
- Filesystem Semantics Capsule behavior;
- the V0.0 Git provider baseline rule;
- persistent-cache disposition;
- the exact initial Rust crate/file map;
- dependency constraints and no-new-dependency default.

Exact numeric resource defaults remain benchmark-calibrated under CORE-D-080 and are intentionally deferred to the evidence-calibration round.

### M02 v1 contract envelope

Every durable/external M02 contract uses an explicit envelope:

```text
M02Envelope<T> {
  schema: "nexlabs.core.workspace"
  version: 1
  kind
  payload: T
}
```

Rules:
- unsupported schema/version fails typed;
- canonical fingerprints operate on the semantic payload plus schema/version;
- diagnostic timestamps, wall-clock durations and human display strings are excluded from semantic identity;
- unordered collections are sorted canonically before DCS/fingerprint;
- secret-bearing raw Git config/remotes are never serialized into durable contracts;
- forward evolution uses a new version, not silent field reinterpretation.

### Frozen identity contracts

- `ProjectBindingId`: CORE action-plane binding identity; never a HIVE ID alias.
- `WorkspaceId`: admitted local source authority + physical root identity.
- `RepositoryId`: local repository boundary/common-dir/object-format identity; remotes excluded.
- `WorktreeId`: concrete checkout identity, distinct from RepositoryId.
- `WorkspaceGeneration`: monotonic within runtime epoch; any accepted correctness-relevant basis change increments it.

All deterministic hashes reuse `core_identity::fingerprint`; M02 does not introduce a second canonical serializer/hash stack.

### Frozen public contract set

`WorkspaceAttachRequestV1`:
- explicit locator/root;
- optional expected ProjectBindingId / RepositoryId / WorktreeId;
- requested assurance profile;
- untracked policy;
- nested-repository policy;
- external-object policy;
- resource-budget profile id;
- optional HIVE association expectation.

Ambient current working directory is never implicit authority.

`AuthorityRootV1`:
- AuthorityRootId;
- authority class;
- logical + canonical existing root;
- physical root identity evidence;
- FSC fingerprint;
- provenance;
- policy/security generation;
- externality;
- inspection profile.

`RepositoryGraphV1`:
- sorted RepositoryNodeV1 list;
- sorted RepositoryEdgeV1 list;
- canonical graph fingerprint.

Node kinds:
WORKSPACE_ROOT, REPOSITORY, WORKTREE, SUBMODULE_DECLARATION, SUBMODULE_MATERIALIZATION, NESTED_REPOSITORY, GIT_COMMON_DIR, EXTERNAL_OBJECT_STORE.

Edge kinds:
CONTAINS, CHECKOUT_OF, USES_COMMON_DIR, DECLARES_SUBMODULE, MATERIALIZES, NESTED_WITHIN, USES_OBJECT_STORE.

Nodes sort by typed stable identity. Edges sort by (edge kind, source id, target id). Diagnostic timestamps are excluded.

`GitEvidenceV1`:
- provider id/version/provenance;
- RepositoryId and optional WorktreeId;
- bare/worktree mode;
- object format;
- HEAD object/ref state;
- index semantic fingerprint;
- tracked delta fingerprint;
- untracked fingerprint under explicit policy;
- submodule/gitlink evidence;
- sparse-checkout declaration;
- common-dir/git-dir identities;
- admitted external-object evidence;
- bounded diagnostic counters.

`FilesystemSemanticsCapsuleV1`:
- root physical identity;
- namespace class;
- canonical-path evidence source;
- case semantics state;
- symlink/reparse capability state;
- normalization policy version;
- evidence confidence/provenance.

Case states remain CASE_SENSITIVE_VERIFIED, CASE_INSENSITIVE_VERIFIED, CASE_PRESERVING_UNKNOWN and UNKNOWN_UNSAFE_FOR_ALIAS_DECISION. OS family alone never proves case behavior.

`WorkspaceBasisV1` component groups:
IDENTITY, AUTHORITY, REPOSITORY_GRAPH, HEAD_STATE, INDEX_STATE, TRACKED_WORKTREE_STATE, UNTRACKED_WORKTREE_STATE, FILESYSTEM_SEMANTICS, CONFIG_GENERATION, SECURITY_POLICY, PROJECT_ASSOCIATION.

Each component has schema/version, semantic fingerprint, provenance/provider generation, validity state and invalidation reasons.

`WorkspaceBasisDiffV1`:
- from/to generation;
- changed component mask;
- before/after fingerprints;
- compact reason/provenance;
- optional evidence refs;
- no raw file inventory by default.

`WorkspaceBindingReceiptV1` is durable evidence and never a live capability.

`WorkspaceHandleV1` is runtime-ephemeral and carries runtime generation/boot epoch, WorkspaceId, WorkspaceGeneration, WorkspaceBasisFingerprint and validity-mask snapshot. A receipt cannot mint a handle without fresh validation.

### BVM v1 profile freeze

BVM uses a M02-owned fixed component mask, with no third-party bitflag dependency.

READ_METADATA:
IDENTITY + REPOSITORY_GRAPH + CONFIG_GENERATION + SECURITY_POLICY.

READ_SOURCE:
IDENTITY + AUTHORITY + FILESYSTEM_SEMANTICS + TRACKED_WORKTREE_STATE + UNTRACKED_WORKTREE_STATE + CONFIG_GENERATION + SECURITY_POLICY.

PLAN_WORK:
READ_SOURCE + REPOSITORY_GRAPH + HEAD_STATE + INDEX_STATE.

EXECUTE_TOOL_READONLY:
PLAN_WORK + action-boundary path revalidation.

MUTATE_SOURCE:
PLAN_WORK + fresh AUTHORITY/FILESYSTEM_SEMANTICS + fresh TRACKED/UNTRACKED basis at admission.

GIT_DELIVERY:
IDENTITY + AUTHORITY + REPOSITORY_GRAPH + HEAD_STATE + INDEX_STATE + TRACKED_WORKTREE_STATE + UNTRACKED_WORKTREE_STATE + CONFIG_GENERATION + SECURITY_POLICY.

HIVE_RECONCILED is an assurance overlay requiring PROJECT_ASSOCIATION. BVM expresses freshness only, never permission.

### FSC v1 probing freeze

FSC is conservative and read-only:
1. physically resolve existing roots under PAF;
2. collect stable root/file identity through standard-library platform metadata where available;
3. record namespace and symlink/reparse evidence;
4. verify case semantics only with reliable evidence;
5. otherwise emit UNKNOWN;
6. never create probe files inside user source;
7. non-existing targets inherit no speculative semantics beyond the proven ancestor;
8. insufficient security-sensitive alias evidence fails closed or requires use-time proof.

No new native dependency is admitted solely for FSC in the initial Work Order. Preflight may propose one only through a governed dependency-admission delta if the pinned Rust standard library cannot satisfy a frozen proof obligation.

### V0.0 Git provider disposition

The initial M02 Work Order requires a hardened system-Git adapter behind a provider-neutral `GitInspector` trait.

It must pass GSI/SPO equivalence, security, cancellation and resource gates before promotion. This baseline choice minimizes duplicate Git semantics and additional supply-chain surface; it is not a claim that system Git is universally superior.

Rust-native/hybrid alternatives remain FUTURE/CONDITIONAL and require separate evidence plus dependency admission.

### Persistent cache disposition

M02 V0.0 uses PEC L1/runtime-epoch cache only.

Persistent L2 is FUTURE and outside the initial Work Order because correctness does not require it and its recovery/corruption/key/secret lifecycle would expand M02. No cache database dependency is admitted.

### Watcher implementation disposition

EIS/CIG and typed EventHint ingestion are required. OS watcher adapters are not required for M02 V0.0 correctness.

Initial implementation includes deterministic manual/Noop hint sources and overflow/loss semantics. Linux/Windows/macOS watcher adapters remain later optimizations, avoiding a watcher dependency before measured need.

### Exact initial crate map

New crate: `crates/core-workspace/`

```text
crates/core-workspace/
  Cargo.toml
  src/
    lib.rs
    contracts.rs
    state.rs
    identity.rs
    authority.rs
    repository.rs
    basis.rs
    association.rs
    reconcile.rs
    invalidation.rs
    cache.rs
    hashing.rs
    service.rs
    git/
      mod.rs
      system.rs
  tests/
    attach.rs
    identity.rs
    authority.rs
    repository_graph.rs
    git_system.rs
    reconciliation.rs
    invalidation.rs
    cache.rs
    hashing.rs
    drift.rs
    adversarial.rs
    fixtures/
      README.md
  benches/
    m02_workspace.rs
```

Existing files expected to change:
- `Cargo.toml`;
- `Cargo.lock`;
- `crates/core-contracts/src/lib.rs`;
- `crates/core-config/src/lib.rs`;
- `crates/core-cli/src/main.rs` only for bounded diagnostic commands required by acceptance tests;
- `fuzz/Cargo.toml`;
- `fuzz/fuzz_targets/m02_path_authority.rs`;
- `fuzz/fuzz_targets/m02_repository_graph.rs`;
- `fuzz/fuzz_targets/m02_git_evidence.rs`;
- `.github/workflows/governance.yml`;
- `docs/evidence/M02-EXECUTION-REPORT.md`.

No other crate split is authorized without Correction Delta.

### Dependency rules

Initial `core-workspace` dependencies are restricted to:
- core-contracts;
- core-identity;
- core-config;
- workspace serde;
- workspace serde_json;
- workspace sha2;
- workspace thiserror;
- workspace tokio.

Root Tokio may add only the `process` and `fs` features if M02 preflight confirms they are needed.

No gix, git2, watcher framework, database, glob/walk framework or platform FFI crate is admitted by this round.

The executor cannot add a third-party dependency for convenience. If a frozen proof obligation cannot be met, execution stops with a dependency-admission Correction Delta containing security, supply-chain and performance justification.

### Internal dependency direction

```text
core-contracts
      ^
      |
core-identity     core-config
      ^              ^
       \            /
        core-workspace
             ^
             |
       later modules
```

`core-workspace` MUST NOT depend on core-runtime, core-cli, core-health, M03+ crates or HIVE code. Runtime epoch/generation data crosses through core-contracts.

### Round 5 unresolved items at Round 5 close

At the end of Round 5, before Round 6 refinement, the remaining items were:
- benchmark/calibration evidence for exact resource defaults;
- final acceptance thresholds from that evidence;
- final WMF/DWS acceleration disposition;
- final executor Work Order and Context Lock;
- final planning audit/freeze.

M02 implementation remains unauthorized.


## Round 6 - Calibration Gate, resource policy and technology disposition

### Round 6 objective

Round 6 resolves the final planning circularity around resource defaults.

M02 cannot honestly benchmark `core-workspace` before `core-workspace` exists. Therefore the planning contract freezes **how** resource defaults are calibrated and what evidence is required, while the frozen Work Order will require the executor to produce the actual numeric defaults inside the same implementation PR before M02 can be accepted.

This refines CORE-D-080 without weakening it: numeric defaults are still evidence-backed and cannot be invented during planning.

### RCG - Resource Calibration Gate

**Problem:** fixed resource values chosen before implementation would be guesses, while runtime-unbounded behavior is unsafe.

**Mechanism:** the M02 Work Order contains a mandatory calibration gate between implementation skeleton and production-ready acceptance.

Execution sequence:

```text
Phase A: implement frozen contracts + bounded mechanisms
    |
    v
CALIBRATION_ONLY state
    |
    +--> fixture matrix
    +--> benchmark matrix
    +--> resource measurements
    +--> semantic equivalence checks
    |
    v
Calibration Evidence Bundle
    |
    +--> proposed WorkspaceResourceBudget defaults
    +--> proposed acceptance thresholds
    |
    v
bounded Calibration Delta in same Work Order/PR
    |
    v
rerun complete exact-head validation
    |
    v
eligible for governed M02 review
```

No M02 completion or production-ready claim is legal while the calibration gate is open.

### Calibration Delta authority

The frozen M02 Work Order may authorize one narrowly bounded `Calibration Delta` after measurements.

The delta MAY change only:
- numeric WorkspaceResourceBudget defaults;
- benchmark-derived acceptance thresholds;
- benchmark fixture metadata/evidence references;
- comments/documentation explaining the selected values.

The delta MUST NOT change:
- architecture;
- ownership;
- public contract meaning;
- authority classes;
- dependency graph;
- Git provider type;
- file/crate topology;
- BVM semantics;
- security invariants;
- DoD.

Any need to change those items is a normal Correction Delta and re-enters governed review.

### Calibration evidence contract

Required artifact: `docs/evidence/M02-CALIBRATION-REPORT.md`.

The report must record:
- base/head SHA;
- OS and architecture;
- Rust version;
- Git version;
- logical CPU count available to the process;
- fixture definitions and generated sizes;
- cold/warm classification;
- run count;
- median and observed range for latency;
- peak/upper-bound process memory evidence where available;
- stdout/stderr/record counts;
- hash bytes and concurrency;
- repository graph nodes/depth;
- cache hit/miss/bypass rates for benchmark scenarios;
- timeout/cancellation behavior;
- selected budget values;
- rejected candidate values and reason;
- exact commands;
- machine-readable evidence references.

No raw secrets, credential-bearing remotes or user source content may enter the calibration report.

### Benchmark protocol

Each benchmark scenario uses:
- one non-measured warm-up when a warm case is relevant;
- at least 5 measured iterations;
- deterministic synthetic/local fixtures;
- the same semantic assertions for every candidate budget;
- isolated cold and warm result classes;
- no network access.

Selection is multi-objective:
1. correctness and security are hard constraints;
2. candidates violating memory/output/deadline bounds are rejected;
3. among valid candidates, prefer the lower-resource candidate when throughput/latency is not materially improved by a higher-resource candidate;
4. cache/delta acceleration is accepted only when full-recompute equivalence holds;
5. benchmark results never weaken a security boundary.

The report must show the evidence used for the selection rather than only the chosen number.

### Calibration fixture families

Required synthetic/local fixtures:
- no-Git workspace;
- normal clean Git repository;
- dirty tracked workspace;
- untracked CONTENT_HASHED workspace;
- linked worktree;
- bare repository;
- nested independent repository;
- initialized and uninitialized submodule layouts;
- sparse checkout;
- external object-store allowed/denied cases;
- hostile config/remote redaction canaries.

Scale dimensions:
- repository graph: 1 / 10 / 100 / 1,000 nodes where fixture cost is bounded;
- changed-path metadata: 0 / 1 / 10 / 100 / 1,000 / 10,000;
- untracked files: 0 / 10 / 1,000 / 10,000;
- hashed content: small-file fanout plus at least one streaming large-file fixture;
- duplicate concurrent proof/hash requests;
- event-hinted and zero-event revalidation.

If a CI environment cannot safely reach a top scale, the report records SKIPPED_RESOURCE_BOUND with the lower proven ceiling. It must never fabricate extrapolated success.

### WorkspaceResourceBudget calibration dimensions

The executor must calibrate and then freeze exact numeric defaults for:
- max_repository_graph_nodes;
- max_recursion_depth;
- max_git_process_duration;
- max_stdout_bytes;
- max_stderr_bytes;
- max_parsed_records;
- max_concurrent_git_inspectors;
- max_concurrent_hash_tasks;
- max_single_file_hash_bytes_before_explicit_policy;
- max_aggregate_hash_bytes_per_validation;
- max_cache_entries;
- max_cache_bytes;
- max_event_hint_backlog;
- max_revalidation_wall_clock.

The configuration schema may expose governed overrides, but:
- overrides cannot disable hard safety validation;
- zero/unlimited sentinel values are forbidden for security-sensitive bounds;
- a rejected/overflowed budget yields a typed error and never partial BOUND.

### Deterministic concurrency selection

Concurrency calibration considers candidate values from 1 up to the host's `available_parallelism()` and any lower configured hard ceiling.

The selected default is the smallest valid candidate on the observed performance/resource Pareto frontier. This avoids assuming that "more threads is faster" and naturally scales from low-core machines to workstation hardware.

M02 V0.0 does not implement permanent self-tuning or background auto-benchmarking. Calibration is build/release evidence, not a hidden runtime optimizer.

### DWS disposition

**DWS - Delta Workspace Snapshot: REQUIRED for M02 V0.0.**

Reason:
- WorkspaceBasisDiff is already a frozen public concept;
- incremental invalidation is central to CPU/I/O and downstream token economy;
- correctness remains protected by equivalence-to-full-recompute tests.

V0.0 DWS is component/changed-set based and does not require a Merkle tree.

Promotion obligations:
- delta/full basis equivalence properties;
- security/policy changes force hard invalidation;
- event loss cannot hide changes;
- no delta path produces a fresher validity state than full recomputation would.

### WMF disposition

**WMF - Workspace Merkle Forest: DEFERRED / FUTURE.**

Reason:
- the V0.0 component fingerprint + DWS model already supports bounded incremental revalidation;
- WMF adds partitioning/canonical-tree complexity before a measured bottleneck exists;
- its contract can be added later without changing WorkspaceBasisV1 semantic meaning.

WMF may be reconsidered only if M02 evidence demonstrates a material scalability bottleneck that DWS cannot address within resource budgets.

No WMF-specific code/dependency enters the initial Work Order.

### Final M02 V0.0 technology disposition

REQUIRED:
- WIL Workspace Identity Lattice;
- CWB Canonical Workspace Basis;
- PAF Path Authority Firewall;
- BRL Basis Reconciliation Layer;
- WDG Workspace Drift Guard;
- RBR Repository Boundary Resolver;
- WBR Workspace Binding Receipt;
- DWS Delta Workspace Snapshot;
- BVM Basis Validity Matrix;
- GSI Git Safe Inspection;
- ACR Authority Chain Receipt;
- FSC Filesystem Semantics Capsule;
- EIS Event Invalidation Spine;
- CIG Causal Invalidation Graph;
- PEC L1 Proof Economy Cache;
- BHC Bounded Hash Conveyor;
- SPO Semantic Provider Oracle contract;
- RCG Resource Calibration Gate.

DEFERRED / FUTURE:
- WMF Workspace Merkle Forest;
- PEC persistent L2;
- Rust-native/hybrid GitInspector providers;
- OS-native watcher adapters;
- persistent self-tuning/autocalibration.

### Round 6 planning freeze result

After Round 6, architecture no longer requires guessed numeric resource values before implementation.

The final planning round must now:
- create the exact Work Order;
- create Context Lock/fingerprints;
- encode the RCG/Calibration Delta authority;
- freeze acceptance criteria and STOP CONDITION;
- perform the final cross-source audit;
- authorize M02 implementation only after APPROVED promotion of that freeze.

M02 implementation remains unauthorized.


## Final planning freeze candidate

M02 planning rounds 1-6 are complete and the executor contract is compiled.

Frozen execution artifacts:
- Work Order: `.engineering/work-orders/CORE-WO-M02-001.md`;
- Context Lock: `.engineering/context-locks/CORE-WO-M02-001.json`;
- Evidence Bundle skeleton: `.engineering/evidence/CORE-WO-M02-001.json`;
- Codex handoff: `docs/work-orders/CODEX-HANDOFF-M02.md`.

The Work Order contains:
- one-module implementation authority boundary;
- eight ordered construction packets A-H;
- HIVE preflight and SOLO degradation behavior;
- exact initial file/dependency map;
- 41 acceptance criteria;
- mandatory adversarial/property/fuzz/security/supply-chain/Windows/Ubuntu evidence;
- mandatory RCG Calibration Gate;
- bounded Calibration Delta authority;
- exact Evidence Bundle expectations;
- final READY_FOR_REVIEW / BLOCKED STOP CONDITION.

No unresolved architecture choice remains for M02 V0.0.

Execution is still unauthorized in this candidate because:
- the final planning freeze has not yet received governed exact-head approval/promotion;
- the execution `authorizedBase` cannot be bound until the promoted merge SHA exists;
- the pending Context Lock therefore remains `PENDING_PROMOTION`.

After APPROVED promotion, only a bounded execution-admission delta may:
- bind the promoted main SHA as `authorizedBase`;
- recompute relevant canonical fingerprints if promotion changed them;
- set Context Lock `status=ACTIVE`;
- set `productImplementationAuthorized=true`;
- mark `CORE-WO-M02-001` as the active Work Order.

No architecture, scope, dependency or acceptance semantics may change during that admission delta.


## Execution admission

The final M02 planning freeze was APPROVED by Review 005 (canonical Issue #35) and promoted through PR #33 at merge `bae47b2021a897396109dfcf42e8632dde13ec21`.

The bounded execution-admission delta now activates:
- `CORE-WO-M02-001`;
- the M02 Context Lock against the approved product base;
- ELEVATED assurance;
- M02-only implementation authority.

No architecture, scope, dependency, contract, acceptance criterion, technology disposition or STOP CONDITION changes in this admission.

Codex execution remains governed by Packets A-H, mandatory RCG calibration and independent exact-head review before any completion promotion.
