//! Disposable runtime-epoch L1 proof cache.

use crate::{ComponentMask, M02Error, WorkspaceGeneration, WorkspaceId, WorkspaceResourceBudget};
use core_identity::fingerprint;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProofCacheKey {
    pub schema_version: u16,
    pub workspace_id: WorkspaceId,
    pub runtime_epoch: u64,
    pub generation: WorkspaceGeneration,
    pub component_mask: ComponentMask,
    pub authority_generation: u64,
    pub policy_generation: u64,
    pub provider_version: String,
    pub filesystem_semantics_fingerprint: String,
    pub proof_kind: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheDecisionReason {
    HitValidated,
    MissNotFound,
    BypassSecurity,
    BypassUnknownSemantics,
    BypassPolicy,
    InvalidatedEvent,
    InvalidatedGeneration,
    InvalidatedProvider,
    InvalidatedAuthority,
    InvalidatedSecurity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheLookup<V> {
    pub value: Option<V>,
    pub reason: CacheDecisionReason,
}

#[derive(Debug, Clone)]
struct Entry<V> {
    value: V,
    bytes: u64,
    key: ProofCacheKey,
}

#[derive(Debug, Clone)]
pub struct ProofCache<V> {
    entries: BTreeMap<String, Entry<V>>,
    max_entries: u64,
    max_bytes: u64,
    bytes: u64,
}

impl<V: Clone + Serialize> ProofCache<V> {
    pub fn new(budget: &WorkspaceResourceBudget) -> Result<Self, M02Error> {
        budget
            .validate()
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        Ok(Self {
            entries: BTreeMap::new(),
            max_entries: budget.max_cache_entries,
            max_bytes: budget.max_cache_bytes,
            bytes: 0,
        })
    }

    fn key_id(key: &ProofCacheKey) -> String {
        fingerprint(key).expect("cache key is serializable")
    }

    pub fn insert(&mut self, key: ProofCacheKey, value: V) -> Result<(), M02Error> {
        let bytes = serde_json::to_vec(&value)
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?
            .len() as u64;
        if bytes > self.max_bytes {
            return Err(M02Error::ResourceBudgetExceeded("cache entry bytes".into()));
        }
        let id = Self::key_id(&key);
        if let Some(previous) = self.entries.remove(&id) {
            self.bytes = self.bytes.saturating_sub(previous.bytes);
        }
        while self.entries.len() as u64 >= self.max_entries
            || self.bytes.saturating_add(bytes) > self.max_bytes
        {
            let Some(oldest) = self.entries.keys().next().cloned() else {
                break;
            };
            if let Some(previous) = self.entries.remove(&oldest) {
                self.bytes = self.bytes.saturating_sub(previous.bytes);
            }
        }
        self.bytes = self.bytes.saturating_add(bytes);
        self.entries.insert(id, Entry { value, bytes, key });
        Ok(())
    }

    pub fn get(&self, key: &ProofCacheKey) -> CacheLookup<V> {
        let id = Self::key_id(key);
        match self.entries.get(&id) {
            Some(entry) if entry.key == *key => CacheLookup {
                value: Some(entry.value.clone()),
                reason: CacheDecisionReason::HitValidated,
            },
            _ => CacheLookup {
                value: None,
                reason: CacheDecisionReason::MissNotFound,
            },
        }
    }

    pub fn invalidate_generation(&mut self, generation: WorkspaceGeneration) {
        self.entries
            .retain(|_, entry| entry.key.generation == generation);
        self.bytes = self.entries.values().map(|entry| entry.bytes).sum();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn bytes(&self) -> u64 {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(generation: u64) -> ProofCacheKey {
        ProofCacheKey {
            schema_version: 1,
            workspace_id: WorkspaceId::new("w"),
            runtime_epoch: 1,
            generation: WorkspaceGeneration::new(1, generation),
            component_mask: ComponentMask::ALL,
            authority_generation: 1,
            policy_generation: 1,
            provider_version: "v1".into(),
            filesystem_semantics_fingerprint: "fsc".into(),
            proof_kind: "path".into(),
        }
    }

    #[test]
    fn cache_hit_has_provenance_and_generation_invalidation() {
        let mut cache = ProofCache::new(&WorkspaceResourceBudget::default()).unwrap();
        cache.insert(key(1), "proof".to_owned()).unwrap();
        assert_eq!(cache.get(&key(1)).reason, CacheDecisionReason::HitValidated);
        cache.invalidate_generation(WorkspaceGeneration::new(1, 2));
        assert_eq!(cache.len(), 0);
    }
}
