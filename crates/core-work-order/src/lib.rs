//! M03 value-only compiler and admission boundary.
//!
//! Public services are synchronous and pure. Callers resolve external evidence
//! before invoking this crate and own timeout enforcement.

mod acceptance;
mod adapters;
mod admission;
mod budget;
mod canonical;
mod compiler;
mod context;
mod contracts;
mod delta;
mod errors;
mod identity;
mod lineage;
mod packets;
mod scope;
mod service;
mod source;

pub use adapters::*;
pub use budget::{M03ResourceBudgetV1, ResourceCalibrationStateV1};
pub use context::reconstruct_mandatory_source_ids;
pub use contracts::*;
pub use errors::*;
pub use identity::*;
pub use lineage::classify_lineage_cas;
pub use scope::{authorized_base_fingerprint, evaluate_scope, scope_fingerprint, ScopeDecisionV1};
pub use service::{
    canonical_semantic_bytes, classify_correction, compile, diff_revision, evaluate_admission,
    materialize_handoff, parse_request, validate_frozen,
};

pub const M03_SCHEMA: &str = contracts::M03_SCHEMA;
pub const M03_VERSION: u16 = contracts::M03_VERSION;
