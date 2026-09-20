# CORE-WO-M02-001 — Project / Workspace Adapter

Status: ACTIVE_AUTHORIZED
Module: M02
Executor: Codex
Risk / assurance: ELEVATED
Execution model: one comprehensive Work Order, eight ordered construction packets
Product implementation authorization: OPEN — execution authorized only from Context Lock authorizedBase `bae47b2021a897396109dfcf42e8632dde13ec21`

## OBJECTIVE

Implement the production-grade M02 Project / Workspace Adapter exactly from the frozen M02 planning basis.

M02 must provide deterministic, zero-LLM workspace attachment, identity, repository/worktree topology, authority/path proof, HIVE association reconciliation, drift detection, bounded incremental revalidation, L1 proof reuse, bounded hashing, and evidence-backed resource calibration.

Do not redesign settled architecture. Do not duplicate HIVE-owned Project Registry, RAG, memory or repository-intelligence capabilities.

## HIVE PREFLIGHT

Before product-code changes:

1. resolve exact Git remote/base/head;
2. verify the admitted Context Lock is current and not STALE;
3. read canonical sources in the required hierarchy;
4. run existing governance validation;
5. detect local HIVE v1.0.0 availability using the repository's existing integration/bootstrap path;
6. if HIVE is available, resolve CORE/project context through HIVE and record actual evidence;
7. if HIVE is unavailable, continue in bounded SOLO mode using canonical Git sources and record degraded HIVE state;
8. never fabricate HIVE health/project/index evidence;
9. emit `docs/evidence/M02-PREFLIGHT.md` before Pack A proceeds.

HIVE is intelligence/context only. It cannot grant local path authority or override contradictory local Git/filesystem evidence.

## CANONICAL BASIS

Load in this order:

1. `docs/project-brain/13-CHECKPOINT.md`
2. `docs/project-brain/16-DECISIONS-LEDGER.md`
3. `docs/project-brain/03-SCOPE.md`
4. `docs/project-brain/15-DEFINITION-OF-DONE.md`
5. `docs/project-brain/04-ARCHITECTURE.md`
6. `docs/project-brain/02-REQUIREMENTS.md`
7. `docs/project-brain/10-SECURITY-GOVERNANCE.md`
8. `docs/project-brain/11-TEST-PLAN.md`
9. `docs/modules/M02-PROJECT-WORKSPACE-ADAPTER.md`
10. `docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md`
11. this Work Order
12. admitted `.engineering/context-locks/CORE-WO-M02-001.json`

Canonical Git content outranks HIVE/cache/chat summaries.

## CONTEXT

The admitted Context Lock, canonical source hierarchy and exact Git base define executable context. Chat history, HIVE retrieval, caches and derived summaries are advisory accelerators only.

Use the canonical basis once as stable prefix, then progressively disclose only packet-relevant deltas, changed files and failing evidence.

## CONTEXT LOCK / STALENESS

Execution is illegal until `.engineering/context-locks/CORE-WO-M02-001.json` is promoted to an admitted execution lock with:
- exact authorized base SHA;
- exact relevant source fingerprints;
- `status=ACTIVE`;
- `productImplementationAuthorized=true`.

If Checkpoint, Decisions Ledger, Scope, DoD, Architecture, Requirements, Security, Test Plan, M02 module plan, this Work Order, or authorized base materially changes, mark the lock STALE and stop affected progression until recompilation/rebase.

## CONTEXT BUDGET

Treat settled M02 architecture/invariants as STABLE PREFIX.

Use progressive disclosure:
- read full canonical basis once at preflight;
- for Pack B-H, prefer fingerprints, changed files, failing evidence and relevant module subsections;
- do not repeatedly ingest the whole repository;
- deterministic Git/AST/hash/test evidence before LLM reasoning;
- use HIVE delta/context capabilities when genuinely available;
- cache/reuse valid independent evidence only when relevant inputs are unchanged.

No LLM inference is allowed in M02 runtime logic, basis computation, calibration or acceptance selection.

## RISK / ASSURANCE

Classification: ELEVATED.

Reason:
- filesystem/path authority;
- hostile repository/config parsing surface;
- stale-handle correctness;
- linked worktrees/submodules/external object-store topology;
- bounded process execution;
- later mutation/delivery modules will rely on M02 proofs.

Required assurance:
- STANDARD checks plus broad adversarial/path/repository/security regression;
- fuzz/property tests;
- Windows + Ubuntu exact-head evidence;
- supply-chain/advisory/SBOM;
- bounded resource/calibration proof;
- independent final review.

## SCOPE

NECESSARY implementation:
- `core-workspace` crate;
- v1 M02 contracts/envelopes;
- attach/revalidate/detach state machine;
- ProjectBindingId / WorkspaceId / RepositoryId / WorktreeId;
- AuthorityRootV1 + PAF + FSC;
- RepositoryGraphV1;
- hardened system-Git `GitInspector` baseline;
- HIVE association capability seam and SOLO/HIVE reconciliation;
- WorkspaceBasisV1 / WorkspaceBasisDiffV1;
- BVM freshness profiles;
- WDG stale-handle behavior;
- EIS + CIG typed invalidation;
- DWS component/changed-set incremental revalidation;
- PEC L1 runtime-epoch proof cache;
- BHC bounded hashing;
- M02 service facade;
- RCG calibration gate and final finite resource defaults;
- required CLI diagnostics only when acceptance tests need them;
- fixtures/property/fuzz/adversarial/bench/evidence.

## OUT OF SCOPE

Do not implement:
- source mutation/patch application;
- Git checkout/reset/branch/commit/merge/push/fetch;
- repository repair;
- network access to Git remotes;
- HIVE RAG/memory/Project Registry duplication;
- M11 sandbox enforcement;
- M20/M21 delivery/release behavior;
- semantic AST/repository intelligence;
- dashboard/UI/TUI;
- PEC persistent L2;
- WMF Workspace Merkle Forest;
- Rust-native/hybrid Git provider;
- OS-native watcher adapters;
- persistent runtime self-tuning/background recalibration;
- new third-party dependencies not admitted below.

## FILES / SOURCES TO READ

Existing:
- `Cargo.toml`
- `Cargo.lock`
- `crates/core-contracts/src/lib.rs`
- `crates/core-identity/src/lib.rs`
- `crates/core-config/src/lib.rs`
- `crates/core-cli/src/main.rs`
- current M01 crates only as contract consumers/reference, not dependency targets
- `fuzz/`
- `.github/workflows/governance.yml`

Create:
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

fuzz/fuzz_targets/
  m02_path_authority.rs
  m02_repository_graph.rs
  m02_git_evidence.rs

docs/evidence/
  M02-PREFLIGHT.md
  M02-CALIBRATION-REPORT.md
  M02-EXECUTION-REPORT.md
```

Permitted existing-file changes:
- root `Cargo.toml` to add `core-workspace` and only required Tokio `process` / `fs` features;
- `Cargo.lock` as deterministic consequence;
- `core-contracts` for cross-crate/versioned HIVE-association and shared receipt/envelope seams only;
- `core-config` for M02 policy + WorkspaceResourceBudget;
- `core-cli` only for bounded diagnostic commands required by acceptance;
- fuzz manifest/targets;
- governance workflow for M02 exact-head gates;
- evidence/docs required by this WO.

No additional crate split or third-party dependency without governed Correction Delta.

## DEPENDENCY RULES

`core-workspace` may depend only on:
- `core-contracts`;
- `core-identity`;
- `core-config`;
- workspace `serde`;
- workspace `serde_json`;
- workspace `sha2`;
- workspace `thiserror`;
- workspace `tokio`.

Forbidden initial dependencies include:
- `core-runtime`;
- `core-health`;
- `core-cli`;
- M03+ crates;
- HIVE source/runtime/database;
- gix;
- git2;
- watcher frameworks;
- databases/cache engines;
- glob/walk frameworks;
- platform FFI crates;
- LLM/provider SDKs.

If a frozen proof obligation cannot be satisfied with the admitted graph, STOP and raise a dependency-admission Correction Delta with minimal reproduction plus security/supply-chain/performance justification.

## REQUIREMENTS

The executor MUST satisfy every applicable accepted M02 requirement in `docs/project-brain/02-REQUIREMENTS.md`, including CORE-R-039 through CORE-R-111.

Mandatory requirement families include:
- deterministic explicit workspace attachment and identity;
- runtime-bound handle / durable receipt separation;
- semantic Git basis and repository/worktree graph correctness;
- explicit untracked/nested/external-object policies;
- lexical + physical path/authority proof;
- versioned HIVE association with no local-authority grant;
- bounded read-only/no-network Git inspection;
- conservative filesystem semantics;
- watcher-independent correctness;
- deterministic causal invalidation and DWS/full equivalence;
- disposable/provenance-aware PEC L1;
- bounded/coalesced hashing;
- explicit finite resource budgets after RCG;
- versioned contracts and canonical ordering;
- one-crate/minimal-dependency boundary;
- bounded Calibration Delta;
- zero-LLM runtime/calibration/evidence selection;
- no WMF in V0.0;
- no forced alternative provider dependency absent admission.

A requirement cannot be weakened by implementation convenience, benchmark result or cache behavior.

## ARCHITECTURE RULES

1. M02 is read-oriented and deterministic.
2. No ambient current working directory becomes implicit authority.
3. WorkspaceHandle is runtime-bound; WorkspaceBindingReceipt is durable evidence only.
4. Receipt deserialization cannot mint a live handle.
5. ProjectBinding/Workspace/Repository/Worktree identities remain distinct.
6. Semantic fingerprints reuse `core_identity::fingerprint`.
7. Remote URLs never define RepositoryId.
8. Authority classes are explicit and non-transitive.
9. Git metadata authority never grants SOURCE_AUTHORITY.
10. Path proof is not M11 sandbox permission.
11. FSC never guesses security-sensitive semantics from OS family.
12. No probe writes inside user source.
13. GitInspector uses argv/API, never shell.
14. Git inspection is read-only, no-network, non-interactive, bounded and cancellation-aware.
15. Hostile pager/helper/diff/textconv/fsmonitor behavior cannot execute through inspection.
16. Provider-specific data cannot leak into canonical GitEvidence semantics.
17. HIVE association cannot grant filesystem authority.
18. WorkspaceBasis component changes create a new WorkspaceGeneration.
19. Stale handles are never silently revived.
20. BVM expresses freshness requirements, not action authorization.
21. Watcher/event hints are non-authoritative.
22. Correctness must hold with zero watcher events.
23. DWS must be equivalent to full recomputation.
24. PEC L1 is derived/disposable and cannot become source truth.
25. mtime/stat alone cannot prove correctness-relevant content equality.
26. BHC streams content, bounds concurrency and cleans up on cancellation.
27. Resource-limit failure yields typed failure, never partial BOUND.
28. Calibration is deterministic, local/no-network and zero-LLM.
29. WMF/L2/Rust-native provider/watchers are not part of V0.0.
30. No known HIGH/CRITICAL finding may survive promotion.

## CONSTRAINTS

- Rust stable, edition/toolchain as pinned by repository.
- Safe Rust by default; any `unsafe` requires explicit inventory, justification and review.
- Headless only.
- Zero LLM inference in M02 product paths and calibration.
- No network access in M02 Git inspection or calibration fixtures.
- No destructive Git/history operations.
- No source mutation.
- No new third-party dependency outside admitted graph without governed correction.
- No ambient CWD authority.
- No user-source probe writes for FSC.
- No unlimited security-sensitive resource sentinel.
- No historical evidence substitution for changed relevant inputs.
- No advancement past a failed packet STOP.
- No implementation outside the frozen file/scope boundary except generated evidence or governed Correction Delta.
- No checkpoint promotion or implementation-PR merge by the executor.

## CONSTRUCTION PACKETS

### Pack A — Contracts, state machine and configuration

Implement:
- v1 envelope/version handling;
- public/internal contract types;
- M02 typed errors;
- attach lifecycle states;
- WorkspaceResourceBudget schema with CALIBRATION_ONLY-safe provisional bounded bootstrap values clearly marked non-final;
- BVM component masks/profiles;
- cross-crate contract seams only where necessary.

STOP A:
- contract/schema tests pass;
- unsupported versions fail typed;
- no live handle can be minted from a receipt;
- dependency graph remains allowed/acyclic.

### Pack B — Identity, authority, PAF and FSC

Implement:
- deterministic IDs;
- AuthorityRootV1;
- lexical + physical containment proof;
- symlink/junction/reparse handling using admitted platform facilities;
- conservative FilesystemSemanticsCapsuleV1;
- action-boundary revalidation primitives.

STOP B:
- alias-equivalent paths behave deterministically;
- escapes fail closed;
- unknown case/alias semantics remain UNKNOWN where not provable;
- no source-tree probe writes.

### Pack C — Repository graph and hardened system Git

Implement:
- provider-neutral GitInspector;
- hardened system-Git provider;
- RepositoryGraphV1 construction;
- normal/bare/linked-worktree/nested/submodule/sparse/external-object cases;
- canonical sorting/fingerprinting;
- hostile Git environment sanitization and output/deadline bounds.

STOP C:
- no shell/network/prompt/mutation path;
- security canaries do not execute helpers/pagers/textconv;
- graph fixtures canonicalize deterministically;
- unsupported/malformed states fail typed.

### Pack D — Association, reconciliation, basis, BVM and drift

Implement:
- HIVE association provider capability seam;
- SOLO/HIVE reconciliation;
- Canonical Workspace Basis;
- BasisDiff;
- WorkspaceGeneration;
- WDG stale handle rules;
- BVM required-mask validation;
- durable receipt creation.

STOP D:
- HIVE cannot override contradictory local evidence;
- compatible drift emits new generation;
- incompatible drift invalidates;
- required profile masks are deterministic.

### Pack E — EIS, CIG, DWS and PEC L1

Implement:
- EventHint ingestion;
- overflow/loss semantics;
- causal invalidation graph;
- component/changed-set DWS;
- runtime-epoch L1 proof cache;
- coalescing/provenance hit/miss/bypass/invalidation reasons.

STOP E:
- zero events preserve correctness;
- overflow broadens invalidation;
- selective/DWS result equals required full recomputation;
- cache corruption/loss simulation cannot create authority;
- policy/provider/security generation invalidates reuse.

### Pack F — BHC hashing and workspace service

Implement:
- bounded streaming hashing;
- equivalent in-flight request coalescing;
- cancellation/deadline permit cleanup;
- attach/revalidate/detach service facade;
- bounded diagnostic CLI only if needed by tests.

STOP F:
- timestamp-preserving content rewrites are detected where content proof is required;
- large-file hashing is bounded-memory;
- symlink/reparse swap during proof fails/revalidates safely;
- cancellation leaves no partial proof/permit leak;
- service lifecycle integration tests pass.

### Pack G — Adversarial, property, fuzz, security and compatibility

Complete:
- all unit/integration/property tests;
- adversarial fixtures;
- three M02 fuzz targets;
- Ubuntu + Windows compatibility;
- zero-LLM proof;
- dependency policy/advisory/license/SBOM;
- secret redaction checks;
- no-network/no-mutation proof.

STOP G:
- required suites pass;
- no forbidden dependency;
- no unresolved HIGH/CRITICAL finding;
- exact candidate is ready for RCG.

### Pack H — RCG calibration, final budgets and evidence

Enter `CALIBRATION_ONLY`.

Run frozen fixture/benchmark protocol and produce `docs/evidence/M02-CALIBRATION-REPORT.md`.

Authorized Calibration Delta MAY change only:
- numeric WorkspaceResourceBudget defaults;
- benchmark-derived acceptance thresholds;
- fixture metadata/evidence references;
- explanatory documentation/comments.

It MUST NOT change:
- architecture;
- ownership;
- contract meaning;
- authority classes;
- dependency graph;
- Git provider class;
- file/crate topology;
- BVM semantics;
- security invariants;
- DoD.

If evidence requires one of those changes: STOP with CORRECTION REQUIRED/BLOCKED.

After Calibration Delta:
- rerun full format/lint/type/build/tests;
- rerun properties/adversarial/fuzz;
- rerun security/supply-chain/SBOM;
- rerun calibration/regression benchmarks;
- update M02 execution report and evidence skeleton;
- push exact candidate and open/update PR.

STOP H:
- final finite budgets exist and have evidence;
- selected/rejected candidates are documented;
- DWS equivalence holds;
- all exact-head gates pass;
- executor reports READY_FOR_REVIEW or BLOCKED.

## CALIBRATION PROTOCOL

Required local/synthetic fixtures:
- no-Git;
- clean normal Git;
- dirty tracked;
- CONTENT_HASHED untracked;
- linked worktree;
- bare;
- nested independent repo;
- initialized/uninitialized submodule layouts;
- sparse checkout;
- allowed/denied external object store;
- hostile config/remote secret canaries.

Scale dimensions where bounded:
- graph nodes: 1 / 10 / 100 / 1,000;
- changed-path metadata: 0 / 1 / 10 / 100 / 1,000 / 10,000;
- untracked: 0 / 10 / 1,000 / 10,000;
- small-file hash fanout + streaming large-file case;
- duplicate concurrent proof/hash calls;
- hinted vs zero-event revalidation.

Each relevant scenario:
- deterministic fixture;
- no network;
- warm-up when applicable;
- at least 5 measured iterations;
- cold/warm separated;
- semantic assertion for every candidate;
- unsupported top scale recorded `SKIPPED_RESOURCE_BOUND`, never extrapolated.

Calibrate/freeze finite values for:
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

Concurrency candidates are bounded by host `available_parallelism()` and governed ceilings. Choose the smallest valid candidate on the observed performance/resource Pareto frontier.

Zero/unlimited sentinel values are forbidden for security-sensitive bounds.

## ACCEPTANCE CRITERIA

1. Context Lock remains valid for the implementation basis.
2. Preflight evidence records Git/governance/HIVE truthfully.
3. `core-workspace` exists at the frozen file boundary.
4. No forbidden third-party/internal dependency is added.
5. Public durable M02 contracts use explicit v1 schema/version semantics.
6. Deterministic fingerprints reuse core-identity.
7. ProjectBinding/Workspace/Repository/Worktree IDs satisfy frozen distinction/equivalence laws.
8. Attach lifecycle has no ambient-CWD authority.
9. Receipt cannot mint live handle without fresh validation.
10. Authority roots are typed, explicit and non-transitive.
11. PAF rejects traversal/symlink/junction/reparse escape.
12. FSC is conservative and performs no source-tree probe writes.
13. RepositoryGraphV1 is canonical and handles required topology fixtures.
14. System Git provider is shell-free, no-network, no-prompt, no-mutation and bounded.
15. Hostile Git configuration cannot execute external helpers through inspection.
16. Secret-bearing remote/config data is redacted from canonical/evidence payloads.
17. HIVE association is versioned and cannot grant local path authority.
18. SOLO mode is fully functional for M02 correctness.
19. WorkspaceBasis and BasisDiff are deterministic.
20. Correctness-relevant drift increments generation or invalidates appropriately.
21. Stale handle reuse fails.
22. BVM profiles enforce required freshness and never grant permission.
23. EIS/CIG correctness survives zero events and overflow/loss.
24. DWS selective revalidation is proven equivalent to required full recomputation.
25. PEC L1 is bounded, observable, disposable and generation/policy/provider aware.
26. mtime/stat alone never proves correctness-relevant content.
27. BHC hashing is streaming, bounded, coalesced and cancellation-safe.
28. Resource exhaustion fails typed without partial BOUND.
29. Required unit/integration/property/adversarial suites pass.
30. Required fuzz targets pass bounded campaign with no crash/invariant violation.
31. Ubuntu and Windows exact-head suites pass.
32. Security/supply-chain/advisory/license/SBOM gates pass.
33. M02 runtime/discovery/revalidation/calibration contains zero LLM inference.
34. RCG produces committed reproducible calibration evidence.
35. Final resource defaults are finite and selected from measured candidates.
36. Calibration Delta stays inside its bounded authority.
37. DWS is included; WMF is absent from V0.0.
38. Persistent L2/watchers/Rust-native provider remain absent unless separately admitted.
39. Execution report maps every criterion to exact evidence.
40. No unresolved HIGH/CRITICAL defect exists.
41. Independent governed exact-head review returns APPROVED.

## TESTS

Required commands/gates include, as applicable to the frozen workspace:
- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --workspace --all-features`;
- M02 integration/property/adversarial suites;
- bounded M02 fuzz campaigns;
- governance validator;
- dependency policy/advisory/license checks;
- SBOM generation/check;
- unsafe inventory;
- zero-LLM/forbidden-dependency checks;
- calibration/benchmark runner;
- exact-head CI on Ubuntu and Windows.

Do not replace required hosted evidence with historical green results from another head.

## EVIDENCE

Produce/update:
- `.engineering/evidence/CORE-WO-M02-001.json`;
- `docs/evidence/M02-PREFLIGHT.md`;
- `docs/evidence/M02-CALIBRATION-REPORT.md`;
- `docs/evidence/M02-EXECUTION-REPORT.md`;
- exact CI run identities;
- test/property/fuzz summaries;
- calibration selected/rejected candidates;
- dependency/advisory/license/SBOM;
- zero-LLM proof;
- HIVE availability/integration evidence only if actually observed;
- changed-file summary;
- residual risk list;
- proposed Checkpoint Delta.

Historical evidence can be reused only with unchanged relevant inputs and explicit justification.

## DELIVERABLES

Executor must:
1. inspect repository before modification;
2. create/use execution branch `feat/m02-project-workspace-adapter` from admitted base;
3. implement Pack A-H in order;
4. run/fix required deterministic checks;
5. commit coherent packet increments;
6. push branch;
7. open/update one PR for CORE-WO-M02-001;
8. attach exact-head evidence;
9. propose, but not self-promote, Checkpoint Delta;
10. return final report in Brazilian Portuguese.

## REVIEW FORMAT PT-BR

Final executor report must contain:
- Work Order id;
- authorized base SHA;
- Context Lock status/fingerprint;
- final head SHA;
- commits by Pack A-H;
- files created/changed;
- HIVE preflight result;
- acceptance criteria 1-41 mapped to evidence;
- test/property/fuzz/security/supply-chain results;
- calibration matrix, final budgets and rejected alternatives;
- errors found/corrected;
- residual risks;
- proposed Checkpoint Delta;
- verdict `READY_FOR_REVIEW` or `BLOCKED`.

## EXECUTOR PERMISSIONS

Codex may implement/test/correct only within this Work Order.

Codex may:
- make local causal corrections necessary for frozen criteria;
- apply the one bounded Calibration Delta in Pack H;
- update evidence;
- commit, push and open/update the governed PR.

Codex may not:
- expand scope;
- add unapproved dependencies;
- move HIVE-owned intelligence into CORE;
- weaken security/quality gates;
- change frozen contract meaning;
- add WMF/L2/watchers/Rust-native provider;
- mutate Git history destructively;
- promote checkpoint or merge its own implementation PR.

## BLOCKER / CORRECTION PROTOCOL

If a frozen requirement is impossible or creates a correctness/security defect:
1. stop the affected packet;
2. preserve valid earlier evidence;
3. report exact blocker + reproduction;
4. propose the smallest Correction Delta;
5. do not redesign unrelated packets;
6. do not continue into later dependent packets until governed resolution.

Calibration-only numeric changes follow Pack H authority. Anything outside it is a normal governed correction.

## FINAL STOP CONDITION

STOP only when either:

### READY_FOR_REVIEW
- Packs A-H STOP conditions all pass;
- all 41 acceptance criteria are evidenced on the exact candidate head;
- RCG is closed with finite calibrated budgets;
- required CI/evidence is green;
- evidence bundle is complete;
- no unresolved HIGH/CRITICAL finding remains;
- branch is pushed and implementation PR is ready for independent review;

OR

### BLOCKED
a reproducible blocker cannot be corrected without changing frozen architecture/scope/dependencies beyond authorized Calibration Delta.

Compilation alone is not completion.
A partial packet sequence is not completion.
Skipped required evidence is not completion.
The executor must not declare APPROVED; only the independent governed reviewer may do so.


## Execution admission record

Final planning freeze review: M02 Review 005 / Issue #35 — APPROVED
Final-freeze PR: #33
Final-freeze exact-head workflow: #84 `35516261944` — Governance, Ubuntu, Windows and fuzz SUCCESS
Promoted execution base: `bae47b2021a897396109dfcf42e8632dde13ec21`

This record changes execution state only. All architecture, scope, dependency, acceptance, calibration and STOP semantics above remain frozen.
