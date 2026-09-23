mod common;

use common::{compiled, fixture, fp, ready_admission};
use core_work_order::*;

fn rebind_source_batch(batch: &mut SourceResolutionBatchV1) {
    for entry in &mut batch.entries {
        entry.evidence_fingerprint = fp(&(
            entry.source_id.as_str(),
            &entry.requested_fingerprint,
            &entry.observed_fingerprint,
            entry.authority_domain,
            &entry.source_revision,
            &entry.resolver_schema,
            entry.resolver_version,
            entry.freshness_state,
            &entry.provenance_fingerprint,
        ));
    }
    let mut entries = batch.entries.clone();
    entries.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    batch.batch_fingerprint = fp(&(&batch.resolver_schema, batch.resolver_version, entries));
}

fn rebind_workspace(evidence: &mut WorkspaceAdmissionEvidenceV1) {
    let mut components = evidence.satisfied_components.clone();
    components.sort();
    evidence.proof_fingerprint = fp(&(
        &evidence.m02_schema,
        evidence.m02_version,
        &evidence.project_binding_id,
        &evidence.workspace_id,
        evidence.runtime_epoch,
        evidence.generation,
        &evidence.basis_fingerprint,
        evidence.required_profile,
        &components,
        evidence.compatibility,
        evidence.freshness,
        &evidence.provenance_fingerprint,
    ));
}

#[test]
fn source_evidence_rejects_stale_unknown_substituted_and_unbound_values() {
    let valid = fixture();
    compiled(&valid);

    let mut stale = fixture();
    stale.context.sources.entries[0].freshness_state = EvidenceFreshnessV1::Stale;
    rebind_source_batch(&mut stale.context.sources);
    let error = compile(&stale.request, &stale.context, &stale.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceStale);

    let mut unknown = fixture();
    unknown.context.sources.entries[0].freshness_state = EvidenceFreshnessV1::Unknown;
    rebind_source_batch(&mut unknown.context.sources);
    let error = compile(&unknown.request, &unknown.context, &unknown.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceEvidenceUnknown);

    let mut substituted = fixture();
    substituted.context.sources.entries[0].observed_fingerprint =
        EvidenceFingerprintV1::new("c".repeat(64)).unwrap();
    rebind_source_batch(&mut substituted.context.sources);
    let error = compile(
        &substituted.request,
        &substituted.context,
        &substituted.budget,
    )
    .unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceSubstituted);

    let mut unbound = fixture();
    unbound.context.sources.entries[0].provenance_fingerprint =
        EvidenceFingerprintV1::new("d".repeat(64)).unwrap();
    let error = compile(&unbound.request, &unbound.context, &unbound.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceEvidenceUnknown);
}

#[test]
fn missing_and_extra_source_entries_fail_closed() {
    let mut missing = fixture();
    missing.context.sources.entries.pop();
    let mut entries = missing.context.sources.entries.clone();
    entries.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    missing.context.sources.batch_fingerprint =
        fp(&(&missing.context.sources.resolver_schema, 1u16, entries));
    let error = compile(&missing.request, &missing.context, &missing.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceMissing);

    let mut substituted = fixture();
    substituted.context.sources.entries[0].source_id =
        SourceRefId::new("unexpected-source").unwrap();
    rebind_source_batch(&mut substituted.context.sources);
    let error = compile(
        &substituted.request,
        &substituted.context,
        &substituted.budget,
    )
    .unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceSubstituted);
}

#[test]
fn source_locators_reject_traversal_and_uri_credentials_without_blocking_at_signs() {
    let mut valid_name = fixture();
    valid_name.request.sources[0].locator = "docs/decision@freeze.md".into();
    compiled(&valid_name);

    let mut traversal = fixture();
    traversal.request.sources[0].locator = "docs/../secret.md".into();
    let error = compile(&traversal.request, &traversal.context, &traversal.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceAuthorityMismatch);

    let mut credential_url = fixture();
    credential_url.request.sources[0].locator_kind = LocatorKindV1::EvidenceReference;
    credential_url.request.sources[0].locator = "https://user:secret@example.test/evidence".into();
    let error = compile(
        &credential_url.request,
        &credential_url.context,
        &credential_url.budget,
    )
    .unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SourceAuthorityMismatch);
}

#[test]
fn workspace_schema_profile_components_and_proof_are_bound() {
    let fixture = fixture();
    let compilation = compiled(&fixture);

    let mut wrong_schema = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    wrong_schema.workspace.m02_schema = "unknown.workspace".into();
    rebind_workspace(&mut wrong_schema.workspace);
    let receipt = evaluate_admission(&compilation.frozen, &wrong_schema, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(receipt
        .reason_codes()
        .contains(&WorkOrderErrorCodeV1::WorkspaceMismatch));

    let mut missing_component =
        ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    missing_component
        .workspace
        .satisfied_components
        .retain(|component| component != "IndexState");
    rebind_workspace(&mut missing_component.workspace);
    let receipt =
        evaluate_admission(&compilation.frozen, &missing_component, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(receipt
        .reason_codes()
        .contains(&WorkOrderErrorCodeV1::BasisIncompatible));

    let mut bad_proof = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    bad_proof.workspace.proof_fingerprint = EvidenceFingerprintV1::new("e".repeat(64)).unwrap();
    let receipt = evaluate_admission(&compilation.frozen, &bad_proof, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(receipt
        .reason_codes()
        .contains(&WorkOrderErrorCodeV1::GovernanceProofMismatch));
}
