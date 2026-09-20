use core_workspace::{
    AssuranceRequirement, BvmProfile, PathOperation, PathValidationRequestV1, RevalidationOutcome,
    WorkspaceAttachRequestV1, WorkspaceService,
};
use std::fs;
use std::process::Command;

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
    let result = service.revalidate(&attached.handle).unwrap();
    assert_eq!(result.outcome, RevalidationOutcome::UpdatedCompatible);
    assert!(service
        .ensure_fresh(&attached.handle, BvmProfile::ReadMetadata)
        .is_err());
    service.detach(&result.handle.unwrap()).unwrap();
    let _ = fs::remove_dir_all(root);
}
