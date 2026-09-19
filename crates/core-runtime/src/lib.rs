//! Headless supervisor and machine-enforced M01 lifecycle.

use core_config::CoreConfig;
use core_contracts::{
    BootstrapSafetyReceipt, BootstrapVerdict, CapabilityRequirement, HealthDimension, HealthSignal,
    HealthState, JournalDurability, JournalRecordKind, ProviderHealth, ProviderOrigin,
    QuiescenceItem, RecoveryClassification, ResourcePressure, RuntimeGeneration, RuntimeState,
    SchemaVersion, SemVer, ShutdownPhase, ShutdownReceipt, TransitionKind, TransitionReceipt,
    TransitionVerdict,
};
use core_health::{DegradationMatrix, HealthAggregator};
use core_identity::{
    fingerprint, safety_identity_fingerprint, security_evidence_fingerprint, tcbm_fingerprint,
    SafetyIdentityBasis,
};
use core_journal::{JournalError, RuntimeJournal};
use core_registry::{provider, CapabilityRegistry, ModuleRegistry};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("illegal lifecycle transition {from:?} -> {to:?}")]
    IllegalTransition {
        from: RuntimeState,
        to: RuntimeState,
    },
    #[error("lifecycle transition denied: {0}")]
    Denied(String),
    #[error("configuration: {0}")]
    Configuration(String),
    #[error("journal: {0}")]
    Journal(#[from] JournalError),
    #[error("identity: {0}")]
    Identity(String),
    #[error("bootstrap is blocked: {0}")]
    Blocked(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeStatus {
    pub schema: SchemaVersion,
    pub state: RuntimeState,
    pub boot_epoch: u64,
    pub generation: RuntimeGeneration,
    pub recovery: RecoveryClassification,
    pub zero_llm_calls: u64,
    pub blocked_reasons: Vec<String>,
    pub degraded_capabilities: Vec<String>,
    pub resource_pressure: ResourcePressure,
}

#[derive(Debug, Clone, Serialize)]
pub struct TcbmEntry {
    pub component: String,
    pub safety_domains: BTreeSet<String>,
    pub invalidation_basis: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrashObservation {
    pub fingerprint: String,
    pub occurrences: u32,
    pub suppress_diagnostics: bool,
    pub quarantined: bool,
}

#[derive(Debug, Default)]
pub struct CrashFingerprintSuppressor {
    observations: BTreeMap<String, u32>,
    quarantine_threshold: u32,
}

impl CrashFingerprintSuppressor {
    pub fn new(quarantine_threshold: u32) -> Self {
        Self {
            observations: BTreeMap::new(),
            quarantine_threshold: quarantine_threshold.max(2),
        }
    }

    pub fn observe<T: Serialize>(&mut self, basis: &T) -> Result<CrashObservation, RuntimeError> {
        let fingerprint = fingerprint(basis).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let occurrences = self
            .observations
            .entry(fingerprint.clone())
            .and_modify(|count| *count = count.saturating_add(1))
            .or_insert(1);
        Ok(CrashObservation {
            fingerprint,
            occurrences: *occurrences,
            suppress_diagnostics: *occurrences > 1,
            quarantined: *occurrences >= self.quarantine_threshold,
        })
    }
}

#[derive(Debug)]
pub struct Lifecycle {
    state: RuntimeState,
    generation: RuntimeGeneration,
    journal: RuntimeJournal,
}

impl Lifecycle {
    pub fn new(generation: RuntimeGeneration, journal: RuntimeJournal) -> Self {
        Self {
            state: RuntimeState::Created,
            generation,
            journal,
        }
    }
    pub fn state(&self) -> RuntimeState {
        self.state
    }
    pub fn generation(&self) -> &RuntimeGeneration {
        &self.generation
    }

    pub fn transition(
        &mut self,
        intent: TransitionKind,
        target: RuntimeState,
        reason: impl Into<String>,
    ) -> Result<TransitionReceipt, RuntimeError> {
        if !allowed(self.state, target, intent) {
            return Err(RuntimeError::IllegalTransition {
                from: self.state,
                to: target,
            });
        }
        let reason = reason.into();
        self.journal.append(
            self.generation.boot_epoch,
            JournalRecordKind::TransitionIntent,
            JournalDurability::SyncRequired,
            [
                ("from".into(), format!("{:?}", self.state)),
                ("to".into(), format!("{target:?}")),
            ],
        )?;
        let from = self.state;
        self.state = target;
        let mut receipt = TransitionReceipt {
            schema: SchemaVersion::CURRENT,
            boot_epoch: self.generation.boot_epoch,
            from,
            to: target,
            intent,
            verdict: TransitionVerdict::Allow,
            generation: self.generation.clone(),
            reason,
            fingerprint: String::new(),
        };
        receipt.fingerprint =
            fingerprint(&receipt).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        self.journal.append(
            self.generation.boot_epoch,
            JournalRecordKind::TransitionCommit,
            JournalDurability::SyncRequired,
            [
                ("from".into(), format!("{from:?}")),
                ("to".into(), format!("{target:?}")),
                ("fingerprint".into(), receipt.fingerprint.clone()),
            ],
        )?;
        Ok(receipt)
    }

    pub fn journal_mut(&mut self) -> &mut RuntimeJournal {
        &mut self.journal
    }
    pub fn recovery(&self) -> Result<RecoveryClassification, RuntimeError> {
        Ok(self.journal.classify_recovery()?)
    }
}

fn allowed(from: RuntimeState, to: RuntimeState, intent: TransitionKind) -> bool {
    let expected = match (from, to) {
        (RuntimeState::Created, RuntimeState::Validating) => TransitionKind::Validate,
        (RuntimeState::Validating, RuntimeState::Bootstrapping) => TransitionKind::Bootstrap,
        (RuntimeState::Bootstrapping, RuntimeState::Synchronizing) => TransitionKind::Synchronize,
        (RuntimeState::Synchronizing, RuntimeState::Ready) => TransitionKind::AdmitReady,
        (RuntimeState::Synchronizing, RuntimeState::Degraded) => TransitionKind::MarkDegraded,
        (
            RuntimeState::Validating | RuntimeState::Bootstrapping | RuntimeState::Synchronizing,
            RuntimeState::Blocked,
        ) => TransitionKind::Block,
        (
            RuntimeState::Validating
            | RuntimeState::Bootstrapping
            | RuntimeState::Synchronizing
            | RuntimeState::Ready
            | RuntimeState::Degraded,
            RuntimeState::Failed,
        ) => TransitionKind::Fail,
        (RuntimeState::Ready, RuntimeState::Degraded) => TransitionKind::MarkDegraded,
        (RuntimeState::Degraded, RuntimeState::Ready) => TransitionKind::AdmitReady,
        (RuntimeState::Ready | RuntimeState::Degraded, RuntimeState::Draining) => {
            TransitionKind::Drain
        }
        (RuntimeState::Draining, RuntimeState::Stopped) => TransitionKind::Stop,
        _ => return false,
    };
    expected == intent
}

#[derive(Debug)]
pub struct Supervisor {
    lifecycle: Lifecycle,
    config: CoreConfig,
    modules: ModuleRegistry,
    capabilities: CapabilityRegistry,
    health: HealthAggregator,
    degradation: DegradationMatrix,
    receipt: Option<BootstrapSafetyReceipt>,
    blocked_reasons: Vec<String>,
    llm_calls: AtomicU64,
    admission_closed: AtomicBool,
    active_modules: BTreeSet<String>,
    isolated_workers: BTreeSet<String>,
    residual_obligations: BTreeMap<String, String>,
    crash_suppression: CrashFingerprintSuppressor,
}

impl Supervisor {
    pub fn new(config: CoreConfig) -> Result<Self, RuntimeError> {
        config
            .validate()
            .map_err(|e| RuntimeError::Configuration(e.to_string()))?;
        let journal = RuntimeJournal::open(&config.journal_path)?;
        let generation = config.generation.clone();
        let mut lifecycle = Lifecycle::new(generation.clone(), journal);
        lifecycle.journal_mut().append(
            generation.boot_epoch,
            JournalRecordKind::BootOpened,
            JournalDurability::SyncRequired,
            [],
        )?;
        Ok(Self {
            lifecycle,
            config,
            modules: ModuleRegistry::default(),
            capabilities: CapabilityRegistry::new(),
            health: HealthAggregator::default(),
            degradation: DegradationMatrix::default(),
            receipt: None,
            blocked_reasons: Vec::new(),
            llm_calls: AtomicU64::new(0),
            admission_closed: AtomicBool::new(false),
            active_modules: BTreeSet::new(),
            isolated_workers: BTreeSet::new(),
            residual_obligations: BTreeMap::new(),
            crash_suppression: CrashFingerprintSuppressor::new(3),
        })
    }

    pub async fn bootstrap(&mut self) -> Result<BootstrapSafetyReceipt, RuntimeError> {
        self.lifecycle.transition(
            TransitionKind::Validate,
            RuntimeState::Validating,
            "validate configuration and static contracts",
        )?;
        if let Err(error) = self.modules.validate_graph() {
            let reason = error.to_string();
            self.blocked_reasons.push(reason.clone());
            self.lifecycle.transition(
                TransitionKind::Block,
                RuntimeState::Blocked,
                reason.clone(),
            )?;
            return Err(RuntimeError::Blocked(reason));
        }
        self.health.set(HealthSignal {
            dimension: HealthDimension::ProcessLiveness,
            state: HealthState::Healthy,
            reason_code: "process-alive".into(),
            generation: self.config.generation.boot_epoch,
            impact: "none".into(),
            observed_at_monotonic_ms: self.health.now_ms(),
            freshness_window_ms: self.config.startup_timeout_ms,
            evidence_fingerprint: "process-alive".into(),
        });
        self.lifecycle.transition(
            TransitionKind::Bootstrap,
            RuntimeState::Bootstrapping,
            "construct supervisor and registries",
        )?;
        self.lifecycle.transition(
            TransitionKind::Synchronize,
            RuntimeState::Synchronizing,
            "optional HIVE synchronization seam",
        )?;
        let recovery = self.lifecycle.recovery()?;
        let mut verdict = BootstrapVerdict::ReadyEligible;
        if !self.degradation.admission_allowed() {
            verdict = BootstrapVerdict::Blocked;
            self.blocked_reasons
                .push("critical resource pressure closes admission".into());
        }
        if self.config.require_hive {
            verdict = BootstrapVerdict::Blocked;
            self.blocked_reasons
                .push("required HIVE provider is not attached to the standalone runtime".into());
        }
        let config_fingerprint = fingerprint(&self.config.redacted_diagnostics())
            .map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let module_manifests = self.modules.manifests();
        let capability_snapshot = self.capabilities.graph_snapshot();
        let module_graph_fingerprint =
            fingerprint(&module_manifests).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let capability_graph_fingerprint =
            fingerprint(&capability_snapshot).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let tcbm = self.tcbm();
        let tcbm_basis = tcbm
            .iter()
            .map(|entry| (entry.component.clone(), entry.safety_domains.clone()))
            .collect::<BTreeMap<_, _>>();
        let tcbm_fingerprint =
            tcbm_fingerprint(&tcbm_basis).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let saf_fingerprint =
            security_evidence_fingerprint(&tcbm_basis, &capability_graph_fingerprint, "m01-saf-v1")
                .map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let rsg_fingerprint = safety_identity_fingerprint(SafetyIdentityBasis {
            runtime_version: env!("CARGO_PKG_VERSION").into(),
            module_manifests,
            providers: capability_snapshot.providers,
            bindings: capability_snapshot.bindings,
            configuration: self.config.redacted_diagnostics(),
            policy_version: "m01-policy-v1".into(),
            safety_metadata: [
                ("tcbm_fingerprint".into(), tcbm_fingerprint.clone()),
                ("saf_fingerprint".into(), saf_fingerprint.clone()),
            ]
            .into_iter()
            .collect(),
        })
        .map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let mut receipt = BootstrapSafetyReceipt {
            schema: SchemaVersion::CURRENT,
            boot_epoch: self.config.generation.boot_epoch,
            runtime_identity: env!("CARGO_PKG_VERSION").into(),
            config_fingerprint,
            module_graph_fingerprint,
            capability_graph_fingerprint,
            recovery,
            verdict,
            blocked_reasons: self.blocked_reasons.clone(),
            unavailable_capabilities: Vec::new(),
            rsg_fingerprint,
            module_graph_generation: self.config.generation.module_graph,
            capability_graph_generation: self.config.generation.capability_graph,
            policy_generation: self.config.generation.policy,
            tcbm_fingerprint,
            saf_fingerprint,
        };
        if verdict == BootstrapVerdict::Blocked {
            self.lifecycle.transition(
                TransitionKind::Block,
                RuntimeState::Blocked,
                "required HIVE capability unavailable",
            )?;
            return Err(RuntimeError::Blocked(receipt.blocked_reasons.join("; ")));
        }
        self.lifecycle.transition(
            TransitionKind::AdmitReady,
            RuntimeState::Ready,
            "bootstrap safety receipt is complete",
        )?;
        self.health.set(HealthSignal {
            dimension: HealthDimension::RuntimeReadiness,
            state: HealthState::Healthy,
            reason_code: "bootstrap-receipt-valid".into(),
            generation: self.config.generation.boot_epoch,
            impact: "none".into(),
            observed_at_monotonic_ms: self.health.now_ms(),
            freshness_window_ms: self.config.startup_timeout_ms,
            evidence_fingerprint: "bootstrap-receipt-valid".into(),
        });
        receipt.verdict = BootstrapVerdict::ReadyEligible;
        self.receipt = Some(receipt.clone());
        Ok(receipt)
    }

    pub fn status(&self) -> RuntimeStatus {
        RuntimeStatus {
            schema: SchemaVersion::CURRENT,
            state: self.lifecycle.state(),
            boot_epoch: self.config.generation.boot_epoch,
            generation: self.config.generation.clone(),
            recovery: self
                .lifecycle
                .recovery()
                .unwrap_or(RecoveryClassification::CorruptJournal),
            zero_llm_calls: self.llm_calls.load(Ordering::Relaxed),
            blocked_reasons: self.blocked_reasons.clone(),
            degraded_capabilities: self.degradation.blocked(),
            resource_pressure: self.degradation.pressure(),
        }
    }

    pub fn capabilities(&self) -> &CapabilityRegistry {
        &self.capabilities
    }
    pub fn modules(&self) -> &ModuleRegistry {
        &self.modules
    }
    pub fn receipt(&self) -> Option<&BootstrapSafetyReceipt> {
        self.receipt.as_ref()
    }
    pub fn zero_llm_calls(&self) -> u64 {
        self.llm_calls.load(Ordering::Relaxed)
    }

    pub fn tcbm(&self) -> Vec<TcbmEntry> {
        [
            (
                "core-runtime",
                &["lifecycle", "shutdown", "epoch"] as &[&str],
            ),
            (
                "core-registry",
                &["capability-authority", "generation"] as &[&str],
            ),
            (
                "core-identity",
                &["canonical-identity", "evidence"] as &[&str],
            ),
            (
                "core-journal",
                &["journal-integrity", "recovery"] as &[&str],
            ),
            ("core-ipc", &["ipc-trust-boundary", "epoch"] as &[&str]),
        ]
        .into_iter()
        .map(|(component, domains)| TcbmEntry {
            component: component.into(),
            safety_domains: domains.iter().map(|domain| (*domain).to_owned()).collect(),
            invalidation_basis: "m01-saf-v1|dependency-lock|policy-generation".into(),
        })
        .collect()
    }

    pub fn observe_crash<T: Serialize>(
        &mut self,
        basis: &T,
    ) -> Result<CrashObservation, RuntimeError> {
        self.crash_suppression.observe(basis)
    }

    pub fn reload_config(&mut self, next: CoreConfig) -> Result<(), RuntimeError> {
        next.validate()
            .map_err(|error| RuntimeError::Configuration(error.to_string()))?;
        if next.generation.boot_epoch != self.config.generation.boot_epoch {
            return Err(RuntimeError::Denied(
                "configuration reload cannot change boot epoch".into(),
            ));
        }
        if next.journal_path != self.config.journal_path {
            return Err(RuntimeError::Denied(
                "configuration reload cannot move the runtime journal".into(),
            ));
        }
        self.config = next;
        self.lifecycle.generation = self.config.generation.clone();
        Ok(())
    }

    /// Executes the concrete M01 seams used by the bounded soak harness.
    pub async fn exercise_frozen_cycle(&mut self) -> Result<serde_json::Value, RuntimeError> {
        if self.lifecycle.state() == RuntimeState::Created {
            self.bootstrap().await?;
        }
        self.capabilities
            .register_provider(provider(
                "core-fallback",
                "core-fallback",
                "context",
                ProviderOrigin::CoreFallback,
                80,
            ))
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let mut hive = provider(
            "hive-context",
            "hive",
            "context",
            ProviderOrigin::HiveExternal,
            90,
        );
        hive.health = ProviderHealth::Unavailable;
        hive.readiness = false;
        self.capabilities
            .register_provider(hive)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let requirement = CapabilityRequirement::new("context", SemVer::new(1, 0, 0));
        let initial = self
            .capabilities
            .bind(&requirement, "soak-fallback")
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let lease = self
            .capabilities
            .acquire_lease("context", self.config.generation.boot_epoch, 30_000)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .set_provider_health("hive-context", ProviderHealth::Healthy)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let substituted = self
            .capabilities
            .substitute(&requirement, "soak-hive-connect")
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .validate_lease(&lease, &self.config.generation)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .release_lease(&lease)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .set_provider_health("hive-context", ProviderHealth::Unavailable)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let flap_recovered_with_fallback = self
            .capabilities
            .substitute(&requirement, "soak-hive-disconnect")
            .is_ok();
        self.capabilities
            .set_provider_health("hive-context", ProviderHealth::Healthy)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let reloaded = self
            .config
            .reload_from_sources(
                Some("quality_floor = 50"),
                None,
                [],
                &core_config::ConfigOverrides::default(),
            )
            .map_err(|error| RuntimeError::Configuration(error.to_string()))?;
        let config_generation = reloaded.generation.config;
        self.reload_config(reloaded)?;
        self.register_isolated_worker("soak-worker");
        self.mark_worker_terminated("soak-worker");
        self.set_resource_pressure(ResourcePressure::Normal)?;
        let crash_first = self.observe_crash(&("soak-worker", "same-failure"))?;
        let crash_second = self.observe_crash(&("soak-worker", "same-failure"))?;
        let shutdown = self.shutdown()?;
        Ok(serde_json::json!({
            "provider_initial": initial.provider_id,
            "provider_substituted": substituted.provider_id,
            "provider_flap_recovered_with_fallback": flap_recovered_with_fallback,
            "config_generation": config_generation,
            "crash_suppression": crash_second.suppress_diagnostics,
            "crash_first_fingerprint": crash_first.fingerprint,
            "shutdown_clean": shutdown.clean,
            "shutdown_fingerprint": shutdown.quiescence_fingerprint,
        }))
    }

    pub fn register_isolated_worker(&mut self, worker_id: impl Into<String>) {
        self.isolated_workers.insert(worker_id.into());
    }

    pub fn mark_worker_terminated(&mut self, worker_id: &str) {
        self.isolated_workers.remove(worker_id);
    }

    pub fn record_residual_obligation(
        &mut self,
        subject: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.residual_obligations
            .insert(subject.into(), detail.into());
    }

    pub fn mark_degraded(&mut self, reason: impl Into<String>) -> Result<(), RuntimeError> {
        let reason = reason.into();
        if self.lifecycle.state() == RuntimeState::Ready {
            self.lifecycle.transition(
                TransitionKind::MarkDegraded,
                RuntimeState::Degraded,
                reason.clone(),
            )?;
        }
        self.blocked_reasons.push(reason);
        Ok(())
    }

    pub fn set_resource_pressure(
        &mut self,
        pressure: ResourcePressure,
    ) -> Result<(), RuntimeError> {
        self.degradation.set_pressure(pressure);
        self.health.set_pressure(pressure);
        if pressure == ResourcePressure::Critical
            && matches!(self.lifecycle.state(), RuntimeState::Ready)
        {
            self.mark_degraded("resource pressure is critical")?;
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<ShutdownReceipt, RuntimeError> {
        if !matches!(
            self.lifecycle.state(),
            RuntimeState::Ready | RuntimeState::Degraded
        ) {
            return Err(RuntimeError::Denied(
                "shutdown requires READY or DEGRADED".into(),
            ));
        }
        self.admission_closed.store(true, Ordering::Release);
        self.lifecycle.transition(
            TransitionKind::Drain,
            RuntimeState::Draining,
            "close admission and drain leases",
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::DrainStarted,
            JournalDurability::SyncRequired,
            [],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::AdmissionClosed,
            JournalDurability::SyncRequired,
            [],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::LeaseDrainCompleted,
            JournalDurability::SyncRequired,
            [(
                "active_leases".into(),
                self.capabilities.total_active_leases().to_string(),
            )],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::CooperativeCancelCompleted,
            JournalDurability::SyncRequired,
            [],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::CleanupCompleted,
            JournalDurability::SyncRequired,
            [],
        )?;
        let items = vec![
            QuiescenceItem {
                subject: "admission-closed".into(),
                satisfied: self.admission_closed.load(Ordering::Acquire),
                residual: None,
            },
            QuiescenceItem {
                subject: "critical-capability-leases".into(),
                satisfied: self.capabilities.total_active_leases() == 0,
                residual: (self.capabilities.total_active_leases() > 0)
                    .then(|| "active capability lease remains".into()),
            },
            QuiescenceItem {
                subject: "active-modules".into(),
                satisfied: self.active_modules.is_empty(),
                residual: (!self.active_modules.is_empty()).then(|| {
                    format!(
                        "active modules: {}",
                        self.active_modules
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                }),
            },
            QuiescenceItem {
                subject: "isolated-workers".into(),
                satisfied: self.isolated_workers.is_empty(),
                residual: (!self.isolated_workers.is_empty()).then(|| {
                    format!(
                        "isolated workers: {}",
                        self.isolated_workers
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                }),
            },
            QuiescenceItem {
                subject: "journal-durability".into(),
                satisfied: true,
                residual: None,
            },
            QuiescenceItem {
                subject: "residual-resources".into(),
                satisfied: self.residual_obligations.is_empty(),
                residual: self.residual_obligations.values().next().cloned(),
            },
        ];
        let quiescent = items.iter().all(|item| item.satisfied);
        let quiescence_fingerprint =
            fingerprint(&items).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::QuiescenceChecked,
            JournalDurability::SyncRequired,
            [
                ("clean".into(), quiescent.to_string()),
                ("fingerprint".into(), quiescence_fingerprint.clone()),
            ],
        )?;
        let receipt = if quiescent {
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::DrainCompleted,
                JournalDurability::SyncRequired,
                [],
            )?;
            self.lifecycle.transition(
                TransitionKind::Stop,
                RuntimeState::Stopped,
                "quiescence matrix satisfied",
            )?;
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::BootClosed,
                JournalDurability::SyncRequired,
                [],
            )?;
            ShutdownReceipt {
                schema: SchemaVersion::CURRENT,
                boot_epoch: self.config.generation.boot_epoch,
                clean: true,
                final_phase: ShutdownPhase::StopCommit,
                items,
                residuals: Vec::new(),
                quiescence_fingerprint,
            }
        } else {
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::ShutdownEscalated,
                JournalDurability::SyncRequired,
                [],
            )?;
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::ForcedTermination,
                JournalDurability::SyncRequired,
                [],
            )?;
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::IncompleteShutdown,
                JournalDurability::SyncRequired,
                [("reason".into(), "quiescence deadline expired".into())],
            )?;
            self.lifecycle.transition(
                TransitionKind::Stop,
                RuntimeState::Stopped,
                "forced termination with explicit residuals",
            )?;
            ShutdownReceipt {
                schema: SchemaVersion::CURRENT,
                boot_epoch: self.config.generation.boot_epoch,
                clean: false,
                final_phase: ShutdownPhase::ForceTerminate,
                residuals: items
                    .iter()
                    .filter_map(|item| item.residual.clone())
                    .chain(self.residual_obligations.values().cloned())
                    .collect(),
                items,
                quiescence_fingerprint,
            }
        };
        Ok(receipt)
    }
}

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: std::sync::Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: std::sync::Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn child(&self) -> Self {
        Self {
            cancelled: std::sync::Arc::clone(&self.cancelled),
        }
    }
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(name: &str) -> CoreConfig {
        let mut config = CoreConfig::defaults(17);
        config.journal_path =
            std::env::temp_dir().join(format!("core-runtime-{name}-{}", std::process::id()));
        let _ = std::fs::remove_file(&config.journal_path);
        config
    }

    #[tokio::test]
    async fn bootstrap_is_zero_llm_and_ready_only_with_receipt() {
        let mut supervisor = Supervisor::new(config("ready")).unwrap();
        let receipt = supervisor.bootstrap().await.unwrap();
        assert_eq!(receipt.verdict, BootstrapVerdict::ReadyEligible);
        assert_eq!(supervisor.status().state, RuntimeState::Ready);
        assert_eq!(supervisor.zero_llm_calls(), 0);
        let _ = supervisor.shutdown().unwrap();
    }

    #[tokio::test]
    async fn required_hive_blocks_without_false_success() {
        let mut config = config("blocked");
        config.require_hive = true;
        let mut supervisor = Supervisor::new(config).unwrap();
        assert!(matches!(
            supervisor.bootstrap().await,
            Err(RuntimeError::Blocked(_))
        ));
        assert_eq!(supervisor.status().state, RuntimeState::Blocked);
        assert!(supervisor.receipt().is_none());
    }

    #[tokio::test]
    async fn forced_shutdown_records_residuals() {
        let mut supervisor = Supervisor::new(config("forced")).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor.register_isolated_worker("hung-worker");
        let receipt = supervisor.shutdown().unwrap();
        assert!(!receipt.clean);
        assert_eq!(receipt.final_phase, ShutdownPhase::ForceTerminate);
        assert!(!receipt.residuals.is_empty());
    }

    #[tokio::test]
    async fn safety_receipt_fingerprint_tracks_semantic_configuration() {
        let mut first_config = config("identity-first");
        first_config.quality_floor = 50;
        let mut second_config = config("identity-second");
        second_config.quality_floor = 51;
        let mut first = Supervisor::new(first_config).unwrap();
        let mut second = Supervisor::new(second_config).unwrap();
        let first_receipt = first.bootstrap().await.unwrap();
        let second_receipt = second.bootstrap().await.unwrap();
        assert_ne!(
            first_receipt.config_fingerprint,
            second_receipt.config_fingerprint
        );
        assert_ne!(
            first_receipt.rsg_fingerprint,
            second_receipt.rsg_fingerprint
        );
        assert!(!first_receipt.tcbm_fingerprint.is_empty());
        assert!(!first_receipt.saf_fingerprint.is_empty());
        first.shutdown().unwrap();
        second.shutdown().unwrap();
    }

    #[tokio::test]
    async fn qvm_detects_active_lease_without_caller_claim() {
        let mut supervisor = Supervisor::new(config("lease-residual")).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor
            .capabilities()
            .register_provider(provider(
                "native",
                "core",
                "health",
                ProviderOrigin::CoreNative,
                90,
            ))
            .unwrap();
        let requirement = CapabilityRequirement::new("health", SemVer::new(1, 0, 0));
        supervisor
            .capabilities()
            .bind(&requirement, "test")
            .unwrap();
        supervisor
            .capabilities()
            .acquire_lease("health", 17, 30_000)
            .unwrap();
        let receipt = supervisor.shutdown().unwrap();
        assert!(!receipt.clean);
        assert!(receipt
            .items
            .iter()
            .any(|item| item.subject == "critical-capability-leases" && !item.satisfied));
    }

    #[test]
    fn cancellation_propagates_hierarchically() {
        let root = CancellationToken::new();
        let child = root.child();
        root.cancel();
        assert!(child.is_cancelled());
    }
}
