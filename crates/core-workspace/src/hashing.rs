//! Bounded streaming hashing and derived request coalescing.

use crate::{M02Error, WorkspaceResourceBudget};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HashProof {
    pub algorithm: String,
    pub digest: String,
    pub byte_len: u64,
    pub canonical_path: PathBuf,
    pub file_identity: String,
}

pub fn hash_file(path: &Path, budget: &WorkspaceResourceBudget) -> Result<HashProof, M02Error> {
    budget
        .validate()
        .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    let canonical = path
        .canonicalize()
        .map_err(|error| M02Error::Io(error.to_string()))?;
    let metadata =
        std::fs::metadata(&canonical).map_err(|error| M02Error::Io(error.to_string()))?;
    if metadata.len() > budget.max_single_file_hash_bytes_before_explicit_policy {
        return Err(M02Error::ResourceBudgetExceeded(
            "single-file hash bytes".into(),
        ));
    }
    let mut file = File::open(&canonical).map_err(|error| M02Error::Io(error.to_string()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut total = 0_u64;
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| M02Error::Io(error.to_string()))?;
        if count == 0 {
            break;
        }
        total = total.saturating_add(count as u64);
        if total > budget.max_aggregate_hash_bytes_per_validation {
            return Err(M02Error::ResourceBudgetExceeded(
                "aggregate hash bytes".into(),
            ));
        }
        digest.update(&buffer[..count]);
    }
    let digest = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let file_identity = metadata_identity(&canonical, &metadata)?;
    let after = std::fs::metadata(&canonical).map_err(|error| M02Error::Io(error.to_string()))?;
    if metadata_identity(&canonical, &after)? != file_identity {
        return Err(M02Error::StaleHandle(
            "file changed during bounded hash".into(),
        ));
    }
    Ok(HashProof {
        algorithm: "sha256-stream-v1".into(),
        digest,
        byte_len: total,
        canonical_path: canonical,
        file_identity,
    })
}

fn metadata_identity(path: &Path, metadata: &std::fs::Metadata) -> Result<String, M02Error> {
    let modified = metadata
        .modified()
        .ok()
        .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
        .map(|value| (value.as_secs(), value.subsec_nanos()));
    core_identity::fingerprint(&(
        path.to_string_lossy().to_string(),
        metadata.len(),
        metadata.permissions().readonly(),
        modified,
    ))
    .map_err(|error| M02Error::InvalidInput(error.to_string()))
}

pub async fn hash_file_with_deadline(
    path: PathBuf,
    budget: WorkspaceResourceBudget,
) -> Result<HashProof, M02Error> {
    let deadline = std::time::Duration::from_millis(budget.max_revalidation_wall_clock_ms);
    tokio::time::timeout(deadline, async move {
        use tokio::io::AsyncReadExt;
        let canonical = tokio::fs::canonicalize(&path)
            .await
            .map_err(|error| M02Error::Io(error.to_string()))?;
        let metadata = tokio::fs::metadata(&canonical)
            .await
            .map_err(|error| M02Error::Io(error.to_string()))?;
        if metadata.len() > budget.max_single_file_hash_bytes_before_explicit_policy {
            return Err(M02Error::ResourceBudgetExceeded(
                "single-file hash bytes".into(),
            ));
        }
        let mut file = tokio::fs::File::open(&canonical)
            .await
            .map_err(|error| M02Error::Io(error.to_string()))?;
        let mut digest = Sha256::new();
        let mut buffer = vec![0_u8; 64 * 1024];
        let mut total = 0_u64;
        loop {
            let count = file
                .read(&mut buffer)
                .await
                .map_err(|error| M02Error::Io(error.to_string()))?;
            if count == 0 {
                break;
            }
            total = total.saturating_add(count as u64);
            if total > budget.max_aggregate_hash_bytes_per_validation {
                return Err(M02Error::ResourceBudgetExceeded(
                    "aggregate hash bytes".into(),
                ));
            }
            digest.update(&buffer[..count]);
        }
        let digest = digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let file_identity = metadata_identity(&canonical, &metadata)?;
        let after = tokio::fs::metadata(&canonical)
            .await
            .map_err(|error| M02Error::Io(error.to_string()))?;
        if metadata_identity(&canonical, &after)? != file_identity {
            return Err(M02Error::StaleHandle(
                "file changed during bounded hash".into(),
            ));
        }
        Ok(HashProof {
            algorithm: "sha256-stream-v1".into(),
            digest,
            byte_len: total,
            canonical_path: canonical,
            file_identity,
        })
    })
    .await
    .map_err(|_| M02Error::ResourceBudgetExceeded("hash deadline".into()))?
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HashCacheKey {
    canonical_path: PathBuf,
    file_identity: String,
}

#[derive(Debug)]
struct PendingHash {
    result: Mutex<Option<Result<HashProof, String>>>,
    ready: Condvar,
}

#[derive(Debug, Default)]
struct ConveyorState {
    proofs: BTreeMap<HashCacheKey, HashProof>,
    in_flight: BTreeMap<PathBuf, Arc<PendingHash>>,
}

#[derive(Debug, Clone)]
pub struct HashConveyor {
    state: Arc<Mutex<ConveyorState>>,
    operations: Arc<AtomicU64>,
}

impl Default for HashConveyor {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(ConveyorState::default())),
            operations: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl HashConveyor {
    pub fn hash(
        &self,
        path: &Path,
        budget: &WorkspaceResourceBudget,
    ) -> Result<HashProof, M02Error> {
        let canonical = path
            .canonicalize()
            .map_err(|error| M02Error::Io(error.to_string()))?;
        let (pending, leader) = {
            let mut state = self.state.lock().map_err(|_| M02Error::Cancelled)?;
            if let Some(pending) = state.in_flight.get(&canonical) {
                (pending.clone(), false)
            } else {
                let pending = Arc::new(PendingHash {
                    result: Mutex::new(None),
                    ready: Condvar::new(),
                });
                state.in_flight.insert(canonical.clone(), pending.clone());
                (pending, true)
            }
        };

        if !leader {
            let mut result = pending.result.lock().map_err(|_| M02Error::Cancelled)?;
            let deadline =
                Instant::now() + Duration::from_millis(budget.max_revalidation_wall_clock_ms);
            while result.is_none() {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return Err(M02Error::ResourceBudgetExceeded(
                        "hash conveyor wait deadline".into(),
                    ));
                }
                let (next, timed_out) = pending
                    .ready
                    .wait_timeout(result, remaining)
                    .map_err(|_| M02Error::Cancelled)?;
                result = next;
                if timed_out.timed_out() && result.is_none() {
                    return Err(M02Error::ResourceBudgetExceeded(
                        "hash conveyor wait deadline".into(),
                    ));
                }
            }
            return match result.as_ref().expect("pending hash completed") {
                Ok(proof) => Ok(proof.clone()),
                Err(error) => Err(M02Error::Io(error.clone())),
            };
        }

        self.operations.fetch_add(1, Ordering::Relaxed);
        // Always re-read before reusing a prior proof. The cache is keyed by
        // path plus file identity, but metadata alone is not content proof;
        // this detects same-size and timestamp-preserving rewrites safely.
        let result = hash_file(&canonical, budget);
        if let Ok(proof) = &result {
            let mut state = self.state.lock().map_err(|_| M02Error::Cancelled)?;
            let key = HashCacheKey {
                canonical_path: proof.canonical_path.clone(),
                file_identity: proof.file_identity.clone(),
            };
            while state.proofs.len() as u64 >= budget.max_cache_entries {
                let Some(oldest) = state.proofs.keys().next().cloned() else {
                    break;
                };
                state.proofs.remove(&oldest);
            }
            state.proofs.insert(key, proof.clone());
            state.in_flight.remove(&canonical);
        } else if let Ok(mut state) = self.state.lock() {
            state.in_flight.remove(&canonical);
        }

        let waiter_result = result
            .as_ref()
            .map(Clone::clone)
            .map_err(ToString::to_string);
        if let Ok(mut completed) = pending.result.lock() {
            *completed = Some(waiter_result);
            pending.ready.notify_all();
        }
        result
    }

    pub async fn hash_with_deadline(
        &self,
        path: PathBuf,
        budget: WorkspaceResourceBudget,
    ) -> Result<HashProof, M02Error> {
        budget
            .validate()
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        let deadline = Duration::from_millis(budget.max_revalidation_wall_clock_ms);
        let worker = self.clone();
        tokio::time::timeout(
            deadline,
            tokio::task::spawn_blocking(move || worker.hash(&path, &budget)),
        )
        .await
        .map_err(|_| M02Error::ResourceBudgetExceeded("hash conveyor deadline".into()))?
        .map_err(|_error| M02Error::Cancelled)?
    }

    pub fn clear(&self) -> Result<(), M02Error> {
        self.state
            .lock()
            .map_err(|_| M02Error::Cancelled)?
            .proofs
            .clear();
        Ok(())
    }

    pub fn hash_operation_count(&self) -> u64 {
        self.operations.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn timestamp_independent_content_proof_changes_on_content() {
        let path = std::env::temp_dir().join(format!("m02-hash-{}", std::process::id()));
        fs::write(&path, b"first").unwrap();
        let first = hash_file(&path, &WorkspaceResourceBudget::default()).unwrap();
        fs::write(&path, b"second").unwrap();
        let second = hash_file(&path, &WorkspaceResourceBudget::default()).unwrap();
        assert_ne!(first.digest, second.digest);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn conveyor_revalidates_same_path_after_rewrite() {
        let path = std::env::temp_dir().join(format!("m02-hash-conveyor-{}", std::process::id()));
        fs::write(&path, b"first").unwrap();
        let conveyor = HashConveyor::default();
        let first = conveyor
            .hash(&path, &WorkspaceResourceBudget::default())
            .unwrap();
        fs::write(&path, b"second").unwrap();
        let second = conveyor
            .hash(&path, &WorkspaceResourceBudget::default())
            .unwrap();
        assert_ne!(first.digest, second.digest);
        assert_eq!(conveyor.hash_operation_count(), 2);
        let _ = fs::remove_file(path);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn conveyor_deadline_does_not_poison_following_hash() {
        let path = std::env::temp_dir().join(format!("m02-hash-deadline-{}", std::process::id()));
        fs::write(&path, vec![b'x'; 1024 * 1024]).unwrap();
        let conveyor = HashConveyor::default();
        let budget = WorkspaceResourceBudget {
            max_revalidation_wall_clock_ms: 1,
            ..WorkspaceResourceBudget::default()
        };
        let _ = conveyor.hash_with_deadline(path.clone(), budget).await;
        assert!(conveyor
            .hash(&path, &WorkspaceResourceBudget::default())
            .is_ok());
        let _ = fs::remove_file(path);
    }
}
