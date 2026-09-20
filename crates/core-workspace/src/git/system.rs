//! Hardened system-Git provider. Commands are explicit argv and read-only.

use super::{GitInspectRequest, GitInspector};
use crate::identity::{repository_id, worktree_id};
use crate::{GitEvidenceV1, HashConveyor, M02Error};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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
            .args([
                "--no-pager",
                "--no-optional-locks",
                "-c",
                "core.fsmonitor=false",
                "-c",
                "core.fsmonitorHookVersion=",
                "-c",
                "core.pager=cat",
                "-c",
                "pager.status=false",
                "-c",
                "diff.external=",
                "-c",
                "filter.lfs.process=",
                "-c",
                "filter.lfs.clean=",
                "-c",
                "filter.lfs.smudge=",
            ])
            .args(args)
            .current_dir(root)
            .env_clear()
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", null_config)
            .env("GIT_CONFIG_SYSTEM", null_config)
            .env("GIT_ATTR_NOSYSTEM", "1")
            .env("GIT_PAGER", "cat")
            .env("GIT_EDITOR", ":")
            .env("GIT_ASKPASS", "")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command
            .spawn()
            .map_err(|error| M02Error::GitInspection(format!("spawn git: {error}")))?;
        let stdout_reader = child.stdout.take().map(|pipe| {
            let cap = budget.max_stdout_bytes;
            thread::spawn(move || {
                let mut output = Vec::new();
                pipe.take(cap.saturating_add(1))
                    .read_to_end(&mut output)
                    .map(|_| output)
                    .map_err(|error| format!("read stdout: {error}"))
            })
        });
        let stderr_reader = child.stderr.take().map(|pipe| {
            let cap = budget.max_stderr_bytes;
            thread::spawn(move || {
                let mut output = Vec::new();
                pipe.take(cap.saturating_add(1))
                    .read_to_end(&mut output)
                    .map(|_| output)
                    .map_err(|error| format!("read stderr: {error}"))
            })
        });

        let deadline = Instant::now() + Duration::from_millis(budget.max_git_process_duration_ms);
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if Instant::now() >= deadline => {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = stdout_reader
                        .as_ref()
                        .map(|reader| reader.thread().unpark());
                    let _ = stderr_reader
                        .as_ref()
                        .map(|reader| reader.thread().unpark());
                    if let Some(reader) = stdout_reader {
                        let _ = reader.join();
                    }
                    if let Some(reader) = stderr_reader {
                        let _ = reader.join();
                    }
                    return Err(M02Error::ResourceBudgetExceeded(
                        "Git process duration; child terminated and reaped".into(),
                    ));
                }
                Ok(None) => thread::sleep(Duration::from_millis(2)),
                Err(error) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(M02Error::GitInspection(format!("poll git: {error}")));
                }
            }
        };
        let stdout = stdout_reader
            .map(|reader| {
                reader
                    .join()
                    .map_err(|_| "stdout reader panicked".to_owned())
            })
            .transpose()
            .map_err(M02Error::GitInspection)?
            .transpose()
            .map_err(M02Error::GitInspection)?
            .unwrap_or_default();
        let stderr = stderr_reader
            .map(|reader| {
                reader
                    .join()
                    .map_err(|_| "stderr reader panicked".to_owned())
            })
            .transpose()
            .map_err(M02Error::GitInspection)?
            .transpose()
            .map_err(M02Error::GitInspection)?
            .unwrap_or_default();
        if stdout.len() as u64 > budget.max_stdout_bytes {
            return Err(M02Error::ResourceBudgetExceeded("Git stdout".into()));
        }
        if stderr.len() as u64 > budget.max_stderr_bytes {
            return Err(M02Error::ResourceBudgetExceeded("Git stderr".into()));
        }
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
        self.inspect_with_conveyor(request, None)
    }
}

impl SystemGitInspector {
    pub fn inspect_with_conveyor(
        &self,
        request: &GitInspectRequest,
        conveyor: Option<&HashConveyor>,
    ) -> Result<Option<GitEvidenceV1>, M02Error> {
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
            crate::UntrackedPolicy::NamesOnly => Self::hash_bytes(&untracked.concat()),
            crate::UntrackedPolicy::ContentHashed => {
                let mut content = Vec::new();
                let mut aggregate_bytes = 0_u64;
                for record in &untracked {
                    let path = Self::untracked_path(&request.root, record)?;
                    let proof = match conveyor {
                        Some(conveyor) => conveyor.hash(&path, &request.budget)?,
                        None => crate::hash_file(&path, &request.budget)?,
                    };
                    aggregate_bytes = aggregate_bytes.saturating_add(proof.byte_len);
                    if aggregate_bytes > request.budget.max_aggregate_hash_bytes_per_validation {
                        return Err(M02Error::ResourceBudgetExceeded(
                            "Git untracked aggregate hash bytes".into(),
                        ));
                    }
                    content.push((
                        Self::relative_untracked_path(&request.root, &path)?,
                        proof.digest,
                        proof.byte_len,
                    ));
                }
                content.sort_by(|left, right| left.0.cmp(&right.0));
                core_identity::fingerprint(&content)
                    .map_err(|error| M02Error::InvalidInput(error.to_string()))?
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

    fn untracked_path(root: &Path, record: &[u8]) -> Result<PathBuf, M02Error> {
        let raw = record.get(2..).ok_or_else(|| {
            M02Error::GitInspection("malformed untracked porcelain record".into())
        })?;
        let relative = String::from_utf8(raw.to_vec())
            .map_err(|_| M02Error::AuthorityViolation("untracked path is not UTF-8".into()))?;
        let relative_path = PathBuf::from(relative);
        if relative_path.is_absolute() {
            return Err(M02Error::AuthorityViolation(
                "absolute untracked path denied".into(),
            ));
        }
        let lexical = crate::lexical_normalize(&root.join(&relative_path))?;
        let logical_root = crate::lexical_normalize(root)?;
        if !lexical.starts_with(&logical_root) {
            return Err(M02Error::AuthorityViolation(
                "untracked path escaped source authority".into(),
            ));
        }
        let canonical_root = root
            .canonicalize()
            .map_err(|error| M02Error::Io(error.to_string()))?;
        let canonical = lexical
            .canonicalize()
            .map_err(|error| M02Error::Io(format!("untracked path disappeared: {error}")))?;
        if !canonical.starts_with(&canonical_root) {
            return Err(M02Error::AuthorityViolation(
                "untracked symlink/reparse escape denied".into(),
            ));
        }
        Ok(canonical)
    }

    fn relative_untracked_path(root: &Path, path: &Path) -> Result<String, M02Error> {
        let root = root
            .canonicalize()
            .map_err(|error| M02Error::Io(error.to_string()))?;
        let relative = path
            .strip_prefix(root)
            .map_err(|error| M02Error::AuthorityViolation(error.to_string()))?;
        Ok(crate::normalized_path_string(relative))
    }

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
