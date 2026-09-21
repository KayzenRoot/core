use core_workspace::{hash_file, HashConveyor, WorkspaceResourceBudget};
use std::fs;

#[test]
fn conveyor_reuses_equivalent_proof_without_unbounded_memory() {
    let path = std::env::temp_dir().join(format!("m02-conveyor-{}", std::process::id()));
    fs::write(&path, vec![b'x'; 1024 * 1024]).unwrap();
    let conveyor = HashConveyor::default();
    let first = conveyor
        .hash(&path, &WorkspaceResourceBudget::default())
        .unwrap();
    let second = conveyor
        .hash(&path, &WorkspaceResourceBudget::default())
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(first.byte_len, 1024 * 1024);
    let _ = fs::remove_file(path);
}

#[test]
fn single_file_limit_is_typed() {
    let path = std::env::temp_dir().join(format!("m02-limit-{}", std::process::id()));
    fs::write(&path, vec![b'x'; 32]).unwrap();
    let budget = WorkspaceResourceBudget {
        max_single_file_hash_bytes_before_explicit_policy: 1,
        ..WorkspaceResourceBudget::default()
    };
    assert!(hash_file(&path, &budget).is_err());
    let _ = fs::remove_file(path);
}
