use core_workspace::git::{GitInspectRequest, GitInspector, SystemGitInspector};
use core_workspace::{
    build_graph, ExternalObjectPolicy, GitEvidenceV1, RepositoryId, WorkspaceResourceBudget,
    WorktreeId,
};
use std::path::PathBuf;
use std::process::Command;

fn evidence(root: &std::path::Path) -> GitEvidenceV1 {
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
        common_dir: root.join(".git"),
        git_dir: root.join(".git"),
        redacted_remote_hints: Vec::new(),
        parsed_records: 0,
    }
}

#[test]
fn external_object_store_is_denied_by_default() {
    let root = std::env::temp_dir().join(format!("m02-alternates-{}", std::process::id()));
    std::fs::create_dir_all(root.join(".git/objects/info")).unwrap();
    std::fs::write(
        root.join(".git/objects/info/alternates"),
        "C:/outside/object-store\n",
    )
    .unwrap();
    let result = build_graph(
        &root,
        &evidence(&root),
        &WorkspaceResourceBudget::default(),
        ExternalObjectPolicy::Deny,
    );
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn hostile_git_config_does_not_execute_external_helper() {
    let root = std::env::temp_dir().join(format!("m02-hostile-git-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let run = |args: &[&str]| {
        assert!(Command::new("git")
            .args(args)
            .current_dir(&root)
            .status()
            .unwrap()
            .success())
    };
    run(&["init"]);
    run(&["config", "user.email", "m02@example.test"]);
    run(&["config", "user.name", "M02 Test"]);
    run(&["config", "diff.external", "canary-do-not-execute"]);
    let result = SystemGitInspector::default().inspect(&GitInspectRequest {
        root: root.clone(),
        untracked_policy: core_workspace::UntrackedPolicy::NamesOnly,
        budget: WorkspaceResourceBudget::default(),
    });
    assert!(result.is_ok());
    assert!(!root.join("canary-do-not-execute").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn no_forbidden_path_authority_transitivity_is_encoded() {
    let _ = PathBuf::from("metadata");
    let graph = build_graph(
        PathBuf::from("C:/workspace").as_path(),
        &evidence(std::path::Path::new("C:/workspace")),
        &WorkspaceResourceBudget::default(),
        ExternalObjectPolicy::AdmitWithProvenance,
    )
    .unwrap();
    assert!(graph
        .nodes
        .iter()
        .any(|node| node.kind == core_workspace::RepositoryNodeKind::GitCommonDir));
}
