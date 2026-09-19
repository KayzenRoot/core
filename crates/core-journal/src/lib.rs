//! Append-only, hash-chained runtime-safety journal.

use core_contracts::{
    JournalDurability, JournalRecordKind, RecoveryClassification, RuntimeJournalRecord,
    SchemaVersion,
};
use core_identity::{canonical_bytes, fingerprint_bytes};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
#[cfg(test)]
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JournalError {
    #[error("journal I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("journal record {line} is invalid: {reason}")]
    Invalid { line: usize, reason: String },
    #[error("journal integrity check failed at line {0}")]
    Integrity(usize),
    #[error("journal serialization failed: {0}")]
    Serialization(String),
}

#[derive(Debug)]
pub struct RuntimeJournal {
    path: PathBuf,
    next_sequence: u64,
    last_hash: String,
}

impl RuntimeJournal {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, JournalError> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut journal = Self {
            path,
            next_sequence: 1,
            last_hash: String::new(),
        };
        if journal.path.exists() {
            let records = journal.read_validated()?;
            if let Some(last) = records.last() {
                journal.next_sequence = last.sequence.saturating_add(1);
                journal.last_hash = last.hash.clone();
            }
        }
        Ok(journal)
    }

    pub fn append(
        &mut self,
        boot_epoch: u64,
        kind: JournalRecordKind,
        durability: JournalDurability,
        payload: impl IntoIterator<Item = (String, String)>,
    ) -> Result<RuntimeJournalRecord, JournalError> {
        let mut record = RuntimeJournalRecord {
            schema: SchemaVersion::CURRENT,
            sequence: self.next_sequence,
            boot_epoch,
            kind,
            durability,
            payload: payload.into_iter().collect(),
            previous_hash: self.last_hash.clone(),
            hash: String::new(),
        };
        let bytes =
            canonical_bytes(&record).map_err(|e| JournalError::Serialization(e.to_string()))?;
        record.hash = fingerprint_bytes(&bytes);
        let line =
            serde_json::to_vec(&record).map_err(|e| JournalError::Serialization(e.to_string()))?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(&line)?;
        file.write_all(b"\n")?;
        match durability {
            JournalDurability::MemoryOnlyDiagnostic => {}
            JournalDurability::FlushRequired => file.flush()?,
            JournalDurability::SyncRequired => {
                file.flush()?;
                file.sync_data()?;
            }
        }
        self.next_sequence = self.next_sequence.saturating_add(1);
        self.last_hash = record.hash.clone();
        Ok(record)
    }

    pub fn read_validated(&self) -> Result<Vec<RuntimeJournalRecord>, JournalError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();
        let mut previous_hash = String::new();
        for (index, line) in reader.lines().enumerate() {
            let line_number = index + 1;
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let record: RuntimeJournalRecord =
                serde_json::from_str(&line).map_err(|e| JournalError::Invalid {
                    line: line_number,
                    reason: e.to_string(),
                })?;
            if record.sequence != line_number as u64 || record.previous_hash != previous_hash {
                return Err(JournalError::Integrity(line_number));
            }
            let expected = record.hash.clone();
            let mut basis = record.clone();
            basis.hash.clear();
            let bytes =
                canonical_bytes(&basis).map_err(|e| JournalError::Serialization(e.to_string()))?;
            if fingerprint_bytes(&bytes) != expected {
                return Err(JournalError::Integrity(line_number));
            }
            previous_hash = record.hash.clone();
            records.push(record);
        }
        Ok(records)
    }

    pub fn classify_recovery(&self) -> Result<RecoveryClassification, JournalError> {
        let records = self.read_validated()?;
        let Some(last) = records.last() else {
            return Ok(RecoveryClassification::RecoverableInterruption);
        };
        Ok(match last.kind {
            JournalRecordKind::BootClosed | JournalRecordKind::DrainCompleted => {
                RecoveryClassification::CleanStop
            }
            JournalRecordKind::IncompleteShutdown | JournalRecordKind::ForcedTermination => {
                RecoveryClassification::RecoverableInterruption
            }
            JournalRecordKind::TransitionIntent | JournalRecordKind::WorkerSpawned => {
                RecoveryClassification::AmbiguousEffect
            }
            _ => RecoveryClassification::RecoverableInterruption,
        })
    }

    pub fn compact(&mut self, keep: usize) -> Result<(), JournalError> {
        let records = self.read_validated()?;
        if records.len() <= keep {
            return Ok(());
        }
        let start = records.len() - keep;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        let mut previous_hash = String::new();
        for (index, mut record) in records[start..].iter().cloned().enumerate() {
            record.sequence = index as u64 + 1;
            record.previous_hash = previous_hash.clone();
            record.hash.clear();
            let bytes =
                canonical_bytes(&record).map_err(|e| JournalError::Serialization(e.to_string()))?;
            record.hash = fingerprint_bytes(&bytes);
            let line = serde_json::to_vec(&record)
                .map_err(|e| JournalError::Serialization(e.to_string()))?;
            file.write_all(&line)?;
            file.write_all(b"\n")?;
            previous_hash = record.hash;
        }
        file.flush()?;
        file.sync_data()?;
        let rebuilt = self.read_validated()?;
        self.next_sequence = rebuilt.last().map_or(1, |record| record.sequence + 1);
        self.last_hash = rebuilt
            .last()
            .map_or_else(String::new, |record| record.hash.clone());
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Test-only helper that truncates a tail without hiding the resulting corruption.
    #[cfg(test)]
    fn truncate_tail(&self) -> Result<(), JournalError> {
        let mut file = OpenOptions::new().write(true).open(&self.path)?;
        let len = file.seek(SeekFrom::End(0))?;
        file.set_len(len.saturating_sub(2))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("core-journal-{name}-{}", std::process::id()))
    }

    #[test]
    fn hash_chain_and_recovery_survive_reopen() {
        let path = temp_path("chain");
        let _ = std::fs::remove_file(&path);
        let mut journal = RuntimeJournal::open(&path).unwrap();
        journal
            .append(
                1,
                JournalRecordKind::BootOpened,
                JournalDurability::SyncRequired,
                [],
            )
            .unwrap();
        journal
            .append(
                1,
                JournalRecordKind::BootClosed,
                JournalDurability::SyncRequired,
                [],
            )
            .unwrap();
        assert_eq!(journal.read_validated().unwrap().len(), 2);
        assert_eq!(
            journal.classify_recovery().unwrap(),
            RecoveryClassification::CleanStop
        );
        drop(journal);
        assert_eq!(RuntimeJournal::open(&path).unwrap().next_sequence, 3);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn truncation_fails_closed() {
        let path = temp_path("truncated");
        let _ = std::fs::remove_file(&path);
        let mut journal = RuntimeJournal::open(&path).unwrap();
        journal
            .append(
                1,
                JournalRecordKind::BootOpened,
                JournalDurability::FlushRequired,
                [],
            )
            .unwrap();
        journal.truncate_tail().unwrap();
        assert!(journal.read_validated().is_err());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn bounded_compaction_rebuilds_the_chain() {
        let path = temp_path("compact");
        let _ = std::fs::remove_file(&path);
        let mut journal = RuntimeJournal::open(&path).unwrap();
        for _ in 0..5 {
            journal
                .append(
                    1,
                    JournalRecordKind::GenerationActivated,
                    JournalDurability::FlushRequired,
                    [],
                )
                .unwrap();
        }
        journal.compact(2).unwrap();
        assert_eq!(journal.read_validated().unwrap().len(), 2);
        let _ = std::fs::remove_file(path);
    }
}
