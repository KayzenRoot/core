//! Versioned M04 Run / Attempt / Step contracts and canonical identity framing.
//!
//! Pack A declares the public value surface. Lifecycle, journal, persistence,
//! replay and service behavior remain in their later construction packets.
//!
//! Domain identity wrappers prevent accidental Rust-level substitution:
//!
//! ```compile_fail
//! use core_run_state::{AttemptId, RunId};
//! let run_id = RunId::new("run-1").unwrap();
//! let _: AttemptId = run_id;
//! ```
//!
//! Raw canonical frame construction is crate-private. Public fingerprint
//! creation must use a future semantic builder with an allow-listed projection:
//!
//! ```compile_fail
//! use core_run_state::CanonicalFrameV1;
//! ```

#[allow(dead_code)]
// Internal framing primitive is retained for future allow-listed semantic builders.
mod canonical;
mod contracts;
mod errors;
mod identity;

pub use canonical::FingerprintDomainV1;
pub use contracts::*;
pub use errors::{M04ErrorClassV1, M04ErrorCodeV1, M04ErrorV1, M04RetryabilityV1};
pub use identity::{
    AttemptId, AttemptOrdinalV1, CanonicalFingerprint, EventId, EventSequenceV1, ExecutionEpoch,
    IdempotencyKey, JournalRoot, RunGeneration, RunId, StepId, StepOrdinalV1,
};

/// Canonical schema identifier for all M04 V1 envelopes.
pub const M04_SCHEMA: &str = "nexlabs.core.run-state";
/// Supported M04 envelope version.
pub const M04_VERSION: u16 = 1;
