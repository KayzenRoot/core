# CORE Technology Candidates

Status: `DISCOVERY_CANDIDATES_NOT_IMPLEMENTED`

These names describe research/design candidates. They are not implementation or performance claims.

## ACS - Adaptive Capability Substitution

CORE consumes abstract intelligence providers. In SOLO mode it uses bounded deterministic/local fallbacks. With HIVE available, HIVE-owned providers replace those fallbacks without changing CORE execution semantics.

Candidate provider contracts:
- ContextProvider;
- MemoryProvider;
- RepositoryIntelligenceProvider;
- DecisionProvider;
- KnowledgeProvider;
- EvidenceHistoryProvider.

Invariant: a fallback must not grow into a duplicate HIVE subsystem.

## TSS - Twin-State Synchronization

HIVE and CORE maintain separate owned state and synchronize by versioned envelopes/fingerprints. They do not share canonical database tables.

Stale intelligence or execution fingerprints fail closed when correctness depends on the changed basis.

## EOF - Evidence Once Fabric

Before expensive work, determine whether compatible evidence already exists. Reuse only when proof validity still holds; otherwise compute the minimum safe delta. HIVE may supply historical intelligence/evidence and CORE may return new execution proofs.

## ENS - Execution Nervous System

Headless structured event spine for work, agents, tools, mutations, verification, correction, review and delivery. HIVE and a future external NexLabs Console may consume the same event contracts.

## PCE - Proof-Carrying Execution

A run cannot transition into a stronger success state unless it carries the typed evidence required by policy for that transition.

## NSP - NexLabs Sync Protocol

Candidate bidirectional HIVE <-> CORE protocol.

HIVE -> CORE candidate Intelligence Capsule:
- project identity;
- canonical basis/checkpoint;
- scope/architecture/decisions;
- risk;
- bounded context/impact;
- relevant verified memory;
- proof obligations;
- context/resource budget;
- stop condition.

CORE -> HIVE candidate Execution Proof Capsule:
- run/attempt/step identity;
- changes;
- commands/tools;
- tests/proofs;
- reviews;
- failures/corrections;
- resource/model/effort evidence;
- residual risks;
- knowledge candidates;
- checkpoint proposal.

HIVE retains authority over durable knowledge promotion. CORE retains authority over execution state.


## M01 candidates
- **RLC — Runtime Lifecycle Calculus:** typed lifecycle rules + transition receipts.
- **CPG — Capability Provenance Graph:** provider origin/compatibility/health/trust/generation graph.
- **RSG — Runtime Safety Genome:** secret-free deterministic safety-basis fingerprint.
- **QDS — Quiescence-Driven Shutdown:** evidence-aware safe drain/cancel shutdown.
- **DCM — Degraded Capability Matrix:** partial safe operation by capability.
- **BSR — Bootstrap Safety Receipt:** proof of the exact basis used to claim READY/DEGRADED.


## Cross-cutting LLM/cache candidates
- **SCP — Stable Context Partitioning:** stable/semi-stable/delta/ephemeral partitioning.
- **CAG — Cache Affinity Graph:** targeted dependency-aware invalidation.
- **DIF — Deterministic Input Fingerprint:** canonical reusable request identity.
- **LCR — LLM Call Reuse Gate:** deterministic reuse/delta gate before token spend.
- **PSM — Prompt Stability Meter:** prefix stability and cache-churn measurement.
- **TEB — Token Economics Budget:** quality-aware token/cache/retry budget envelope.


## M01 Round 3 candidates
- **ZCP — Zero-Copy Context Handles:** immutable content-addressed handles reduce repeated payload copies/serialization across local components.
- **GCL — Generation Coherence Layer:** coherent config/module/capability/policy generations support targeted revalidation and cache invalidation.
- **DCS — Deterministic Canonical Serialization:** canonical bytes and golden vectors for fingerprints, receipts, cache keys and cross-language identities.


## M01 capability-fabric candidates
- **CAL — Capability Atomic Leasing:** immutable bounded leases keep provider/generation coherent across an operation.
- **SIR — Substitution Impact Radius:** computes affected consumers/leases/caches/safety operations before provider substitution.
- **CBR — Capability Binding Receipt:** canonical proof of provider selection/substitution.
- **FCH — Fallback Capability Harness:** deliberately bounded standalone fallbacks with explicit quality ceiling and provenance.
