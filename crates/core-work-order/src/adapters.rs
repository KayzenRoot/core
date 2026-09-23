use crate::contracts::*;

/// Implemented by an outer host. M03 services never accept or invoke it.
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
        work_order_id: &crate::identity::WorkOrderId,
    ) -> Result<LineageSnapshotV1, AdapterFailureV1>;

    fn compare_and_set(
        &self,
        capsule: &LineagePreconditionCapsuleV1,
    ) -> Result<LineageCasResultV1, AdapterFailureV1>;
}
