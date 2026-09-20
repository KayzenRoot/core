//! Provider-neutral read-only Git inspection.

mod system;
use crate::{M02Error, UntrackedPolicy, WorkspaceResourceBudget};
use std::path::Path;
pub use system::{redacted_remote_for_test, SystemGitInspector};

#[derive(Debug, Clone)]
pub struct GitInspectRequest {
    pub root: std::path::PathBuf,
    pub untracked_policy: UntrackedPolicy,
    pub budget: WorkspaceResourceBudget,
}

pub trait GitInspector {
    fn inspect(
        &self,
        request: &GitInspectRequest,
    ) -> Result<Option<crate::GitEvidenceV1>, M02Error>;
}

pub fn is_git_candidate(path: &Path) -> bool {
    path.join(".git").exists()
}
