//! Deterministic M02 identity derivation implementation.

use crate::{AuthorityRootId, ProjectBindingId, RepositoryId, WorkspaceId, WorktreeId};
use core_identity::fingerprint;
use serde::Serialize;
use std::path::Path;

pub fn normalized_path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn derive<T: Serialize>(namespace: &str, value: &T) -> String {
    fingerprint(&(namespace, value)).expect("M02 identity input is serializable")
}

pub fn project_binding_id(
    workspace: &WorkspaceId,
    expected_hive_reference: Option<&str>,
) -> ProjectBindingId {
    ProjectBindingId::new(derive(
        "project-binding-v1",
        &(workspace, expected_hive_reference),
    ))
}

pub fn workspace_id(
    canonical_root: &Path,
    physical_identity: &str,
    filesystem_fingerprint: &str,
) -> WorkspaceId {
    WorkspaceId::new(derive(
        "workspace-v1",
        &(
            normalized_path_string(canonical_root),
            physical_identity,
            filesystem_fingerprint,
        ),
    ))
}

pub fn repository_id(common_dir: &Path, object_format: &str) -> RepositoryId {
    RepositoryId::new(derive(
        "repository-v1",
        &(normalized_path_string(common_dir), object_format),
    ))
}

pub fn worktree_id(worktree_root: &Path, git_dir: &Path, head: &str) -> WorktreeId {
    WorktreeId::new(derive(
        "worktree-v1",
        &(
            normalized_path_string(worktree_root),
            normalized_path_string(git_dir),
            head,
        ),
    ))
}

pub fn authority_root_id(
    class: crate::AuthorityClass,
    canonical_root: &Path,
    physical_identity: &str,
) -> AuthorityRootId {
    AuthorityRootId::new(derive(
        "authority-root-v1",
        &(
            class,
            normalized_path_string(canonical_root),
            physical_identity,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_is_not_repository_identity() {
        let one = repository_id(Path::new("C:/repo/.git"), "sha1");
        let two = repository_id(Path::new("C:/repo/.git"), "sha1");
        assert_eq!(one, two);
    }

    #[test]
    fn linked_worktrees_are_distinct_but_repository_is_shared() {
        let repository = repository_id(Path::new("C:/repo/.git"), "sha1");
        let first = worktree_id(
            Path::new("C:/a"),
            Path::new("C:/repo/.git/worktrees/a"),
            "head",
        );
        let second = worktree_id(
            Path::new("C:/b"),
            Path::new("C:/repo/.git/worktrees/b"),
            "head",
        );
        assert_eq!(repository, repository_id(Path::new("C:/repo/.git"), "sha1"));
        assert_ne!(first, second);
    }
}
