//! Bounded canonical repository/worktree graph construction.

use crate::git::{GitInspectRequest, GitInspector, SystemGitInspector};
use crate::identity::{normalized_path_string, repository_id};
use crate::{
    ExternalObjectPolicy, M02Error, RepositoryEdgeKind, RepositoryEdgeV1, RepositoryGraphV1,
    RepositoryNodeKind, RepositoryNodeV1, UntrackedPolicy, WorkspaceResourceBudget,
};
use core_identity::fingerprint;
use std::fs;
use std::path::{Path, PathBuf};

pub fn inspect_repository(
    root: impl Into<PathBuf>,
    policy: UntrackedPolicy,
    budget: WorkspaceResourceBudget,
) -> Result<Option<(crate::GitEvidenceV1, RepositoryGraphV1)>, M02Error> {
    inspect_repository_with_conveyor(root, policy, budget, None)
}

pub fn inspect_repository_with_conveyor(
    root: impl Into<PathBuf>,
    policy: UntrackedPolicy,
    budget: WorkspaceResourceBudget,
    conveyor: Option<&crate::HashConveyor>,
) -> Result<Option<(crate::GitEvidenceV1, RepositoryGraphV1)>, M02Error> {
    let root = root.into();
    let inspector = SystemGitInspector::default();
    let request = GitInspectRequest {
        root: root.clone(),
        untracked_policy: policy,
        budget: budget.clone(),
    };
    let evidence = inspector.inspect_with_conveyor(&request, conveyor)?;
    evidence
        .map(|git| {
            build_graph(&root, &git, &budget, ExternalObjectPolicy::Deny).map(|graph| (git, graph))
        })
        .transpose()
}

pub fn build_graph(
    root: &Path,
    git: &crate::GitEvidenceV1,
    budget: &WorkspaceResourceBudget,
    external_policy: ExternalObjectPolicy,
) -> Result<RepositoryGraphV1, M02Error> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let workspace_id = format!("workspace:{}", normalized_path_string(root));
    nodes.push(node(&workspace_id, RepositoryNodeKind::WorkspaceRoot, root));
    let repository_key = format!("repository:{}", git.repository_id.value);
    nodes.push(node(
        &repository_key,
        RepositoryNodeKind::Repository,
        &git.common_dir,
    ));
    edges.push(edge(
        RepositoryEdgeKind::Contains,
        &workspace_id,
        &repository_key,
    ));
    if let Some(worktree) = &git.worktree_id {
        let worktree_key = format!("worktree:{}", worktree.value);
        nodes.push(node(&worktree_key, RepositoryNodeKind::Worktree, root));
        edges.push(edge(
            RepositoryEdgeKind::CheckoutOf,
            &worktree_key,
            &repository_key,
        ));
        edges.push(edge(
            RepositoryEdgeKind::Contains,
            &workspace_id,
            &worktree_key,
        ));
    }
    let common_key = format!("common-dir:{}", normalized_path_string(&git.common_dir));
    nodes.push(node(
        &common_key,
        RepositoryNodeKind::GitCommonDir,
        &git.common_dir,
    ));
    edges.push(edge(
        RepositoryEdgeKind::UsesCommonDir,
        &repository_key,
        &common_key,
    ));

    let gitmodules = root.join(".gitmodules");
    if gitmodules.is_file() {
        let raw =
            fs::read_to_string(&gitmodules).map_err(|error| M02Error::Io(error.to_string()))?;
        if raw.len() as u64 > budget.max_stdout_bytes {
            return Err(M02Error::ResourceBudgetExceeded(".gitmodules".into()));
        }
        for (index, path) in submodule_paths(&raw).into_iter().enumerate() {
            if index as u64 >= budget.max_parsed_records {
                return Err(M02Error::ResourceBudgetExceeded("submodule records".into()));
            }
            let declaration = format!("submodule:{index}:{}", path.display());
            nodes.push(node(
                &declaration,
                RepositoryNodeKind::SubmoduleDeclaration,
                &path,
            ));
            edges.push(edge(
                RepositoryEdgeKind::DeclaresSubmodule,
                &repository_key,
                &declaration,
            ));
            let materialized = root.join(&path).join(".git").exists();
            if materialized {
                let materialized_key = format!("materialized:{index}:{}", path.display());
                nodes.push(node(
                    &materialized_key,
                    RepositoryNodeKind::SubmoduleMaterialization,
                    &root.join(&path),
                ));
                edges.push(edge(
                    RepositoryEdgeKind::Materializes,
                    &declaration,
                    &materialized_key,
                ));
            }
        }
    }

    for (index, path) in nested_repository_paths(root, budget)?
        .into_iter()
        .enumerate()
    {
        let key = format!("nested:{index}:{}", normalized_path_string(&path));
        let semantic_fingerprint = nested_repository_fingerprint(&path, budget)?;
        nodes.push(RepositoryNodeV1 {
            id: key.clone(),
            kind: RepositoryNodeKind::NestedRepository,
            path,
            semantic_fingerprint,
        });
        edges.push(edge(
            RepositoryEdgeKind::NestedWithin,
            &key,
            &repository_key,
        ));
    }

    let alternates = git
        .common_dir
        .join("objects")
        .join("info")
        .join("alternates");
    if alternates.is_file() {
        let raw =
            fs::read_to_string(&alternates).map_err(|error| M02Error::Io(error.to_string()))?;
        for value in raw.lines().filter(|line| !line.trim().is_empty()) {
            let object_root = PathBuf::from(value.trim());
            if !object_root.starts_with(root)
                && matches!(external_policy, ExternalObjectPolicy::Deny)
            {
                return Err(M02Error::AuthorityViolation(
                    "external Git object store denied".into(),
                ));
            }
            let key = format!("object-store:{}", normalized_path_string(&object_root));
            nodes.push(node(
                &key,
                RepositoryNodeKind::ExternalObjectStore,
                &object_root,
            ));
            edges.push(edge(
                RepositoryEdgeKind::UsesObjectStore,
                &repository_key,
                &key,
            ));
        }
    }

    if budget.max_repository_graph_nodes < nodes.len() as u64 {
        return Err(M02Error::ResourceBudgetExceeded(
            "repository graph nodes".into(),
        ));
    }
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    edges.sort_by(|left, right| {
        (left.kind, &left.source, &left.target).cmp(&(right.kind, &right.source, &right.target))
    });
    let fingerprint = fingerprint(
        &(M02GraphPayload {
            nodes: &nodes,
            edges: &edges,
        }),
    )
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    Ok(RepositoryGraphV1 {
        schema_version: crate::M02_VERSION,
        nodes,
        edges,
        fingerprint,
    })
}

pub fn empty_graph(root: &Path) -> Result<RepositoryGraphV1, M02Error> {
    let id = format!("workspace:{}", normalized_path_string(root));
    let nodes = vec![node(&id, RepositoryNodeKind::WorkspaceRoot, root)];
    let fingerprint = fingerprint(
        &(M02GraphPayload {
            nodes: &nodes,
            edges: &[],
        }),
    )
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    Ok(RepositoryGraphV1 {
        schema_version: crate::M02_VERSION,
        nodes,
        edges: Vec::new(),
        fingerprint,
    })
}

pub fn empty_graph_with_policy(
    root: &Path,
    policy: UntrackedPolicy,
    budget: &WorkspaceResourceBudget,
) -> Result<RepositoryGraphV1, M02Error> {
    let inventory = filesystem_inventory_fingerprint(root, policy, budget)?;
    let nested = nested_repository_paths(root, budget)?;
    let mut nodes = vec![node(
        &format!("workspace:{}", normalized_path_string(root)),
        RepositoryNodeKind::WorkspaceRoot,
        root,
    )];
    let mut edges = Vec::new();
    nodes[0].semantic_fingerprint = fingerprint(&(
        RepositoryNodeKind::WorkspaceRoot,
        normalized_path_string(root),
        &inventory,
    ))
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    let workspace_id = nodes[0].id.clone();
    for (index, path) in nested.into_iter().enumerate() {
        let key = format!("nested:{index}:{}", normalized_path_string(&path));
        nodes.push(RepositoryNodeV1 {
            id: key.clone(),
            kind: RepositoryNodeKind::NestedRepository,
            path: path.clone(),
            semantic_fingerprint: nested_repository_fingerprint(&path, budget)?,
        });
        edges.push(edge(RepositoryEdgeKind::NestedWithin, &key, &workspace_id));
    }
    finalize_graph(nodes, edges)
}

#[derive(serde::Serialize)]
struct M02GraphPayload<'a> {
    nodes: &'a [RepositoryNodeV1],
    edges: &'a [RepositoryEdgeV1],
}

fn finalize_graph(
    mut nodes: Vec<RepositoryNodeV1>,
    mut edges: Vec<RepositoryEdgeV1>,
) -> Result<RepositoryGraphV1, M02Error> {
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    edges.sort_by(|left, right| {
        (left.kind, &left.source, &left.target).cmp(&(right.kind, &right.source, &right.target))
    });
    let fingerprint = fingerprint(
        &(M02GraphPayload {
            nodes: &nodes,
            edges: &edges,
        }),
    )
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    Ok(RepositoryGraphV1 {
        schema_version: crate::M02_VERSION,
        nodes,
        edges,
        fingerprint,
    })
}

fn nested_repository_paths(
    root: &Path,
    budget: &WorkspaceResourceBudget,
) -> Result<Vec<PathBuf>, M02Error> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    let mut visited = 0_u64;
    let mut pending = vec![(root.to_path_buf(), 0_u32)];
    while let Some((directory, depth)) = pending.pop() {
        if depth >= budget.max_recursion_depth {
            return Err(M02Error::ResourceBudgetExceeded(
                "nested repository recursion depth".into(),
            ));
        }
        let mut entries = fs::read_dir(&directory)
            .map_err(|error| M02Error::Io(format!("read nested repository directory: {error}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| M02Error::Io(format!("read nested repository entry: {error}")))?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            if entry.file_name() == ".git" {
                continue;
            }
            visited = visited.saturating_add(1);
            if visited > budget.max_parsed_records {
                return Err(M02Error::ResourceBudgetExceeded(
                    "nested repository records".into(),
                ));
            }
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|error| M02Error::Io(format!("read nested repository type: {error}")))?;
            if !file_type.is_dir() || file_type.is_symlink() {
                continue;
            }
            if path.join(".git").exists() {
                paths.push(path);
            } else {
                pending.push((path, depth + 1));
            }
        }
    }
    Ok(paths)
}

fn nested_repository_fingerprint(
    path: &Path,
    budget: &WorkspaceResourceBudget,
) -> Result<String, M02Error> {
    let inspector = SystemGitInspector::default();
    match inspector.inspect(&GitInspectRequest {
        root: path.to_path_buf(),
        untracked_policy: UntrackedPolicy::ExcludedByPolicy,
        budget: budget.clone(),
    })? {
        Some(evidence) => fingerprint(&(
            &evidence.repository_id,
            &evidence.worktree_id,
            &evidence.head,
            &evidence.index_fingerprint,
            &evidence.tracked_delta_fingerprint,
        )),
        None => fingerprint(&normalized_path_string(path)),
    }
    .map_err(|error| M02Error::InvalidInput(error.to_string()))
}

#[derive(serde::Serialize)]
struct InventoryEntry {
    path: String,
    size: u64,
    proof: String,
}

fn filesystem_inventory_fingerprint(
    root: &Path,
    policy: UntrackedPolicy,
    budget: &WorkspaceResourceBudget,
) -> Result<String, M02Error> {
    if policy == UntrackedPolicy::ExcludedByPolicy {
        return Ok("excluded-by-policy".into());
    }
    let canonical_root = root
        .canonicalize()
        .map_err(|error| M02Error::Io(format!("canonicalize inventory root: {error}")))?;
    let mut pending = vec![(root.to_path_buf(), 0_u32)];
    let mut entries = Vec::new();
    let mut aggregate_bytes = 0_u64;
    while let Some((directory, depth)) = pending.pop() {
        if depth >= budget.max_recursion_depth {
            return Err(M02Error::ResourceBudgetExceeded(
                "workspace inventory recursion depth".into(),
            ));
        }
        let mut children = fs::read_dir(&directory)
            .map_err(|error| M02Error::Io(format!("read workspace inventory: {error}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| M02Error::Io(format!("read workspace inventory entry: {error}")))?;
        children.sort_by_key(|entry| entry.path());
        for child in children {
            if child.file_name() == ".git" {
                continue;
            }
            if entries.len() as u64 >= budget.max_parsed_records {
                return Err(M02Error::ResourceBudgetExceeded(
                    "workspace inventory records".into(),
                ));
            }
            let path = child.path();
            let file_type = child
                .file_type()
                .map_err(|error| M02Error::Io(format!("read workspace inventory type: {error}")))?;
            if file_type.is_dir() && !file_type.is_symlink() {
                entries.push(InventoryEntry {
                    path: normalized_path_string(path.strip_prefix(root).map_err(|error| {
                        M02Error::InvalidInput(format!(
                            "workspace inventory relative path: {error}"
                        ))
                    })?),
                    size: 0,
                    proof: "directory".into(),
                });
                pending.push((path, depth + 1));
                continue;
            }
            let canonical = path.canonicalize().map_err(|error| {
                M02Error::Io(format!("canonicalize workspace inventory entry: {error}"))
            })?;
            if !canonical.starts_with(&canonical_root) {
                return Err(M02Error::AuthorityViolation(
                    "workspace inventory escaped source authority".into(),
                ));
            }
            let metadata = fs::metadata(&canonical).map_err(|error| {
                M02Error::Io(format!("metadata workspace inventory entry: {error}"))
            })?;
            let relative = normalized_path_string(path.strip_prefix(root).map_err(|error| {
                M02Error::InvalidInput(format!("workspace inventory relative path: {error}"))
            })?);
            let proof = match policy {
                UntrackedPolicy::NamesOnly => "names-only".into(),
                UntrackedPolicy::ContentHashed => {
                    aggregate_bytes = aggregate_bytes.saturating_add(metadata.len());
                    if aggregate_bytes > budget.max_aggregate_hash_bytes_per_validation {
                        return Err(M02Error::ResourceBudgetExceeded(
                            "workspace inventory aggregate hash bytes".into(),
                        ));
                    }
                    if metadata.len() > budget.max_single_file_hash_bytes_before_explicit_policy {
                        return Err(M02Error::ResourceBudgetExceeded(
                            "workspace inventory single-file hash bytes".into(),
                        ));
                    }
                    crate::hash_file(&canonical, budget)?.digest
                }
                UntrackedPolicy::ExcludedByPolicy => unreachable!(),
            };
            entries.push(InventoryEntry {
                path: relative,
                size: if policy == UntrackedPolicy::NamesOnly {
                    0
                } else {
                    metadata.len()
                },
                proof,
            });
        }
    }
    fingerprint(&entries).map_err(|error| M02Error::InvalidInput(error.to_string()))
}

fn node(id: &str, kind: RepositoryNodeKind, path: &Path) -> RepositoryNodeV1 {
    let semantic_fingerprint = fingerprint(&(kind, normalized_path_string(path))).unwrap();
    RepositoryNodeV1 {
        id: id.into(),
        kind,
        path: path.to_path_buf(),
        semantic_fingerprint,
    }
}

fn edge(kind: RepositoryEdgeKind, source: &str, target: &str) -> RepositoryEdgeV1 {
    RepositoryEdgeV1 {
        kind,
        source: source.into(),
        target: target.into(),
    }
}

fn submodule_paths(raw: &str) -> Vec<PathBuf> {
    raw.lines()
        .filter_map(|line| line.trim().strip_prefix("path = ").map(PathBuf::from))
        .collect()
}

pub fn repository_id_for_common_dir(common_dir: &Path, object_format: &str) -> crate::RepositoryId {
    repository_id(common_dir, object_format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GitEvidenceV1, RepositoryId};

    fn evidence() -> GitEvidenceV1 {
        GitEvidenceV1 {
            provider: "test".into(),
            provider_version: "1".into(),
            repository_id: RepositoryId::new("repo"),
            worktree_id: Some(crate::WorktreeId::new("tree")),
            is_bare: false,
            object_format: "sha1".into(),
            head: "head".into(),
            symbolic_head: Some("main".into()),
            index_fingerprint: "i".into(),
            tracked_delta_fingerprint: "t".into(),
            untracked_fingerprint: "u".into(),
            submodule_fingerprint: "s".into(),
            sparse_checkout_fingerprint: "sp".into(),
            common_dir: PathBuf::from("C:/repo/.git"),
            git_dir: PathBuf::from("C:/repo/.git"),
            redacted_remote_hints: Vec::new(),
            parsed_records: 0,
        }
    }

    #[test]
    fn graph_fingerprint_is_order_independent() {
        let first = build_graph(
            Path::new("C:/repo"),
            &evidence(),
            &WorkspaceResourceBudget::default(),
            ExternalObjectPolicy::Deny,
        )
        .unwrap();
        let second = build_graph(
            Path::new("C:/repo"),
            &evidence(),
            &WorkspaceResourceBudget::default(),
            ExternalObjectPolicy::Deny,
        )
        .unwrap();
        assert_eq!(first.fingerprint, second.fingerprint);
        assert!(first.nodes.windows(2).all(|pair| pair[0].id <= pair[1].id));
    }
}
