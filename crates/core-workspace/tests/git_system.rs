use core_workspace::git::{
    redacted_remote_for_test, GitInspectRequest, GitInspector, SystemGitInspector,
};
use core_workspace::{M02Error, UntrackedPolicy, WorkspaceResourceBudget};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

fn helper_path(root: &Path, name: &str) -> PathBuf {
    root.join(if cfg!(windows) {
        format!("{name}.cmd")
    } else {
        format!("{name}.sh")
    })
}

fn write_helper(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }
}

fn make_sleep_helper(path: &Path) {
    if cfg!(windows) {
        let source = path.with_extension("rs");
        write_helper(
            &source,
            r#"fn main() {
    std::thread::sleep(std::time::Duration::from_secs(30));
}
"#,
        );
        let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| std::ffi::OsString::from("rustc"));
        let output = Command::new(rustc)
            .args([
                "--edition=2021",
                source.to_string_lossy().as_ref(),
                "-o",
                path.to_string_lossy().as_ref(),
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "sleep helper compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        write_helper(
            path,
            "#!/bin/sh\nPATH=/usr/bin:/bin\nexport PATH\nexec sleep 30\n",
        );
    }
}

fn make_fsmonitor_helper(path: &Path, canary: &Path) {
    let canary = canary.to_string_lossy();
    if cfg!(windows) {
        write_helper(
            path,
            &format!("@echo off\r\n>\"{canary}\" echo invoked\r\nexit /b 0\r\n"),
        );
    } else {
        let shell_path = canary.replace('\'', "'\\''");
        write_helper(
            path,
            &format!("#!/bin/sh\nprintf invoked > '{shell_path}'\nexit 0\n"),
        );
    }
}

fn make_git_fixture(root: &Path) {
    fs::create_dir_all(root).unwrap();
    for args in [
        vec!["init"],
        vec!["config", "user.email", "m02@example.test"],
        vec!["config", "user.name", "M02 Test"],
    ] {
        let status = Command::new("git")
            .args(args)
            .current_dir(root)
            .status()
            .unwrap();
        assert!(status.success());
    }
}

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

#[test]
fn content_hashed_untracked_bytes_change_the_git_basis() {
    let root = std::env::temp_dir().join(format!("m02-git-untracked-{}", std::process::id()));
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
    std::fs::write(root.join("untracked.txt"), b"first").unwrap();
    let request = || GitInspectRequest {
        root: root.clone(),
        untracked_policy: UntrackedPolicy::ContentHashed,
        budget: WorkspaceResourceBudget::default(),
    };
    let first = SystemGitInspector::default()
        .inspect(&request())
        .unwrap()
        .unwrap();
    std::fs::write(root.join("untracked.txt"), b"second").unwrap();
    let second = SystemGitInspector::default()
        .inspect(&request())
        .unwrap()
        .unwrap();
    assert_ne!(first.untracked_fingerprint, second.untracked_fingerprint);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn hostile_fsmonitor_configuration_never_executes_a_canary() {
    let root = std::env::temp_dir().join(format!("m02-git-fsmonitor-{}", std::process::id()));
    make_git_fixture(&root);
    let canary = root.join("fsmonitor-canary");
    let helper = helper_path(&root, "fsmonitor-helper");
    make_fsmonitor_helper(&helper, &canary);
    let helper_value = helper.to_string_lossy().into_owned();
    let status = Command::new("git")
        .args(["config", "--local", "core.fsmonitor", &helper_value])
        .current_dir(&root)
        .status()
        .unwrap();
    assert!(status.success());

    let result = SystemGitInspector::default().inspect(&GitInspectRequest {
        root: root.clone(),
        untracked_policy: UntrackedPolicy::ExcludedByPolicy,
        budget: WorkspaceResourceBudget::default(),
    });
    if let Err(error) = result {
        assert!(!error.to_string().is_empty());
    }
    assert!(!canary.exists(), "hostile fsmonitor helper was executed");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn hostile_dual_output_is_capped_without_deadlock() {
    let root = std::env::temp_dir().join(format!("m02-git-dual-output-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let helper = helper_path(&root, "dual-output-helper");
    let body = if cfg!(windows) {
        "@echo off\r\necho xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\necho yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy 1>&2\r\n"
    } else {
        "#!/bin/sh\nprintf 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'\nprintf 'yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy' >&2\n"
    };
    write_helper(&helper, body);
    let budget = WorkspaceResourceBudget {
        max_stdout_bytes: 16,
        max_stderr_bytes: 16,
        ..WorkspaceResourceBudget::default()
    };
    let started = Instant::now();
    let error = SystemGitInspector {
        git_program: helper.to_string_lossy().into_owned(),
    }
    .inspect(&GitInspectRequest {
        root: root.clone(),
        untracked_policy: UntrackedPolicy::ExcludedByPolicy,
        budget,
    })
    .expect_err("dual output must exceed an independent cap");
    assert!(matches!(error, M02Error::ResourceBudgetExceeded(_)));
    assert!(started.elapsed() < Duration::from_secs(5));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn hostile_git_deadline_kills_reaps_and_does_not_poison_next_inspection() {
    // Keep the executable helper on the checked-out workspace filesystem. Some CI
    // runners can mount the OS temp directory with execution restrictions, which
    // would make this deadline test exercise spawn failure instead of timeout.
    let root = std::env::current_dir()
        .unwrap()
        .join("target")
        .join(format!("m02-git-deadline-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let helper = if cfg!(windows) {
        root.join("sleep-helper.exe")
    } else {
        helper_path(&root, "sleep-helper")
    };
    make_sleep_helper(&helper);
    let budget = WorkspaceResourceBudget {
        max_git_process_duration_ms: 500,
        ..WorkspaceResourceBudget::default()
    };
    let started = Instant::now();
    let error = SystemGitInspector {
        git_program: helper.to_string_lossy().into_owned(),
    }
    .inspect(&GitInspectRequest {
        root: root.clone(),
        untracked_policy: UntrackedPolicy::ExcludedByPolicy,
        budget,
    })
    .expect_err("sleeping fake Git must hit the typed deadline");
    assert!(matches!(error, M02Error::ResourceBudgetExceeded(_)));
    assert!(started.elapsed() < Duration::from_secs(5));

    let next_root =
        std::env::temp_dir().join(format!("m02-git-after-deadline-{}", std::process::id()));
    make_git_fixture(&next_root);
    let next_started = Instant::now();
    assert!(SystemGitInspector::default()
        .inspect(&GitInspectRequest {
            root: next_root.clone(),
            untracked_policy: UntrackedPolicy::ExcludedByPolicy,
            budget: WorkspaceResourceBudget::default(),
        })
        .is_ok());
    assert!(next_started.elapsed() < Duration::from_secs(5));
    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_dir_all(next_root);
}
