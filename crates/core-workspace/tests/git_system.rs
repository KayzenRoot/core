use core_workspace::git::{
    redacted_remote_for_test, GitInspectRequest, GitInspector, SystemGitInspector,
};
use core_workspace::{UntrackedPolicy, WorkspaceResourceBudget};
use std::process::Command;

#[test]
fn remote_credentials_are_redacted_before_evidence() {
    assert_eq!(
        redacted_remote_for_test("https://user:secret@example.test/repo.git"),
        "https://<redacted>@example.test/repo.git"
    );
    assert_eq!(
        redacted_remote_for_test("https://example.test/repo.git?token=abc"),
        "<redacted-remote>"
    );
}

#[test]
fn system_git_inspects_a_local_repository_without_network_or_mutation() {
    let root = std::env::temp_dir().join(format!("m02-git-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let run = |args: &[&str]| {
        let status = Command::new("git")
            .args(args)
            .current_dir(&root)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    };
    run(&["init"]);
    run(&["config", "user.email", "m02@example.test"]);
    run(&["config", "user.name", "M02 Test"]);
    std::fs::write(root.join("file.txt"), "content").unwrap();
    run(&["add", "file.txt"]);
    run(&["commit", "-m", "fixture"]);
    let before = std::fs::read(root.join("file.txt")).unwrap();
    let evidence = SystemGitInspector::default()
        .inspect(&GitInspectRequest {
            root: root.clone(),
            untracked_policy: UntrackedPolicy::ContentHashed,
            budget: WorkspaceResourceBudget::default(),
        })
        .unwrap()
        .unwrap();
    assert!(!evidence.is_bare);
    assert!(!evidence.head.is_empty());
    assert_eq!(before, std::fs::read(root.join("file.txt")).unwrap());
    let _ = std::fs::remove_dir_all(root);
}
