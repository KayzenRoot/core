//! Bounded streaming hashing and derived request coalescing.

use crate::{M02Error, WorkspaceResourceBudget};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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
    let file_identity = core_identity::fingerprint(&(
        canonical.to_string_lossy().to_string(),
        metadata.len(),
        metadata.permissions().readonly(),
    ))
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    Ok(HashProof {
        algorithm: "sha256-stream-v1".into(),
        digest,
        byte_len: total,
        canonical_path: canonical,
        file_identity,
    })
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
        let file_identity = core_identity::fingerprint(&(
            canonical.to_string_lossy().to_string(),
            metadata.len(),
            metadata.permissions().readonly(),
        ))
        .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
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

#[derive(Debug, Default)]
pub struct HashConveyor {
    proofs: Mutex<BTreeMap<PathBuf, HashProof>>,
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
        if let Some(proof) = self
            .proofs
            .lock()
            .map_err(|_| M02Error::Cancelled)?
            .get(&canonical)
            .cloned()
        {
            return Ok(proof);
        }
        let proof = hash_file(&canonical, budget)?;
        self.proofs
            .lock()
            .map_err(|_| M02Error::Cancelled)?
            .insert(canonical, proof.clone());
        Ok(proof)
    }

    pub fn clear(&self) -> Result<(), M02Error> {
        self.proofs.lock().map_err(|_| M02Error::Cancelled)?.clear();
        Ok(())
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
}
