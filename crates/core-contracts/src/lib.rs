//! Versioned, dependency-light contracts shared by the M01 crates.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const CONTRACT_SCHEMA_MAJOR: u16 = 1;
pub const CONTRACT_SCHEMA_MINOR: u16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}

impl SchemaVersion {
    pub const CURRENT: Self = Self {
        major: CONTRACT_SCHEMA_MAJOR,
        minor: CONTRACT_SCHEMA_MINOR,
    };

    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn compatible_with(self, required: Self) -> bool {
        self.major == required.major && self.minor >= required.minor
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SemVer {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl SemVer {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    pub fn compatible_with(self, required: Self) -> bool {
        self.major == required.major && self >= required
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::new(0, 1, 0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeGeneration {
    pub boot_epoch: u64,
    pub config: u64,
    pub module_graph: u64,
    pub capability_graph: u64,
    pub policy: u64,
}

impl RuntimeGeneration {
    pub fn new(boot_epoch: u64) -> Self {
        Self {
            boot_epoch,
            config: 0,
            module_graph: 0,
            capability_graph: 0,
            policy: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeState {
    Created,
    Validating,
    Bootstrapping,
    Synchronizing,
    Ready,
    Degraded,
    Blocked,
    Draining,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionKind {
    Validate,
    Bootstrap,
    Synchronize,
    AdmitReady,
    MarkDegraded,
    Block,
    Drain,
    Stop,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionVerdict {
    Allow,
    Deny,
    Defer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionReason {
    Allowed,
    IllegalTransition,
    InvariantViolation,
    CapabilityUnavailable,
    ActiveLeases,
    DeadlineExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TransitionCondition {
    InvariantsSatisfied,
    CapabilityFloorSatisfied,
    NoActiveLeases,
    DeadlineBudgetAvailable,
    JournalIntentDurable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDegradationState {
    pub ready: bool,
    pub degraded: bool,
    pub unavailable: BTreeSet<String>,
    pub quality_floor_satisfied: bool,
}

impl Default for CapabilityDegradationState {
    fn default() -> Self {
        Self {
            ready: true,
            degraded: false,
            unavailable: BTreeSet::new(),
            quality_floor_satisfied: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActiveLeaseSummary {
    pub total: u64,
    pub safety_critical: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionContext {
    pub current_state: RuntimeState,
    pub requested_transition: TransitionKind,
    pub target_state: RuntimeState,
    pub invariant_set: BTreeSet<String>,
    pub generation: RuntimeGeneration,
    pub capability_state: CapabilityDegradationState,
    pub active_leases: ActiveLeaseSummary,
    pub deadline_budget_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionDecision {
    pub verdict: TransitionVerdict,
    pub reason: TransitionReason,
    pub conditions: BTreeSet<TransitionCondition>,
    pub deadline_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    ConfigurationInvalid,
    ModuleGraphInvalid,
    CapabilityUnavailable,
    CapabilityIncompatible,
    ProviderDisconnected,
    StartupTimeout,
    StartupInvariantFailed,
    ModuleStartFailed,
    ModuleCrashed,
    CancellationFailed,
    DrainTimeout,
    ShutdownForced,
    InternalInvariantViolation,
    StaleEpoch,
    JournalCorrupt,
    FrameTooLarge,
    ProtocolMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractErrorInfo {
    pub code: ErrorCode,
    pub severity: Severity,
    pub retryable: bool,
    pub detail: String,
}

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("invalid contract: {0}")]
    Invalid(String),
    #[error("incompatible schema {actual:?}; required {required:?}")]
    IncompatibleSchema {
        actual: SchemaVersion,
        required: SchemaVersion,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationClass {
    InProcessTrusted,
    ChildProcess,
    SandboxRequired,
    ExternalProvider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum AssuranceClass {
    Untrusted,
    #[default]
    Standard,
    Verified,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum ResourcePressure {
    #[default]
    Normal,
    Pressured,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub name: String,
    pub contract: SemVer,
    pub required_features: BTreeSet<String>,
    pub quality_floor: u8,
    pub policy: String,
    pub required_authorities: BTreeSet<String>,
    pub minimum_assurance: AssuranceClass,
    pub minimum_trust: u8,
    #[serde(default)]
    pub required_origin: Option<ProviderOrigin>,
    #[serde(default)]
    pub required_provider_class: Option<String>,
    pub preferred_origin: Option<ProviderOrigin>,
    pub preferred_provider_class: Option<String>,
    #[serde(default)]
    pub ownership: CapabilityOwnership,
    pub cache_affinity: String,
    pub max_latency_micros: Option<u64>,
    pub max_cost_milli: Option<u64>,
}

impl CapabilityRequirement {
    pub fn new(name: impl Into<String>, contract: SemVer) -> Self {
        Self {
            name: name.into(),
            contract,
            required_features: BTreeSet::new(),
            quality_floor: 0,
            policy: String::new(),
            required_authorities: BTreeSet::new(),
            minimum_assurance: AssuranceClass::Standard,
            minimum_trust: 0,
            required_origin: None,
            required_provider_class: None,
            preferred_origin: None,
            preferred_provider_class: None,
            ownership: CapabilityOwnership::CoreOwned,
            cache_affinity: String::new(),
            max_latency_micros: None,
            max_cost_milli: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProviderOrigin {
    CoreNative,
    CoreFallback,
    HiveExternal,
    OtherExternal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CapabilityOwnership {
    #[default]
    CoreOwned,
    HiveOwnedIntelligence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderHealth {
    Healthy,
    Degraded,
    Unavailable,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityProviderDescriptor {
    pub provider_id: String,
    pub module_id: String,
    pub capability: String,
    pub contract: SemVer,
    pub origin: ProviderOrigin,
    pub features: BTreeSet<String>,
    pub quality: u8,
    pub health: ProviderHealth,
    pub trust: u8,
    pub activation_generation: u64,
    pub fingerprint: String,
    pub policy: String,
    pub authority_requirements: BTreeSet<String>,
    pub assurance: AssuranceClass,
    pub latency_micros: u64,
    pub cost_milli: u64,
    pub performance_score: u32,
    pub provider_class: String,
    pub cache_affinity: String,
    pub dependency_capabilities: BTreeSet<String>,
    #[serde(default)]
    pub evidence_dependencies: BTreeSet<String>,
    #[serde(default)]
    pub safety_critical: bool,
    pub readiness: bool,
    pub quarantined: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityBindingReceipt {
    pub schema: SchemaVersion,
    pub capability: String,
    pub provider_id: String,
    pub provider_fingerprint: String,
    pub binding_generation: u64,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityBindingIdentity {
    pub capability: String,
    pub provider_id: String,
    pub provider_fingerprint: String,
    pub provider_generation: u64,
    pub binding_generation: u64,
    pub active_leases: u64,
    #[serde(default)]
    pub runtime_generation: RuntimeGeneration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLease {
    pub schema: SchemaVersion,
    pub lease_id: String,
    pub capability: String,
    pub provider_id: String,
    pub provider_fingerprint: String,
    pub binding_generation: u64,
    pub boot_epoch: u64,
    #[serde(default)]
    pub runtime_generation: RuntimeGeneration,
    pub expires_at_monotonic_ms: u64,
    pub revoked: bool,
    pub policy: String,
    pub authority_requirements: BTreeSet<String>,
    pub assurance: AssuranceClass,
    pub cache_affinity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub module_id: String,
    pub contract: SemVer,
    pub implementation: SemVer,
    pub build_identity: String,
    pub isolation: IsolationClass,
    pub required: Vec<CapabilityRequirement>,
    pub optional: Vec<CapabilityRequirement>,
    pub provided: BTreeSet<String>,
    pub startup_dependencies: BTreeSet<String>,
    pub critical: bool,
    pub lifecycle_hooks: BTreeSet<String>,
    pub health_contract: String,
    pub startup_deadline_ms: u64,
    pub shutdown_deadline_ms: u64,
    pub config_namespace: String,
    pub config_schema: SchemaVersion,
    pub authority_requirements: BTreeSet<String>,
    pub event_contract_versions: BTreeMap<String, SemVer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionReceipt {
    pub schema: SchemaVersion,
    pub boot_epoch: u64,
    pub from: RuntimeState,
    pub to: RuntimeState,
    pub intent: TransitionKind,
    pub verdict: TransitionVerdict,
    pub generation: RuntimeGeneration,
    pub reason: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootstrapVerdict {
    ReadyEligible,
    DegradedEligible,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapSafetyReceipt {
    pub schema: SchemaVersion,
    pub boot_epoch: u64,
    pub runtime_identity: String,
    pub config_fingerprint: String,
    pub module_graph_fingerprint: String,
    pub capability_graph_fingerprint: String,
    pub recovery: RecoveryClassification,
    pub verdict: BootstrapVerdict,
    pub blocked_reasons: Vec<String>,
    pub unavailable_capabilities: Vec<String>,
    pub rsg_fingerprint: String,
    pub module_graph_generation: u64,
    pub capability_graph_generation: u64,
    pub policy_generation: u64,
    pub tcbm_fingerprint: String,
    pub saf_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationCoherenceBasis {
    pub runtime: RuntimeGeneration,
    pub binding_generation: u64,
    pub provider_activation_generation: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GenerationDimension {
    Boot,
    Config,
    Module,
    Capability,
    Policy,
    Binding,
    ProviderActivation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationCoherenceReceipt {
    pub schema: SchemaVersion,
    pub capability: String,
    pub coherent: bool,
    pub basis: GenerationCoherenceBasis,
    pub expected: GenerationCoherenceBasis,
    pub mismatches: BTreeSet<GenerationDimension>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedComputingBaseMap {
    pub schema: SchemaVersion,
    pub policy_version: String,
    pub dependency_lock_fingerprint: String,
    pub safety_critical_components: BTreeSet<String>,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unavailable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HealthDimension {
    ProcessLiveness,
    RuntimeReadiness,
    ModuleGraph,
    Capability,
    JournalIntegrity,
    ResourcePressure,
    ExternalDependency,
    ShutdownSafety,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthSignal {
    pub dimension: HealthDimension,
    pub state: HealthState,
    pub reason_code: String,
    pub generation: u64,
    pub impact: String,
    pub observed_at_monotonic_ms: u64,
    pub freshness_window_ms: u64,
    pub evidence_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthSnapshot {
    pub schema: SchemaVersion,
    pub baseline_fingerprint: String,
    pub signals: BTreeMap<HealthDimension, HealthSignal>,
    pub delta: BTreeMap<String, String>,
    pub pressure: ResourcePressure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryClassification {
    CleanStop,
    RecoverableInterruption,
    OrphanedResource,
    StaleExternalState,
    AmbiguousEffect,
    CorruptJournal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShutdownPhase {
    DrainRequested,
    AdmissionClosed,
    LeaseDrain,
    CooperativeCancel,
    Cleanup,
    QuiescenceCheck,
    StopCommit,
    Escalate,
    ForceTerminate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuiescenceItem {
    pub subject: String,
    pub satisfied: bool,
    pub residual: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShutdownReceipt {
    pub schema: SchemaVersion,
    pub boot_epoch: u64,
    pub clean: bool,
    pub final_phase: ShutdownPhase,
    pub items: Vec<QuiescenceItem>,
    pub residuals: Vec<String>,
    pub quiescence_fingerprint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalDurability {
    MemoryOnlyDiagnostic,
    FlushRequired,
    SyncRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalRecordKind {
    BootOpened,
    BootClosed,
    TransitionIntent,
    TransitionCommit,
    GenerationActivated,
    WorkerSpawned,
    WorkerTerminated,
    DrainStarted,
    AdmissionClosed,
    LeaseDrainStarted,
    LeaseDrainAttempted,
    LeaseDrainTimedOut,
    LeaseDrainCompleted,
    CooperativeCancelStarted,
    CooperativeCancelAttempted,
    CooperativeCancelTimedOut,
    CooperativeCancelCompleted,
    CleanupStarted,
    CleanupAttempted,
    CleanupTimedOut,
    CleanupCompleted,
    QuiescenceChecked,
    DrainCompleted,
    ShutdownEscalated,
    ForcedTermination,
    IncompleteShutdown,
    WorkerSupervisionTransition,
    RecoveryResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeJournalRecord {
    pub schema: SchemaVersion,
    pub sequence: u64,
    pub boot_epoch: u64,
    pub kind: JournalRecordKind,
    pub durability: JournalDurability,
    pub payload: BTreeMap<String, String>,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdempotencyEnvelope {
    pub schema: SchemaVersion,
    pub operation_id: String,
    pub intent_fingerprint: String,
    pub idempotency_key: String,
    pub generation: RuntimeGeneration,
    pub precondition_fingerprint: String,
    pub expected_postcondition: String,
    pub reconciliation_method: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHandle {
    pub schema: SchemaVersion,
    pub digest: String,
    pub byte_len: u64,
    pub locator: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenEconomicsBudget {
    pub input_limit: u64,
    pub output_limit: u64,
    pub retry_limit: u64,
    pub quality_floor: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheEfficiencyEnvelope {
    pub stable_prefix_ratio_milli: u16,
    pub exact_hits: u64,
    pub evidence_hits: u64,
    pub semantic_hits: u64,
    pub bypasses: u64,
    pub invalidations: u64,
}
