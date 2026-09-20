//! Canonical workspace basis and deterministic component deltas.

use crate::identity::workspace_id;
use crate::reconcile::{AssociationRequest, ProjectAssociationProvider};
use crate::{
    AuthorityRootV1, ComponentMask, M02Error, RepositoryGraphV1, WorkspaceAttachRequestV1,
    WorkspaceBasisDiffV1, WorkspaceBasisV1, WorkspaceGeneration,
};
use core_identity::fingerprint;
use std::collections::BTreeMap;

pub fn build_basis(
    request: &WorkspaceAttachRequestV1,
    authority: AuthorityRootV1,
    graph: RepositoryGraphV1,
    git: Option<crate::GitEvidenceV1>,
    association: crate::ProjectAssociationEvidence,
) -> Result<WorkspaceBasisV1, M02Error> {
    let capsule = crate::probe_filesystem_semantics(&authority.canonical_existing_root)?;
    let workspace_id = workspace_id(
        &authority.canonical_existing_root,
        &capsule.root_physical_identity,
        &authority.filesystem_semantics_fingerprint,
    );
    let repository_id = git.as_ref().map(|value| value.repository_id.clone());
    let worktree_id = git.as_ref().and_then(|value| value.worktree_id.clone());
    let local_basis = fingerprint(&(
        &workspace_id,
        &repository_id,
        &worktree_id,
        &graph.fingerprint,
        request.policy_generation,
        request.security_generation,
    ))
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    let reconciliation = crate::reconcile::reconcile(
        &workspace_id,
        repository_id.as_ref(),
        &local_basis,
        &association,
    );
    if reconciliation.status == crate::ReconciliationStatus::Conflict
        && matches!(
            request.assurance,
            crate::AssuranceRequirement::HiveReconciled
        )
    {
        return Err(M02Error::AssociationConflict(reconciliation.reason.clone()));
    }
    let mut basis = WorkspaceBasisV1 {
        schema_version: crate::M02_VERSION,
        project_binding_id: crate::project_binding_id(
            &workspace_id,
            request.hive_project_reference.as_deref(),
        ),
        workspace_id,
        repository_id,
        worktree_id,
        authority_roots: vec![authority],
        filesystem_semantics: capsule,
        repository_graph: graph,
        git,
        association,
        reconciliation,
        untracked_policy: request.untracked_policy,
        config_generation: request.policy_generation,
        security_generation: request.security_generation,
        component_fingerprints: BTreeMap::new(),
    };
    basis.component_fingerprints = component_fingerprints(&basis)?;
    Ok(basis)
}

pub fn component_fingerprints(
    basis: &WorkspaceBasisV1,
) -> Result<BTreeMap<crate::BasisComponent, String>, M02Error> {
    use crate::BasisComponent as C;
    let mut values = BTreeMap::new();
    // The explicit fields below are canonical and exclude observation timestamps.
    values.insert(
        C::Identity,
        fingerprint(&(&basis.project_binding_id, &basis.workspace_id))
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::Authority,
        fingerprint(&basis.authority_roots).map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::RepositoryGraph,
        basis.repository_graph.fingerprint.clone(),
    );
    values.insert(
        C::WorktreeIdentity,
        fingerprint(&(&basis.repository_id, &basis.worktree_id))
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::HeadState,
        fingerprint(
            &basis
                .git
                .as_ref()
                .map(|git| (&git.head, &git.symbolic_head)),
        )
        .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::IndexState,
        fingerprint(&basis.git.as_ref().map(|git| &git.index_fingerprint))
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::TrackedWorktreeState,
        fingerprint(&basis.git.as_ref().map(|git| &git.tracked_delta_fingerprint))
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::UntrackedWorktreeState,
        fingerprint(&(
            &basis.untracked_policy,
            &basis.git.as_ref().map(|git| &git.untracked_fingerprint),
        ))
        .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::FilesystemSemantics,
        fingerprint(&basis.filesystem_semantics)
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::ConfigGeneration,
        fingerprint(&basis.config_generation).map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::SecurityPolicy,
        fingerprint(&basis.security_generation)
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::ProjectAssociation,
        fingerprint(&basis.association).map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::SubmoduleState,
        fingerprint(&basis.git.as_ref().map(|git| &git.submodule_fingerprint))
            .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    values.insert(
        C::SparseCheckoutState,
        fingerprint(
            &basis
                .git
                .as_ref()
                .map(|git| &git.sparse_checkout_fingerprint),
        )
        .map_err(|e| M02Error::InvalidInput(e.to_string()))?,
    );
    Ok(values)
}

pub fn diff(
    before: &WorkspaceBasisV1,
    after: &WorkspaceBasisV1,
    from_generation: WorkspaceGeneration,
    to_generation: WorkspaceGeneration,
) -> Result<WorkspaceBasisDiffV1, M02Error> {
    let mut changed = ComponentMask::NONE;
    let mut reasons = Vec::new();
    for (component, value) in &after.component_fingerprints {
        if before.component_fingerprints.get(component) != Some(value) {
            changed = changed.union(ComponentMask::only(*component));
            reasons.push(format!("{component:?}"));
        }
    }
    Ok(WorkspaceBasisDiffV1 {
        from_generation,
        to_generation,
        changed_components: changed,
        before_fingerprint: before.fingerprint()?,
        after_fingerprint: after.fingerprint()?,
        reasons,
    })
}

pub fn required_components_fresh(
    basis: &WorkspaceBasisV1,
    required: ComponentMask,
    dirty: ComponentMask,
) -> Result<(), M02Error> {
    if required.0 & dirty.0 != 0 {
        return Err(M02Error::StaleHandle(format!(
            "required basis components are dirty: {dirty:?}"
        )));
    }
    basis.validate_schema()
}

pub fn association_for_request(
    provider: &dyn ProjectAssociationProvider,
    workspace_id: crate::WorkspaceId,
    repository_id: Option<crate::RepositoryId>,
    request: &WorkspaceAttachRequestV1,
    local_basis_fingerprint: String,
) -> Result<crate::ProjectAssociationEvidence, M02Error> {
    provider.resolve(&AssociationRequest {
        workspace_id,
        repository_id,
        project_reference: request.hive_project_reference.clone(),
        local_basis_fingerprint,
        generation: request.policy_generation,
    })
}
