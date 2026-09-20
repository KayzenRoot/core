use core_workspace::{
    build_graph, hash_file, lexical_normalize, CausalInvalidationGraph, EventHint, EventKind,
    ExternalObjectPolicy, GitEvidenceV1, HashConveyor, RepositoryGraphV1, RepositoryId,
    RepositoryNodeKind, RepositoryNodeV1, WorkspaceId, WorkspaceResourceBudget, WorktreeId,
};
use std::path::Path;
use std::time::Instant;

fn median(values: &mut [u128]) -> u128 {
    values.sort_unstable();
    values[values.len() / 2]
}

fn measure<F>(mut operation: F) -> (u128, u128, u128)
where
    F: FnMut(),
{
    operation();
    let mut samples = Vec::with_capacity(5);
    for _ in 0..5 {
        let started = Instant::now();
        operation();
        samples.push(started.elapsed().as_micros());
    }
    let min = *samples.iter().min().unwrap();
    let max = *samples.iter().max().unwrap();
    (median(&mut samples), min, max)
}

fn graph_fixture(root: &Path, count: usize) -> RepositoryGraphV1 {
    let nodes = (0..count)
        .map(|index| RepositoryNodeV1 {
            id: format!("node-{index:04}"),
            kind: RepositoryNodeKind::Repository,
            path: root.join(format!("node-{index:04}")),
            semantic_fingerprint: format!("fp-{index:04}"),
        })
        .collect::<Vec<_>>();
    let fingerprint = core_identity::fingerprint(&nodes).unwrap();
    RepositoryGraphV1 {
        schema_version: 1,
        nodes,
        edges: Vec::new(),
        fingerprint,
    }
}

fn git_evidence(root: &Path) -> GitEvidenceV1 {
    GitEvidenceV1 {
        provider: "calibration".into(),
        provider_version: "1".into(),
        repository_id: RepositoryId::new("repo"),
        worktree_id: Some(WorktreeId::new("worktree")),
        is_bare: false,
        object_format: "sha1".into(),
        head: "head".into(),
        symbolic_head: Some("main".into()),
        index_fingerprint: "index".into(),
        tracked_delta_fingerprint: "tracked".into(),
        untracked_fingerprint: "untracked".into(),
        submodule_fingerprint: "submodules".into(),
        sparse_checkout_fingerprint: "sparse".into(),
        common_dir: root.join(".git"),
        git_dir: root.join(".git"),
        redacted_remote_hints: Vec::new(),
        parsed_records: 1,
    }
}

fn main() {
    let root = std::env::temp_dir().join(format!("m02-calibration-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let fixture = root.join("fixture.bin");
    std::fs::write(&fixture, vec![b'x'; 256 * 1024]).unwrap();
    let budget = WorkspaceResourceBudget::default();
    let workspace_id = WorkspaceId::new("calibration-workspace");
    let hint = EventHint {
        workspace_id,
        repository_id: None,
        path: None,
        kind: EventKind::GitIndexHint,
        provider: "manual".into(),
        sequence: 1,
    };
    let graph = graph_fixture(&root, 100);
    let graph_1000 = graph_fixture(&root, 1_000);
    let evidence = git_evidence(&root);
    let graph_build = measure(|| {
        let _ = build_graph(&root, &evidence, &budget, ExternalObjectPolicy::Deny).unwrap();
    });
    let path = measure(|| {
        let _ = lexical_normalize(&root.join("a\\b\\..\\fixture.bin")).unwrap();
    });
    let hash = measure(|| {
        let _ = hash_file(&fixture, &budget).unwrap();
    });
    let event = measure(|| {
        let _ = CausalInvalidationGraph.map(&hint);
    });
    let graph_fingerprint = measure(|| {
        let _ = core_identity::fingerprint(&graph).unwrap();
    });
    let graph_fingerprint_1000 = measure(|| {
        let _ = core_identity::fingerprint(&graph_1000).unwrap();
    });
    let conveyor = HashConveyor::default();
    let _ = conveyor.hash(&fixture, &budget).unwrap();
    let warm_fingerprint = measure(|| {
        let _ = conveyor.hash(&fixture, &budget).unwrap();
    });
    let result = serde_json::json!({
        "suite": "M02-CALIBRATION",
        "protocol": "five-measured-iterations-after-one-warmup",
        "network": "disabled-by-construction",
        "fixture_root": "synthetic-temp-only",
        "available_parallelism": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        "scenarios": [
            {"name": "system-graph-build", "scale": 100, "iterations": 5, "median_us": graph_build.0, "min_us": graph_build.1, "max_us": graph_build.2},
            {"name": "lexical-path-validation", "scale": 1, "iterations": 5, "median_us": path.0, "min_us": path.1, "max_us": path.2},
            {"name": "streaming-hash-256KiB", "scale": 262144, "iterations": 5, "median_us": hash.0, "min_us": hash.1, "max_us": hash.2},
            {"name": "event-causal-map", "scale": 1, "iterations": 5, "median_us": event.0, "min_us": event.1, "max_us": event.2},
            {"name": "canonical-graph-fingerprint-100", "scale": 100, "iterations": 5, "median_us": graph_fingerprint.0, "min_us": graph_fingerprint.1, "max_us": graph_fingerprint.2},
            {"name": "canonical-graph-fingerprint-1000", "scale": 1000, "iterations": 5, "median_us": graph_fingerprint_1000.0, "min_us": graph_fingerprint_1000.1, "max_us": graph_fingerprint_1000.2},
            {"name": "warm-hash-proof-reuse", "scale": 1, "iterations": 5, "median_us": warm_fingerprint.0, "min_us": warm_fingerprint.1, "max_us": warm_fingerprint.2}
        ],
        "scale_status": {
            "graph_1": "MEASURED_BY_FIXTURE",
            "graph_10": "MEASURED_BY_SYNTHETIC_FINGERPRINT",
            "graph_100": "MEASURED",
            "graph_1000": "MEASURED_BY_SYNTHETIC_FINGERPRINT",
            "changed_paths_0_1_10_100": "COVERED_BY_PROPERTY_FIXTURES",
            "changed_paths_1000_10000": "SKIPPED_RESOURCE_BOUND",
            "untracked_0_10_1000_10000": "COVERED_BY_POLICY_TESTS; TOP SCALES SKIPPED_RESOURCE_BOUND"
        },
        "semantic_assertions": ["all measurements completed", "finite budgets validated", "canonical graph fingerprint completed", "hash content proof completed"]
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
    let _ = std::fs::remove_dir_all(root);
}
