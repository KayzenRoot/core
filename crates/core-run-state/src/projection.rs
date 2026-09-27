//! Pack B: deterministic, provisional lifecycle projections and closure checks.
//!
//! These helpers do not append journal events, advance the generation or
//! publish durable state. Packs C/D/F bind them to verified events, CAS and
//! atomic persistence. A derived projection never grants execution authority.

use crate::{
    is_terminal_attempt, is_terminal_run, is_terminal_step, validate_attempt_transition,
    validate_run_transition, validate_step_transition, AttemptId, AttemptOrdinalV1,
    AttemptProjectionV1, AttemptStatusV1, ExternalReferenceEvidenceV1, M04ErrorClassV1,
    M04ErrorCodeV1, M04ErrorV1, M04ReasonCodeV1, M04RetryabilityV1, RunProjectionV1, RunStatusV1,
    StepId, StepOrdinalV1, StepProjectionV1, StepStatusV1,
};
use std::collections::BTreeSet;

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

fn invalid_projection() -> M04ErrorV1 {
    M04ErrorV1::new(
        M04ErrorClassV1::InternalInvariantViolation,
        M04ErrorCodeV1::InternalInvariantViolation,
        M04RetryabilityV1::InternalBug,
    )
}

/// Verify that ordinals form exactly the zero-based, non-reused sequence.
fn contiguous_ordinals(values: impl IntoIterator<Item = u64>) -> Result<u64, M04ErrorV1> {
    let mut ordinals = BTreeSet::new();
    for ordinal in values {
        if !ordinals.insert(ordinal) {
            return Err(invalid_projection());
        }
    }
    for (index, ordinal) in ordinals.iter().enumerate() {
        let expected = u64::try_from(index).map_err(|_| invalid_projection())?;
        if *ordinal != expected {
            return Err(invalid_projection());
        }
    }
    u64::try_from(ordinals.len()).map_err(|_| invalid_projection())
}

/// Successful or explicitly skipped steps are the only successful dispositions.
pub fn step_satisfies_completion(step: &StepProjectionV1) -> bool {
    step.status == StepStatusV1::Succeeded
        || (step.status == StepStatusV1::Skipped
            && step.reason == Some(M04ReasonCodeV1::ExplicitAuthorizedSkip))
}

/// A successful Attempt must account for every recorded required Step.
/// Pack E separately verifies the actual authority attached to every skip.
pub fn attempt_satisfies_completion(attempt: &AttemptProjectionV1) -> bool {
    attempt.steps.values().all(step_satisfies_completion)
}

/// Fail closed on duplicate/gapped ordinals, cross-key lineage and impossible
/// success/skip dispositions before deriving any further projection.
pub fn validate_projection_structure(run: &RunProjectionV1) -> Result<(), M04ErrorV1> {
    contiguous_ordinals(run.attempts.values().map(|attempt| attempt.ordinal.get()))?;
    if run.status == RunStatusV1::Created && !run.attempts.is_empty() {
        return Err(invalid_projection());
    }

    for (attempt_id, attempt) in &run.attempts {
        if attempt_id != &attempt.attempt_id {
            return Err(invalid_projection());
        }
        contiguous_ordinals(attempt.steps.values().map(|step| step.ordinal.get()))?;
        for (step_id, step) in &attempt.steps {
            if step_id != &step.step_id {
                return Err(invalid_projection());
            }
            if step.status == StepStatusV1::Skipped {
                if step.reason != Some(M04ReasonCodeV1::ExplicitAuthorizedSkip) {
                    return Err(invalid_projection());
                }
            } else if step.reason == Some(M04ReasonCodeV1::ExplicitAuthorizedSkip)
                || (is_terminal_step(step.status) && step.reason.is_none())
            {
                return Err(invalid_projection());
            }
        }
        if attempt.status == AttemptStatusV1::Succeeded && !attempt_satisfies_completion(attempt) {
            return Err(invalid_projection());
        }
    }
    if run.status == RunStatusV1::Succeeded {
        validate_run_completion(run).map_err(|_| invalid_projection())?;
    }
    Ok(())
}

/// A completed Run needs at least one successful Attempt with all its required
/// Steps satisfied. All Attempts must be terminal before the Run succeeds.
/// M04 does not choose a winning Attempt or implement retry policy.
pub fn validate_run_completion(run: &RunProjectionV1) -> Result<(), M04ErrorV1> {
    if !run
        .attempts
        .values()
        .all(|attempt| is_terminal_attempt(attempt.status))
    {
        return Err(invalid_transition());
    }
    if !run.attempts.values().any(|attempt| {
        attempt.status == AttemptStatusV1::Succeeded && attempt_satisfies_completion(attempt)
    }) {
        return Err(invalid_transition());
    }
    if run
        .attempts
        .values()
        .flat_map(|attempt| attempt.steps.values())
        .any(|step| !is_terminal_step(step.status))
    {
        return Err(invalid_transition());
    }
    Ok(())
}

/// Derive an initial Attempt projection without modifying the journal-bound
/// Run. The caller must only publish it after a matching durable event.
pub fn derive_attempt_created(
    run: &RunProjectionV1,
    attempt_id: AttemptId,
    ordinal: AttemptOrdinalV1,
) -> Result<AttemptProjectionV1, M04ErrorV1> {
    validate_projection_structure(run)?;
    if !matches!(run.status, RunStatusV1::Admitted | RunStatusV1::Active) {
        return Err(invalid_transition());
    }
    if run.attempts.contains_key(&attempt_id) {
        return Err(invalid_input());
    }
    let next = contiguous_ordinals(run.attempts.values().map(|attempt| attempt.ordinal.get()))?;
    if ordinal.get() != next {
        return Err(invalid_input());
    }
    Ok(AttemptProjectionV1 {
        attempt_id,
        ordinal,
        status: AttemptStatusV1::Created,
        steps: Default::default(),
    })
}

/// Derive a declared Step only under a live Run and nonterminal Attempt.
/// The parent Attempt must already be part of the supplied Run projection.
pub fn derive_step_declared(
    run: &RunProjectionV1,
    attempt_id: &AttemptId,
    step_id: StepId,
    ordinal: StepOrdinalV1,
) -> Result<StepProjectionV1, M04ErrorV1> {
    validate_projection_structure(run)?;
    if !matches!(run.status, RunStatusV1::Admitted | RunStatusV1::Active) {
        return Err(invalid_transition());
    }
    let attempt = run.attempts.get(attempt_id).ok_or_else(invalid_input)?;
    if is_terminal_attempt(attempt.status) || attempt.steps.contains_key(&step_id) {
        return Err(invalid_transition());
    }
    let next = contiguous_ordinals(attempt.steps.values().map(|step| step.ordinal.get()))?;
    if ordinal.get() != next {
        return Err(invalid_input());
    }
    Ok(StepProjectionV1 {
        step_id,
        ordinal,
        status: StepStatusV1::Declared,
        reason: None,
    })
}

/// Derive a Run transition without selecting execution/retry policy or
/// advancing journal/generation state. Success additionally proves closure.
pub fn derive_run_transition(
    run: &RunProjectionV1,
    target: RunStatusV1,
) -> Result<RunStatusV1, M04ErrorV1> {
    validate_projection_structure(run)?;
    validate_run_transition(run.status, target)?;
    if target == RunStatusV1::Succeeded {
        validate_run_completion(run)?;
    }
    Ok(target)
}

/// Derive an existing Attempt transition. Parent closure blocks new child
/// activation but allows explicit terminal closeout after interruption.
pub fn derive_attempt_transition(
    run: &RunProjectionV1,
    attempt_id: &AttemptId,
    target: AttemptStatusV1,
) -> Result<AttemptProjectionV1, M04ErrorV1> {
    validate_projection_structure(run)?;
    let attempt = run.attempts.get(attempt_id).ok_or_else(invalid_input)?;
    validate_attempt_transition(attempt.status, target)?;
    if is_terminal_run(run.status) && target == AttemptStatusV1::Active {
        return Err(invalid_transition());
    }
    if target == AttemptStatusV1::Succeeded && !attempt_satisfies_completion(attempt) {
        return Err(invalid_transition());
    }
    let mut next = attempt.clone();
    next.status = target;
    Ok(next)
}

/// Derive a Step transition, preserving the caller-supplied closed reason and
/// verifying skip-reference shape and lineage. Pack E must prove prior durable
/// attachment of the skip authority before any event is published.
pub fn derive_step_transition(
    run: &RunProjectionV1,
    attempt_id: &AttemptId,
    step_id: &StepId,
    target: StepStatusV1,
    reason: Option<M04ReasonCodeV1>,
    skip_authority: Option<&ExternalReferenceEvidenceV1>,
) -> Result<StepProjectionV1, M04ErrorV1> {
    validate_projection_structure(run)?;
    let attempt = run.attempts.get(attempt_id).ok_or_else(invalid_input)?;
    let step = attempt.steps.get(step_id).ok_or_else(invalid_input)?;
    if (is_terminal_run(run.status) || is_terminal_attempt(attempt.status))
        && matches!(target, StepStatusV1::Ready | StepStatusV1::Active)
    {
        return Err(invalid_transition());
    }
    validate_step_transition(
        step.status,
        target,
        reason,
        skip_authority,
        &run.run_id,
        attempt_id,
        step_id,
    )?;
    let mut next = step.clone();
    next.status = target;
    next.reason = reason;
    Ok(next)
}
