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
