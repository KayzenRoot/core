use core_workspace::{
    BasisComponent, CausalInvalidationGraph, ComponentMask, EventHint, EventKind, WorkspaceId,
};

#[test]
fn git_index_hint_only_invalidates_index() {
    let hint = EventHint {
        workspace_id: WorkspaceId::new("w"),
        repository_id: None,
        path: None,
        kind: EventKind::GitIndexHint,
        provider: "manual".into(),
        sequence: 1,
    };
    let result = CausalInvalidationGraph.map(&hint);
    assert_eq!(
        result.dirty_components,
        ComponentMask::only(BasisComponent::IndexState)
    );
    assert!(!result.broad);
}
