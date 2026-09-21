#![no_main]

use core_workspace::{build_graph, ExternalObjectPolicy, GitEvidenceV1, RepositoryId, WorkspaceResourceBudget, WorktreeId};
use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;

fuzz_target!(|data: &[u8]| {
    let digest = core_identity::fingerprint_bytes(data);
    let evidence = GitEvidenceV1 {
        provider: "fuzz".into(), provider_version: "1".into(), repository_id: RepositoryId::new(digest.clone()),
        worktree_id: Some(WorktreeId::new(digest)), is_bare: false, object_format: "sha1".into(),
        head: "head".into(), symbolic_head: None, index_fingerprint: "i".into(), tracked_delta_fingerprint: "t".into(),
        untracked_fingerprint: "u".into(), submodule_fingerprint: "s".into(), sparse_checkout_fingerprint: "sp".into(),
        common_dir: PathBuf::from("fuzz-common"), git_dir: PathBuf::from("fuzz-git"), redacted_remote_hints: Vec::new(), parsed_records: 0,
    };
    let _ = build_graph(&PathBuf::from("fuzz-root"), &evidence, &WorkspaceResourceBudget::default(), ExternalObjectPolicy::Deny);
});
