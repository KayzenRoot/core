use core_run_state::{
    validate_v1_contract_header, AttemptId, AttemptOrdinalV1, AttemptStatusV1,
    BoundaryRevalidationCapsuleV1, CancelRunRequestV1, CanonicalEventV1, CanonicalFingerprint,
    ContinuationFrameV1, ContractKindV1, EventId, EventKindV1, EventPayloadV1, EventSequenceV1,
    ExecutionEpoch, ExternalReferenceEvidenceV1, ExternalReferenceKindV1, ExternalReferenceOwnerV1,
    FingerprintDomainV1, IdempotencyKey, JournalRoot, M04EnvelopeV1, M04ErrorCodeV1, M04ErrorV1,
    M04ReasonCodeV1, M04SchemaV1, M04VersionV1, RawM04EnvelopeV1, RunGeneration, RunId,
    RunProjectionV1, RunSnapshotV1, RunStatusV1, StepId, StepOrdinalV1, StepStatusV1, M04_SCHEMA,
    M04_VERSION,
};
use core_work_order::{evaluate_admission, materialize_handoff, WorkOrderIdentityRefV1};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[path = "../../core-work-order/tests/common/mod.rs"]
mod work_order_common;

fn fingerprint(byte: char) -> CanonicalFingerprint {
    CanonicalFingerprint::new(byte.to_string().repeat(64)).expect("valid fingerprint")
}

fn cancel_request() -> CancelRunRequestV1 {
    CancelRunRequestV1 {
        run_id: RunId::new("run-17").expect("valid run id"),
        expected_generation: RunGeneration::new(3),
        idempotency_key: IdempotencyKey::new("caller:cancel-1").expect("valid key"),
        request_fingerprint: fingerprint('a'),
        reason: M04ReasonCodeV1::CallerCancellation,
    }
}

#[test]
fn typed_envelope_round_trips_with_fixed_schema_version_and_kind() {
    let envelope = M04EnvelopeV1::new(cancel_request());
    assert_eq!(envelope.schema(), "nexlabs.core.run-state");
    assert_eq!(envelope.version(), 1);
    assert_eq!(envelope.kind(), ContractKindV1::CancelRunRequest);

    let encoded = serde_json::to_string(&envelope).expect("serialize typed envelope");
    let decoded: M04EnvelopeV1<CancelRunRequestV1> =
        serde_json::from_str(&encoded).expect("validate and deserialize typed envelope");
    assert_eq!(decoded, envelope);
}

#[test]
fn envelope_rejects_unknown_schema_version_and_kind_as_typed_errors() {
    let valid = cancel_request();

    let error = RawM04EnvelopeV1 {
        schema: "nexlabs.core.other".to_owned(),
        version: 1,
        kind: "CANCEL_RUN_REQUEST".to_owned(),
        payload: valid.clone(),
    }
    .validate()
    .expect_err("unknown schema must fail");
    assert_eq!(error.code, M04ErrorCodeV1::UnsupportedSchema);

    let error = RawM04EnvelopeV1 {
        schema: "nexlabs.core.run-state".to_owned(),
        version: 2,
        kind: "CANCEL_RUN_REQUEST".to_owned(),
        payload: valid.clone(),
    }
    .validate()
    .expect_err("unknown version must fail");
    assert_eq!(error.code, M04ErrorCodeV1::UnsupportedVersion);

    let error = RawM04EnvelopeV1 {
        schema: "nexlabs.core.run-state".to_owned(),
        version: 1,
        kind: "UNRECOGNIZED_KIND".to_owned(),
        payload: valid.clone(),
    }
    .validate()
    .expect_err("unknown kind must fail");
    assert_eq!(error.code, M04ErrorCodeV1::UnsupportedKind);

    let error = RawM04EnvelopeV1 {
        schema: "nexlabs.core.run-state".to_owned(),
        version: 1,
        kind: "RUN_ADMISSION_REQUEST".to_owned(),
        payload: valid,
    }
    .validate()
    .expect_err("known but mismatched kind must fail");
    assert_eq!(error.code, M04ErrorCodeV1::KindPayloadMismatch);
}

#[test]
fn raw_envelope_requires_typed_validation_before_contract_acceptance() {
    let wire = serde_json::json!({
        "schema": "nexlabs.core.other",
        "version": 1,
        "kind": "CANCEL_RUN_REQUEST",
        "payload": cancel_request(),
    });
    let raw: RawM04EnvelopeV1<CancelRunRequestV1> =
        serde_json::from_value(wire.clone()).expect("raw boundary preserves untrusted metadata");
    assert_eq!(
        raw.validate()
            .expect_err("raw value must not become a typed V1 envelope")
            .code,
        M04ErrorCodeV1::UnsupportedSchema
    );
    assert!(serde_json::from_value::<M04EnvelopeV1<CancelRunRequestV1>>(wire).is_err());
}

#[test]
fn identifiers_and_fingerprints_reject_invalid_values() {
    let error = RunId::new("run with spaces").expect_err("spaces are not valid ID bytes");
    assert_eq!(error.code, M04ErrorCodeV1::InvalidIdentifier);
    assert!(serde_json::from_str::<RunId>("\"bad id\"").is_err());

    let error =
        CanonicalFingerprint::new("A".repeat(64)).expect_err("fingerprints must use lowercase hex");
    assert_eq!(error.code, M04ErrorCodeV1::InvalidFingerprint);
    assert!(serde_json::from_str::<CanonicalFingerprint>("\"short\"").is_err());
}

#[test]
fn closed_event_registry_rejects_arbitrary_values() {
    assert_eq!(EventKindV1::RunCreated.code(), 1);
    assert_eq!(EventKindV1::ReferenceAttached.code(), 10);
    assert!(serde_json::from_str::<EventKindV1>("\"ARBITRARY_EVENT\"").is_err());
}

#[test]
fn error_contract_remains_serializable_and_typed() {
    let error = M04ErrorV1::unsupported_version();
    let encoded = serde_json::to_string(&error).expect("serialize error contract");
    let decoded: M04ErrorV1 = serde_json::from_str(&encoded).expect("deserialize error contract");
    assert_eq!(decoded, error);
}

fn schema() -> M04SchemaV1 {
    M04SchemaV1::new(M04_SCHEMA).expect("supported M04 schema")
}

fn version() -> M04VersionV1 {
    M04VersionV1::new(M04_VERSION).expect("supported M04 version")
}

fn boundary_fixture() -> (BoundaryRevalidationCapsuleV1, WorkOrderIdentityRefV1) {
    static FIXTURE: OnceLock<(BoundaryRevalidationCapsuleV1, WorkOrderIdentityRefV1)> =
        OnceLock::new();
    FIXTURE
        .get_or_init(|| {
            let fixture = work_order_common::fixture();
            let compilation = work_order_common::compiled(&fixture);
            let admission = work_order_common::ready_admission(
                &fixture.request,
                &fixture.context,
                &compilation.frozen,
            );
            let receipt = evaluate_admission(&compilation.frozen, &admission, &fixture.budget)
                .expect("valid fixture admission");
            let admitted_work_order =
                materialize_handoff(&compilation.frozen, &receipt, &fixture.budget)
                    .expect("valid admitted work order");
            let work_order_identity = WorkOrderIdentityRefV1 {
                work_order_id: admitted_work_order.work_order_id().clone(),
                revision: admitted_work_order.revision(),
                work_order_fingerprint: admitted_work_order.work_order_fingerprint().clone(),
            };
            let revalidation = admitted_work_order.run_start_revalidation().clone();
            (
                BoundaryRevalidationCapsuleV1 {
                    schema: schema(),
                    version: version(),
                    admitted_work_order,
                    work_order_identity: work_order_identity.clone(),
                    revalidation,
                    execution_epoch: ExecutionEpoch::new(1),
                    capsule_fingerprint: fingerprint('b'),
                },
                work_order_identity,
            )
        })
        .clone()
}

fn external_reference() -> ExternalReferenceEvidenceV1 {
    ExternalReferenceEvidenceV1 {
        schema: schema(),
        version: version(),
        owner: ExternalReferenceOwnerV1::Execution,
        kind: ExternalReferenceKindV1::Evidence,
        immutable_locator: "evidence://run-17/output".to_owned(),
        immutable_identity: "evidence-17".to_owned(),
        content_fingerprint: Some(fingerprint('c')),
        run_id: RunId::new("run-17").expect("valid run id"),
        attempt_id: Some(AttemptId::new("attempt-1").expect("valid attempt id")),
        step_id: Some(StepId::new("step-1").expect("valid step id")),
    }
}

fn event_payloads() -> Vec<(EventKindV1, EventPayloadV1)> {
    let (boundary, work_order_identity) = boundary_fixture();
    let reference = external_reference();
    let attempt_id = AttemptId::new("attempt-1").expect("valid attempt id");
    let step_id = StepId::new("step-1").expect("valid step id");
    vec![
        (
            EventKindV1::RunCreated,
            EventPayloadV1::RunCreated {
                work_order: work_order_identity,
                boundary_fingerprint: boundary.capsule_fingerprint,
            },
        ),
        (
            EventKindV1::RunAdmitted,
            EventPayloadV1::RunAdmitted {
                boundary_fingerprint: fingerprint('b'),
            },
        ),
        (
            EventKindV1::RunTransitioned,
            EventPayloadV1::RunTransitioned {
                from: RunStatusV1::Created,
                to: RunStatusV1::Admitted,
                reason: None,
            },
        ),
        (
            EventKindV1::RunCancellationAccepted,
            EventPayloadV1::RunCancellationAccepted {
                cancellation_epoch: 1,
                reason: M04ReasonCodeV1::CallerCancellation,
            },
        ),
        (
            EventKindV1::AttemptCreated,
            EventPayloadV1::AttemptCreated {
                attempt_id: attempt_id.clone(),
                ordinal: AttemptOrdinalV1::new(0),
            },
        ),
        (
            EventKindV1::AttemptTransitioned,
            EventPayloadV1::AttemptTransitioned {
                attempt_id: attempt_id.clone(),
                from: AttemptStatusV1::Created,
                to: AttemptStatusV1::Active,
                reason: None,
            },
        ),
        (
            EventKindV1::StepDeclared,
            EventPayloadV1::StepDeclared {
                attempt_id: attempt_id.clone(),
                step_id: step_id.clone(),
                ordinal: StepOrdinalV1::new(0),
            },
        ),
        (
            EventKindV1::StepTransitioned,
            EventPayloadV1::StepTransitioned {
                attempt_id: attempt_id.clone(),
                step_id,
                from: StepStatusV1::Declared,
                to: StepStatusV1::Active,
                reason: None,
            },
        ),
        (
            EventKindV1::ContinuationCreated,
            EventPayloadV1::ContinuationCreated {
                source_attempt_id: attempt_id,
                attempt_id: AttemptId::new("attempt-2").expect("valid attempt id"),
                execution_epoch: ExecutionEpoch::new(2),
                boundary_fingerprint: fingerprint('b'),
            },
        ),
        (
            EventKindV1::ReferenceAttached,
            EventPayloadV1::ReferenceAttached { reference },
        ),
    ]
}

fn canonical_event(kind: EventKindV1, payload: EventPayloadV1) -> CanonicalEventV1 {
    CanonicalEventV1::try_new(
        FingerprintDomainV1::Event,
        kind,
        EventId::new("event-1").expect("valid event id"),
        RunId::new("run-17").expect("valid run id"),
        None,
        None,
        EventSequenceV1::new(1),
        RunGeneration::new(0),
        RunGeneration::new(1),
        IdempotencyKey::new("event:1").expect("valid idempotency key"),
        fingerprint('d'),
        JournalRoot::new("e".repeat(64)).expect("valid journal root"),
        JournalRoot::new("f".repeat(64)).expect("valid journal root"),
        payload,
    )
    .expect("event kind must match its payload")
}

fn assert_round_trip_and_reject_unknown_header<T>(valid: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let value = serde_json::to_value(valid).expect("serialize V1 contract");
    assert_eq!(value["schema"], M04_SCHEMA);
    assert_eq!(value["version"], M04_VERSION);
    let decoded: T = serde_json::from_value(value.clone()).expect("round-trip V1 contract");
    assert_eq!(&decoded, valid);

    let mut unknown_schema = value.clone();
    unknown_schema["schema"] = serde_json::json!("nexlabs.core.other");
    assert!(serde_json::from_value::<T>(unknown_schema).is_err());

    let mut unknown_version = value;
    unknown_version["version"] = serde_json::json!(2);
    assert!(serde_json::from_value::<T>(unknown_version).is_err());
}

#[test]
fn versioned_contract_headers_validate_and_fail_closed_for_every_dto() {
    assert_eq!(
        M04SchemaV1::new("nexlabs.core.other")
            .expect_err("unknown schema must fail")
            .code,
        M04ErrorCodeV1::UnsupportedSchema
    );
    assert_eq!(
        M04VersionV1::new(2)
            .expect_err("unknown version must fail")
            .code,
        M04ErrorCodeV1::UnsupportedVersion
    );
    assert_eq!(
        validate_v1_contract_header("nexlabs.core.other", 1)
            .expect_err("unknown schema must fail before V1 acceptance")
            .code,
        M04ErrorCodeV1::UnsupportedSchema
    );
    assert_eq!(
        validate_v1_contract_header(M04_SCHEMA, 2)
            .expect_err("unknown version must fail before V1 acceptance")
            .code,
        M04ErrorCodeV1::UnsupportedVersion
    );

    let (boundary, _) = boundary_fixture();
    let continuation = ContinuationFrameV1 {
        schema: schema(),
        version: version(),
        domain: FingerprintDomainV1::Continuation,
        run_id: RunId::new("run-17").expect("valid run id"),
        source_attempt_id: AttemptId::new("attempt-1").expect("valid attempt id"),
        last_committed_generation: RunGeneration::new(1),
        last_event_sequence: EventSequenceV1::new(1),
        journal_root: JournalRoot::new("e".repeat(64)).expect("valid journal root"),
        boundary_fingerprint: fingerprint('b'),
        execution_epoch: ExecutionEpoch::new(1),
        cursor_fingerprint: fingerprint('c'),
    };
    let reference = external_reference();
    let projection = RunProjectionV1 {
        run_id: RunId::new("run-17").expect("valid run id"),
        status: RunStatusV1::Created,
        generation: RunGeneration::new(1),
        last_event_sequence: EventSequenceV1::new(1),
        journal_root: JournalRoot::new("e".repeat(64)).expect("valid journal root"),
        boundary: boundary.clone(),
        attempts: BTreeMap::new(),
        idempotency_records: BTreeMap::new(),
    };
    let snapshot = RunSnapshotV1 {
        schema: schema(),
        version: version(),
        run_id: projection.run_id.clone(),
        source_generation: projection.generation,
        last_event_sequence: projection.last_event_sequence,
        journal_root: projection.journal_root.clone(),
        projection_fingerprint: fingerprint('a'),
        projection,
    };

    assert_round_trip_and_reject_unknown_header(&boundary);
    assert_round_trip_and_reject_unknown_header(&continuation);
    assert_round_trip_and_reject_unknown_header(&canonical_event(
        EventKindV1::ReferenceAttached,
        EventPayloadV1::ReferenceAttached {
            reference: reference.clone(),
        },
    ));
    assert_round_trip_and_reject_unknown_header(&snapshot);
    assert_round_trip_and_reject_unknown_header(&reference);

    for wrong_header in [
        ("schema", serde_json::json!("nexlabs.core.other")),
        ("version", serde_json::json!(2)),
    ] {
        let mut nested_boundary = serde_json::to_value(&snapshot).unwrap();
        nested_boundary["projection"]["boundary"][wrong_header.0] = wrong_header.1.clone();
        assert!(serde_json::from_value::<RunSnapshotV1>(nested_boundary).is_err());

        let mut nested_reference = serde_json::to_value(canonical_event(
            EventKindV1::ReferenceAttached,
            EventPayloadV1::ReferenceAttached {
                reference: external_reference(),
            },
        ))
        .unwrap();
        nested_reference["payload"]["data"]["reference"][wrong_header.0] = wrong_header.1;
        assert!(serde_json::from_value::<CanonicalEventV1>(nested_reference).is_err());
    }
}

#[test]
fn canonical_event_enforces_every_kind_payload_pair_on_construction_and_deserialization() {
    let pairs = event_payloads();
    assert_eq!(pairs.len(), 10);
    for (kind, payload) in pairs {
        let event = canonical_event(kind, payload);
        event.validate().expect("constructed event remains valid");
        let encoded = serde_json::to_string(&event).expect("serialize canonical event");
        let decoded: CanonicalEventV1 =
            serde_json::from_str(&encoded).expect("deserialize matching event pair");
        assert_eq!(decoded, event);
        assert_eq!(decoded.event_kind(), kind);
    }

    let mismatched = CanonicalEventV1::try_new(
        FingerprintDomainV1::Event,
        EventKindV1::RunCreated,
        EventId::new("event-2").unwrap(),
        RunId::new("run-17").unwrap(),
        None,
        None,
        EventSequenceV1::new(2),
        RunGeneration::new(1),
        RunGeneration::new(2),
        IdempotencyKey::new("event:2").unwrap(),
        fingerprint('d'),
        JournalRoot::new("e".repeat(64)).unwrap(),
        JournalRoot::new("f".repeat(64)).unwrap(),
        EventPayloadV1::ReferenceAttached {
            reference: external_reference(),
        },
    )
    .expect_err("RunCreated + ReferenceAttached must fail");
    assert_eq!(mismatched.code, M04ErrorCodeV1::KindPayloadMismatch);

    let valid = canonical_event(EventKindV1::RunCreated, event_payloads().remove(0).1);
    let mut mismatched_wire = serde_json::to_value(valid).unwrap();
    mismatched_wire["payload"] = serde_json::to_value(EventPayloadV1::ReferenceAttached {
        reference: external_reference(),
    })
    .unwrap();
    assert!(serde_json::from_value::<CanonicalEventV1>(mismatched_wire).is_err());
}
