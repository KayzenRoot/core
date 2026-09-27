//! Pack B provisional projection, ordinal and parent-child adversarial tests.

use core_run_state::{
    attempt_satisfies_completion, derive_attempt_created, derive_attempt_transition,
    derive_run_transition, derive_step_declared, derive_step_transition, step_satisfies_completion,
    validate_projection_structure, AttemptId, AttemptOrdinalV1, AttemptProjectionV1,
    AttemptStatusV1, BoundaryRevalidationCapsuleV1, CanonicalFingerprint, EventSequenceV1,
    ExecutionEpoch, ExternalReferenceEvidenceV1, ExternalReferenceKindV1,
    ExternalReferenceOwnerV1, JournalRoot, M04ErrorCodeV1, M04ReasonCodeV1, M04SchemaV1,
    M04VersionV1, RunGeneration, RunId, RunProjectionV1, RunStatusV1, StepId, StepOrdinalV1,
    StepProjectionV1, StepStatusV1, M04_SCHEMA, M04_VERSION,
};
use core_work_order::{evaluate_admission, materialize_handoff, WorkOrderIdentityRefV1};
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[path = "../../core-work-order/tests/common/mod.rs"]
mod work_order_common;

fn fingerprint(ch: char) -> CanonicalFingerprint {
    CanonicalFingerprint::new(ch.to_string().repeat(64)).unwrap()
}

fn boundary_fixture() -> BoundaryRevalidationCapsuleV1 {
    static BOUNDARY: OnceLock<BoundaryRevalidationCapsuleV1> = OnceLock::new();
    BOUNDARY
        .get_or_init(|| {
            let fixture = work_order_common::fixture();
            let compiled = work_order_common::compiled(&fixture);
            let admission = work_order_common::ready_admission(
                &fixture.request,
                &fixture.context,
                &compiled.frozen,
            );
            let receipt = evaluate_admission(&compiled.frozen, &admission, &fixture.budget).unwrap();
            let admitted = materialize_handoff(&compiled.frozen, &receipt, &fixture.budget).unwrap();
            let identity = WorkOrderIdentityRefV1 {
                work_order_id: admitted.work_order_id().clone(),
                revision: admitted.revision(),
                work_order_fingerprint: admitted.work_order_fingerprint().clone(),
            };
            BoundaryRevalidationCapsuleV1 {
                schema: M04SchemaV1::new(M04_SCHEMA).unwrap(),
                version: M04VersionV1::new(M04_VERSION).unwrap(),
                revalidation: admitted.run_start_revalidation().clone(),
                admitted_work_order: admitted,
                work_order_identity: identity,
                execution_epoch: ExecutionEpoch::new(1),
                capsule_fingerprint: fingerprint('b'),
            }
        })
        .clone()
}

fn run() -> RunProjectionV1 {
    RunProjectionV1 {
        run_id: RunId::new("run-01").unwrap(),
        status: RunStatusV1::Admitted,
        generation: RunGeneration::new(1),
        last_event_sequence: EventSequenceV1::new(1),
        journal_root: JournalRoot::new("e".repeat(64)).unwrap(),
        boundary: boundary_fixture(),
        attempts: BTreeMap::new(),
        idempotency_records: BTreeMap::new(),
    }
}

fn with_attempt() -> (RunProjectionV1, AttemptId) {
    let mut run = run();
    let id = AttemptId::new("attempt-01").unwrap();
    let attempt = derive_attempt_created(&run, id.clone(), AttemptOrdinalV1::new(0)).unwrap();
    run.attempts.insert(id.clone(), attempt);
    (run, id)
}

fn skip_authority(run: &RunId, attempt: &AttemptId, step: &StepId) -> ExternalReferenceEvidenceV1 {
    ExternalReferenceEvidenceV1 {
        schema: M04SchemaV1::new(M04_SCHEMA).unwrap(),
        version: M04VersionV1::new(M04_VERSION).unwrap(),
        owner: ExternalReferenceOwnerV1::Review,
        kind: ExternalReferenceKindV1::ReviewRecord,
        immutable_locator: "artifact:explicit-skip".into(),
        immutable_identity: "approval-01".into(),
        content_fingerprint: Some(fingerprint('a')),
        run_id: run.clone(),
        attempt_id: Some(attempt.clone()),
        step_id: Some(step.clone()),
    }
}

#[test]
fn attempts_have_contiguous_ordinals_and_never_reuse_identity() {
    let (mut run, first) = with_attempt();
    let second = AttemptId::new("attempt-02").unwrap();
    assert_eq!(
        derive_attempt_created(&run, first, AttemptOrdinalV1::new(1))
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidInput
    );
    assert_eq!(
        derive_attempt_created(&run, second.clone(), AttemptOrdinalV1::new(2))
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidInput
    );
    let next = derive_attempt_created(&run, second.clone(), AttemptOrdinalV1::new(1)).unwrap();
    run.attempts.insert(second, next);
    assert!(validate_projection_structure(&run).is_ok());
    run.status = RunStatusV1::Cancelled;
    assert_eq!(
        derive_attempt_created(
            &run,
            AttemptId::new("attempt-03").unwrap(),
            AttemptOrdinalV1::new(2)
        )
        .unwrap_err()
        .code,
        M04ErrorCodeV1::InvalidTransition
    );
}

#[test]
fn steps_require_live_parent_and_contiguous_nonreused_ordinals() {
    let (mut run, attempt) = with_attempt();
    let first = StepId::new("step-01").unwrap();
    let step = derive_step_declared(&run, &attempt, first.clone(), StepOrdinalV1::new(0)).unwrap();
    run.attempts.get_mut(&attempt).unwrap().steps.insert(first.clone(), step);
    assert_eq!(
        derive_step_declared(&run, &attempt, first, StepOrdinalV1::new(1))
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidTransition
    );
    let second = StepId::new("step-02").unwrap();
    assert_eq!(
        derive_step_declared(&run, &attempt, second.clone(), StepOrdinalV1::new(2))
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidInput
    );
    let second_step =
        derive_step_declared(&run, &attempt, second.clone(), StepOrdinalV1::new(1)).unwrap();
    run.attempts.get_mut(&attempt).unwrap().steps.insert(second, second_step);
    assert!(validate_projection_structure(&run).is_ok());
    run.attempts.get_mut(&attempt).unwrap().status = AttemptStatusV1::Blocked;
    assert_eq!(
        derive_step_declared(
            &run,
            &attempt,
            StepId::new("step-03").unwrap(),
            StepOrdinalV1::new(2)
        )
        .unwrap_err()
        .code,
        M04ErrorCodeV1::InvalidTransition
    );
}

#[test]
fn parent_success_requires_satisfied_steps_and_terminal_attempts() {
    let (mut run, attempt) = with_attempt();
    run.status = RunStatusV1::Active;
    let step_id = StepId::new("step-01").unwrap();
    let mut step = derive_step_declared(&run, &attempt, step_id.clone(), StepOrdinalV1::new(0))
        .unwrap();
    step.status = StepStatusV1::Active;
    run.attempts.get_mut(&attempt).unwrap().status = AttemptStatusV1::Active;
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id.clone(), step);
    assert_eq!(
        derive_attempt_transition(&run, &attempt, AttemptStatusV1::Succeeded)
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidTransition
    );
    let completed = derive_step_transition(
        &run,
        &attempt,
        &step_id,
        StepStatusV1::Succeeded,
        Some(M04ReasonCodeV1::ExecutionOutcome),
        None,
    )
    .unwrap();
    assert!(step_satisfies_completion(&completed));
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id, completed);
    assert_eq!(
        derive_run_transition(&run, RunStatusV1::Succeeded)
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidTransition
    );
    let completed_attempt =
        derive_attempt_transition(&run, &attempt, AttemptStatusV1::Succeeded).unwrap();
    assert!(attempt_satisfies_completion(&completed_attempt));
    run.attempts.insert(attempt, completed_attempt);
    assert_eq!(derive_run_transition(&run, RunStatusV1::Succeeded).unwrap(), RunStatusV1::Succeeded);
    assert_eq!(run.generation.get(), 1, "derivation cannot mint generation authority");
    assert_eq!(run.last_event_sequence.get(), 1, "derivation cannot append a journal event");
}

#[test]
fn skipped_requires_explicit_lineage_bound_evidence_and_closed_reason() {
    let (mut run, attempt) = with_attempt();
    run.status = RunStatusV1::Active;
    let step_id = StepId::new("step-01").unwrap();
    let step = derive_step_declared(&run, &attempt, step_id.clone(), StepOrdinalV1::new(0))
        .unwrap();
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id.clone(), step);
    let authority = skip_authority(&run.run_id, &attempt, &step_id);
    assert_eq!(
        derive_step_transition(
            &run,
            &attempt,
            &step_id,
            StepStatusV1::Skipped,
            Some(M04ReasonCodeV1::ExplicitAuthorizedSkip),
            None
        )
        .unwrap_err()
        .code,
        M04ErrorCodeV1::InvalidTransition
    );
    let invalid = ExternalReferenceEvidenceV1 {
        run_id: RunId::new("other-run").unwrap(),
        ..authority.clone()
    };
    assert_eq!(
        derive_step_transition(
            &run,
            &attempt,
            &step_id,
            StepStatusV1::Skipped,
            Some(M04ReasonCodeV1::ExplicitAuthorizedSkip),
            Some(&invalid)
        )
        .unwrap_err()
        .code,
        M04ErrorCodeV1::LineageMismatch
    );
    let skipped = derive_step_transition(
        &run,
        &attempt,
        &step_id,
        StepStatusV1::Skipped,
        Some(M04ReasonCodeV1::ExplicitAuthorizedSkip),
        Some(&authority),
    )
    .unwrap();
    assert!(step_satisfies_completion(&skipped));
    run.attempts.get_mut(&attempt).unwrap().status = AttemptStatusV1::Active;
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id, skipped);
    let completed = derive_attempt_transition(&run, &attempt, AttemptStatusV1::Succeeded).unwrap();
    run.attempts.insert(attempt, completed);
    assert!(derive_run_transition(&run, RunStatusV1::Succeeded).is_ok());
}

#[test]
fn invalid_projection_fails_closed_on_cross_keys_gaps_and_fake_success() {
    let (mut run, attempt) = with_attempt();
    run.attempts.get_mut(&attempt).unwrap().ordinal = AttemptOrdinalV1::new(3);
    assert_eq!(
        validate_projection_structure(&run).unwrap_err().code,
        M04ErrorCodeV1::InternalInvariantViolation
    );
    run.attempts.get_mut(&attempt).unwrap().ordinal = AttemptOrdinalV1::new(0);
    run.attempts.get_mut(&attempt).unwrap().attempt_id = AttemptId::new("foreign").unwrap();
    assert_eq!(
        validate_projection_structure(&run).unwrap_err().code,
        M04ErrorCodeV1::InternalInvariantViolation
    );
    run.attempts.get_mut(&attempt).unwrap().attempt_id = attempt.clone();
    let mut step = StepProjectionV1 {
        step_id: StepId::new("step-01").unwrap(),
        ordinal: StepOrdinalV1::new(0),
        status: StepStatusV1::Skipped,
        reason: None,
    };
    let step_id = step.step_id.clone();
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id.clone(), step.clone());
    assert_eq!(
        validate_projection_structure(&run).unwrap_err().code,
        M04ErrorCodeV1::InternalInvariantViolation
    );
    step.reason = Some(M04ReasonCodeV1::ExplicitAuthorizedSkip);
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id, step);
    assert!(validate_projection_structure(&run).is_ok());
    run.attempts.get_mut(&attempt).unwrap().status = AttemptStatusV1::Succeeded;
    assert!(validate_projection_structure(&run).is_ok());
    run.status = RunStatusV1::Succeeded;
    assert!(validate_projection_structure(&run).is_ok());
}

#[test]
fn terminal_parent_blocks_new_activation_but_permits_existing_closeout() {
    let (mut run, attempt) = with_attempt();
    let step_id = StepId::new("step-01").unwrap();
    let mut step = derive_step_declared(&run, &attempt, step_id.clone(), StepOrdinalV1::new(0))
        .unwrap();
    step.status = StepStatusV1::Active;
    run.attempts.get_mut(&attempt).unwrap().steps.insert(step_id.clone(), step);
    run.attempts.get_mut(&attempt).unwrap().status = AttemptStatusV1::Active;
    run.status = RunStatusV1::Interrupted;
    assert_eq!(
        derive_step_transition(&run, &attempt, &step_id, StepStatusV1::Ready, None, None)
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidTransition
    );
    assert_eq!(
        derive_step_transition(
            &run,
            &attempt,
            &step_id,
            StepStatusV1::Interrupted,
            Some(M04ReasonCodeV1::RuntimeShutdown),
            None
        )
        .unwrap()
        .status,
        StepStatusV1::Interrupted
    );
    assert_eq!(
        derive_attempt_transition(&run, &attempt, AttemptStatusV1::Active)
            .unwrap_err()
            .code,
        M04ErrorCodeV1::InvalidTransition
    );
}
