use core_workspace::{
    BindingState, BindingStateMachine, M02Envelope, M02Error, WorkspaceAttachRequestV1,
    WorkspaceBindingReceiptV1, WorkspaceResourceBudget,
};

#[test]
fn unsupported_envelope_version_fails_closed() {
    let envelope = M02Envelope {
        schema: "nexlabs.core.workspace".to_owned(),
        version: 99,
        kind: "test".to_owned(),
        payload: 1_u8,
    };
    assert!(matches!(
        envelope.validate(),
        Err(M02Error::UnsupportedVersion { version: 99, .. })
    ));
}

#[test]
fn receipt_cannot_mint_live_handle() {
    let receipt = WorkspaceBindingReceiptV1 {
        envelope: M02Envelope::new(
            "workspace-binding-receipt",
            core_workspace::ReceiptPayloadV1 {
                project_binding_id: core_workspace::ProjectBindingId::new("p"),
                workspace_id: core_workspace::WorkspaceId::new("w"),
                repository_id: None,
                worktree_id: None,
                generation: core_workspace::WorkspaceGeneration::new(1, 1),
                basis_fingerprint: "basis".to_owned(),
                authority_roots: Vec::new(),
                reconciliation: core_workspace::ReconciliationReceiptV1 {
                    status: core_workspace::ReconciliationStatus::StandaloneVerified,
                    association: core_workspace::AssociationStatus::Unavailable,
                    reason: "test".to_owned(),
                    local_fingerprint: "local".to_owned(),
                    hive_fingerprint: None,
                },
                requires_use_time_revalidation: true,
            },
        ),
    };
    assert!(matches!(
        receipt.to_handle_without_revalidation(),
        Err(M02Error::ReceiptCannotMintHandle)
    ));
}

#[test]
fn attach_request_rejects_unbounded_budget() {
    let mut request = WorkspaceAttachRequestV1::new(".", 1);
    request.resource_budget = WorkspaceResourceBudget {
        max_repository_graph_nodes: 0,
        ..WorkspaceResourceBudget::default()
    };
    assert!(request.validate().is_err());
}

#[test]
fn lifecycle_does_not_skip_validation() {
    let mut machine = BindingStateMachine::default();
    machine.transition(BindingState::Discovering).unwrap();
    assert!(machine.transition(BindingState::Bound).is_err());
}
