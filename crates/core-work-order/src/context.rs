use crate::budget::{ensure_count, M03ResourceBudgetV1};
use crate::canonical::digest;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::{ContextRefId, SourceRefId};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn validate_context_budget(
    budget: &ContextBudgetEnvelopeV1,
    source_manifest: &[CanonicalSourceRefV1],
    packet: Option<&WorkPacketSpecV1>,
    resource_budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    let dimensions = [
        budget.max_manifest_entries,
        budget.max_inline_bytes,
        budget.max_expanded_source_bytes,
        budget.max_packet_inline_bytes,
        budget.max_packet_expanded_bytes,
        budget.max_hive_refs,
        budget.max_prior_evidence_refs,
    ];
    if dimensions
        .iter()
        .any(|value| *value == 0 || *value == u64::MAX)
    {
        return Err(error(Category::Resource, Code::InvalidContextBudget));
    }
    ensure_count(source_manifest.len(), budget.max_manifest_entries)?;
    ensure_count(source_manifest.len(), resource_budget.max_context_refs)?;
    let source_ids: BTreeSet<_> = source_manifest.iter().map(|s| &s.source_id).collect();
    if budget
        .mandatory_source_ids
        .iter()
        .any(|id| !source_ids.contains(id))
    {
        return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
    }
    if let Some(packet) = packet {
        let mandatory: BTreeSet<_> = budget.mandatory_source_ids.iter().collect();
        if packet
            .required_source_ids
            .iter()
            .any(|id| !source_ids.contains(id))
            || mandatory.iter().any(|id| !source_ids.contains(*id))
        {
            return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
        }
        ensure_count(
            packet.required_source_ids.len(),
            budget.max_manifest_entries,
        )?;
    }
    Ok(())
}

pub(crate) fn build_packet_context_plans(
    dag: &WorkPacketDagV1,
    source_manifest: &[CanonicalSourceRefV1],
    global_budget: &ContextBudgetEnvelopeV1,
    resource_budget: &M03ResourceBudgetV1,
) -> Result<Vec<PacketContextPlanV1>, WorkOrderErrorV1> {
    validate_context_budget(global_budget, source_manifest, None, resource_budget)?;
    let source_map: BTreeMap<_, _> = source_manifest.iter().map(|s| (&s.source_id, s)).collect();
    let mut plans = Vec::with_capacity(dag.packets.len());
    for packet in &dag.packets {
        validate_context_budget(
            &packet.context_budget,
            source_manifest,
            Some(packet),
            resource_budget,
        )?;
        let mut stable_prefix = global_budget.mandatory_source_ids.clone();
        stable_prefix.sort();
        stable_prefix.dedup();
        let mut packet_required = packet.required_source_ids.clone();
        packet_required.sort();
        packet_required.dedup();
        let mut validate_only = Vec::new();
        let mut optional = Vec::new();
        for source_id in stable_prefix.iter().chain(packet_required.iter()) {
            let source = source_map
                .get(source_id)
                .ok_or_else(|| error(Category::AcceptanceEvidence, Code::DanglingReference))?;
            match source.expansion_policy {
                SourceExpansionPolicyV1::ValidateFingerprintOnly => {
                    validate_only.push(source_id.clone())
                }
                SourceExpansionPolicyV1::OptionalDiagnostic => optional.push(source_id.clone()),
                SourceExpansionPolicyV1::AlwaysLoad | SourceExpansionPolicyV1::PacketOnDemand => {}
            }
        }
        if !packet.context_budget.allowed_expansion_reasons.is_empty() {
            for source in source_manifest {
                if source.expansion_policy == SourceExpansionPolicyV1::OptionalDiagnostic
                    && !stable_prefix.contains(&source.source_id)
                    && !packet_required.contains(&source.source_id)
                {
                    optional.push(source.source_id.clone());
                }
            }
        }
        validate_only.sort();
        validate_only.dedup();
        optional.sort();
        optional.dedup();
        let projection = (
            "nexlabs.core.work-order.packet-context.v1",
            &packet.packet_id,
            &stable_prefix,
            &packet_required,
            &validate_only,
            &optional,
            &packet.context_budget,
        );
        let manifest_fingerprint = digest(projection)?;
        let plan = PacketContextPlanV1 {
            packet_id: packet.packet_id.clone(),
            stable_prefix_source_ids: stable_prefix,
            packet_required_source_ids: packet_required,
            validate_only_source_ids: validate_only,
            optional_expandable_source_ids: optional,
            context_budget: packet.context_budget.clone(),
            expansion_reasons: packet.context_budget.allowed_expansion_reasons.clone(),
            manifest_fingerprint,
        };
        let reconstructed = reconstruct_mandatory_source_ids(&plan);
        let independently_required: BTreeSet<_> = global_budget
            .mandatory_source_ids
            .iter()
            .chain(packet.required_source_ids.iter())
            .cloned()
            .collect();
        if reconstructed != independently_required {
            return Err(error(
                Category::AcceptanceEvidence,
                Code::PartialOutputForbidden,
            ));
        }
        plans.push(plan);
    }
    Ok(plans)
}

pub fn reconstruct_mandatory_source_ids(plan: &PacketContextPlanV1) -> BTreeSet<SourceRefId> {
    plan.stable_prefix_source_ids
        .iter()
        .chain(plan.packet_required_source_ids.iter())
        .cloned()
        .collect()
}

pub(crate) fn build_context_mesh(
    dag: &WorkPacketDagV1,
    source_manifest: &[CanonicalSourceRefV1],
    global_budget: &ContextBudgetEnvelopeV1,
    budget: &M03ResourceBudgetV1,
) -> Result<PacketContextMeshV1, WorkOrderErrorV1> {
    ensure_count(source_manifest.len(), budget.max_context_refs)?;
    let mut shared_sources = Vec::new();
    let mut refs_by_source = BTreeMap::new();
    for source in source_manifest {
        let context_digest = digest(source.source_id.as_str())?;
        let id = ContextRefId::new(format!("ctx.v1.{}", context_digest.as_str()))?;
        refs_by_source.insert(source.source_id.clone(), id.clone());
        shared_sources.push(ContextMeshSourceV1 {
            context_ref_id: id,
            source_id: source.source_id.clone(),
            source_fingerprint: source.expected_semantic_fingerprint.clone(),
            expansion_policy: source.expansion_policy,
        });
    }
    shared_sources.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    let mut packet_edges = Vec::new();
    for packet in &dag.packets {
        let mut required: BTreeSet<_> = global_budget
            .mandatory_source_ids
            .iter()
            .chain(packet.required_source_ids.iter())
            .collect();
        if !packet.context_budget.allowed_expansion_reasons.is_empty() {
            required.extend(source_manifest.iter().filter_map(|source| {
                (source.expansion_policy == SourceExpansionPolicyV1::OptionalDiagnostic
                    && !global_budget
                        .mandatory_source_ids
                        .contains(&source.source_id)
                    && !packet.required_source_ids.contains(&source.source_id))
                .then_some(&source.source_id)
            }));
        }
        for source in source_manifest {
            if required.contains(&source.source_id) {
                packet_edges.push(PacketContextEdgeV1 {
                    packet_id: packet.packet_id.clone(),
                    context_ref_id: refs_by_source[&source.source_id].clone(),
                });
            }
        }
    }
    packet_edges.sort();
    ensure_count(packet_edges.len(), budget.max_context_refs)?;
    let fingerprint = digest((&shared_sources, &packet_edges))?;
    Ok(PacketContextMeshV1 {
        shared_sources,
        packet_edges,
        fingerprint,
    })
}
