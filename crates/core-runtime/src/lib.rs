//! Headless supervisor and machine-enforced M01 lifecycle.

use core_config::CoreConfig;
use core_contracts::{
    BootstrapSafetyReceipt, BootstrapVerdict, HealthDimension, HealthSignal, HealthState,
    JournalDurability, JournalRecordKind, QuiescenceItem, RecoveryClassification,
    RuntimeGeneration, RuntimeState, SchemaVersion, ShutdownPhase, ShutdownReceipt, TransitionKind,
    TransitionReceipt, TransitionVerdict,
};
use core_health::{DegradationMatrix, HealthAggregator};
use core_identity::fingerprint;
use core_journal::{JournalError, RuntimeJournal};
use core_registry::{CapabilityRegistry, ModuleRegistry};
use serde::Serialize;
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
        if self.config.require_hive {
            verdict = BootstrapVerdict::Blocked;
            self.blocked_reasons
                .push("required HIVE provider is not attached to the standalone runtime".into());
        }
        let config_fingerprint = fingerprint(&self.config.redacted_diagnostics())
            .map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let module_graph_fingerprint =
            fingerprint(&self.modules.len()).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let capability_graph_fingerprint = fingerprint(&self.degradation.len())
            .map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let rsg_fingerprint = fingerprint(&(
            config_fingerprint.clone(),
            module_graph_fingerprint.clone(),
            capability_graph_fingerprint.clone(),
        ))
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

    pub fn shutdown(&mut self, quiescent: bool) -> Result<ShutdownReceipt, RuntimeError> {
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
        let item = QuiescenceItem {
            subject: "runtime-admission-and-leases".into(),
            satisfied: quiescent,
            residual: (!quiescent)
                .then(|| "worker/module residual requires next-boot reconciliation".into()),
        };
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
                items: vec![item],
                residuals: Vec::new(),
            }
        } else {
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
                items: vec![item],
                residuals: vec!["runtime safety reconciliation required before READY".into()],
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
        let _ = supervisor.shutdown(true).unwrap();
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
        let receipt = supervisor.shutdown(false).unwrap();
        assert!(!receipt.clean);
        assert_eq!(receipt.final_phase, ShutdownPhase::ForceTerminate);
        assert!(!receipt.residuals.is_empty());
    }

    #[test]
    fn cancellation_propagates_hierarchically() {
        let root = CancellationToken::new();
        let child = root.child();
        root.cancel();
        assert!(child.is_cancelled());
    }
}
