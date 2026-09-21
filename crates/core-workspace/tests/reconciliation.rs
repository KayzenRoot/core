use core_workspace::{
    reconcile, AssociationStatus, ProjectAssociationEvidence, ReconciliationStatus, RepositoryId,
    WorkspaceId,
};

fn evidence(
    workspace: WorkspaceId,
    repository: Option<RepositoryId>,
    status: AssociationStatus,
) -> ProjectAssociationEvidence {
    ProjectAssociationEvidence {
        provider_origin: "hive".into(),
        provider_version: "1".into(),
        project_reference: Some("core".into()),
        asserted_workspace: Some(workspace),
        asserted_repository: repository,
        generation: 1,
        fingerprint: "hive-proof".into(),
        status,
        provenance: "mcp-read-only".into(),
    }
}

#[test]
fn contradictory_hive_evidence_is_explicit_conflict() {
    let local = WorkspaceId::new("local");
    let remote = WorkspaceId::new("other");
    let result = reconcile(
        &local,
        None,
        "local-basis",
        &evidence(remote, None, AssociationStatus::Match),
    );
    assert_eq!(result.status, ReconciliationStatus::Conflict);
}

#[test]
fn unavailable_hive_keeps_standalone_binding() {
    let local = WorkspaceId::new("local");
    let result = reconcile(
        &local,
        None,
        "local-basis",
        &evidence(local.clone(), None, AssociationStatus::Unavailable),
    );
    assert_eq!(result.status, ReconciliationStatus::StandaloneVerified);
}
