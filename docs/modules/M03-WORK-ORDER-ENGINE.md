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


## Round 2 - v1 contract and admission semantics

Round 2 moves M03 from conceptual ownership into a near-frozen public semantic contract.

### Contract envelope

All durable/external M03 payloads use an explicit envelope:

```text
M03Envelope<T> {
  schema: "nexlabs.core.work-order",
  version: 1,
  kind: <typed kind>,
  payload: T
}
```

Rules:
- unsupported schema/version fails typed;
- semantic fingerprints include schema/version/kind;
- diagnostic timestamps, wall durations, display-only rendering metadata and transport IDs are excluded from semantic identity;
- unordered semantic collections are canonical-sorted;
- all fingerprints reuse the existing `core_identity::fingerprint` stack;
- raw secrets, access tokens and secret-bearing provider payloads are forbidden from durable Work Order contracts.

### Contract separation

M03 v1 separates authoring, frozen semantics, admission proof and runtime state.

```text
WorkOrderRequestV1
      |
      v
WorkOrderCompiler
      |
      v
FrozenWorkOrderV1   (immutable semantic revision)
      |
      +--> WorkOrderRevisionDiffV1
      |
      v
WorkOrderAdmissionRequestV1
      |
      +--> external governance proof
      +--> current M02 workspace/basis proof
      +--> current Context Lock proof
      +--> policy/config generation
      |
      v
WorkOrderAdmissionReceiptV1
      |
      v
AdmittedWorkOrderV1  (immutable handoff snapshot)
      |
      v
M04 Run instantiation
```

M03 does not embed mutable Run/Attempt/Step state into any of these contracts.

### WorkOrderRequestV1

Non-executable authoring input.

Candidate fields:
- requested logical WorkOrderId or AUTO_NEW;
- title / objective;
- target module/scope class;
- canonical source declarations/refs;
- requested scope/out-of-scope;
- requested packet declarations;
- acceptance/evidence declarations;
- risk/assurance request;
- context-budget request;
- correction-policy request;
- final stop-condition request;
- optional lineage parent;
- authoring provenance.

A request is never executable even if structurally valid.

### FrozenWorkOrderV1

The immutable semantic revision.

Candidate shape:

```text
FrozenWorkOrderV1 {
  work_order_id
  revision
  work_order_fingerprint
  compiler_contract
  semantic_spec
  source_manifest
  workspace_requirement
  context_lock_requirement
  governance_requirement
  scope_envelope
  risk_assurance_profile
  packet_graph
  acceptance_evidence_graph
  context_budget_envelope
  correction_policy
  stop_condition
  lineage
}
```

The revision number is monotonic within a WorkOrderId lineage, but ordering alone is never authority. The fingerprint is the semantic identity.

### WorkOrder semantic fingerprint

Included by default:
- schema/version;
- logical WorkOrderId + revision semantics;
- compiler/canonicalization contract generation;
- required canonical source identities;
- workspace requirement semantics;
- Context Lock requirement semantics;
- machine-readable scope/out-of-scope;
- dependency policy;
- risk/assurance semantics;
- packet graph;
- acceptance/evidence declaration graph;
- mandatory context-source requirements;
- correction policy;
- stop condition;
- semantic lineage parent.

Excluded by default:
- timestamps;
- PR/issue URLs used only diagnostically;
- wall-clock compile duration;
- UI labels/icons;
- rendering order when canonical sort order exists;
- actual runtime evidence artifacts produced later;
- M04 Run/Attempt/Step identities.

### Immutable revision law

A `FrozenWorkOrderV1` is never edited in place.

If semantic contract fields change:
- produce a new WorkOrderRevision;
- compute a new fingerprint;
- link lineage using a typed edge;
- previous READY admission becomes stale/superseded for new Run creation as applicable.

A correction to source code, tests or evidence that remains fully within the existing frozen semantic contract does NOT itself require a new Work Order revision.

## Round 2 identity contracts

### WorkOrderId

Stable logical lineage identity.

Properties:
- does not encode Git branch;
- does not encode current revision;
- does not change merely because implementation head changes;
- collision-resistant using existing identity primitives.

### WorkOrderRevision

Monotonic integer/newtype scoped to WorkOrderId.

Rules:
- starts at 1 for first frozen semantic revision;
- only semantic contract recompilation increments it;
- no decrement/reuse;
- revision number is not sufficient to prove content identity.

### WorkOrderFingerprint

Canonical semantic identity of one frozen revision.

### WorkOrderCompilationId

Fingerprint of:
- WorkOrderFingerprint;
- compiler schema/version;
- canonicalization algorithm version;
- relevant compiler policy/config generation.

This distinguishes the same semantic Work Order recompiled under materially different compiler semantics.

### WorkPacketId

Stable ID scoped to one WorkOrderId lineage.

Packet semantic change in a later Work Order revision may preserve WorkPacketId only when it remains the same logical responsibility. Split/merge creates new packet IDs plus lineage metadata.

### AcceptanceCriterionId / EvidenceRequirementId

Stable logical IDs scoped to WorkOrderId lineage.

Changes that materially alter obligation meaning require a new criterion/evidence ID or an explicit supersedes relation.

## Round 2 packet graph decision

M03 v1 uses a bounded DAG rather than a flat ordered list.

Each `WorkPacketSpecV1` includes:
- packet_id;
- objective;
- prerequisite_packet_ids;
- required_source_refs;
- required_workspace_freshness_profile;
- allowed_mutation_surface;
- required_capability_declarations;
- context_budget;
- acceptance_criterion_ids;
- evidence_requirement_ids;
- packet_stop_condition.

Rules:
- graph MUST be acyclic;
- all referenced packet IDs MUST exist;
- deterministic topological order uses WorkPacketId as tie-breaker;
- a packet dependency is semantic and fingerprinted;
- graph declaration does not itself authorize parallel execution;
- M04/M08 later decide legal runtime scheduling/orchestration under their own policies;
- no packet may widen parent Work Order scope.

This provides dependency precision without stealing M04 scheduling ownership.

## Round 2 source manifest contract

`CanonicalSourceRefV1` candidate fields:
- source_id;
- source_class;
- authority_domain;
- locator_kind;
- locator;
- semantic_fingerprint/blob_sha/capability_identity as applicable;
- required_for_compile;
- required_for_admission;
- required_for_packet_ids[];
- freshness_policy;
- provenance_class;
- secret_classification;
- expansion_policy.

Canonical source classes initially freeze as:
- CHECKPOINT;
- DECISION_LEDGER;
- SCOPE;
- DEFINITION_OF_DONE;
- ARCHITECTURE;
- REQUIREMENTS;
- SECURITY_POLICY;
- TEST_PLAN;
- MODULE_PLAN;
- GOVERNANCE_POLICY;
- WORKSPACE_BASIS;
- CONTEXT_LOCK;
- HIVE_CONTEXT_REFERENCE;
- PRIOR_EVIDENCE_REFERENCE;
- OTHER_VERSIONED_CAPABILITY.

HIVE_CONTEXT_REFERENCE is never canonical Git authority merely by class.

### Source expansion policy

Candidate values:
- ALWAYS_LOAD;
- PACKET_ON_DEMAND;
- VALIDATE_FINGERPRINT_ONLY;
- OPTIONAL_DIAGNOSTIC.

A source marked required by scope/risk/policy cannot be downgraded to OPTIONAL_DIAGNOSTIC merely to reduce tokens.

## Round 2 workspace requirement

M03 does not embed a live `WorkspaceHandle` in the frozen semantic payload because handles are runtime-epoch-bound.

Instead, `WorkspaceRequirementV1` declares:
- expected WorkspaceId / optional ProjectBindingId constraints;
- required BVM profile per Work Order and optionally packet;
- required repository/worktree identities when semantically necessary;
- allowed standalone/HIVE assurance mode;
- required workspace policy/security generation constraints;
- whether dirty/untracked state is permitted;
- basis compatibility policy.

Admission then checks these requirements against a fresh M02 handle/basis.

### Basis compatibility

Candidate statuses:
- EXACT_MATCH;
- COMPATIBLE_REFRESH;
- INCOMPATIBLE;
- UNKNOWN.

Rules:
- UNKNOWN never becomes READY;
- semantic workspace identity mismatch is INCOMPATIBLE;
- a compatible new generation may be admitted only when all Work Order-declared required basis semantics remain valid;
- M03 does not decide filesystem truth itself.

## Round 2 Context Lock requirement

`ContextLockRequirementV1` freezes:
- expected Context Lock schema/version;
- required WorkOrderId/revision binding;
- authorized product base constraint;
- canonical source fingerprint set or manifest fingerprint;
- implementation-authorized flag requirement;
- staleness rule identity.

Anti-circularity rule:
- `FrozenWorkOrderV1` fingerprints the *requirements/constraints* for a Context Lock, not the concrete Context Lock fingerprint when that lock itself binds back to the Work Order revision/fingerprint;
- the concrete Context Lock fingerprint is captured in `WorkOrderAdmissionReceiptV1` after the frozen Work Order already exists.

At admission:
- lock must be ACTIVE/current where execution requires it;
- Work Order semantic fingerprint must match the lock binding;
- material canonical-source mismatch yields STALE/BLOCKED;
- diagnostic-only source changes do not silently invalidate semantic fingerprints unless policy says they are correctness-relevant.

## Round 2 governance proof boundary

M03 may validate `GovernanceAdmissionProofV1`, but does not mint governance approval.

Candidate proof fields:
- governance schema/version;
- project/repository identity;
- WorkOrderId/revision/fingerprint;
- review/verdict reference;
- approved exact head/base constraints;
- authorized scope class;
- assurance mode;
- policy generation;
- proof fingerprint/signature capability ref if available.

Admission requires an external ACCEPTED/APPROVED proof satisfying policy.

Missing proof => `GOVERNANCE_PROOF_MISSING`.
Conflicting proof => `POLICY_CONFLICT`.
M03 never converts either into READY.

## Round 2 admission contracts

### WorkOrderAdmissionRequestV1

Contains references to:
- FrozenWorkOrderV1;
- current M02 WorkspaceHandle / WorkspaceBasis evidence;
- current Context Lock proof;
- GovernanceAdmissionProofV1;
- current compiler/policy/security/config generations;
- optional current HIVE context provenance;
- requested target admission mode.

### WorkOrderAdmissionReceiptV1

Durable proof of one admission evaluation, not an evergreen execution capability.

Candidate fields:
- work_order_id/revision/fingerprint;
- compilation_id;
- admission_status;
- workspace_id + admitted WorkspaceGeneration/BasisFingerprint;
- context_lock_fingerprint;
- governance_proof_fingerprint;
- policy/security/config generations;
- assurance result;
- source-manifest verification result;
- rejection/staleness reasons;
- admission_fingerprint;
- diagnostic timestamp.

Admission statuses freeze as:
- READY;
- REJECTED;
- STALE;
- BLOCKED;
- SUPERSEDED.

A receipt is itself immutable evidence of one evaluation. A later change does not mutate an old READY receipt into STALE; a fresh admission evaluation yields a new receipt/status. M04 MUST re-check the receipt's required freshness/binding preconditions at Run creation; it is not a perpetual capability.

### AdmittedWorkOrderV1

Compact M04 handoff snapshot:
- FrozenWorkOrderV1 ref/fingerprint;
- READY WorkOrderAdmissionReceiptV1 ref/fingerprint;
- packet graph;
- scope envelope;
- acceptance/evidence IDs;
- context manifest;
- stop condition.

It does not contain mutable runtime status.

## Round 2 ScopeEnvelopeV1

Candidate structure:
- allowed_modules[];
- allowed_crates_or_packages[];
- allowed_path_prefixes[];
- forbidden_path_prefixes[];
- allowed_artifact_classes[];
- forbidden_artifact_classes[];
- source_mutation_policy;
- documentation_mutation_policy;
- evidence_mutation_policy;
- generated_artifact_policy;
- dependency_policy;
- allowed_correction_classes[];
- maximum_scope_class.

Rules:
- deny beats allow;
- ambiguous path/module classification fails closed;
- child packet scope is intersection with parent Work Order scope;
- dependency addition is never inferred from crate/path permission;
- M02 path authority proof remains separately required at use time.

## Round 2 semantic delta model

Two distinct concepts are frozen.

### ExecutionCorrectionProposalV1

A proposal to change implementation/tests/evidence while keeping the same FrozenWorkOrderV1.

Examples:
- implementation bug fix inside scope;
- test-only correction already permitted;
- evidence regeneration;
- numeric calibration when explicitly permitted.

SDF classifies the proposal against `CorrectionPolicyV1`.

If permitted, the Work Order revision does not change.

### WorkOrderRevisionDiffV1

A semantic comparison between two FrozenWorkOrder revisions.

Field-level changes map to classes:
- SEMANTIC_OBJECTIVE_CHANGE;
- SCOPE_CHANGE;
- DEPENDENCY_POLICY_CHANGE;
- ARCHITECTURE_RULE_CHANGE;
- SECURITY_POLICY_CHANGE;
- PACKET_GRAPH_CHANGE;
- ACCEPTANCE_CHANGE;
- EVIDENCE_REQUIREMENT_CHANGE;
- CONTEXT_REQUIREMENT_CHANGE;
- STOP_CONDITION_CHANGE;
- GOVERNANCE_REQUIREMENT_CHANGE;
- WORKSPACE_REQUIREMENT_CHANGE;
- NON_SEMANTIC_DIAGNOSTIC_CHANGE.

Any change to a field stored inside `FrozenWorkOrderV1` requires a new immutable object; semantic fields require a new WorkOrderRevision/fingerprint. `NON_SEMANTIC_DIAGNOSTIC_CHANGE` applies only to external diagnostic/transport/rendering metadata that is not stored as mutable content inside the frozen revision and does not alter its semantic fingerprint.

A field cannot be classified as non-semantic merely because its textual diff is small.

## Round 2 correction-policy matrix

`CorrectionPolicyV1` explicitly states which ExecutionCorrection classes are allowed under the same frozen revision.

Initial classes:
- IMPLEMENTATION_WITHIN_SCOPE;
- TEST_ONLY_WITHIN_SCOPE;
- EVIDENCE_REGENERATION;
- DOCUMENTATION_WITHIN_SCOPE;
- NUMERIC_CALIBRATION;
- GENERATED_ARTIFACT_REFRESH.

Never same-revision by default:
- DEPENDENCY_ADMISSION;
- SCOPE_EXPANSION;
- ARCHITECTURE_CHANGE;
- SECURITY_POLICY_CHANGE;
- ACCEPTANCE_WEAKENING;
- STOP_CONDITION_WEAKENING.

A project may explicitly make a class stricter. It cannot silently make forbidden semantic mutation non-semantic.

## Round 2 Acceptance Evidence Graph v1

`AcceptanceCriterionV1`:
- criterion_id;
- statement;
- obligation_class;
- required_evidence_ids[];
- packet_ids[];
- blocking_severity;
- applicability_policy.

`EvidenceRequirementV1`:
- evidence_id;
- evidence_class;
- producer_module_hint;
- exact_head_required;
- platform_requirements[];
- freshness_policy;
- packet_ids[];
- security_classification.

Graph rules:
- every blocking criterion has >=1 required evidence edge unless applicability policy deterministically proves N/A;
- every required evidence ID resolves;
- orphan required evidence is rejected unless explicitly global;
- M03 declares requirements only; M15 later binds actual evidence artifacts;
- acceptance criteria cannot be weakened by deleting edges under a same-revision correction.

## Round 2 StopConditionV1

Machine-readable terminal condition declaration.

Candidate fields:
- success_predicates[];
- blocked_predicates[];
- prohibited_early_exit_conditions[];
- required_acceptance_criterion_ids[];
- required_evidence_requirement_ids[];
- reviewer_verdict_required;
- checkpoint_promotion_allowed flag (normally false for executor).

A Work Order without explicit terminal semantics cannot become READY.

## Round 2 context budget contract

`ContextBudgetEnvelopeV1` freezes dimensions, not final numeric defaults yet:
- max_manifest_entries;
- max_inline_bytes;
- max_expanded_source_bytes;
- max_packet_inline_bytes;
- max_packet_expanded_bytes;
- max_hive_refs;
- max_prior_evidence_refs;
- expansion_deadline;
- allowed_expansion_reasons[];
- mandatory_source_override behavior.

Rules:
- budgets reduce payload, never correctness obligations;
- exceeding a context budget yields typed CONTEXT_BUDGET_INVALID / RESOURCE_BUDGET_EXCEEDED or requires explicit governed expansion;
- no silent truncation of mandatory semantic data;
- source refs/fingerprints remain preferred to full copies.

Numeric defaults remain calibration-gated for later rounds.

## Round 2 staleness model

WSF tracks correctness-relevant dependency classes:
- WORK_ORDER_SEMANTICS;
- CANONICAL_SOURCE;
- WORKSPACE_BASIS;
- CONTEXT_LOCK;
- GOVERNANCE_PROOF;
- COMPILER_POLICY;
- SECURITY_POLICY;
- CONTEXT_REFERENCE.

Change hints may nominate affected classes but cannot prove absence of change.

Admission revalidation rules:
- required fingerprints/generations are verified deterministically;
- UNKNOWN broadens to STALE/BLOCKED;
- selective validation must be no weaker than full required dependency verification;
- a stale admission receipt cannot mint a new M04 Run.

## Round 2 proprietary mechanism disposition

Promoted from candidate to REQUIRED FOR M03 V0.0 semantics:
- WOC Work Order Compiler;
- SDF Scope Delta Firewall;
- AEG Acceptance Evidence Graph;
- CBE Context Budget Envelope;
- WLG Work Order Lineage Graph;
- WSF Work Staleness Frontier;
- WPC Work Provenance Capsule.

"Required" here means the semantic capabilities are part of the planned V0.0 contract. Implementation shape/file map remains unfrozen.

## Round 2 unresolved items

Still pending:
- exact Rust crate/file map;
- dependency graph;
- exact serialization structs/enums and error Rust API;
- resource budget numeric defaults/calibration protocol;
- exact compiler/admission service interfaces;
- persistence strategy, if any;
- exact fuzz/property targets;
- benchmarks and thresholds;
- final DoD;
- final Work Order/Context Lock/executor packet.

## Round 2 STOP CONDITION

Round 2 is complete when:
- v1 contract separation is accepted;
- packet DAG semantics are accepted;
- source/workspace/Context Lock/governance bindings are explicit;
- admission receipt is explicitly non-evergreen;
- same-revision execution corrections are separated from semantic Work Order revisions;
- ScopeEnvelope/SDF delta laws are explicit;
- Acceptance Evidence Graph v1 semantics are explicit;
- StopCondition/context budget/staleness contracts are explicit;
- M03 implementation remains unauthorized.


## Round 3 - compiler services, lineage concurrency and resource architecture

Round 3 freezes the M03 service model and implementation direction while keeping product implementation unauthorized.

## Stateless compiler/service law

M03 V0.0 is stateless-by-default.

The core Work Order engine:
- does not own a database;
- does not own Git persistence;
- does not allocate mutable Run state;
- does not maintain a hidden canonical Work Order registry;
- does not fetch HIVE context on its own;
- does not scan the repository tree;
- does not perform network I/O.

It consumes explicit resolved inputs/snapshots and produces deterministic contracts/receipts.

Durability remains with the repository/GEF/source-of-truth domain until a later module explicitly owns a durable registry.

## Round 3 service interfaces

The semantic service surface freezes around these operations.

### compile

```text
compile(
  WorkOrderRequestV1,
  CompilationContextV1
) -> WorkOrderCompilationV1
```

`CompilationContextV1` contains only explicit resolved inputs:
- compiler/policy/config generation;
- canonical source manifest inputs;
- current lineage snapshot;
- project/module namespace;
- applicable scope/security policy;
- optional HIVE context refs already resolved by an external context provider.

Output:
- FrozenWorkOrderV1;
- WorkOrderCompilationId;
- LineagePreconditionCapsuleV1;
- CompilationReceiptV1;
- deterministic warnings/diagnostics outside frozen semantics.

### validate_frozen

```text
validate_frozen(FrozenWorkOrderV1) -> WorkOrderValidationReceiptV1
```

Checks canonical serialization, IDs, packet DAG, scope, AEG, stop condition, correction policy, source manifest and finite resource limits.

### diff_revision

```text
diff_revision(
  FrozenWorkOrderV1 before,
  FrozenWorkOrderV1 after
) -> WorkOrderRevisionDiffV1
```

Produces deterministic field/semantic-class deltas.

### classify_correction

```text
classify_correction(
  FrozenWorkOrderV1,
  ExecutionCorrectionProposalV1
) -> CorrectionClassificationReceiptV1
```

Returns:
- ALLOWED_SAME_REVISION;
- REQUIRES_NEW_REVISION;
- REQUIRES_DEPENDENCY_ADMISSION;
- REQUIRES_SECURITY_GOVERNANCE;
- REJECTED_OUT_OF_SCOPE;
- BLOCKED_AMBIGUOUS.

It never mutates the Work Order.

### evaluate_admission

```text
evaluate_admission(
  FrozenWorkOrderV1,
  WorkOrderAdmissionInputsV1
) -> WorkOrderAdmissionReceiptV1
```

Inputs include already-resolved:
- M02 WorkspaceHandle/basis evidence;
- Context Lock evidence;
- external governance proof;
- current source-manifest verification;
- compiler/policy/security/config generations;
- lineage/supersession snapshot.

### materialize_handoff

```text
materialize_handoff(
  FrozenWorkOrderV1,
  READY WorkOrderAdmissionReceiptV1
) -> AdmittedWorkOrderV1
```

Fails if receipt identity does not bind exactly to the frozen revision.

## No hidden I/O rule

The compiler/service core does not open arbitrary repository paths, run Git, call HIVE, query GitHub or write Work Order files.

External adapters resolve those concerns and provide typed evidence.

This:
- improves determinism;
- minimizes dependency/supply-chain surface;
- makes property/fuzz testing easier;
- avoids duplicated M02/HIVE/Git responsibilities;
- permits future CLI/API/daemon adapters without changing compiler semantics.

## Work Order logical ID allocation

Round 2's AUTO_NEW direction is refined.

M03 V0.0 MUST NOT generate opaque random WorkOrderIds internally.

A new Work Order uses one of:
- explicit caller-provided WorkOrderId; or
- deterministic `WorkOrderLogicalKeyV1` containing project namespace + module/scope namespace + stable logical key.

The compiler derives the ID using the existing core-identity stack.

Sequence-number allocation such as `CORE-WO-M03-001` remains an external repository/governance concern unless a later durable registry is admitted.

This avoids hidden randomness and distributed sequence races inside M03.

## LineageSnapshotV1

To compile a new semantic revision, the caller supplies a trusted lineage snapshot.

Candidate fields:
- WorkOrderId;
- latest known revision;
- latest frozen fingerprint;
- latest compilation ID;
- superseded revision set/fingerprint summary;
- lineage edge summary;
- source/store generation;
- provenance/fingerprint.

M03 validates the snapshot but does not own its persistence.

## LPC - Lineage Precondition Capsule

**Problem:** stateless compilers can race: two planners may both derive revision N+1 from the same parent.

**Mechanism:** every compilation that creates a new revision emits `LineagePreconditionCapsuleV1`:

```text
expected_work_order_id
expected_parent_revision
expected_parent_fingerprint
expected_store_generation
new_revision
new_work_order_fingerprint
lineage_precondition_fingerprint
```

The external durable writer MUST compare the expected parent/store generation before making the new revision canonical.

If the lineage advanced meanwhile:
- write fails with LINEAGE_CONFLICT;
- compiler output is not silently rebased;
- caller must re-resolve lineage and recompile.

**Expected benefit:** compare-and-set semantics without M03 owning a database/lock service.

**Promotion criterion:** concurrency fixtures prove two competing N+1 candidates cannot both become canonical under the same lineage precondition.

LPC is REQUIRED for V0.0 whenever a new revision is persisted.

## Revision numbering rules

- first semantic revision = 1;
- subsequent semantic revision = previous canonical revision + 1;
- no gaps under normal persistence;
- no revision reuse;
- rejected/unpersisted candidate revisions do not become canonical history;
- fingerprint, not revision number, proves semantic identity;
- external store commits lineage atomically with its precondition where supported.

## External persistence disposition

M03 V0.0 selects NO internal database.

Canonical persisted Work Orders may remain Git-tracked files under the governed repository workflow.

M03 exposes canonical serialization bytes/contracts for an external writer but does not:
- create commits;
- push branches;
- resolve Git conflicts;
- update checkpoint;
- self-promote a Work Order.

Future database/index support may be added as a storage adapter only if evidence proves Git/file persistence inadequate.

## Canonical serialization service

M03 reuses core-identity canonical fingerprint primitives.

Round 3 direction:
- typed structs -> canonical semantic projection;
- explicitly sorted collections;
- no map iteration identity;
- UTF-8 validation;
- normalized enum/string domains;
- bounded string/list sizes;
- deterministic JSON may be used for evidence/display, but fingerprint semantics depend on the canonical projection rather than incidental pretty-print formatting.

No second generic canonical-hash framework is introduced.

## Packet Context Plan

CBE compilation produces a compact `PacketContextPlanV1` for each packet:
- stable_prefix_source_ids[];
- packet_required_source_ids[];
- validate_only_source_ids[];
- optional_expandable_source_ids[];
- context_budget dimensions;
- expansion reasons;
- PacketContextManifestFingerprint.

This plan does not contain executor prompt text.

M04/M08/HIVE-aware context adapters later materialize actual context from these refs.

### Token-economy invariant

If two packets share stable sources, M03 emits shared refs/fingerprints rather than duplicate raw content.

A source body enters downstream prompt/context only when required by expansion policy.

## Optional derived compile memoization

M03 may expose an in-memory derived compilation memo/cache candidate, but it is never source truth.

Candidate cache key:
- normalized WorkOrderRequest semantic fingerprint;
- CompilationContext semantic fingerprint;
- compiler schema/algorithm version;
- policy/config generation.

Cache hit must reproduce the identical WorkOrderCompilationV1.

Cache loss/corruption degrades performance only.

Persistent compile cache is OUT OF SCOPE for V0.0.

Whether L1 memoization ships initially remains benchmark-gated; correctness cannot depend on it.

## M03ResourceBudget dimensions

Round 3 freezes resource dimensions, not numeric defaults.

Required dimensions:
- max_request_bytes;
- max_frozen_work_order_bytes;
- max_string_bytes;
- max_source_manifest_entries;
- max_packets;
- max_packet_dependency_edges;
- max_scope_rules;
- max_acceptance_criteria;
- max_evidence_requirements;
- max_acceptance_evidence_edges;
- max_lineage_edges;
- max_context_refs;
- max_correction_rules;
- max_semantic_diff_entries;
- max_diagnostic_entries;
- max_compile_wall_clock;
- max_validate_wall_clock;
- max_diff_wall_clock;
- max_admission_wall_clock.

Rules:
- all security-sensitive dimensions are finite after calibration;
- zero/unlimited sentinel cannot bypass a security bound;
- budget breach is typed and produces no partially READY Work Order;
- no automatic persistent self-tuning at runtime;
- numeric defaults require reproducible calibration evidence.

## M03 Resource Calibration Gate direction

M03 will use an evidence-driven Resource Calibration Gate analogous in principle to M02 but specific to metadata/compiler workloads.

Planning freezes:
- dimensions;
- fixture families;
- measurement semantics;
- safety rules.

Implementation later measures:
- compile/validate/diff/admission latency;
- serialized/canonical payload sizes;
- memory behavior under graph/cardinality extremes;
- context manifest size reductions;
- source/packet/criterion scaling.

Numeric defaults remain an implementation-evidence output before final promotion.

## Resource fixture families

Candidate deterministic synthetic fixtures:
- minimal valid Work Order;
- many canonical sources;
- wide packet DAG;
- deep-but-bounded packet DAG;
- dense dependency DAG;
- many scope allow/deny rules;
- many criteria/evidence edges;
- large lineage snapshot;
- large revision diff;
- many context refs;
- near-budget strings;
- invalid cycles/dangling refs;
- adversarial repeated IDs;
- stale admission input matrix.

No network/LLM required.

## Exact error model direction

M03 errors freeze into categories plus machine-readable reason codes.

### INVALID_INPUT
- UNSUPPORTED_SCHEMA;
- INVALID_ID;
- INVALID_REVISION;
- INVALID_ENUM_VALUE;
- NON_CANONICAL_INPUT;
- DUPLICATE_ID;
- DANGLING_REFERENCE;
- PACKET_DEPENDENCY_CYCLE;
- ACCEPTANCE_GAP;
- EVIDENCE_GAP;
- INVALID_STOP_CONDITION;
- INVALID_CONTEXT_BUDGET.

### STALE_OR_CONFLICT
- SOURCE_STALE;
- WORKSPACE_STALE;
- CONTEXT_LOCK_STALE;
- GOVERNANCE_PROOF_STALE;
- SUPERSEDED_REVISION;
- LINEAGE_CONFLICT;
- COMPILER_POLICY_CHANGED;
- SECURITY_POLICY_CHANGED.

### POLICY_BLOCK
- AMBIGUOUS_SCOPE;
- FORBIDDEN_DELTA;
- DEPENDENCY_ADMISSION_REQUIRED;
- SECURITY_GOVERNANCE_REQUIRED;
- GOVERNANCE_PROOF_MISSING;
- GOVERNANCE_PROOF_MISMATCH;
- SECRET_MATERIAL_REJECTED.

### RESOURCE_BLOCK
- REQUEST_TOO_LARGE;
- CARDINALITY_LIMIT_EXCEEDED;
- SERIALIZED_SIZE_EXCEEDED;
- COMPILATION_DEADLINE_EXCEEDED;
- ADMISSION_DEADLINE_EXCEEDED.

### INTERNAL
- COMPILATION_NONDETERMINISM;
- FINGERPRINT_INVARIANT_VIOLATION;
- INTERNAL_INVARIANT_VIOLATION.

Errors carry:
- category;
- reason code;
- safe context IDs/fingerprints;
- bounded diagnostics;
- retryability class;
- no raw secret content.

## Retryability model

Candidate values:
- NON_RETRYABLE_INPUT;
- RETRY_AFTER_REFRESH;
- RETRY_AFTER_GOVERNANCE;
- RETRY_AFTER_RESOURCE_CHANGE;
- INTERNAL_BUG.

M03 never retries hiddenly because refreshing lineage/workspace/governance can change semantic inputs.

## Initial Rust crate direction

Round 3 chooses a one-crate direction for M03 V0.0:

`crates/core-work-order/`

Candidate source decomposition:

```text
src/
  lib.rs
  contracts.rs
  errors.rs
  identity.rs
  canonical.rs
  compiler.rs
  scope.rs
  packets.rs
  acceptance.rs
  context.rs
  lineage.rs
  delta.rs
  admission.rs
  budget.rs
  service.rs
```

Candidate tests:

```text
tests/
  compile.rs
  canonical.rs
  identity.rs
  scope.rs
  packets.rs
  acceptance.rs
  lineage.rs
  delta.rs
  admission.rs
  context.rs
  resources.rs
  adversarial.rs
```

Candidate benches:
- `benches/m03_work_order.rs`

Exact file map remains candidate until later freeze.

## Dependency direction

Candidate direct internal dependencies:
- core-contracts;
- core-identity;
- core-config;
- core-workspace.

Candidate workspace third-party dependencies:
- serde;
- serde_json;
- sha2;
- thiserror.

No Tokio is required by the core compiler/service baseline.

No database, async runtime, Git library, watcher framework, graph framework, regex engine, LLM SDK or HIVE runtime dependency is currently justified.

New dependency admission later requires explicit evidence.

**Round 4 refinement:** the Round 3 lists above are historical candidate directions, not the final admitted set. Round 4 supersedes them for initial V0.0 implementation: direct dependencies are limited to `core-contracts`, `core-identity`, `core-workspace`, `serde`, `serde_json` and `thiserror`. `core-config` and direct `sha2` are not admitted into `core-work-order`; resource contracts are M03-owned and hashing is reused through `core-identity`.

## Dependency graph invariant

```text
core-contracts
core-identity
core-config
core-workspace
       \   |   /
        core-work-order
             |
             v
          future M04
```

`core-workspace` MUST NOT depend back on `core-work-order`.

M03 must not depend on M04+.

## Compiler determinism proof

Implementation must eventually prove:
- semantically equivalent request permutations compile identically;
- repeated compile with same context is byte/fingerprint equivalent;
- cache/no-cache output equivalent;
- canonical serialization independent of hash-map iteration;
- diagnostics excluded from semantic identity;
- adversarial malformed input cannot produce partially frozen output.

## Admission determinism proof

With identical frozen revision + resolved admission inputs:
- receipt semantic fields/fingerprint are identical;
- diagnostics/timestamp may differ outside semantic fingerprint;
- READY decision is deterministic;
- UNKNOWN never becomes READY.

## Round 3 proprietary additions

### LPC - Lineage Precondition Capsule
Promoted to REQUIRED semantic mechanism for canonical revision persistence.

### PCM - Packet Context Mesh

**Problem:** packets share many sources but naive manifests duplicate references and future prompt bodies.

**Mechanism:** deduplicate stable source refs across packets into a shared canonical mesh plus per-packet edge sets.

**Expected benefit:** reduced manifest/prompt repetition and deterministic context reuse.

**Primary risk:** hidden source omission through over-deduplication.

**Promotion criterion:** reconstructing each PacketContextPlan from PCM yields exactly the same mandatory source set as independent compilation.

PCM is REQUIRED semantically as the deduplicated representation strategy; exact internal data structure remains implementation-defined.

### DCR - Deterministic Compilation Receipt

**Problem:** reviewers need to know exactly which compiler/policy/source inputs produced a FrozenWorkOrder.

**Mechanism:** `CompilationReceiptV1` records request/context fingerprints, compiler algorithm/schema version, lineage precondition fingerprint, output fingerprint and bounded diagnostics.

**Expected benefit:** reproducible compiler provenance without copying source bodies.

DCR is REQUIRED for V0.0.

## Round 3 unresolved items

Still pending before implementation freeze:
- final exact Rust file map;
- final dependency admission;
- numeric M03ResourceBudget defaults and calibration protocol details;
- exact public Rust struct/enum signatures;
- exact source resolver/adaptor interfaces outside core compiler;
- fuzz target list;
- property test laws and benchmark thresholds;
- final production DoD;
- final Work Order / Context Lock / executor packet.

## Round 3 STOP CONDITION

Round 3 is complete when:
- stateless core/service law is frozen;
- compile/validate/diff/classify/admit/handoff operations are frozen;
- no-hidden-I/O boundary is frozen;
- deterministic WorkOrderId allocation direction is frozen;
- external lineage snapshot + LPC concurrency model is frozen;
- no-internal-database persistence direction is frozen;
- resource budget dimensions are frozen;
- error/retryability taxonomy is frozen;
- one-crate/dependency direction is recorded;
- PCM and DCR are recorded;
- implementation remains unauthorized.

## Round 4 - public Rust contract freeze, adapter boundaries and implementation map

Round 4 turns the Round 1-3 semantic model into an executor-addressable Rust contract and exact initial implementation surface without authorizing product implementation.

It freezes:
- the exact initial `core-work-order` crate/file map;
- the final initial dependency set;
- top-level public Rust entry points and contract ownership;
- external resolution/adaptor data boundaries;
- semantic/canonical field ownership rules;
- exact fuzz/property target families;
- benchmark/calibration protocol and threshold policy;
- production DoD direction required before the final planning freeze.

Numeric `M03ResourceBudget` defaults remain implementation-evidence outputs. Round 4 freezes how they are measured and admitted; it does not invent production numbers.

### Round 4 contract ownership law

M03 v1 contracts are owned by `core-work-order` unless they already belong to an accepted lower module.

Lower-module types remain reused by reference/type dependency:
- `core_contracts::SchemaVersion` and shared assurance/runtime primitives;
- `core_identity::canonical_bytes` / `fingerprint`;
- M02 public workspace identity/freshness contracts from `core-workspace`.

M03 MUST NOT duplicate M02 workspace identity, canonical hashing, runtime generation, or shared assurance contracts.

M03-specific durable contracts MUST NOT be added to `core-contracts` merely to make them globally visible. They remain in `core-work-order` until a later proven cross-module extraction need exists.

### Public constants

The initial crate exports:

```rust
pub const M03_SCHEMA: &str = "nexlabs.core.work-order";
pub const M03_VERSION: u16 = 1;
pub const M03_COMPILER_ALGORITHM: &str = "m03-woc-v1";
```

Schema/version mismatch is typed and fail-closed.

### Frozen identity newtypes

Public semantic identities are explicit newtypes rather than raw interchangeable strings:

```rust
pub struct WorkOrderId(pub String);
pub struct WorkOrderFingerprint(pub String);
pub struct WorkOrderCompilationId(pub String);
pub struct WorkPacketId(pub String);
pub struct CriterionId(pub String);
pub struct EvidenceRequirementId(pub String);
pub struct CanonicalSourceId(pub String);
pub struct ContextLockRequirementId(pub String);
pub struct GovernanceRequirementId(pub String);
pub struct LineageGeneration(pub u64);
pub struct WorkOrderRevision(pub u64);
```

Rules:
- `WorkOrderRevision(0)` is invalid;
- IDs are bounded UTF-8 strings with contract-specific maximum byte lengths charged against M03 resource budget;
- semantic IDs are case-sensitive and byte-stable;
- M03 performs no Unicode case folding or normalization unless a future field-specific policy explicitly admits it;
- fingerprint newtypes contain lowercase SHA-256 hex produced only through `core-identity`;
- duplicate logical IDs within a semantic namespace fail before freeze.

### Envelope

Durable/external v1 contracts use:

```rust
pub struct M03Envelope<T> {
    pub schema: String,
    pub version: u16,
    pub kind: M03ContractKind,
    pub payload: T,
}

pub enum M03ContractKind {
    WorkOrderRequest,
    FrozenWorkOrder,
    CompilationReceipt,
    RevisionDiff,
    CorrectionProposal,
    AdmissionReceipt,
    AdmittedWorkOrder,
    LineageSnapshot,
    LineagePrecondition,
}
```

The envelope validates schema, version and kind before semantic processing. Unknown version/kind never downgrades into v1.

### Authoring request

`WorkOrderRequestV1` freezes the semantic authoring inputs:

```rust
pub struct WorkOrderRequestV1 {
    pub logical_key: WorkOrderLogicalKeyV1,
    pub explicit_work_order_id: Option<WorkOrderId>,
    pub objective: BoundedTextV1,
    pub source_manifest: CanonicalSourceManifestV1,
    pub scope: ScopeEnvelopeV1,
    pub packets: Vec<WorkPacketDeclarationV1>,
    pub acceptance: AcceptanceEvidenceGraphV1,
    pub context_budget: ContextBudgetEnvelopeV1,
    pub correction_policy: CorrectionPolicyV1,
    pub workspace_requirement: WorkspaceRequirementV1,
    pub context_lock_requirement: ContextLockRequirementV1,
    pub governance_requirement: GovernanceRequirementV1,
    pub risk_assurance_profile: RiskAssuranceProfileV1,
    pub stop_condition: StopConditionV1,
    pub resource_budget: M03ResourceBudget,
}
```

The objective may be human-readable but remains bounded semantic input. It grants no authority outside the machine-readable fields.

### Compilation context

Compilation consumes only explicit resolved state:

```rust
pub struct CompilationContextV1 {
    pub compiler_generation: CompilerGenerationV1,
    pub policy_generation: u64,
    pub lineage: LineageSnapshotV1,
    pub resolved_sources: ResolvedSourceSetV1,
}

pub struct CompilerGenerationV1 {
    pub algorithm: String,
    pub schema_version: u16,
    pub policy_schema_version: u16,
}
```

No ambient repository, current branch, HIVE state, GitHub state, filesystem state or clock lookup is legal inside compile.

### Source boundary contracts

`CanonicalSourceManifestV1` is semantic declaration. Resolution is separate evidence.

```rust
pub struct CanonicalSourceManifestV1 {
    pub entries: Vec<CanonicalSourceRefV1>,
}

pub struct ResolvedSourceSetV1 {
    pub manifest_fingerprint: String,
    pub entries: Vec<ResolvedSourceEvidenceV1>,
    pub resolver_generation: String,
}

pub struct ResolvedSourceEvidenceV1 {
    pub source_id: CanonicalSourceId,
    pub semantic_fingerprint: String,
    pub provenance_fingerprint: String,
    pub freshness: SourceFreshnessV1,
    pub secret_classification: SecretClassificationV1,
}
```

Round 4 deliberately freezes a **data boundary**, not an I/O trait.

There is no `SourceResolver` trait called by `WorkOrderCompiler` in V0.0. External orchestration resolves Git/filesystem/HIVE/GitHub/document state and passes `ResolvedSourceSetV1`. This prevents an innocent-looking trait implementation from smuggling hidden network/process/filesystem authority into the compiler.

### Workspace admission boundary

Frozen semantics persist only requirements:

```rust
pub struct WorkspaceRequirementV1 {
    pub expected_project_binding_id: Option<core_workspace::ProjectBindingId>,
    pub expected_workspace_id: Option<core_workspace::WorkspaceId>,
    pub required_profile: core_workspace::BvmProfile,
    pub required_basis_components: core_workspace::ComponentMask,
    pub allow_compatible_refresh: bool,
}
```

Admission receives ephemeral current M02 evidence separately:

```rust
pub struct WorkspaceAdmissionEvidenceV1 {
    pub handle: core_workspace::WorkspaceHandleV1,
    pub current_basis_fingerprint: String,
    pub compatibility: WorkspaceCompatibilityV1,
}

pub enum WorkspaceCompatibilityV1 {
    ExactMatch,
    CompatibleRefresh,
    Incompatible,
    Unknown,
}
```

`WorkspaceHandleV1` never enters the frozen semantic fingerprint.

### Context Lock boundary

```rust
pub struct ContextLockRequirementV1 {
    pub requirement_id: ContextLockRequirementId,
    pub required_schema: u16,
    pub required_source_manifest_fingerprint: String,
    pub required_base_fingerprint: Option<String>,
    pub implementation_authorization_required: bool,
}

pub struct ContextLockAdmissionProofV1 {
    pub requirement_id: ContextLockRequirementId,
    pub concrete_lock_fingerprint: String,
    pub bound_work_order_id: WorkOrderId,
    pub bound_work_order_revision: WorkOrderRevision,
    pub bound_work_order_fingerprint: WorkOrderFingerprint,
    pub source_manifest_fingerprint: String,
    pub base_fingerprint: Option<String>,
    pub implementation_authorized: bool,
}
```

The concrete lock fingerprint remains admission evidence and never participates in the frozen fingerprint cycle.

### Governance proof boundary

```rust
pub struct GovernanceRequirementV1 {
    pub requirement_id: GovernanceRequirementId,
    pub required_policy_generation: String,
    pub required_verdict: GovernanceVerdictV1,
}

pub struct GovernanceAdmissionProofV1 {
    pub requirement_id: GovernanceRequirementId,
    pub verdict: GovernanceVerdictV1,
    pub policy_generation: String,
    pub bound_work_order_fingerprint: WorkOrderFingerprint,
    pub bound_base_fingerprint: Option<String>,
    pub proof_fingerprint: String,
}

pub enum GovernanceVerdictV1 {
    Approved,
    CorrectionRequired,
    Blocked,
}
```

Only `Approved` can satisfy an execution-capable governance requirement. M03 validates proof shape/binding; it does not mint the proof.

### Lineage contracts

```rust
pub struct WorkOrderRevisionRefV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub fingerprint: WorkOrderFingerprint,
}

pub struct LineageSnapshotV1 {
    pub work_order_id: WorkOrderId,
    pub current_revision: Option<WorkOrderRevisionRefV1>,
    pub store_generation: LineageGeneration,
}

pub struct LineagePreconditionCapsuleV1 {
    pub work_order_id: WorkOrderId,
    pub expected_parent: Option<WorkOrderRevisionRefV1>,
    pub expected_store_generation: LineageGeneration,
    pub proposed_revision: WorkOrderRevision,
    pub precondition_fingerprint: String,
}
```

A new logical Work Order requires `current_revision=None` and proposes revision 1. Existing lineage proposes exactly N+1.

### Compilation output

```rust
pub struct CompileSuccessV1 {
    pub frozen: FrozenWorkOrderV1,
    pub lineage_precondition: LineagePreconditionCapsuleV1,
    pub receipt: CompilationReceiptV1,
}

pub struct FrozenWorkOrderV1 {
    pub schema_version: u16,
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub fingerprint: WorkOrderFingerprint,
    pub semantic: FrozenWorkOrderSemanticV1,
}

pub struct FrozenWorkOrderSemanticV1 {
    pub objective: BoundedTextV1,
    pub source_manifest: CanonicalSourceManifestV1,
    pub scope: ScopeEnvelopeV1,
    pub packets: Vec<WorkPacketDeclarationV1>,
    pub acceptance: AcceptanceEvidenceGraphV1,
    pub context_budget: ContextBudgetEnvelopeV1,
    pub correction_policy: CorrectionPolicyV1,
    pub workspace_requirement: WorkspaceRequirementV1,
    pub context_lock_requirement: ContextLockRequirementV1,
    pub governance_requirement: GovernanceRequirementV1,
    pub risk_assurance_profile: RiskAssuranceProfileV1,
    pub stop_condition: StopConditionV1,
    pub packet_context_mesh: PacketContextMeshV1,
}
```

The fingerprint is over the explicit semantic projection only. The stored `fingerprint` field itself is not recursively included in its own projection.

### Deterministic Compilation Receipt

```rust
pub struct CompilationReceiptV1 {
    pub compilation_id: WorkOrderCompilationId,
    pub request_fingerprint: String,
    pub compilation_context_fingerprint: String,
    pub compiler_generation: CompilerGenerationV1,
    pub lineage_precondition_fingerprint: String,
    pub output_fingerprint: WorkOrderFingerprint,
    pub resource_usage: M03ResourceUsageV1,
    pub safe_diagnostics: Vec<SafeDiagnosticV1>,
}
```

Receipt wall-clock timestamp is intentionally absent from the semantic receipt. Host-side logs may timestamp the operation externally.

### Packet declaration and DAG

```rust
pub struct WorkPacketDeclarationV1 {
    pub packet_id: WorkPacketId,
    pub objective: BoundedTextV1,
    pub depends_on: Vec<WorkPacketId>,
    pub scope: PacketScopeV1,
    pub required_source_ids: Vec<CanonicalSourceId>,
    pub acceptance_criterion_ids: Vec<CriterionId>,
    pub required_workspace_profile: Option<core_workspace::BvmProfile>,
}
```

Canonical order:
1. reject duplicate packet IDs;
2. validate all dependency refs;
3. validate child scope is a subset/intersection of parent scope;
4. topologically sort with deterministic packet-ID tie breaking;
5. persist packets in that canonical topological order.

The compiler never interprets the DAG as permission to run concurrently.

### Acceptance Evidence Graph

```rust
pub struct AcceptanceCriterionV1 {
    pub criterion_id: CriterionId,
    pub description: BoundedTextV1,
    pub blocking: bool,
    pub evidence_requirement_ids: Vec<EvidenceRequirementId>,
}

pub struct EvidenceRequirementV1 {
    pub evidence_requirement_id: EvidenceRequirementId,
    pub evidence_class: EvidenceClassV1,
    pub required: bool,
}

pub struct AcceptanceEvidenceGraphV1 {
    pub criteria: Vec<AcceptanceCriterionV1>,
    pub requirements: Vec<EvidenceRequirementV1>,
}
```

Every blocking criterion must have at least one required evidence edge or an explicit typed N/A rule defined by policy.

### Packet Context Mesh

```rust
pub struct PacketContextMeshV1 {
    pub shared_sources: Vec<CanonicalSourceId>,
    pub packet_edges: Vec<PacketContextEdgeV1>,
}

pub struct PacketContextEdgeV1 {
    pub packet_id: WorkPacketId,
    pub stable_prefix_source_ids: Vec<CanonicalSourceId>,
    pub packet_required_source_ids: Vec<CanonicalSourceId>,
    pub validate_only_source_ids: Vec<CanonicalSourceId>,
    pub optional_expandable_source_ids: Vec<CanonicalSourceId>,
}
```

All lists are sorted and duplicate-free. Reconstructing one packet must yield exactly the independently required source set for that packet.

### Scope and dependency policy

`ScopeEnvelopeV1` remains deny-first and freezes:
- allowed modules/crates/packages;
- allowed paths/path prefixes;
- forbidden paths/path prefixes;
- allowed artifact classes;
- source mutation policy;
- generated/vendor policy;
- dependency policy;
- security-policy mutation policy.

Dependency policy is explicit:

```rust
pub enum DependencyPolicyV1 {
    NoDependencyChanges,
    ExistingOnly,
    GovernedAdmissionRequired,
    ExplicitAllowList(Vec<String>),
}
```

A path/crate allow never implies dependency authority.

### Correction and semantic delta contracts

```rust
pub struct ExecutionCorrectionProposalV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub proposed_classes: Vec<CorrectionClassV1>,
    pub affected_paths: Vec<String>,
    pub dependency_changes: Vec<DependencyChangeV1>,
    pub semantic_claim: NoSemanticChangeClaimV1,
}

pub struct WorkOrderRevisionDiffV1 {
    pub from: WorkOrderRevisionRefV1,
    pub to: WorkOrderRevisionRefV1,
    pub classes: Vec<SemanticDeltaClassV1>,
    pub changed_component_ids: Vec<String>,
    pub semantic_change: bool,
}
```

Classification is deterministic from canonical projections, not from prose labels supplied by the caller.

### Admission input and receipt

```rust
pub struct AdmissionInputsV1 {
    pub workspace: WorkspaceAdmissionEvidenceV1,
    pub context_lock: ContextLockAdmissionProofV1,
    pub governance: GovernanceAdmissionProofV1,
    pub source_verification: SourceVerificationEvidenceV1,
    pub current_policy_generation: String,
}

pub enum AdmissionDecisionV1 {
    Ready,
    Stale,
    Blocked,
}

pub struct WorkOrderAdmissionReceiptV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub decision: AdmissionDecisionV1,
    pub workspace_generation: core_workspace::WorkspaceGeneration,
    pub workspace_basis_fingerprint: String,
    pub context_lock_fingerprint: String,
    pub governance_proof_fingerprint: String,
    pub source_verification_fingerprint: String,
    pub policy_generation: String,
    pub receipt_fingerprint: String,
    pub reasons: Vec<AdmissionReasonV1>,
}
```

A receipt is immutable historical proof. Later staleness creates another evaluation/receipt.

### M04 handoff contract

```rust
pub struct AdmittedWorkOrderV1 {
    pub frozen: FrozenWorkOrderV1,
    pub admission: WorkOrderAdmissionReceiptV1,
}
```

Materialization requires:
- receipt decision `Ready`;
- exact ID/revision/fingerprint match;
- all required receipt bindings present;
- no semantic mutation between frozen object and receipt.

M04 remains responsible for Run-start freshness revalidation.

### Public service API freeze

V0.0 exports a stateless service facade:

```rust
#[derive(Debug, Default, Clone, Copy)]
pub struct WorkOrderService;

impl WorkOrderService {
    pub fn derive_work_order_id(
        logical_key: &WorkOrderLogicalKeyV1,
    ) -> Result<WorkOrderId, WorkOrderError>;

    pub fn compile(
        request: &WorkOrderRequestV1,
        context: &CompilationContextV1,
    ) -> Result<CompileSuccessV1, WorkOrderError>;

    pub fn validate_frozen(
        frozen: &FrozenWorkOrderV1,
        budget: &M03ResourceBudget,
    ) -> Result<ValidationReportV1, WorkOrderError>;

    pub fn diff(
        from: &FrozenWorkOrderV1,
        to: &FrozenWorkOrderV1,
        budget: &M03ResourceBudget,
    ) -> Result<WorkOrderRevisionDiffV1, WorkOrderError>;

    pub fn classify_correction(
        frozen: &FrozenWorkOrderV1,
        proposal: &ExecutionCorrectionProposalV1,
        budget: &M03ResourceBudget,
    ) -> Result<CorrectionClassificationV1, WorkOrderError>;

    pub fn evaluate_admission(
        frozen: &FrozenWorkOrderV1,
        inputs: &AdmissionInputsV1,
        budget: &M03ResourceBudget,
    ) -> Result<WorkOrderAdmissionReceiptV1, WorkOrderError>;

    pub fn materialize_handoff(
        frozen: &FrozenWorkOrderV1,
        receipt: &WorkOrderAdmissionReceiptV1,
    ) -> Result<AdmittedWorkOrderV1, WorkOrderError>;
}
```

These methods perform no external I/O and accept no resolver/store/network/process handles.

### Error contract freeze

```rust
pub struct WorkOrderError {
    pub category: ErrorCategoryV1,
    pub reason: ErrorReasonV1,
    pub retryability: RetryabilityV1,
    pub diagnostics: Vec<SafeDiagnosticV1>,
}

pub enum ErrorCategoryV1 {
    InvalidInput,
    StaleOrConflict,
    PolicyBlock,
    ResourceBlock,
    InternalInvariant,
}

pub enum RetryabilityV1 {
    Never,
    RetrySameInput,
    RetryAfterSourceRefresh,
    RetryAfterWorkspaceRefresh,
    RetryAfterLineageRefresh,
    RetryAfterGovernance,
    RetryAfterResourceChange,
}
```

Reason codes are a closed v1 enum containing the Round 3 taxonomy. Unknown internal states map to `InternalInvariant` and never to READY.

### M03ResourceBudget exact dimensions

The public budget type freezes these finite fields:

```rust
pub struct M03ResourceBudget {
    pub max_request_bytes: u64,
    pub max_frozen_bytes: u64,
    pub max_source_manifest_entries: u32,
    pub max_packet_count: u32,
    pub max_packet_dependency_edges: u32,
    pub max_acceptance_criteria: u32,
    pub max_evidence_requirements: u32,
    pub max_context_edges: u32,
    pub max_delta_items: u32,
    pub max_diagnostics: u32,
    pub max_total_string_bytes: u64,
    pub max_operation_millis: u64,
}
```

Zero/unlimited sentinels are invalid for production profiles.

`M03ResourceBudget` is owned by `core-work-order`, not `core-config`, because it is part of M03's public semantic/safety contract and must be explicit at call boundaries.

### Resource usage accounting

```rust
pub struct M03ResourceUsageV1 {
    pub request_bytes: u64,
    pub frozen_bytes: u64,
    pub source_entries: u32,
    pub packets: u32,
    pub dependency_edges: u32,
    pub criteria: u32,
    pub evidence_requirements: u32,
    pub context_edges: u32,
    pub delta_items: u32,
    pub diagnostic_count: u32,
    pub total_string_bytes: u64,
}
```

Accounting must happen incrementally enough that attacker-controlled input cannot force unbounded allocation before a budget check.

### Exact initial crate/file map

Round 4 freezes the initial implementation topology:

```text
crates/core-work-order/
  Cargo.toml
  src/
    lib.rs
    contracts.rs
    ids.rs
    errors.rs
    canonical.rs
    compiler.rs
    validation.rs
    scope.rs
    packets.rs
    acceptance.rs
    context.rs
    lineage.rs
    delta.rs
    admission.rs
    budget.rs
    external.rs
    service.rs
  tests/
    compile.rs
    canonical.rs
    identity.rs
    scope.rs
    packets.rs
    acceptance.rs
    context.rs
    lineage.rs
    delta.rs
    admission.rs
    resources.rs
    adversarial.rs
    fixtures/
      README.md
  benches/
    m03_work_order.rs
```

File ownership:
- `contracts.rs`: durable/ephemeral DTOs and enums;
- `ids.rs`: identity newtypes and validation;
- `errors.rs`: error/retryability/safe diagnostics;
- `canonical.rs`: M03 semantic projections only; hashing remains delegated to core-identity;
- `compiler.rs`: request -> frozen + LPC + DCR;
- `validation.rs`: frozen self-consistency validation;
- `scope.rs`: deny-first scope/dependency rules;
- `packets.rs`: DAG validation/topological normalization;
- `acceptance.rs`: AEG validation;
- `context.rs`: context budget/PCM;
- `lineage.rs`: snapshot/LPC validation only;
- `delta.rs`: semantic diff/correction classification;
- `admission.rs`: explicit admission evaluation;
- `budget.rs`: budget validation/accounting;
- `external.rs`: external evidence/request-response data boundaries only, no I/O adapters;
- `service.rs`: stateless public facade.

No second M03 crate is admitted initially.

### Repository files expected to change during implementation

Initial M03 Work Order may change only:
- root `Cargo.toml` to add `crates/core-work-order`;
- `Cargo.lock`;
- the exact new `crates/core-work-order/**` map above;
- `fuzz/Cargo.toml`;
- the frozen M03 fuzz target files below;
- `.github/workflows/governance.yml` to add exact M03 validation jobs/gates;
- M03 execution/calibration evidence documents;
- canonical checkpoint/DoD/backlog files only through governed evidence/promotion deltas.

`core-workspace`, `core-runtime`, `core-cli`, `core-identity`, `core-contracts` and `core-config` are not expected to require product-code edits for initial M03 implementation. Any need to alter them is a Correction Delta and must prove necessity.

### Final initial dependency admission

`core-work-order` direct internal dependencies:
- `core-contracts`;
- `core-identity`;
- `core-workspace`.

Direct workspace third-party dependencies:
- `serde`;
- `serde_json`;
- `thiserror`.

Not admitted as direct M03 dependencies:
- `sha2`: hashing remains through core-identity;
- `core-config`: M03ResourceBudget is M03-owned and explicit;
- `tokio` or another async runtime;
- `regex`;
- `petgraph` or another graph framework;
- Git libraries;
- filesystem walkers/glob frameworks;
- database/cache engines;
- watcher frameworks;
- HIVE SDK/runtime;
- LLM SDKs;
- HTTP/network clients.

No new third-party crate is required by Round 4.

### Dependency graph freeze

```text
core-contracts
      ^
      |
core-identity        core-workspace
      ^                   ^
       \                 /
        \               /
         core-work-order
              |
              v
           future M04+
```

`core-workspace` remains independent of `core-work-order`.
M03 cannot depend on M04+.
No runtime/CLI dependency is allowed into M03 core.

### Fuzz target freeze

The initial implementation must add exactly these M03 fuzz targets unless a governed Correction Delta changes the set:

```text
fuzz/fuzz_targets/m03_compile_request.rs
fuzz/fuzz_targets/m03_canonical_projection.rs
fuzz/fuzz_targets/m03_packet_dag.rs
fuzz/fuzz_targets/m03_revision_delta.rs
fuzz/fuzz_targets/m03_lineage_precondition.rs
fuzz/fuzz_targets/m03_admission.rs
fuzz/fuzz_targets/m03_context_mesh.rs
```

Target obligations:
- never panic/abort on arbitrary bounded bytes;
- no unbounded allocation before resource rejection;
- successful parse/compile output passes self-validation;
- invalid graph/reference/lineage/admission states never become FROZEN/READY.

### Property-law freeze

Implementation property tests must prove at minimum:

1. **Compile replay law:** same semantic request/context -> identical frozen bytes/fingerprint, LPC and semantic DCR fields.
2. **Permutation law:** permutation of unordered input collections -> identical semantic result.
3. **Diagnostic exclusion law:** safe non-semantic diagnostic changes -> unchanged WorkOrderFingerprint.
4. **Round-trip law:** serialize/deserialize supported v1 contract -> semantic equality and stable fingerprint.
5. **Packet DAG law:** canonical topological order is deterministic; cycles/dangling refs reject.
6. **Packet scope law:** packet scope never widens parent Work Order scope.
7. **Deny precedence law:** explicit deny always defeats allow.
8. **AEG completeness law:** blocking criterion cannot be orphaned from required evidence/N-A policy.
9. **PCM reconstruction law:** mesh reconstruction equals independently compiled mandatory source set.
10. **Context completeness law:** budget pressure never silently removes mandatory source refs.
11. **Lineage successor law:** proposed revision is exactly 1 for empty lineage or N+1 for existing lineage.
12. **LPC conflict law:** stale parent/store generation cannot be accepted by canonical persistence contract.
13. **Correction firewall law:** dependency/scope/architecture/security/acceptance/stop semantic changes cannot classify as same-revision correction.
14. **Admission determinism law:** identical frozen + resolved admission evidence -> identical semantic receipt/decision.
15. **Unknown fail-closed law:** UNKNOWN workspace/source/lock/governance state never becomes READY.
16. **Receipt replay law:** changed correctness-relevant generation invalidates reuse.
17. **Resource atomicity law:** resource/deadline failure yields no partial frozen/admitted success.
18. **Cache independence law:** if disposable memoization is implemented, hit/miss result equivalence is exact.
19. **Secret-canary law:** raw secret canaries never appear in durable contracts/safe diagnostics.
20. **No-hidden-I/O law:** static dependency/API audit plus tests show compile/validate/diff/classify/admit/handoff do not invoke process/network/Git/HIVE/filesystem discovery.

### Benchmark families

Round 4 freezes benchmark families, not production numeric budgets.

`benches/m03_work_order.rs` must measure:
- request validation;
- canonical semantic projection;
- compile;
- frozen validation;
- packet DAG normalization;
- AEG validation;
- PCM construction/reconstruction;
- revision diff;
- correction classification;
- admission evaluation;
- serialization/deserialization;
- optional memo hit/miss if memoization is implemented.

Scaling axes:
- canonical source entries;
- packet count;
- packet dependency edges;
- criteria/evidence edges;
- context mesh edges;
- delta item count;
- total bounded semantic string bytes.

Benchmark fixtures are synthetic/local and deterministic.

### Benchmark threshold policy

Hard correctness thresholds:
- zero semantic mismatch across repeated/permuted deterministic cases;
- zero READY/FROZEN partial success after typed resource failure;
- zero mandatory-source loss under PCM/context-budget reconstruction;
- zero secret-canary leakage;
- zero dependency-cycle or stale-lineage acceptance.

Performance thresholds:
- exact absolute latency/memory/cardinality defaults are produced by the M03 Resource Calibration Gate during implementation;
- regression checks use the repository's accepted compatible PRB policy rather than a second arbitrary M03 percentage;
- a candidate that satisfies latency by weakening correctness/security is rejected;
- scaling evidence must show no unexplained superlinear cliff relative to the expected algorithmic work: canonical sorting O(N log N), DAG traversal O(V+E), direct indexed lookups O(log N) or amortized equivalent;
- unsupported fixture scale is recorded as bounded/skipped, never extrapolated to PASS.

### M03 Resource Calibration Gate protocol

The future implementation Work Order must contain:

```text
Phase A - implement frozen contracts under conservative test-only ceilings
    |
    v
CALIBRATION_ONLY
    |
    +--> deterministic fixture matrix
    +--> benchmark families
    +--> resource/accounting evidence
    +--> semantic/property assertions for every candidate profile
    |
    v
M03 Calibration Evidence Bundle
    |
    +--> proposed finite M03ResourceBudget defaults
    +--> proposed absolute acceptance thresholds
    +--> selected/rejected candidate rationale
    |
    v
bounded Calibration Delta
    |
    v
full exact-head validation rerun
    |
    v
eligible for final M03 implementation review
```

The bounded Calibration Delta may change only:
- numeric M03ResourceBudget defaults;
- benchmark-derived absolute thresholds;
- fixture metadata/evidence references;
- explanatory documentation.

It cannot change contracts, file map, dependency graph, semantic ownership, error taxonomy, security invariants, Work Order scope or DoD.

### Round 4 production DoD direction

Before M03 can ever be declared complete, exact-head evidence must additionally prove:
- the exact public service signatures above;
- the exact initial file/dependency map;
- v1 envelope/schema rejection;
- canonical identity/serialization properties;
- all 20 property laws;
- all seven fuzz targets;
- Windows + Ubuntu exact-head CI;
- supply-chain/advisory/SBOM gates;
- M03 Resource Calibration Gate with finite selected defaults;
- benchmark/regression evidence under compatible repository PRB policy;
- no-hidden-I/O proof;
- zero-LLM compile/validate/diff/classify/admit/handoff proof;
- external source/workspace/Context Lock/governance/lineage inputs cannot escalate authority;
- executor Evidence Bundle and proposed Checkpoint Delta;
- independent governed review with no unresolved HIGH/CRITICAL finding.

### Round 4 unresolved items

After Round 4, the only planning items allowed to remain before implementation authorization are:
- exact numeric M03ResourceBudget defaults, intentionally evidence-derived during implementation;
- the final M03 production DoD wording/criterion numbering;
- final implementation Work Order;
- final Context Lock;
- final planning audit/freeze and execution-base binding.

No public API/file-map/dependency/fuzz/property-family redesign is expected after Round 4 without a governed Correction Delta.

### Round 4 STOP CONDITION

Round 4 is complete when:
- exact initial public Rust service/data boundary is frozen;
- exact crate/file map is frozen;
- final initial dependency set is frozen;
- external resolution boundaries are explicit data contracts with no hidden I/O trait;
- M02 workspace types are reused without persisting live handles in frozen semantics;
- fuzz targets and property laws are frozen;
- benchmark/calibration threshold policy is frozen;
- production DoD direction is explicit;
- implementation remains unauthorized.

