use crate::errors::{DiagnosticV1, WorkOrderErrorCodeV1};
use crate::identity::{
    AcceptanceCriterionId, ContextRefId, EvidenceFingerprintV1, EvidenceRequirementId,
    GovernanceProofId, LineageEdgeId, SemanticFieldIdV1, SourceRefId, WorkOrderCompilationId,
    WorkOrderFingerprint, WorkOrderId, WorkOrderLogicalKeyV1, WorkOrderRevision, WorkPacketId,
};
use serde::{Deserialize, Serialize};

pub const M03_SCHEMA: &str = "nexlabs.core.work-order";
pub const M03_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderEnvelope<T> {
    pub schema: String,
    pub version: u16,
    pub kind: WorkOrderContractKindV1,
    pub payload: T,
}

impl<T> WorkOrderEnvelope<T> {
    pub fn new(kind: WorkOrderContractKindV1, payload: T) -> Self {
        Self {
            schema: M03_SCHEMA.to_owned(),
            version: M03_VERSION,
            kind,
            payload,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClassV1 {
    Checkpoint,
    DecisionLedger,
    Scope,
    DefinitionOfDone,
    Architecture,
    Requirements,
    SecurityPolicy,
    TestPlan,
    ModulePlan,
    GovernancePolicy,
    WorkspaceBasis,
    ContextLock,
    HiveContextReference,
    PriorEvidenceReference,
    OtherVersionedCapability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityDomainV1 {
    CanonicalGit,
    Gef,
    WorkspaceEvidence,
    ExternalGovernance,
    HiveAdvisory,
    CallerProvided,
    Derived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocatorKindV1 {
    RepositoryPath,
    GitBlob,
    EvidenceReference,
    CapabilityReference,
    HiveReference,
    OpaqueVersionedReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFreshnessPolicyV1 {
    RequireCurrent,
    RequireFingerprintMatch,
    AdvisoryOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClassV1 {
    CanonicalGit,
    GefEvidence,
    M02Evidence,
    ExternalVerifier,
    HiveReference,
    CallerEvidence,
    Derived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretClassificationV1 {
    Public,
    Internal,
    SecretReferenceOnly,
    SecretForbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceExpansionPolicyV1 {
    AlwaysLoad,
    PacketOnDemand,
    ValidateFingerprintOnly,
    OptionalDiagnostic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalSourceRefV1 {
    pub source_id: SourceRefId,
    pub source_class: SourceClassV1,
    pub authority_domain: AuthorityDomainV1,
    pub locator_kind: LocatorKindV1,
    pub locator: String,
    pub expected_semantic_fingerprint: EvidenceFingerprintV1,
    pub required_for_compile: bool,
    pub required_for_admission: bool,
    pub required_packet_ids: Vec<WorkPacketId>,
    pub freshness_policy: SourceFreshnessPolicyV1,
    pub provenance_class: ProvenanceClassV1,
    pub secret_classification: SecretClassificationV1,
    pub expansion_policy: SourceExpansionPolicyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessV1 {
    Current,
    Stale,
    Unknown,
    Substituted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceResolutionEvidenceV1 {
    pub source_id: SourceRefId,
    pub requested_fingerprint: EvidenceFingerprintV1,
    pub observed_fingerprint: EvidenceFingerprintV1,
    pub authority_domain: AuthorityDomainV1,
    pub source_revision: String,
    pub resolver_schema: String,
    pub resolver_version: u16,
    pub freshness_state: EvidenceFreshnessV1,
    pub provenance_fingerprint: EvidenceFingerprintV1,
    pub evidence_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceResolutionBatchV1 {
    pub resolver_schema: String,
    pub resolver_version: u16,
    pub entries: Vec<SourceResolutionEvidenceV1>,
    pub batch_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceFreshnessProfileV1 {
    ReadMetadata,
    ReadSource,
    PlanWork,
    ExecuteToolReadonly,
    MutateSource,
    GitDelivery,
    HiveReconciledOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAssuranceRequirementV1 {
    StandaloneRequired,
    StandaloneOrHive,
    HiveReconciledRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirtyUntrackedPolicyV1 {
    RequireClean,
    AllowAndBind,
    ExcludedByPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasisCompatibilityV1 {
    ExactMatch,
    CompatibleRefresh,
    Incompatible,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceRequirementV1 {
    pub expected_project_binding_id: Option<String>,
    pub expected_workspace_id: Option<String>,
    pub required_profile: WorkspaceFreshnessProfileV1,
    pub required_basis_components: Vec<String>,
    pub allow_compatible_refresh: bool,
    pub assurance_requirement: WorkspaceAssuranceRequirementV1,
    pub dirty_untracked_policy: DirtyUntrackedPolicyV1,
    pub required_m02_schema: String,
    pub required_m02_version: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextLockRequirementV1 {
    pub required_schema: String,
    pub required_version: u16,
    pub require_active: bool,
    pub authorized_base_constraint: String,
    pub required_source_ids: Vec<SourceRefId>,
    pub require_implementation_authorized: bool,
    pub staleness_policy: SourceFreshnessPolicyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextLockStateV1 {
    Active,
    Inactive,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextLockEvidenceV1 {
    pub schema: String,
    pub version: u16,
    pub status: ContextLockStateV1,
    pub lock_fingerprint: EvidenceFingerprintV1,
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub authorized_base_fingerprint: EvidenceFingerprintV1,
    pub source_set_fingerprint: EvidenceFingerprintV1,
    pub implementation_authorized: bool,
    pub policy_generation: u64,
    pub freshness: EvidenceFreshnessV1,
    pub verifier_provenance: EvidenceFingerprintV1,
    pub proof_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalGovernanceVerdictV1 {
    Accepted,
    Rejected,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRequirementV1 {
    pub required_verdict: ExternalGovernanceVerdictV1,
    pub require_exact_base: bool,
    pub require_exact_head: bool,
    pub authorized_scope_fingerprint: EvidenceFingerprintV1,
    pub minimum_policy_generation: u64,
    pub require_current_freshness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HiveContextRefV1 {
    pub context_id: ContextRefId,
    pub hive_project_id: String,
    pub content_fingerprint: EvidenceFingerprintV1,
    pub snapshot_id: String,
    pub freshness: EvidenceFreshnessV1,
    pub provenance_fingerprint: EvidenceFingerprintV1,
    pub advisory_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderRevisionRefV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub fingerprint: WorkOrderFingerprint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageRelationV1 {
    Supersedes,
    SplitFrom,
    MergedFrom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderLineageEdgeV1 {
    pub edge_id: LineageEdgeId,
    pub from_revision: WorkOrderRevision,
    pub from_fingerprint: WorkOrderFingerprint,
    pub to_revision: WorkOrderRevision,
    pub to_fingerprint: WorkOrderFingerprint,
    pub relation: LineageRelationV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderLineageRefV1 {
    pub parent: Option<WorkOrderRevisionRefV1>,
    pub edges: Vec<WorkOrderLineageEdgeV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineageSnapshotV1 {
    pub work_order_id: WorkOrderId,
    pub current_revision: Option<WorkOrderRevision>,
    pub current_fingerprint: Option<WorkOrderFingerprint>,
    pub current_compilation_id: Option<WorkOrderCompilationId>,
    pub store_generation: u64,
    pub superseded_revisions: Vec<WorkOrderRevisionRefV1>,
    pub edges: Vec<WorkOrderLineageEdgeV1>,
    pub snapshot_fingerprint: EvidenceFingerprintV1,
    pub provenance_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineagePreconditionCapsuleV1 {
    pub work_order_id: WorkOrderId,
    pub expected_parent_revision: Option<WorkOrderRevision>,
    pub expected_parent_fingerprint: Option<WorkOrderFingerprint>,
    pub expected_store_generation: u64,
    pub proposed_revision: WorkOrderRevision,
    pub proposed_fingerprint: WorkOrderFingerprint,
    pub precondition_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageCasResultV1 {
    Applied,
    Conflict,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeEffectV1 {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationPolicyV1 {
    Deny,
    WithinAllowedScope,
    EvidenceOnly,
    DocumentationOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyChangePolicyV1 {
    NoChanges,
    NamedDependenciesOnly,
    GovernedAdmissionRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyPolicyV1 {
    pub change_policy: DependencyChangePolicyV1,
    pub allowed_direct_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClassV1 {
    File,
    Crate,
    Module,
    Repository,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrectionClassV1 {
    ImplementationWithinScope,
    TestOnlyWithinScope,
    EvidenceRegeneration,
    DocumentationWithinScope,
    NumericCalibration,
    GeneratedArtifactRefresh,
    DependencyAdmission,
    ScopeExpansion,
    ArchitectureChange,
    SecurityPolicyChange,
    AcceptanceWeakening,
    StopConditionWeakening,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeEnvelopeV1 {
    pub allowed_modules: Vec<String>,
    pub denied_modules: Vec<String>,
    pub allowed_crates_or_packages: Vec<String>,
    pub denied_crates_or_packages: Vec<String>,
    pub allowed_path_prefixes: Vec<String>,
    pub denied_path_prefixes: Vec<String>,
    pub allowed_artifact_classes: Vec<String>,
    pub denied_artifact_classes: Vec<String>,
    pub source_mutation_policy: MutationPolicyV1,
    pub documentation_mutation_policy: MutationPolicyV1,
    pub evidence_mutation_policy: MutationPolicyV1,
    pub generated_artifact_policy: MutationPolicyV1,
    pub dependency_policy: DependencyPolicyV1,
    pub allowed_correction_classes: Vec<CorrectionClassV1>,
    pub maximum_scope_class: ScopeClassV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskAssuranceProfileV1 {
    pub risk_class: String,
    pub assurance_mode: String,
    pub high_critical_blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextBudgetEnvelopeV1 {
    pub max_manifest_entries: u64,
    pub max_inline_bytes: u64,
    pub max_expanded_source_bytes: u64,
    pub max_packet_inline_bytes: u64,
    pub max_packet_expanded_bytes: u64,
    pub max_hive_refs: u64,
    pub max_prior_evidence_refs: u64,
    pub mandatory_source_ids: Vec<SourceRefId>,
    pub expansion_policy: SourceExpansionPolicyV1,
    pub allowed_expansion_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StopConditionV1 {
    pub success_predicates: Vec<String>,
    pub blocked_predicates: Vec<String>,
    pub prohibited_early_exit_conditions: Vec<String>,
    pub required_acceptance_criterion_ids: Vec<AcceptanceCriterionId>,
    pub required_evidence_requirement_ids: Vec<EvidenceRequirementId>,
    pub reviewer_verdict_required: bool,
    pub checkpoint_promotion_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkPacketSpecV1 {
    pub packet_id: WorkPacketId,
    pub objective: String,
    pub prerequisite_packet_ids: Vec<WorkPacketId>,
    pub required_source_ids: Vec<SourceRefId>,
    pub workspace_freshness_profile: WorkspaceFreshnessProfileV1,
    pub scope: ScopeEnvelopeV1,
    pub acceptance_criterion_ids: Vec<AcceptanceCriterionId>,
    pub evidence_requirement_ids: Vec<EvidenceRequirementId>,
    pub context_budget: ContextBudgetEnvelopeV1,
    pub stop_condition: StopConditionV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkPacketDagV1 {
    pub packets: Vec<WorkPacketSpecV1>,
    pub topological_order: Vec<WorkPacketId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicabilityPolicyV1 {
    AlwaysApplicable,
    DeterministicNotApplicableAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceCriterionV1 {
    pub criterion_id: AcceptanceCriterionId,
    pub statement: String,
    pub blocking: bool,
    pub required_evidence_ids: Vec<EvidenceRequirementId>,
    pub packet_ids: Vec<WorkPacketId>,
    pub applicability_policy: ApplicabilityPolicyV1,
    pub not_applicable_rationale: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRequirementV1 {
    pub evidence_id: EvidenceRequirementId,
    pub evidence_class: String,
    pub producer_module_hint: String,
    pub exact_head_required: bool,
    pub platform_requirements: Vec<String>,
    pub freshness_policy: SourceFreshnessPolicyV1,
    pub packet_ids: Vec<WorkPacketId>,
    pub security_classification: SecretClassificationV1,
    pub global: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CriterionEvidenceEdgeV1 {
    pub criterion_id: AcceptanceCriterionId,
    pub evidence_id: EvidenceRequirementId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceEvidenceGraphV1 {
    pub criteria: Vec<AcceptanceCriterionV1>,
    pub evidence_requirements: Vec<EvidenceRequirementV1>,
    pub edges: Vec<CriterionEvidenceEdgeV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrectionPolicyV1 {
    pub same_revision_classes: Vec<CorrectionClassV1>,
    pub forbidden_classes: Vec<CorrectionClassV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionCorrectionProposalV1 {
    pub proposal_fingerprint: EvidenceFingerprintV1,
    pub changed_paths: Vec<String>,
    pub artifact_classes: Vec<String>,
    pub requested_classes: Vec<CorrectionClassV1>,
    pub added_dependencies: Vec<String>,
    pub changed_semantic_fields: Vec<SemanticFieldIdV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderSemanticV1 {
    pub objective: String,
    pub scope: ScopeEnvelopeV1,
    pub packet_dag: WorkPacketDagV1,
    pub acceptance: AcceptanceEvidenceGraphV1,
    pub context_budget: ContextBudgetEnvelopeV1,
    pub correction_policy: CorrectionPolicyV1,
    pub stop_condition: StopConditionV1,
    pub risk_assurance: RiskAssuranceProfileV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeterministicCompilationReceiptV1 {
    pub request_fingerprint: EvidenceFingerprintV1,
    pub compilation_context_fingerprint: EvidenceFingerprintV1,
    pub compiler_contract_version: u16,
    pub algorithm_version: String,
    pub policy_generation: u64,
    pub config_generation: u64,
    pub security_generation: u64,
    pub lineage_precondition_fingerprint: Option<EvidenceFingerprintV1>,
    pub output_fingerprint: WorkOrderFingerprint,
    pub receipt_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

impl FrozenWorkOrderV1 {
    pub fn work_order_id(&self) -> &WorkOrderId {
        &self.work_order_id
    }
    pub fn revision(&self) -> WorkOrderRevision {
        self.revision
    }
    pub fn fingerprint(&self) -> &WorkOrderFingerprint {
        &self.fingerprint
    }
    pub fn compilation_id(&self) -> &WorkOrderCompilationId {
        &self.compilation_id
    }
    pub fn semantic(&self) -> &WorkOrderSemanticV1 {
        &self.semantic
    }
    pub fn source_manifest(&self) -> &[CanonicalSourceRefV1] {
        &self.source_manifest
    }
    pub fn workspace_requirement(&self) -> &WorkspaceRequirementV1 {
        &self.workspace_requirement
    }
    pub fn context_lock_requirement(&self) -> &ContextLockRequirementV1 {
        &self.context_lock_requirement
    }
    pub fn governance_requirement(&self) -> &GovernanceRequirementV1 {
        &self.governance_requirement
    }
    pub fn lineage(&self) -> &WorkOrderLineageRefV1 {
        &self.lineage
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
    ) -> Self {
        Self {
            work_order_id,
            revision,
            fingerprint,
            compilation_id,
            semantic,
            source_manifest,
            workspace_requirement,
            context_lock_requirement,
            governance_requirement,
            lineage,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderCompilationV1 {
    pub frozen: FrozenWorkOrderV1,
    pub receipt: DeterministicCompilationReceiptV1,
    pub lineage_precondition: Option<LineagePreconditionCapsuleV1>,
    pub diagnostics: Vec<DiagnosticV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderValidationReceiptV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub semantic_projection_fingerprint: EvidenceFingerprintV1,
    pub receipt_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrectionDispositionV1 {
    AllowedSameRevision,
    RequiresNewRevision,
    Forbidden,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrectionClassificationReceiptV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
    pub proposal_fingerprint: EvidenceFingerprintV1,
    pub disposition: CorrectionDispositionV1,
    pub reason_codes: Vec<WorkOrderErrorCodeV1>,
    pub receipt_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextMeshSourceV1 {
    pub context_ref_id: ContextRefId,
    pub source_id: SourceRefId,
    pub source_fingerprint: EvidenceFingerprintV1,
    pub expansion_policy: SourceExpansionPolicyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketContextEdgeV1 {
    pub packet_id: WorkPacketId,
    pub context_ref_id: ContextRefId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketContextMeshV1 {
    pub shared_sources: Vec<ContextMeshSourceV1>,
    pub packet_edges: Vec<PacketContextEdgeV1>,
    pub fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketContextPlanV1 {
    pub packet_id: WorkPacketId,
    pub stable_prefix_source_ids: Vec<SourceRefId>,
    pub packet_required_source_ids: Vec<SourceRefId>,
    pub validate_only_source_ids: Vec<SourceRefId>,
    pub optional_expandable_source_ids: Vec<SourceRefId>,
    pub context_budget: ContextBudgetEnvelopeV1,
    pub expansion_reasons: Vec<String>,
    pub manifest_fingerprint: EvidenceFingerprintV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionModeV1 {
    Execution,
    RunStartRevalidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderAdmissionStatusV1 {
    Ready,
    Rejected,
    Stale,
    Blocked,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderAdmissionReceiptV1 {
    work_order_id: WorkOrderId,
    revision: WorkOrderRevision,
    work_order_fingerprint: WorkOrderFingerprint,
    compilation_id: WorkOrderCompilationId,
    status: WorkOrderAdmissionStatusV1,
    source_batch_fingerprint: EvidenceFingerprintV1,
    workspace_id: String,
    workspace_runtime_epoch: u64,
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

impl WorkOrderAdmissionReceiptV1 {
    pub fn work_order_id(&self) -> &WorkOrderId {
        &self.work_order_id
    }
    pub fn revision(&self) -> WorkOrderRevision {
        self.revision
    }
    pub fn work_order_fingerprint(&self) -> &WorkOrderFingerprint {
        &self.work_order_fingerprint
    }
    pub fn compilation_id(&self) -> &WorkOrderCompilationId {
        &self.compilation_id
    }
    pub fn status(&self) -> WorkOrderAdmissionStatusV1 {
        self.status
    }
    pub fn source_batch_fingerprint(&self) -> &EvidenceFingerprintV1 {
        &self.source_batch_fingerprint
    }
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }
    pub fn workspace_runtime_epoch(&self) -> u64 {
        self.workspace_runtime_epoch
    }
    pub fn workspace_generation(&self) -> u64 {
        self.workspace_generation
    }
    pub fn workspace_basis_fingerprint(&self) -> &EvidenceFingerprintV1 {
        &self.workspace_basis_fingerprint
    }
    pub fn context_lock_fingerprint(&self) -> Option<&EvidenceFingerprintV1> {
        self.context_lock_fingerprint.as_ref()
    }
    pub fn governance_proof_fingerprint(&self) -> Option<&EvidenceFingerprintV1> {
        self.governance_proof_fingerprint.as_ref()
    }
    pub fn policy_generation(&self) -> u64 {
        self.policy_generation
    }
    pub fn security_generation(&self) -> u64 {
        self.security_generation
    }
    pub fn config_generation(&self) -> u64 {
        self.config_generation
    }
    pub fn reason_codes(&self) -> &[WorkOrderErrorCodeV1] {
        &self.reason_codes
    }
    pub fn receipt_fingerprint(&self) -> &EvidenceFingerprintV1 {
        &self.receipt_fingerprint
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        work_order_id: WorkOrderId,
        revision: WorkOrderRevision,
        work_order_fingerprint: WorkOrderFingerprint,
        compilation_id: WorkOrderCompilationId,
        status: WorkOrderAdmissionStatusV1,
        source_batch_fingerprint: EvidenceFingerprintV1,
        workspace_id: String,
        workspace_runtime_epoch: u64,
        workspace_generation: u64,
        workspace_basis_fingerprint: EvidenceFingerprintV1,
        context_lock_fingerprint: Option<EvidenceFingerprintV1>,
        governance_proof_fingerprint: Option<EvidenceFingerprintV1>,
        policy_generation: u64,
        security_generation: u64,
        config_generation: u64,
        reason_codes: Vec<WorkOrderErrorCodeV1>,
        receipt_fingerprint: EvidenceFingerprintV1,
    ) -> Self {
        Self {
            work_order_id,
            revision,
            work_order_fingerprint,
            compilation_id,
            status,
            source_batch_fingerprint,
            workspace_id,
            workspace_runtime_epoch,
            workspace_generation,
            workspace_basis_fingerprint,
            context_lock_fingerprint,
            governance_proof_fingerprint,
            policy_generation,
            security_generation,
            config_generation,
            reason_codes,
            receipt_fingerprint,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunStartRevalidationV1 {
    pub source_batch_fingerprint: EvidenceFingerprintV1,
    pub workspace_id: String,
    pub workspace_runtime_epoch: u64,
    pub workspace_generation: u64,
    pub workspace_basis_fingerprint: EvidenceFingerprintV1,
    pub context_lock_fingerprint: EvidenceFingerprintV1,
    pub governance_proof_fingerprint: EvidenceFingerprintV1,
    pub policy_generation: u64,
    pub security_generation: u64,
    pub config_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    run_start_revalidation: RunStartRevalidationV1,
}

impl AdmittedWorkOrderV1 {
    pub fn work_order_id(&self) -> &WorkOrderId {
        &self.work_order_id
    }
    pub fn revision(&self) -> WorkOrderRevision {
        self.revision
    }
    pub fn work_order_fingerprint(&self) -> &WorkOrderFingerprint {
        &self.work_order_fingerprint
    }
    pub fn compilation_id(&self) -> &WorkOrderCompilationId {
        &self.compilation_id
    }
    pub fn admission_receipt_fingerprint(&self) -> &EvidenceFingerprintV1 {
        &self.admission_receipt_fingerprint
    }
    pub fn packet_dag(&self) -> &WorkPacketDagV1 {
        &self.packet_dag
    }
    pub fn scope(&self) -> &ScopeEnvelopeV1 {
        &self.scope
    }
    pub fn acceptance_criterion_ids(&self) -> &[AcceptanceCriterionId] {
        &self.acceptance_criterion_ids
    }
    pub fn evidence_requirement_ids(&self) -> &[EvidenceRequirementId] {
        &self.evidence_requirement_ids
    }
    pub fn context_plan(&self) -> &PacketContextPlanV1 {
        &self.context_plan
    }
    pub fn stop_condition(&self) -> &StopConditionV1 {
        &self.stop_condition
    }
    pub fn run_start_revalidation(&self) -> &RunStartRevalidationV1 {
        &self.run_start_revalidation
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
        run_start_revalidation: RunStartRevalidationV1,
    ) -> Self {
        Self {
            work_order_id,
            revision,
            work_order_fingerprint,
            compilation_id,
            admission_receipt_fingerprint,
            packet_dag,
            scope,
            acceptance_criterion_ids,
            evidence_requirement_ids,
            context_plan,
            stop_condition,
            run_start_revalidation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderIdentityRefV1 {
    pub work_order_id: WorkOrderId,
    pub revision: WorkOrderRevision,
    pub work_order_fingerprint: WorkOrderFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactBaseHeadV1 {
    pub exact_base: String,
    pub exact_head: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEvidenceRequestV1 {
    pub required_profile: WorkspaceFreshnessProfileV1,
    pub required_components: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextLockEvidenceRequestV1 {
    pub requirement: ContextLockRequirementV1,
    pub identity: WorkOrderIdentityRefV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceProofRequestV1 {
    pub requirement: GovernanceRequirementV1,
    pub identity: WorkOrderIdentityRefV1,
    pub target: ExactBaseHeadV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HiveContextRequestV1 {
    pub work_order_id: WorkOrderId,
    pub packet_ids: Vec<WorkPacketId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterBudgetV1 {
    pub max_input_bytes: u64,
    pub max_result_entries: u64,
    pub max_string_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterFailureV1 {
    pub code: String,
    pub subject: Option<String>,
}
