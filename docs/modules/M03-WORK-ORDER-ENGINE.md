# M03 - Work Order Engine

Status: `DISCOVERY_IN_PROGRESS`

## Mission

M03 turns governed work intent into a deterministic, versioned, machine-verifiable Work Order that later CORE modules can execute without reinterpreting scope, source authority, acceptance criteria, evidence obligations, context budgets or stop conditions.

M03 is a compiler and admission boundary for work semantics. It does not execute commands, run attempts, mutate source, choose agents or models, collect final evidence, review results or deliver Git changes.

The central goal is to eliminate ambiguous execution prompts.

## Ownership boundary

### M03 owns

- Work Order request normalization and schema validation;
- deterministic WorkOrderId / revision / fingerprint semantics;
- canonical compiled Work Order representation;
- explicit scope, out-of-scope and allowed-delta policy;
- source/provenance manifest for canonical inputs;
- binding to M02 workspace identity/basis requirements;
- Context Lock references and staleness preconditions;
- risk / assurance requirements declared by the Work Order;
- context budget envelopes and compact context references;
- ordered execution-packet declarations and stable packet IDs;
- acceptance criteria and evidence-requirement declarations;
- final stop-condition semantics;
- correction-delta policy and allowed mutation classes;
- Work Order lineage, supersession and revision compatibility metadata;
- semantic diff classification between Work Order revisions;
- machine-readable readiness/rejection receipts consumed by M04.

### M03 does NOT own

- local project/workspace truth, path authority or repository basis, which belong to M02;
- durable Run / Attempt / Step execution state, which belongs to M04;
- host/IDE/tool-host integration, which belongs to M05;
- capability negotiation, which belongs to M06;
- specialist/agent registry, which belongs to M07;
- agent orchestration, which belongs to M08;
- model/effort routing, which belongs to M09;
- runtime policy enforcement beyond Work Order semantics, which belongs to M10;
- capability lease/sandbox authority, which belongs to M11;
- command/tool execution, which belongs to M12;
- source mutation, which belongs to M13;
- verification planning execution, which belongs to M14;
- final evidence collection/attestation, which belongs to M15;
- independent review/verdict, which belongs to M16;
- defect remediation orchestration, which belongs to M17;
- recovery/resume runtime behavior, which belongs to M18;
- Git/GitHub delivery, which belongs to M20;
- CI/release execution, which belongs to M21;
- HIVE durable memory/RAG/repository intelligence.

## Core invariant

CORE MUST NOT execute ambiguous human prose directly.

Before M04 may instantiate a Run, it must receive an admitted, immutable, machine-verifiable Work Order revision whose semantic fingerprint, scope, canonical-source bindings, workspace requirements, acceptance criteria, evidence obligations, risk/assurance profile and stop conditions are explicit.

## Operating modes

### Standalone mode

M03 compiles against deterministic local/canonical inputs:
- explicit Work Order request;
- canonical repository documents;
- M02 WorkspaceHandle / WorkspaceBasis references;
- local GEF governance artifacts;
- typed configuration/policy inputs.

Standalone mode must remain complete without HIVE.

### HIVE-enhanced mode

When compatible HIVE is available:
- HIVE may provide compact context references, retrieval results and project knowledge;
- HIVE context remains advisory input unless the canonical source hierarchy explicitly names it as source authority;
- HIVE cannot silently rewrite frozen Work Order semantics;
- HIVE context identity/provenance is recorded in the compiled source/context manifest;
- stale HIVE context may reduce assurance or trigger refresh but cannot override newer Git/filesystem truth.

## Round 1 state model candidate

```text
DRAFT
  -> NORMALIZING
  -> VALIDATING
  -> COMPILED
  -> FROZEN
  -> ADMISSION_PENDING
  -> READY

DRAFT/NORMALIZING/VALIDATING
  -> REJECTED

COMPILED/FROZEN/ADMISSION_PENDING/READY
  -> STALE

FROZEN/ADMISSION_PENDING/READY
  -> SUPERSEDED

STALE
  -> RECOMPILING
  -> FROZEN
```

Interpretation:
- DRAFT is non-executable authoring material.
- COMPILED is deterministic but not yet frozen/admitted.
- FROZEN means semantic content is immutable for that revision.
- ADMISSION_PENDING means external governance/workspace/context prerequisites still need proof.
- READY means the exact revision is eligible for M04 Run instantiation.
- STALE never silently returns to READY. Recompilation produces a new revision/fingerprint.
- SUPERSEDED is terminal for new execution.

State names are Round 1 candidates until later M03 rounds freeze the contract.

## Identity and revision model

M03 must distinguish:

1. `WorkOrderId`
   - stable logical lineage identity for one governed unit of work.

2. `WorkOrderRevision`
   - monotonic semantic revision within a WorkOrderId lineage.

3. `WorkOrderFingerprint`
   - deterministic fingerprint of the canonical semantic payload.

4. `WorkOrderCompilationId`
   - deterministic identity of compiler/schema/policy inputs plus WorkOrderFingerprint.

5. `WorkPacketId`
   - stable packet identity inside one Work Order revision.

6. `AcceptanceCriterionId`
   - stable identifier for one acceptance obligation.

7. `EvidenceRequirementId`
   - stable identifier for one evidence obligation.

8. `ContextManifestFingerprint`
   - compact fingerprint of admitted context/source references, not raw context text by default.

A changed semantic requirement, scope boundary, dependency, acceptance rule, stop condition or admitted source basis MUST produce a new semantic revision/fingerprint.

## Candidate compiled Work Order model

A compiled Work Order may contain:

- schema/version/kind;
- WorkOrderId + revision + fingerprint;
- title/mission;
- module/scope class;
- source/provenance manifest;
- required M02 workspace identity/basis/freshness profile;
- Context Lock reference/fingerprint;
- explicit scope and out-of-scope;
- allowed file/crate/path envelope;
- dependency constraints;
- requirements references;
- architecture/invariant references;
- risk/assurance classification;
- execution-packet declarations;
- packet ordering/dependency edges;
- context budget per packet;
- capability requirement declarations for later M06;
- host/tool requirement declarations for later M05/M12;
- acceptance criteria;
- evidence requirements;
- tests/benchmark/security obligations;
- correction-delta classes;
- final STOP CONDITION;
- supersession/lineage metadata;
- diagnostic metadata excluded from semantic identity where appropriate.

M03 defines these semantics. M04 owns their execution state.

## Canonical source model

M03 must bind a Work Order to canonical input references without embedding whole repositories or giant prompt transcripts.

Each source entry should carry:
- source class;
- canonical path or external capability reference;
- content/blob/fingerprint identity;
- authority/provenance;
- required/optional flag;
- semantic role;
- freshness/staleness policy.

Candidate source classes:
- CHECKPOINT;
- DECISION_LEDGER;
- SCOPE;
- DEFINITION_OF_DONE;
- ARCHITECTURE;
- REQUIREMENTS;
- SECURITY_POLICY;
- TEST_PLAN;
- MODULE_PLAN;
- WORKSPACE_BASIS;
- CONTEXT_LOCK;
- HIVE_CONTEXT_REFERENCE;
- GOVERNANCE_POLICY;
- PRIOR_EVIDENCE_REFERENCE.

## Scope model

Work Order scope must be mechanically distinguishable from prose commentary.

Candidate scope contract:
- allowed modules;
- allowed crates/packages;
- allowed path prefixes;
- allowed artifact classes;
- forbidden modules/paths;
- dependency-admission rule;
- product-code mutation allowed/denied;
- documentation/evidence mutation allowed/denied;
- generated artifact policy;
- calibration/correction delta classes.

A request outside frozen scope must fail typed or require a new Work Order revision.

## Work packet model

M03 packet declarations are execution semantics, not Run state.

Each packet candidate carries:
- WorkPacketId;
- objective;
- required canonical inputs;
- allowed mutation surface;
- required freshness profile;
- prerequisite packet IDs;
- acceptance subset;
- evidence subset;
- context budget;
- packet STOP condition.

M04 later instantiates runtime Run/Attempt/Step records from the immutable packet declarations.

## Acceptance / evidence model

M03 must compile a deterministic bipartite relationship:

```text
AcceptanceCriterion
       |
       +--> EvidenceRequirement
       +--> EvidenceRequirement
       |
       +--> optional Test/Benchmark/Security obligation refs
```

A criterion is not considered evidenced merely because a command passed. Later modules must bind actual EvidenceArtifacts to the declared EvidenceRequirement IDs.

M03 owns the declaration graph. M15 later owns evidence collection/attestation.

## Correction model

Existing Work Order revisions are immutable.

Corrections produce a new revision with an explicit semantic delta classification.

Candidate delta classes:
- EVIDENCE_ONLY;
- DOCUMENTATION_ONLY;
- TEST_ONLY;
- NUMERIC_CALIBRATION;
- IMPLEMENTATION_WITHIN_SCOPE;
- DEPENDENCY_ADMISSION;
- SCOPE_CHANGE;
- ARCHITECTURE_CHANGE;
- SECURITY_POLICY_CHANGE.

Only classes explicitly allowed by the frozen Work Order may proceed without returning to broader planning/governance.

## Token/context economy direction

M03 must minimize repeated LLM/context load while preserving correctness.

Default downstream representation should prefer:
- stable IDs;
- source fingerprints;
- compact manifests;
- packet-specific source refs;
- acceptance/evidence IDs;
- semantic diffs;
- unchanged stable-prefix references.

Large raw documents remain fetchable by reference but are not copied into every packet by default.

## Round 1 proprietary technology candidates

### WOC - Work Order Compiler

**Problem:** free-form prompts allow hidden ambiguity and executor reinterpretation.

**Mechanism:** deterministic normalization/validation/compiler pipeline from WorkOrderRequest + canonical refs + policy inputs into versioned canonical Work Order payload.

**Expected benefit:** reproducibility, smaller prompts, safer delegation, exact semantic identity.

**Primary risk:** compiler schema becomes too rigid.

**Promotion criterion:** semantically equivalent inputs compile to identical canonical payload/fingerprint; invalid/ambiguous required fields fail typed.

### SDF - Scope Delta Firewall

**Problem:** correction cycles often smuggle scope expansion into an apparently small fix.

**Mechanism:** compare requested/revised Work Order semantics against the frozen revision and classify changed semantic fields into allowed or forbidden delta classes.

**Expected benefit:** prevents accidental architecture/dependency/scope creep during correction.

**Primary risk:** false positives on harmless documentation changes.

**Promotion criterion:** differential fixture matrix proves semantic changes cannot masquerade as evidence/documentation-only deltas.

### AEG - Acceptance Evidence Graph

**Problem:** acceptance criteria and test/evidence output are often linked only by prose.

**Mechanism:** stable criterion IDs connected to explicit EvidenceRequirement IDs and obligation classes.

**Expected benefit:** machine-verifiable coverage, smaller review context, clean handoff to M14/M15/M16.

**Primary risk:** graph verbosity.

**Promotion criterion:** every frozen acceptance criterion is reachable from at least one required evidence obligation or carries a typed rationale for evidence-not-applicable.

### CBE - Context Budget Envelope

**Problem:** executors repeatedly ingest the full project context even when a packet needs only a narrow subset.

**Mechanism:** packet-level context budgets and canonical source-reference manifests define stable prefix versus packet delta material.

**Expected benefit:** lower token/input cost and faster executor startup while retaining exact provenance.

**Primary risk:** under-disclosure.

**Promotion criterion:** packet compilation never omits a source marked required by scope/risk/policy; context references are deterministic and can expand on demand.

### WLG - Work Order Lineage Graph

**Problem:** corrections, supersession and retries can blur which Work Order revision actually authorized execution.

**Mechanism:** immutable lineage edges such as REVISES, SUPERSEDES, CORRECTS and DERIVES_FROM.

**Expected benefit:** clear audit/recovery history and prevents execution from obsolete revisions.

**Primary risk:** unnecessary durable registry complexity.

**Promotion criterion:** lineage is representable as compact versioned contracts without requiring a database in V0.0.

### WSF - Work Staleness Frontier

**Problem:** any upstream source change can trigger unnecessary full recompilation, but ignoring changes is unsafe.

**Mechanism:** deterministic mapping from source/policy/workspace changes to affected Work Order semantic components and packet readiness.

**Expected benefit:** bounded recompile/delta work and token savings.

**Primary risk:** selective invalidation weaker than full validation.

**Promotion criterion:** selective staleness classification must be no weaker than full semantic recompilation; uncertain changes force broad stale state.

### WPC - Work Provenance Capsule

**Problem:** executors need proof of why a Work Order exists and what authority/source basis produced it, without copying full upstream documents.

**Mechanism:** compact provenance capsule containing canonical source fingerprints, governance refs, workspace basis refs, compiler/policy generation and lineage refs.

**Expected benefit:** proof-carrying execution handoff with low context cost.

**Primary risk:** provenance fields enter semantic identity unnecessarily.

**Promotion criterion:** correctness-relevant provenance is fingerprinted; diagnostic-only metadata is excluded deterministically.

## Round 1 failure taxonomy candidates

Typed failures should distinguish at least:
- UNSUPPORTED_SCHEMA;
- INVALID_WORK_ORDER;
- AMBIGUOUS_SCOPE;
- SOURCE_MISSING;
- SOURCE_STALE;
- WORKSPACE_STALE;
- CONTEXT_LOCK_STALE;
- GOVERNANCE_PROOF_MISSING;
- POLICY_CONFLICT;
- FORBIDDEN_DELTA;
- ACCEPTANCE_GAP;
- EVIDENCE_GAP;
- PACKET_DEPENDENCY_CYCLE;
- CONTEXT_BUDGET_INVALID;
- RESOURCE_BUDGET_EXCEEDED;
- SUPERSEDED_REVISION;
- COMPILATION_NONDETERMINISM;
- INTERNAL_INVARIANT_VIOLATION.

No failure may be silently converted into READY.

## Round 1 security / safety concerns

M03 is metadata-heavy but still security-relevant because later execution trusts it.

Threats:
- crafted Work Order widening path/module scope;
- source-fingerprint substitution;
- stale Context Lock replay;
- revision rollback/superseded execution;
- acceptance/evidence omission;
- packet dependency cycles;
- hidden dependency admission;
- secret material embedded into context/provenance fields;
- oversized Work Orders causing memory/token/resource abuse;
- inconsistent canonical serialization;
- semantic change disguised as documentation/evidence-only delta.

Round 1 direction:
- fail closed;
- explicit finite size/cardinality budgets;
- canonical sorting/fingerprinting;
- secret-safe compact refs;
- immutable revisions;
- no direct prose execution;
- zero-LLM compiler/validator baseline.

## Round 1 performance / token objectives

M03 should be cheap enough to run on every Work Order compile/admission boundary.

Initial objectives:
- zero LLM inference required for parsing/validation/canonical compilation;
- O(n log n) or better canonicalization over bounded Work Order collections;
- no repository tree scan owned by M03;
- reuse M02 workspace/basis fingerprints rather than rediscovering files;
- compact packet manifests instead of duplicated source bodies;
- deterministic semantic diff between revisions;
- bounded Work Order size/cardinality;
- downstream packet context should scale with packet needs, not total project size where canonical references permit.

Numeric budgets remain for later M03 rounds and must be evidence-backed rather than invented.

## Round 1 downstream contract

M04 should receive something equivalent to:

```text
AdmittedWorkOrderV1
  WorkOrderId
  WorkOrderRevision
  WorkOrderFingerprint
  WorkOrderCompilationId
  WorkspaceBinding requirement/ref
  ContextLock ref/fingerprint
  ScopeEnvelope
  RiskAssuranceProfile
  PacketSpecs[]
  AcceptanceCriteria[]
  EvidenceRequirements[]
  StopCondition
  CorrectionPolicy
  ContextManifest
  ProvenanceCapsule
```

M04 may create Run/Attempt/Step state from this contract but may not reinterpret or mutate its semantic meaning.

## Explicit out of scope for Round 1

- final crate/file map;
- final dependency set;
- persistence/database selection;
- executor implementation;
- runtime scheduling;
- Run/Attempt/Step state;
- agent/model routing;
- tool execution;
- source mutation;
- evidence collection engine;
- review engine;
- Git delivery;
- UI;
- final numeric resource limits.

## Round 1 questions deferred to later rounds

- exact Work Order v1 public schema;
- whether packet dependencies remain ordered-list-only or permit a bounded DAG;
- persistence model for lineage, if any;
- exact admission proof contract from GEF;
- exact M02 freshness profile required at compile versus run start;
- exact context budget units and hard ceilings;
- exact correction-delta compatibility matrix;
- exact crate/file map;
- exact fuzz/property/benchmark matrix;
- Work Order resource calibration protocol;
- final DoD / Work Order / Context Lock / executor packet.

## Round 1 STOP CONDITION

Round 1 is complete when:
- M03 ownership boundary is explicit;
- M03/M04 separation is explicit;
- immutable revision model is accepted as discovery baseline;
- canonical source/scope/packet/acceptance/evidence/context concepts are recorded;
- proprietary mechanism candidates are recorded with promotion criteria;
- initial threat/failure/performance directions are recorded;
- no product implementation is authorized.

M03 remains discovery-only after Round 1.
