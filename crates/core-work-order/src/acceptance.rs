use crate::budget::{ensure_count, M03ResourceBudgetV1};
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::{AcceptanceCriterionId, EvidenceRequirementId};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn validate_graph(
    graph: &AcceptanceEvidenceGraphV1,
    dag: &WorkPacketDagV1,
    stop: &StopConditionV1,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    ensure_count(graph.criteria.len(), budget.max_criteria)?;
    ensure_count(
        graph.evidence_requirements.len(),
        budget.max_evidence_requirements,
    )?;
    ensure_count(graph.edges.len(), budget.max_acceptance_evidence_edges)?;
    let packet_ids: BTreeSet<_> = dag.packets.iter().map(|p| &p.packet_id).collect();
    let mut criteria = BTreeMap::new();
    for criterion in &graph.criteria {
        if criteria
            .insert(&criterion.criterion_id, criterion)
            .is_some()
        {
            return Err(error(Category::PacketGraph, Code::DuplicateId));
        }
        for packet_id in &criterion.packet_ids {
            if !packet_ids.contains(packet_id) {
                return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
            }
        }
        if !criterion.blocking && criterion.required_evidence_ids.is_empty() {
            continue;
        }
        if criterion.applicability_policy
            == ApplicabilityPolicyV1::DeterministicNotApplicableAllowed
            && criterion
                .not_applicable_rationale
                .as_ref()
                .is_some_and(|r| !r.trim().is_empty())
        {
            continue;
        }
        if criterion.blocking && criterion.required_evidence_ids.is_empty() {
            return Err(error(Category::AcceptanceEvidence, Code::AcceptanceGap));
        }
    }
    let mut evidence = BTreeMap::new();
    for requirement in &graph.evidence_requirements {
        if evidence
            .insert(&requirement.evidence_id, requirement)
            .is_some()
        {
            return Err(error(Category::PacketGraph, Code::DuplicateId));
        }
        for packet_id in &requirement.packet_ids {
            if !packet_ids.contains(packet_id) {
                return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
            }
        }
    }
    let mut edges = BTreeSet::new();
    let mut by_criterion =
        BTreeMap::<&AcceptanceCriterionId, BTreeSet<&EvidenceRequirementId>>::new();
    let mut evidence_degree = BTreeMap::<&EvidenceRequirementId, usize>::new();
    for edge in &graph.edges {
        if !criteria.contains_key(&edge.criterion_id) || !evidence.contains_key(&edge.evidence_id) {
            return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
        }
        if !edges.insert((&edge.criterion_id, &edge.evidence_id)) {
            return Err(error(Category::PacketGraph, Code::DuplicateId));
        }
        by_criterion
            .entry(&edge.criterion_id)
            .or_default()
            .insert(&edge.evidence_id);
        *evidence_degree.entry(&edge.evidence_id).or_default() += 1;
    }
    for criterion in &graph.criteria {
        let linked = by_criterion.get(&criterion.criterion_id);
        for required in &criterion.required_evidence_ids {
            if !evidence.contains_key(required) {
                return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
            }
            if !linked.is_some_and(|set| set.contains(required)) {
                return Err(error(Category::AcceptanceEvidence, Code::EvidenceGap));
            }
        }
        let has_required = linked.is_some_and(|set| !set.is_empty());
        if linked.is_some_and(|set| {
            set.iter()
                .any(|evidence_id| !criterion.required_evidence_ids.contains(evidence_id))
        }) {
            return Err(error(Category::AcceptanceEvidence, Code::EvidenceGap));
        }
        let na_valid = criterion.applicability_policy
            == ApplicabilityPolicyV1::DeterministicNotApplicableAllowed
            && criterion
                .not_applicable_rationale
                .as_ref()
                .is_some_and(|r| !r.trim().is_empty());
        if criterion.blocking && !has_required && !na_valid {
            return Err(error(Category::AcceptanceEvidence, Code::AcceptanceGap));
        }
    }
    for requirement in &graph.evidence_requirements {
        if evidence_degree
            .get(&requirement.evidence_id)
            .copied()
            .unwrap_or(0)
            == 0
            && !requirement.global
        {
            return Err(error(Category::AcceptanceEvidence, Code::UnexplainedOrphan));
        }
    }
    let criterion_ids: BTreeSet<_> = graph
        .criteria
        .iter()
        .map(|c| c.criterion_id.clone())
        .collect();
    let evidence_ids: BTreeSet<_> = graph
        .evidence_requirements
        .iter()
        .map(|e| e.evidence_id.clone())
        .collect();
    let required_criteria: BTreeSet<_> = stop
        .required_acceptance_criterion_ids
        .iter()
        .cloned()
        .collect();
    let required_evidence: BTreeSet<_> = stop
        .required_evidence_requirement_ids
        .iter()
        .cloned()
        .collect();
    if required_criteria.len() != stop.required_acceptance_criterion_ids.len()
        || !required_criteria.is_subset(&criterion_ids)
        || required_evidence.len() != stop.required_evidence_requirement_ids.len()
        || !required_evidence.is_subset(&evidence_ids)
    {
        return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
    }
    Ok(())
}
