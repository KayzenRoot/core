use core_workspace::{
    build_graph, empty_graph_with_policy, ExternalObjectPolicy, GitEvidenceV1, RepositoryId,
    RepositoryNodeKind, UntrackedPolicy, WorkspaceResourceBudget, WorktreeId,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn evidence() -> GitEvidenceV1 {
    GitEvidenceV1 {
        provider: "test".into(),
        provider_version: "1".into(),
        repository_id: RepositoryId::new("repo"),
        worktree_id: Some(WorktreeId::new("tree")),
        is_bare: false,
        object_format: "sha1".into(),
        head: "head".into(),
        symbolic_head: None,
        index_fingerprint: "i".into(),
        tracked_delta_fingerprint: "t".into(),
        untracked_fingerprint: "u".into(),
        submodule_fingerprint: "s".into(),
        sparse_checkout_fingerprint: "sp".into(),
        common_dir: PathBuf::from("C:/repo/.git"),
        git_dir: PathBuf::from("C:/repo/.git"),
        redacted_remote_hints: vec!["https://<redacted>@example.test/core.git".into()],
        parsed_records: 1,
    }
}

#[test]
fn graph_contains_separate_repository_and_worktree_nodes() {
    let graph = build_graph(
        &PathBuf::from("C:/repo"),
        &evidence(),
        &WorkspaceResourceBudget::default(),
        ExternalObjectPolicy::Deny,
    )
    .unwrap();
    assert!(graph
        .nodes
        .iter()
        .any(|node| node.kind == RepositoryNodeKind::Repository));
    assert!(graph
        .nodes
        .iter()
        .any(|node| node.kind == RepositoryNodeKind::Worktree));
}

#[test]
fn standalone_graph_represents_nested_repository_boundary() {
    let root = std::env::temp_dir().join(format!("m02-nested-{}", std::process::id()));
    let nested = root.join("nested");
    fs::create_dir_all(&nested).unwrap();
    assert!(Command::new("git")
        .args(["init"])
        .current_dir(&nested)
        .status()
        .unwrap()
        .success());
    let graph = empty_graph_with_policy(
        &root,
        UntrackedPolicy::NamesOnly,
        &WorkspaceResourceBudget::default(),
    )
    .unwrap();
    assert!(graph
        .nodes
        .iter()
        .any(|node| node.kind == RepositoryNodeKind::NestedRepository));
    assert!(graph
        .edges
        .iter()
        .any(|edge| edge.kind == core_workspace::RepositoryEdgeKind::NestedWithin));
    let _ = fs::remove_dir_all(root);
}
