//! Exhaustive Pack B transition-matrix and explicit-skip boundary tests.

use core_run_state::{
    is_terminal_attempt, is_terminal_run, is_terminal_step, validate_attempt_transition,
    validate_run_transition, validate_skip_authority, validate_step_transition, AttemptId,
    AttemptStatusV1, ExternalReferenceEvidenceV1, ExternalReferenceKindV1,
    ExternalReferenceOwnerV1, M04ErrorCodeV1, M04ReasonCodeV1, M04SchemaV1, M04VersionV1, RunId,
    RunStatusV1, StepId, StepStatusV1, M04_SCHEMA, M04_VERSION,
};

const RUN: [RunStatusV1; 8] = [
    RunStatusV1::Created,
    RunStatusV1::Admitted,
    RunStatusV1::Active,
    RunStatusV1::Succeeded,
    RunStatusV1::Failed,
    RunStatusV1::Cancelled,
    RunStatusV1::Blocked,
    RunStatusV1::Interrupted,
];
const ATTEMPT: [AttemptStatusV1; 7] = [
    AttemptStatusV1::Created,
    AttemptStatusV1::Active,
    AttemptStatusV1::Succeeded,
    AttemptStatusV1::Failed,
    AttemptStatusV1::Cancelled,
    AttemptStatusV1::Blocked,
    AttemptStatusV1::Interrupted,
];
const STEP: [StepStatusV1; 9] = [
    StepStatusV1::Declared,
    StepStatusV1::Ready,
    StepStatusV1::Active,
    StepStatusV1::Succeeded,
    StepStatusV1::Failed,
    StepStatusV1::Cancelled,
    StepStatusV1::Blocked,
    StepStatusV1::Skipped,
    StepStatusV1::Interrupted,
];

fn lineage() -> (RunId, AttemptId, StepId) {
    (
        RunId::new("run-01").unwrap(),
        AttemptId::new("attempt-01").unwrap(),
        StepId::new("step-01").unwrap(),
    )
}

fn skip_reference(run: &RunId, attempt: &AttemptId, step: &StepId) -> ExternalReferenceEvidenceV1 {
    ExternalReferenceEvidenceV1 {
        schema: M04SchemaV1::new(M04_SCHEMA).unwrap(),
        version: M04VersionV1::new(M04_VERSION).unwrap(),
        owner: ExternalReferenceOwnerV1::Review,
        kind: ExternalReferenceKindV1::ReviewRecord,
        immutable_locator: "artifact:skip-approval".into(),
        immutable_identity: "skip-approval-1".into(),
        content_fingerprint: None,
        run_id: run.clone(),
        attempt_id: Some(attempt.clone()),
        step_id: Some(step.clone()),
    }
}

#[test]
fn every_run_transition_pair_matches_the_frozen_matrix() {
    let mut legal_count = 0;
    for from in RUN {
        for to in RUN {
            let expected = match from {
                RunStatusV1::Created => matches!(
                    to,
                    RunStatusV1::Admitted | RunStatusV1::Blocked | RunStatusV1::Cancelled
                ),
                RunStatusV1::Admitted => matches!(
                    to,
                    RunStatusV1::Active
                        | RunStatusV1::Blocked
                        | RunStatusV1::Cancelled
                        | RunStatusV1::Interrupted
                ),
                RunStatusV1::Active => matches!(
                    to,
                    RunStatusV1::Succeeded
                        | RunStatusV1::Failed
                        | RunStatusV1::Blocked
                        | RunStatusV1::Cancelled
                        | RunStatusV1::Interrupted
                ),
                _ => false,
            };
            let result = validate_run_transition(from, to);
            if expected {
                legal_count += 1;
                assert!(result.is_ok(), "{from:?} -> {to:?} must be legal");
            } else {
                assert_eq!(
                    result.unwrap_err().code,
                    M04ErrorCodeV1::InvalidTransition,
                    "{from:?} -> {to:?}"
                );
            }
        }
    }
    assert_eq!(legal_count, 12);
    for state in RUN {
        assert_eq!(
            is_terminal_run(state),
            !matches!(
                state,
                RunStatusV1::Created | RunStatusV1::Admitted | RunStatusV1::Active
            )
        );
    }
}

#[test]
fn every_attempt_transition_pair_matches_the_frozen_matrix() {
    let mut legal_count = 0;
    for from in ATTEMPT {
        for to in ATTEMPT {
            let expected = match from {
                AttemptStatusV1::Created => matches!(
                    to,
                    AttemptStatusV1::Active | AttemptStatusV1::Blocked | AttemptStatusV1::Cancelled
                ),
                AttemptStatusV1::Active => matches!(
                    to,
                    AttemptStatusV1::Succeeded
                        | AttemptStatusV1::Failed
                        | AttemptStatusV1::Blocked
                        | AttemptStatusV1::Cancelled
                        | AttemptStatusV1::Interrupted
                ),
                _ => false,
            };
            let result = validate_attempt_transition(from, to);
            if expected {
                legal_count += 1;
                assert!(result.is_ok(), "{from:?} -> {to:?} must be legal");
            } else {
                assert_eq!(
                    result.unwrap_err().code,
                    M04ErrorCodeV1::InvalidTransition,
                    "{from:?} -> {to:?}"
                );
            }
        }
    }
    assert_eq!(legal_count, 8);
    for state in ATTEMPT {
        assert_eq!(
            is_terminal_attempt(state),
            !matches!(state, AttemptStatusV1::Created | AttemptStatusV1::Active)
        );
    }
}

#[test]
fn every_step_transition_pair_matches_the_frozen_matrix() {
    let (run, attempt, step) = lineage();
    let authority = skip_reference(&run, &attempt, &step);
    let mut legal_count = 0;
    for from in STEP {
        for to in STEP {
            let expected = match from {
                StepStatusV1::Declared => matches!(
                    to,
                    StepStatusV1::Ready
                        | StepStatusV1::Blocked
                        | StepStatusV1::Cancelled
                        | StepStatusV1::Skipped
                ),
                StepStatusV1::Ready => matches!(
                    to,
                    StepStatusV1::Active
                        | StepStatusV1::Blocked
                        | StepStatusV1::Cancelled
                        | StepStatusV1::Skipped
                ),
                StepStatusV1::Active => matches!(
                    to,
                    StepStatusV1::Succeeded
                        | StepStatusV1::Failed
                        | StepStatusV1::Blocked
                        | StepStatusV1::Cancelled
                        | StepStatusV1::Interrupted
                ),
                _ => false,
            };
            let reason =
                (to == StepStatusV1::Skipped).then_some(M04ReasonCodeV1::ExplicitAuthorizedSkip);
            let result =
                validate_step_transition(from, to, reason, Some(&authority), &run, &attempt, &step);
            if expected {
                legal_count += 1;
                assert!(result.is_ok(), "{from:?} -> {to:?} must be legal");
            } else {
                assert_eq!(
                    result.unwrap_err().code,
                    M04ErrorCodeV1::InvalidTransition,
                    "{from:?} -> {to:?}"
                );
            }
        }
    }
    assert_eq!(legal_count, 13);
    for state in STEP {
        assert_eq!(
            is_terminal_step(state),
            !matches!(
                state,
                StepStatusV1::Declared | StepStatusV1::Ready | StepStatusV1::Active
            )
        );
    }
}

#[test]
fn skipped_requires_an_explicit_non_execution_reason_and_matching_reference() {
    let (run, attempt, step) = lineage();
    let reference = skip_reference(&run, &attempt, &step);
    for from in [StepStatusV1::Declared, StepStatusV1::Ready] {
        for (reason, evidence) in [
            (None, Some(&reference)),
            (Some(M04ReasonCodeV1::ExecutionOutcome), Some(&reference)),
            (Some(M04ReasonCodeV1::ExplicitAuthorizedSkip), None),
        ] {
            assert_eq!(
                validate_step_transition(
                    from,
                    StepStatusV1::Skipped,
                    reason,
                    evidence,
                    &run,
                    &attempt,
                    &step
                )
                .unwrap_err()
                .code,
                M04ErrorCodeV1::InvalidTransition,
            );
        }
        assert!(validate_step_transition(
            from,
            StepStatusV1::Skipped,
            Some(M04ReasonCodeV1::ExplicitAuthorizedSkip),
            Some(&reference),
            &run,
            &attempt,
            &step,
        )
        .is_ok());
    }
    assert_eq!(
        validate_step_transition(
            StepStatusV1::Active,
            StepStatusV1::Skipped,
            Some(M04ReasonCodeV1::ExplicitAuthorizedSkip),
            Some(&reference),
            &run,
            &attempt,
            &step,
        )
        .unwrap_err()
        .code,
        M04ErrorCodeV1::InvalidTransition,
    );
}

#[test]
fn skip_reference_shape_and_lineage_are_fail_closed() {
    let (run, attempt, step) = lineage();
    let reference = skip_reference(&run, &attempt, &step);
    assert!(validate_skip_authority(&reference, &run, &attempt, &step).is_ok());
    for mut invalid in [
        ExternalReferenceEvidenceV1 {
            run_id: RunId::new("foreign-run").unwrap(),
            ..reference.clone()
        },
        ExternalReferenceEvidenceV1 {
            attempt_id: Some(AttemptId::new("foreign-attempt").unwrap()),
            ..reference.clone()
        },
        ExternalReferenceEvidenceV1 {
            step_id: None,
            ..reference.clone()
        },
    ] {
        assert_eq!(
            validate_skip_authority(&invalid, &run, &attempt, &step)
                .unwrap_err()
                .code,
            M04ErrorCodeV1::LineageMismatch,
        );
        invalid.immutable_locator.clear();
    }
    for invalid in [
        ExternalReferenceEvidenceV1 {
            immutable_locator: String::new(),
            ..reference.clone()
        },
        ExternalReferenceEvidenceV1 {
            immutable_identity: String::new(),
            ..reference
        },
    ] {
        assert_eq!(
            validate_skip_authority(&invalid, &run, &attempt, &step)
                .unwrap_err()
                .code,
            M04ErrorCodeV1::InvalidInput,
        );
    }
}
