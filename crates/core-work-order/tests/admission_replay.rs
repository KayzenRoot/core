mod common;

use common::{compiled, fixture, fp, ready_admission};
use core_work_order::*;
use std::sync::mpsc;
use std::time::Duration;

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
fn admission_receipts_are_deterministic_and_handoff_binds_all_revalidation_fields() {
    let fixture = fixture();
    let compilation = compiled(&fixture);
    let request = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);

    let receipt = evaluate_admission(&compilation.frozen, &request, &fixture.budget).unwrap();
    let replay = evaluate_admission(&compilation.frozen, &request, &fixture.budget).unwrap();
    assert_eq!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert_eq!(receipt, replay);
    assert_eq!(receipt.receipt_fingerprint(), replay.receipt_fingerprint());

    let handoff = materialize_handoff(&compilation.frozen, &receipt, &fixture.budget).unwrap();
    assert_eq!(handoff.work_order_id(), compilation.frozen.work_order_id());
    assert_eq!(handoff.revision(), compilation.frozen.revision());
    assert_eq!(
        handoff.work_order_fingerprint(),
        compilation.frozen.fingerprint()
    );
    assert_eq!(
        handoff.admission_receipt_fingerprint(),
        receipt.receipt_fingerprint()
    );
    let revalidation = handoff.run_start_revalidation();
    assert_eq!(
        revalidation.source_batch_fingerprint,
        request.sources.batch_fingerprint
    );
    assert_eq!(revalidation.workspace_id, request.workspace.workspace_id);
    assert_eq!(
        revalidation.workspace_runtime_epoch,
        request.workspace.runtime_epoch
    );
    assert_eq!(
        revalidation.workspace_generation,
        request.workspace.generation
    );
    assert_eq!(
        revalidation.workspace_basis_fingerprint,
        request.workspace.basis_fingerprint
    );
    assert_eq!(
        Some(&revalidation.context_lock_fingerprint),
        receipt.context_lock_fingerprint()
    );
    assert_eq!(
        Some(&revalidation.governance_proof_fingerprint),
        receipt.governance_proof_fingerprint()
    );
    assert_eq!(revalidation.policy_generation, request.policy_generation);
    assert_eq!(
        revalidation.security_generation,
        request.security_generation
    );
    assert_eq!(revalidation.config_generation, request.config_generation);
}

#[test]
fn stale_unknown_substituted_and_mismatched_proofs_never_become_ready() {
    let fixture = fixture();
    let compilation = compiled(&fixture);

    let mut stale = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    stale.workspace.freshness = EvidenceFreshnessV1::Stale;
    rebind_workspace(&mut stale.workspace);
    let receipt = evaluate_admission(&compilation.frozen, &stale, &fixture.budget).unwrap();
    assert_eq!(receipt.status(), WorkOrderAdmissionStatusV1::Stale);

    let mut unknown = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    unknown.workspace.freshness = EvidenceFreshnessV1::Unknown;
    rebind_workspace(&mut unknown.workspace);
    let receipt = evaluate_admission(&compilation.frozen, &unknown, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(receipt
        .reason_codes()
        .contains(&WorkOrderErrorCodeV1::UnknownNotAdmissible));

    let mut bad_lock = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    bad_lock
        .context_lock
        .as_mut()
        .unwrap()
        .authorized_base_fingerprint = EvidenceFingerprintV1::new("f".repeat(64)).unwrap();
    let receipt = evaluate_admission(&compilation.frozen, &bad_lock, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(receipt
        .reason_codes()
        .contains(&WorkOrderErrorCodeV1::ContextLockStale));

    let mut bad_governance =
        ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    bad_governance.governance.as_mut().unwrap().exact_head = None;
    let receipt =
        evaluate_admission(&compilation.frozen, &bad_governance, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(receipt
        .reason_codes()
        .contains(&WorkOrderErrorCodeV1::GovernanceProofMismatch));
}

#[test]
fn receipt_identity_mismatch_and_serialized_tampering_are_rejected() {
    let fixture = fixture();
    let compilation = compiled(&fixture);
    let request = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    let receipt = evaluate_admission(&compilation.frozen, &request, &fixture.budget).unwrap();

    let mut wrong_identity = request.clone();
    wrong_identity.revision = WorkOrderRevision::new(2).unwrap();
    let error =
        evaluate_admission(&compilation.frozen, &wrong_identity, &fixture.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::ReceiptReplay);

    let mut altered = serde_json::to_value(&receipt).unwrap();
    altered["workspace_generation"] = serde_json::Value::from(999);
    let forged: WorkOrderAdmissionReceiptV1 = serde_json::from_value(altered).unwrap();
    let error = materialize_handoff(&compilation.frozen, &forged, &fixture.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::ReceiptReplay);
}

#[test]
fn blocked_receipt_cannot_materialize_a_handoff() {
    let fixture = fixture();
    let compilation = compiled(&fixture);
    let mut request = ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    request.context_lock = None;
    let receipt = evaluate_admission(&compilation.frozen, &request, &fixture.budget).unwrap();
    assert_ne!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    assert!(materialize_handoff(&compilation.frozen, &receipt, &fixture.budget).is_err());
}

#[test]
fn caller_deadline_discards_late_compilation_before_admission() {
    let fixture = fixture();
    let request = fixture.request.clone();
    let context = fixture.context.clone();
    let budget = fixture.budget.clone();
    let (release_tx, release_rx) = mpsc::sync_channel::<()>(0);
    let (result_tx, result_rx) = mpsc::channel();

    let worker = std::thread::spawn(move || {
        release_rx.recv().unwrap();
        result_tx
            .send(core_work_order::compile(&request, &context, &budget))
            .unwrap();
    });

    assert!(matches!(
        result_rx.recv_timeout(Duration::ZERO),
        Err(mpsc::RecvTimeoutError::Timeout)
    ));
    let caller_deadline_expired = true;
    release_tx.send(()).unwrap();

    let late_result = result_rx.recv().unwrap();
    assert!(late_result.is_ok());
    let accepted = if caller_deadline_expired {
        None
    } else {
        Some(late_result.unwrap().frozen)
    };
    assert!(accepted.is_none());
    worker.join().unwrap();
}
