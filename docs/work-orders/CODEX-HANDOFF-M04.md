# CODEX HANDOFF — M04 Run / Attempt / Step Engine

Work Order: `CORE-WO-M04-001`  
Increment: `CORE-M04-FREEZE-001`  
Status: `BLOCKED_PACK_A_STOP / SYNC_006_APPROVED_PROMOTED / POST_PROMOTION_PREFLIGHT_PASSED`
Future execution branch: `feat/m04-run-state`

## STOP BEFORE EXECUTION

Do **not** modify product code, Cargo manifests/lockfiles, fuzz targets, benchmark code or runtime crates unless all of the following are true on canonical `origin/main`:

1. CORE-M04-FREEZE-001 has been independently exact-head reviewed and promoted as canonical base `f6b422be5465d5a93d0b8fcf4c9507c205663072`.
2. CORE-M04-ADMIT-001 has been independently reviewed and promoted to canonical `origin/main` as merge `bb6f631284361fae29479c66f62ca88bebf3d79c`.
3. `.engineering/context-locks/CORE-WO-M04-001.json` on canonical main is `ACTIVE`.
4. The lock contains a concrete `authorizedBase`.
5. `productImplementationAuthorized = true`.
6. The lock binds the exact current `CORE-WO-M04-001` blob and the exact locked canonical source fingerprints.
7. The execution branch is exactly `feat/m04-run-state` and is created from the admitted post-promotion canonical main.
8. Git/governance preflight passes and optional HIVE preflight is recorded honestly if available.

If any condition is absent, stale, UNKNOWN, conflicting or exists only on a PR branch, stop with `BLOCKED / NOT_AUTHORIZED`.

## CANONICAL READ ORDER

Read:
1. `docs/project-brain/13-CHECKPOINT.md`
2. `docs/project-brain/16-DECISIONS-LEDGER.md`
3. `docs/project-brain/03-SCOPE.md`
4. `docs/project-brain/15-DEFINITION-OF-DONE.md`
5. `docs/project-brain/04-ARCHITECTURE.md`
6. `docs/project-brain/02-REQUIREMENTS.md`
7. `docs/project-brain/10-SECURITY-GOVERNANCE.md`
8. `docs/project-brain/11-TEST-PLAN.md`
9. `docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md`
10. `.engineering/SOURCE-HIERARCHY.md`
11. `.engineering/gef/GEF-CURRENT.json`
12. `AGENTS.md`
13. `.engineering/work-orders/CORE-WO-M04-001.md`
14. the exact ACTIVE M04 Context Lock.

Consult M03 implementation contracts only to satisfy the frozen M03→M04 handoff boundary. Do not reinterpret M03 as M04 policy.

## EXECUTION MODEL

Execute Packs A-H in order:
- A contracts / typed identity / canonical framing;
- B lifecycle / projection;
- C journal / replay / snapshots;
- D generation-CAS / idempotency / cancellation;
- E BRC / ICF / external references;
- F resource limits / prepared-finalized receipt boundary / store conformance / no hidden I/O;
- G properties / adversarial tests / six fuzz targets / security / supply chain;
- H deterministic calibration / bounded numeric delta / evidence / exact-head CI / PR handoff.

Do not advance a dependent pack while an upstream blocking obligation is unresolved.

## FROZEN IMPLEMENTATION BOUNDARY

Implement one crate: `crates/core-run-state`.

Allowed direct production dependencies:
- `core-work-order`
- `core-identity`
- `serde`
- `thiserror`

No direct production dependency on Tokio, `core-runtime`, `core-workspace`, Git/HIVE/GitHub SDKs, filesystem/network/process/database/time APIs, Criterion, proptest, graph/cache frameworks or a new cryptography stack.

`serde_json` is test/tooling-only unless a separate governed proof admits production use.

No production persistence backend is selected by this Work Order.

## RECEIPT AUTHORITY RULE

`prepare_*` returns `PreparedCommitV1<R>` with a pending operation-specific receipt. That receipt is **not committed authority**.

The host persists the prepared commit using the `M04StateStoreV1` equivalent atomic compare-and-commit port. Only after a matching `DurableCommitReceiptV1` is passed to pure `finalize_commit` may `R` be released as authoritative.

Any RunId/fingerprint/generation/event-sequence/journal-root mismatch fails closed.

## CALIBRATION

Do not invent resource defaults.

Pack H may run deterministic supported-platform measurements and apply the one authorized Calibration Delta only to:
- finite numeric `M04ResourceLimitsV1` defaults/thresholds;
- calibration fixtures/results/report;
- numeric test expectations that necessarily follow the selected limits.

If implementation evidence suggests a semantic, dependency, authority, backend or security change, stop and request a governed Correction Delta.

## EVIDENCE

Maintain `.engineering/evidence/CORE-WO-M04-001.json`.

AC-M04-001..023 map exactly one-to-one to EV-M04-001..023.

For each implementation-owned criterion record exact head, command/artifact, platform where applicable, result and evidence path/reference. Do not mark AC-M04-023/EV-M04-023 satisfied. That criterion belongs to the independent reviewer.

## TERMINAL STATES

### READY_FOR_REVIEW
Allowed only after Packs A-H, AC-M04-001..022, calibration/post-calibration reruns, required exact-head CI/security/supply-chain evidence and the implementation PR are complete with no unresolved HIGH/CRITICAL executor finding.

### BLOCKED
Use when any authorization, locked source, packet obligation, acceptance/evidence node, finite resource selection, CI/security gate or frozen contract is missing/stale/conflicting/failing or requires out-of-scope change.

Never return `APPROVED`. Never merge your own implementation.


## Admission candidate note

CORE-M04-SYNC-003 was APPROVED by M04-REVIEW-009 / Issue #97 at exact head `f9a5a7847e268000a5249ae8e69c81ed22b924ad`; PR #96; workflow `35994572596` with 10/10 jobs SUCCESS; promotion merge `b79891f489d8c7117aee15e1dca47abb9e23dea3`; unresolved HIGH/CRITICAL: 0.

CORE-M04-SYNC-004 was independently APPROVED by M04-REVIEW-010 / Issue #100 at exact head `19123d50bebe1a13257d8e2768fca7a1ca1d3393`; PR #99; workflow `36013624792` completed 10/10 jobs SUCCESS; squash promotion merge `b2ac8e0db72a2145948e7773295b1b252c5e4eab`; unresolved HIGH/CRITICAL: 0.

CORE-M04-SYNC-005 was independently APPROVED by M04-REVIEW-011 / Issue #102 at exact head `e8b0548c2ecd7c22edbeb3f02de8a238f9c49ffc`; PR #101; workflow `36033275191` completed 10/10 jobs SUCCESS; promotion merge `251f15495b82df8270ebc12fa93807ffaa15fba4`; unresolved HIGH/CRITICAL: 0.

CORE-M04-SYNC-006 was independently approved and squash-promoted to canonical main at `d2b750f917f841fe715aafea9e0f80fbd3bc1035`. The required fresh Git/HIVE/governance preflight passed; Pack A began on `feat/m04-run-state` under the ACTIVE Context Lock. Its current STOP A result is recorded below.

## CORE-M04-SYNC-006 promotion and Pack A preflight

- M04-REVIEW-012 / Issue #105 records APPROVED for PR #104 reviewed head `11982778fdf0c8e345f72f5bd709f94cb14bfe86`.
- PR #104 is merged; its reviewed base was `251f15495b82df8270ebc12fa93807ffaa15fba4`, and squash promotion is `d2b750f917f841fe715aafea9e0f80fbd3bc1035`.
- Workflow `36236642339` is completed with SUCCESS: 10/10 jobs.
- Fresh `origin/main` recheck matched `d2b750f917f841fe715aafea9e0f80fbd3bc1035`.
- The ACTIVE Context Lock has `productImplementationAuthorized=true`, `executionStatus=ACTIVE_AUTHORIZED_NOT_STARTED`, and `authorizedBase=f6b422be5465d5a93d0b8fcf4c9507c205663072`.
- The frozen Work Order blob is `8beffbc19e73a60f49a1e4d7797427391680d5e3`; all nine locked source fingerprints match canonical Git.
- The authorized base is an ancestor of canonical main. The five intervening commits are exactly CORE-M04-ADMIT-001 (#94), CORE-M04-SYNC-003 (#96), CORE-M04-SYNC-004 (#99), CORE-M04-SYNC-005 (#101), and CORE-M04-SYNC-006 (#104).
- Read-only HIVE resolved the D:/Projects CORE registration, but its recorded checkout is at `fdb4dbe165e74b009c43df3874b6043c9b94710b` and dirty; `checkpoint.read` returned `source_not_current`. The Work Order permits `SOLO_GIT_CANONICAL`, so this preflight used canonical Git as authority and recorded HIVE as advisory.
- The original D:/Projects/core checkout remains on `main` at `fdb4dbe165e74b009c43df3874b6043c9b94710b` with its pre-existing `AGENTS.md` modification and untracked `.gitattributes`; execution uses the clean isolated worktree.
- `feat/m04-run-state` was created from the exact canonical main in `D:/Projects/core-m04-sync-006-solo`.

## Pack A execution and stop result

Pack A contracts, identity wrappers, canonical framing, and focused tests were added on the authorized branch. No Pack B files or lifecycle behavior were added.

Passing local checks on Windows:
- `cargo test -p core-run-state --locked` — 8 integration tests and 1 compile-fail documentation test passed.
- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` — PASS.
- `python -m py_compile scripts/validate_governance.py scripts/hive_bootstrap.py scripts/hive_mcp.py` — PASS.
- `python scripts/validate_governance.py` — PASS; GEF/HIVE bridges consistent and 27 required artifacts present.
- `python -m unittest discover -s tests -p "test_*.py" -v` — 6 tests passed.
- `git diff --check` — PASS.

STOP A is **BLOCKED** by the CI-required workspace test command `cargo test --workspace --all-targets --all-features --locked`. It failed in the unchanged M02 test `crates/core-workspace/tests/git_system.rs:225`, `hostile_dual_output_is_capped_without_deadlock`, with `dual output must exceed an independent cap: None`. An isolated rerun of that exact test failed with the same assertion. The M04 focused crate tests passed during the workspace run and again on their own.

No claim is made that the broader test gate passed. No hosted exact-head CI was started; changes remain uncommitted and unpushed, no implementation PR was opened, all EV-M04 nodes remain PENDING, and Pack B was not started. The current local work is on `feat/m04-run-state` at base HEAD `d2b750f917f841fe715aafea9e0f80fbd3bc1035` with the Pack A changes uncommitted.

## Pack A baseline-versus-candidate triage CD-01 — 2026-09-26

The complete `CORE-M04-PACK-A-CD01-BASELINE-TRIAGE.pdf` prompt was reviewed and its checklist executed. The baseline and candidate are separate detached worktrees at exact base `d2b750f917f841fe715aafea9e0f80fbd3bc1035`. The baseline is clean. The candidate contains only the Pack A crate/workspace delta and the permitted Evidence, GEF and handoff updates.

The PDF's suspected `crates/core-workspace/tests/process_supervisor.rs` path does not exist and is not tracked at the base or candidate. The actual failing test is `crates/core-workspace/tests/git_system.rs:201-225`; that source is unchanged in baseline, candidate and the execution checkout (SHA-256 `85A280A33AAA247E562AB8FDDB8CDAC5C0530A624A55A7688F8BEAEA533126FA`, Git blob `245f72569d69758710036bb1a57ff6810af9712e`). No M02 test or implementation file was edited.

The exact `cargo test --workspace --all-targets --all-features --locked` command failed on baseline and candidate with exit 101 at `hostile_dual_output_is_capped_without_deadlock`, panic text `dual output must exceed an independent cap: None`. The candidate's exact isolated rerun also failed with exit 101. Logs are retained at:
- `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-triage-cd01\baseline-workspace-test.log` (SHA-256 `8756C5F7037E46FF776AF5B91984F26E5E16A6B360826C5A58991A94213C6574`)
- `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-triage-cd01\candidate-workspace-test.log` (SHA-256 `FA958602419F006297F7F3A57241E7AE431EE04265BE3BAEC265C75EB1465D32`)
- `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-triage-cd01\candidate-isolated-git-system.log` (SHA-256 `3D2FF49A68928C71D52C67B659AB647149E3C6163B79D13F90CFE1DA57DC1762`)

The failure is independent of the Pack A delta. Source inspection and a temporary Windows Rust probe using the M02 adapter's command arguments and sanitized environment reproduced a child process that starts but exits non-zero with empty stdout/stderr. In `crates/core-workspace/src/git/system.rs:54,93,244-247`, spawn/command failures are typed as `GitInspection` and the first `rev-parse` maps that error to `Ok(None)`, which explains the test's observed `None`. The test helper is a `.cmd` file (`crates/core-workspace/tests/git_system.rs:10-13,201-225`), while the production adapter clears the environment before launching it. The retained probe log is `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-triage-cd01\windows-cmd-spawn-probe\sanitized-cmd-spawn-probe.log` (SHA-256 `2F541B0D43CC10502898C7795D3A9ED1F844708775FB18F62E7173DD7ABC2BA4`). A governed M02 follow-up should replace the Windows stream-test helper with a native test executable (or otherwise prove the sanitized batch-helper contract) and rerun the isolated and full workspace tests. No M02 change is included here.

The Pack A deep review found: domain-specific Run/Attempt/Step/Event/idempotency IDs are distinct Rust types; identifiers and fingerprints validate at construction and deserialization; the type-level cross-domain substitution compile-fail doctest passes; envelope metadata, contract-kind, event, reason and error registries are closed; the canonical frame uses `CORE-M04`, fixed V1/schema/domain header fields, strictly increasing explicit tags, big-endian fixed-width integers, length-prefixed variable-width bytes and the shared `core_identity::fingerprint_bytes`; the golden bytes and digest, domain separation, field-order rejection and direct repeated-byte invariance all pass. Projection maps use `BTreeMap`. Pack A adds no lifecycle service, clock, random source or I/O. Direct production dependencies remain the four admitted dependencies; JSON support is test-only.

One boundary remains explicit for later semantic fingerprint builders: `CanonicalFrameV1::push_bytes`/`push_string` accept caller-provided bytes (`crates/core-run-state/src/canonical.rs:87-110,169-200`), so the generic primitive alone cannot prevent a caller from encoding a diagnostic, secret or time value under a payload/source/target tag. The current `EventPayloadV1` and `CanonicalEventV1` shapes (`crates/core-run-state/src/contracts.rs:415-488`) contain no diagnostic or timestamp field, and no Pack A semantic event-fingerprint builder exists. This is recorded as a future caller-side allow-list requirement, not as proof that arbitrary bytes are safe to fingerprint. Do not implement a new semantic projection in this triage; Packs B-H remain unstarted.

After adding the direct canonical-byte invariance assertion, candidate checks passed: `cargo test -p core-run-state --locked` (8 integration tests plus 1 compile-fail doctest), `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`. The full workspace gate remains red on both baseline and candidate, so STOP A remains **BLOCKED**. No commit, push, hosted CI or implementation PR was created; all EV-M04 nodes remain PENDING and Pack B was not started.

## CORE-M04-PACK-A CD-02 — Windows M02 fixture correction and Pack A audit

The complete three-page `CORE-M04-PACK-A-CD02-WINDOWS-M02-HELPER.pdf` was read. Its direct request field was blank, so the PDF's explicit correction delta was executed under the repository's PDF Work Order rule. The exact branch remains `feat/m04-run-state`, based on `d2b750f917f841fe715aafea9e0f80fbd3bc1035`; canonical `origin/main` was rechecked at that SHA before editing. All nine locked source fingerprints, the ACTIVE Context Lock, authorized-base ancestry and Work Order blob remain unchanged. HIVE returned CORE as OFFLINE, so this run records `SOLO_GIT_CANONICAL`; no HIVE evidence was fabricated. The separate `D:/Projects/core` checkout remains untouched.

The CD-01 reference to `crates/core-workspace/tests/process_supervisor.rs` is historical and superseded. The verified failure is `crates/core-workspace/tests/git_system.rs::hostile_dual_output_is_capped_without_deadlock`; the base test was at line 225. CD-01's old path record remains in Evidence for history. The saved probe line `spawn=ok success=false stdout="" stderr=""` reports the `.cmd` child status and its empty output. The status-0 wording in the CD-02 prompt refers to the outer probe process, which printed that line; it does not show that the batch helper succeeded or exercised dual output.

### A/B/C isolation and results

- **A — clean baseline:** detached worktree `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-cd02\A-baseline`, clean at exact base `d2b750f917f841fe715aafea9e0f80fbd3bc1035`. The CD-02 focused test reproduced the original failure (exit 101, `None`). CD-01's separate full workspace run on this same clean base also failed the unchanged test (exit 101).
- **B — base plus fixture-only correction:** detached worktree `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-cd02\B-fixture-only`, same base, with exactly one changed file: `crates/core-workspace/tests/git_system.rs`. The focused Windows test and `cargo test --workspace --all-targets --all-features --locked` both passed (exit 0). The fixture-only patch SHA-256 is `0BE92A9B412014601810C06E5E8A19DA07389DBC0001C05D60B2864058B636D1`.
- **C — Pack A plus CD-02:** authorized candidate checkout `D:/Projects/core-m04-sync-006-solo`, branch `feat/m04-run-state`, with the existing Pack A files, the same fixture correction and the minimum Evidence/GEF/handoff updates. The focused Windows test, full workspace command, Pack A crate tests, fmt, Clippy, governance validator and diff check passed. The focused test also passed in Linux x86_64 using a read-only source mount and Rust 1.98.1 Docker image `rust:1.98.1-slim-bookworm` (digest `sha256:ff521445a372125ed4f76e1453a1f8098f2d05332d1601d30db1c1f62757e730`). The product-plus-fixture patch SHA-256 is `DAA9470994807D36A436FA78E4A3D5D6579C154A86404BA766C0C8B594FFDC0F`.

The corrected helper is a deterministic Rust executable compiled into the test's temporary fixture directory and invoked directly by path. It writes exactly 50 `x` bytes to stdout and 50 `y` bytes to stderr, exits 0, and has no shell, `.cmd`, `COMSPEC` or runtime environment dependency. `env_clear()` remains in `SystemGitInspector`; a direct fixture probe also uses `env_clear()` and asserts the child's success status, both exact output vectors, both lengths and their distinct contents. The test keeps the existing 16-byte limits, asserts `ResourceBudgetExceeded("Git stdout")` with the actual default stderr cap, then `ResourceBudgetExceeded("Git stderr")` with the actual default stdout cap, and finally retains the hostile dual-overflow assertion with both caps at 16. Each run returns below the existing five-second bound. The production M02 supervisor was not changed.

CD-02 logs and their hashes are retained under `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-cd02`. The baseline focused log is `A-focused-baseline.log` (`E1FF5FF26F45E93E84354E99D4E3BF41C481272001BD6691645DF0531D13DCD4`); B focused/full logs are `B-focused-fixture-only.log` (`A46227F7C15FBBBD6EBFED186819B419E06DEE4C22B7BF0648B28A7EEDA8C2F9`) and `B-workspace-fixture-only.log` (`F3D4EE977C7B9026FCFC6D5C31CCB27BB4B15999F471AF3950791408EE1D8E82`); C focused Windows, Linux, Pack A and full workspace logs are `C-focused-candidate.log` (`9D1D3966D52476D3262F323086A461E55CC965BBA4B93EDCAF19FEE9D1B29034`), `C-focused-linux-docker.log` (`69FE3E11CB9C8294CE5689C1176EABCC036E5F8B6A4C4D8CC575C13488FE595B`), `C-pack-a-focused.log` (`E1BCF6DB6FB190694D6AB10ABCC6BFA7C28E091CCC8C443B1C8A667B37A9F1A0`) and `C-workspace-after-canonical-fix.log` (`EAF30DFA41F38EFBE768C01942B382E24B7D14EB189B7A27DA6CB3C749F186FF`). The raw B full log includes a failed PowerShell-only metadata header expression before Cargo started; the Cargo command itself completed with exit 0, and the raw log is preserved without alteration.

### Final Pack A canonical API review

The audit confirmed the original public `CanonicalFrameV1` raw-byte builder could hash caller-selected bytes without an allow-listed semantic projection. CD-02 therefore narrows the public surface: `CanonicalFrameV1` and `CanonicalFieldTagV1` are crate-private and no longer re-exported. `FingerprintDomainV1` remains public because it is a field type in the public `ContinuationFrameV1` and `CanonicalEventV1` contracts; it does not expose frame construction or hashing. A new compile-fail doctest proves callers cannot import the raw frame builder. Golden bytes/digest, domain separation, stable domain/tag codes, primitive encodings, field ordering and public fingerprint value round-trip tests pass. A temporary compile error from initially hiding `FingerprintDomainV1` was corrected by keeping this contract field type public while retaining the raw builder restriction.

The shared `core_identity::fingerprint_bytes` primitive remains the digest implementation. `CanonicalFingerprint::new` validates the external lowercase-hex value shape; it does not hash a caller byte slice. Pack A still has no semantic event fingerprint builder. Such future public builders must accept typed allow-listed semantic fields and exclude diagnostics, timestamps, locale-dependent values and secrets. Pack A emits no semantic event projection, and the raw builder is not exposed in the current public API.

### CD-02 disposition and next gate

The local STOP A checks are green on Windows, and the focal M02 helper check is green on Linux. All 23 EV-M04 nodes remain PENDING because their full acceptance evidence has not been produced; no Pack B work has started. Work Order, Context Lock and the nine source fingerprints are unchanged. The proposed Checkpoint Delta is to record the corrected test-only M02 prerequisite and Pack A source/test evidence only after exact-head PR checks and independent audit; no checkpoint promotion is made here. The remaining action is the required staged/unstaged/untracked scope audit, commit and push on `feat/m04-run-state`, then open/update the PR and wait for Windows/Ubuntu exact-head checks and independent review.

### CD-02 final local rerun after Evidence/GEF/handoff updates

After the CD-02 records and line-ending cleanup, the exact candidate command `cargo test --workspace --all-targets --all-features --locked` passed again on Windows with exit 0. Final log: `C:\Users\csn19\AppData\Local\Temp\core-m04-packa-cd02\C-workspace-clean-final.log` (SHA-256 `D73446882531601A1DADC7920243D463352859B3CB65A4115FADFD24FF03AB25`). The post-update reruns also passed: `cargo fmt --all -- --check` (`C-fmt-check-clean-final.log`, SHA-256 `365A9C244540EC2D263B7193E62A4BDDA83D0273D04D8A9420858981C81E514B`), workspace Clippy with `-D warnings` (`C-clippy-clean-final.log`, SHA-256 `654A0B0D988297CF3930E4F850559C9D22DA7DCFE47DB6DD1814ED2B94C34143`), `python scripts/validate_governance.py` (`C-governance-clean-final.log`, SHA-256 `B077B7FE4B7ACA5B56DBAFA2F3A63403CBC799DF1E3D39AF393AF6F09F4DB633`), and staged plus unstaged `git diff --check` (`C-diff-check-clean-final.log`, SHA-256 `D5D2FA7570AA3087B69D05EFBE8DD55E25A63B735B455E6F579524771F2510F8`). A remains clean; B has only the M02 test file; C contains only Pack A, the fixture helper, and the governed Evidence/GEF/handoff updates. A fresh `git ls-remote` still resolves `origin/main` to `d2b750f917f841fe715aafea9e0f80fbd3bc1035`; no remote `feat/m04-run-state` branch exists yet.
