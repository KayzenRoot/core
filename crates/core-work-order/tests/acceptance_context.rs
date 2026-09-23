mod common;

use common::{compiled, fixture, ready_admission};
use core_work_order::*;
use std::collections::BTreeSet;

#[test]
fn blocking_criteria_require_resolvable_evidence_edges() {
    let mut missing_edge = fixture();
    missing_edge.request.acceptance.edges.clear();
    assert!(compile(
        &missing_edge.request,
        &missing_edge.context,
        &missing_edge.budget
    )
    .is_err());

    let mut dangling_edge = fixture();
    dangling_edge.request.acceptance.edges[0].evidence_id =
        EvidenceRequirementId::new("missing-evidence").unwrap();
    assert!(compile(
        &dangling_edge.request,
        &dangling_edge.context,
        &dangling_edge.budget
    )
    .is_err());
}

#[test]
fn unexplained_orphans_and_packet_obligation_mismatches_fail_closed() {
    let mut orphan = fixture();
    let mut evidence = orphan.request.acceptance.evidence_requirements[0].clone();
    evidence.evidence_id = EvidenceRequirementId::new("orphan-evidence").unwrap();
    evidence.packet_ids = vec![WorkPacketId::new("packet-1").unwrap()];
    orphan
        .request
        .acceptance
        .evidence_requirements
        .push(evidence);
    assert!(compile(&orphan.request, &orphan.context, &orphan.budget).is_err());

    let mut unbound = fixture();
    unbound.request.packets[0].evidence_requirement_ids.clear();
    assert!(compile(&unbound.request, &unbound.context, &unbound.budget).is_err());
}

#[test]
fn mandatory_pcm_reconstruction_is_lossless_and_bounded() {
    let fixture = fixture();
    let compilation = compiled(&fixture);
    let admission_request =
        ready_admission(&fixture.request, &fixture.context, &compilation.frozen);
    let receipt =
        evaluate_admission(&compilation.frozen, &admission_request, &fixture.budget).unwrap();
    assert_eq!(
        receipt.status(),
        WorkOrderAdmissionStatusV1::Ready,
        "reasons: {:?}",
        receipt.reason_codes()
    );
    let handoff = materialize_handoff(&compilation.frozen, &receipt, &fixture.budget).unwrap();

    let reconstructed = reconstruct_mandatory_source_ids(handoff.context_plan());
    let independently_required: BTreeSet<_> = handoff
        .context_plan()
        .stable_prefix_source_ids
        .iter()
        .chain(handoff.context_plan().packet_required_source_ids.iter())
        .cloned()
        .collect();
    assert_eq!(reconstructed, independently_required);
    assert_eq!(handoff.context_plan().packet_id.as_str(), "packet-1");

    let mut too_small = fixture;
    too_small.request.context_budget.max_manifest_entries = 1;
    let error = compile(&too_small.request, &too_small.context, &too_small.budget).unwrap_err();
    assert_eq!(error.category, WorkOrderErrorCategoryV1::Resource);
}
