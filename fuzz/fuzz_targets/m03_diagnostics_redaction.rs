#![no_main]

use core_work_order::*;
use libfuzzer_sys::fuzz_target;

fn contains_secret_marker(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    [
        "secret",
        "password",
        "token",
        "credential",
        "authorization",
        "api-key",
        "api_key",
        "apikey",
        "bearer",
        "canary",
        "prompt",
        "payload",
    ]
    .iter()
    .any(|marker| value.contains(marker))
}

fn assert_diagnostic_payload_is_safe(value: &serde_json::Value) {
    if let Some(code) = value.get("code").and_then(serde_json::Value::as_str) {
        assert!(!contains_secret_marker(code));
    }
    if let Some(subject) = value
        .get("subject")
        .and_then(serde_json::Value::as_object)
        .and_then(|subject| subject.get("value"))
        .and_then(serde_json::Value::as_str)
    {
        assert!(!contains_secret_marker(subject));
    }
}

fuzz_target!(|data: &[u8]| {
    if data.len() > 4_096 {
        return;
    }
    let raw = String::from_utf8_lossy(data).into_owned();
    let diagnostic = DiagnosticV1 {
        code: raw.clone(),
        severity: DiagnosticSeverityV1::Warning,
        subject: SafeSubjectRefV1::new(SafeSubjectKindV1::Evidence, raw.clone()),
        redaction_class: Some(RedactionClassV1::ExternalPayload),
    };
    let encoded_diagnostic = serde_json::to_string(&diagnostic);
    if let Ok(encoded) = encoded_diagnostic {
        let value = serde_json::from_str::<serde_json::Value>(&encoded).unwrap();
        assert_diagnostic_payload_is_safe(&value);
    }

    let error = WorkOrderErrorV1::new(
        WorkOrderErrorCategoryV1::SourceProvenance,
        WorkOrderErrorCodeV1::SourceEvidenceUnknown,
        RetryabilityV1::Never,
    )
    .with_subject(SafeSubjectKindV1::Source, raw.clone())
    .with_diagnostic(raw, DiagnosticSeverityV1::Warning, None, None);
    let encoded_error = serde_json::to_string(&error).unwrap();
    let value = serde_json::from_str::<serde_json::Value>(&encoded_error).unwrap();
    for diagnostic in value
        .get("diagnostics")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
    {
        assert_diagnostic_payload_is_safe(diagnostic);
    }
    for subject in value
        .get("subjects")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
    {
        if let Some(subject) = subject.get("value").and_then(serde_json::Value::as_str) {
            assert!(!contains_secret_marker(subject));
        }
    }
});
