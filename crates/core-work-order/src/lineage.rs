use crate::budget::{ensure_count, M03ResourceBudgetV1};
use crate::canonical::digest;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::WorkOrderRevision;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn validate_snapshot(
    snapshot: &LineageSnapshotV1,
    work_order_id: &crate::identity::WorkOrderId,
    parent: Option<&WorkOrderRevisionRefV1>,
    proposed_revision: WorkOrderRevision,
    budget: &M03ResourceBudgetV1,
) -> Result<Option<LineagePreconditionCapsuleV1>, WorkOrderErrorV1> {
    ensure_count(
        snapshot.superseded_revisions.len(),
        budget.max_lineage_edges,
    )?;
    ensure_count(snapshot.edges.len(), budget.max_lineage_edges)?;
    if &snapshot.work_order_id != work_order_id
        || snapshot.current_revision.is_some() != snapshot.current_fingerprint.is_some()
    {
        return Err(error(Category::Lineage, Code::SnapshotStale));
    }
    let mut superseded_revisions = snapshot.superseded_revisions.clone();
    superseded_revisions.sort_by_key(|a| a.revision);
    let mut edges = snapshot.edges.clone();
    edges.sort_by(|a, b| a.edge_id.cmp(&b.edge_id));
    let expected_snapshot_fp = digest((
        &snapshot.work_order_id,
        snapshot.current_revision,
        &snapshot.current_fingerprint,
        &snapshot.current_compilation_id,
        snapshot.store_generation,
        superseded_revisions,
        edges,
        &snapshot.provenance_fingerprint,
    ))?;
    if expected_snapshot_fp != snapshot.snapshot_fingerprint {
        return Err(error(Category::Lineage, Code::SnapshotStale));
    }
    let mut revisions = BTreeSet::new();
    let mut fingerprints = BTreeMap::new();
    for reference in &snapshot.superseded_revisions {
        if reference.work_order_id != *work_order_id || !revisions.insert(reference.revision) {
            return Err(error(Category::Lineage, Code::LineageConflict));
        }
        if snapshot
            .current_revision
            .is_some_and(|current| reference.revision >= current)
        {
            return Err(error(Category::Lineage, Code::LineageConflict));
        }
        fingerprints.insert(reference.revision, reference.fingerprint.clone());
    }
    if let (Some(revision), Some(fingerprint)) = (
        snapshot.current_revision,
        snapshot.current_fingerprint.as_ref(),
    ) {
        if fingerprints.insert(revision, fingerprint.clone()).is_some() {
            return Err(error(Category::Lineage, Code::LineageConflict));
        }
    } else if snapshot.current_compilation_id.is_some() {
        return Err(error(Category::Lineage, Code::SnapshotStale));
    }
    let mut edge_ids = BTreeSet::new();
    let mut edge_values = BTreeSet::new();
    for edge in &snapshot.edges {
        if !edge_ids.insert(&edge.edge_id)
            || !edge_values.insert((edge.from_revision, edge.to_revision, edge.relation))
        {
            return Err(error(Category::Lineage, Code::LineageConflict));
        }
        if edge.from_revision >= edge.to_revision {
            return Err(error(Category::Lineage, Code::LineageConflict));
        }
        if fingerprints.get(&edge.from_revision) != Some(&edge.from_fingerprint)
            || fingerprints.get(&edge.to_revision) != Some(&edge.to_fingerprint)
        {
            return Err(error(Category::Lineage, Code::LpcMismatch));
        }
    }
    let expected_next = match (
        snapshot.current_revision,
        snapshot.current_fingerprint.clone(),
        parent,
    ) {
        (None, None, None) => WorkOrderRevision::new(1)?,
        (Some(current), Some(fingerprint), Some(parent_ref)) => {
            if parent_ref.work_order_id != *work_order_id
                || parent_ref.revision != current
                || parent_ref.fingerprint != fingerprint
            {
                return Err(error(Category::Lineage, Code::LpcMismatch));
            }
            current.next()?
        }
        (Some(_), Some(_), None) => return Err(error(Category::Lineage, Code::RevisionNotNext)),
        (None, None, Some(_)) => return Err(error(Category::Lineage, Code::LpcMismatch)),
        _ => return Err(error(Category::Lineage, Code::SnapshotStale)),
    };
    if proposed_revision != expected_next {
        return Err(error(Category::Lineage, Code::RevisionNotNext));
    }
    let capsule_fingerprint = digest((
        work_order_id,
        &snapshot.current_revision,
        &snapshot.current_fingerprint,
        snapshot.store_generation,
        proposed_revision,
    ))?;
    Ok(Some(LineagePreconditionCapsuleV1 {
        work_order_id: work_order_id.clone(),
        expected_parent_revision: snapshot.current_revision,
        expected_parent_fingerprint: snapshot.current_fingerprint.clone(),
        expected_store_generation: snapshot.store_generation,
        proposed_revision,
        proposed_fingerprint: crate::identity::WorkOrderFingerprint::new("0".repeat(64))?,
        precondition_fingerprint: capsule_fingerprint,
    }))
}

pub(crate) fn bind_capsule_fingerprint(
    capsule: &mut LineagePreconditionCapsuleV1,
    proposed_fingerprint: crate::identity::WorkOrderFingerprint,
) -> Result<(), WorkOrderErrorV1> {
    capsule.proposed_fingerprint = proposed_fingerprint;
    capsule.precondition_fingerprint = digest((
        &capsule.work_order_id,
        &capsule.expected_parent_revision,
        &capsule.expected_parent_fingerprint,
        capsule.expected_store_generation,
        capsule.proposed_revision,
        &capsule.proposed_fingerprint,
    ))?;
    Ok(())
}

pub fn classify_lineage_cas(result: LineageCasResultV1) -> Result<(), WorkOrderErrorV1> {
    match result {
        LineageCasResultV1::Applied => Ok(()),
        LineageCasResultV1::Conflict => Err(error(Category::Lineage, Code::LineageConflict)),
        LineageCasResultV1::Rejected => Err(error(Category::Lineage, Code::LpcMismatch)),
    }
}

pub(crate) fn validate_capsule(
    capsule: &LineagePreconditionCapsuleV1,
) -> Result<(), WorkOrderErrorV1> {
    let expected = digest((
        &capsule.work_order_id,
        &capsule.expected_parent_revision,
        &capsule.expected_parent_fingerprint,
        capsule.expected_store_generation,
        capsule.proposed_revision,
        &capsule.proposed_fingerprint,
    ))?;
    if expected != capsule.precondition_fingerprint {
        return Err(error(Category::Lineage, Code::LpcMismatch));
    }
    Ok(())
}
