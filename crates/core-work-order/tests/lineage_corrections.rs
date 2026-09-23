mod common;

use common::{compiled, fixture, fp};
use core_work_order::*;

fn bind_next_revision(candidate: &mut common::Fixture, current: &FrozenWorkOrderV1) {
    candidate.request.parent = Some(WorkOrderRevisionRefV1 {
        work_order_id: current.work_order_id().clone(),
        revision: current.revision(),
        fingerprint: current.fingerprint().clone(),
    });
    let snapshot = &mut candidate.context.lineage;
    snapshot.current_revision = Some(current.revision());
    snapshot.current_fingerprint = Some(current.fingerprint().clone());
    snapshot.current_compilation_id = Some(current.compilation_id().clone());
    snapshot.store_generation = 1;
    let mut superseded = snapshot.superseded_revisions.clone();
    superseded.sort_by_key(|entry| entry.revision);
    let mut edges = snapshot.edges.clone();
    edges.sort_by(|left, right| left.edge_id.cmp(&right.edge_id));
    snapshot.snapshot_fingerprint = fp(&(
        &snapshot.work_order_id,
        snapshot.current_revision,
        &snapshot.current_fingerprint,
        &snapshot.current_compilation_id,
        snapshot.store_generation,
        superseded,
        edges,
        &snapshot.provenance_fingerprint,
    ));
}

fn rebind_snapshot(snapshot: &mut LineageSnapshotV1) {
    let mut superseded = snapshot.superseded_revisions.clone();
    superseded.sort_by_key(|entry| entry.revision);
    let mut edges = snapshot.edges.clone();
    edges.sort_by(|left, right| left.edge_id.cmp(&right.edge_id));
    snapshot.snapshot_fingerprint = fp(&(
        &snapshot.work_order_id,
        snapshot.current_revision,
        &snapshot.current_fingerprint,
        &snapshot.current_compilation_id,
        snapshot.store_generation,
        superseded,
        edges,
        &snapshot.provenance_fingerprint,
    ));
}

#[test]
fn lineage_capsules_bind_monotonic_revisions_and_external_cas() {
    let initial = fixture();
    let current = compiled(&initial).frozen;

    let mut first_candidate = fixture();
    bind_next_revision(&mut first_candidate, &current);
    first_candidate.request.objective.push_str(" candidate A");
    let first = compile(
        &first_candidate.request,
        &first_candidate.context,
        &first_candidate.budget,
    )
    .unwrap();

    let mut competing_candidate = fixture();
    bind_next_revision(&mut competing_candidate, &current);
    competing_candidate
        .request
        .objective
        .push_str(" candidate B");
    let competing = compile(
        &competing_candidate.request,
        &competing_candidate.context,
        &competing_candidate.budget,
    )
    .unwrap();

    assert_eq!(first.frozen.revision().get(), 2);
    assert_eq!(competing.frozen.revision().get(), 2);
    let first_lpc = first.lineage_precondition.unwrap();
    let competing_lpc = competing.lineage_precondition.unwrap();
    assert_eq!(first_lpc.expected_store_generation, 1);
    assert_eq!(competing_lpc.expected_store_generation, 1);
    assert_ne!(
        first_lpc.proposed_fingerprint,
        competing_lpc.proposed_fingerprint
    );
    assert!(classify_lineage_cas(LineageCasResultV1::Applied).is_ok());
    let conflict = classify_lineage_cas(LineageCasResultV1::Conflict).unwrap_err();
    assert_eq!(conflict.code, WorkOrderErrorCodeV1::LineageConflict);
    assert!(classify_lineage_cas(LineageCasResultV1::Rejected).is_err());
}

#[test]
fn semantic_diff_requires_next_revision_and_same_revision_corrections_are_policy_bound() {
    let input = fixture();
    let frozen = compiled(&input).frozen;

    let allowed = ExecutionCorrectionProposalV1 {
        proposal_fingerprint: EvidenceFingerprintV1::new("c".repeat(64)).unwrap(),
        changed_paths: vec!["crates/core-work-order/tests/public_contracts.rs".into()],
        artifact_classes: vec!["test".into()],
        requested_classes: vec![CorrectionClassV1::TestOnlyWithinScope],
        added_dependencies: vec![],
        changed_semantic_fields: vec![],
    };
    let receipt = classify_correction(&frozen, &allowed, &input.budget).unwrap();
    assert_eq!(
        receipt.disposition,
        CorrectionDispositionV1::AllowedSameRevision
    );

    let forbidden = ExecutionCorrectionProposalV1 {
        proposal_fingerprint: EvidenceFingerprintV1::new("d".repeat(64)).unwrap(),
        changed_paths: vec![],
        artifact_classes: vec![],
        requested_classes: vec![CorrectionClassV1::ScopeExpansion],
        added_dependencies: vec![],
        changed_semantic_fields: vec![],
    };
    let receipt = classify_correction(&frozen, &forbidden, &input.budget).unwrap();
    assert_eq!(receipt.disposition, CorrectionDispositionV1::Forbidden);
    assert!(receipt
        .reason_codes
        .contains(&WorkOrderErrorCodeV1::ForbiddenDelta));

    let mut next_candidate = fixture();
    bind_next_revision(&mut next_candidate, &frozen);
    next_candidate.request.objective.push_str(" changed");
    let next = compile(
        &next_candidate.request,
        &next_candidate.context,
        &next_candidate.budget,
    )
    .unwrap();
    let diff = diff_revision(&frozen, &next.frozen, &input.budget).unwrap();
    assert!(diff.requires_new_revision);
    assert!(diff.forbidden_reason_codes.is_empty());
}

#[test]
fn stale_lineage_snapshot_and_invalid_next_revision_are_rejected() {
    let current = compiled(&fixture()).frozen;
    let mut stale = fixture();
    bind_next_revision(&mut stale, &current);
    stale.context.lineage.store_generation += 1;
    let error = compile(&stale.request, &stale.context, &stale.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::SnapshotStale);

    let mut wrong_parent = fixture();
    bind_next_revision(&mut wrong_parent, &current);
    wrong_parent.request.parent.as_mut().unwrap().revision = WorkOrderRevision::new(2).unwrap();
    assert!(compile(
        &wrong_parent.request,
        &wrong_parent.context,
        &wrong_parent.budget
    )
    .is_err());
}

#[test]
fn lineage_edges_reject_dangling_endpoints_and_duplicate_ids() {
    let current = compiled(&fixture()).frozen;
    let mut dangling = fixture();
    bind_next_revision(&mut dangling, &current);
    dangling.context.lineage.edges.push(WorkOrderLineageEdgeV1 {
        edge_id: LineageEdgeId::new("edge-dangling").unwrap(),
        from_revision: current.revision(),
        from_fingerprint: current.fingerprint().clone(),
        to_revision: current.revision().next().unwrap(),
        to_fingerprint: WorkOrderFingerprint::new("c".repeat(64)).unwrap(),
        relation: LineageRelationV1::Supersedes,
    });
    rebind_snapshot(&mut dangling.context.lineage);
    let error = compile(&dangling.request, &dangling.context, &dangling.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::LpcMismatch);

    let mut duplicate = fixture();
    let work_order_id = duplicate.context.lineage.work_order_id.clone();
    let revision_one = WorkOrderRevision::new(1).unwrap();
    let revision_two = WorkOrderRevision::new(2).unwrap();
    let fingerprint_one = WorkOrderFingerprint::new("a".repeat(64)).unwrap();
    let fingerprint_two = WorkOrderFingerprint::new("b".repeat(64)).unwrap();
    duplicate.request.parent = Some(WorkOrderRevisionRefV1 {
        work_order_id: work_order_id.clone(),
        revision: revision_two,
        fingerprint: fingerprint_two.clone(),
    });
    duplicate.context.lineage.current_revision = Some(revision_two);
    duplicate.context.lineage.current_fingerprint = Some(fingerprint_two.clone());
    duplicate.context.lineage.superseded_revisions = vec![WorkOrderRevisionRefV1 {
        work_order_id,
        revision: revision_one,
        fingerprint: fingerprint_one.clone(),
    }];
    let edge = WorkOrderLineageEdgeV1 {
        edge_id: LineageEdgeId::new("edge-duplicate").unwrap(),
        from_revision: revision_one,
        from_fingerprint: fingerprint_one,
        to_revision: revision_two,
        to_fingerprint: fingerprint_two,
        relation: LineageRelationV1::Supersedes,
    };
    duplicate.context.lineage.edges = vec![edge.clone(), edge];
    rebind_snapshot(&mut duplicate.context.lineage);
    let error = compile(&duplicate.request, &duplicate.context, &duplicate.budget).unwrap_err();
    assert_eq!(error.code, WorkOrderErrorCodeV1::LineageConflict);
}
