use core_run_state::{
    CancelRunRequestV1, CanonicalFingerprint, ContractKindV1, EventKindV1, IdempotencyKey,
    M04EnvelopeV1, M04ErrorCodeV1, M04ErrorV1, M04ReasonCodeV1, RawM04EnvelopeV1, RunGeneration,
    RunId,
};

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
