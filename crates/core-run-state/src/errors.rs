use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Stable top-level M04 error classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum M04ErrorClassV1 {
    InvalidInput,
    InvalidTransition,
    LineageMismatch,
    GenerationConflict,
    IdempotencyConflict,
    StaleAuthority,
    StaleContinuation,
    ReplayIntegrityFailure,
    ResourceLimitExceeded,
    Cancelled,
    Blocked,
    UnsupportedVersion,
    ExternalReferenceRejected,
    StorageConflict,
    InternalInvariantViolation,
}

/// Closed V1 machine-readable error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum M04ErrorCodeV1 {
    InvalidInput,
    InvalidIdentifier,
    InvalidFingerprint,
    InvalidCanonicalFieldOrder,
    UnsupportedSchema,
    UnsupportedVersion,
    UnsupportedKind,
    KindPayloadMismatch,
    InvalidTransition,
    LineageMismatch,
    GenerationConflict,
    IdempotencyConflict,
    StaleAuthority,
    StaleContinuation,
    ReplayIntegrityFailure,
    ResourceLimitExceeded,
    Cancelled,
    Blocked,
    ExternalReferenceRejected,
    StorageConflict,
    InternalInvariantViolation,
}

/// Retryability fact reported to the caller; this is not retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum M04RetryabilityV1 {
    Never,
    CallerMayRetry,
    AfterRevalidation,
    AfterResourceChange,
    InternalBug,
}

/// Bounded diagnostic payload. Producers apply M04ResourceLimitsV1 before
/// publication; the type itself does not invent a numeric limit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M04DiagnosticV1 {
    pub code: String,
    pub message: String,
}

/// Stable typed M04 error contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[serde(deny_unknown_fields)]
#[error("{class:?}/{code:?}")]
pub struct M04ErrorV1 {
    pub class: M04ErrorClassV1,
    pub code: M04ErrorCodeV1,
    pub retryability: M04RetryabilityV1,
    pub diagnostic: Option<M04DiagnosticV1>,
}

impl M04ErrorV1 {
    pub const fn new(
        class: M04ErrorClassV1,
        code: M04ErrorCodeV1,
        retryability: M04RetryabilityV1,
    ) -> Self {
        Self {
            class,
            code,
            retryability,
            diagnostic: None,
        }
    }

    pub fn with_diagnostic(mut self, code: impl Into<String>, message: impl Into<String>) -> Self {
        self.diagnostic = Some(M04DiagnosticV1 {
            code: code.into(),
            message: message.into(),
        });
        self
    }

    pub const fn invalid_identifier() -> Self {
        Self::new(
            M04ErrorClassV1::InvalidInput,
            M04ErrorCodeV1::InvalidIdentifier,
            M04RetryabilityV1::Never,
        )
    }

    pub const fn invalid_fingerprint() -> Self {
        Self::new(
            M04ErrorClassV1::InvalidInput,
            M04ErrorCodeV1::InvalidFingerprint,
            M04RetryabilityV1::Never,
        )
    }

    pub const fn invalid_canonical_field_order() -> Self {
        Self::new(
            M04ErrorClassV1::InvalidInput,
            M04ErrorCodeV1::InvalidCanonicalFieldOrder,
            M04RetryabilityV1::Never,
        )
    }

    pub const fn unsupported_schema() -> Self {
        Self::new(
            M04ErrorClassV1::UnsupportedVersion,
            M04ErrorCodeV1::UnsupportedSchema,
            M04RetryabilityV1::Never,
        )
    }

    pub const fn unsupported_version() -> Self {
        Self::new(
            M04ErrorClassV1::UnsupportedVersion,
            M04ErrorCodeV1::UnsupportedVersion,
            M04RetryabilityV1::Never,
        )
    }

    pub const fn unsupported_kind() -> Self {
        Self::new(
            M04ErrorClassV1::UnsupportedVersion,
            M04ErrorCodeV1::UnsupportedKind,
            M04RetryabilityV1::Never,
        )
    }

    pub const fn kind_payload_mismatch() -> Self {
        Self::new(
            M04ErrorClassV1::InvalidInput,
            M04ErrorCodeV1::KindPayloadMismatch,
            M04RetryabilityV1::Never,
        )
    }
}
