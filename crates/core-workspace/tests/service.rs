use core_workspace::{
    AssuranceRequirement, BasisComponent, BvmProfile, EventHint, EventKind, PathOperation,
    PathValidationRequestV1, RevalidationOutcome, WorkspaceAttachRequestV1,
    WorkspaceResourceBudget, WorkspaceService,
};
use std::fs;
use std::process::Command;

fn run_git(root: &std::path::Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(root)
        .status()
        .unwrap();
    assert!(status.success(), "git {:?} failed", args);
}

fn git_workspace(label: &str, runtime_epoch: u64) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("m02-service-{label}-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    run_git(&root, &["init"]);
    run_git(&root, &["config", "user.email", "m02@example.test"]);
    run_git(&root, &["config", "user.name", "M02 Test"]);
    fs::write(root.join("tracked.txt"), "initial").unwrap();
    run_git(&root, &["add", "tracked.txt"]);
    run_git(&root, &["commit", "-m", "fixture"]);
    let _ = runtime_epoch;
    root
}

fn add_nested_repository(root: &std::path::Path) -> std::path::PathBuf {
    let nested = root.join("nested");
    fs::create_dir_all(&nested).unwrap();
    run_git(&nested, &["init"]);
    nested
}

#[test]
fn attach_path_revalidate_and_detach_are_explicit() {
    let root = std::env::temp_dir().join(format!("m02-service-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    for args in [
        vec!["init"],
        vec!["config", "user.email", "m02@example.test"],
        vec!["config", "user.name", "M02 Test"],
    ] {
        assert!(Command::new("git")
            .args(args)
            .current_dir(&root)
            .status()
            .unwrap()
            .success());
    }
    fs::write(root.join("tracked.txt"), "tracked").unwrap();
    assert!(Command::new("git")
        .args(["add", "tracked.txt"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    assert!(Command::new("git")
        .args(["commit", "-m", "fixture"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    let mut service = WorkspaceService::new(7);
    let mut request = WorkspaceAttachRequestV1::new(&root, 7);
    request.assurance = AssuranceRequirement::Standalone;
    let attached = service.attach(request).unwrap();
    let authority = attached.basis.authority_roots[0].id.clone();
    let path = service
        .validate_path(
            &attached.handle,
            &PathValidationRequestV1 {
                requested: "new.txt".into(),
                operation: PathOperation::ReadSource,
                root: authority,
            },
        )
        .unwrap();
    assert!(path.use_time_revalidation_required);
    service
        .ensure_fresh(&attached.handle, BvmProfile::ReadMetadata)
        .unwrap();
    fs::write(root.join("new.txt"), "drift").unwrap();
    let result = service
        .revalidate_for(&attached.handle, BvmProfile::ReadSource)
        .unwrap();
    assert_eq!(result.outcome, RevalidationOutcome::UpdatedCompatible);
    assert!(service
        .ensure_fresh(&attached.handle, BvmProfile::ReadMetadata)
        .is_err());
    service.detach(&result.handle.unwrap()).unwrap();
    let _ = fs::remove_dir_all(root);
}

#[test]
fn git_content_hashing_uses_the_service_conveyor() {
    let root = std::env::temp_dir().join(format!("m02-service-bhc-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    for args in [
        vec!["init"],
        vec!["config", "user.email", "m02@example.test"],
        vec!["config", "user.name", "M02 Test"],
    ] {
        assert!(Command::new("git")
            .args(args)
            .current_dir(&root)
            .status()
            .unwrap()
            .success());
    }
    fs::write(root.join("untracked.txt"), "untracked").unwrap();
    let mut service = WorkspaceService::new(11);
    service
        .attach(WorkspaceAttachRequestV1::new(&root, 11))
        .unwrap();
    assert!(service.hash_operation_count() > 0);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn no_git_content_change_invalidates_standalone_binding() {
    let root = std::env::temp_dir().join(format!("m02-service-no-git-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("initial.txt"), "initial").unwrap();
    let mut service = WorkspaceService::new(8);
    let request = WorkspaceAttachRequestV1::new(&root, 8);
    let attached = service.attach(request).unwrap();
    fs::write(root.join("new.txt"), "new content").unwrap();
    let result = service.revalidate(&attached.handle).unwrap();
    assert_eq!(result.outcome, RevalidationOutcome::UpdatedCompatible);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn selective_metadata_revalidation_preserves_source_dirtiness() {
    let root = git_workspace("selective", 9);
    let source = root.join("tracked.txt");
    let mut service = WorkspaceService::new(9);
    let attached = service
        .attach(WorkspaceAttachRequestV1::new(&root, 9))
        .unwrap();
    fs::write(&source, "changed").unwrap();
    let hint = EventHint {
        workspace_id: attached.basis.workspace_id.clone(),
        repository_id: None,
        path: Some(source.clone()),
        kind: EventKind::PathContentChanged,
        provider: "test".into(),
        sequence: 1,
    };
    service.push_event_hint(hint);
    let metadata = service
        .revalidate_for(&attached.handle, BvmProfile::ReadMetadata)
        .unwrap();
    assert_eq!(metadata.outcome, RevalidationOutcome::StillValid);
    assert!(service
        .ensure_fresh(&attached.handle, BvmProfile::ReadSource)
        .is_err());
    let source_result = service
        .revalidate_for(&attached.handle, BvmProfile::ReadSource)
        .unwrap();
    assert_eq!(
        source_result.outcome,
        RevalidationOutcome::UpdatedCompatible
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn unrelated_event_hint_cannot_hide_nested_repository_graph_drift() {
    let root = git_workspace("nested-drift", 12);
    let mut service = WorkspaceService::new(12);
    let attached = service
        .attach(WorkspaceAttachRequestV1::new(&root, 12))
        .unwrap();
    let _nested = add_nested_repository(&root);
    service.push_event_hint(EventHint {
        workspace_id: attached.basis.workspace_id.clone(),
        repository_id: None,
        path: Some(root.join("unrelated.txt")),
        kind: EventKind::PathContentChanged,
        provider: "test".into(),
        sequence: 1,
    });

    let result = service
        .revalidate_for(&attached.handle, BvmProfile::ReadMetadata)
        .unwrap();
    assert_ne!(result.outcome, RevalidationOutcome::StillValid);
    assert!(result
        .diff
        .as_ref()
        .expect("graph drift must produce a delta")
        .changed_components
        .contains(BasisComponent::RepositoryGraph));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn selective_revalidation_matches_full_recomputation_for_same_mask() {
    let root = git_workspace("dws-equivalence", 13);
    let mut selective = WorkspaceService::new(13);
    let mut full = WorkspaceService::new(13);
    let selective_attached = selective
        .attach(WorkspaceAttachRequestV1::new(&root, 13))
        .unwrap();
    let full_attached = full
        .attach(WorkspaceAttachRequestV1::new(&root, 13))
        .unwrap();
    let _nested = add_nested_repository(&root);
    selective.push_event_hint(EventHint {
        workspace_id: selective_attached.basis.workspace_id.clone(),
        repository_id: None,
        path: Some(root.join("unrelated.txt")),
        kind: EventKind::PathContentChanged,
        provider: "test".into(),
        sequence: 1,
    });

    let selective_result = selective
        .revalidate_for(&selective_attached.handle, BvmProfile::ReadMetadata)
        .unwrap();
    let full_result = full
        .revalidate_for(&full_attached.handle, BvmProfile::ReadMetadata)
        .unwrap();
    assert_eq!(selective_result, full_result);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn invalidation_overflow_broadens_and_blocks_freshness() {
    let root = std::env::temp_dir().join(format!("m02-service-overflow-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("source.txt"), "initial").unwrap();
    let mut service = WorkspaceService::new(10);
    let attached = service
        .attach(WorkspaceAttachRequestV1::new(&root, 10))
        .unwrap();
    let budget = WorkspaceResourceBudget {
        max_event_hint_backlog: 1,
        ..WorkspaceResourceBudget::default()
    };
    service.set_budget(budget).unwrap();
    let hint = || EventHint {
        workspace_id: attached.basis.workspace_id.clone(),
        repository_id: None,
        path: None,
        kind: EventKind::PathMetadataChanged,
        provider: "test".into(),
        sequence: 1,
    };
    service.push_event_hint(hint());
    let overflow = service.push_event_hint(hint());
    assert!(overflow.broad);
    assert_eq!(
        service.dirty_components(),
        core_workspace::ComponentMask::ALL
    );
    assert!(service
        .ensure_fresh(&attached.handle, BvmProfile::ReadMetadata)
        .is_err());
    let _ = fs::remove_dir_all(root);
}
