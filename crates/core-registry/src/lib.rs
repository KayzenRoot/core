//! Deterministic module/capability admission and atomic substitution fabric.

use core_contracts::{
    AssuranceClass, CapabilityBindingIdentity, CapabilityBindingReceipt, CapabilityLease,
    CapabilityProviderDescriptor, CapabilityRequirement, ModuleManifest, ProviderHealth,
    ProviderOrigin, RuntimeGeneration, SchemaVersion, SemVer,
};
use core_identity::fingerprint;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("duplicate module: {0}")]
    DuplicateModule(String),
    #[error("module dependency cycle: {0}")]
    DependencyCycle(String),
    #[error("unknown module dependency: {0}")]
    UnknownDependency(String),
    #[error("no eligible provider for capability {0}")]
    NoProvider(String),
    #[error("quality floor cannot be satisfied for capability {0}")]
    QualityFloor(String),
    #[error("lease is stale or revoked")]
    StaleLease,
    #[error("identity error: {0}")]
    Identity(String),
    #[error("invalid module manifest: {0}")]
    InvalidManifest(String),
}

#[derive(Debug, Default, Clone)]
pub struct ModuleRegistry {
    modules: BTreeMap<String, ModuleManifest>,
}

impl ModuleRegistry {
    pub fn register(&mut self, manifest: ModuleManifest) -> Result<(), RegistryError> {
        validate_manifest(&manifest)?;
        if self.modules.contains_key(&manifest.module_id) {
            return Err(RegistryError::DuplicateModule(manifest.module_id));
        }
        for dependency in &manifest.startup_dependencies {
            if !self.modules.contains_key(dependency) && dependency != &manifest.module_id {
                return Err(RegistryError::UnknownDependency(dependency.clone()));
            }
        }
        let module_id = manifest.module_id.clone();
        self.modules.insert(module_id.clone(), manifest);
        if let Err(error) = self.validate_graph() {
            self.modules.remove(&module_id);
            return Err(error);
        }
        Ok(())
    }

    pub fn validate_graph(&self) -> Result<(), RegistryError> {
        fn visit(
            node: &str,
            graph: &BTreeMap<String, ModuleManifest>,
            visiting: &mut BTreeSet<String>,
            visited: &mut BTreeSet<String>,
        ) -> Result<(), RegistryError> {
            if visiting.contains(node) {
                return Err(RegistryError::DependencyCycle(node.to_owned()));
            }
            if visited.contains(node) {
                return Ok(());
            }
            visiting.insert(node.to_owned());
            let manifest = graph
                .get(node)
                .ok_or_else(|| RegistryError::UnknownDependency(node.to_owned()))?;
            for dep in &manifest.startup_dependencies {
                visit(dep, graph, visiting, visited)?;
            }
            visiting.remove(node);
            visited.insert(node.to_owned());
            Ok(())
        }
        let mut visiting = BTreeSet::new();
        let mut visited = BTreeSet::new();
        for node in self.modules.keys() {
            visit(node, &self.modules, &mut visiting, &mut visited)?;
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ModuleManifest> {
        self.modules.get(id)
    }
    pub fn len(&self) -> usize {
        self.modules.len()
    }
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub fn manifests(&self) -> Vec<ModuleManifest> {
        self.modules.values().cloned().collect()
    }

    pub fn ids(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
}

fn validate_manifest(manifest: &ModuleManifest) -> Result<(), RegistryError> {
    if manifest.module_id.trim().is_empty() {
        return Err(RegistryError::InvalidManifest("module_id is empty".into()));
    }
    if manifest.health_contract.trim().is_empty() {
        return Err(RegistryError::InvalidManifest(
            "health_contract is required".into(),
        ));
    }
    if manifest.config_namespace.trim().is_empty() {
        return Err(RegistryError::InvalidManifest(
            "config_namespace is required".into(),
        ));
    }
    if manifest.startup_deadline_ms == 0 || manifest.shutdown_deadline_ms == 0 {
        return Err(RegistryError::InvalidManifest(
            "lifecycle deadlines must be positive".into(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct CapabilityRegistry {
    inner: Arc<RwLock<RegistryState>>,
}

#[derive(Debug)]
struct RegistryState {
    providers: BTreeMap<String, Vec<CapabilityProviderDescriptor>>,
    bindings: BTreeMap<String, Binding>,
    retired_bindings: BTreeMap<(String, u64), Binding>,
    leases: BTreeMap<String, LeaseRecord>,
    next_generation: u64,
    next_lease: u64,
    monotonic_origin: Instant,
}

#[derive(Debug, Clone)]
struct Binding {
    provider: CapabilityProviderDescriptor,
    generation: u64,
    active_leases: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProvenanceRelation {
    pub capability: String,
    pub provider_id: String,
    pub module_id: String,
    pub origin: ProviderOrigin,
    pub binding_generation: u64,
    pub provider_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilityGraphSnapshot {
    pub providers: Vec<CapabilityProviderDescriptor>,
    pub bindings: Vec<CapabilityBindingIdentity>,
    pub provenance: Vec<ProvenanceRelation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubstitutionImpact {
    pub capability: String,
    pub active_leases: u64,
    pub affected_capabilities: BTreeSet<String>,
    pub cache_affinities: BTreeSet<String>,
    pub requires_revalidation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FallbackCapabilityHarness {
    pub provider_id: String,
    pub quality_ceiling: u8,
    pub features: BTreeSet<String>,
    pub provenance: ProviderOrigin,
}

#[derive(Debug, Clone)]
struct LeaseRecord {
    capability: String,
    binding_generation: u64,
    expires_at_monotonic_ms: u64,
    revoked: bool,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self {
            providers: BTreeMap::new(),
            bindings: BTreeMap::new(),
            retired_bindings: BTreeMap::new(),
            leases: BTreeMap::new(),
            next_generation: 0,
            next_lease: 0,
            monotonic_origin: Instant::now(),
        }
    }
}

impl RegistryState {
    fn now_ms(&self) -> u64 {
        self.monotonic_origin
            .elapsed()
            .as_millis()
            .min(u128::from(u64::MAX)) as u64
    }

    fn binding(&self, capability: &str, generation: u64) -> Option<&Binding> {
        self.bindings
            .get(capability)
            .filter(|binding| binding.generation == generation)
            .or_else(|| {
                self.retired_bindings
                    .get(&(capability.to_owned(), generation))
            })
    }

    fn decrement_lease(&mut self, capability: &str, generation: u64) {
        if let Some(binding) = self.bindings.get_mut(capability) {
            if binding.generation == generation {
                binding.active_leases = binding.active_leases.saturating_sub(1);
                return;
            }
        }

        let key = (capability.to_owned(), generation);
        let should_remove = self
            .retired_bindings
            .get_mut(&key)
            .map(|binding| {
                binding.active_leases = binding.active_leases.saturating_sub(1);
                binding.active_leases == 0
            })
            .unwrap_or(false);
        if should_remove {
            self.retired_bindings.remove(&key);
        }
    }

    fn prune_expired(&mut self) {
        let now = self.now_ms();
        let expired: Vec<_> = self
            .leases
            .iter()
            .filter(|(_, lease)| lease.revoked || lease.expires_at_monotonic_ms <= now)
            .map(|(lease_id, lease)| {
                (
                    lease_id.clone(),
                    lease.capability.clone(),
                    lease.binding_generation,
                )
            })
            .collect();
        for (lease_id, capability, generation) in expired {
            self.leases.remove(&lease_id);
            self.decrement_lease(&capability, generation);
        }
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(RegistryState {
                next_generation: 1,
                ..Default::default()
            })),
        }
    }

    pub fn register_provider(
        &self,
        mut provider: CapabilityProviderDescriptor,
    ) -> Result<(), RegistryError> {
        if provider.fingerprint.is_empty() {
            let mut basis = provider.clone();
            basis.fingerprint.clear();
            provider.fingerprint =
                fingerprint(&basis).map_err(|e| RegistryError::Identity(e.to_string()))?;
        }
        let mut state = self.inner.write().expect("registry lock poisoned");
        let entries = state
            .providers
            .entry(provider.capability.clone())
            .or_default();
        if entries
            .iter()
            .any(|item| item.provider_id == provider.provider_id)
        {
            return Err(RegistryError::DuplicateModule(provider.provider_id));
        }
        entries.push(provider);
        entries.sort_by(|left, right| left.provider_id.cmp(&right.provider_id));
        Ok(())
    }

    pub fn set_provider_health(
        &self,
        provider_id: &str,
        health: ProviderHealth,
    ) -> Result<(), RegistryError> {
        let mut state = self.inner.write().expect("registry lock poisoned");
        for providers in state.providers.values_mut() {
            if let Some(provider) = providers
                .iter_mut()
                .find(|item| item.provider_id == provider_id)
            {
                provider.health = health;
                provider.readiness = matches!(health, ProviderHealth::Healthy);
                return Ok(());
            }
        }
        Err(RegistryError::NoProvider(provider_id.to_owned()))
    }

    pub fn quarantine_provider(
        &self,
        provider_id: &str,
        quarantined: bool,
    ) -> Result<(), RegistryError> {
        let mut state = self.inner.write().expect("registry lock poisoned");
        for providers in state.providers.values_mut() {
            if let Some(provider) = providers
                .iter_mut()
                .find(|item| item.provider_id == provider_id)
            {
                provider.quarantined = quarantined;
                return Ok(());
            }
        }
        Err(RegistryError::NoProvider(provider_id.to_owned()))
    }

    pub fn resolve(
        &self,
        requirement: &CapabilityRequirement,
    ) -> Result<CapabilityProviderDescriptor, RegistryError> {
        let state = self.inner.read().expect("registry lock poisoned");
        resolve_from_state(&state, requirement)
    }

    pub fn bind(
        &self,
        requirement: &CapabilityRequirement,
        reason: impl Into<String>,
    ) -> Result<CapabilityBindingReceipt, RegistryError> {
        let mut state = self.inner.write().expect("registry lock poisoned");
        state.prune_expired();
        let provider = resolve_from_state(&state, requirement)?;
        let generation = state.next_generation;
        state.next_generation = state.next_generation.saturating_add(1);
        let previous = state.bindings.insert(
            requirement.name.clone(),
            Binding {
                provider: provider.clone(),
                generation,
                active_leases: 0,
            },
        );
        if let Some(previous) = previous {
            if previous.active_leases > 0 {
                state
                    .retired_bindings
                    .insert((requirement.name.clone(), previous.generation), previous);
            }
        }
        Ok(CapabilityBindingReceipt {
            schema: SchemaVersion::CURRENT,
            capability: requirement.name.clone(),
            provider_id: provider.provider_id,
            provider_fingerprint: provider.fingerprint,
            binding_generation: generation,
            reason: reason.into(),
        })
    }

    pub fn substitute(
        &self,
        requirement: &CapabilityRequirement,
        reason: impl Into<String>,
    ) -> Result<CapabilityBindingReceipt, RegistryError> {
        self.bind(requirement, reason)
    }

    pub fn acquire_lease(
        &self,
        capability: &str,
        boot_epoch: u64,
        ttl_ms: u64,
    ) -> Result<CapabilityLease, RegistryError> {
        let mut state = self.inner.write().expect("registry lock poisoned");
        state.prune_expired();
        let lease_id = format!("lease-{}", state.next_lease);
        state.next_lease = state.next_lease.saturating_add(1);
        let expires_at_monotonic_ms = state.now_ms().saturating_add(ttl_ms);
        let (
            provider_id,
            provider_fingerprint,
            binding_generation,
            policy,
            authority_requirements,
            assurance,
            cache_affinity,
        ) = {
            let binding = state
                .bindings
                .get_mut(capability)
                .ok_or_else(|| RegistryError::NoProvider(capability.to_owned()))?;
            if binding.provider.health != ProviderHealth::Healthy {
                return Err(RegistryError::NoProvider(capability.to_owned()));
            }
            binding.active_leases = binding.active_leases.saturating_add(1);
            (
                binding.provider.provider_id.clone(),
                binding.provider.fingerprint.clone(),
                binding.generation,
                binding.provider.policy.clone(),
                binding.provider.authority_requirements.clone(),
                binding.provider.assurance,
                binding.provider.cache_affinity.clone(),
            )
        };
        state.leases.insert(
            lease_id.clone(),
            LeaseRecord {
                capability: capability.to_owned(),
                binding_generation,
                expires_at_monotonic_ms,
                revoked: false,
            },
        );
        Ok(CapabilityLease {
            schema: SchemaVersion::CURRENT,
            lease_id,
            capability: capability.to_owned(),
            provider_id,
            provider_fingerprint,
            binding_generation,
            boot_epoch,
            expires_at_monotonic_ms,
            revoked: false,
            policy,
            authority_requirements,
            assurance,
            cache_affinity,
        })
    }

    pub fn validate_lease(
        &self,
        lease: &CapabilityLease,
        generation: &RuntimeGeneration,
    ) -> Result<(), RegistryError> {
        if lease.revoked || lease.boot_epoch != generation.boot_epoch {
            return Err(RegistryError::StaleLease);
        }
        let mut state = self.inner.write().expect("registry lock poisoned");
        state.prune_expired();
        let now = state.now_ms();
        let record = state
            .leases
            .get(&lease.lease_id)
            .ok_or(RegistryError::StaleLease)?;
        if record.capability != lease.capability
            || record.binding_generation != lease.binding_generation
            || record.expires_at_monotonic_ms != lease.expires_at_monotonic_ms
            || record.revoked
            || record.expires_at_monotonic_ms <= now
        {
            return Err(RegistryError::StaleLease);
        }
        let binding = state
            .binding(&lease.capability, lease.binding_generation)
            .ok_or(RegistryError::StaleLease)?;
        if binding.provider.provider_id != lease.provider_id
            || binding.provider.fingerprint != lease.provider_fingerprint
        {
            return Err(RegistryError::StaleLease);
        }
        Ok(())
    }

    pub fn active_binding(&self, capability: &str) -> Option<(String, u64)> {
        let state = self.inner.read().expect("registry lock poisoned");
        state
            .bindings
            .get(capability)
            .map(|binding| (binding.provider.provider_id.clone(), binding.generation))
    }

    pub fn release_lease(&self, lease: &CapabilityLease) -> Result<(), RegistryError> {
        let mut state = self.inner.write().expect("registry lock poisoned");
        state.prune_expired();
        let record = state
            .leases
            .get(&lease.lease_id)
            .cloned()
            .ok_or(RegistryError::StaleLease)?;
        let binding = state
            .binding(&lease.capability, lease.binding_generation)
            .ok_or(RegistryError::StaleLease)?;
        if record.capability != lease.capability
            || record.binding_generation != lease.binding_generation
            || record.expires_at_monotonic_ms != lease.expires_at_monotonic_ms
            || binding.provider.provider_id != lease.provider_id
            || binding.provider.fingerprint != lease.provider_fingerprint
            || lease.revoked
        {
            return Err(RegistryError::StaleLease);
        }
        state.leases.remove(&lease.lease_id);
        state.decrement_lease(&lease.capability, lease.binding_generation);
        Ok(())
    }

    pub fn revoke_lease(&self, lease_id: &str) -> Result<(), RegistryError> {
        let mut state = self.inner.write().expect("registry lock poisoned");
        state.prune_expired();
        let record = state
            .leases
            .remove(lease_id)
            .ok_or(RegistryError::StaleLease)?;
        state.decrement_lease(&record.capability, record.binding_generation);
        Ok(())
    }

    pub fn active_lease_count(&self, capability: &str, generation: u64) -> u64 {
        let state = self.inner.read().expect("registry lock poisoned");
        state
            .binding(capability, generation)
            .map(|binding| binding.active_leases)
            .unwrap_or(0)
    }

    pub fn total_active_leases(&self) -> u64 {
        let state = self.inner.read().expect("registry lock poisoned");
        state.leases.len() as u64
    }

    pub fn graph_snapshot(&self) -> CapabilityGraphSnapshot {
        let state = self.inner.read().expect("registry lock poisoned");
        let mut providers = state
            .providers
            .values()
            .flat_map(|items| items.iter().cloned())
            .collect::<Vec<_>>();
        providers.sort_by(|left, right| {
            left.capability
                .cmp(&right.capability)
                .then_with(|| left.provider_id.cmp(&right.provider_id))
        });
        let mut bindings = state
            .bindings
            .iter()
            .map(|(capability, binding)| CapabilityBindingIdentity {
                capability: capability.clone(),
                provider_id: binding.provider.provider_id.clone(),
                provider_fingerprint: binding.provider.fingerprint.clone(),
                provider_generation: binding.provider.activation_generation,
                binding_generation: binding.generation,
                active_leases: binding.active_leases,
            })
            .collect::<Vec<_>>();
        bindings.sort_by(|left, right| left.capability.cmp(&right.capability));
        let provenance = bindings
            .iter()
            .filter_map(|binding| {
                state
                    .bindings
                    .get(&binding.capability)
                    .map(|active| ProvenanceRelation {
                        capability: binding.capability.clone(),
                        provider_id: active.provider.provider_id.clone(),
                        module_id: active.provider.module_id.clone(),
                        origin: active.provider.origin,
                        binding_generation: active.generation,
                        provider_fingerprint: active.provider.fingerprint.clone(),
                    })
            })
            .collect();
        CapabilityGraphSnapshot {
            providers,
            bindings,
            provenance,
        }
    }

    pub fn validate_generation_coherence(
        &self,
        capability: &str,
        generation: u64,
    ) -> Result<(), RegistryError> {
        let state = self.inner.read().expect("registry lock poisoned");
        let binding = state
            .binding(capability, generation)
            .ok_or(RegistryError::StaleLease)?;
        if binding.provider.activation_generation > generation {
            return Err(RegistryError::StaleLease);
        }
        Ok(())
    }

    pub fn substitution_impact_details(&self, capability: &str) -> SubstitutionImpact {
        let state = self.inner.read().expect("registry lock poisoned");
        let Some(binding) = state.bindings.get(capability) else {
            return SubstitutionImpact {
                capability: capability.to_owned(),
                active_leases: 0,
                affected_capabilities: BTreeSet::new(),
                cache_affinities: BTreeSet::new(),
                requires_revalidation: false,
            };
        };
        let mut affected_capabilities = BTreeSet::new();
        let mut cache_affinities = BTreeSet::new();
        let mut pending = vec![capability.to_owned()];
        while let Some(current) = pending.pop() {
            if !affected_capabilities.insert(current.clone()) {
                continue;
            }
            if let Some(current_binding) = state.bindings.get(&current) {
                if !current_binding.provider.cache_affinity.is_empty() {
                    cache_affinities.insert(current_binding.provider.cache_affinity.clone());
                }
                pending.extend(
                    current_binding
                        .provider
                        .dependency_capabilities
                        .iter()
                        .cloned(),
                );
            }
        }
        let requires_revalidation = binding.active_leases > 0 || affected_capabilities.len() > 1;
        SubstitutionImpact {
            capability: capability.to_owned(),
            active_leases: binding.active_leases,
            affected_capabilities,
            cache_affinities,
            requires_revalidation,
        }
    }

    pub fn fallback_harness(&self, capability: &str) -> Option<FallbackCapabilityHarness> {
        let state = self.inner.read().expect("registry lock poisoned");
        state
            .providers
            .get(capability)?
            .iter()
            .find_map(|provider| {
                (provider.origin == ProviderOrigin::CoreFallback).then(|| {
                    FallbackCapabilityHarness {
                        provider_id: provider.provider_id.clone(),
                        quality_ceiling: provider.quality,
                        features: provider.features.clone(),
                        provenance: provider.origin,
                    }
                })
            })
    }

    pub fn substitution_impact(&self, capability: &str) -> BTreeSet<String> {
        let state = self.inner.read().expect("registry lock poisoned");
        if state.bindings.contains_key(capability) {
            [capability.to_owned()].into_iter().collect()
        } else {
            BTreeSet::new()
        }
    }
}

fn resolve_from_state(
    state: &RegistryState,
    requirement: &CapabilityRequirement,
) -> Result<CapabilityProviderDescriptor, RegistryError> {
    let candidates = state
        .providers
        .get(&requirement.name)
        .ok_or_else(|| RegistryError::NoProvider(requirement.name.clone()))?;
    let mut eligible: Vec<_> = candidates
        .iter()
        .filter(|provider| {
            provider.contract.compatible_with(requirement.contract)
                && requirement.required_features.is_subset(&provider.features)
                && requirement
                    .required_authorities
                    .is_subset(&provider.authority_requirements)
                && provider.assurance >= requirement.minimum_assurance
                && provider.trust >= requirement.minimum_trust
                && requirement
                    .preferred_origin
                    .is_none_or(|origin| provider.origin == origin)
                && requirement
                    .preferred_provider_class
                    .as_deref()
                    .is_none_or(|class| provider.provider_class == class)
                && (requirement.policy.is_empty() || provider.policy == requirement.policy)
                && (requirement.cache_affinity.is_empty()
                    || provider.cache_affinity == requirement.cache_affinity)
                && requirement
                    .max_latency_micros
                    .is_none_or(|limit| provider.latency_micros <= limit)
                && requirement
                    .max_cost_milli
                    .is_none_or(|limit| provider.cost_milli <= limit)
                && provider.health == ProviderHealth::Healthy
                && provider.readiness
                && !provider.quarantined
                && provider.quality >= requirement.quality_floor
        })
        .cloned()
        .collect();
    if eligible.is_empty() {
        if candidates.iter().any(|provider| {
            provider.contract.compatible_with(requirement.contract)
                && provider.quality < requirement.quality_floor
        }) {
            return Err(RegistryError::QualityFloor(requirement.name.clone()));
        }
        return Err(RegistryError::NoProvider(requirement.name.clone()));
    }
    eligible.sort_by(|left, right| {
        preferred_rank(right, requirement)
            .cmp(&preferred_rank(left, requirement))
            .then_with(|| origin_rank(right.origin).cmp(&origin_rank(left.origin)))
            .then_with(|| right.quality.cmp(&left.quality))
            .then_with(|| right.trust.cmp(&left.trust))
            .then_with(|| right.performance_score.cmp(&left.performance_score))
            .then_with(|| left.latency_micros.cmp(&right.latency_micros))
            .then_with(|| left.cost_milli.cmp(&right.cost_milli))
            .then_with(|| left.provider_id.cmp(&right.provider_id))
    });
    Ok(eligible.remove(0))
}

fn origin_rank(origin: ProviderOrigin) -> u8 {
    match origin {
        ProviderOrigin::HiveExternal => 4,
        ProviderOrigin::CoreNative => 3,
        ProviderOrigin::OtherExternal => 2,
        ProviderOrigin::CoreFallback => 1,
    }
}

fn preferred_rank(
    provider: &CapabilityProviderDescriptor,
    requirement: &CapabilityRequirement,
) -> u8 {
    let origin = requirement
        .preferred_origin
        .is_some_and(|wanted| wanted == provider.origin);
    let class = requirement
        .preferred_provider_class
        .as_deref()
        .is_some_and(|wanted| wanted == provider.provider_class);
    u8::from(origin) * 2 + u8::from(class)
}

pub fn provider(
    provider_id: &str,
    module_id: &str,
    capability: &str,
    origin: ProviderOrigin,
    quality: u8,
) -> CapabilityProviderDescriptor {
    CapabilityProviderDescriptor {
        provider_id: provider_id.into(),
        module_id: module_id.into(),
        capability: capability.into(),
        contract: SemVer::new(1, 0, 0),
        origin,
        features: BTreeSet::new(),
        quality,
        health: ProviderHealth::Healthy,
        trust: quality,
        activation_generation: 1,
        fingerprint: String::new(),
        policy: String::new(),
        authority_requirements: BTreeSet::new(),
        assurance: AssuranceClass::Standard,
        latency_micros: 0,
        cost_milli: 0,
        performance_score: u32::from(quality),
        provider_class: "default".into(),
        cache_affinity: String::new(),
        dependency_capabilities: BTreeSet::new(),
        readiness: true,
        quarantined: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    fn manifest(id: &str, deps: &[&str]) -> ModuleManifest {
        ModuleManifest {
            module_id: id.into(),
            contract: SemVer::new(1, 0, 0),
            implementation: SemVer::new(1, 0, 0),
            build_identity: id.into(),
            isolation: core_contracts::IsolationClass::InProcessTrusted,
            required: vec![],
            optional: vec![],
            provided: BTreeSet::new(),
            startup_dependencies: deps.iter().map(|x| (*x).to_owned()).collect(),
            critical: false,
            lifecycle_hooks: ["start".into(), "stop".into()].into_iter().collect(),
            health_contract: "health.v1".into(),
            startup_deadline_ms: 1_000,
            shutdown_deadline_ms: 1_000,
            config_namespace: format!("core.{id}"),
            config_schema: SchemaVersion::CURRENT,
            authority_requirements: BTreeSet::new(),
            event_contract_versions: BTreeMap::new(),
        }
    }

    #[test]
    fn dependency_cycle_fails_closed() {
        let mut registry = ModuleRegistry::default();
        registry.register(manifest("a", &[])).unwrap();
        registry.register(manifest("b", &["a"])).unwrap();
        registry
            .modules
            .get_mut("a")
            .unwrap()
            .startup_dependencies
            .insert("b".into());
        assert!(matches!(
            registry.validate_graph(),
            Err(RegistryError::DependencyCycle(_))
        ));
    }

    #[test]
    fn failed_registration_rolls_back_only_inserted_module() {
        let mut registry = ModuleRegistry::default();
        registry.register(manifest("z-valid", &[])).unwrap();
        assert!(matches!(
            registry.register(manifest("a-invalid", &["a-invalid"])),
            Err(RegistryError::DependencyCycle(_))
        ));
        assert!(registry.get("z-valid").is_some());
        assert!(registry.get("a-invalid").is_none());
    }

    #[test]
    fn hive_provider_wins_without_llm() {
        let registry = CapabilityRegistry::new();
        registry
            .register_provider(provider(
                "fallback",
                "fallback",
                "context",
                ProviderOrigin::CoreFallback,
                100,
            ))
            .unwrap();
        registry
            .register_provider(provider(
                "hive",
                "hive",
                "context",
                ProviderOrigin::HiveExternal,
                80,
            ))
            .unwrap();
        let requirement = CapabilityRequirement::new("context", SemVer::new(1, 0, 0));
        assert_eq!(
            registry.bind(&requirement, "test").unwrap().provider_id,
            "hive"
        );
    }

    #[test]
    fn stale_epoch_lease_is_rejected() {
        let registry = CapabilityRegistry::new();
        registry
            .register_provider(provider(
                "native",
                "core",
                "health",
                ProviderOrigin::CoreNative,
                90,
            ))
            .unwrap();
        let req = CapabilityRequirement::new("health", SemVer::new(1, 0, 0));
        registry.bind(&req, "startup").unwrap();
        let lease = registry.acquire_lease("health", 4, 100).unwrap();
        assert!(matches!(
            registry.validate_lease(&lease, &RuntimeGeneration::new(5)),
            Err(RegistryError::StaleLease)
        ));
    }

    #[test]
    fn old_generation_remains_valid_until_lease_release() {
        let registry = CapabilityRegistry::new();
        registry
            .register_provider(provider(
                "old",
                "core",
                "context",
                ProviderOrigin::CoreNative,
                90,
            ))
            .unwrap();
        registry
            .register_provider(provider(
                "new",
                "core",
                "context",
                ProviderOrigin::CoreNative,
                95,
            ))
            .unwrap();
        let req = CapabilityRequirement::new("context", SemVer::new(1, 0, 0));
        let old_binding = registry.bind(&req, "initial").unwrap();
        let lease = registry.acquire_lease("context", 4, 1_000).unwrap();
        let new_binding = registry.substitute(&req, "rotation").unwrap();

        assert_ne!(
            old_binding.binding_generation,
            new_binding.binding_generation
        );
        assert_eq!(registry.active_binding("context").unwrap().0, "new");
        assert_eq!(
            registry.active_lease_count("context", old_binding.binding_generation),
            1
        );
        registry
            .validate_lease(&lease, &RuntimeGeneration::new(4))
            .unwrap();
        registry.release_lease(&lease).unwrap();
        assert_eq!(
            registry.active_lease_count("context", old_binding.binding_generation),
            0
        );
    }

    #[test]
    fn lease_expiration_is_monotonic_and_enforced() {
        let registry = CapabilityRegistry::new();
        registry
            .register_provider(provider(
                "native",
                "core",
                "health",
                ProviderOrigin::CoreNative,
                90,
            ))
            .unwrap();
        let req = CapabilityRequirement::new("health", SemVer::new(1, 0, 0));
        registry.bind(&req, "startup").unwrap();
        let lease = registry.acquire_lease("health", 4, 0).unwrap();
        assert!(matches!(
            registry.validate_lease(&lease, &RuntimeGeneration::new(4)),
            Err(RegistryError::StaleLease)
        ));
    }

    #[test]
    fn acs_applies_policy_authority_and_deterministic_tie_breaks() {
        let registry = CapabilityRegistry::new();
        let mut ineligible = provider(
            "ineligible",
            "core",
            "context",
            ProviderOrigin::CoreNative,
            100,
        );
        ineligible.policy = "restricted".into();
        ineligible.authority_requirements.insert("read".into());
        let mut eligible = provider(
            "eligible",
            "core",
            "context",
            ProviderOrigin::CoreNative,
            90,
        );
        eligible.policy = "standard".into();
        eligible.authority_requirements.insert("read".into());
        eligible.cache_affinity = "context-v1".into();
        registry.register_provider(ineligible).unwrap();
        registry.register_provider(eligible).unwrap();
        let mut requirement = CapabilityRequirement::new("context", SemVer::new(1, 0, 0));
        requirement.policy = "standard".into();
        requirement.required_authorities.insert("read".into());
        requirement.cache_affinity = "context-v1".into();
        assert_eq!(
            registry.resolve(&requirement).unwrap().provider_id,
            "eligible"
        );
    }

    #[test]
    fn concurrent_readers_keep_old_lease_generation_coherent_during_substitution() {
        let registry = Arc::new(CapabilityRegistry::new());
        let old = provider("old", "core", "context", ProviderOrigin::CoreNative, 80);
        let mut new = provider("new", "core", "context", ProviderOrigin::CoreNative, 90);
        new.health = ProviderHealth::Unavailable;
        new.readiness = false;
        registry.register_provider(old.clone()).unwrap();
        registry.register_provider(new.clone()).unwrap();
        let requirement = CapabilityRequirement::new("context", SemVer::new(1, 0, 0));
        let old_binding = registry.bind(&requirement, "initial").unwrap();
        let lease = registry.acquire_lease("context", 9, 100_000).unwrap();
        registry
            .set_provider_health("new", ProviderHealth::Healthy)
            .unwrap();
        let next = registry
            .substitute(&requirement, "provider-change")
            .unwrap();
        assert_ne!(old_binding.binding_generation, next.binding_generation);
        let readers = (0..8)
            .map(|_| {
                let registry = Arc::clone(&registry);
                let lease = lease.clone();
                thread::spawn(move || {
                    for _ in 0..100 {
                        registry
                            .validate_lease(&lease, &RuntimeGeneration::new(9))
                            .unwrap();
                    }
                })
            })
            .collect::<Vec<_>>();
        for reader in readers {
            reader.join().unwrap();
        }
        assert_eq!(lease.provider_id, old.provider_id);
        assert_eq!(
            registry.active_lease_count("context", old_binding.binding_generation),
            1
        );
        registry.release_lease(&lease).unwrap();
        assert_eq!(
            registry.active_lease_count("context", old_binding.binding_generation),
            0
        );
    }

    #[test]
    fn provider_flap_and_generation_coherence_are_bounded() {
        let registry = CapabilityRegistry::new();
        registry
            .register_provider(provider(
                "native",
                "core",
                "health",
                ProviderOrigin::CoreNative,
                90,
            ))
            .unwrap();
        let requirement = CapabilityRequirement::new("health", SemVer::new(1, 0, 0));
        let first = registry.bind(&requirement, "initial").unwrap();
        registry
            .set_provider_health("native", ProviderHealth::Unavailable)
            .unwrap();
        assert!(registry.substitute(&requirement, "flap-down").is_err());
        registry
            .set_provider_health("native", ProviderHealth::Healthy)
            .unwrap();
        let second = registry.substitute(&requirement, "flap-up").unwrap();
        registry
            .validate_generation_coherence("health", second.binding_generation)
            .unwrap();
        assert!(second.binding_generation > first.binding_generation);
    }

    #[test]
    fn sir_calculates_dependency_radius_and_cache_affinity() {
        let registry = CapabilityRegistry::new();
        let mut derived_provider = provider(
            "derived",
            "core",
            "derived.context",
            ProviderOrigin::CoreNative,
            90,
        );
        derived_provider.cache_affinity = "derived-v1".into();
        derived_provider
            .dependency_capabilities
            .insert("context".into());
        registry.register_provider(derived_provider).unwrap();
        registry
            .register_provider(provider(
                "context",
                "core",
                "context",
                ProviderOrigin::CoreNative,
                90,
            ))
            .unwrap();
        registry
            .bind(
                &CapabilityRequirement::new("context", SemVer::new(1, 0, 0)),
                "context",
            )
            .unwrap();
        registry
            .bind(
                &CapabilityRequirement::new("derived.context", SemVer::new(1, 0, 0)),
                "derived",
            )
            .unwrap();
        let impact = registry.substitution_impact_details("derived.context");
        assert!(impact.affected_capabilities.contains("context"));
        assert!(impact.requires_revalidation);
        assert!(impact.cache_affinities.contains("derived-v1"));
    }
}
