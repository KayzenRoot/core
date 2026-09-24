# CORE-WO-M04-001 — M04 Run / Attempt / Step Engine

Status: FINAL_FREEZE_CANDIDATE / IMPLEMENTATION_UNAUTHORIZED
Increment: CORE-M04-FREEZE-001
Module: M04 — Run / Attempt / Step Engine
Repository: KayzenRoot/core
Planning base: 0f448c708aafd8f5b4cb6303effdabd7e028bc6f
Planning branch: planning/m04-final-freeze-round5
Future execution branch: feat/m04-run-state
Risk / assurance: ELEVATED
Execution model: one comprehensive implementation Work Order, eight ordered construction packets
Product implementation authorization: FALSE — a separate governed execution-admission delta is required after this final freeze is independently reviewed and promoted
Context Lock: .engineering/context-locks/CORE-WO-M04-001.json — PENDING_PROMOTION
Evidence skeleton: .engineering/evidence/CORE-WO-M04-001.json
Executor handoff: docs/work-orders/CODEX-HANDOFF-M04.md

## OBJECTIVE

Implement the production-grade M04 Run / Attempt / Step Engine exactly from promoted M04 Rounds 1-4 and this final planning freeze.

M04 owns deterministic execution-state truth between one current admitted M03 Work Order handoff and later execution/policy/verification modules. It owns typed Run/Attempt/Step identity, lifecycle transitions, causal ordering, append-only history, generation/CAS semantics, idempotency, cancellation state, journal/replay, continuation, bounded references, derived snapshots and finite resource accounting. It does not execute host commands, mutate repositories, choose models/providers, own retry/recovery policy, verify later-module evidence truth, select a persistence backend, or grant implementation authority.

Do not reopen frozen architecture, contracts, dependency direction, M03/M04 ownership, journal authority, BRC/ICF semantics, prepared-versus-durable receipt semantics, acceptance meaning or deferred persistence decisions. If frozen semantics prove insufficient, stop and request the smallest governed Correction Delta.

## PROMOTED PLANNING BASIS

M04 Rounds 1-4 are promoted canonical planning truth.

Round 1 froze ownership, M03 READY/BRC gating, typed lineage direction, append-only history, causal ordering, fail-closed replay/publication and backend-neutral zero-LLM core behavior.

Round 2 froze closed Run/Attempt/Step transition matrices, terminal immutability, RunGeneration CAS serialization, fingerprint-bound idempotency, cancellation precedence, contiguous replay-authoritative journal semantics, ICF continuation/new-epoch behavior, BRC exactness and finite resource dimensions.

Round 3 froze versioned public request/receipt contracts, typed IDs/errors, canonical framing/fingerprint laws, closed event/reason registries, journal-authoritative snapshots, bounded external references, adapter direction and EV-M04-001..023.

Round 4 froze one `core-run-state` crate, the exact file/test/fuzz/benchmark map, minimal dependencies, pure `PreparedCommitV1<R>` semantics, `M04StateStoreV1`, caller-owned reference evidence, shared digest reuse through M04 framing, six fuzz targets, benchmark/calibration protocol and RAS/TLG/CER/RJR/BRC/ICF/ASF as required V0.0 mechanisms.

Round 4 review: M04-REVIEW-005 / Issue #87, PR #86, exact head `997fcccc79e786a271ba01a6fa7854bf1eaf8bce`, workflow `35945725379`, promotion merge `63af0f735dd0419fb02f9879efed340ddb07da30`.

Round 4 promotion synchronization: M04-REVIEW-006 / Issue #91, PR #88, exact head `6dfb25bc6f2ef176358698ecd7f622a1754ed383`, workflow `35978048885`, promotion merge `0f448c708aafd8f5b4cb6303effdabd7e028bc6f`.

The separately reviewed CI reliability correction is canonical before this planning base: PR #89 / Issue #90, merge `b34252891d3e0cd72183205e13cf46a372d09ba3`.

## HIVE PREFLIGHT

No direct HIVE MCP/connector is available in this planning chat environment. This freeze therefore uses exact Git canonical sources in SOLO mode and makes no current claim about CORE registration/checkpoint state in HIVE.

Before future product-code changes, the executor MUST:
1. resolve exact remote, canonical main, branch, HEAD and cleanliness;
2. attempt the optional HIVE v1.0.0-compatible read-only preflight if available;
3. record only observed project/context/checkpoint results;
4. use HIVE only as advisory/context acceleration;
5. continue safely in SOLO Git-canonical mode when HIVE is unavailable/unresolved, unless a later explicit frozen requirement changes that rule;
6. never treat another project, stale index or memory result as CORE authority.

## CANONICAL BASIS AND SOURCE HIERARCHY

Planning base: `0f448c708aafd8f5b4cb6303effdabd7e028bc6f`.

Candidate canonical source Git blob IDs:

| Canonical source | Candidate blob |
| --- | --- |
| docs/project-brain/13-CHECKPOINT.md | 3f3ecca766ca93956ac4c9f782368e0abfcf39da |
| docs/project-brain/16-DECISIONS-LEDGER.md | a55a912576d8279dfe2bde89f43f19e4a1dd7b16 |
| docs/project-brain/03-SCOPE.md | d8f917770f4b72ab0628be0da8744350289a21a5 |
| docs/project-brain/15-DEFINITION-OF-DONE.md | 864f1974b340fca9a5a2d2e7e297c8f4fcfe5553 |
| docs/project-brain/04-ARCHITECTURE.md | 09f56adc576ee477fe61c5588fee201eac841bc2 |
| docs/project-brain/02-REQUIREMENTS.md | b68bbdf41bbf06b1f1587261f39c31eee32e7cb9 |
| docs/project-brain/10-SECURITY-GOVERNANCE.md | 66a39179fb312eb229870aa5367e89708f6bfd66 |
| docs/project-brain/11-TEST-PLAN.md | 1784085e690e4fdfbdb5bd2b893d58f704c0fa6e |
| docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md | 882147c31c4dd2f5ed837474f4c914ba4c025c8f |

Authority remains governed by `.engineering/SOURCE-HIERARCHY.md`: Git is repository truth; Checkpoint is project-state authority; Decisions/ADRs govern decisions; Scope governs scope; Requirements and Architecture govern product contracts; DoD governs completion; an admitted Work Order governs execution only after separate admission; Test Plan and exact-head evidence govern validation. GEF/HIVE bridges are derived.

At implementation start, material change to the exact active lock, admitted base, this Work Order, any locked canonical source, governing policy or frozen module plan makes execution STALE and blocks affected progression until governed re-admission/correction.

## CONTEXT LOCK / AUTHORIZATION

The Round 5 lock is intentionally non-authoritative:
- status: PENDING_PROMOTION;
- authorizedBase: null;
- productImplementationAuthorized: false;
- active execution: forbidden.

Implementation is forbidden unless a later separate governed admission delta is independently reviewed/promoted and the exact lock on canonical `origin/main` becomes ACTIVE with:
- a concrete authorized base equal to the promoted final-freeze basis selected by admission;
- execution branch `feat/m04-run-state`;
- exact Work Order and canonical source fingerprints;
- ELEVATED assurance;
- explicit M04-only authorized scope;
- `productImplementationAuthorized = true`;
- canonical-main-gated activation.

A PR branch carrying ACTIVE-like text never grants authority.

## SCOPE

### Necessary future implementation

Implement only CORE-WO-M04-001:
- one focused `crates/core-run-state` crate;
- versioned Run/Attempt/Step/event/snapshot/reference/budget/error contracts;
- typed non-interchangeable IDs, ordinals, generation and epoch wrappers;
- deterministic M04 binary framing and domain separation over `core_identity::fingerprint_bytes`;
- closed TLG transitions, parent/child closure and terminal immutability;
- deterministic projections;
- append-only RJR journal, root chaining, replay and verified snapshots;
- CER RunGeneration fencing;
- fingerprint-bound idempotency;
- monotonic cancellation;
- BRC exact M03 authority validation;
- ICF interruption/continuation/new-epoch semantics;
- bounded external-reference evidence attachment;
- pure `prepare_*` functions returning `PreparedCommitV1<R>`;
- pure `finalize_commit` that releases operation-specific receipts only after exact durable receipt verification;
- host-facing `M04StateStoreV1` port contract with ASF all-or-nothing compare-and-commit semantics;
- finite evidence-calibrated `M04ResourceLimitsV1`;
- exact property/adversarial, fuzz, benchmark, security, supply-chain and cross-platform evidence below.

### Out of scope

Do not implement/add:
- a concrete production database/persistence backend;
- destructive canonical journal compaction/archive;
- distributed replication/consensus;
- persistent snapshot cache;
- runtime self-tuning;
- direct Tokio/core-runtime/core-workspace dependency in core-run-state;
- Git/HIVE/GitHub/network/process/database/filesystem/clock authority in the pure core;
- M05+ policy/executor/verification logic;
- random/time-derived semantic IDs;
- hidden retry/recovery policy;
- new product paths beyond the frozen map without a governed Correction Delta.

## FROZEN FILE MAP

Product paths:
- Cargo.toml
- Cargo.lock
- crates/core-run-state/Cargo.toml
- crates/core-run-state/src/lib.rs
- src/contracts.rs
- src/identity.rs
- src/canonical.rs
- src/transition.rs
- src/journal.rs
- src/projection.rs
- src/idempotency.rs
- src/cancellation.rs
- src/boundary.rs
- src/continuation.rs
- src/reference.rs
- src/budget.rs
- src/errors.rs
- src/store.rs
- src/service.rs

Test/evidence paths:
- crates/core-run-state/tests/public_contracts.rs
- tests/transitions.rs
- tests/cas_idempotency.rs
- tests/cancellation.rs
- tests/replay.rs
- tests/boundary_continuation.rs
- tests/canonical_identity.rs
- tests/snapshots.rs
- tests/references.rs
- tests/resources_no_io.rs
- benches/m04_run_state.rs
- fuzz/fuzz_targets/m04_transition_event.rs
- fuzz/fuzz_targets/m04_replay_journal.rs
- fuzz/fuzz_targets/m04_identity_canonical.rs
- fuzz/fuzz_targets/m04_brc_icf.rs
- fuzz/fuzz_targets/m04_reference.rs
- fuzz/fuzz_targets/m04_resource_limits.rs

Paths under `src/` and `tests/` are relative to `crates/core-run-state`.

## DEPENDENCY RULES

Allowed direct internal dependencies:
- core-work-order
- core-identity

Allowed existing third-party dependencies:
- serde with derive
- thiserror

Fuzz-only:
- existing libfuzzer-sys in the separate fuzz package.

`serde_json` is test/tooling-only unless a later exact governed proof demonstrates production necessity.

No direct production dependency is admitted on Tokio, core-runtime, core-workspace, Git libraries, HIVE/GitHub SDKs, network/process/filesystem/database/time APIs, graph/cache frameworks, Criterion, proptest or a new cryptography stack.

## REQUIREMENTS

The future implementation covers the exact M04 requirement set CORE-R-229 through CORE-R-284 as frozen by the promoted final planning freeze. No requirement may be silently omitted.

## CONSTRUCTION PACKETS

### Pack A — Contracts, identity and canonical framing
Create crate/workspace wiring, V1 contracts, typed IDs, ordinals/generations/epochs, closed event/reason/error registries, M04 domain framing and golden vectors.

STOP A: public contracts compile/round-trip; invalid IDs/versions/kinds/cross-domain substitutions fail typed; canonical vectors deterministic.

### Pack B — Lifecycle and projection
Implement Run/Attempt/Step TLG tables, terminal immutability, parent/child closure, ordinals and deterministic projection.

STOP B: all legal transitions accepted; all illegal transitions rejected; no terminal reactivation; no parent/child invariant breach.

### Pack C — Journal, replay and snapshots
Implement canonical events, sequence/root chaining, replay, derived projections and snapshot verification/rebuild.

STOP C: replay equals incremental projection; reorder/truncation/substitution/root corruption fails closed; snapshots cannot mint authority.

### Pack D — CAS, idempotency and cancellation
Implement RunGeneration fencing, prepared commits, idempotency records/conflicts and monotonic cancellation precedence.

STOP D: same-generation conflicting writers cannot both commit in the conformance model; exact replay is idempotent; conflicting key reuse and stale generations fail without partial semantic advancement.

### Pack E — BRC, ICF and external references
Implement exact M03 handoff/BRC checks, continuation/new epoch, bounded reference evidence and lineage checks.

STOP E: stale/substituted/UNKNOWN authority, continuation mismatch and cross-lineage references fail closed; raw external artifact bodies are not required durable state.

### Pack F — Resource/store/purity boundaries
Implement M04ResourceLimitsV1, budget checks, pure prepare/finalize service surface and M04StateStoreV1 conformance contract/fakes.

STOP F: cap+1 is atomic failure; no hidden I/O/LLM; pending receipts cannot become authority before exact durable finalization; store fakes cannot reinterpret semantics.

### Pack G — Security, properties, fuzz and supply chain
Complete deterministic property/adversarial laws, six bounded fuzz targets, secret canaries, dependency/static checks, advisory/license/SBOM evidence and both-platform suites.

STOP G: all named properties/fuzz/security/supply-chain gates pass on the exact implementation head with bounded resources and no secret echo.

### Pack H — Calibration, evidence, exact-head CI and handoff
Run `m04_run_state` calibration on deterministic synthetic fixtures, select finite positive resource defaults from evidence, apply only the authorized numeric/evidence Calibration Delta if needed, rerun affected tests, complete AC/EV mapping and open implementation PR.

STOP H: all AC-M04-001..023 except reviewer-owned AC-M04-023 have exact-head evidence; Windows/Ubuntu hosted gates pass; Evidence Bundle complete; no unresolved HIGH/CRITICAL executor finding; status READY_FOR_REVIEW.

## RESOURCE CALIBRATION GATE

Benchmark dimensions:
- attempts/run;
- steps/attempt;
- events/run;
- same-generation concurrent prepared operations;
- canonical event payload bytes;
- replay length;
- snapshot interval/rebuild;
- reference count/bytes;
- continuation cursor bytes.

Protocol:
- deterministic synthetic fixtures;
- exact source SHA/toolchain/OS/CPU/fixture version/command recorded;
- one warm-up plus at least five measured iterations for each supported scenario/platform;
- median/min/max recorded;
- each candidate finite limit and cap+1 exercised;
- unsupported scales reported, never extrapolated;
- no fabricated planning measurements.

One Calibration Delta may alter numeric limits/thresholds, associated fixture expectations and calibration evidence only. After it, every affected exact-head gate must rerun.

## ACCEPTANCE / EVIDENCE MAP

- AC-M04-001 -> EV-M04-001: public contract compile/API conformance.
- AC-M04-002 -> EV-M04-002: exhaustive legal transition matrix.
- AC-M04-003 -> EV-M04-003: illegal transition/property rejection.
- AC-M04-004 -> EV-M04-004: generation/CAS concurrency races.
- AC-M04-005 -> EV-M04-005: idempotency replay/conflict.
- AC-M04-006 -> EV-M04-006: cancellation precedence races.
- AC-M04-007 -> EV-M04-007: journal replay equivalence.
- AC-M04-008 -> EV-M04-008: reorder/truncation/substitution/root corruption rejection.
- AC-M04-009 -> EV-M04-009: BRC stale/substitution.
- AC-M04-010 -> EV-M04-010: ICF continuation/epoch.
- AC-M04-011 -> EV-M04-011: typed identity substitution.
- AC-M04-012 -> EV-M04-012: cross-platform canonical golden vectors.
- AC-M04-013 -> EV-M04-013: resource at-limit/cap+1 atomic failure.
- AC-M04-014 -> EV-M04-014: snapshot verification/rebuild equivalence.
- AC-M04-015 -> EV-M04-015: external-reference shape/lineage/bounds.
- AC-M04-016 -> EV-M04-016: no hidden I/O.
- AC-M04-017 -> EV-M04-017: zero-LLM core.
- AC-M04-018 -> EV-M04-018: six bounded fuzz campaigns.
- AC-M04-019 -> EV-M04-019: finite calibration report.
- AC-M04-020 -> EV-M04-020: dependency/advisory/license/SBOM.
- AC-M04-021 -> EV-M04-021: exact-head Windows CI.
- AC-M04-022 -> EV-M04-022: exact-head Ubuntu CI.
- AC-M04-023 -> EV-M04-023: independent exact-head review, zero unresolved HIGH/CRITICAL.

AC-M04-023 is reviewer-owned and cannot be satisfied by the executor.

## EVIDENCE RULES

Every implementation evidence record must bind the exact implementation head and name the command/artifact/platform/result. UNKNOWN, stale, missing or conflicting evidence is not PASS.

Do not claim implementation evidence during this planning increment. The Evidence Bundle skeleton must keep EV-M04-001..023 pending until future execution produces actual evidence.

## CORRECTION POLICY

If implementation finds a reproducible defect/impossibility:
1. stop affected packet and dependents;
2. preserve valid independent evidence;
3. record exact head, obligation, reproduction and impact;
4. apply only a correction already permitted by frozen scope or numeric Calibration Delta;
5. otherwise request the smallest governed Correction Delta;
6. rerun affected exact-head gates.

No architecture expansion by convenience.

## FINAL EXECUTOR STOP CONDITION

### READY_FOR_REVIEW

Only after Packs A-H are complete, AC-M04-001..022 have exact-head implementation evidence, calibration and post-calibration reruns are complete, required hosted CI/security/supply-chain evidence is green, the Evidence Bundle is complete, the implementation PR is open and no unresolved HIGH/CRITICAL executor finding remains.

READY_FOR_REVIEW is not approval.

### BLOCKED

If any authorization, source binding, lock, packet obligation, AC/EV, finite resource selection, CI/security gate or frozen contract is missing, stale, conflicting or requires out-of-scope change, stop and report the exact gap.

The executor MUST NEVER return APPROVED. Independent governed review owns AC-M04-023 and promotion authority.

## PLANNING STOP CONDITION

CORE-M04-FREEZE-001 stops after this Work Order, pending Context Lock, Evidence Bundle skeleton, executor handoff, canonical source bindings and candidate checkpoint/GEF state are independently reviewed. No product implementation is authorized by creation or promotion of this planning packet.
