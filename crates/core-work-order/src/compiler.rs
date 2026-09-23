use crate::acceptance;
use crate::budget::{
    ensure_count, ensure_serialized_size, ensure_string_size, M03ResourceBudgetV1,
};
use crate::canonical::{
    compilation_context_fingerprint, compilation_id, digest, normalize_request,
    semantic_fingerprint,
};
use crate::context;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::{
    EvidenceFingerprintV1, WorkOrderCompilationId, WorkOrderFingerprint, WorkOrderRevision,
};
use crate::lineage;
use crate::packets;
use crate::scope;
use crate::source;
use serde::Serialize;
use std::collections::BTreeSet;

const M02_SCHEMA: &str = "nexlabs.core.workspace";
const M02_VERSION: u16 = 1;

pub(crate) fn workspace_profile_components(
    profile: WorkspaceFreshnessProfileV1,
) -> BTreeSet<String> {
    use WorkspaceFreshnessProfileV1 as Profile;
    let mut components: BTreeSet<&'static str> = match profile {
        Profile::ReadMetadata => [
            "Identity",
            "RepositoryGraph",
            "ConfigGeneration",
            "SecurityPolicy",
        ]
        .into_iter()
        .collect(),
        Profile::ReadSource => [
            "Identity",
            "Authority",
            "FilesystemSemantics",
            "TrackedWorktreeState",
            "UntrackedWorktreeState",
            "ConfigGeneration",
            "SecurityPolicy",
        ]
        .into_iter()
        .collect(),
        Profile::PlanWork => [
            "Identity",
            "Authority",
            "FilesystemSemantics",
            "TrackedWorktreeState",
            "UntrackedWorktreeState",
            "ConfigGeneration",
            "SecurityPolicy",
            "RepositoryGraph",
            "HeadState",
            "IndexState",
        ]
        .into_iter()
        .collect(),
        Profile::ExecuteToolReadonly => [
            "Identity",
            "Authority",
            "FilesystemSemantics",
            "TrackedWorktreeState",
            "UntrackedWorktreeState",
            "ConfigGeneration",
            "SecurityPolicy",
            "RepositoryGraph",
            "HeadState",
            "IndexState",
        ]
        .into_iter()
        .collect(),
        Profile::MutateSource => [
            "Identity",
            "Authority",
            "FilesystemSemantics",
            "TrackedWorktreeState",
            "UntrackedWorktreeState",
            "ConfigGeneration",
            "SecurityPolicy",
            "RepositoryGraph",
            "HeadState",
            "IndexState",
        ]
        .into_iter()
        .collect(),
        Profile::GitDelivery => [
            "Identity",
            "Authority",
            "RepositoryGraph",
            "HeadState",
            "IndexState",
            "TrackedWorktreeState",
            "UntrackedWorktreeState",
            "ConfigGeneration",
            "SecurityPolicy",
        ]
        .into_iter()
        .collect(),
        Profile::HiveReconciledOperation => ["ProjectAssociation"].into_iter().collect(),
    };
    if profile == Profile::MutateSource {
        components.extend(["Authority", "FilesystemSemantics"]);
    }
    components.into_iter().map(str::to_owned).collect()
}

pub(crate) fn parse_request(
    input: &[u8],
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderEnvelope<WorkOrderRequestV1>, WorkOrderErrorV1> {
    budget.validate()?;
    if u64::try_from(input.len()).map_or(true, |size| size > budget.max_request_bytes) {
        return Err(error(Category::Resource, Code::RequestTooLarge));
    }
    validate_json_lexical_limits(input, budget.max_parse_depth, budget.max_string_bytes)?;
    let raw: serde_json::Value = serde_json::from_slice(input)
        .map_err(|_| error(Category::SchemaVersion, Code::InvalidEnvelope))?;
    let Some(object) = raw.as_object() else {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    };
    if object.get("schema").and_then(serde_json::Value::as_str) != Some(M03_SCHEMA) {
        return Err(error(Category::SchemaVersion, Code::UnsupportedSchema));
    }
    if object.get("version").and_then(serde_json::Value::as_u64) != Some(u64::from(M03_VERSION)) {
        return Err(error(Category::SchemaVersion, Code::UnsupportedVersion));
    }
    if object.get("kind").and_then(serde_json::Value::as_str) != Some("request") {
        return Err(error(Category::SchemaVersion, Code::InvalidKind));
    }
    let envelope: WorkOrderEnvelope<WorkOrderRequestV1> =
        serde_json::from_slice(input).map_err(|parse_error| {
            let safe_message = parse_error.to_string();
            let code = if safe_message.contains("unknown variant") {
                Code::InvalidEnumValue
            } else if safe_message.contains("invalid typed identifier")
                || safe_message.contains("invalid fingerprint")
            {
                Code::InvalidId
            } else if safe_message.contains("revision must be positive") {
                Code::InvalidRevision
            } else {
                Code::InvalidEnvelope
            };
            error(Category::SchemaVersion, code)
        })?;
    validate_request(&envelope.payload, budget)?;
    Ok(envelope)
}

pub(crate) fn validate_request(
    request: &WorkOrderRequestV1,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    budget.validate()?;
    let encoded = serde_json::to_vec(request)
        .map_err(|_| error(Category::SchemaVersion, Code::InvalidEnvelope))?;
    if u64::try_from(encoded.len()).map_or(true, |size| size > budget.max_request_bytes) {
        return Err(error(Category::Resource, Code::RequestTooLarge));
    }
    validate_value_string_limits(request, budget)?;
    validate_request_structure(request, budget)
}

fn validate_request_structure(
    request: &WorkOrderRequestV1,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    ensure_string_size(&request.objective, budget.max_string_bytes)?;
    if request.objective.trim().is_empty() {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    }
    if request.sources.is_empty()
        || !request
            .sources
            .iter()
            .any(|source| source.required_for_compile)
        || request.packets.is_empty()
        || request.acceptance.criteria.is_empty()
        || request.acceptance.evidence_requirements.is_empty()
    {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    }
    if request.requested_work_order_id.is_some() == request.logical_key.is_some() {
        return Err(error(Category::SchemaVersion, Code::InvalidId));
    }
    if let Some(key) = &request.logical_key {
        key.validate()?;
    }
    if request.workspace.required_m02_schema != M02_SCHEMA
        || request.workspace.required_m02_version != M02_VERSION
    {
        return Err(error(Category::SchemaVersion, Code::UnsupportedVersion));
    }
    validate_basis_components(&request.workspace.required_basis_components)?;
    if request.workspace.assurance_requirement
        == WorkspaceAssuranceRequirementV1::HiveReconciledRequired
        && (request.workspace.required_profile
            != WorkspaceFreshnessProfileV1::HiveReconciledOperation
            || !request
                .workspace
                .required_basis_components
                .iter()
                .any(|v| v == "ProjectAssociation"))
    {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    source::validate_manifest(&request.sources, budget)?;
    scope::validate_scope(&request.scope, budget)?;
    if scope::scope_fingerprint(&request.scope)? != request.governance.authorized_scope_fingerprint
    {
        return Err(error(Category::ScopeDelta, Code::ForbiddenDelta));
    }
    validate_request_duplicates(request)?;
    let dag = packets::build_dag(request.packets.clone(), &request.scope, budget)?;
    let packet_ids: BTreeSet<_> = dag.packets.iter().map(|p| p.packet_id.clone()).collect();
    let source_ids: BTreeSet<_> = request
        .sources
        .iter()
        .map(|s| s.source_id.clone())
        .collect();
    if request
        .context_lock
        .required_source_ids
        .iter()
        .any(|id| !source_ids.contains(id))
    {
        return Err(error(Category::SourceProvenance, Code::SourceMissing));
    }
    if request.context_lock.required_source_ids.iter().any(|id| {
        request.sources.iter().any(|source| {
            source.source_id == *id && source.source_class == SourceClassV1::ContextLock
        })
    }) {
        return Err(error(
            Category::SourceProvenance,
            Code::SourceAuthorityMismatch,
        ));
    }
    for source_ref in &request.sources {
        if source_ref
            .required_packet_ids
            .iter()
            .any(|id| !packet_ids.contains(id))
        {
            return Err(error(Category::SourceProvenance, Code::DanglingReference));
        }
    }
    for packet in &dag.packets {
        if packet
            .required_source_ids
            .iter()
            .any(|id| !source_ids.contains(id))
        {
            return Err(error(Category::SourceProvenance, Code::DanglingReference));
        }
        context::validate_context_budget(
            &packet.context_budget,
            &request.sources,
            Some(packet),
            budget,
        )?;
    }
    context::validate_context_budget(&request.context_budget, &request.sources, None, budget)?;
    acceptance::validate_graph(&request.acceptance, &dag, &request.stop_condition, budget)?;
    validate_packet_obligations(&dag, &request.acceptance)?;
    if request.context_lock.required_schema.is_empty()
        || request.context_lock.required_version == 0
        || request.context_lock.authorized_base_constraint.is_empty()
    {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    }
    if request.governance.minimum_policy_generation == 0
        || request
            .governance
            .authorized_scope_fingerprint
            .as_str()
            .is_empty()
        || (request.governance.require_exact_head && !request.governance.require_exact_base)
    {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    }
    validate_stop_condition(&request.stop_condition, &request.acceptance)?;
    Ok(())
}

fn validate_request_duplicates(request: &WorkOrderRequestV1) -> Result<(), WorkOrderErrorV1> {
    check_unique(request.sources.iter().map(|s| &s.source_id))?;
    check_unique(request.context_budget.mandatory_source_ids.iter())?;
    check_unique(request.context_lock.required_source_ids.iter())?;
    check_unique(request.workspace.required_basis_components.iter())?;
    check_unique(request.scope.allowed_correction_classes.iter())?;
    check_unique(request.correction_policy.same_revision_classes.iter())?;
    check_unique(request.correction_policy.forbidden_classes.iter())?;
    check_unique(
        request
            .stop_condition
            .required_acceptance_criterion_ids
            .iter(),
    )?;
    check_unique(
        request
            .stop_condition
            .required_evidence_requirement_ids
            .iter(),
    )?;
    check_unique(request.stop_condition.success_predicates.iter())?;
    check_unique(request.stop_condition.blocked_predicates.iter())?;
    check_unique(
        request
            .stop_condition
            .prohibited_early_exit_conditions
            .iter(),
    )?;
    check_unique(request.context_budget.allowed_expansion_reasons.iter())?;
    for packet in &request.packets {
        check_unique(packet.prerequisite_packet_ids.iter())?;
        check_unique(packet.required_source_ids.iter())?;
        check_unique(packet.acceptance_criterion_ids.iter())?;
        check_unique(packet.evidence_requirement_ids.iter())?;
        check_unique(packet.context_budget.mandatory_source_ids.iter())?;
        check_unique(packet.context_budget.allowed_expansion_reasons.iter())?;
        check_unique(packet.scope.allowed_correction_classes.iter())?;
    }
    check_unique(request.acceptance.criteria.iter().map(|c| &c.criterion_id))?;
    check_unique(
        request
            .acceptance
            .evidence_requirements
            .iter()
            .map(|e| &e.evidence_id),
    )?;
    check_unique(request.acceptance.edges.iter())?;
    for criterion in &request.acceptance.criteria {
        check_unique(criterion.required_evidence_ids.iter())?;
        check_unique(criterion.packet_ids.iter())?;
    }
    for evidence in &request.acceptance.evidence_requirements {
        check_unique(evidence.packet_ids.iter())?;
        check_unique(evidence.platform_requirements.iter())?;
    }
    for source_ref in &request.sources {
        check_unique(source_ref.required_packet_ids.iter())?;
    }
    Ok(())
}

fn check_unique<T: Ord>(values: impl Iterator<Item = T>) -> Result<(), WorkOrderErrorV1> {
    let mut seen = BTreeSet::new();
    if values.into_iter().all(|value| seen.insert(value)) {
        Ok(())
    } else {
        Err(error(Category::PacketGraph, Code::DuplicateId))
    }
}

pub(crate) fn validate_basis_components(values: &[String]) -> Result<(), WorkOrderErrorV1> {
    const COMPONENTS: &[&str] = &[
        "Identity",
        "Authority",
        "RepositoryGraph",
        "WorktreeIdentity",
        "HeadState",
        "IndexState",
        "TrackedWorktreeState",
        "UntrackedWorktreeState",
        "FilesystemSemantics",
        "ConfigGeneration",
        "SecurityPolicy",
        "ProjectAssociation",
        "SubmoduleState",
        "SparseCheckoutState",
    ];
    if values
        .iter()
        .any(|value| !COMPONENTS.contains(&value.as_str()))
    {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    Ok(())
}

fn validate_stop_condition(
    stop: &StopConditionV1,
    graph: &AcceptanceEvidenceGraphV1,
) -> Result<(), WorkOrderErrorV1> {
    if stop.success_predicates.is_empty()
        || stop.blocked_predicates.is_empty()
        || stop.prohibited_early_exit_conditions.is_empty()
        || !stop.reviewer_verdict_required
        || stop.checkpoint_promotion_allowed
    {
        return Err(error(Category::SchemaVersion, Code::InvalidStopCondition));
    }
    let criteria: BTreeSet<_> = graph
        .criteria
        .iter()
        .map(|c| c.criterion_id.clone())
        .collect();
    let evidence: BTreeSet<_> = graph
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
    if required_criteria != criteria || required_evidence != evidence {
        return Err(error(Category::AcceptanceEvidence, Code::DanglingReference));
    }
    Ok(())
}

fn validate_packet_obligations(
    dag: &WorkPacketDagV1,
    graph: &AcceptanceEvidenceGraphV1,
) -> Result<(), WorkOrderErrorV1> {
    for packet in &dag.packets {
        let expected_criteria: BTreeSet<_> = graph
            .criteria
            .iter()
            .filter(|criterion| criterion.packet_ids.contains(&packet.packet_id))
            .map(|criterion| criterion.criterion_id.clone())
            .collect();
        let expected_evidence: BTreeSet<_> = graph
            .evidence_requirements
            .iter()
            .filter(|evidence| evidence.packet_ids.contains(&packet.packet_id))
            .map(|evidence| evidence.evidence_id.clone())
            .collect();
        let packet_criteria: BTreeSet<_> =
            packet.acceptance_criterion_ids.iter().cloned().collect();
        let packet_evidence: BTreeSet<_> =
            packet.evidence_requirement_ids.iter().cloned().collect();
        let stop_criteria: BTreeSet<_> = packet
            .stop_condition
            .required_acceptance_criterion_ids
            .iter()
            .cloned()
            .collect();
        let stop_evidence: BTreeSet<_> = packet
            .stop_condition
            .required_evidence_requirement_ids
            .iter()
            .cloned()
            .collect();
        if packet_criteria != expected_criteria
            || packet_evidence != expected_evidence
            || stop_criteria != expected_criteria
            || stop_evidence != expected_evidence
            || packet.stop_condition.success_predicates.is_empty()
            || packet.stop_condition.blocked_predicates.is_empty()
            || packet
                .stop_condition
                .prohibited_early_exit_conditions
                .is_empty()
            || !packet.stop_condition.reviewer_verdict_required
            || packet.stop_condition.checkpoint_promotion_allowed
        {
            return Err(error(Category::AcceptanceEvidence, Code::EvidenceGap));
        }
    }
    Ok(())
}

fn validate_value_string_limits<T: Serialize>(
    value: &T,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    let value = serde_json::to_value(value)
        .map_err(|_| error(Category::SchemaVersion, Code::InvalidEnvelope))?;
    fn walk(
        value: &serde_json::Value,
        depth: u32,
        budget: &M03ResourceBudgetV1,
    ) -> Result<(), WorkOrderErrorV1> {
        if depth > budget.max_parse_depth {
            return Err(error(Category::Resource, Code::ContextLimitExceeded));
        }
        match value {
            serde_json::Value::String(text) => ensure_string_size(text, budget.max_string_bytes),
            serde_json::Value::Array(values) => {
                for child in values {
                    walk(child, depth.saturating_add(1), budget)?;
                }
                Ok(())
            }
            serde_json::Value::Object(values) => {
                for (key, child) in values {
                    ensure_string_size(key, budget.max_string_bytes)?;
                    walk(child, depth.saturating_add(1), budget)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    walk(&value, 0, budget)
}

fn validate_json_lexical_limits(
    input: &[u8],
    max_depth: u32,
    max_string: u64,
) -> Result<(), WorkOrderErrorV1> {
    let mut depth = 0u32;
    let mut in_string = false;
    let mut escaped = false;
    let mut string_start = 0usize;
    for (index, byte) in input.iter().copied().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                if u64::try_from(index.saturating_sub(string_start))
                    .map_or(true, |size| size > max_string)
                {
                    return Err(error(Category::Resource, Code::CardinalityLimitExceeded));
                }
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
            string_start = index.saturating_add(1);
        } else if byte == b'{' || byte == b'[' {
            depth = depth.saturating_add(1);
            if depth > max_depth {
                return Err(error(Category::Resource, Code::ContextLimitExceeded));
            }
        } else if byte == b'}' || byte == b']' {
            depth = depth.saturating_sub(1);
        }
    }
    if in_string {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    }
    Ok(())
}

pub(crate) fn compile(
    request: &WorkOrderRequestV1,
    context: &CompilationContextV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderCompilationV1, WorkOrderErrorV1> {
    validate_request(request, budget)?;
    ensure_count(context.hive_context_refs.len(), budget.max_context_refs)?;
    if context.compiler_contract_version != M03_VERSION
        || context.algorithm_version.is_empty()
        || context.policy_generation == 0
        || context.security_generation == 0
        || context.config_generation == 0
    {
        return Err(error(Category::SchemaVersion, Code::UnsupportedVersion));
    }
    ensure_string_size(&context.algorithm_version, budget.max_string_bytes)?;
    let mut hive_ids = BTreeSet::new();
    for hive_ref in &context.hive_context_refs {
        if !hive_ids.insert(&hive_ref.context_id) || !hive_ref.advisory_only {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceAuthorityMismatch,
            ));
        }
    }
    source::validate_source_batch(&request.sources, &context.sources, false, budget)?;
    let mut normalized = request.clone();
    normalize_request(&mut normalized);
    let work_order_id = match (&normalized.requested_work_order_id, &normalized.logical_key) {
        (Some(id), None) => id.clone(),
        (None, Some(key)) => crate::identity::WorkOrderId::from_logical_key(key)?,
        _ => return Err(error(Category::SchemaVersion, Code::InvalidId)),
    };
    let revision = match &normalized.parent {
        Some(parent) => parent.revision.next()?,
        None => WorkOrderRevision::new(1)?,
    };
    let mut capsule = lineage::validate_snapshot(
        &context.lineage,
        &work_order_id,
        normalized.parent.as_ref(),
        revision,
        budget,
    )?;
    let packet_dag = packets::build_dag(normalized.packets.clone(), &normalized.scope, budget)?;
    acceptance::validate_graph(
        &normalized.acceptance,
        &packet_dag,
        &normalized.stop_condition,
        budget,
    )?;
    validate_packet_obligations(&packet_dag, &normalized.acceptance)?;
    let plans = context::build_packet_context_plans(
        &packet_dag,
        &normalized.sources,
        &normalized.context_budget,
        budget,
    )?;
    let _mesh = context::build_context_mesh(
        &packet_dag,
        &normalized.sources,
        &normalized.context_budget,
        budget,
    )?;
    let mut lineage_ref = WorkOrderLineageRefV1 {
        parent: normalized.parent.clone(),
        edges: context.lineage.edges.clone(),
    };
    lineage_ref.edges.sort_by(|a, b| a.edge_id.cmp(&b.edge_id));
    let semantic = WorkOrderSemanticV1 {
        objective: normalized.objective.clone(),
        scope: normalized.scope.clone(),
        packet_dag,
        acceptance: normalized.acceptance.clone(),
        context_budget: normalized.context_budget.clone(),
        correction_policy: normalized.correction_policy.clone(),
        stop_condition: normalized.stop_condition.clone(),
        risk_assurance: normalized.risk_assurance.clone(),
    };
    let zero_fp = WorkOrderFingerprint::new("0".repeat(64))?;
    let zero_id = WorkOrderCompilationId::new("0".repeat(64))?;
    let mut frozen = FrozenWorkOrderV1::new(
        work_order_id.clone(),
        revision,
        zero_fp,
        zero_id,
        semantic,
        normalized.sources.clone(),
        normalized.workspace.clone(),
        normalized.context_lock.clone(),
        normalized.governance.clone(),
        lineage_ref,
    );
    let work_order_fingerprint = semantic_fingerprint(&frozen)?;
    if let Some(capsule) = capsule.as_mut() {
        lineage::bind_capsule_fingerprint(capsule, work_order_fingerprint.clone())?;
        lineage::validate_capsule(capsule)?;
    }
    let compilation_id = compilation_id(&work_order_fingerprint, context)?;
    frozen = FrozenWorkOrderV1::new(
        work_order_id.clone(),
        revision,
        work_order_fingerprint.clone(),
        compilation_id.clone(),
        frozen.semantic().clone(),
        frozen.source_manifest().to_vec(),
        frozen.workspace_requirement().clone(),
        frozen.context_lock_requirement().clone(),
        frozen.governance_requirement().clone(),
        frozen.lineage().clone(),
    );
    let request_fingerprint = digest(&normalized)?;
    let context_fingerprint = compilation_context_fingerprint(context)?;
    let output_fingerprint = semantic_fingerprint(&frozen)?;
    let receipt_fingerprint = digest((
        &request_fingerprint,
        &context_fingerprint,
        context.compiler_contract_version,
        &context.algorithm_version,
        context.policy_generation,
        context.config_generation,
        context.security_generation,
        capsule.as_ref().map(|c| &c.precondition_fingerprint),
        &output_fingerprint,
    ))?;
    let receipt = DeterministicCompilationReceiptV1 {
        request_fingerprint,
        compilation_context_fingerprint: context_fingerprint,
        compiler_contract_version: context.compiler_contract_version,
        algorithm_version: context.algorithm_version.clone(),
        policy_generation: context.policy_generation,
        config_generation: context.config_generation,
        security_generation: context.security_generation,
        lineage_precondition_fingerprint: capsule
            .as_ref()
            .map(|c| c.precondition_fingerprint.clone()),
        output_fingerprint,
        receipt_fingerprint,
    };
    let _ = plans;
    validate_frozen(&frozen, budget)?;
    Ok(WorkOrderCompilationV1 {
        frozen,
        receipt,
        lineage_precondition: capsule,
        diagnostics: Vec::new(),
    })
}

pub(crate) fn validate_frozen(
    frozen: &FrozenWorkOrderV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderValidationReceiptV1, WorkOrderErrorV1> {
    budget.validate()?;
    let encoded = serde_json::to_vec(frozen)
        .map_err(|_| error(Category::SchemaVersion, Code::InvalidEnvelope))?;
    ensure_serialized_size(encoded.len(), budget.max_frozen_bytes)?;
    validate_frozen_string_limits(frozen, budget)?;
    source::validate_manifest(frozen.source_manifest(), budget)?;
    scope::validate_scope(&frozen.semantic().scope, budget)?;
    packets::validate_dag(
        &frozen.semantic().packet_dag,
        &frozen.semantic().scope,
        budget,
    )?;
    acceptance::validate_graph(
        &frozen.semantic().acceptance,
        &frozen.semantic().packet_dag,
        &frozen.semantic().stop_condition,
        budget,
    )?;
    validate_packet_obligations(&frozen.semantic().packet_dag, &frozen.semantic().acceptance)?;
    validate_stop_condition(
        &frozen.semantic().stop_condition,
        &frozen.semantic().acceptance,
    )?;
    validate_workspace_requirement(frozen.workspace_requirement())?;
    if frozen
        .semantic()
        .correction_policy
        .same_revision_classes
        .iter()
        .any(|class| {
            frozen
                .semantic()
                .correction_policy
                .forbidden_classes
                .contains(class)
        })
    {
        return Err(error(Category::ScopeDelta, Code::ForbiddenDelta));
    }
    context::validate_context_budget(
        &frozen.semantic().context_budget,
        frozen.source_manifest(),
        None,
        budget,
    )?;
    for packet in &frozen.semantic().packet_dag.packets {
        context::validate_context_budget(
            &packet.context_budget,
            frozen.source_manifest(),
            Some(packet),
            budget,
        )?;
    }
    if let Some(parent) = &frozen.lineage().parent {
        if parent.work_order_id != *frozen.work_order_id()
            || parent.revision.next()? != frozen.revision()
        {
            return Err(error(Category::Lineage, Code::RevisionNotNext));
        }
    } else if frozen.revision() != WorkOrderRevision::new(1)? {
        return Err(error(Category::Lineage, Code::LpcMismatch));
    }
    let semantic_projection_fingerprint = EvidenceFingerprintV1::new(
        core_identity::fingerprint_bytes(&crate::canonical::semantic_projection_bytes(frozen)?),
    )?;
    if semantic_fingerprint(frozen)? != *frozen.fingerprint() {
        return Err(error(
            Category::InternalInvariant,
            Code::FingerprintMismatch,
        ));
    }
    let receipt_fingerprint = digest((
        frozen.work_order_id(),
        frozen.revision(),
        frozen.fingerprint(),
        &semantic_projection_fingerprint,
    ))?;
    Ok(WorkOrderValidationReceiptV1 {
        work_order_id: frozen.work_order_id().clone(),
        revision: frozen.revision(),
        work_order_fingerprint: frozen.fingerprint().clone(),
        semantic_projection_fingerprint,
        receipt_fingerprint,
    })
}

fn validate_workspace_requirement(
    requirement: &WorkspaceRequirementV1,
) -> Result<(), WorkOrderErrorV1> {
    if requirement.required_m02_schema != M02_SCHEMA
        || requirement.required_m02_version != M02_VERSION
        || (requirement.assurance_requirement
            == WorkspaceAssuranceRequirementV1::HiveReconciledRequired
            && (requirement.required_profile
                != WorkspaceFreshnessProfileV1::HiveReconciledOperation
                || !requirement
                    .required_basis_components
                    .iter()
                    .any(|component| component == "ProjectAssociation")))
    {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    validate_basis_components(&requirement.required_basis_components)
}

fn validate_frozen_string_limits<T: Serialize>(
    value: &T,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    let value = serde_json::to_value(value)
        .map_err(|_| error(Category::SchemaVersion, Code::InvalidEnvelope))?;
    fn visit(
        value: &serde_json::Value,
        budget: &M03ResourceBudgetV1,
    ) -> Result<(), WorkOrderErrorV1> {
        match value {
            serde_json::Value::String(text) => ensure_string_size(text, budget.max_string_bytes),
            serde_json::Value::Array(values) => values.iter().try_for_each(|v| visit(v, budget)),
            serde_json::Value::Object(values) => values.iter().try_for_each(|(k, v)| {
                ensure_string_size(k, budget.max_string_bytes)?;
                visit(v, budget)
            }),
            _ => Ok(()),
        }
    }
    visit(&value, budget)
}
