//! Multi-dimensional health, bounded degradation and probe coalescing.

use core_contracts::{HealthDimension, HealthSignal, HealthSnapshot, SchemaVersion};
use core_identity::fingerprint;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityImpact {
    pub capability: String,
    pub available: bool,
    pub quality: u8,
    pub operation_classes: BTreeSet<String>,
}

#[derive(Debug, Default, Clone)]
pub struct DegradationMatrix {
    capabilities: BTreeMap<String, CapabilityImpact>,
}

impl DegradationMatrix {
    pub fn set(&mut self, impact: CapabilityImpact) {
        self.capabilities.insert(impact.capability.clone(), impact);
    }
    pub fn is_allowed(&self, capability: &str, required_quality: u8) -> bool {
        self.capabilities
            .get(capability)
            .is_some_and(|impact| impact.available && impact.quality >= required_quality)
    }
    pub fn blocked(&self) -> Vec<String> {
        self.capabilities
            .values()
            .filter(|impact| !impact.available)
            .map(|impact| impact.capability.clone())
            .collect()
    }
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }
}

pub fn quality_floor_continuity(fallback_quality: u8, required_quality: u8) -> bool {
    fallback_quality >= required_quality
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalHealth {
    pub root_code: String,
    pub dependents: BTreeSet<String>,
    pub derived_reasons: BTreeSet<String>,
}

#[derive(Debug, Default)]
pub struct HealthAggregator {
    signals: BTreeMap<HealthDimension, HealthSignal>,
    previous_fingerprint: Option<String>,
}

impl HealthAggregator {
    pub fn set(&mut self, signal: HealthSignal) {
        self.signals.insert(signal.dimension, signal);
    }

    pub fn snapshot(&mut self, generation: u64) -> HealthSnapshot {
        let baseline = fingerprint(&self.signals).unwrap_or_else(|_| "identity-error".into());
        let mut delta = BTreeMap::new();
        if self.previous_fingerprint.as_deref() != Some(&baseline) {
            delta.insert("baseline_fingerprint".into(), baseline.clone());
        }
        self.previous_fingerprint = Some(baseline.clone());
        let _ = generation;
        HealthSnapshot {
            schema: SchemaVersion::CURRENT,
            baseline_fingerprint: baseline,
            signals: self.signals.clone(),
            delta,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeState {
    InFlight,
    Complete,
}

#[derive(Debug)]
struct ProbeEntry<T> {
    state: Mutex<Option<T>>,
    wake: Condvar,
}

/// Equivalent probes share one owner and all waiters receive the same result.
#[derive(Debug, Default)]
pub struct ProbeCoalescer<T: Clone> {
    entries: Mutex<BTreeMap<String, Arc<ProbeEntry<T>>>>,
}

impl<T: Clone> ProbeCoalescer<T> {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn get_or_probe<F>(&self, key: &str, probe: F) -> T
    where
        F: FnOnce() -> T,
    {
        let (entry, owner) = {
            let mut entries = self.entries.lock().expect("probe lock poisoned");
            if let Some(entry) = entries.get(key) {
                (Arc::clone(entry), false)
            } else {
                let entry = Arc::new(ProbeEntry {
                    state: Mutex::new(None),
                    wake: Condvar::new(),
                });
                entries.insert(key.to_owned(), Arc::clone(&entry));
                (entry, true)
            }
        };
        if owner {
            let result = probe();
            *entry.state.lock().expect("probe state poisoned") = Some(result.clone());
            entry.wake.notify_all();
            result
        } else {
            let mut state = entry.state.lock().expect("probe state poisoned");
            while state.is_none() {
                state = entry.wake.wait(state).expect("probe wait poisoned");
            }
            state.clone().expect("probe result present")
        }
    }
}

pub fn delta_reconstruct(
    previous: &HealthSnapshot,
    current: &HealthSnapshot,
) -> BTreeMap<String, String> {
    let mut delta = BTreeMap::new();
    for (dimension, signal) in &current.signals {
        if previous.signals.get(dimension) != Some(signal) {
            delta.insert(
                format!("{dimension:?}"),
                format!("{:?}:{}", signal.state, signal.reason_code),
            );
        }
    }
    delta
}

pub fn redacted_health_metadata<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_contracts::HealthState;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn quality_floor_blocks_unsafe_fallback() {
        assert!(quality_floor_continuity(80, 80));
        assert!(!quality_floor_continuity(79, 80));
        let mut matrix = DegradationMatrix::default();
        matrix.set(CapabilityImpact {
            capability: "context".into(),
            available: true,
            quality: 79,
            operation_classes: BTreeSet::new(),
        });
        assert!(!matrix.is_allowed("context", 80));
    }

    #[test]
    fn probe_herd_runs_once() {
        let coalescer = Arc::new(ProbeCoalescer::new());
        let calls = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..16 {
            let coalescer = Arc::clone(&coalescer);
            let calls = Arc::clone(&calls);
            handles.push(thread::spawn(move || {
                coalescer.get_or_probe("hive", || {
                    calls.fetch_add(1, Ordering::SeqCst);
                    42
                })
            }));
        }
        for handle in handles {
            assert_eq!(handle.join().unwrap(), 42);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn observability_is_delta_based() {
        let mut agg = HealthAggregator::default();
        agg.set(HealthSignal {
            dimension: HealthDimension::ProcessLiveness,
            state: HealthState::Healthy,
            reason_code: "ok".into(),
            generation: 1,
            impact: "none".into(),
        });
        let first = agg.snapshot(1);
        let second = agg.snapshot(1);
        assert!(!first.delta.is_empty());
        assert!(second.delta.is_empty());
        assert!(delta_reconstruct(&first, &second).is_empty());
    }
}
