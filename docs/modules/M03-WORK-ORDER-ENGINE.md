# M03 - Work Order Engine

Status: `ROUND_4_REVIEW_CANDIDATE / IMPLEMENTATION_UNAUTHORIZED`

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


## Round 4 - public contracts, adapters and validation freeze

Status: PLANNING REVIEW CANDIDATE. Rounds 1-3 remain accepted. This section does not authorize M03 implementation.

### Round 4 preservation rules

Round 4 preserves the compiler/admission boundary, immutable revisions, zero-LLM semantics, M02 ownership, no hidden I/O, external lineage/LPC, HIVE's advisory role, the bounded packet DAG, complete AEG, non-evergreen admission and CORE-D-132 reviewer-first correction policy. It freezes the public and validation surfaces only. There is no source code, dependency, lockfile, runtime, registry or persistence change in this planning round.

The exact names and field ownership below are normative for the later M03 V0.0 implementation Work Order. The types are contracts, not a claim that those Rust files or functions already exist.

### Public Rust schema and identity

Every root payload that crosses a durable or external M03 boundary is wrapped with the exact schema identifier nexlabs.core.work-order, version 1, and a closed kind enum. Nested DTOs inherit the root M03 schema/version or carry their explicitly named producer schema/version where stated. Bare service return structs are in-memory values; before persistence or transport they MUST be wrapped with the corresponding root kind below. Unknown schema, version, kind, enum value or required field is a typed error; no downgrade or best-effort reinterpretation is allowed.

~~~rust
pub const M03_SCHEMA: &str = "nexlabs.core.work-order";
pub const M03_VERSION: u16 = 1;

pub struct WorkOrderEnvelope<T> {
    pub schema: String,
    pub version: u16,
    pub kind: WorkOrderContractKindV1,
    pub payload: T,
}

pub struct WorkOrderId(String);
pub struct WorkOrderRevision(u32);
pub struct WorkOrderFingerprint(String); // lowercase SHA-256 hex from core-identity
pub struct WorkOrderCompilationId(String); // lowercase SHA-256 hex from core-identity
pub struct EvidenceFingerprintV1(String); // lowercase SHA-256 hex from core-identity
pub struct SourceRefId(String);
pub struct WorkPacketId(String);
pub struct AcceptanceCriterionId(String);
pub struct EvidenceRequirementId(String);
pub struct ContextRefId(String);
pub struct GovernanceProofId(String);
pub struct LineageEdgeId(String);
pub struct SemanticFieldIdV1(String); // validated closed canonical semantic-field identifier

pub struct WorkOrderLogicalKeyV1 {
    pub project_namespace: String,
    pub module_namespace: String,
    pub stable_key: String,
}

pub struct WorkOrderRequestV1 {
    pub requested_work_order_id: Option<WorkOrderId>,
    pub logical_key: Option<WorkOrderLogicalKeyV1>,
    pub objective: String,
    pub sources: Vec<CanonicalSourceRefV1>,
    pub workspace: WorkspaceRequirementV1,
    pub context_lock: ContextLockRequirementV1,
    pub governance: GovernanceRequirementV1,
    pub scope: ScopeEnvelopeV1,
    pub packets: Vec<WorkPacketSpecV1>,
    pub acceptance: AcceptanceEvidenceGraphV1,
    pub context_budget: ContextBudgetEnvelopeV1,
    pub correction_policy: CorrectionPolicyV1,
    pub stop_condition: StopConditionV1,
    pub risk_assurance: RiskAssuranceProfileV1,
    pub parent: Option<WorkOrderRevisionRefV1>,
}

pub struct FrozenWorkOrderV1 {
    work_order_id: WorkOrderId,
    revision: WorkOrderRevision,
    fingerprint: WorkOrderFingerprint,
    compilation_id: WorkOrderCompilationId,
    semantic: WorkOrderSemanticV1,
    source_manifest: Vec<CanonicalSourceRefV1>,
    workspace_requirement: WorkspaceRequirementV1,
    context_lock_requirement: ContextLockRequirementV1,
    governance_requirement: GovernanceRequirementV1,
    lineage: WorkOrderLineageRefV1,
}

pub struct WorkOrderCompilationV1 {
    frozen: FrozenWorkOrderV1,
    receipt: DeterministicCompilationReceiptV1,
    lineage_precondition: Option<LineagePreconditionCapsuleV1>,
    diagnostics: Vec<DiagnosticV1>,
}
~~~

The supporting envelope and resolved-evidence DTO shapes are also frozen:

~~~rust
pub enum WorkOrderContractKindV1 {
    Request,
    Frozen,
    Compilation,
    ValidationReceipt,
    RevisionDiff,
    CorrectionClassification,
    AdmissionRequest,
    AdmissionReceipt,
    AdmittedHandoff,
}

pub struct CompilationContextV1 {
    pub compiler_contract_version: u16,
    pub algorithm_version: String,
    pub policy_generation: u64,
    pub security_generation: u64,
    pub config_generation: u64,
    pub sources: SourceResolutionBatchV1,
    pub lineage: LineageSnapshotV1,
    pub hive_context_refs: Vec<HiveContextRefV1>,
}

pub struct SourceResolutionBatchV1 {
    pub resolver_schema: String,
    pub resolver_version: u16,
    pub entries: Vec<SourceResolutionEvidenceV1>,
    pub batch_fingerprint: EvidenceFingerprintV1,
}

pub struct WorkspaceAdmissionEvidenceV1 {
    pub m02_schema: String,
    pub m02_version: u16,
    pub project_binding_id: Option<String>,
    pub workspace_id: String,
    pub runtime_epoch: u64,
    pub generation: u64,
    pub basis_fingerprint: EvidenceFingerprintV1,
    pub required_profile: WorkspaceFreshnessProfileV1,
    pub satisfied_components: Vec<String>,
    pub compatibility: BasisCompatibilityV1,
    pub freshness: EvidenceFreshnessV1,
    pub provenance_fingerprint: EvidenceFingerprintV1,
    pub proof_fingerprint: EvidenceFingerprintV1,
}

pub struct ContextLockEvidenceV1 {
    pub schema: String,
    pub version: u16,
    pub lock_fingerprint: EvidenceFingerprintV1,
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub authorized_base_fingerprint: EvidenceFingerprintV1,
    pub source_set_fingerprint: EvidenceFingerprintV1,
    pub implementation_authorized: bool,
    pub freshness: EvidenceFreshnessV1,
    pub verifier_provenance: EvidenceFingerprintV1,
    pub proof_fingerprint: EvidenceFingerprintV1,
}

pub struct VerifiedGovernanceProofV1 {
    pub schema: String,
    pub version: u16,
    pub proof_id: GovernanceProofId,
    pub project_repository_fingerprint: EvidenceFingerprintV1,
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub exact_base: String,
    pub exact_head: Option<String>,
    pub verdict: ExternalGovernanceVerdictV1,
    pub authorized_scope_fingerprint: EvidenceFingerprintV1,
    pub policy_generation: u64,
    pub freshness: EvidenceFreshnessV1,
    pub verifier_provenance: EvidenceFingerprintV1,
    pub proof_fingerprint: EvidenceFingerprintV1,
}

pub struct WorkOrderAdmissionRequestV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub requested_mode: AdmissionModeV1,
    pub sources: SourceResolutionBatchV1,
    pub workspace: WorkspaceAdmissionEvidenceV1,
    pub context_lock: Option<ContextLockEvidenceV1>,
    pub governance: Option<VerifiedGovernanceProofV1>,
    pub policy_generation: u64,
    pub security_generation: u64,
    pub config_generation: u64,
}

pub enum WorkOrderAdmissionStatusV1 {
    Ready,
    Rejected,
    Stale,
    Blocked,
    Superseded,
}

pub struct HiveContextRefV1 {
    pub context_id: ContextRefId,
    pub hive_project_id: String,
    pub content_fingerprint: EvidenceFingerprintV1,
    pub snapshot_id: String,
    pub freshness: EvidenceFreshnessV1,
    pub provenance_fingerprint: EvidenceFingerprintV1,
    pub advisory_only: bool, // must be true
}

pub struct M03ResourceBudgetV1 {
    pub max_request_bytes: u64,
    pub max_frozen_bytes: u64,
    pub max_string_bytes: u64,
    pub max_source_refs: u64,
    pub max_packets: u64,
    pub max_packet_edges: u64,
    pub max_scope_rules: u64,
    pub max_criteria: u64,
    pub max_evidence_requirements: u64,
    pub max_acceptance_evidence_edges: u64,
    pub max_lineage_edges: u64,
    pub max_context_refs: u64,
    pub max_correction_rules: u64,
    pub max_diff_entries: u64,
    pub max_diagnostic_entries: u64,
    pub max_parse_depth: u32,
    pub calibration_state: ResourceCalibrationStateV1,
}

pub struct WorkOrderErrorV1 {
    pub category: WorkOrderErrorCategoryV1,
    pub code: WorkOrderErrorCodeV1,
    pub retryability: RetryabilityV1,
    pub subjects: Vec<SafeSubjectRefV1>,
    pub diagnostics: Vec<DiagnosticV1>,
}

pub struct WorkOrderAdmissionReceiptV1 {
    work_order_id: WorkOrderId,
    revision: WorkOrderRevision,
    work_order_fingerprint: WorkOrderFingerprint,
    compilation_id: WorkOrderCompilationId,
    status: WorkOrderAdmissionStatusV1,
    source_batch_fingerprint: EvidenceFingerprintV1,
    workspace_id: String,
    workspace_generation: u64,
    workspace_basis_fingerprint: EvidenceFingerprintV1,
    context_lock_fingerprint: Option<EvidenceFingerprintV1>,
    governance_proof_fingerprint: Option<EvidenceFingerprintV1>,
    policy_generation: u64,
    security_generation: u64,
    config_generation: u64,
    reason_codes: Vec<WorkOrderErrorCodeV1>,
    receipt_fingerprint: EvidenceFingerprintV1,
}

pub struct WorkOrderValidationReceiptV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub semantic_projection_fingerprint: EvidenceFingerprintV1,
    pub receipt_fingerprint: EvidenceFingerprintV1,
}

pub struct WorkOrderRevisionDiffV1 {
    pub work_order_id: WorkOrderId,
    pub before_revision: WorkOrderRevision,
    pub before_fingerprint: WorkOrderFingerprint,
    pub after_revision: WorkOrderRevision,
    pub after_fingerprint: WorkOrderFingerprint,
    pub changed_semantic_fields: Vec<SemanticFieldIdV1>,
    pub requires_new_revision: bool,
    pub forbidden_reason_codes: Vec<WorkOrderErrorCodeV1>,
    pub diff_fingerprint: EvidenceFingerprintV1,
}

pub enum CorrectionDispositionV1 {
    AllowedSameRevision,
    RequiresNewRevision,
    Forbidden,
}

pub struct CorrectionClassificationReceiptV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub proposal_fingerprint: EvidenceFingerprintV1,
    pub disposition: CorrectionDispositionV1,
    pub reason_codes: Vec<WorkOrderErrorCodeV1>,
    pub receipt_fingerprint: EvidenceFingerprintV1,
}

pub struct AdmittedWorkOrderV1 {
    work_order_id: WorkOrderId,
    revision: WorkOrderRevision,
    work_order_fingerprint: WorkOrderFingerprint,
    compilation_id: WorkOrderCompilationId,
    admission_receipt_fingerprint: EvidenceFingerprintV1,
    packet_dag: WorkPacketDagV1,
    scope: ScopeEnvelopeV1,
    acceptance_criterion_ids: Vec<AcceptanceCriterionId>,
    evidence_requirement_ids: Vec<EvidenceRequirementId>,
    context_plan: PacketContextPlanV1,
    stop_condition: StopConditionV1,
}
~~~

Additional enum domains are closed V1 types: SemanticFieldIdV1 is accepted only from the frozen canonical semantic-field registry; WorkspaceFreshnessProfileV1 mirrors only the named M02 BVM profiles; BasisCompatibilityV1 is EXACT_MATCH/COMPATIBLE_REFRESH/INCOMPATIBLE/UNKNOWN; EvidenceFreshnessV1 is CURRENT/STALE/UNKNOWN/SUBSTITUTED; ExternalGovernanceVerdictV1 is an externally verified accepted/rejected/blocked result; AdmissionModeV1 is the requested policy class; ResourceCalibrationStateV1 is UNCALIBRATED/CALIBRATED; EvidenceFingerprintV1 is a validated lowercase 64-character digest; and SafeSubjectRefV1 contains a typed subject kind plus a bounded safe ID/fingerprint. Every ID/fingerprint wrapper uses `#[serde(transparent)]`; all public contract structs/enums derive Serialize/Deserialize, closed enums serialize in snake_case, and required contract fields have no silent defaults.

M03ResourceBudgetV1 has no Default implementation and every value must be finite and positive. It contains deterministic size, depth and cardinality limits only. Wall-clock deadlines are caller-owned orchestration guards outside the pure core: core-work-order never reads a host clock or ambient timer. If a caller deadline expires, the caller MUST discard any concurrent/late result and record typed caller-owned deadline-failure evidence; a timed-out invocation cannot yield an accepted FROZEN, READY or handoff object. This preserves CORE-R-207 / CORE-D-148 without making identical value inputs depend on scheduler or machine timing.

Compatibility refinement: the previously accepted Round 3 reason families `COMPILATION_DEADLINE_EXCEEDED` and `ADMISSION_DEADLINE_EXCEEDED` classify orchestration-bound timeout outcomes; they do not authorize core-work-order to read an ambient clock. Operation-specific timeout evidence remains caller-owned and outside WorkOrderFingerprint.

All ID and fingerprint wrappers are distinct Rust types with private validated constructors and stable serialized string representations. FrozenWorkOrderV1, WorkOrderAdmissionReceiptV1 and AdmittedWorkOrderV1 have private fields and read-only accessors, with no in-place mutation API. IDs are non-empty, bounded, domain-separated values. Revision is a positive u32 and advances by exactly one for a newly canonical semantic revision. Fingerprints are validated lowercase 64-character SHA-256 hex strings. WorkOrderId is caller supplied or derived deterministically from WorkOrderLogicalKeyV1 through core-identity; M03 never uses random UUIDs, clocks, branch names, cwd, map iteration or host identity to mint IDs.

The remaining durable V1 contracts have these exact public fields and meanings:

- WorkOrderSemanticV1: objective, explicit scope, packet DAG, acceptance/evidence graph, context budget, correction policy, stop condition and risk/assurance profile.
- CanonicalSourceRefV1: source_id, source_class, authority_domain, locator_kind, explicit locator, expected semantic fingerprint, required_for_compile, required_for_admission, required_packet_ids, freshness_policy, provenance_class, secret_classification and expansion_policy.
- SourceResolutionEvidenceV1: source_id, requested_fingerprint, observed_fingerprint, authority_domain, source_revision, resolver_schema/version, freshness_state, provenance_fingerprint and evidence_fingerprint. It carries references and fingerprints only; raw source bytes and credentials are not durable payloads.
- WorkspaceRequirementV1: expected workspace/project-binding identity constraints, required M02 BVM profile, required basis components, compatibility policy, standalone/HIVE assurance requirement, dirty/untracked policy and required M02 schema/version.
- ContextLockRequirementV1: required lock schema/version, work-order binding, authorized base constraint, canonical source-set fingerprint, implementation-authorized requirement and staleness policy.
- GovernanceRequirementV1: required external verdict class, exact work-order/base/head/scope binding, policy generation and freshness requirement. It cannot encode self-approval.
- ScopeEnvelopeV1: allow and deny sets for modules, crates/packages, path prefixes and artifact classes; source/documentation/evidence/generated-artifact policies; dependency policy; correction classes; maximum scope class. Deny wins, ambiguity blocks, and each packet receives only the intersection with this envelope.
- WorkPacketSpecV1: packet_id, objective, prerequisite_packet_ids, required_source_ids, workspace freshness profile, packet scope, criterion IDs, evidence IDs, context budget and packet stop condition. WorkPacketDagV1 contains the packet records and deterministic topological_order.
- AcceptanceEvidenceGraphV1: criteria, evidence requirements and explicit criterion-to-evidence edges. A blocking criterion has one or more required evidence edges unless deterministic N/A applies; dangling and unexplained orphan nodes fail validation.
- ContextBudgetEnvelopeV1: finite per-manifest/per-packet inline and expansion dimensions, mandatory-source set, expansion policy and allowed expansion reasons. It never weakens obligations or silently truncates a mandatory source.
- CorrectionPolicyV1 and ExecutionCorrectionProposalV1: explicit same-revision change classes and the proposed path/artifact/dependency/acceptance/stop-condition delta. WorkOrderRevisionDiffV1 separately classifies changes to frozen semantic fields.
- LineageSnapshotV1: work_order_id, optional current revision/fingerprint, store_generation, bounded superseded revision/fingerprint references, bounded lineage edges and snapshot/provenance fingerprints.
- LineagePreconditionCapsuleV1: work_order_id, expected parent revision/fingerprint, expected store_generation, proposed revision/fingerprint and precondition_fingerprint. It is an external compare-and-set input, not a persistence command.
- DeterministicCompilationReceiptV1: request, context, compiler contract/algorithm, policy/config/security generations, lineage-precondition and output fingerprints. Diagnostics are bounded and outside the frozen semantic projection.
- WorkOrderAdmissionRequestV1: work_order_id/revision/fingerprint, requested admission mode, resolved source evidence, M02 workspace evidence, Context Lock evidence, external governance proof and current policy/security/config generations.
- WorkOrderAdmissionReceiptV1: exact work-order identity, compilation_id, status, source verification fingerprint, M02 workspace/generation/basis fingerprint, Context Lock fingerprint, governance proof fingerprint, policy generations, reason codes and receipt fingerprint. It is immutable evidence of one evaluation.
- AdmittedWorkOrderV1: frozen Work Order reference/fingerprint, exact READY receipt reference/fingerprint, packet DAG, scope, acceptance/evidence IDs, context plan and stop condition. It contains no Run/Attempt/Step state.
- DiagnosticV1: bounded diagnostic code, severity, safe subject IDs/fingerprints and optional redaction class. Raw prompt/source text, credentials, secret-bearing provider payloads and unbounded OS/process messages are forbidden.

Enums are closed, versioned and serialized in snake_case. The minimum enum domains are: admission status READY/REJECTED/STALE/BLOCKED/SUPERSEDED; evidence freshness CURRENT/STALE/UNKNOWN/SUBSTITUTED; basis compatibility EXACT_MATCH/COMPATIBLE_REFRESH/INCOMPATIBLE/UNKNOWN; scope effect ALLOW/DENY; delta class values from the Round 2 semantic delta model; and the error/retryability domains below. Unrecognized values fail closed.

### Semantic projection and canonical identity

WorkOrderFingerprint covers only the versioned semantic projection: envelope schema/version/kind; WorkOrderId and revision; objective; source identities, expected fingerprints, authority/provenance/freshness and expansion obligations; workspace, Context Lock and governance requirements; allow/deny and dependency policy; risk/assurance; packet nodes/edges/scopes; acceptance/evidence nodes/edges; context limits/mandatory references; correction policy; stop condition; and semantic lineage parent. Compiler implementation identity, canonicalization algorithm version and policy/config/security generations are deliberately excluded from WorkOrderFingerprint and are bound by WorkOrderCompilationId instead.

WorkOrderFingerprint excludes mutable diagnostics, timestamps, wall durations, UI/rendering order, transport IDs, PR URLs, raw context bodies, actual future evidence artifacts, and all M04 runtime identities. Diagnostic and transport data do not live inside FrozenWorkOrderV1. WorkOrderCompilationId additionally binds the WorkOrderFingerprint to compiler schema/algorithm, relevant policy/config/security generations and resolved compilation-context fingerprint. AdmissionReceipt fingerprint separately binds current freshness and authority evidence.

All unordered collections are explicitly sorted by their typed stable key before calling core-identity canonical_bytes/fingerprint. core-identity sorts JSON object keys but preserves array order; M03 must therefore sort source, packet, edge, criterion, evidence, policy and lineage collections itself. Canonical semantic bytes are independent of pretty-print formatting. Equivalent semantic permutations yield identical bytes/fingerprints; diagnostic-only differences leave WorkOrderFingerprint unchanged.

### Pure service API

The later crate exports these synchronous functions. They receive all semantic evidence and deterministic resource budgets explicitly, have no global/config/cwd/clock state and return no partially frozen or READY object on error. Caller-owned wall-clock timeout enforcement wraps these calls outside core-work-order and must discard any result from an invocation whose deadline has expired.

~~~rust
pub fn parse_request(
    input: &[u8],
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderEnvelope<WorkOrderRequestV1>, WorkOrderErrorV1>;

pub fn compile(
    request: &WorkOrderRequestV1,
    context: &CompilationContextV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderCompilationV1, WorkOrderErrorV1>;

pub fn validate_frozen(
    frozen: &FrozenWorkOrderV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderValidationReceiptV1, WorkOrderErrorV1>;

pub fn diff_revision(
    before: &FrozenWorkOrderV1,
    after: &FrozenWorkOrderV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderRevisionDiffV1, WorkOrderErrorV1>;

pub fn classify_correction(
    frozen: &FrozenWorkOrderV1,
    proposal: &ExecutionCorrectionProposalV1,
    budget: &M03ResourceBudgetV1,
) -> Result<CorrectionClassificationReceiptV1, WorkOrderErrorV1>;

pub fn evaluate_admission(
    frozen: &FrozenWorkOrderV1,
    request: &WorkOrderAdmissionRequestV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderAdmissionReceiptV1, WorkOrderErrorV1>;

pub fn materialize_handoff(
    frozen: &FrozenWorkOrderV1,
    receipt: &WorkOrderAdmissionReceiptV1,
    budget: &M03ResourceBudgetV1,
) -> Result<AdmittedWorkOrderV1, WorkOrderErrorV1>;

pub fn canonical_semantic_bytes(
    frozen: &FrozenWorkOrderV1,
) -> Result<Vec<u8>, WorkOrderErrorV1>;
~~~

CompilationContextV1 contains compiler_contract_version, algorithm_version, policy_generation, security_generation, config_generation, SourceResolutionBatchV1, authoritative LineageSnapshotV1 and optional HiveContextRefV1 values already resolved by a caller. It does not contain resolver objects or filesystem/network handles. evaluate_admission consumes already-resolved proof snapshots. materialize_handoff accepts only the exact matching READY immutable receipt. No service performs path reads, Git, HIVE, GitHub, network, process execution, refresh, retry, storage, commit, push or checkpoint update.

### External resolver and evidence adapter seams

The following caller-implemented traits define the external boundary. Their implementation may perform its separately authorized resolution work. M03 service functions never accept or invoke these traits; an outer host calls adapters first and passes bounded evidence DTOs to the pure functions.

~~~rust
pub trait CanonicalSourceResolverV1 {
    fn resolve(
        &self,
        refs: &[CanonicalSourceRefV1],
        budget: &AdapterBudgetV1,
    ) -> Result<SourceResolutionBatchV1, AdapterFailureV1>;
}

pub trait M02WorkspaceEvidenceResolverV1 {
    fn resolve(
        &self,
        requirement: &WorkspaceRequirementV1,
        request: &WorkspaceEvidenceRequestV1,
    ) -> Result<WorkspaceAdmissionEvidenceV1, AdapterFailureV1>;
}

pub trait ContextLockEvidenceResolverV1 {
    fn resolve(
        &self,
        requirement: &ContextLockRequirementV1,
        identity: &WorkOrderIdentityRefV1,
    ) -> Result<ContextLockEvidenceV1, AdapterFailureV1>;
}

pub trait GovernanceProofResolverV1 {
    fn resolve(
        &self,
        requirement: &GovernanceRequirementV1,
        identity: &WorkOrderIdentityRefV1,
        target: &ExactBaseHeadV1,
    ) -> Result<VerifiedGovernanceProofV1, AdapterFailureV1>;
}

pub trait HiveContextResolverV1 {
    fn resolve(
        &self,
        request: &HiveContextRequestV1,
        budget: &AdapterBudgetV1,
    ) -> Result<Vec<HiveContextRefV1>, AdapterFailureV1>;
}

pub trait ExternalLineageStoreV1 {
    fn snapshot(
        &self,
        work_order_id: &WorkOrderId,
    ) -> Result<LineageSnapshotV1, AdapterFailureV1>;

    fn compare_and_set(
        &self,
        capsule: &LineagePreconditionCapsuleV1,
    ) -> Result<LineageCasResultV1, AdapterFailureV1>;
}
~~~

WorkspaceAdmissionEvidenceV1 is a narrow value snapshot, not a second workspace model: it carries producer schema/version, ProjectBindingId and WorkspaceId values, M02 generation, basis fingerprint, required BVM profile and satisfied component references, compatibility/freshness state, provenance fingerprint and proof fingerprint. The external adapter maps the current M02 public evidence into this DTO. It does not copy paths, repository inventories, live handles or Git output. Wrong schema/version, missing required basis components, mismatch, replay or UNKNOWN blocks admission.

ContextLockEvidenceV1 carries lock schema/version/fingerprint, WorkOrderId/revision/fingerprint binding, authorized base/source-set fingerprints, implementation_authorized, status/freshness, verifier provenance and proof fingerprint. VerifiedGovernanceProofV1 carries project/repository identity, WorkOrderId/revision/fingerprint, exact base/head, external verdict, authorized scope fingerprint, policy generation, status/freshness, proof fingerprint and external verifier provenance. Only the external governance adapter verifies authority; M03 checks compatibility and cannot mint approval.

HiveContextRefV1 carries only HIVE project/context identity, content fingerprint, retrieval snapshot/provenance reference, freshness and an explicit advisory-only marker. It has no canonical authority and cannot override Git or M02 evidence. If optional HIVE lookup is unavailable, the adapter reports that state; it cannot fabricate a reference or change Work Order semantics.

Adapter errors use bounded codes and safe evidence references only. Adapter traits accept explicit identities/requirements and finite AdapterBudgetV1; ambient cwd, global clients, credentials and raw secret payloads are not part of M03 contracts.

### Typed errors and explicit retryability

WorkOrderErrorV1 contains category, code, retryability, bounded safe subject references and bounded diagnostics. Categories and required reason-code families are:

- SCHEMA_VERSION: UNSUPPORTED_SCHEMA, UNSUPPORTED_VERSION, INVALID_KIND, INVALID_ENVELOPE.
- SOURCE_PROVENANCE: SOURCE_MISSING, SOURCE_STALE, SOURCE_SUBSTITUTED, SOURCE_AUTHORITY_MISMATCH, SOURCE_EVIDENCE_UNKNOWN.
- SCOPE_DELTA: AMBIGUOUS_SCOPE, DENY_OVERRIDES_ALLOW, PACKET_SCOPE_WIDENING, FORBIDDEN_DELTA, DEPENDENCY_ADMISSION_REQUIRED.
- PACKET_GRAPH: DUPLICATE_ID, DANGLING_REFERENCE, CYCLE, GRAPH_LIMIT_EXCEEDED, NONDETERMINISTIC_ORDER.
- ACCEPTANCE_EVIDENCE: ACCEPTANCE_GAP, EVIDENCE_GAP, UNEXPLAINED_ORPHAN.
- LINEAGE: SNAPSHOT_STALE, REVISION_NOT_NEXT, SUPERSEDED_REVISION, LINEAGE_CONFLICT, LPC_MISMATCH.
- ADMISSION_STALENESS: WORKSPACE_MISMATCH, BASIS_INCOMPATIBLE, CONTEXT_LOCK_STALE, GOVERNANCE_PROOF_MISSING, GOVERNANCE_PROOF_MISMATCH, POLICY_STALE, RECEIPT_REPLAY, UNKNOWN_NOT_ADMISSIBLE.
- RESOURCE: REQUEST_TOO_LARGE, SERIALIZED_SIZE_EXCEEDED, CARDINALITY_LIMIT_EXCEEDED, GRAPH_LIMIT_EXCEEDED, CONTEXT_LIMIT_EXCEEDED.
- INTERNAL_INVARIANT: NONDETERMINISTIC_COMPILATION, FINGERPRINT_MISMATCH, PARTIAL_OUTPUT_FORBIDDEN, INTERNAL_INVARIANT_VIOLATION.

Retryability values are NEVER, AFTER_EXPLICIT_REFRESH, AFTER_GOVERNANCE, AFTER_RESOURCE_CHANGE and INTERNAL_BUG. The value describes a caller action only. M03 never performs the refresh, governance change, resource change or retry. A failed check that might change authority cannot be hidden behind a retry. Wall-clock timeout is not a WorkOrderErrorV1 generated by the pure core; the caller records typed orchestration deadline evidence and rejects any late result. The accepted Round 3 deadline reason families remain external timeout classifications rather than permission for ambient clock access.

### Frozen M03 V0.0 file and dependency map

M03 is one focused crate. The following is the implementation map, not files created by Round 4.

| Future path | Responsibility |
| --- | --- |
| Cargo.toml | Add crates/core-work-order as one root workspace member; preserve lockfile discipline. |
| Cargo.lock | Regenerate through Cargo only if adding the workspace member changes the resolved workspace package entries; never hand-edit. |
| crates/core-work-order/Cargo.toml | Declare the minimal direct dependencies below. |
| src/lib.rs | Public V1 exports, schema constants and pure service boundary. |
| src/contracts.rs | Envelopes, request/frozen/admission/handoff, source/scope/context/packet/AEG/lineage/evidence DTOs. |
| src/identity.rs | Typed IDs, validation and deterministic logical-key allocation. |
| src/canonical.rs | Semantic projection, explicit ordering, canonical bytes and fingerprint wrappers. |
| src/source.rs | Source manifest, bounded provenance and freshness rules. |
| src/adapters.rs | External resolver trait and adapter evidence interfaces; no adapter implementation or I/O. |
| src/compiler.rs | Parse/compile/freeze and validation orchestration over explicit values. |
| src/scope.rs | Allow/deny, scope intersection and correction firewall. |
| src/packets.rs | Bounded DAG validation and deterministic topological ordering. |
| src/acceptance.rs | AEG completeness, edge and N/A validation. |
| src/context.rs | Context budgets, source expansion and lossless PCM reconstruction. |
| src/lineage.rs | Snapshot/LPC validation and external CAS result classification. |
| src/delta.rs | Semantic revision diff and same-revision correction classification. |
| src/admission.rs | Explicit source/M02/lock/governance freshness admission and handoff checks. |
| src/budget.rs | Finite M03ResourceBudgetV1 validation and budget checks. |
| src/errors.rs | Closed error categories, codes, safe diagnostics and retryability. |
| src/service.rs | The pure public functions listed above. |
| tests/public_contracts.rs, tests/canonical_identity.rs | Public API/version round-trip, semantic projection, stable IDs and canonical vectors. |
| tests/source_workspace.rs, tests/adapters_no_io.rs | Source provenance, M02 snapshot bindings, freshness and external adapter boundary. |
| tests/scope_packets.rs, tests/acceptance_context.rs | Deny/intersection, DAG order, AEG completeness and PCM reconstruction. |
| tests/lineage_corrections.rs, tests/admission_replay.rs | LPC CAS, semantic deltas, correction policy, stale/replayed admission and handoff. |
| tests/resources_redaction.rs | Finite limits, no partial output and secret-safe bounded diagnostics. |
| fuzz/Cargo.toml | Add one core-work-order path dependency and explicit target declarations to the existing separate cargo-fuzz package; reuse libfuzzer-sys. |
| fuzz/Cargo.lock | Regenerate through Cargo only if the separate fuzz package resolution changes; never hand-edit. |
| fuzz/fuzz_targets/m03_*.rs | Bounded parser/canonicalizer, packet DAG, scope/delta, lineage/LPC, source provenance and admission/replay harnesses. |
| benches/m03_work_order.rs | Stable built-in bench harness for deterministic synthetic fixtures; no Criterion dependency. |

Unit laws live beside their modules; integration/property laws exercise only public contracts and service functions. Property cases use deterministic bounded generators/exhaustive permutations in the existing Rust test harness, so Round 4 admits no property-testing dependency. Fuzzing extends the existing libfuzzer-sys harness rather than adding a second fuzz runtime. Benchmarking uses the repository's existing harness=false plus std::time pattern, not Criterion.

### Dependency admission and acyclic rule

Deterministic inspection at base 9773d84 found ten workspace members and no core-work-order crate. The existing core-workspace manifest directly depends on core-config, core-contracts, core-identity, serde, serde_json, sha2, thiserror and Tokio with fs/io/macros/net/process/rt-multi-thread/sync/time features. M03 only needs M02 evidence values; linking the whole M02 service crate would import unnecessary process/network-capable transitive surface into the compiler boundary.

The frozen direct dependency set is therefore:

- Internal: core-identity only, for canonical_bytes/fingerprint and the already accepted SHA-256 identity stack. M02 evidence is adapted outside M03 into the bounded versioned DTOs above; core-workspace and core-config are not direct dependencies.
- Existing workspace third-party dependencies: serde with derive, serde_json for bounded JSON parsing/serialization, and thiserror for typed errors. sha2 is not direct because core-identity owns the fingerprint implementation.
- Fuzz-only: existing libfuzzer-sys in fuzz/Cargo.toml; no new fuzz engine.
- No proptest, Criterion, Tokio, Git library, graph library, regex engine, HIVE/GitHub SDK, network/process/filesystem crate, database, cache store or cryptography crate is admitted.

The transitive core-identity closure observed at this base is core-contracts, serde, serde_json, sha2 and thiserror; it has no process/network/database dependency. If an implementation proposal needs a different dependency or direct M02 type, it requires separate dependency/architecture admission with a new exact-head review. This narrows the Round 3 candidate list without changing its accepted ownership or evidence semantics.

The graph is acyclic: core-contracts/core-identity/core-config/core-workspace remain on their current downward graph; core-work-order depends only on core-identity plus serialization/error crates; the external host/adapters may depend on core-workspace and core-work-order to translate evidence; M04 may consume core-work-order. No dependency points from M02 to M03, from M03 to M04+, or from the pure M03 core back to an adapter/host.

### Property and adversarial law matrix

Every law is an implementation acceptance obligation. The planning round does not claim these tests have been implemented.

| Surface | Required law |
| --- | --- |
| Envelope/version | V1 round-trips; unknown schema/version/kind, duplicate required IDs and unsupported enum values fail typed with no downgrade. |
| Canonical identity | Equivalent field/map/collection permutations yield equal semantic bytes and fingerprints; diagnostics, timestamps and rendering changes do not. |
| IDs/revisions | Explicit or logical-key IDs are deterministic; revision starts at one, advances by one, is never reused, and never substitutes for fingerprint equality. |
| Packet DAG | All valid edge permutations yield one topological order; dangling IDs, duplicate IDs, self edges, cycles and budget overflow fail. |
| Scope firewall | Deny overrides allow; every packet scope is a subset/intersection of parent scope; ambiguity and hidden dependency changes block. |
| AEG/PCM | Every blocking criterion has complete evidence edges; no dangling/hidden obligation; reconstructing every packet context from PCM equals the independently required mandatory source set. |
| Lineage/LPC | Two competing N+1 capsules from one snapshot cannot both pass external CAS; stale store_generation is LINEAGE_CONFLICT; M03 performs no write/rebase. |
| Diff/corrections | Same-revision changes are limited to explicit policy classes; semantic, acceptance, scope, stop, dependency or security changes require a new revision; textual smallness cannot downgrade the class. |
| Source provenance | Altered identity/fingerprint/authority, missing proof, substitution, stale or UNKNOWN evidence cannot preserve freshness or READY. |
| Workspace/lock/governance | Wrong M02 schema, workspace/generation/basis/profile, Context Lock fingerprint, authorized base, governance verdict/head/scope/policy or replayed receipt fails closed. |
| Admission | Identical frozen revision and evidence yield identical semantic receipt; changed or unknown required evidence yields non-READY; old receipts are immutable and non-evergreen. |
| Diagnostics/secrets | Secret canaries, raw prompt/source bodies and raw adapter/process errors never appear in durable contracts or diagnostics; diagnostic differences never alter semantic identity. |
| Resource/atomicity | All core budgets are finite and positive; exact-bound inputs are handled according to policy; over-limit core cases produce typed errors and no partial FROZEN, READY or handoff object. Caller-owned wall-clock timeout discards any late result and records typed timeout evidence without changing pure-core determinism. |
| No hidden I/O | Public services can be replayed from value inputs alone and do not read files, invoke Git/HIVE/GitHub/process/network, access cwd/global clients or persist lineage. |

### Fuzz target matrix

Harnesses are pure, bounded and have no filesystem/network/process access. Each target caps byte input, nesting, collection cardinality and execution work through the supplied budget; panics, hangs, memory amplification, partial outputs and secret leakage are failures.

| Target | Input boundary and oracle |
| --- | --- |
| m03_envelope_canonical | Arbitrary bytes, JSON envelopes, schema/version/kind, duplicate IDs and canonical serialization; no panic and typed rejection. |
| m03_packet_dag | Packet IDs and edge lists; terminate within graph budget, reject cycles/dangling edges, stable order for equivalent graphs. |
| m03_scope_delta | Allow/deny rules and proposal deltas; deny precedence, intersection and no hidden scope/dependency widening. |
| m03_lineage_lpc | Revision/fingerprint/store-generation tuples; deterministic capsule, stale-CAS rejection and no internal persistence. |
| m03_source_provenance | Source refs/evidence/freshness/provenance combinations; substitution, authority mismatch and UNKNOWN never become current. |
| m03_admission_replay | Workspace, Context Lock, governance, policy and receipt bindings; any mismatch/replay/unknown required input is non-READY. |
| m03_diagnostics_redaction | Bounded hostile strings and secret canaries; output is bounded and redacted with no raw payload echo. |

Fuzz corpus seeds are synthetic public-schema samples and malformed boundary cases only; no real user repository contents or secrets.

### Benchmark and finite-resource calibration policy

No measured M03 result or numeric production default is claimed in Round 4. M03ResourceBudgetV1 has explicit finite positive fields for request/frozen bytes, string bytes, source refs, packets, DAG edges, scope rules, criteria, evidence nodes/edges, lineage edges, context refs, correction rules, diff entries, diagnostics and parser depth. There is no zero/unlimited sentinel, implicit host default or runtime self-tuning. Wall-clock deadline policy is measured and enforced by the caller/host boundary, never by reading time inside the pure core. Exact values remain CALIBRATION_GATED until code and reproducible fixtures exist.

The deterministic synthetic benchmark matrix varies:

- compile and validate by canonical sources, bytes and nested source references;
- packet DAG validation by packet nodes, edge count, width/depth and cycle checks;
- scope/correction classification by allow/deny rules, delta entries and changed semantic fields;
- AEG/PCM by criteria, evidence nodes/edges, shared refs and reconstructed mandatory sources;
- lineage/diff by prior revisions/edges and changed versus unchanged semantic fields;
- admission/handoff by source, M02, Context Lock, governance and policy evidence counts;
- serialization/context size by inline versus referenced material and deduplication ratio;
- resource boundary scenarios at the measured candidate cap and cap-plus-one rejection for every security-sensitive dimension.

Cold/warm/cache-hit/cache-miss cases apply only if a separately justified disposable L1 compile memo is later admitted. The baseline has no cache and no cache benchmark obligation. Cache and no-cache output must remain semantically identical if introduced.

Use deterministic local fixtures, no network, no HIVE calls and no LLM. Warm once, record at least five measured iterations per scenario, and report median/min/max alongside exact command, toolchain, OS, CPU, fixture generator/version, candidate SHA and semantic assertion results. Run the same relevant matrix on Windows and Ubuntu. Resource selection must be reproducible, finite, safe for supported inputs, reject unsupported scales rather than extrapolate, and include selected/rejected candidate rationale. Any hard thresholds/defaults enter only through the implementation Work Order's bounded Calibration Gate and committed report; a timeout/overflow or missing scenario blocks production acceptance and never yields partial success.

### Production DoD direction and stop state

M03 V0.0 later completes only when the exact planned contract/file/dependency map is implemented without scope drift; every public contract is V1-versioned; deterministic serialization/fingerprint golden vectors and all property/fuzz laws pass; all six service operations and adapter evidence are exact-head covered; M02/Context Lock/governance replay and freshness tests fail closed; diagnostic redaction is proven; no-hidden-I/O and dependency graph checks pass; every finite resource dimension is calibrated with a reproducible report; required performance/resource scenarios pass on Windows and Ubuntu; fuzz campaigns are bounded and successful; supply-chain/advisory/license/SBOM checks pass; AEG evidence binds every blocking DoD criterion; and independent exact-head review records no unresolved HIGH/CRITICAL defect.

Round 4 ends at a planning review candidate only. Independent exact-head audit and the existing governance gate must approve before checkpoint promotion. This section grants no M03 code implementation, Work Order execution, release, merge or next-round authority.
