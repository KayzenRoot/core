mod common;

use common::{compiled, fixture, fp};
use core_work_order::*;

#[test]
fn semantic_input_permutations_preserve_bytes_and_fingerprints() {
    let baseline_input = fixture();
    let baseline = compiled(&baseline_input);
    let canonical_bytes = canonical_semantic_bytes(&baseline.frozen).unwrap();
    assert_eq!(
        core_identity::fingerprint_bytes(&canonical_bytes),
        "fd8cb2cf6285074bf7a28eaf3fb2ac083d2aed42ade2f62926901f0e5969b948"
    );
    assert_eq!(
        baseline.frozen.fingerprint().as_str(),
        "fd8cb2cf6285074bf7a28eaf3fb2ac083d2aed42ade2f62926901f0e5969b948"
    );

    let mut permuted_input = fixture();
    let request = &mut permuted_input.request;
    request.sources.reverse();
    request.packets.reverse();
    request.acceptance.criteria.reverse();
    request.acceptance.evidence_requirements.reverse();
    request.acceptance.edges.reverse();
    request
        .stop_condition
        .required_acceptance_criterion_ids
        .reverse();
    request
        .stop_condition
        .required_evidence_requirement_ids
        .reverse();
    request.context_lock.required_source_ids.reverse();
    request.scope.allowed_correction_classes.reverse();
    request.correction_policy.same_revision_classes.reverse();
    request.correction_policy.forbidden_classes.reverse();
    for packet in &mut request.packets {
        packet.context_budget.mandatory_source_ids.reverse();
    }
    permuted_input.context.sources.entries.reverse();
    let mut canonical_entries = permuted_input.context.sources.entries.clone();
    canonical_entries.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    permuted_input.context.sources.batch_fingerprint = fp(&(
        &permuted_input.context.sources.resolver_schema,
        permuted_input.context.sources.resolver_version,
        canonical_entries,
    ));

    let permuted = compile(
        &permuted_input.request,
        &permuted_input.context,
        &permuted_input.budget,
    )
    .unwrap();

    assert_eq!(baseline.frozen.fingerprint(), permuted.frozen.fingerprint());
    assert_eq!(
        canonical_bytes,
        canonical_semantic_bytes(&permuted.frozen).unwrap()
    );
    assert_eq!(
        baseline.frozen.compilation_id(),
        permuted.frozen.compilation_id()
    );
}

#[test]
fn compilation_context_generation_changes_only_compilation_identity() {
    let initial = fixture();
    let first = compiled(&initial);
    let mut changed = fixture();
    changed.context.config_generation += 1;
    let second = compile(&changed.request, &changed.context, &changed.budget).unwrap();

    assert_eq!(first.frozen.fingerprint(), second.frozen.fingerprint());
    assert_ne!(
        first.frozen.compilation_id(),
        second.frozen.compilation_id()
    );
}

#[test]
fn hive_context_is_advisory_and_only_changes_compilation_provenance() {
    let plain = fixture();
    let plain_result = compiled(&plain);
    let mut advisory = fixture();
    advisory.context.hive_context_refs.push(HiveContextRefV1 {
        context_id: ContextRefId::new("hive-context-1").unwrap(),
        hive_project_id: "non-authoritative-project-ref".into(),
        content_fingerprint: EvidenceFingerprintV1::new("a".repeat(64)).unwrap(),
        snapshot_id: "snapshot-1".into(),
        freshness: EvidenceFreshnessV1::Unknown,
        provenance_fingerprint: EvidenceFingerprintV1::new("b".repeat(64)).unwrap(),
        advisory_only: true,
    });
    let advisory_result = compile(&advisory.request, &advisory.context, &advisory.budget).unwrap();

    assert_eq!(
        plain_result.frozen.fingerprint(),
        advisory_result.frozen.fingerprint()
    );
    assert_ne!(
        plain_result.frozen.compilation_id(),
        advisory_result.frozen.compilation_id()
    );

    advisory.context.hive_context_refs[0].advisory_only = false;
    let error = compile(&advisory.request, &advisory.context, &advisory.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceAuthorityMismatch);
}

#[test]
fn diagnostic_changes_are_outside_frozen_semantic_identity() {
    let fixture = fixture();
    let mut compilation = compiled(&fixture);
    let fingerprint = compilation.frozen.fingerprint().clone();
    let bytes = canonical_semantic_bytes(&compilation.frozen).unwrap();

    compilation.diagnostics.push(DiagnosticV1 {
        code: "SAFE_TEST_DIAGNOSTIC".into(),
        severity: DiagnosticSeverityV1::Info,
        subject: None,
        redaction_class: None,
    });

    assert_eq!(compilation.frozen.fingerprint(), &fingerprint);
    assert_eq!(
        canonical_semantic_bytes(&compilation.frozen).unwrap(),
        bytes
    );
}

#[test]
fn semantic_change_alters_work_order_fingerprint_and_is_revision_scoped() {
    let original_input = fixture();
    let original = compiled(&original_input);
    let mut changed_input = fixture();
    changed_input
        .request
        .objective
        .push_str(" with a semantic change");
    let changed = compile(
        &changed_input.request,
        &changed_input.context,
        &changed_input.budget,
    )
    .unwrap();
    let diff = diff_revision(&original.frozen, &changed.frozen, &original_input.budget).unwrap();

    assert_ne!(original.frozen.fingerprint(), changed.frozen.fingerprint());
    assert!(diff.requires_new_revision);
    assert!(diff
        .changed_semantic_fields
        .iter()
        .any(|field| field.as_str() == "objective"));
    assert!(diff
        .forbidden_reason_codes
        .contains(&WorkOrderErrorCodeV1::RevisionNotNext));
}
