use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::{EvidenceFingerprintV1, WorkOrderCompilationId, WorkOrderFingerprint};
use core_identity::{canonical_bytes, fingerprint};
use serde::Serialize;

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct SemanticProjectionV1<'a> {
    schema: &'static str,
    version: u16,
    kind: WorkOrderContractKindV1,
    work_order_id: &'a crate::identity::WorkOrderId,
    revision: crate::identity::WorkOrderRevision,
    semantic: &'a WorkOrderSemanticV1,
    source_manifest: &'a [CanonicalSourceRefV1],
    workspace_requirement: &'a WorkspaceRequirementV1,
    context_lock_requirement: &'a ContextLockRequirementV1,
    governance_requirement: &'a GovernanceRequirementV1,
    lineage_parent: &'a Option<WorkOrderRevisionRefV1>,
}

pub(crate) fn digest<T: Serialize>(value: T) -> Result<EvidenceFingerprintV1, WorkOrderErrorV1> {
    fingerprint(&value)
        .map_err(|_| {
            error(
                Category::InternalInvariant,
                Code::InternalInvariantViolation,
            )
        })
        .and_then(EvidenceFingerprintV1::new)
}

pub(crate) fn normalize_request(request: &mut WorkOrderRequestV1) {
    request
        .sources
        .sort_by(|a, b| a.source_id.cmp(&b.source_id));
    for source in &mut request.sources {
        source.required_packet_ids.sort();
        source.required_packet_ids.dedup();
    }
    request.workspace.required_basis_components.sort();
    request.workspace.required_basis_components.dedup();
    request.context_lock.required_source_ids.sort();
    request.context_lock.required_source_ids.dedup();
    normalize_scope(&mut request.scope);
    normalize_context_budget(&mut request.context_budget);
    normalize_stop_condition(&mut request.stop_condition);
    request.correction_policy.same_revision_classes.sort();
    request.correction_policy.same_revision_classes.dedup();
    request.correction_policy.forbidden_classes.sort();
    request.correction_policy.forbidden_classes.dedup();
    normalize_acceptance(&mut request.acceptance);
    request
        .packets
        .sort_by(|a, b| a.packet_id.cmp(&b.packet_id));
    for packet in &mut request.packets {
        packet.prerequisite_packet_ids.sort();
        packet.prerequisite_packet_ids.dedup();
        packet.required_source_ids.sort();
        packet.required_source_ids.dedup();
        packet.acceptance_criterion_ids.sort();
        packet.acceptance_criterion_ids.dedup();
        packet.evidence_requirement_ids.sort();
        packet.evidence_requirement_ids.dedup();
        normalize_scope(&mut packet.scope);
        normalize_context_budget(&mut packet.context_budget);
        normalize_stop_condition(&mut packet.stop_condition);
    }
}

pub(crate) fn normalize_scope(scope: &mut ScopeEnvelopeV1) {
    for values in [
        &mut scope.allowed_modules,
        &mut scope.denied_modules,
        &mut scope.allowed_crates_or_packages,
        &mut scope.denied_crates_or_packages,
        &mut scope.allowed_path_prefixes,
        &mut scope.denied_path_prefixes,
        &mut scope.allowed_artifact_classes,
        &mut scope.denied_artifact_classes,
        &mut scope.dependency_policy.allowed_direct_dependencies,
    ] {
        values.sort();
        values.dedup();
    }
    scope.allowed_correction_classes.sort();
    scope.allowed_correction_classes.dedup();
}

pub(crate) fn normalize_context_budget(budget: &mut ContextBudgetEnvelopeV1) {
    budget.mandatory_source_ids.sort();
    budget.mandatory_source_ids.dedup();
    budget.allowed_expansion_reasons.sort();
    budget.allowed_expansion_reasons.dedup();
}

pub(crate) fn normalize_stop_condition(stop: &mut StopConditionV1) {
    stop.success_predicates.sort();
    stop.success_predicates.dedup();
    stop.blocked_predicates.sort();
    stop.blocked_predicates.dedup();
    stop.prohibited_early_exit_conditions.sort();
    stop.prohibited_early_exit_conditions.dedup();
    stop.required_acceptance_criterion_ids.sort();
    stop.required_acceptance_criterion_ids.dedup();
    stop.required_evidence_requirement_ids.sort();
    stop.required_evidence_requirement_ids.dedup();
}

pub(crate) fn normalize_acceptance(graph: &mut AcceptanceEvidenceGraphV1) {
    graph
        .criteria
        .sort_by(|a, b| a.criterion_id.cmp(&b.criterion_id));
    for criterion in &mut graph.criteria {
        criterion.required_evidence_ids.sort();
        criterion.required_evidence_ids.dedup();
        criterion.packet_ids.sort();
        criterion.packet_ids.dedup();
    }
    graph
        .evidence_requirements
        .sort_by(|a, b| a.evidence_id.cmp(&b.evidence_id));
    for evidence in &mut graph.evidence_requirements {
        evidence.platform_requirements.sort();
        evidence.platform_requirements.dedup();
        evidence.packet_ids.sort();
        evidence.packet_ids.dedup();
    }
    graph.edges.sort();
    graph.edges.dedup();
}

pub(crate) fn semantic_projection_bytes(
    frozen: &FrozenWorkOrderV1,
) -> Result<Vec<u8>, WorkOrderErrorV1> {
    let projection = SemanticProjectionV1 {
        schema: M03_SCHEMA,
        version: M03_VERSION,
        kind: WorkOrderContractKindV1::Frozen,
        work_order_id: frozen.work_order_id(),
        revision: frozen.revision(),
        semantic: frozen.semantic(),
        source_manifest: frozen.source_manifest(),
        workspace_requirement: frozen.workspace_requirement(),
        context_lock_requirement: frozen.context_lock_requirement(),
        governance_requirement: frozen.governance_requirement(),
        lineage_parent: &frozen.lineage().parent,
    };
    canonical_bytes(&projection).map_err(|_| {
        error(
            Category::InternalInvariant,
            Code::InternalInvariantViolation,
        )
    })
}

pub(crate) fn semantic_fingerprint(
    frozen: &FrozenWorkOrderV1,
) -> Result<WorkOrderFingerprint, WorkOrderErrorV1> {
    let bytes = semantic_projection_bytes(frozen)?;
    WorkOrderFingerprint::new(core_identity::fingerprint_bytes(&bytes))
}

pub(crate) fn compilation_id(
    semantic_fingerprint: &WorkOrderFingerprint,
    context: &CompilationContextV1,
) -> Result<WorkOrderCompilationId, WorkOrderErrorV1> {
    let context_fingerprint = compilation_context_fingerprint(context)?;
    let identity = (
        "nexlabs.core.work-order.compilation.v1",
        semantic_fingerprint,
        context.compiler_contract_version,
        &context.algorithm_version,
        context.policy_generation,
        context.security_generation,
        context.config_generation,
        context_fingerprint,
    );
    WorkOrderCompilationId::new(fingerprint(&identity).map_err(|_| {
        error(
            Category::InternalInvariant,
            Code::InternalInvariantViolation,
        )
    })?)
}

pub(crate) fn compilation_context_fingerprint(
    context: &CompilationContextV1,
) -> Result<EvidenceFingerprintV1, WorkOrderErrorV1> {
    let mut normalized = context.clone();
    normalized
        .sources
        .entries
        .sort_by(|a, b| a.source_id.cmp(&b.source_id));
    normalized
        .hive_context_refs
        .sort_by(|a, b| a.context_id.cmp(&b.context_id));
    normalized
        .lineage
        .superseded_revisions
        .sort_by_key(|a| a.revision);
    normalized
        .lineage
        .edges
        .sort_by(|a, b| a.edge_id.cmp(&b.edge_id));
    digest(normalized)
}
