//! Pack B: closed, deterministic Run/Attempt/Step transition laws.
//!
//! These are pure admission checks, not durable transitions. Pack C owns
//! journal publication and replay, and Pack F owns prepare/store/finalize.
//! SKIPPED requires a caller-supplied, lineage-bound authority reference.
//! Authenticity and durable attachment of that reference belong to Pack E.

use crate::{
    AttemptId, AttemptStatusV1, ExternalReferenceEvidenceV1, M04ErrorClassV1, M04ErrorCodeV1,
    M04ErrorV1, M04ReasonCodeV1, M04RetryabilityV1, RunId, RunStatusV1, StepId, StepStatusV1,
};

fn invalid_transition() -> M04ErrorV1 {
    M04ErrorV1::new(
        M04ErrorClassV1::InvalidTransition,
        M04ErrorCodeV1::InvalidTransition,
        M04RetryabilityV1::Never,
    )
}

fn invalid_input() -> M04ErrorV1 {
    M04ErrorV1::new(
        M04ErrorClassV1::InvalidInput,
        M04ErrorCodeV1::InvalidInput,
        M04RetryabilityV1::Never,
    )
}

fn lineage_mismatch() -> M04ErrorV1 {
    M04ErrorV1::new(
        M04ErrorClassV1::LineageMismatch,
        M04ErrorCodeV1::LineageMismatch,
        M04RetryabilityV1::Never,
    )
}

/// Only these Run states can have outgoing lifecycle edges.
pub const fn is_terminal_run(status: RunStatusV1) -> bool {
    matches!(
        status,
        RunStatusV1::Succeeded
            | RunStatusV1::Failed
            | RunStatusV1::Cancelled
            | RunStatusV1::Blocked
            | RunStatusV1::Interrupted
    )
}

/// Only these Attempt states can have outgoing lifecycle edges.
pub const fn is_terminal_attempt(status: AttemptStatusV1) -> bool {
    matches!(
        status,
        AttemptStatusV1::Succeeded
            | AttemptStatusV1::Failed
            | AttemptStatusV1::Cancelled
            | AttemptStatusV1::Blocked
            | AttemptStatusV1::Interrupted
    )
}

/// SKIPPED is a terminal historical fact and cannot reactivate.
pub const fn is_terminal_step(status: StepStatusV1) -> bool {
    matches!(
        status,
        StepStatusV1::Succeeded
            | StepStatusV1::Failed
            | StepStatusV1::Cancelled
            | StepStatusV1::Blocked
            | StepStatusV1::Skipped
            | StepStatusV1::Interrupted
    )
}

/// Frozen Round 2 Run transition matrix. Any non-table edge fails typed.
pub fn validate_run_transition(from: RunStatusV1, to: RunStatusV1) -> Result<(), M04ErrorV1> {
    let legal = matches!(
        (from, to),
        (
            RunStatusV1::Created,
            RunStatusV1::Admitted | RunStatusV1::Blocked | RunStatusV1::Cancelled
        ) | (
            RunStatusV1::Admitted,
            RunStatusV1::Active
                | RunStatusV1::Blocked
                | RunStatusV1::Cancelled
                | RunStatusV1::Interrupted
        ) | (
            RunStatusV1::Active,
            RunStatusV1::Succeeded
                | RunStatusV1::Failed
                | RunStatusV1::Blocked
                | RunStatusV1::Cancelled
                | RunStatusV1::Interrupted
        )
    );
    if legal {
        Ok(())
    } else {
        Err(invalid_transition())
    }
}

/// Frozen Round 2 Attempt transition matrix.
pub fn validate_attempt_transition(
    from: AttemptStatusV1,
    to: AttemptStatusV1,
) -> Result<(), M04ErrorV1> {
    let legal = matches!(
        (from, to),
        (
            AttemptStatusV1::Created,
            AttemptStatusV1::Active | AttemptStatusV1::Blocked | AttemptStatusV1::Cancelled
        ) | (
            AttemptStatusV1::Active,
            AttemptStatusV1::Succeeded
                | AttemptStatusV1::Failed
                | AttemptStatusV1::Blocked
                | AttemptStatusV1::Cancelled
                | AttemptStatusV1::Interrupted
        )
    );
    if legal {
        Ok(())
    } else {
        Err(invalid_transition())
    }
}

/// Validate the *shape and lineage*, not the external truth, of an explicit
/// non-execution authority reference. Pack E must ensure this reference is
/// already durably attached and authorized before a SKIPPED event is prepared.
pub fn validate_skip_authority(
    reference: &ExternalReferenceEvidenceV1,
    run_id: &RunId,
    attempt_id: &AttemptId,
    step_id: &StepId,
) -> Result<(), M04ErrorV1> {
    if reference.run_id != *run_id
        || reference.attempt_id.as_ref() != Some(attempt_id)
        || reference.step_id.as_ref() != Some(step_id)
    {
        return Err(lineage_mismatch());
    }
    if reference.schema.as_str() != crate::M04_SCHEMA
        || reference.version.get() != crate::M04_VERSION
        || reference.immutable_locator.is_empty()
        || reference.immutable_identity.is_empty()
    {
        return Err(invalid_input());
    }
    Ok(())
}

/// Frozen Round 2 Step transition matrix. SKIPPED is only allowed from
/// DECLARED/READY with the explicit closed reason and authority reference.
/// This method is pure and does not claim a durable state change.
#[allow(clippy::too_many_arguments)]
pub fn validate_step_transition(
    from: StepStatusV1,
    to: StepStatusV1,
    reason: Option<M04ReasonCodeV1>,
    authority_reference: Option<&ExternalReferenceEvidenceV1>,
    run_id: &RunId,
    attempt_id: &AttemptId,
    step_id: &StepId,
) -> Result<(), M04ErrorV1> {
    let legal = matches!(
        (from, to),
        (
            StepStatusV1::Declared,
            StepStatusV1::Ready
                | StepStatusV1::Blocked
                | StepStatusV1::Cancelled
                | StepStatusV1::Skipped
        ) | (
            StepStatusV1::Ready,
            StepStatusV1::Active
                | StepStatusV1::Blocked
                | StepStatusV1::Cancelled
                | StepStatusV1::Skipped
        ) | (
            StepStatusV1::Active,
            StepStatusV1::Succeeded
                | StepStatusV1::Failed
                | StepStatusV1::Blocked
                | StepStatusV1::Cancelled
                | StepStatusV1::Interrupted
        )
    );
    if !legal {
        return Err(invalid_transition());
    }
    if to == StepStatusV1::Skipped {
        if reason != Some(M04ReasonCodeV1::ExplicitAuthorizedSkip) {
            return Err(invalid_transition());
        }
        let reference = authority_reference.ok_or_else(invalid_transition)?;
        validate_skip_authority(reference, run_id, attempt_id, step_id)?;
    }
    Ok(())
}
