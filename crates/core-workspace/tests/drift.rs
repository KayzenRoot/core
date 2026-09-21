use core_workspace::{
    diff, AssociationStatus, ComponentMask, FilesystemSemanticsCapsuleV1, FilesystemSemanticsState,
    ProjectAssociationEvidence, ProjectBindingId, ReconciliationReceiptV1, ReconciliationStatus,
    RepositoryGraphV1, UntrackedPolicy, WorkspaceBasisV1, WorkspaceGeneration, WorkspaceId,
};
use std::collections::BTreeMap;

fn basis(head: &str) -> WorkspaceBasisV1 {
    let mut value = WorkspaceBasisV1 {
        schema_version: 1,
        project_binding_id: ProjectBindingId::new("binding"),
        workspace_id: WorkspaceId::new("workspace"),
        repository_id: None,
        worktree_id: None,
        authority_roots: Vec::new(),
        filesystem_semantics: FilesystemSemanticsCapsuleV1 {
            root_physical_identity: "physical".into(),
            namespace_class: "test".into(),
            canonical_path_evidence: "root".into(),
            case_state: FilesystemSemanticsState::CasePreservingUnknown,
            symlink_or_reparse_supported: true,
            normalization_policy_version: "v1".into(),
            provenance: "test".into(),
        },
        repository_graph: RepositoryGraphV1 {
            schema_version: 1,
            nodes: Vec::new(),
            edges: Vec::new(),
            fingerprint: "graph".into(),
        },
        git: None,
        association: ProjectAssociationEvidence {
            provider_origin: "solo".into(),
            provider_version: "1".into(),
            project_reference: None,
            asserted_workspace: None,
            asserted_repository: None,
            generation: 1,
            fingerprint: "assoc".into(),
            status: AssociationStatus::Unavailable,
            provenance: "test".into(),
        },
        reconciliation: ReconciliationReceiptV1 {
            status: ReconciliationStatus::StandaloneVerified,
            association: AssociationStatus::Unavailable,
            reason: "test".into(),
            local_fingerprint: "local".into(),
            hive_fingerprint: None,
        },
        untracked_policy: UntrackedPolicy::ContentHashed,
        config_generation: 1,
        security_generation: 1,
        component_fingerprints: BTreeMap::new(),
    };
    value.repository_graph.fingerprint = head.into();
    value.component_fingerprints = core_workspace::component_fingerprints(&value).unwrap();
    value
}

#[test]
fn changed_component_produces_reconstructable_diff() {
    let first = basis("one");
    let second = basis("two");
    let delta = diff(
        &first,
        &second,
        WorkspaceGeneration::new(1, 1),
        WorkspaceGeneration::new(1, 2),
    )
    .unwrap();
    assert_ne!(delta.changed_components, ComponentMask::NONE);
    assert!(delta
        .reasons
        .iter()
        .any(|reason| reason == "RepositoryGraph"));
}
