//! Headless supervisor and machine-enforced M01 lifecycle.

use core_config::CoreConfig;
use core_contracts::{
    ActiveLeaseSummary, BootstrapSafetyReceipt, BootstrapVerdict, CapabilityDegradationState,
    CapabilityRequirement, GenerationCoherenceBasis, HealthDimension, HealthSignal, HealthState,
    JournalDurability, JournalRecordKind, ProviderHealth, ProviderOrigin, QuiescenceItem,
    RecoveryClassification, ResourcePressure, RuntimeGeneration, RuntimeState, SchemaVersion,
    SemVer, ShutdownPhase, ShutdownReceipt, TransitionCondition, TransitionContext,
    TransitionDecision, TransitionKind, TransitionReason, TransitionReceipt, TransitionVerdict,
    TrustedComputingBaseMap,
};
use core_health::{DegradationMatrix, HealthAggregator, ProbeCoalescer};
use core_identity::{
    dependency_lock_fingerprint, fingerprint, safety_identity_fingerprint,
    security_evidence_fingerprint, tcbm_evidence_fingerprint, SafetyIdentityBasis,
};
use core_journal::{JournalError, RuntimeJournal};
use core_registry::{provider, CapabilityRegistry, ModuleRegistry};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::watch;
use tokio::time::Instant as TokioInstant;

const CARGO_LOCK_CONTENTS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.lock"));

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("illegal lifecycle transition {from:?} -> {to:?}")]
    IllegalTransition {
        from: RuntimeState,
        to: RuntimeState,
    },
    #[error("lifecycle transition denied: {0}")]
    Denied(String),
    #[error("lifecycle transition denied with typed decision: {decision:?}")]
    TransitionDenied { decision: TransitionDecision },
    #[error("worker admission denied or deferred: {receipt:?}")]
    WorkerAdmission { receipt: WorkerAdmissionReceipt },
    #[error("worker completion was reported for an unknown worker: {0}")]
    WorkerCompletion(String),
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
    pub subject: String,
    pub fingerprint: String,
    pub occurrences: u32,
    pub suppress_diagnostics: bool,
    pub quarantined: bool,
    pub state: WorkerSupervisionState,
    pub restart_backoff_ms: u64,
    pub next_restart_at_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WorkerSupervisionState {
    Running,
    RestartBackoff,
    Stabilizing,
    Quarantined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WorkerAdmissionDecision {
    Allow,
    Defer,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerAdmissionReceipt {
    pub worker_id: String,
    pub decision: WorkerAdmissionDecision,
    pub state: Option<WorkerSupervisionState>,
    pub retry_after_ms: Option<u64>,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WorkerCompletionStatus {
    Running,
    CancellationRequested,
    Completed,
}

#[derive(Debug)]
struct WorkerCompletionRegistry {
    states: Mutex<BTreeMap<String, WorkerCompletionStatus>>,
    revision: watch::Sender<u64>,
}

impl WorkerCompletionRegistry {
    fn new() -> Self {
        let (revision, _) = watch::channel(0_u64);
        Self {
            states: Mutex::new(BTreeMap::new()),
            revision,
        }
    }

    fn publish_change(&self) {
        let next = self.revision.borrow().saturating_add(1);
        self.revision.send_replace(next);
    }

    fn register(&self, worker_id: &str) {
        self.states
            .lock()
            .expect("worker completion registry lock poisoned")
            .insert(worker_id.to_owned(), WorkerCompletionStatus::Running);
        self.publish_change();
    }

    fn request_cancellation(&self) {
        let mut states = self
            .states
            .lock()
            .expect("worker completion registry lock poisoned");
        let mut changed = false;
        for state in states.values_mut() {
            if *state == WorkerCompletionStatus::Running {
                *state = WorkerCompletionStatus::CancellationRequested;
                changed = true;
            }
        }
        drop(states);
        if changed {
            self.publish_change();
        }
    }

    fn report_completed(&self, worker_id: &str) -> Result<(), RuntimeError> {
        let mut states = self
            .states
            .lock()
            .expect("worker completion registry lock poisoned");
        let state = states
            .get_mut(worker_id)
            .ok_or_else(|| RuntimeError::WorkerCompletion(worker_id.to_owned()))?;
        if *state != WorkerCompletionStatus::Completed {
            *state = WorkerCompletionStatus::Completed;
            drop(states);
            self.publish_change();
        }
        Ok(())
    }

    fn status(&self, worker_id: &str) -> Option<WorkerCompletionStatus> {
        self.states
            .lock()
            .expect("worker completion registry lock poisoned")
            .get(worker_id)
            .copied()
    }

    fn completed_worker_ids(&self) -> Vec<String> {
        self.states
            .lock()
            .expect("worker completion registry lock poisoned")
            .iter()
            .filter(|(_, state)| **state == WorkerCompletionStatus::Completed)
            .map(|(worker_id, _)| worker_id.clone())
            .collect()
    }

    fn remove(&self, worker_id: &str) {
        let removed = self
            .states
            .lock()
            .expect("worker completion registry lock poisoned")
            .remove(worker_id)
            .is_some();
        if removed {
            self.publish_change();
        }
    }

    fn subscribe(&self) -> watch::Receiver<u64> {
        self.revision.subscribe()
    }
}

#[derive(Debug, Clone)]
pub struct WorkerCompletionHandle {
    worker_id: String,
    registry: Arc<WorkerCompletionRegistry>,
    cancellation: CancellationToken,
}

impl WorkerCompletionHandle {
    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }

    pub fn status(&self) -> Option<WorkerCompletionStatus> {
        self.registry.status(&self.worker_id)
    }

    pub async fn wait_for_cancellation(&self) {
        self.cancellation.cancelled().await;
    }

    pub fn report_completed(&self) -> Result<(), RuntimeError> {
        self.registry.report_completed(&self.worker_id)
    }
}

#[derive(Debug, Clone)]
struct CrashRecord {
    fingerprint: String,
    occurrences: u32,
    last_seen_ms: u64,
    state: WorkerSupervisionState,
    next_restart_at_ms: u64,
}

#[derive(Debug)]
pub struct CrashFingerprintSuppressor {
    observations: BTreeMap<String, CrashRecord>,
    quarantine_threshold: u32,
    restart_window_ms: u64,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
    clock: Instant,
}

impl CrashFingerprintSuppressor {
    pub fn new(quarantine_threshold: u32) -> Self {
        Self {
            observations: BTreeMap::new(),
            quarantine_threshold: quarantine_threshold.max(2),
            restart_window_ms: 30_000,
            initial_backoff_ms: 100,
            max_backoff_ms: 30_000,
            clock: Instant::now(),
        }
    }

    pub fn observe<T: Serialize>(&mut self, basis: &T) -> Result<CrashObservation, RuntimeError> {
        self.observe_subject("runtime", basis)
    }

    pub fn observe_subject<T: Serialize>(
        &mut self,
        subject: &str,
        basis: &T,
    ) -> Result<CrashObservation, RuntimeError> {
        let fingerprint =
            fingerprint(&(subject, basis)).map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let now_ms = self.clock.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
        let record = self
            .observations
            .entry(subject.to_owned())
            .or_insert_with(|| CrashRecord {
                fingerprint: fingerprint.clone(),
                occurrences: 0,
                last_seen_ms: now_ms,
                state: WorkerSupervisionState::Running,
                next_restart_at_ms: 0,
            });
        if record.fingerprint != fingerprint
            || now_ms.saturating_sub(record.last_seen_ms) > self.restart_window_ms
        {
            record.fingerprint = fingerprint.clone();
            record.occurrences = 0;
            record.state = WorkerSupervisionState::Running;
        }
        record.occurrences = record.occurrences.saturating_add(1);
        record.last_seen_ms = now_ms;
        let backoff = self
            .initial_backoff_ms
            .saturating_mul(1u64 << record.occurrences.saturating_sub(1).min(16))
            .min(self.max_backoff_ms);
        record.state = if record.occurrences >= self.quarantine_threshold {
            WorkerSupervisionState::Quarantined
        } else {
            WorkerSupervisionState::RestartBackoff
        };
        record.next_restart_at_ms = now_ms.saturating_add(backoff);
        let quarantined = record.state == WorkerSupervisionState::Quarantined;
        Ok(CrashObservation {
            subject: subject.to_owned(),
            fingerprint,
            occurrences: record.occurrences,
            suppress_diagnostics: record.occurrences > 1,
            quarantined,
            state: record.state,
            restart_backoff_ms: backoff,
            next_restart_at_ms: now_ms.saturating_add(backoff),
        })
    }

    pub fn reset_subject(&mut self, subject: &str) {
        if let Some(record) = self.observations.get_mut(subject) {
            record.occurrences = 0;
            record.state = WorkerSupervisionState::Stabilizing;
            record.next_restart_at_ms = self.clock.elapsed().as_millis() as u64;
        }
    }

    pub fn admission(&self, subject: &str) -> WorkerAdmissionReceipt {
        let now_ms = self.clock.elapsed().as_millis() as u64;
        let Some(record) = self.observations.get(subject) else {
            return WorkerAdmissionReceipt {
                worker_id: subject.to_owned(),
                decision: WorkerAdmissionDecision::Allow,
                state: None,
                retry_after_ms: None,
                reason: "no prior supervision failure".into(),
            };
        };
        match record.state {
            WorkerSupervisionState::Quarantined => WorkerAdmissionReceipt {
                worker_id: subject.to_owned(),
                decision: WorkerAdmissionDecision::Deny,
                state: Some(record.state),
                retry_after_ms: None,
                reason: "worker is quarantined; explicit reset is required".into(),
            },
            WorkerSupervisionState::RestartBackoff if now_ms < record.next_restart_at_ms => {
                WorkerAdmissionReceipt {
                    worker_id: subject.to_owned(),
                    decision: WorkerAdmissionDecision::Defer,
                    state: Some(record.state),
                    retry_after_ms: Some(record.next_restart_at_ms - now_ms),
                    reason: "restart backoff has not expired".into(),
                }
            }
            _ => WorkerAdmissionReceipt {
                worker_id: subject.to_owned(),
                decision: WorkerAdmissionDecision::Allow,
                state: Some(record.state),
                retry_after_ms: None,
                reason: "supervision admission conditions satisfied".into(),
            },
        }
    }

    pub fn authorize_admission(&mut self, subject: &str) -> WorkerAdmissionReceipt {
        let mut receipt = self.admission(subject);
        if receipt.decision == WorkerAdmissionDecision::Allow {
            if let Some(record) = self.observations.get_mut(subject) {
                if record.state == WorkerSupervisionState::RestartBackoff {
                    record.state = WorkerSupervisionState::Stabilizing;
                    receipt.state = Some(record.state);
                }
            }
        }
        receipt
    }

    pub fn state(&self, subject: &str) -> Option<WorkerSupervisionState> {
        self.observations.get(subject).map(|record| record.state)
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

    pub fn decide(context: &TransitionContext) -> TransitionDecision {
        let mut conditions = BTreeSet::new();
        if !allowed(
            context.current_state,
            context.target_state,
            context.requested_transition,
        ) {
            return TransitionDecision {
                verdict: TransitionVerdict::Deny,
                reason: TransitionReason::IllegalTransition,
                conditions,
                deadline_ms: None,
            };
        }
        if context.invariant_set.iter().any(|item| item == "blocked") {
            return TransitionDecision {
                verdict: TransitionVerdict::Deny,
                reason: TransitionReason::InvariantViolation,
                conditions,
                deadline_ms: None,
            };
        }
        if matches!(context.target_state, RuntimeState::Stopped)
            && context.invariant_set.contains("quiescence-not-proven")
            && !context
                .invariant_set
                .contains("forced-termination-authorized")
        {
            return TransitionDecision {
                verdict: TransitionVerdict::Deny,
                reason: TransitionReason::InvariantViolation,
                conditions,
                deadline_ms: None,
            };
        }
        if matches!(context.target_state, RuntimeState::Ready)
            && (!context.capability_state.ready
                || !context.capability_state.quality_floor_satisfied)
        {
            let reason = if context.deadline_budget_ms == 0 {
                TransitionReason::DeadlineExhausted
            } else {
                TransitionReason::CapabilityUnavailable
            };
            return TransitionDecision {
                verdict: if context.deadline_budget_ms == 0 {
                    TransitionVerdict::Deny
                } else {
                    TransitionVerdict::Defer
                },
                reason,
                conditions,
                deadline_ms: (context.deadline_budget_ms > 0).then_some(context.deadline_budget_ms),
            };
        }
        if matches!(context.target_state, RuntimeState::Draining) && context.active_leases.total > 0
        {
            conditions.insert(TransitionCondition::DeadlineBudgetAvailable);
        }
        conditions.insert(TransitionCondition::InvariantsSatisfied);
        conditions.insert(TransitionCondition::CapabilityFloorSatisfied);
        if context.active_leases.total == 0 {
            conditions.insert(TransitionCondition::NoActiveLeases);
        }
        if context.deadline_budget_ms > 0 {
            conditions.insert(TransitionCondition::DeadlineBudgetAvailable);
        }
        TransitionDecision {
            verdict: TransitionVerdict::Allow,
            reason: TransitionReason::Allowed,
            conditions,
            deadline_ms: (context.deadline_budget_ms > 0).then_some(context.deadline_budget_ms),
        }
    }

    pub fn transition(
        &mut self,
        intent: TransitionKind,
        target: RuntimeState,
        reason: impl Into<String>,
    ) -> Result<TransitionReceipt, RuntimeError> {
        let context = TransitionContext {
            current_state: self.state,
            requested_transition: intent,
            target_state: target,
            invariant_set: BTreeSet::new(),
            generation: self.generation.clone(),
            capability_state: CapabilityDegradationState::default(),
            active_leases: ActiveLeaseSummary::default(),
            deadline_budget_ms: u64::MAX,
        };
        self.transition_with_context(context, reason)
    }

    pub fn transition_with_context(
        &mut self,
        context: TransitionContext,
        reason: impl Into<String>,
    ) -> Result<TransitionReceipt, RuntimeError> {
        let decision =
            if context.current_state != self.state || context.generation != self.generation {
                TransitionDecision {
                    verdict: TransitionVerdict::Deny,
                    reason: TransitionReason::InvariantViolation,
                    conditions: BTreeSet::new(),
                    deadline_ms: None,
                }
            } else {
                Self::decide(&context)
            };
        let reason = reason.into();
        self.journal.append(
            self.generation.boot_epoch,
            JournalRecordKind::TransitionIntent,
            JournalDurability::SyncRequired,
            [
                ("from".into(), format!("{:?}", context.current_state)),
                ("to".into(), format!("{:?}", context.target_state)),
                ("verdict".into(), format!("{:?}", decision.verdict)),
                ("reason".into(), format!("{:?}", decision.reason)),
                (
                    "conditions".into(),
                    decision
                        .conditions
                        .iter()
                        .map(|condition| format!("{condition:?}"))
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                (
                    "deadline_ms".into(),
                    decision
                        .deadline_ms
                        .map_or_else(String::new, |value| value.to_string()),
                ),
            ],
        )?;
        if decision.verdict != TransitionVerdict::Allow {
            return Err(RuntimeError::TransitionDenied { decision });
        }
        let from = self.state;
        self.state = context.target_state;
        let mut receipt = TransitionReceipt {
            schema: SchemaVersion::CURRENT,
            boot_epoch: self.generation.boot_epoch,
            from,
            to: context.target_state,
            intent: context.requested_transition,
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
                ("to".into(), format!("{:?}", context.target_state)),
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
    quarantined_workers: BTreeSet<String>,
    residual_obligations: BTreeMap<String, String>,
    crash_suppression: CrashFingerprintSuppressor,
    probe_coalescer: ProbeCoalescer<HealthSignal>,
    cancellation: CancellationToken,
    worker_completions: Arc<WorkerCompletionRegistry>,
    quiescence_proven: bool,
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
            quarantined_workers: BTreeSet::new(),
            residual_obligations: BTreeMap::new(),
            crash_suppression: CrashFingerprintSuppressor::new(3),
            probe_coalescer: ProbeCoalescer::new(),
            cancellation: CancellationToken::new(),
            worker_completions: Arc::new(WorkerCompletionRegistry::new()),
            quiescence_proven: false,
        })
    }

    fn capability_degradation_state(&self) -> CapabilityDegradationState {
        let unavailable = self
            .degradation
            .blocked()
            .into_iter()
            .collect::<BTreeSet<_>>();
        let quality_floor_satisfied =
            self.degradation.pressure() != ResourcePressure::Critical && unavailable.is_empty();
        CapabilityDegradationState {
            ready: quality_floor_satisfied,
            degraded: self.degradation.pressure() != ResourcePressure::Normal
                || !unavailable.is_empty(),
            unavailable,
            quality_floor_satisfied,
        }
    }

    pub fn build_transition_context(
        &self,
        intent: TransitionKind,
        target: RuntimeState,
        deadline_budget_ms: u64,
    ) -> TransitionContext {
        let capability_state = self.capability_degradation_state();
        let mut invariant_set = BTreeSet::new();
        if matches!(self.lifecycle.state(), RuntimeState::Blocked)
            || (matches!(target, RuntimeState::Ready) && !capability_state.ready)
        {
            invariant_set.insert("blocked".into());
        }
        if matches!(target, RuntimeState::Stopped) && !self.quiescence_proven {
            invariant_set.insert("quiescence-not-proven".into());
        }
        let active_leases = self.capabilities.active_lease_summary();
        if !self.active_modules.is_empty() {
            invariant_set.insert("active-modules".into());
        }
        if !self.isolated_workers.is_empty() {
            invariant_set.insert("active-workers".into());
        }
        TransitionContext {
            current_state: self.lifecycle.state(),
            requested_transition: intent,
            target_state: target,
            invariant_set,
            generation: self.config.generation.clone(),
            capability_state,
            active_leases,
            deadline_budget_ms,
        }
    }

    fn transition_with_live_context(
        &mut self,
        intent: TransitionKind,
        target: RuntimeState,
        reason: impl Into<String>,
        deadline_budget_ms: u64,
        forced_termination: bool,
    ) -> Result<TransitionReceipt, RuntimeError> {
        let mut context = self.build_transition_context(intent, target, deadline_budget_ms);
        if forced_termination && matches!(target, RuntimeState::Stopped) {
            context.invariant_set.remove("quiescence-not-proven");
            context
                .invariant_set
                .insert("forced-termination-authorized".into());
        }
        self.lifecycle.transition_with_context(context, reason)
    }

    pub async fn bootstrap(&mut self) -> Result<BootstrapSafetyReceipt, RuntimeError> {
        self.transition_with_live_context(
            TransitionKind::Validate,
            RuntimeState::Validating,
            "validate configuration and static contracts",
            self.config.startup_timeout_ms,
            false,
        )?;
        if let Err(error) = self.modules.validate_graph() {
            let reason = error.to_string();
            self.blocked_reasons.push(reason.clone());
            self.transition_with_live_context(
                TransitionKind::Block,
                RuntimeState::Blocked,
                reason.clone(),
                self.config.startup_timeout_ms,
                false,
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
        self.transition_with_live_context(
            TransitionKind::Bootstrap,
            RuntimeState::Bootstrapping,
            "construct supervisor and registries",
            self.config.startup_timeout_ms,
            false,
        )?;
        self.transition_with_live_context(
            TransitionKind::Synchronize,
            RuntimeState::Synchronizing,
            "optional HIVE synchronization seam",
            self.config.startup_timeout_ms,
            false,
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
        let dependency_lock = dependency_lock_fingerprint(CARGO_LOCK_CONTENTS);
        let tcbm_map = self.tcbm_map();
        let tcbm_fingerprint = tcbm_evidence_fingerprint(&tcbm_map, &dependency_lock, "m01-saf-v1")
            .map_err(|e| RuntimeError::Identity(e.to_string()))?;
        let saf_fingerprint =
            security_evidence_fingerprint(&tcbm_map, &dependency_lock, "m01-saf-v1")
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
            self.transition_with_live_context(
                TransitionKind::Block,
                RuntimeState::Blocked,
                "required HIVE capability unavailable",
                self.config.startup_timeout_ms,
                false,
            )?;
            return Err(RuntimeError::Blocked(receipt.blocked_reasons.join("; ")));
        }
        self.transition_with_live_context(
            TransitionKind::AdmitReady,
            RuntimeState::Ready,
            "bootstrap safety receipt is complete",
            self.config.startup_timeout_ms,
            false,
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
                "core-contracts",
                &["lifecycle-contracts", "capability-authority"] as &[&str],
            ),
            (
                "core-config",
                &["configuration-admission", "secret-redaction"] as &[&str],
            ),
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
            ("core-health", &["health-authority", "freshness"] as &[&str]),
        ]
        .into_iter()
        .map(|(component, domains)| TcbmEntry {
            component: component.into(),
            safety_domains: domains.iter().map(|domain| (*domain).to_owned()).collect(),
            invalidation_basis: format!(
                "m01-saf-v1|cargo-lock:{}|policy-generation",
                dependency_lock_fingerprint(CARGO_LOCK_CONTENTS)
            ),
        })
        .collect()
    }

    pub fn tcbm_map(&self) -> TrustedComputingBaseMap {
        let entries = self.tcbm();
        TrustedComputingBaseMap {
            schema: SchemaVersion::CURRENT,
            policy_version: "m01-saf-v1".into(),
            dependency_lock_fingerprint: dependency_lock_fingerprint(CARGO_LOCK_CONTENTS),
            safety_critical_components: entries
                .iter()
                .map(|entry| entry.component.clone())
                .collect(),
            entries: entries
                .iter()
                .map(|entry| entry.component.clone())
                .collect(),
        }
    }

    pub fn observe_crash<T: Serialize>(
        &mut self,
        basis: &T,
    ) -> Result<CrashObservation, RuntimeError> {
        self.crash_suppression.observe(basis)
    }

    pub fn observe_worker_crash<T: Serialize>(
        &mut self,
        worker_id: &str,
        basis: &T,
    ) -> Result<CrashObservation, RuntimeError> {
        let observation = self.crash_suppression.observe_subject(worker_id, basis)?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::WorkerSupervisionTransition,
            JournalDurability::SyncRequired,
            [
                ("subject".into(), worker_id.into()),
                ("fingerprint".into(), observation.fingerprint.clone()),
                ("state".into(), format!("{:?}", observation.state)),
                ("occurrences".into(), observation.occurrences.to_string()),
                (
                    "restart_backoff_ms".into(),
                    observation.restart_backoff_ms.to_string(),
                ),
            ],
        )?;
        if observation.quarantined {
            self.quarantined_workers.insert(worker_id.into());
            self.isolated_workers.remove(worker_id);
        }
        Ok(observation)
    }

    pub fn reset_worker_supervision(&mut self, worker_id: &str) -> Result<(), RuntimeError> {
        self.crash_suppression.reset_subject(worker_id);
        self.quarantined_workers.remove(worker_id);
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::WorkerSupervisionTransition,
            JournalDurability::SyncRequired,
            [
                ("subject".into(), worker_id.into()),
                ("state".into(), "Stabilizing".into()),
                ("reason".into(), "explicit-reset-re-entry".into()),
            ],
        )?;
        Ok(())
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
        let mut requirement = CapabilityRequirement::new("context", SemVer::new(1, 0, 0));
        requirement.ownership = core_contracts::CapabilityOwnership::HiveOwnedIntelligence;
        let initial = self
            .capabilities
            .bind_with_generation(&requirement, "soak-fallback", &self.config.generation)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let lease = self
            .capabilities
            .acquire_lease_with_generation("context", &self.config.generation, 30_000)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .set_provider_health("hive-context", ProviderHealth::Healthy)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let substituted = self
            .capabilities
            .substitute_with_generation(&requirement, "soak-hive-connect", &self.config.generation)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .validate_lease(&lease, &self.config.generation)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let coherence = self
            .capabilities
            .generation_coherence(
                "context",
                GenerationCoherenceBasis {
                    runtime: self.config.generation.clone(),
                    binding_generation: substituted.binding_generation,
                    provider_activation_generation: 1,
                },
            )
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .release_lease(&lease)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        self.capabilities
            .set_provider_health("hive-context", ProviderHealth::Unavailable)
            .map_err(|error| RuntimeError::Denied(error.to_string()))?;
        let flap_recovered_with_fallback = self
            .capabilities
            .substitute_with_generation(
                &requirement,
                "soak-hive-disconnect",
                &self.config.generation,
            )
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
        self.register_isolated_worker("soak-worker")?;
        self.mark_worker_terminated("soak-worker");
        self.set_resource_pressure(ResourcePressure::Normal)?;
        let crash_first = self.observe_crash(&("soak-worker", "same-failure"))?;
        let crash_second = self.observe_crash(&("soak-worker", "same-failure"))?;
        let probe_key = "hive-context-health";
        let first_probe = self.probe_coalescer.get_or_probe_fresh(
            probe_key,
            self.health.now_ms(),
            self.config.startup_timeout_ms,
            || HealthSignal {
                dimension: HealthDimension::ExternalDependency,
                state: HealthState::Healthy,
                reason_code: "hive-probe-ok".into(),
                generation: self.config.generation.boot_epoch,
                impact: "none".into(),
                observed_at_monotonic_ms: 0,
                freshness_window_ms: self.config.startup_timeout_ms,
                evidence_fingerprint: "hive-probe-ok".into(),
            },
        );
        let second_probe = self.probe_coalescer.get_or_probe_fresh(
            probe_key,
            self.health.now_ms(),
            self.config.startup_timeout_ms,
            || HealthSignal {
                dimension: HealthDimension::ExternalDependency,
                state: HealthState::Healthy,
                reason_code: "hive-probe-refreshed".into(),
                generation: self.config.generation.boot_epoch,
                impact: "none".into(),
                observed_at_monotonic_ms: 0,
                freshness_window_ms: self.config.startup_timeout_ms,
                evidence_fingerprint: "hive-probe-refreshed".into(),
            },
        );
        self.health.set(first_probe.clone());
        let probe_authorized = self.health.can_authorize(
            HealthDimension::ExternalDependency,
            self.config.generation.boot_epoch,
        );
        let stale_signal = HealthSignal {
            observed_at_monotonic_ms: 0,
            freshness_window_ms: 0,
            ..first_probe.clone()
        };
        self.health.set(stale_signal);
        let stale_probe_authorized = self.health.can_authorize(
            HealthDimension::ExternalDependency,
            self.config.generation.boot_epoch,
        );
        self.register_isolated_worker("soak-worker")?;
        let worker_crash = self.observe_worker_crash("soak-worker", &"same-crash")?;
        let worker_crash_repeat = self.observe_worker_crash("soak-worker", &"same-crash")?;
        let worker_crash_quarantine = self.observe_worker_crash("soak-worker", &"same-crash")?;
        self.reset_worker_supervision("soak-worker")?;
        self.register_isolated_worker("soak-worker")?;
        self.mark_worker_terminated("soak-worker");
        let shutdown = self.shutdown().await?;
        Ok(serde_json::json!({
            "provider_initial": initial.provider_id,
            "provider_substituted": substituted.provider_id,
            "provider_flap_recovered_with_fallback": flap_recovered_with_fallback,
            "generation_coherent": coherence.coherent,
            "config_generation": config_generation,
            "crash_suppression": crash_second.suppress_diagnostics,
            "crash_first_fingerprint": crash_first.fingerprint,
            "probe_coalesced": first_probe.evidence_fingerprint == second_probe.evidence_fingerprint,
            "probe_authorized": probe_authorized,
            "stale_probe_authorized": stale_probe_authorized,
            "worker_crash_suppressed": worker_crash_repeat.suppress_diagnostics,
            "worker_quarantined": worker_crash_quarantine.quarantined,
            "worker_initial_state": format!("{:?}", worker_crash.state),
            "shutdown_clean": shutdown.clean,
            "shutdown_fingerprint": shutdown.quiescence_fingerprint,
        }))
    }

    pub fn register_isolated_worker(
        &mut self,
        worker_id: impl Into<String>,
    ) -> Result<WorkerAdmissionReceipt, RuntimeError> {
        let worker_id = worker_id.into();
        if self.quarantined_workers.contains(&worker_id) {
            return Err(RuntimeError::WorkerAdmission {
                receipt: WorkerAdmissionReceipt {
                    worker_id,
                    decision: WorkerAdmissionDecision::Deny,
                    state: Some(WorkerSupervisionState::Quarantined),
                    retry_after_ms: None,
                    reason: "worker is quarantined; explicit reset is required".into(),
                },
            });
        }
        let receipt = self.crash_suppression.authorize_admission(&worker_id);
        if receipt.decision != WorkerAdmissionDecision::Allow {
            return Err(RuntimeError::WorkerAdmission { receipt });
        }
        if !self.isolated_workers.insert(worker_id.clone()) {
            return Err(RuntimeError::WorkerAdmission {
                receipt: WorkerAdmissionReceipt {
                    worker_id,
                    decision: WorkerAdmissionDecision::Deny,
                    state: Some(WorkerSupervisionState::Running),
                    retry_after_ms: None,
                    reason: "worker is already admitted".into(),
                },
            });
        }
        self.worker_completions.register(&worker_id);
        Ok(receipt)
    }

    pub fn worker_completion_handle(&self, worker_id: &str) -> Option<WorkerCompletionHandle> {
        if !self.isolated_workers.contains(worker_id) {
            return None;
        }
        Some(WorkerCompletionHandle {
            worker_id: worker_id.to_owned(),
            registry: Arc::clone(&self.worker_completions),
            cancellation: self.cancellation.clone(),
        })
    }

    pub fn mark_worker_terminated(&mut self, worker_id: &str) {
        let _ = self.worker_completions.report_completed(worker_id);
        self.isolated_workers.remove(worker_id);
        self.worker_completions.remove(worker_id);
    }

    fn reconcile_completed_workers(&mut self) {
        for worker_id in self.worker_completions.completed_worker_ids() {
            self.isolated_workers.remove(&worker_id);
            self.worker_completions.remove(&worker_id);
        }
    }

    fn remaining_shutdown_budget(&self, deadline: TokioInstant) -> u64 {
        deadline
            .saturating_duration_since(TokioInstant::now())
            .as_millis()
            .min(u128::from(u64::MAX)) as u64
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
            self.transition_with_live_context(
                TransitionKind::MarkDegraded,
                RuntimeState::Degraded,
                reason.clone(),
                self.config.startup_timeout_ms,
                false,
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

    pub async fn shutdown(&mut self) -> Result<ShutdownReceipt, RuntimeError> {
        if !matches!(
            self.lifecycle.state(),
            RuntimeState::Ready | RuntimeState::Degraded
        ) {
            return Err(RuntimeError::Denied(
                "shutdown requires READY or DEGRADED".into(),
            ));
        }
        self.quiescence_proven = false;
        self.admission_closed.store(true, Ordering::Release);
        let shutdown_deadline =
            TokioInstant::now() + Duration::from_millis(self.config.shutdown_timeout_ms);
        self.transition_with_live_context(
            TransitionKind::Drain,
            RuntimeState::Draining,
            "close admission and drain leases",
            self.remaining_shutdown_budget(shutdown_deadline),
            false,
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
        let mut lease_changes = self.capabilities.subscribe_lease_changes();
        let mut active_leases = self.capabilities.total_active_leases();
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::LeaseDrainStarted,
            JournalDurability::SyncRequired,
            [
                ("active_leases".into(), active_leases.to_string()),
                (
                    "deadline_ms".into(),
                    self.config.shutdown_timeout_ms.to_string(),
                ),
            ],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::LeaseDrainAttempted,
            JournalDurability::SyncRequired,
            [("active_leases".into(), active_leases.to_string())],
        )?;
        while active_leases > 0 {
            let remaining = shutdown_deadline.saturating_duration_since(TokioInstant::now());
            if remaining.is_zero() {
                break;
            }
            let deadline_sleep = tokio::time::sleep(remaining);
            let expiry_sleep = tokio::time::sleep(
                self.capabilities
                    .next_lease_expiry()
                    .map_or(remaining, |expiry| std::cmp::min(remaining, expiry)),
            );
            tokio::pin!(deadline_sleep);
            tokio::pin!(expiry_sleep);
            tokio::select! {
                changed = lease_changes.changed() => {
                    if changed.is_err() {
                        break;
                    }
                    active_leases = self.capabilities.total_active_leases();
                }
                _ = &mut deadline_sleep => break,
                _ = &mut expiry_sleep => {
                    active_leases = self.capabilities.total_active_leases();
                }
            }
        }
        active_leases = self.capabilities.total_active_leases();
        if active_leases == 0 {
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::LeaseDrainCompleted,
                JournalDurability::SyncRequired,
                [],
            )?;
        } else {
            self.lifecycle.journal_mut().append(
                self.config.generation.boot_epoch,
                JournalRecordKind::LeaseDrainTimedOut,
                JournalDurability::SyncRequired,
                [("active_leases".into(), active_leases.to_string())],
            )?;
        }
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::CooperativeCancelStarted,
            JournalDurability::SyncRequired,
            [],
        )?;
        self.worker_completions.request_cancellation();
        self.cancellation.cancel();
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::CooperativeCancelAttempted,
            JournalDurability::SyncRequired,
            [("cancel_requested".into(), "true".into())],
        )?;
        let mut completion_revision = self.worker_completions.subscribe();
        loop {
            self.reconcile_completed_workers();
            if self.active_modules.is_empty() && self.isolated_workers.is_empty() {
                break;
            }
            let remaining = shutdown_deadline.saturating_duration_since(TokioInstant::now());
            if remaining.is_zero() {
                break;
            }
            let deadline_sleep = tokio::time::sleep(remaining);
            tokio::pin!(deadline_sleep);
            tokio::select! {
                changed = completion_revision.changed() => {
                    if changed.is_err() {
                        break;
                    }
                }
                _ = &mut deadline_sleep => break,
            }
        }
        self.reconcile_completed_workers();
        let cancellation_complete =
            self.active_modules.is_empty() && self.isolated_workers.is_empty();
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            if cancellation_complete {
                JournalRecordKind::CooperativeCancelCompleted
            } else {
                JournalRecordKind::CooperativeCancelTimedOut
            },
            JournalDurability::SyncRequired,
            [],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::CleanupStarted,
            JournalDurability::SyncRequired,
            [],
        )?;
        self.reconcile_completed_workers();
        let cleanup_complete = self.active_modules.is_empty()
            && self.isolated_workers.is_empty()
            && self.residual_obligations.is_empty();
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            JournalRecordKind::CleanupAttempted,
            JournalDurability::SyncRequired,
            [],
        )?;
        self.lifecycle.journal_mut().append(
            self.config.generation.boot_epoch,
            if cleanup_complete {
                JournalRecordKind::CleanupCompleted
            } else {
                JournalRecordKind::CleanupTimedOut
            },
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
                satisfied: active_leases == 0,
                residual: (active_leases > 0).then(|| "active capability lease remains".into()),
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
        self.quiescence_proven = quiescent;
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
            self.transition_with_live_context(
                TransitionKind::Stop,
                RuntimeState::Stopped,
                "quiescence matrix satisfied",
                self.remaining_shutdown_budget(shutdown_deadline),
                false,
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
            self.transition_with_live_context(
                TransitionKind::Stop,
                RuntimeState::Stopped,
                "forced termination with explicit residuals",
                self.remaining_shutdown_budget(shutdown_deadline),
                true,
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
    state: Arc<watch::Sender<bool>>,
}

impl CancellationToken {
    pub fn new() -> Self {
        let (state, _) = watch::channel(false);
        Self {
            state: Arc::new(state),
        }
    }
    pub fn child(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
    pub fn cancel(&self) {
        self.state.send_replace(true);
    }
    pub fn is_cancelled(&self) -> bool {
        let receiver = self.state.subscribe();
        let cancelled = *receiver.borrow();
        cancelled
    }
    pub async fn cancelled(&self) {
        let mut receiver = self.state.subscribe();
        while !*receiver.borrow() {
            if receiver.changed().await.is_err() {
                return;
            }
        }
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
        let _ = supervisor.shutdown().await.unwrap();
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
        let mut shutdown_config = config("forced");
        shutdown_config.shutdown_timeout_ms = 5;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor.register_isolated_worker("hung-worker").unwrap();
        let receipt = supervisor.shutdown().await.unwrap();
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
        first.shutdown().await.unwrap();
        second.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn qvm_detects_active_lease_without_caller_claim() {
        let mut shutdown_config = config("lease-residual");
        shutdown_config.shutdown_timeout_ms = 5;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
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
        let receipt = supervisor.shutdown().await.unwrap();
        assert!(!receipt.clean);
        assert!(receipt
            .items
            .iter()
            .any(|item| item.subject == "critical-capability-leases" && !item.satisfied));
    }

    #[test]
    fn rlc_returns_typed_allow_deny_and_defer_without_mutation() {
        let base = TransitionContext {
            current_state: RuntimeState::Ready,
            requested_transition: TransitionKind::Drain,
            target_state: RuntimeState::Draining,
            invariant_set: BTreeSet::new(),
            generation: RuntimeGeneration::new(1),
            capability_state: CapabilityDegradationState::default(),
            active_leases: ActiveLeaseSummary::default(),
            deadline_budget_ms: 100,
        };
        assert_eq!(Lifecycle::decide(&base).verdict, TransitionVerdict::Allow);
        let mut deferred = base.clone();
        deferred.active_leases.total = 1;
        let decision = Lifecycle::decide(&deferred);
        assert_eq!(decision.verdict, TransitionVerdict::Allow);
        assert_eq!(decision.reason, TransitionReason::Allowed);
        let mut capability_deferred = TransitionContext {
            current_state: RuntimeState::Synchronizing,
            requested_transition: TransitionKind::AdmitReady,
            target_state: RuntimeState::Ready,
            capability_state: CapabilityDegradationState {
                ready: false,
                degraded: true,
                unavailable: ["context".into()].into_iter().collect(),
                quality_floor_satisfied: false,
            },
            ..deferred
        };
        let decision = Lifecycle::decide(&capability_deferred);
        assert_eq!(decision.verdict, TransitionVerdict::Defer);
        assert_eq!(decision.reason, TransitionReason::CapabilityUnavailable);
        capability_deferred.deadline_budget_ms = 0;
        let decision = Lifecycle::decide(&capability_deferred);
        assert_eq!(decision.verdict, TransitionVerdict::Deny);
        assert_eq!(decision.reason, TransitionReason::DeadlineExhausted);
        let mut denied = base;
        denied.invariant_set.insert("blocked".into());
        let decision = Lifecycle::decide(&denied);
        assert_eq!(decision.verdict, TransitionVerdict::Deny);
        assert_eq!(decision.reason, TransitionReason::InvariantViolation);
    }

    #[tokio::test]
    async fn shutdown_completion_records_follow_proof() {
        let mut shutdown_config = config("causal-shutdown");
        shutdown_config.shutdown_timeout_ms = 5;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
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
        supervisor.register_isolated_worker("hung-worker").unwrap();
        let receipt = supervisor.shutdown().await.unwrap();
        assert!(!receipt.clean);
        let records = supervisor.lifecycle.journal.read_validated().unwrap();
        assert!(!records
            .iter()
            .any(|record| record.kind == JournalRecordKind::LeaseDrainCompleted));
        assert!(!records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CooperativeCancelCompleted));
        assert!(!records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CleanupCompleted));
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::LeaseDrainTimedOut));
    }

    #[tokio::test]
    async fn crash_loop_quarantines_and_explicit_reset_reenters_worker() {
        let mut supervisor = Supervisor::new(config("crash-loop")).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor.register_isolated_worker("worker-a").unwrap();
        let first = supervisor
            .observe_worker_crash("worker-a", &"same-crash")
            .unwrap();
        let second = supervisor
            .observe_worker_crash("worker-a", &"same-crash")
            .unwrap();
        let third = supervisor
            .observe_worker_crash("worker-a", &"same-crash")
            .unwrap();
        assert_eq!(first.state, WorkerSupervisionState::RestartBackoff);
        assert!(second.suppress_diagnostics);
        assert!(third.quarantined);
        supervisor.reset_worker_supervision("worker-a").unwrap();
        supervisor.register_isolated_worker("worker-a").unwrap();
        assert_eq!(
            supervisor.crash_suppression.state("worker-a"),
            Some(WorkerSupervisionState::Stabilizing)
        );
        supervisor.mark_worker_terminated("worker-a");
        supervisor.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn worker_admission_cannot_bypass_backoff_or_quarantine() {
        let mut shutdown_config = config("worker-admission");
        shutdown_config.shutdown_timeout_ms = 10;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor.register_isolated_worker("worker-a").unwrap();
        supervisor.mark_worker_terminated("worker-a");

        supervisor
            .observe_worker_crash("worker-a", &"same-crash")
            .unwrap();
        let deferred = supervisor
            .register_isolated_worker("worker-a")
            .expect_err("backoff must defer direct re-registration");
        assert!(matches!(
            deferred,
            RuntimeError::WorkerAdmission { receipt }
                if receipt.decision == WorkerAdmissionDecision::Defer
        ));
        supervisor
            .observe_worker_crash("worker-a", &"same-crash")
            .unwrap();
        supervisor
            .observe_worker_crash("worker-a", &"same-crash")
            .unwrap();
        let quarantined = supervisor
            .register_isolated_worker("worker-a")
            .expect_err("quarantine must deny direct re-registration");
        assert!(matches!(
            quarantined,
            RuntimeError::WorkerAdmission { receipt }
                if receipt.decision == WorkerAdmissionDecision::Deny
        ));
        supervisor.reset_worker_supervision("worker-a").unwrap();
        supervisor.register_isolated_worker("worker-a").unwrap();
        supervisor.mark_worker_terminated("worker-a");
        supervisor.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn lease_expiration_completes_before_shutdown_deadline() {
        let mut shutdown_config = config("lease-before-deadline");
        shutdown_config.shutdown_timeout_ms = 1_000;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
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
        let generation = supervisor.status().generation;
        supervisor
            .capabilities()
            .bind_with_generation(&requirement, "deadline-test", &generation)
            .unwrap();
        supervisor
            .capabilities()
            .acquire_lease_with_generation("health", &generation, 10)
            .unwrap();
        let started = Instant::now();
        let receipt = supervisor.shutdown().await.unwrap();
        assert!(started.elapsed() < Duration::from_millis(800));
        assert!(receipt.clean);
        let records = supervisor.lifecycle.journal.read_validated().unwrap();
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::LeaseDrainCompleted));
        assert!(!records
            .iter()
            .any(|record| record.kind == JournalRecordKind::LeaseDrainTimedOut));
    }

    #[tokio::test]
    async fn lease_released_from_separate_task_wakes_qds_before_deadline() {
        let mut shutdown_config = config("lease-release-wakes-qds");
        shutdown_config.shutdown_timeout_ms = 1_000;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
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
        let generation = supervisor.status().generation;
        supervisor
            .capabilities()
            .bind_with_generation(&requirement, "release-wakes-qds", &generation)
            .unwrap();
        let lease = supervisor
            .capabilities()
            .acquire_lease("health", 17, 30_000)
            .unwrap();
        let capabilities = supervisor.capabilities().clone();
        let releaser = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(40)).await;
            capabilities.release_lease(&lease).unwrap();
        });

        let started = Instant::now();
        let receipt = supervisor.shutdown().await.unwrap();
        let elapsed = started.elapsed();
        releaser.await.unwrap();

        assert!(receipt.clean, "lease release receipt: {receipt:?}");
        assert!(elapsed < Duration::from_millis(900));
        let records = supervisor.lifecycle.journal.read_validated().unwrap();
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::LeaseDrainCompleted));
    }

    #[tokio::test]
    async fn unreleased_lease_times_out_only_after_shutdown_deadline() {
        let timeout_ms = 50;
        let mut shutdown_config = config("unreleased-lease-deadline");
        shutdown_config.shutdown_timeout_ms = timeout_ms;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
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
        let generation = supervisor.status().generation;
        supervisor
            .capabilities()
            .bind_with_generation(&requirement, "unreleased-lease", &generation)
            .unwrap();
        let lease = supervisor
            .capabilities()
            .acquire_lease("health", 17, 30_000)
            .unwrap();
        let capabilities = supervisor.capabilities().clone();

        let started = Instant::now();
        let receipt = supervisor.shutdown().await.unwrap();
        let elapsed = started.elapsed();
        capabilities.release_lease(&lease).unwrap();

        assert!(!receipt.clean, "unreleased lease must be residual");
        assert!(elapsed >= Duration::from_millis(timeout_ms));
        assert!(receipt
            .items
            .iter()
            .any(|item| { item.subject == "critical-capability-leases" && !item.satisfied }));
    }

    #[tokio::test]
    async fn multiple_leases_release_in_different_order_without_lost_notifications() {
        let mut shutdown_config = config("multiple-lease-release-order");
        shutdown_config.shutdown_timeout_ms = 500;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
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
        let generation = supervisor.status().generation;
        supervisor
            .capabilities()
            .bind_with_generation(&requirement, "multiple-release-order", &generation)
            .unwrap();
        let leases = (0..3)
            .map(|_| {
                supervisor
                    .capabilities()
                    .acquire_lease("health", 17, 30_000)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let capabilities = supervisor.capabilities().clone();
        let release_order = vec![leases[1].clone(), leases[2].clone(), leases[0].clone()];
        let releaser = tokio::spawn(async move {
            let mut released = Vec::new();
            for (index, lease) in release_order.into_iter().enumerate() {
                tokio::time::sleep(Duration::from_millis(20)).await;
                let lease_id = lease.lease_id.clone();
                capabilities.release_lease(&lease).unwrap();
                released.push((index, lease_id));
            }
            released
        });

        let receipt = supervisor.shutdown().await.unwrap();
        let released = releaser.await.unwrap();

        assert_eq!(released.len(), 3);
        assert!(receipt.clean, "multiple lease receipt: {receipt:?}");
        assert!(receipt
            .items
            .iter()
            .any(|item| item.subject == "critical-capability-leases" && item.satisfied));
    }

    #[tokio::test]
    async fn worker_finishes_after_cancel_before_shutdown_deadline() {
        let mut shutdown_config = config("worker-before-deadline");
        shutdown_config.shutdown_timeout_ms = 500;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor
            .register_isolated_worker("finishing-worker")
            .unwrap();
        let worker = supervisor
            .worker_completion_handle("finishing-worker")
            .unwrap();
        let worker_task = tokio::spawn(async move {
            worker.wait_for_cancellation().await;
            tokio::time::sleep(Duration::from_millis(10)).await;
            worker.report_completed().unwrap();
        });
        let receipt = supervisor.shutdown().await.unwrap();
        worker_task.await.unwrap();
        assert!(receipt.clean);
        let records = supervisor.lifecycle.journal.read_validated().unwrap();
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CooperativeCancelCompleted));
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CleanupCompleted));
        assert!(!records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CooperativeCancelTimedOut));
    }

    #[tokio::test]
    async fn multiple_workers_report_completion_without_lost_notifications() {
        let mut shutdown_config = config("multiple-workers-before-deadline");
        shutdown_config.shutdown_timeout_ms = 1_000;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
        supervisor.bootstrap().await.unwrap();
        let mut worker_tasks = Vec::new();
        for (worker_id, delay_ms) in [
            ("worker-fast", 5_u64),
            ("worker-middle", 25),
            ("worker-slow", 50),
        ] {
            supervisor.register_isolated_worker(worker_id).unwrap();
            let worker = supervisor.worker_completion_handle(worker_id).unwrap();
            worker_tasks.push(tokio::spawn(async move {
                worker.wait_for_cancellation().await;
                assert_eq!(
                    worker.status(),
                    Some(WorkerCompletionStatus::CancellationRequested)
                );
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                worker.report_completed().unwrap();
                worker.worker_id().to_owned()
            }));
        }

        let receipt = supervisor.shutdown().await.unwrap();
        let mut completed_ids = Vec::new();
        for task in worker_tasks {
            completed_ids.push(task.await.unwrap());
        }
        assert_eq!(
            completed_ids,
            vec![
                "worker-fast".to_owned(),
                "worker-middle".to_owned(),
                "worker-slow".to_owned()
            ]
        );
        assert!(receipt.clean, "multi-worker shutdown receipt: {receipt:?}");
        assert!(receipt
            .items
            .iter()
            .any(|item| item.subject == "isolated-workers" && item.satisfied));
    }

    #[tokio::test]
    async fn worker_completion_at_deadline_boundary_is_fail_closed() {
        let timeout_ms = 30;
        let mut shutdown_config = config("worker-deadline-boundary");
        shutdown_config.shutdown_timeout_ms = timeout_ms;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor
            .register_isolated_worker("boundary-worker")
            .unwrap();
        let worker = supervisor
            .worker_completion_handle("boundary-worker")
            .unwrap();
        let worker_task = tokio::spawn(async move {
            worker.wait_for_cancellation().await;
            tokio::time::sleep(Duration::from_millis(25)).await;
            worker.report_completed().unwrap();
        });

        let started = Instant::now();
        let receipt = supervisor.shutdown().await.unwrap();
        let elapsed = started.elapsed();
        worker_task.await.unwrap();
        assert!(receipt.clean || elapsed >= Duration::from_millis(timeout_ms));
    }

    #[tokio::test]
    async fn shutdown_timeout_escalates_only_after_deadline() {
        let mut shutdown_config = config("deadline-timeout");
        shutdown_config.shutdown_timeout_ms = 5;
        let mut supervisor = Supervisor::new(shutdown_config).unwrap();
        supervisor.bootstrap().await.unwrap();
        supervisor.register_isolated_worker("hung-worker").unwrap();
        let started = Instant::now();
        let receipt = supervisor.shutdown().await.unwrap();
        assert!(!receipt.clean);
        assert!(started.elapsed() >= Duration::from_millis(5));
        let records = supervisor.lifecycle.journal.read_validated().unwrap();
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CooperativeCancelTimedOut));
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::CleanupTimedOut));
        assert!(records
            .iter()
            .any(|record| record.kind == JournalRecordKind::ShutdownEscalated));
    }

    #[test]
    fn cancellation_propagates_hierarchically() {
        let root = CancellationToken::new();
        let child = root.child();
        root.cancel();
        assert!(child.is_cancelled());
    }
}
