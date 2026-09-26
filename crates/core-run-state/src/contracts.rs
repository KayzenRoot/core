//! Public, versioned M04 value contracts.
//!
//! These declarations intentionally contain no lifecycle, replay, persistence,
//! cancellation, or policy behavior. Those mechanisms belong to later packets.

use crate::{
    AttemptId, AttemptOrdinalV1, CanonicalFingerprint, EventId, EventSequenceV1, ExecutionEpoch,
    FingerprintDomainV1, IdempotencyKey, JournalRoot, M04ErrorClassV1, M04ErrorCodeV1, M04ErrorV1,
    M04RetryabilityV1, RunGeneration, RunId, StepId, StepOrdinalV1,
};
use core_work_order::{AdmittedWorkOrderV1, RunStartRevalidationV1, WorkOrderIdentityRefV1};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

/// Stable closed registry of serialized V1 contract kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContractKindV1 {
    RunAdmissionRequest,
    RunAdmissionReceipt,
    CreateAttemptRequest,
    AttemptReceipt,
    DeclareStepRequest,
    StepReceipt,
    TransitionRequest,
    TransitionReceipt,
    CancelRunRequest,
    CancellationReceipt,
    CreateContinuationRequest,
    ContinuationReceipt,
    ReplayRequest,
    ReplayProjection,
    AttachReferenceRequest,
    ReferenceReceipt,
    CanonicalEvent,
    RunSnapshot,
    ExternalReferenceEvidence,
    ResourceLimits,
    BoundaryRevalidationCapsule,
    ContinuationFrame,
    Error,
}

impl ContractKindV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunAdmissionRequest => "RUN_ADMISSION_REQUEST",
            Self::RunAdmissionReceipt => "RUN_ADMISSION_RECEIPT",
            Self::CreateAttemptRequest => "CREATE_ATTEMPT_REQUEST",
            Self::AttemptReceipt => "ATTEMPT_RECEIPT",
            Self::DeclareStepRequest => "DECLARE_STEP_REQUEST",
            Self::StepReceipt => "STEP_RECEIPT",
            Self::TransitionRequest => "TRANSITION_REQUEST",
            Self::TransitionReceipt => "TRANSITION_RECEIPT",
            Self::CancelRunRequest => "CANCEL_RUN_REQUEST",
            Self::CancellationReceipt => "CANCELLATION_RECEIPT",
            Self::CreateContinuationRequest => "CREATE_CONTINUATION_REQUEST",
            Self::ContinuationReceipt => "CONTINUATION_RECEIPT",
            Self::ReplayRequest => "REPLAY_REQUEST",
            Self::ReplayProjection => "REPLAY_PROJECTION",
            Self::AttachReferenceRequest => "ATTACH_REFERENCE_REQUEST",
            Self::ReferenceReceipt => "REFERENCE_RECEIPT",
            Self::CanonicalEvent => "CANONICAL_EVENT",
            Self::RunSnapshot => "RUN_SNAPSHOT",
            Self::ExternalReferenceEvidence => "EXTERNAL_REFERENCE_EVIDENCE",
            Self::ResourceLimits => "RESOURCE_LIMITS",
            Self::BoundaryRevalidationCapsule => "BOUNDARY_REVALIDATION_CAPSULE",
            Self::ContinuationFrame => "CONTINUATION_FRAME",
            Self::Error => "ERROR",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "RUN_ADMISSION_REQUEST" => Self::RunAdmissionRequest,
            "RUN_ADMISSION_RECEIPT" => Self::RunAdmissionReceipt,
            "CREATE_ATTEMPT_REQUEST" => Self::CreateAttemptRequest,
            "ATTEMPT_RECEIPT" => Self::AttemptReceipt,
            "DECLARE_STEP_REQUEST" => Self::DeclareStepRequest,
            "STEP_RECEIPT" => Self::StepReceipt,
            "TRANSITION_REQUEST" => Self::TransitionRequest,
            "TRANSITION_RECEIPT" => Self::TransitionReceipt,
            "CANCEL_RUN_REQUEST" => Self::CancelRunRequest,
            "CANCELLATION_RECEIPT" => Self::CancellationReceipt,
            "CREATE_CONTINUATION_REQUEST" => Self::CreateContinuationRequest,
            "CONTINUATION_RECEIPT" => Self::ContinuationReceipt,
            "REPLAY_REQUEST" => Self::ReplayRequest,
            "REPLAY_PROJECTION" => Self::ReplayProjection,
            "ATTACH_REFERENCE_REQUEST" => Self::AttachReferenceRequest,
            "REFERENCE_RECEIPT" => Self::ReferenceReceipt,
            "CANONICAL_EVENT" => Self::CanonicalEvent,
            "RUN_SNAPSHOT" => Self::RunSnapshot,
            "EXTERNAL_REFERENCE_EVIDENCE" => Self::ExternalReferenceEvidence,
            "RESOURCE_LIMITS" => Self::ResourceLimits,
            "BOUNDARY_REVALIDATION_CAPSULE" => Self::BoundaryRevalidationCapsule,
            "CONTINUATION_FRAME" => Self::ContinuationFrame,
            "ERROR" => Self::Error,
            _ => return None,
        })
    }
}

impl Serialize for ContractKindV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ContractKindV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).ok_or_else(|| de::Error::custom("unsupported M04 contract kind"))
    }
}

/// Validated M04 schema metadata used by every versioned V1 contract.
///
/// The inner value is deliberately private: public contract structs may expose
/// this field, but callers cannot use a struct literal to attach another
/// schema and still obtain a V1 value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M04SchemaV1(());

impl M04SchemaV1 {
    pub fn new(value: impl AsRef<str>) -> Result<Self, M04ErrorV1> {
        validate_v1_contract_header(value.as_ref(), crate::M04_VERSION)?;
        Ok(Self(()))
    }

    pub const fn as_str(self) -> &'static str {
        crate::M04_SCHEMA
    }

    pub(crate) const fn supported() -> Self {
        Self(())
    }
}

impl Serialize for M04SchemaV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(crate::M04_SCHEMA)
    }
}

impl<'de> Deserialize<'de> for M04SchemaV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

/// Validated M04 version metadata used by every versioned V1 contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M04VersionV1(());

impl M04VersionV1 {
    pub fn new(value: u16) -> Result<Self, M04ErrorV1> {
        validate_v1_contract_header(crate::M04_SCHEMA, value)?;
        Ok(Self(()))
    }

    pub const fn get(self) -> u16 {
        crate::M04_VERSION
    }

    pub(crate) const fn supported() -> Self {
        Self(())
    }
}

impl Serialize for M04VersionV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(crate::M04_VERSION)
    }
}

impl<'de> Deserialize<'de> for M04VersionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u16::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

/// Validate raw schema/version metadata before accepting it as any M04 V1
/// contract. Deserializers and public typed constructors share this check.
pub fn validate_v1_contract_header(schema: &str, version: u16) -> Result<(), M04ErrorV1> {
    if schema != crate::M04_SCHEMA {
        return Err(M04ErrorV1::unsupported_schema());
    }
    if version != crate::M04_VERSION {
        return Err(M04ErrorV1::unsupported_version());
    }
    Ok(())
}

fn validate_fingerprint_domain(
    domain: FingerprintDomainV1,
    expected: FingerprintDomainV1,
) -> Result<(), M04ErrorV1> {
    if domain == expected {
        return Ok(());
    }

    Err(M04ErrorV1::new(
        M04ErrorClassV1::InvalidInput,
        M04ErrorCodeV1::InvalidInput,
        M04RetryabilityV1::Never,
    ))
}

mod sealed {
    pub trait Contract {}
}

/// Implemented only by the closed set of M04 payload contracts.
pub trait M04ContractV1: sealed::Contract + Serialize + for<'de> Deserialize<'de> {
    const KIND: ContractKindV1;
}

/// A typed envelope whose schema, version, and kind are fixed by its payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M04EnvelopeV1<T: M04ContractV1> {
    schema: String,
    version: u16,
    kind: ContractKindV1,
    payload: T,
}

impl<T: M04ContractV1> M04EnvelopeV1<T> {
    pub fn new(payload: T) -> Self {
        Self {
            schema: crate::M04_SCHEMA.to_owned(),
            version: crate::M04_VERSION,
            kind: T::KIND,
            payload,
        }
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub const fn version(&self) -> u16 {
        self.version
    }

    pub const fn kind(&self) -> ContractKindV1 {
        self.kind
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

/// Untrusted envelope metadata kept raw until [`RawM04EnvelopeV1::validate`].
///
/// This boundary type is not a validated M04 V1 contract. Its raw schema and
/// version fields intentionally remain readable so validation can return the
/// corresponding typed error before converting it into [`M04EnvelopeV1`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawM04EnvelopeV1<T> {
    pub schema: String,
    pub version: u16,
    pub kind: String,
    pub payload: T,
}

impl<T: M04ContractV1> RawM04EnvelopeV1<T> {
    pub fn validate(self) -> Result<M04EnvelopeV1<T>, crate::M04ErrorV1> {
        if self.schema != crate::M04_SCHEMA {
            return Err(crate::M04ErrorV1::unsupported_schema());
        }
        if self.version != crate::M04_VERSION {
            return Err(crate::M04ErrorV1::unsupported_version());
        }
        let kind =
            ContractKindV1::parse(&self.kind).ok_or_else(crate::M04ErrorV1::unsupported_kind)?;
        if kind != T::KIND {
            return Err(crate::M04ErrorV1::kind_payload_mismatch());
        }
        Ok(M04EnvelopeV1 {
            schema: self.schema,
            version: self.version,
            kind,
            payload: self.payload,
        })
    }
}

impl<'de, T: M04ContractV1> Deserialize<'de> for M04EnvelopeV1<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawM04EnvelopeV1::<T>::deserialize(deserializer)?
            .validate()
            .map_err(de::Error::custom)
    }
}

/// Closed Run lifecycle state vocabulary. Legal edges are defined in Pack B.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunStatusV1 {
    Created,
    Admitted,
    Active,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
    Interrupted,
}

/// Closed Attempt lifecycle state vocabulary. Legal edges are defined in Pack B.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttemptStatusV1 {
    Created,
    Active,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
    Interrupted,
}

/// Closed Step lifecycle state vocabulary. Legal edges are defined in Pack B.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StepStatusV1 {
    Declared,
    Ready,
    Active,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
    Skipped,
    Interrupted,
}

/// Closed event registry. Values and discriminants are frozen for V1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventKindV1 {
    RunCreated = 1,
    RunAdmitted = 2,
    RunTransitioned = 3,
    RunCancellationAccepted = 4,
    AttemptCreated = 5,
    AttemptTransitioned = 6,
    StepDeclared = 7,
    StepTransitioned = 8,
    ContinuationCreated = 9,
    ReferenceAttached = 10,
}

impl EventKindV1 {
    pub const fn code(self) -> u16 {
        self as u16
    }
}

/// Closed reason namespace classes reserved by the promoted M04 contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum M04ReasonClassV1 {
    CallerCancellation,
    RuntimeShutdown,
    StaleAuthority,
    InvalidLineage,
    ResourceCap,
    ExternalReferenceRejection,
    ExecutionOutcome,
    ExplicitAuthorizedSkip,
    InvariantProtection,
}

/// Closed, machine-readable V1 reason codes; diagnostics never replace these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum M04ReasonCodeV1 {
    CallerCancellation = 1,
    RuntimeShutdown = 2,
    StaleAuthority = 3,
    InvalidLineage = 4,
    ResourceCap = 5,
    ExternalReferenceRejected = 6,
    ExecutionOutcome = 7,
    ExplicitAuthorizedSkip = 8,
    InvariantProtection = 9,
}

impl M04ReasonCodeV1 {
    pub const fn code(self) -> u16 {
        self as u16
    }

    pub const fn class(self) -> M04ReasonClassV1 {
        match self {
            Self::CallerCancellation => M04ReasonClassV1::CallerCancellation,
            Self::RuntimeShutdown => M04ReasonClassV1::RuntimeShutdown,
            Self::StaleAuthority => M04ReasonClassV1::StaleAuthority,
            Self::InvalidLineage => M04ReasonClassV1::InvalidLineage,
            Self::ResourceCap => M04ReasonClassV1::ResourceCap,
            Self::ExternalReferenceRejected => M04ReasonClassV1::ExternalReferenceRejection,
            Self::ExecutionOutcome => M04ReasonClassV1::ExecutionOutcome,
            Self::ExplicitAuthorizedSkip => M04ReasonClassV1::ExplicitAuthorizedSkip,
            Self::InvariantProtection => M04ReasonClassV1::InvariantProtection,
        }
    }
}

/// Operation domains scope caller-provided idempotency keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum M04OperationDomainV1 {
    RunAdmission = 1,
    AttemptCreation = 2,
    StepDeclaration = 3,
    Transition = 4,
    Cancellation = 5,
    Continuation = 6,
    ReferenceAttachment = 7,
}

impl M04OperationDomainV1 {
    pub const fn code(self) -> u16 {
        self as u16
    }
}

/// Canonical semantic key for one operation-scoped idempotency record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotencyRecordKeyV1 {
    pub run_id: RunId,
    pub domain: M04OperationDomainV1,
    pub key: IdempotencyKey,
}

/// Recorded semantic result metadata used by later idempotency behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotencyRecordV1 {
    pub key: IdempotencyRecordKeyV1,
    pub request_fingerprint: CanonicalFingerprint,
    pub event_sequence: EventSequenceV1,
    pub resulting_generation: RunGeneration,
    pub resulting_journal_root: JournalRoot,
}

/// Versioned proof of M03 admission authority captured at a Run boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryRevalidationCapsuleV1 {
    pub schema: M04SchemaV1,
    pub version: M04VersionV1,
    pub admitted_work_order: AdmittedWorkOrderV1,
    pub work_order_identity: WorkOrderIdentityRefV1,
    pub revalidation: RunStartRevalidationV1,
    pub execution_epoch: ExecutionEpoch,
    pub capsule_fingerprint: CanonicalFingerprint,
}

/// Versioned, host-neutral continuation cursor. It carries no recovery policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationFrameV1 {
    pub schema: M04SchemaV1,
    pub version: M04VersionV1,
    domain: FingerprintDomainV1,
    pub run_id: RunId,
    pub source_attempt_id: AttemptId,
    pub last_committed_generation: RunGeneration,
    pub last_event_sequence: EventSequenceV1,
    pub journal_root: JournalRoot,
    pub boundary_fingerprint: CanonicalFingerprint,
    pub execution_epoch: ExecutionEpoch,
    pub cursor_fingerprint: CanonicalFingerprint,
}

impl ContinuationFrameV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        domain: FingerprintDomainV1,
        run_id: RunId,
        source_attempt_id: AttemptId,
        last_committed_generation: RunGeneration,
        last_event_sequence: EventSequenceV1,
        journal_root: JournalRoot,
        boundary_fingerprint: CanonicalFingerprint,
        execution_epoch: ExecutionEpoch,
        cursor_fingerprint: CanonicalFingerprint,
    ) -> Result<Self, M04ErrorV1> {
        validate_fingerprint_domain(domain, FingerprintDomainV1::Continuation)?;
        Ok(Self {
            schema: M04SchemaV1::supported(),
            version: M04VersionV1::supported(),
            domain,
            run_id,
            source_attempt_id,
            last_committed_generation,
            last_event_sequence,
            journal_root,
            boundary_fingerprint,
            execution_epoch,
            cursor_fingerprint,
        })
    }

    pub const fn domain(&self) -> FingerprintDomainV1 {
        self.domain
    }

    pub fn validate(&self) -> Result<(), M04ErrorV1> {
        validate_v1_contract_header(self.schema.as_str(), self.version.get())?;
        validate_fingerprint_domain(self.domain, FingerprintDomainV1::Continuation)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContinuationFrameV1Wire {
    schema: M04SchemaV1,
    version: M04VersionV1,
    domain: FingerprintDomainV1,
    run_id: RunId,
    source_attempt_id: AttemptId,
    last_committed_generation: RunGeneration,
    last_event_sequence: EventSequenceV1,
    journal_root: JournalRoot,
    boundary_fingerprint: CanonicalFingerprint,
    execution_epoch: ExecutionEpoch,
    cursor_fingerprint: CanonicalFingerprint,
}

impl<'de> Deserialize<'de> for ContinuationFrameV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ContinuationFrameV1Wire::deserialize(deserializer)?;
        validate_v1_contract_header(wire.schema.as_str(), wire.version.get())
            .map_err(de::Error::custom)?;
        validate_fingerprint_domain(wire.domain, FingerprintDomainV1::Continuation)
            .map_err(de::Error::custom)?;
        Ok(Self {
            schema: wire.schema,
            version: wire.version,
            domain: wire.domain,
            run_id: wire.run_id,
            source_attempt_id: wire.source_attempt_id,
            last_committed_generation: wire.last_committed_generation,
            last_event_sequence: wire.last_event_sequence,
            journal_root: wire.journal_root,
            boundary_fingerprint: wire.boundary_fingerprint,
            execution_epoch: wire.execution_epoch,
            cursor_fingerprint: wire.cursor_fingerprint,
        })
    }
}

/// Closed target union for transition requests; Pack B supplies transition laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename_all = "SCREAMING_SNAKE_CASE",
    tag = "entity",
    content = "status"
)]
pub enum TransitionTargetV1 {
    Run(RunStatusV1),
    Attempt(AttemptStatusV1),
    Step(StepStatusV1),
}

/// Typed event payload variants. No arbitrary event name or opaque body exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "kind", content = "data")]
pub enum EventPayloadV1 {
    RunCreated {
        work_order: WorkOrderIdentityRefV1,
        boundary_fingerprint: CanonicalFingerprint,
    },
    RunAdmitted {
        boundary_fingerprint: CanonicalFingerprint,
    },
    RunTransitioned {
        from: RunStatusV1,
        to: RunStatusV1,
        reason: Option<M04ReasonCodeV1>,
    },
    RunCancellationAccepted {
        cancellation_epoch: u64,
        reason: M04ReasonCodeV1,
    },
    AttemptCreated {
        attempt_id: AttemptId,
        ordinal: AttemptOrdinalV1,
    },
    AttemptTransitioned {
        attempt_id: AttemptId,
        from: AttemptStatusV1,
        to: AttemptStatusV1,
        reason: Option<M04ReasonCodeV1>,
    },
    StepDeclared {
        attempt_id: AttemptId,
        step_id: StepId,
        ordinal: StepOrdinalV1,
    },
    StepTransitioned {
        attempt_id: AttemptId,
        step_id: StepId,
        from: StepStatusV1,
        to: StepStatusV1,
        reason: Option<M04ReasonCodeV1>,
    },
    ContinuationCreated {
        source_attempt_id: AttemptId,
        attempt_id: AttemptId,
        execution_epoch: ExecutionEpoch,
        boundary_fingerprint: CanonicalFingerprint,
    },
    ReferenceAttached {
        reference: ExternalReferenceEvidenceV1,
    },
}

/// One canonical event envelope. Optional diagnostic timestamps are omitted by design.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalEventV1 {
    pub schema: M04SchemaV1,
    pub version: M04VersionV1,
    domain: FingerprintDomainV1,
    event_kind: EventKindV1,
    pub event_id: EventId,
    pub run_id: RunId,
    pub attempt_id: Option<AttemptId>,
    pub step_id: Option<StepId>,
    pub event_sequence: EventSequenceV1,
    pub expected_generation: RunGeneration,
    pub resulting_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub payload_fingerprint: CanonicalFingerprint,
    pub prior_journal_root: JournalRoot,
    pub resulting_journal_root: JournalRoot,
    payload: EventPayloadV1,
}

impl CanonicalEventV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        domain: FingerprintDomainV1,
        event_kind: EventKindV1,
        event_id: EventId,
        run_id: RunId,
        attempt_id: Option<AttemptId>,
        step_id: Option<StepId>,
        event_sequence: EventSequenceV1,
        expected_generation: RunGeneration,
        resulting_generation: RunGeneration,
        idempotency_key: IdempotencyKey,
        payload_fingerprint: CanonicalFingerprint,
        prior_journal_root: JournalRoot,
        resulting_journal_root: JournalRoot,
        payload: EventPayloadV1,
    ) -> Result<Self, M04ErrorV1> {
        validate_fingerprint_domain(domain, FingerprintDomainV1::Event)?;
        validate_event_kind_payload(event_kind, &payload)?;
        validate_event_lineage(
            &run_id,
            attempt_id.as_ref(),
            step_id.as_ref(),
            &payload,
        )?;
        Ok(Self {
            schema: M04SchemaV1::supported(),
            version: M04VersionV1::supported(),
            domain,
            event_kind,
            event_id,
            run_id,
            attempt_id,
            step_id,
            event_sequence,
            expected_generation,
            resulting_generation,
            idempotency_key,
            payload_fingerprint,
            prior_journal_root,
            resulting_journal_root,
            payload,
        })
    }

    pub const fn event_kind(&self) -> EventKindV1 {
        self.event_kind
    }

    pub const fn domain(&self) -> FingerprintDomainV1 {
        self.domain
    }

    pub fn payload(&self) -> &EventPayloadV1 {
        &self.payload
    }

    pub fn validate(&self) -> Result<(), M04ErrorV1> {
        validate_v1_contract_header(self.schema.as_str(), self.version.get())?;
        validate_fingerprint_domain(self.domain, FingerprintDomainV1::Event)?;
        validate_event_kind_payload(self.event_kind, &self.payload)?;
        validate_event_lineage(
            &self.run_id,
            self.attempt_id.as_ref(),
            self.step_id.as_ref(),
            &self.payload,
        )
    }
}

fn validate_event_kind_payload(
    event_kind: EventKindV1,
    payload: &EventPayloadV1,
) -> Result<(), M04ErrorV1> {
    match (event_kind, payload) {
        (EventKindV1::RunCreated, EventPayloadV1::RunCreated { .. })
        | (EventKindV1::RunAdmitted, EventPayloadV1::RunAdmitted { .. })
        | (EventKindV1::RunTransitioned, EventPayloadV1::RunTransitioned { .. })
        | (EventKindV1::RunCancellationAccepted, EventPayloadV1::RunCancellationAccepted { .. })
        | (EventKindV1::AttemptCreated, EventPayloadV1::AttemptCreated { .. })
        | (EventKindV1::AttemptTransitioned, EventPayloadV1::AttemptTransitioned { .. })
        | (EventKindV1::StepDeclared, EventPayloadV1::StepDeclared { .. })
        | (EventKindV1::StepTransitioned, EventPayloadV1::StepTransitioned { .. })
        | (EventKindV1::ContinuationCreated, EventPayloadV1::ContinuationCreated { .. })
        | (EventKindV1::ReferenceAttached, EventPayloadV1::ReferenceAttached { .. }) => Ok(()),
        _ => Err(M04ErrorV1::kind_payload_mismatch()),
    }
}

fn event_lineage_mismatch() -> M04ErrorV1 {
    M04ErrorV1::new(
        M04ErrorClassV1::LineageMismatch,
        M04ErrorCodeV1::LineageMismatch,
        M04RetryabilityV1::Never,
    )
}

fn validate_event_lineage(
    run_id: &RunId,
    attempt_id: Option<&AttemptId>,
    step_id: Option<&StepId>,
    payload: &EventPayloadV1,
) -> Result<(), M04ErrorV1> {
    let matches_payload = match payload {
        EventPayloadV1::RunCreated { .. }
        | EventPayloadV1::RunAdmitted { .. }
        | EventPayloadV1::RunTransitioned { .. }
        | EventPayloadV1::RunCancellationAccepted { .. } => {
            attempt_id.is_none() && step_id.is_none()
        }
        EventPayloadV1::AttemptCreated {
            attempt_id: payload_attempt_id,
            ..
        }
        | EventPayloadV1::AttemptTransitioned {
            attempt_id: payload_attempt_id,
            ..
        } => attempt_id == Some(payload_attempt_id) && step_id.is_none(),
        EventPayloadV1::StepDeclared {
            attempt_id: payload_attempt_id,
            step_id: payload_step_id,
            ..
        }
        | EventPayloadV1::StepTransitioned {
            attempt_id: payload_attempt_id,
            step_id: payload_step_id,
            ..
        } => attempt_id == Some(payload_attempt_id) && step_id == Some(payload_step_id),
        EventPayloadV1::ContinuationCreated {
            attempt_id: created_attempt_id,
            ..
        } => attempt_id == Some(created_attempt_id) && step_id.is_none(),
        EventPayloadV1::ReferenceAttached { reference } => {
            run_id == &reference.run_id
                && attempt_id == reference.attempt_id.as_ref()
                && step_id == reference.step_id.as_ref()
        }
    };

    if matches_payload {
        Ok(())
    } else {
        Err(event_lineage_mismatch())
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CanonicalEventV1Wire {
    schema: M04SchemaV1,
    version: M04VersionV1,
    domain: FingerprintDomainV1,
    event_kind: EventKindV1,
    event_id: EventId,
    run_id: RunId,
    attempt_id: Option<AttemptId>,
    step_id: Option<StepId>,
    event_sequence: EventSequenceV1,
    expected_generation: RunGeneration,
    resulting_generation: RunGeneration,
    idempotency_key: IdempotencyKey,
    payload_fingerprint: CanonicalFingerprint,
    prior_journal_root: JournalRoot,
    resulting_journal_root: JournalRoot,
    payload: EventPayloadV1,
}

impl<'de> Deserialize<'de> for CanonicalEventV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CanonicalEventV1Wire::deserialize(deserializer)?;
        validate_v1_contract_header(wire.schema.as_str(), wire.version.get())
            .map_err(de::Error::custom)?;
        validate_fingerprint_domain(wire.domain, FingerprintDomainV1::Event)
            .map_err(de::Error::custom)?;
        validate_event_kind_payload(wire.event_kind, &wire.payload).map_err(de::Error::custom)?;
        validate_event_lineage(
            &wire.run_id,
            wire.attempt_id.as_ref(),
            wire.step_id.as_ref(),
            &wire.payload,
        )
        .map_err(de::Error::custom)?;
        Ok(Self {
            schema: wire.schema,
            version: wire.version,
            domain: wire.domain,
            event_kind: wire.event_kind,
            event_id: wire.event_id,
            run_id: wire.run_id,
            attempt_id: wire.attempt_id,
            step_id: wire.step_id,
            event_sequence: wire.event_sequence,
            expected_generation: wire.expected_generation,
            resulting_generation: wire.resulting_generation,
            idempotency_key: wire.idempotency_key,
            payload_fingerprint: wire.payload_fingerprint,
            prior_journal_root: wire.prior_journal_root,
            resulting_journal_root: wire.resulting_journal_root,
            payload: wire.payload,
        })
    }
}

/// Verified journal boundary metadata used by replay and snapshot contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JournalBoundaryV1 {
    pub run_id: RunId,
    pub generation: RunGeneration,
    pub last_event_sequence: EventSequenceV1,
    pub journal_root: JournalRoot,
    pub boundary_fingerprint: CanonicalFingerprint,
}

/// Derived Step state. The journal remains authoritative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepProjectionV1 {
    pub step_id: StepId,
    pub ordinal: StepOrdinalV1,
    pub status: StepStatusV1,
    pub reason: Option<M04ReasonCodeV1>,
}

/// Derived Attempt state. The journal remains authoritative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptProjectionV1 {
    pub attempt_id: AttemptId,
    pub ordinal: AttemptOrdinalV1,
    pub status: AttemptStatusV1,
    pub steps: BTreeMap<StepId, StepProjectionV1>,
}

/// Derived Run state. It cannot supersede the canonical journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunProjectionV1 {
    pub run_id: RunId,
    pub status: RunStatusV1,
    pub generation: RunGeneration,
    pub last_event_sequence: EventSequenceV1,
    pub journal_root: JournalRoot,
    pub boundary: BoundaryRevalidationCapsuleV1,
    pub attempts: BTreeMap<AttemptId, AttemptProjectionV1>,
    pub idempotency_records: BTreeMap<IdempotencyRecordKeyV1, IdempotencyRecordV1>,
}

/// Projection returned by deterministic replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayProjectionV1 {
    pub boundary: JournalBoundaryV1,
    pub projection: RunProjectionV1,
}

/// Versioned journal-bound snapshot of a derived Run projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSnapshotV1 {
    pub schema: M04SchemaV1,
    pub version: M04VersionV1,
    pub run_id: RunId,
    pub source_generation: RunGeneration,
    pub last_event_sequence: EventSequenceV1,
    pub journal_root: JournalRoot,
    pub projection_fingerprint: CanonicalFingerprint,
    pub projection: RunProjectionV1,
}

/// Reference domains are closed at the M04 boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalReferenceOwnerV1 {
    Execution,
    Mutation,
    Verification,
    Review,
}

/// Reference kinds carry metadata only; artifact bodies remain caller-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalReferenceKindV1 {
    Outcome,
    Evidence,
    Artifact,
    ReviewRecord,
}

/// Bounded caller-owned evidence attached to a Run/Attempt/Step lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalReferenceEvidenceV1 {
    pub schema: M04SchemaV1,
    pub version: M04VersionV1,
    pub owner: ExternalReferenceOwnerV1,
    pub kind: ExternalReferenceKindV1,
    pub immutable_locator: String,
    pub immutable_identity: String,
    pub content_fingerprint: Option<CanonicalFingerprint>,
    pub run_id: RunId,
    pub attempt_id: Option<AttemptId>,
    pub step_id: Option<StepId>,
}

/// Finite caller-selected resource caps; Pack F defines admission behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M04ResourceLimitsV1 {
    pub attempts_per_run: u64,
    pub steps_per_attempt: u64,
    pub events_per_run: u64,
    pub event_canonical_bytes: u64,
    pub diagnostic_bytes: u64,
    pub continuation_cursor_bytes: u64,
    pub references_per_run: u64,
    pub reference_bytes_per_run: u64,
    pub replay_depth: u64,
}

/// Operation-specific receipt metadata shared by all durable receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommitPositionV1 {
    pub run_id: RunId,
    pub generation: RunGeneration,
    pub event_sequence: EventSequenceV1,
    pub journal_root: JournalRoot,
}

macro_rules! receipt_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            pub position: CommitPositionV1,
        }
    };
}

receipt_type!(RunAdmissionReceiptV1);
receipt_type!(AttemptReceiptV1);
receipt_type!(StepReceiptV1);
receipt_type!(TransitionReceiptV1);
receipt_type!(CancellationReceiptV1);
receipt_type!(ContinuationReceiptV1);
receipt_type!(ReferenceReceiptV1);

/// Exact prepared semantic result awaiting host-owned durable commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedCommitV1<R> {
    pub run_id: RunId,
    pub expected_generation: RunGeneration,
    pub resulting_generation: RunGeneration,
    pub prior_journal_root: JournalRoot,
    pub operation_fingerprint: CanonicalFingerprint,
    pub event_sequence: EventSequenceV1,
    pub resulting_journal_root: JournalRoot,
    pub event: CanonicalEventV1,
    pub projection: RunProjectionV1,
    pub pending_receipt: R,
}

/// Exact durable-store acknowledgment required before a pending receipt is authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DurableCommitReceiptV1 {
    pub run_id: RunId,
    pub operation_fingerprint: CanonicalFingerprint,
    pub expected_generation: RunGeneration,
    pub resulting_generation: RunGeneration,
    pub event_sequence: EventSequenceV1,
    pub resulting_journal_root: JournalRoot,
}

/// Requests carry all semantic and idempotency bindings as explicit values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunAdmissionRequestV1 {
    pub run_id: RunId,
    pub admitted_work_order: AdmittedWorkOrderV1,
    pub expected_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateAttemptRequestV1 {
    pub run_id: RunId,
    pub attempt_id: AttemptId,
    pub expected_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclareStepRequestV1 {
    pub run_id: RunId,
    pub attempt_id: AttemptId,
    pub step_id: StepId,
    pub expected_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionRequestV1 {
    pub run_id: RunId,
    pub attempt_id: Option<AttemptId>,
    pub step_id: Option<StepId>,
    pub target: TransitionTargetV1,
    pub reason: Option<M04ReasonCodeV1>,
    pub expected_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelRunRequestV1 {
    pub run_id: RunId,
    pub expected_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
    pub reason: M04ReasonCodeV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateContinuationRequestV1 {
    pub run_id: RunId,
    pub source_attempt_id: AttemptId,
    pub attempt_id: AttemptId,
    pub expected_generation: RunGeneration,
    pub continuation: ContinuationFrameV1,
    pub boundary: BoundaryRevalidationCapsuleV1,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayRequestV1 {
    pub run_id: RunId,
    pub events: Vec<CanonicalEventV1>,
    pub expected_boundary: Option<JournalBoundaryV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttachReferenceRequestV1 {
    pub run_id: RunId,
    pub expected_generation: RunGeneration,
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: CanonicalFingerprint,
    pub reference: ExternalReferenceEvidenceV1,
}

macro_rules! impl_contract {
    ($type:ty, $kind:ident) => {
        impl sealed::Contract for $type {}
        impl M04ContractV1 for $type {
            const KIND: ContractKindV1 = ContractKindV1::$kind;
        }
    };
}

impl_contract!(RunAdmissionRequestV1, RunAdmissionRequest);
impl_contract!(RunAdmissionReceiptV1, RunAdmissionReceipt);
impl_contract!(CreateAttemptRequestV1, CreateAttemptRequest);
impl_contract!(AttemptReceiptV1, AttemptReceipt);
impl_contract!(DeclareStepRequestV1, DeclareStepRequest);
impl_contract!(StepReceiptV1, StepReceipt);
impl_contract!(TransitionRequestV1, TransitionRequest);
impl_contract!(TransitionReceiptV1, TransitionReceipt);
impl_contract!(CancelRunRequestV1, CancelRunRequest);
impl_contract!(CancellationReceiptV1, CancellationReceipt);
impl_contract!(CreateContinuationRequestV1, CreateContinuationRequest);
impl_contract!(ContinuationReceiptV1, ContinuationReceipt);
impl_contract!(ReplayRequestV1, ReplayRequest);
impl_contract!(ReplayProjectionV1, ReplayProjection);
impl_contract!(AttachReferenceRequestV1, AttachReferenceRequest);
impl_contract!(ReferenceReceiptV1, ReferenceReceipt);
impl_contract!(CanonicalEventV1, CanonicalEvent);
impl_contract!(RunSnapshotV1, RunSnapshot);
impl_contract!(ExternalReferenceEvidenceV1, ExternalReferenceEvidence);
impl_contract!(M04ResourceLimitsV1, ResourceLimits);
impl_contract!(BoundaryRevalidationCapsuleV1, BoundaryRevalidationCapsule);
impl_contract!(ContinuationFrameV1, ContinuationFrame);
impl_contract!(crate::M04ErrorV1, Error);

#[cfg(test)]
mod domain_validation_tests {
    use super::*;

    fn fingerprint(value: char) -> CanonicalFingerprint {
        CanonicalFingerprint::new(value.to_string().repeat(64)).expect("valid fingerprint")
    }

    #[test]
    fn validate_rejects_a_mutated_event_domain() {
        let mut event = CanonicalEventV1::try_new(
            FingerprintDomainV1::Event,
            EventKindV1::RunAdmitted,
            EventId::new("event-1").expect("valid event id"),
            RunId::new("run-1").expect("valid run id"),
            None,
            None,
            EventSequenceV1::new(1),
            RunGeneration::new(0),
            RunGeneration::new(1),
            IdempotencyKey::new("event:1").expect("valid idempotency key"),
            fingerprint('a'),
            JournalRoot::new("b".repeat(64)).expect("valid journal root"),
            JournalRoot::new("c".repeat(64)).expect("valid journal root"),
            EventPayloadV1::RunAdmitted {
                boundary_fingerprint: fingerprint('d'),
            },
        )
        .expect("event domain must be accepted");

        event.domain = FingerprintDomainV1::Request;
        assert_eq!(
            event
                .validate()
                .expect_err("wrong event domain must fail")
                .code,
            M04ErrorCodeV1::InvalidInput
        );
    }

    #[test]
    fn validate_rejects_a_mutated_continuation_domain() {
        let mut continuation = ContinuationFrameV1::try_new(
            FingerprintDomainV1::Continuation,
            RunId::new("run-1").expect("valid run id"),
            AttemptId::new("attempt-1").expect("valid attempt id"),
            RunGeneration::new(1),
            EventSequenceV1::new(1),
            JournalRoot::new("b".repeat(64)).expect("valid journal root"),
            fingerprint('c'),
            ExecutionEpoch::new(1),
            fingerprint('d'),
        )
        .expect("continuation domain must be accepted");

        continuation.domain = FingerprintDomainV1::Event;
        assert_eq!(
            continuation
                .validate()
                .expect_err("wrong continuation domain must fail")
                .code,
            M04ErrorCodeV1::InvalidInput
        );
    }
}
