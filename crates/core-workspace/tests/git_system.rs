use core_workspace::git::{
    redacted_remote_for_test, GitInspectRequest, GitInspector, SystemGitInspector,
};
use core_workspace::{M02Error, UntrackedPolicy, WorkspaceResourceBudget};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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
    // Compile the same native helper on both OSes. A shell script plus an
    // external sleep binary can exit before the inspector's timeout path,
    // turning a deadline regression test into a false "not a Git repo".
    let source = path.with_extension("rs");
    write_helper(
        &source,
        r#"fn main() {
    if let Some(path) = std::env::args()
        .find_map(|arg| arg.strip_prefix("--probe-ready=").map(str::to_owned))
    {
        std::fs::write(path, b"ready").expect("probe marker");
    }
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
}

fn make_dual_output_helper(path: &Path) {
    // A native executable works uniformly with the inspector's env_clear()
    // on both platforms; a shell script in OS temp could fail before
    // producing any bytes and be confused with "not a Git repository".
    let source = path.with_extension("rs");
    write_helper(
        &source,
        r#"fn main() {
    use std::io::Write;
    std::io::stdout().write_all(&[b'x'; 64]).expect("write stdout");
    std::io::stderr().write_all(&[b'y'; 64]).expect("write stderr");
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
        "dual-output helper compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
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
    // Use the checked-out workspace target tree instead of a potentially
    // noexec OS temp directory. Preflight the same native binary under the
    // empty environment used by the production Git inspector.
    let root = std::env::current_dir()
        .unwrap()
        .join("target")
        .join(format!("m02-git-dual-output-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let helper = root.join(if cfg!(windows) {
        "dual-output-helper.exe"
    } else {
        "dual-output-helper"
    });
    make_dual_output_helper(&helper);

    let mut probe = Command::new(&helper)
        .current_dir(&root)
        .env_clear()
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("native dual-output helper must start");
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut exited = false;
    while Instant::now() < deadline {
        if probe.try_wait().expect("probe helper status").is_some() {
            exited = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    if !exited {
        probe.kill().expect("terminate an unresponsive probe");
    }
    // Reap both the normal-exit and timeout paths, then inspect each stream.
    let status = probe.wait().expect("reap the probe helper");
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    probe
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut stdout)
        .unwrap();
    probe
        .stderr
        .take()
        .unwrap()
        .read_to_end(&mut stderr)
        .unwrap();
    assert!(exited && status.success(), "native dual-output preflight failed");
    assert_eq!(stdout, vec![b'x'; 64]);
    assert_eq!(stderr, vec![b'y'; 64]);

    // Prove each cap independently, then exercise both simultaneous caps.
    for (stdout_cap, stderr_cap) in [(16, 1024), (1024, 16), (16, 16)] {
        let budget = WorkspaceResourceBudget {
            max_stdout_bytes: stdout_cap,
            max_stderr_bytes: stderr_cap,
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
        .expect_err("native dual output must hit the independent cap");
        assert!(matches!(error, M02Error::ResourceBudgetExceeded(_)));
        assert!(started.elapsed() < Duration::from_secs(5));
    }
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
    let helper = root.join(if cfg!(windows) {
        "sleep-helper.exe"
    } else {
        "sleep-helper"
    });
    make_sleep_helper(&helper);

    // Prove the exact helper starts with no inherited environment before
    // asserting timeout behavior. Diagnose an early exit instead of silently
    // treating the test as proof that the inspector missed a deadline.
    let ready = root.join("native-helper-ready");
    // A prior panic can leave this PID-scoped target directory behind.
    let _ = fs::remove_file(&ready);
    let mut probe = Command::new(&helper)
        .arg(format!("--probe-ready={}", ready.display()))
        .current_dir(&root)
        .env_clear()
        .spawn()
        .expect("native sleep helper must start with an empty environment");
    let preflight_deadline = Instant::now() + Duration::from_secs(5);
    let mut early_exit = None;
    while !ready.exists() && Instant::now() < preflight_deadline {
        early_exit = probe.try_wait().expect("probe helper status");
        if early_exit.is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    let ready_seen = ready.exists();
    if early_exit.is_none() {
        probe.kill().expect("terminate the probe helper");
    }
    // wait() must be called on both the early-exit and the kill paths.
    probe.wait().expect("reap the probe helper");
    assert!(
        ready_seen && early_exit.is_none(),
        "native sleep helper did not become ready: early_exit={early_exit:?}"
    );

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
