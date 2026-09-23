# CORE-M03-DISC-004 - M03 Round 4 planning evidence

## Execution basis

- Increment: `CORE-M03-DISC-004`
- Captured: 2026-09-22 21:42 -03:00; the final exact-head CI addendum is linked from the PR after the evidence update is pushed.
- Repository: `KayzenRoot/core` (`https://github.com/KayzenRoot/core`)
- Canonical base: `main` at `9773d84bee119db8f964feb64ed06a2acd4456c8`
- Branch: `planning/m03-round-4-contracts-validation`
- Worktree: `D:\Projects\core-m03-round4`, clean when created at the exact base.
- Initial published PR head SHA: `1f896f9d6fbc7042d0f65d871c950a47a2fedb32`. The exact final source SHA is shown by PR #62's source-branch metadata and repeated in its body. This evidence file is committed on that branch; embedding the hash of its own containing commit in the file would be self-referential. The PR body/check record is the linked exact-head evidence record.
- Executor environment: Windows PowerShell; Git 2.55.0.windows.3; Python 3.12.10 (`C:/Users/csn19/AppData/Local/Programs/Python/Python312/python.exe`); rustc 1.98.1 and Cargo 1.98.1 from the installed stable MSVC toolchain. Cargo is invoked by its installed absolute path because it is not on PATH; `RUSTC` is set to the matching installed rustc for Cargo-launched helper processes.
- PR: #62, open against `main`, https://github.com/KayzenRoot/core/pull/62. The final source SHA and exact-head CI run/context IDs are recorded in the PR metadata/body/checks after the evidence update is pushed; no merge is authorized.

## Preflight and HIVE result

- `git fetch origin --prune` completed; fetched `origin/main` at the admitted base above.
- In the original checkout, `git checkout main` confirmed the branch and `git pull --ff-only origin main` stopped because pre-existing user changes in `AGENTS.md` would be overwritten. The original `AGENTS.md` edit and untracked `.gitattributes` were preserved. No user change was discarded.
- Created the required isolated branch/worktree from `origin/main`. At creation, branch and HEAD were exact and `git status --short` was empty. A repeat fetch confirmed `origin/main` still matched the admitted base.
- HIVE MCP was reachable. `project.list` completed with 7 projects and `truncated=false`; CORE is not registered. The HIVE v1.0.0 project itself reported `READY` at pinned commit `a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf`. No CORE checkpoint was available to read through HIVE, so no HIVE checkpoint evidence is claimed. CORE's canonical Git sources were used under the work order's degraded-safe planning allowance.
- Canonical sources read in order: `AGENTS.md`; `.engineering/SOURCE-HIERARCHY.md`; `docs/project-brain/13-CHECKPOINT.md`; `16-DECISIONS-LEDGER.md`; `03-SCOPE.md`; `15-DEFINITION-OF-DONE.md`; `04-ARCHITECTURE.md`; `02-REQUIREMENTS.md`; `10-SECURITY-GOVERNANCE.md`; `11-TEST-PLAN.md`; `docs/modules/M03-WORK-ORDER-ENGINE.md`; `docs/modules/00-MASTER-MODULE-MAP.md`; `docs/project-brain/14-BACKLOG.md`; `.engineering/gef/GEF-POLICY.md`; `.engineering/gef/GEF-REVIEW-PROTOCOL.md`. Cargo workspace manifests and the relevant M02/M01 public source surfaces were inspected to verify dependency reality.
- No unresolved canonical conflict was found. Round 4 preserves accepted Rounds 1-3 and reviewer-first CORE-D-132. Scope remains documentation/planning only; there is no M03 implementation Work Order or Context Lock in this increment.

## Changed files

1. `.engineering/CHECKPOINT.json` - mirror the canonical next step for the independent exact-head audit.
2. `.engineering/CHECKPOINT.md` - matching human-readable GEF checkpoint mirror.
3. `.engineering/evidence/CORE-M03-DISC-004.md` - this evidence bundle; PR metadata is its linked exact-head/CI addendum.
4. `docs/modules/00-MASTER-MODULE-MAP.md` - show Round 4 as an unpromoted review candidate and preserve the implementation gate.
5. `docs/modules/M03-WORK-ORDER-ENGINE.md` - freeze Round 4 public contracts, pure APIs, adapters, file map, dependency boundary, validation laws, resource policy, and future DoD direction.
6. `docs/project-brain/02-REQUIREMENTS.md` - append CORE-R-208 through CORE-R-228 sequentially.
7. `docs/project-brain/04-ARCHITECTURE.md` - refine the candidate dependency direction from inspected Cargo evidence and specify adapter/core boundaries.
8. `docs/project-brain/10-SECURITY-GOVERNANCE.md` - record M03 V1 fail-closed, secrecy, replay, scope, and external-authority constraints.
9. `docs/project-brain/11-TEST-PLAN.md` - record property, adversarial, fuzz, benchmark, resource, and exact-head evidence obligations.
10. `docs/project-brain/13-CHECKPOINT.md` - record the candidate and proposed next audit while explicitly retaining no-promotion/no-implementation authority.
11. `docs/project-brain/14-BACKLOG.md` - update M03 to Round 4 candidate pending independent review.
12. `docs/project-brain/15-DEFINITION-OF-DONE.md` - add the traceable M03 V0.0 production completion direction.
13. `docs/project-brain/16-DECISIONS-LEDGER.md` - append CORE-D-150 through CORE-D-161 without renumbering prior decisions; all are planning-candidate decisions pending independent audit/promotion.

No product source, tests, Cargo manifest, lockfile, active protection rule, or generated runtime artifact changed.

## Round 4 freeze summary

- Versioned `nexlabs.core.work-order` V1 contracts cover request, immutable frozen revision, compilation receipt, admission request/receipt, and immutable M04 handoff; IDs, revisions, fingerprints, source/evidence, scope, context, packet DAG, acceptance graph, lineage/LPC, diagnostics, and typed errors have explicit ownership.
- Synchronous pure operations define compile, validation, semantic diff, correction classification, admission evaluation, handoff materialization, and canonical semantic bytes. I/O, refresh, retries, persistence, and external authority remain caller-owned.
- External source, M02 workspace/basis, Context Lock, governance, lineage/CAS, and optional HIVE seams exchange bounded typed evidence with provenance, fingerprints, and freshness. M02 retains workspace truth; HIVE remains advisory.
- The one-crate future file map includes library modules plus unit/integration/property/fuzz/benchmark targets. The direct dependency disposition is `core-identity` plus existing `serde`, `serde_json`, and `thiserror`; no direct M02 service, Tokio, Git/network/process, database, or cache authority is admitted. Cargo metadata showed 10 existing crates and no M03 crate; `core-workspace` currently enables Tokio filesystem/network/process features, while the inspected `core-identity` dependency closure has no such authority.
- Property/adversarial laws and bounded fuzz targets cover schema/canonicalization, DAG/scope/AEG, correction, lineage/LPC, provenance, admission replay/staleness, reconstruction, resources, and diagnostics/redaction. Benchmark dimensions and repeat/warm-up procedure are frozen; no M03 measurements or unproven numeric defaults are claimed. Calibration remains evidence-gated.
- Production DoD direction traces exact-head Windows/Ubuntu, deterministic serialization, no-hidden-I/O, security, property/fuzz, resource/performance, dependency/supply-chain, and independent-review evidence. Product implementation and its Work Order/Context Lock remain unauthorized and uncreated.

## Requirements and decisions

- Requirements: `CORE-R-208` through `CORE-R-228` (21 appended requirements).
- Decisions: `CORE-D-150` through `CORE-D-161` (12 appended Round 4 planning decisions).
- Candidate state: accepted for Round 4 planning content only; independent audit and canonical promotion are pending. These additions authorize no implementation.

## Validation commands and results

| Command / evidence | Result |
| --- | --- |
| `git diff --check` | PASS on the final pre-commit tree after correcting five trailing whitespace lines in the checkpoint. |
| `python -m py_compile scripts/validate_governance.py scripts/hive_bootstrap.py` | PASS with system Python 3.12.10. |
| `cargo metadata --format-version 1 --no-deps` | PASS; confirmed 10 workspace crates and no `core-work-order`; no manifest or lockfile was changed. |
| `cargo tree --locked -e normal -p core-identity` | PASS; inspected dependency closure and confirmed no process/network/database dependency. |
| `python scripts/validate_governance.py` | Initial run found a real stale derived checkpoint mirror after the canonical next-step change. Updated only the two exact GEF mirror fields; rerun PASS: GEF/HIVE pins and checkpoint/source bridges consistent; 27 required artifacts. |
| `python -m unittest discover -s tests -p "test_*.py" -v` | PASS; 6 tests. |
| `cargo test --workspace --all-targets` | First attempt could not find `rustc` in PATH from an existing Git helper test. Reran the same gate with `RUSTC` set to the installed Rust 1.98.1 executable; PASS, 103 tests across the workspace. Existing M01/M02 harness-false benchmark executables ran; no M03 performance result is claimed. |
| `cargo clippy --workspace --all-targets -- -D warnings` with installed `RUSTC` | PASS; finished with no warnings. |
| Exact-head GitHub Actions | Pending the final evidence-update push. Historical workflows are not substituted. Record all seven required contexts and run IDs in PR #62 before handoff. |

## Acceptance checklist

1. PASS - exact admitted base, clean isolated worktree, and required branch verified.
2. PASS - required canonical sources read in order; no unresolved source conflict found.
3. PASS - accepted Round 1-3 invariants and CORE-D-132 reviewer-first boundary preserved.
4. PASS - versioned public type ownership and API responsibilities specified for later mechanical implementation.
5. PASS - pure operation inputs, outputs, errors, and no-hidden-I/O boundary specified.
6. PASS - external adapter seams use bounded, fingerprinted, freshness/provenance-bearing evidence.
7. PASS - M02, Context Lock, governance, replay, and staleness evidence consumption specified.
8. PASS - future crate/file map includes unit/integration/property/fuzz/bench targets and remains acyclic.
9. PASS - minimal dependency policy is grounded in current Cargo metadata/tree; no unadmitted authority added.
10. PASS - semantic and diagnostic projections plus deterministic identity are separated.
11. PASS - typed failure/retry classes cover the required source, schema, scope, graph, lineage, resource, admission, stale/replay, and invariant failures.
12. PASS - property laws cover replay/permutations, DAG, scope, AEG, CAS, provenance, reconstruction, partial output, and redaction.
13. PASS - parser/canonicalizer, DAG, scope/delta, lineage/LPC, and admission/provenance fuzz boundaries are bounded.
14. PASS - benchmark scaling dimensions and conditional cache cases are specified.
15. PASS - finite resource/measurement policy contains no fabricated measurements; numeric calibration is evidence-gated.
16. PASS - production DoD direction traces exact-head cross-platform, security, property/fuzz, performance/resource, and supply-chain evidence.
17. PASS - durable secrecy, hidden I/O, downgrade, replay, stale READY, scope downgrade, and authority-changing retry controls are specified.
18. PASS - requirement and decision IDs append sequentially; accepted history was not renumbered.
19. PASS - checkpoint, backlog, module map, and module status agree on review-candidate state and unauthorized implementation.
20. PASS WITH LINKED EXACT-HEAD ADDENDUM - this bundle records the exact base and links to the PR record for the final self-containing commit SHA and exact-head CI identifiers.
21. PASS - no unresolved HIGH/CRITICAL planning inconsistency was identified during the bounded consistency review; independent audit remains pending.
22. PASS - exactly one PR (#62) is open from this branch to `main`; it is unmerged. Its final source SHA and required CI will be bound to the final evidence-update head before handoff.

## Risks and proposed checkpoint delta

- CORE has no HIVE project registration, so no HIVE checkpoint could be read. Planning relied on the exact canonical Git base under the work order's degraded-safe allowance.
- The original checkout's local `AGENTS.md` modification and untracked `.gitattributes` prevented its fast-forward pull; both were preserved by executing from a clean isolated worktree.
- Cargo was unavailable by command name, but the installed stable MSVC Cargo/Rust toolchain supported the requested repository gates. The explicit `RUSTC` setting resolved the existing helper test's PATH assumption.
- The independent exact-head reviewer, promotion gate, and exact-head hosted CI remain external gates. No source implementation or measured M03 runtime/performance evidence exists or is claimed.
- Proposed Checkpoint Delta (NOT PROMOTED): record the Round 4 planning surfaces as a candidate pending independent exact-head review; retain M03 implementation as unauthorized; retain the later implementation Work Order and Context Lock as pending; set next action to independent review against Scope, Architecture, Requirements, Security, Test Plan, DoD, and all 22 acceptance criteria.

## PR exact-head and CI addendum

PR #62: https://github.com/KayzenRoot/core/pull/62. The PR source-branch metadata is the authoritative record of the final source SHA; its body will repeat that SHA and list each required context with run/workflow identifier and terminal status after the final evidence-update push. `READY_FOR_REVIEW` is allowed only if the exact final head is open against `main`, all seven required contexts succeeded, and this evidence bundle is complete. Do not merge or promote the checkpoint.


## Reviewer-first Correction Delta - Review 001 pre-approval

Independent review found three planning inconsistencies that were directly correctable inside the admitted documentation-only scope:

1. **Pure-core deadline contradiction:** the candidate placed wall-clock millisecond deadline fields inside `M03ResourceBudgetV1` while also requiring synchronous value-replay purity and no ambient state. Correction: core budgets are deterministic size/depth/cardinality limits only; wall-clock deadlines are caller-owned orchestration guards, the core reads no clock, and any timed-out/late result is discarded before it can become FROZEN/READY/handoff evidence.
2. **Semantic/compiler identity overlap:** the candidate included compiler contract generation inside `WorkOrderFingerprint` while `WorkOrderCompilationId` is the accepted compiler-identity layer. Correction: compiler/canonicalizer/policy/config/security implementation generations are excluded from WorkOrderFingerprint and remain bound by WorkOrderCompilationId.
3. **Incomplete public service-result freeze:** validation/diff/correction return types were referenced by public signatures without sufficiently frozen root/result shapes. Correction: root contract kinds now include compilation/validation/diff/correction results and exact result fields/disposition are frozen sufficiently for a mechanical implementation Work Order.

These corrections do not add product code, dependencies, implementation authority, scope, runtime behavior, or new module ownership. They preserve CORE-D-132 reviewer-first correction policy and invalidate prior exact-head CI. The PR source-branch metadata is the authoritative corrected head; fresh seven-context exact-head CI is required before an APPROVED verdict.
