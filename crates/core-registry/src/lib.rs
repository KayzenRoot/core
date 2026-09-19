//! Deterministic module/capability admission and atomic substitution fabric.

use core_contracts::{
    CapabilityBindingReceipt, CapabilityLease, CapabilityProviderDescriptor, CapabilityRequirement,
    ModuleManifest, ProviderHealth, ProviderOrigin, RuntimeGeneration, SchemaVersion, SemVer,
};
use core_identity::fingerprint;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};
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
}

#[derive(Debug, Default, Clone)]
pub struct ModuleRegistry {
    modules: BTreeMap<String, ModuleManifest>,
}

impl ModuleRegistry {
    pub fn register(&mut self, manifest: ModuleManifest) -> Result<(), RegistryError> {
        if self.modules.contains_key(&manifest.module_id) {
            return Err(RegistryError::DuplicateModule(manifest.module_id));
        }
        for dependency in &manifest.startup_dependencies {
            if !self.modules.contains_key(dependency) && dependency != &manifest.module_id {
                return Err(RegistryError::UnknownDependency(dependency.clone()));
            }
        }
        self.modules.insert(manifest.module_id.clone(), manifest);
        if let Err(error) = self.validate_graph() {
            let id = self.modules.keys().last().cloned();
            if let Some(id) = id {
                self.modules.remove(&id);
            }
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
}

#[derive(Debug, Clone)]
pub struct CapabilityRegistry {
    inner: Arc<RwLock<RegistryState>>,
}

#[derive(Debug, Default)]
struct RegistryState {
    providers: BTreeMap<String, Vec<CapabilityProviderDescriptor>>,
    bindings: BTreeMap<String, Binding>,
    next_generation: u64,
    next_lease: u64,
}

#[derive(Debug, Clone)]
struct Binding {
    provider: CapabilityProviderDescriptor,
    generation: u64,
    active_leases: u64,
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

    pub fn resolve(
        &self,
        requirement: &CapabilityRequirement,
    ) -> Result<CapabilityProviderDescriptor, RegistryError> {
        let state = self.inner.read().expect("registry lock poisoned");
        let candidates = state
            .providers
            .get(&requirement.name)
            .ok_or_else(|| RegistryError::NoProvider(requirement.name.clone()))?;
        let mut eligible: Vec<_> = candidates
            .iter()
            .filter(|provider| {
                provider.contract.compatible_with(requirement.contract)
                    && requirement.required_features.is_subset(&provider.features)
                    && provider.health == ProviderHealth::Healthy
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
            origin_rank(right.origin)
                .cmp(&origin_rank(left.origin))
                .then_with(|| right.quality.cmp(&left.quality))
                .then_with(|| right.trust.cmp(&left.trust))
                .then_with(|| left.provider_id.cmp(&right.provider_id))
        });
        Ok(eligible.remove(0))
    }

    pub fn bind(
        &self,
        requirement: &CapabilityRequirement,
        reason: impl Into<String>,
    ) -> Result<CapabilityBindingReceipt, RegistryError> {
        let provider = self.resolve(requirement)?;
        let mut state = self.inner.write().expect("registry lock poisoned");
        let generation = state.next_generation;
        state.next_generation = state.next_generation.saturating_add(1);
        state.bindings.insert(
            requirement.name.clone(),
            Binding {
                provider: provider.clone(),
                generation,
                active_leases: 0,
            },
        );
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
        let lease_id = format!("lease-{}", state.next_lease);
        state.next_lease = state.next_lease.saturating_add(1);
        let (provider_id, provider_fingerprint, binding_generation) = {
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
            )
        };
        Ok(CapabilityLease {
            schema: SchemaVersion::CURRENT,
            lease_id,
            capability: capability.to_owned(),
            provider_id,
            provider_fingerprint,
            binding_generation,
            boot_epoch,
            expires_at_monotonic_ms: ttl_ms,
            revoked: false,
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
        let state = self.inner.read().expect("registry lock poisoned");
        let binding = state
            .bindings
            .get(&lease.capability)
            .ok_or(RegistryError::StaleLease)?;
        if binding.generation != lease.binding_generation
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
        let binding = state
            .bindings
            .get_mut(&lease.capability)
            .ok_or(RegistryError::StaleLease)?;
        if binding.generation != lease.binding_generation
            || binding.provider.fingerprint != lease.provider_fingerprint
            || lease.revoked
        {
            return Err(RegistryError::StaleLease);
        }
        binding.active_leases = binding.active_leases.saturating_sub(1);
        Ok(())
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

fn origin_rank(origin: ProviderOrigin) -> u8 {
    match origin {
        ProviderOrigin::HiveExternal => 4,
        ProviderOrigin::CoreNative => 3,
        ProviderOrigin::OtherExternal => 2,
        ProviderOrigin::CoreFallback => 1,
    }
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
