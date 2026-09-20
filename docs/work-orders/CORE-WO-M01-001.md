# CORE-WO-M01-001 — Core Runtime & Lifecycle

Status: COMPLETED_APPROVED
Module: M01
Executor: Codex
Execution model: one Work Order, eight internal construction packets
Product implementation authorization: CLOSED — WORK ORDER COMPLETED

## Mission
Implement the production-grade Rust/Tokio CORE runtime substrate exactly from the frozen M01 planning basis. Do not redesign settled architecture. Preserve HIVE/CORE ownership boundaries and zero-LLM lifecycle.

## Canonical inputs
1. docs/project-brain/13-CHECKPOINT.md
2. docs/project-brain/16-DECISIONS-LEDGER.md
3. docs/project-brain/03-SCOPE.md
4. docs/project-brain/15-DEFINITION-OF-DONE.md
5. docs/project-brain/04-ARCHITECTURE.md
6. docs/project-brain/02-REQUIREMENTS.md
7. docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md
8. docs/research/CORE-TECHNOLOGY-CANDIDATES.md

## Context/cache rule
Treat settled architecture/invariants as STABLE PREFIX. Feed each packet only its delta, changed files and failing evidence. Do not rediscover the whole repository between packets unless canonical basis fingerprints changed.

## Required construction packets
A Contracts + deterministic identity.
B Configuration + admission.
C Registries + capability substitution.
D Journal + recovery.
E IPC + worker boundary.
F Health + degradation.
G Runtime + lifecycle.
H CLI + benchmarks/fuzz/security/evidence.

Packet STOP conditions are defined in the M01 module spec and are mandatory.

## Global invariants
- Rust stable + Tokio.
- Safe Rust by default.
- Headless, no UI/TUI.
- Zero LLM calls for M01 bootstrap/lifecycle/health/shutdown.
- No HIVE source/runtime/database dependency.
- No LLM provider SDK.
- No arbitrary dynamic-library plugins.
- No microservices/cluster requirement.
- No localhost TCP for first-party same-machine IPC by default.
- Deterministic canonical identity.
- Cache is derived, never canonical truth.
- No silent fallback below quality/policy floor.
- No ambiguous crash effect may be called successful.
- No unresolved HIGH/CRITICAL defect at completion.

## Acceptance criteria
1. All M01-required crates/contracts/mechanisms exist at governed boundaries.
2. Workspace dependency graph is acyclic and forbidden dependencies are absent.
3. RLC state transitions are machine-enforced with receipts.
4. BSR is required before READY/DEGRADED eligibility.
5. Module/capability registries are deterministic and support CAL/SIR/CBR/FCH.
6. Capability substitution is atomic and generation/epoch coherent.
7. DCS golden vectors prove stable canonical identity.
8. Runtime Journal integrity/recovery/corruption handling passes.
9. SBR/EEB reject stale epoch mutation and blind replay.
10. QDS/QVM distinguish clean shutdown from residual forced termination.
11. DCM/QFC enforce safe degradation and quality-floor continuity.
12. PHC/HCC/ODF health behavior passes concurrency/delta tests.
13. Local IPC is bounded, versioned, fuzzed and platform-adapted.
14. Security/supply-chain gates and unsafe inventory pass.
15. Unit/property/integration/fuzz/failure-injection suites pass.
16. Soak tests show no unbounded growth.
17. PRB/WNF benchmark evidence is reproducible and no unapproved material regression exists.
18. CLI machine outputs are versioned/stable.
19. Zero-LLM M01 proof passes.
20. Exact-head evidence bundle identifies commit/toolchain/platform/baselines.
21. Independent governed review verdict is APPROVED.

## Benchmark policy
First valid implementation establishes baseline on recorded hardware classes. Do not fabricate absolute targets. Subsequent changes compare only compatible WNF/hardware fingerprints. Material regression requires correction or approved ADR exception.

## Evidence bundle
At minimum: commit SHA; Rust/toolchain; dependency lock fingerprint; platform/hardware metadata; test/property/fuzz summaries; failure-injection summary; benchmark/PRB/WNF results; supply-chain/advisory/license/SBOM evidence; unsafe inventory; zero-LLM proof; known residual risks; exact file/change summary.

## Executor permissions
Codex may implement, test and make local corrective changes necessary to satisfy this frozen WO. It may not expand product scope, move HIVE-owned intelligence into CORE, weaken quality/security gates, silently change contracts, or proceed past an architectural blocker by inventing a replacement design.

## Blocker protocol
If a frozen decision is technically impossible or creates a demonstrable correctness/security defect:
1. stop affected packet;
2. preserve passing prior evidence;
3. report exact blocker + minimal reproduction;
4. propose smallest compatible correction;
5. do not redesign unrelated packets.

## FINAL STOP CONDITION
STOP only when all packet STOPs and all 21 acceptance criteria are satisfied with exact-head evidence, OR when a governed BLOCKED condition is reached that cannot be corrected without changing frozen architecture.

Do not claim completion from compilation alone.
Do not claim completion with failing/skipped required tests.
Do not claim completion with unresolved HIGH/CRITICAL findings.


## Completion record
Execution completed and was finally accepted by governed Review 011.
Final reviewed head: `aac0f143ea576a11013e4346076b8b3b4bd24282`
Final exact-head workflow: #54 `35488781894`
Final correction promotion merge: `d70b4296afbba93e8849ab6160e9b1caf5281e7d`
Result: COMPLETED_APPROVED. This Work Order is historical and grants no further implementation authority.
