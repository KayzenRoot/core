//! Hardened system-Git provider. Commands are explicit argv and read-only.

use super::{GitInspectRequest, GitInspector};
use crate::identity::{repository_id, worktree_id};
use crate::{GitEvidenceV1, M02Error};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct SystemGitInspector {
    pub git_program: String,
}

impl Default for SystemGitInspector {
    fn default() -> Self {
        Self {
            git_program: "git".into(),
        }
    }
}

struct GitOutput {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    status: std::process::ExitStatus,
}

impl SystemGitInspector {
    fn run(
        &self,
        root: &Path,
        args: &[&str],
        budget: &crate::WorkspaceResourceBudget,
    ) -> Result<GitOutput, M02Error> {
        let null_config = if cfg!(windows) { "NUL" } else { "/dev/null" };
        let mut command = Command::new(&self.git_program);
        command
            .args(["--no-optional-locks"])
            .args(args)
            .current_dir(root)
            .env_clear()
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", null_config)
            .env("GIT_CONFIG_SYSTEM", null_config)
            .env("GIT_PAGER", "cat")
            .env("GIT_EDITOR", ":")
            .env("GIT_ASKPASS", "")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command
            .spawn()
            .map_err(|error| M02Error::GitInspection(format!("spawn git: {error}")))?;
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        if let Some(pipe) = child.stdout.take() {
            pipe.take(budget.max_stdout_bytes.saturating_add(1))
                .read_to_end(&mut stdout)
                .map_err(|error| M02Error::GitInspection(format!("read stdout: {error}")))?;
        }
        if stdout.len() as u64 > budget.max_stdout_bytes {
            let _ = child.kill();
            return Err(M02Error::ResourceBudgetExceeded("Git stdout".into()));
        }
        if let Some(pipe) = child.stderr.take() {
            pipe.take(budget.max_stderr_bytes.saturating_add(1))
                .read_to_end(&mut stderr)
                .map_err(|error| M02Error::GitInspection(format!("read stderr: {error}")))?;
        }
        if stderr.len() as u64 > budget.max_stderr_bytes {
            let _ = child.kill();
            return Err(M02Error::ResourceBudgetExceeded("Git stderr".into()));
        }
        let status = child
            .wait()
            .map_err(|error| M02Error::GitInspection(format!("wait git: {error}")))?;
        Ok(GitOutput {
            stdout,
            stderr,
            status,
        })
    }

    fn query(
        &self,
        root: &Path,
        args: &[&str],
        budget: &crate::WorkspaceResourceBudget,
    ) -> Result<String, M02Error> {
        let result = self.run(root, args, budget)?;
        if !result.status.success() {
            return Err(M02Error::GitInspection(
                String::from_utf8_lossy(&result.stderr).trim().to_owned(),
            ));
        }
        String::from_utf8(result.stdout)
            .map(|value| value.trim().to_owned())
            .map_err(|error| M02Error::GitInspection(format!("non-UTF8 Git output: {error}")))
    }

    fn hash_bytes(bytes: &[u8]) -> String {
        let digest = Sha256::digest(bytes);
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn resolve_git_path(root: &Path, value: &str) -> PathBuf {
        let candidate = PathBuf::from(value);
        if candidate.is_absolute() {
            candidate
        } else {
            root.join(candidate)
        }
    }

    fn redact_remote(value: &str) -> String {
        let mut redacted = value.trim().to_owned();
        if let Some(scheme) = redacted.find("://") {
            let authority_start = scheme + 3;
            if let Some(at) = redacted[authority_start..].find('@') {
                redacted.replace_range(authority_start..authority_start + at + 1, "<redacted>@");
            }
        }
        if redacted.contains("token=")
            || redacted.contains("password=")
            || redacted.contains("secret=")
        {
            redacted = "<redacted-remote>".into();
        }
        redacted
    }
}

impl GitInspector for SystemGitInspector {
    fn inspect(&self, request: &GitInspectRequest) -> Result<Option<GitEvidenceV1>, M02Error> {
        request
            .budget
            .validate()
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        let git_dir_raw =
            match self.query(&request.root, &["rev-parse", "--git-dir"], &request.budget) {
                Ok(value) => value,
                Err(M02Error::GitInspection(_)) => return Ok(None),
                Err(error) => return Err(error),
            };
        let common_raw = self.query(
            &request.root,
            &["rev-parse", "--git-common-dir"],
            &request.budget,
        )?;
        let bare = self.query(
            &request.root,
            &["rev-parse", "--is-bare-repository"],
            &request.budget,
        )? == "true";
        let root_path = if bare {
            request
                .root
                .canonicalize()
                .map_err(|error| M02Error::GitInspection(error.to_string()))?
        } else {
            PathBuf::from(self.query(
                &request.root,
                &["rev-parse", "--show-toplevel"],
                &request.budget,
            )?)
        };
        let git_dir = Self::resolve_git_path(&request.root, &git_dir_raw)
            .canonicalize()
            .unwrap_or_else(|_| Self::resolve_git_path(&request.root, &git_dir_raw));
        let common_dir = Self::resolve_git_path(&request.root, &common_raw)
            .canonicalize()
            .unwrap_or_else(|_| Self::resolve_git_path(&request.root, &common_raw));
        let object_format = self
            .query(
                &request.root,
                &["rev-parse", "--show-object-format"],
                &request.budget,
            )
            .unwrap_or_else(|_| "sha1".into());
        let head = self
            .query(&request.root, &["rev-parse", "HEAD"], &request.budget)
            .unwrap_or_else(|_| "unborn".into());
        let symbolic_head = self
            .query(
                &request.root,
                &["symbolic-ref", "--quiet", "--short", "HEAD"],
                &request.budget,
            )
            .ok();
        let index = self.query_bytes(
            &request.root,
            &["ls-files", "--stage", "-z"],
            &request.budget,
        )?;
        let status = self.run(
            &request.root,
            &[
                "status",
                "--porcelain=v2",
                "--branch",
                "--untracked-files=all",
                "-z",
            ],
            &request.budget,
        )?;
        if !status.status.success() {
            return Err(M02Error::GitInspection(
                String::from_utf8_lossy(&status.stderr).trim().to_owned(),
            ));
        }
        let records: Vec<&[u8]> = status
            .stdout
            .split(|byte| *byte == 0)
            .filter(|record| !record.is_empty())
            .collect();
        if records.len() as u64 > request.budget.max_parsed_records {
            return Err(M02Error::ResourceBudgetExceeded(
                "Git parsed records".into(),
            ));
        }
        let tracked: Vec<&[u8]> = records
            .iter()
            .copied()
            .filter(|record| !record.starts_with(b"?"))
            .collect();
        let untracked: Vec<&[u8]> = records
            .iter()
            .copied()
            .filter(|record| record.starts_with(b"? "))
            .collect();
        let tracked_fingerprint = Self::hash_bytes(&tracked.concat());
        let untracked_fingerprint = match request.untracked_policy {
            crate::UntrackedPolicy::ExcludedByPolicy => "excluded-by-policy".into(),
            crate::UntrackedPolicy::NamesOnly | crate::UntrackedPolicy::ContentHashed => {
                Self::hash_bytes(&untracked.concat())
            }
        };
        let submodule_fingerprint = Self::hash_bytes(&index);
        let sparse = self
            .query(
                &request.root,
                &["config", "--local", "--get", "core.sparseCheckout"],
                &request.budget,
            )
            .unwrap_or_else(|_| "false".into());
        let remote_output = self
            .query(
                &request.root,
                &["config", "--local", "--get-regexp", r"^remote\..*\.url$"],
                &request.budget,
            )
            .unwrap_or_default();
        let redacted_remote_hints = remote_output.lines().map(Self::redact_remote).collect();
        let repository = repository_id(&common_dir, &object_format);
        let worktree = (!bare).then(|| worktree_id(&root_path, &git_dir, &head));
        Ok(Some(GitEvidenceV1 {
            provider: "system-git".into(),
            provider_version: "argv-readonly-v1".into(),
            repository_id: repository,
            worktree_id: worktree,
            is_bare: bare,
            object_format,
            head,
            symbolic_head,
            index_fingerprint: Self::hash_bytes(&index),
            tracked_delta_fingerprint: tracked_fingerprint,
            untracked_fingerprint,
            submodule_fingerprint,
            sparse_checkout_fingerprint: Self::hash_bytes(sparse.as_bytes()),
            common_dir,
            git_dir,
            redacted_remote_hints,
            parsed_records: records.len() as u64,
        }))
    }
}

impl SystemGitInspector {
    fn query_bytes(
        &self,
        root: &Path,
        args: &[&str],
        budget: &crate::WorkspaceResourceBudget,
    ) -> Result<Vec<u8>, M02Error> {
        let result = self.run(root, args, budget)?;
        if !result.status.success() {
            return Err(M02Error::GitInspection(
                String::from_utf8_lossy(&result.stderr).trim().to_owned(),
            ));
        }
        Ok(result.stdout)
    }

    pub fn inspect_with_deadline(
        &self,
        request: GitInspectRequest,
    ) -> impl std::future::Future<Output = Result<Option<GitEvidenceV1>, M02Error>> + Send + 'static
    {
        let inspector = self.clone();
        async move {
            let duration =
                std::time::Duration::from_millis(request.budget.max_git_process_duration_ms);
            match tokio::time::timeout(
                duration,
                tokio::task::spawn_blocking(move || inspector.inspect(&request)),
            )
            .await
            {
                Ok(joined) => joined.map_err(|error| M02Error::GitInspection(error.to_string()))?,
                Err(_) => Err(M02Error::ResourceBudgetExceeded(
                    "Git process duration".into(),
                )),
            }
        }
    }
}

pub fn redacted_remote_for_test(value: &str) -> String {
    SystemGitInspector::redact_remote(value)
}
