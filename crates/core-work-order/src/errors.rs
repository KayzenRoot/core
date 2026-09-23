use serde::{de, Deserialize, Deserializer, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderErrorCategoryV1 {
    SchemaVersion,
    SourceProvenance,
    ScopeDelta,
    PacketGraph,
    AcceptanceEvidence,
    Lineage,
    AdmissionStaleness,
    Resource,
    InternalInvariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderErrorCodeV1 {
    UnsupportedSchema,
    UnsupportedVersion,
    InvalidKind,
    InvalidEnvelope,
    InvalidId,
    InvalidRevision,
    InvalidEnumValue,
    NonCanonicalInput,
    SourceMissing,
    SourceStale,
    SourceSubstituted,
    SourceAuthorityMismatch,
    SourceEvidenceUnknown,
    WorkspaceMismatch,
    BasisIncompatible,
    ContextLockStale,
    GovernanceProofMissing,
    GovernanceProofMismatch,
    PolicyStale,
    ReceiptReplay,
    UnknownNotAdmissible,
    AmbiguousScope,
    DenyOverridesAllow,
    PacketScopeWidening,
    ForbiddenDelta,
    DependencyAdmissionRequired,
    DuplicateId,
    DanglingReference,
    Cycle,
    GraphLimitExceeded,
    NondeterministicOrder,
    AcceptanceGap,
    EvidenceGap,
    UnexplainedOrphan,
    SnapshotStale,
    RevisionNotNext,
    SupersededRevision,
    LineageConflict,
    LpcMismatch,
    RequestTooLarge,
    SerializedSizeExceeded,
    CardinalityLimitExceeded,
    ContextLimitExceeded,
    InvalidStopCondition,
    InvalidContextBudget,
    NondeterministicCompilation,
    FingerprintMismatch,
    PartialOutputForbidden,
    InternalInvariantViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryabilityV1 {
    Never,
    AfterExplicitRefresh,
    AfterGovernance,
    AfterResourceChange,
    InternalBug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeSubjectKindV1 {
    WorkOrder,
    Revision,
    Source,
    Workspace,
    ContextLock,
    GovernanceProof,
    Packet,
    Criterion,
    Evidence,
    Lineage,
    Field,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SafeSubjectRefV1 {
    pub kind: SafeSubjectKindV1,
    pub value: String,
}

impl SafeSubjectRefV1 {
    pub fn new(kind: SafeSubjectKindV1, value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if is_safe_subject_value(&value) {
            Some(Self { kind, value })
        } else {
            None
        }
    }
}

impl<'de> Deserialize<'de> for SafeSubjectRefV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawSubject {
            kind: SafeSubjectKindV1,
            value: String,
        }
        let raw = RawSubject::deserialize(deserializer)?;
        SafeSubjectRefV1::new(raw.kind, raw.value)
            .ok_or_else(|| de::Error::custom("invalid safe subject reference"))
    }
}

fn is_safe_subject_value(value: &str) -> bool {
    if value.is_empty()
        || value.len() > 256
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.:/".contains(&b))
    {
        return false;
    }
    let lower = value.to_ascii_lowercase();
    ![
        "secret",
        "password",
        "token",
        "credential",
        "authorization",
        "api-key",
        "api_key",
        "apikey",
        "bearer",
        "canary",
        "prompt",
        "payload",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn is_safe_diagnostic_code(value: &str) -> bool {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return false;
    }
    let lower = value.to_ascii_lowercase();
    ![
        "secret",
        "password",
        "token",
        "credential",
        "authorization",
        "api-key",
        "api_key",
        "apikey",
        "bearer",
        "canary",
        "prompt",
        "payload",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn serialize_safe_diagnostic_code<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if is_safe_diagnostic_code(value) {
        serializer.serialize_str(value)
    } else {
        Err(serde::ser::Error::custom("invalid safe diagnostic code"))
    }
}

fn deserialize_safe_diagnostic_code<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if is_safe_diagnostic_code(&value) {
        Ok(value)
    } else {
        Err(de::Error::custom("invalid safe diagnostic code"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverityV1 {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClassV1 {
    None,
    SecretLikeValue,
    ExternalPayload,
    UnboundedText,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticV1 {
    #[serde(
        serialize_with = "serialize_safe_diagnostic_code",
        deserialize_with = "deserialize_safe_diagnostic_code"
    )]
    pub code: String,
    pub severity: DiagnosticSeverityV1,
    pub subject: Option<SafeSubjectRefV1>,
    pub redaction_class: Option<RedactionClassV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[serde(deny_unknown_fields)]
#[error("{category:?}:{code:?}")]
pub struct WorkOrderErrorV1 {
    pub category: WorkOrderErrorCategoryV1,
    pub code: WorkOrderErrorCodeV1,
    pub retryability: RetryabilityV1,
    pub subjects: Vec<SafeSubjectRefV1>,
    pub diagnostics: Vec<DiagnosticV1>,
}

impl WorkOrderErrorV1 {
    pub fn new(
        category: WorkOrderErrorCategoryV1,
        code: WorkOrderErrorCodeV1,
        retryability: RetryabilityV1,
    ) -> Self {
        Self {
            category,
            code,
            retryability,
            subjects: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn with_subject(mut self, kind: SafeSubjectKindV1, value: impl Into<String>) -> Self {
        if self.subjects.len() < 16 {
            if let Some(subject) = SafeSubjectRefV1::new(kind, value) {
                self.subjects.push(subject);
            }
        }
        self
    }

    pub fn with_diagnostic(
        mut self,
        code: impl Into<String>,
        severity: DiagnosticSeverityV1,
        subject: Option<SafeSubjectRefV1>,
        redaction_class: Option<RedactionClassV1>,
    ) -> Self {
        let code = code.into();
        if self.diagnostics.len() < 16 && is_safe_diagnostic_code(&code) {
            self.diagnostics.push(DiagnosticV1 {
                code,
                severity,
                subject,
                redaction_class,
            });
        }
        self
    }
}

pub(crate) fn error(
    category: WorkOrderErrorCategoryV1,
    code: WorkOrderErrorCodeV1,
) -> WorkOrderErrorV1 {
    let retryability = match (category, code) {
        (WorkOrderErrorCategoryV1::SourceProvenance, WorkOrderErrorCodeV1::SourceStale)
        | (WorkOrderErrorCategoryV1::AdmissionStaleness, WorkOrderErrorCodeV1::ContextLockStale)
        | (WorkOrderErrorCategoryV1::AdmissionStaleness, WorkOrderErrorCodeV1::PolicyStale)
        | (WorkOrderErrorCategoryV1::Lineage, WorkOrderErrorCodeV1::SnapshotStale) => {
            RetryabilityV1::AfterExplicitRefresh
        }
        (
            WorkOrderErrorCategoryV1::AdmissionStaleness,
            WorkOrderErrorCodeV1::GovernanceProofMissing,
        )
        | (
            WorkOrderErrorCategoryV1::AdmissionStaleness,
            WorkOrderErrorCodeV1::GovernanceProofMismatch,
        ) => RetryabilityV1::AfterGovernance,
        (WorkOrderErrorCategoryV1::Resource, _) => RetryabilityV1::AfterResourceChange,
        (WorkOrderErrorCategoryV1::InternalInvariant, _) => RetryabilityV1::InternalBug,
        _ => RetryabilityV1::Never,
    };
    WorkOrderErrorV1::new(category, code, retryability)
}
