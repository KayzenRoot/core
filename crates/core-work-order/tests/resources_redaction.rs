mod common;

use common::fixture;
use core_work_order::*;

#[test]
fn finite_positive_budgets_and_cap_plus_one_fail_atomically() {
    let fixture = fixture();
    assert!(M03ResourceBudgetV1::new(
        0,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        ResourceCalibrationStateV1::Uncalibrated
    )
    .is_err());
    assert!(M03ResourceBudgetV1::new(
        u64::MAX,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        ResourceCalibrationStateV1::Calibrated
    )
    .is_err());

    let mut uncalibrated = fixture.budget.clone();
    uncalibrated.calibration_state = ResourceCalibrationStateV1::Uncalibrated;
    assert_eq!(
        compile(&fixture.request, &fixture.context, &uncalibrated)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::InvalidContextBudget
    );

    let mut packet_cap = fixture.budget.clone();
    packet_cap.max_packets = 1;
    let result = compile(&fixture.request, &fixture.context, &packet_cap);
    assert_eq!(
        result.unwrap_err().code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    let mut frozen_cap = fixture.budget.clone();
    frozen_cap.max_frozen_bytes = 1;
    let result = compile(&fixture.request, &fixture.context, &frozen_cap);
    assert_eq!(
        result.unwrap_err().code,
        WorkOrderErrorCodeV1::SerializedSizeExceeded
    );

    let encoded = serde_json::to_vec(&WorkOrderEnvelope::new(
        WorkOrderContractKindV1::Request,
        fixture.request.clone(),
    ))
    .unwrap();
    let mut request_cap = fixture.budget.clone();
    request_cap.max_request_bytes = (encoded.len() - 1) as u64;
    assert_eq!(
        parse_request(&encoded, &request_cap).unwrap_err().code,
        WorkOrderErrorCodeV1::RequestTooLarge
    );
}

#[test]
fn parser_enforces_bounded_depth_and_never_returns_partial_values() {
    let fixture = fixture();
    let encoded = serde_json::to_vec(&WorkOrderEnvelope::new(
        WorkOrderContractKindV1::Request,
        fixture.request.clone(),
    ))
    .unwrap();
    let mut shallow = fixture.budget.clone();
    shallow.max_parse_depth = 1;
    assert!(parse_request(&encoded, &shallow).is_err());

    let mut too_many_sources = fixture;
    too_many_sources.budget.max_source_refs = 1;
    assert!(compile(
        &too_many_sources.request,
        &too_many_sources.context,
        &too_many_sources.budget
    )
    .is_err());
}

#[test]
fn safe_subjects_and_diagnostics_do_not_serialize_secret_canaries() {
    let canary = "canary_api_key_secret_7f03";
    let error = WorkOrderErrorV1::new(
        WorkOrderErrorCategoryV1::SchemaVersion,
        WorkOrderErrorCodeV1::InvalidEnvelope,
        RetryabilityV1::Never,
    )
    .with_subject(SafeSubjectKindV1::Source, canary)
    .with_diagnostic(
        canary,
        DiagnosticSeverityV1::Error,
        None,
        Some(RedactionClassV1::SecretLikeValue),
    );
    let serialized = serde_json::to_string(&error).unwrap();
    assert!(!serialized.contains(canary));
    assert!(!error.to_string().contains(canary));
    assert!(SafeSubjectRefV1::new(SafeSubjectKindV1::Source, canary).is_none());
    for unsafe_subject in [
        "user_prompt_ref",
        "external_payload_ref",
        "cpayloadary_api_",
    ] {
        assert!(SafeSubjectRefV1::new(SafeSubjectKindV1::Source, unsafe_subject).is_none());
        let error = WorkOrderErrorV1::new(
            WorkOrderErrorCategoryV1::SourceProvenance,
            WorkOrderErrorCodeV1::SourceEvidenceUnknown,
            RetryabilityV1::Never,
        )
        .with_subject(SafeSubjectKindV1::Source, unsafe_subject);
        assert!(!serde_json::to_string(&error)
            .unwrap()
            .contains(unsafe_subject));
    }

    let direct_diagnostic = DiagnosticV1 {
        code: canary.into(),
        severity: DiagnosticSeverityV1::Error,
        subject: None,
        redaction_class: Some(RedactionClassV1::SecretLikeValue),
    };
    let encoded = serde_json::to_string(&direct_diagnostic);
    assert!(encoded.is_err() || !encoded.unwrap().contains(canary));
}

#[test]
fn malformed_external_payload_is_not_echoed_by_typed_errors() {
    let fixture = fixture();
    let hostile = br#"{"schema":"nexlabs.core.work-order","version":1,"kind":"request","payload":"canary_provider_payload_api_key_7f03"}"#;
    let error = parse_request(hostile, &fixture.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::InvalidEnvelope);
    assert!(!error
        .to_string()
        .contains("canary_provider_payload_api_key_7f03"));
    assert!(!serde_json::to_string(&error)
        .unwrap()
        .contains("canary_provider_payload_api_key_7f03"));
}
