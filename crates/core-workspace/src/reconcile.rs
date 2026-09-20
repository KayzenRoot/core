//! HIVE association capability seam and deterministic reconciliation.

use crate::{
    AssociationStatus, M02Error, ProjectAssociationEvidence, ProjectBindingId,
    ReconciliationReceiptV1, ReconciliationStatus, RepositoryId, WorkspaceId,
};
use core_identity::fingerprint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationRequest {
    pub workspace_id: WorkspaceId,
    pub repository_id: Option<RepositoryId>,
    pub project_reference: Option<String>,
    pub local_basis_fingerprint: String,
    pub generation: u64,
}

pub trait ProjectAssociationProvider: Send + Sync {
    fn resolve(&self, request: &AssociationRequest)
        -> Result<ProjectAssociationEvidence, M02Error>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NoopAssociationProvider;

impl ProjectAssociationProvider for NoopAssociationProvider {
    fn resolve(
        &self,
        request: &AssociationRequest,
    ) -> Result<ProjectAssociationEvidence, M02Error> {
        let fingerprint =
            fingerprint(&("unavailable", &request.workspace_id, &request.repository_id))
                .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        Ok(ProjectAssociationEvidence {
            provider_origin: "solo".into(),
            provider_version: "none-v1".into(),
            project_reference: None,
            asserted_workspace: None,
            asserted_repository: None,
            generation: request.generation,
            fingerprint,
            status: AssociationStatus::Unavailable,
            provenance: "no-hive-provider".into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StaticAssociationProvider {
    pub evidence: ProjectAssociationEvidence,
}

impl ProjectAssociationProvider for StaticAssociationProvider {
    fn resolve(
        &self,
        _request: &AssociationRequest,
    ) -> Result<ProjectAssociationEvidence, M02Error> {
        Ok(self.evidence.clone())
    }
}

pub fn reconcile(
    workspace_id: &WorkspaceId,
    repository_id: Option<&RepositoryId>,
    local_basis_fingerprint: &str,
    association: &ProjectAssociationEvidence,
) -> ReconciliationReceiptV1 {
    let conflict = association.status == AssociationStatus::Conflict
        || association
            .asserted_workspace
            .as_ref()
            .is_some_and(|value| value != workspace_id)
        || association
            .asserted_repository
            .as_ref()
            .zip(repository_id)
            .is_some_and(|(left, right)| left != right);
    let status = if conflict {
        ReconciliationStatus::Conflict
    } else if association.status == AssociationStatus::Match {
        ReconciliationStatus::Consistent
    } else if association.status == AssociationStatus::Unavailable
        || association.status == AssociationStatus::NotRequested
    {
        ReconciliationStatus::StandaloneVerified
    } else {
        ReconciliationStatus::Partial
    };
    ReconciliationReceiptV1 {
        status,
        association: association.status,
        reason: if conflict {
            "local-and-HIVE-evidence-conflict".into()
        } else {
            "authority-domains-reconciled-without-overwrite".into()
        },
        local_fingerprint: local_basis_fingerprint.into(),
        hive_fingerprint: Some(association.fingerprint.clone()),
    }
}

pub fn binding_id_for_reconciliation(
    workspace_id: &WorkspaceId,
    project_reference: Option<&str>,
) -> ProjectBindingId {
    crate::project_binding_id(workspace_id, project_reference)
}
