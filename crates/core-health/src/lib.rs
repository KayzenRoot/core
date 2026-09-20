//! Multi-dimensional health, bounded degradation and probe coalescing.

use core_contracts::{
    HealthDimension, HealthSignal, HealthSnapshot, ResourcePressure, SchemaVersion,
};
use core_identity::fingerprint;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;

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
    pressure: ResourcePressure,
}

impl DegradationMatrix {
    pub fn set(&mut self, impact: CapabilityImpact) {
        self.capabilities.insert(impact.capability.clone(), impact);
    }

    pub fn set_pressure(&mut self, pressure: ResourcePressure) {
        self.pressure = pressure;
    }

    pub fn pressure(&self) -> ResourcePressure {
        self.pressure
    }

    pub fn admission_allowed(&self) -> bool {
        self.pressure != ResourcePressure::Critical
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

#[derive(Debug)]
pub struct HealthAggregator {
    signals: BTreeMap<HealthDimension, HealthSignal>,
    previous_fingerprint: Option<String>,
    monotonic_origin: Instant,
    pressure: ResourcePressure,
}

impl Default for HealthAggregator {
    fn default() -> Self {
        Self {
            signals: BTreeMap::new(),
            previous_fingerprint: None,
            monotonic_origin: Instant::now(),
            pressure: ResourcePressure::Normal,
        }
    }
}

impl HealthAggregator {
    pub fn set(&mut self, signal: HealthSignal) {
        self.signals.insert(signal.dimension, signal);
    }

    pub fn set_pressure(&mut self, pressure: ResourcePressure) {
        self.pressure = pressure;
    }

    pub fn pressure(&self) -> ResourcePressure {
        self.pressure
    }

    pub fn now_ms(&self) -> u64 {
        self.monotonic_origin.elapsed().as_millis() as u64
    }

    pub fn can_authorize(&self, dimension: HealthDimension, generation: u64) -> bool {
        let now = self.now_ms();
        self.signals.get(&dimension).is_some_and(|signal| {
            signal.generation == generation
                && signal.state == core_contracts::HealthState::Healthy
                && now.saturating_sub(signal.observed_at_monotonic_ms) <= signal.freshness_window_ms
        })
    }

    pub fn causal_health(&self) -> Option<CausalHealth> {
        let root = self
            .signals
            .values()
            .find(|signal| signal.state != core_contracts::HealthState::Healthy)?;
        let dependents = self
            .signals
            .values()
            .filter(|signal| signal.state != core_contracts::HealthState::Healthy)
            .map(|signal| format!("{:?}", signal.dimension))
            .collect();
        Some(CausalHealth {
            root_code: root.reason_code.clone(),
            dependents,
            derived_reasons: [root.impact.clone()].into_iter().collect(),
        })
    }

    pub fn snapshot(&mut self, generation: u64) -> HealthSnapshot {
        let baseline = fingerprint(&(&self.signals, self.pressure))
            .unwrap_or_else(|_| "identity-error".into());
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
            pressure: self.pressure,
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
    state: Mutex<Option<(T, u64)>>,
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
            *entry.state.lock().expect("probe state poisoned") = Some((result.clone(), 0));
            entry.wake.notify_all();
            result
        } else {
            let mut state = entry.state.lock().expect("probe state poisoned");
            while state.is_none() {
                state = entry.wake.wait(state).expect("probe wait poisoned");
            }
            state.clone().expect("probe result present").0
        }
    }

    pub fn get_or_probe_fresh<F>(&self, key: &str, now_ms: u64, freshness_ms: u64, probe: F) -> T
    where
        F: FnOnce() -> T,
    {
        if let Some(entry) = self.entries.lock().expect("probe lock poisoned").get(key) {
            if let Some((value, observed_at)) =
                entry.state.lock().expect("probe state poisoned").clone()
            {
                if now_ms.saturating_sub(observed_at) <= freshness_ms {
                    return value;
                }
            }
        }
        let result = probe();
        let entry = Arc::new(ProbeEntry {
            state: Mutex::new(Some((result.clone(), now_ms))),
            wake: Condvar::new(),
        });
        self.entries
            .lock()
            .expect("probe lock poisoned")
            .insert(key.to_owned(), entry);
        result
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
            observed_at_monotonic_ms: 0,
            freshness_window_ms: 1_000,
            evidence_fingerprint: "health-ok".into(),
        });
        let first = agg.snapshot(1);
        let second = agg.snapshot(1);
        assert!(!first.delta.is_empty());
        assert!(second.delta.is_empty());
        assert!(delta_reconstruct(&first, &second).is_empty());
    }

    #[test]
    fn stale_health_cannot_authorize_safety_transition() {
        let mut agg = HealthAggregator::default();
        let observed = agg.now_ms();
        agg.set(HealthSignal {
            dimension: HealthDimension::Capability,
            state: HealthState::Healthy,
            reason_code: "provider-ready".into(),
            generation: 7,
            impact: "none".into(),
            observed_at_monotonic_ms: observed,
            freshness_window_ms: 0,
            evidence_fingerprint: "provider-ready".into(),
        });
        std::thread::sleep(std::time::Duration::from_millis(2));
        assert!(!agg.can_authorize(HealthDimension::Capability, 7));
    }

    #[test]
    fn critical_pressure_closes_admission() {
        let mut matrix = DegradationMatrix::default();
        matrix.set_pressure(ResourcePressure::Critical);
        assert!(!matrix.admission_allowed());
        matrix.set_pressure(ResourcePressure::Pressured);
        assert!(matrix.admission_allowed());
    }
}
