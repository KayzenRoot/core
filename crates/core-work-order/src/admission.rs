use crate::budget::{ensure_count, ensure_string_size, M03ResourceBudgetV1};
use crate::canonical::digest;
use crate::context;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::source;
use std::collections::BTreeSet;

pub(crate) fn evaluate(
    frozen: &FrozenWorkOrderV1,
    request: &WorkOrderAdmissionRequestV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderAdmissionReceiptV1, WorkOrderErrorV1> {
    if request.work_order_id != *frozen.work_order_id()
        || request.revision != frozen.revision()
        || request.work_order_fingerprint != *frozen.fingerprint()
    {
        return Err(error(Category::AdmissionStaleness, Code::ReceiptReplay));
    }
    let source_check =
        source::validate_source_batch(frozen.source_manifest(), &request.sources, true, budget);
    let mut codes = Vec::new();
    let mut status = WorkOrderAdmissionStatusV1::Ready;
    if let Err(reason) = source_check {
        status = status_for(reason.code);
        codes.push(reason.code);
    }
    let workspace = &request.workspace;
    let workspace_result = validate_workspace(frozen.workspace_requirement(), workspace, budget);
    if let Err(reason) = workspace_result {
        status = more_restrictive(status, status_for(reason.code));
        codes.push(reason.code);
    }
    let source_set_fingerprint = expected_context_lock_source_set(frozen)?;
    let lock_fingerprint = request
        .context_lock
        .as_ref()
        .map(|lock| lock.lock_fingerprint.clone());
    match (
        &request.context_lock,
        frozen.context_lock_requirement().require_active,
    ) {
        (None, _) => {
            status = more_restrictive(status, WorkOrderAdmissionStatusV1::Blocked);
            codes.push(Code::ContextLockStale);
        }
        (Some(lock), _) => {
            if let Err(reason) = validate_context_lock(
                frozen,
                lock,
                &source_set_fingerprint,
                request.policy_generation,
            ) {
                status = more_restrictive(status, status_for(reason.code));
                codes.push(reason.code);
            }
        }
    }
    let governance_fingerprint = request
        .governance
        .as_ref()
        .map(|proof| proof.proof_fingerprint.clone());
    match &request.governance {
        None => {
            status = more_restrictive(status, WorkOrderAdmissionStatusV1::Blocked);
            codes.push(Code::GovernanceProofMissing);
        }
        Some(proof) => {
            if let Err(reason) = validate_governance(frozen, proof, request) {
                status = more_restrictive(status, status_for(reason.code));
                codes.push(reason.code);
            }
        }
    }
    if request.policy_generation < frozen.governance_requirement().minimum_policy_generation {
        status = more_restrictive(status, WorkOrderAdmissionStatusV1::Stale);
        codes.push(Code::PolicyStale);
    }
    if request.policy_generation == 0
        || request.security_generation == 0
        || request.config_generation == 0
    {
        status = more_restrictive(status, WorkOrderAdmissionStatusV1::Stale);
        codes.push(Code::PolicyStale);
    }
    codes.sort();
    codes.dedup();
    ensure_count(codes.len(), budget.max_diagnostic_entries)?;
    let receipt_fingerprint = digest((
        frozen.work_order_id(),
        frozen.revision(),
        frozen.fingerprint(),
        frozen.compilation_id(),
        status,
        &request.sources.batch_fingerprint,
        &workspace.workspace_id,
        workspace.runtime_epoch,
        workspace.generation,
        &workspace.basis_fingerprint,
        &lock_fingerprint,
        &governance_fingerprint,
        request.policy_generation,
        request.security_generation,
        request.config_generation,
        &codes,
    ))?;
    Ok(WorkOrderAdmissionReceiptV1::new(
        frozen.work_order_id().clone(),
        frozen.revision(),
        frozen.fingerprint().clone(),
        frozen.compilation_id().clone(),
        status,
        request.sources.batch_fingerprint.clone(),
        workspace.workspace_id.clone(),
        workspace.runtime_epoch,
        workspace.generation,
        workspace.basis_fingerprint.clone(),
        lock_fingerprint,
        governance_fingerprint,
        request.policy_generation,
        request.security_generation,
        request.config_generation,
        codes,
        receipt_fingerprint,
    ))
}

fn validate_workspace(
    requirement: &WorkspaceRequirementV1,
    evidence: &WorkspaceAdmissionEvidenceV1,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    ensure_string_size(&evidence.workspace_id, budget.max_string_bytes)?;
    ensure_count(evidence.satisfied_components.len(), budget.max_context_refs)?;
    if evidence.m02_schema != requirement.required_m02_schema
        || evidence.m02_schema != "nexlabs.core.workspace"
        || evidence.m02_version != requirement.required_m02_version
        || evidence.m02_version != 1
        || evidence.workspace_id.is_empty()
        || evidence.runtime_epoch == 0
        || evidence.generation == 0
        || requirement
            .expected_workspace_id
            .as_ref()
            .is_some_and(|expected| expected != &evidence.workspace_id)
        || requirement
            .expected_project_binding_id
            .as_ref()
            .is_some_and(|expected| evidence.project_binding_id.as_ref() != Some(expected))
    {
        return Err(error(Category::AdmissionStaleness, Code::WorkspaceMismatch));
    }
    if evidence.required_profile != requirement.required_profile {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    crate::compiler::validate_basis_components(&evidence.satisfied_components)?;
    if evidence.freshness != EvidenceFreshnessV1::Current {
        return Err(error(
            Category::AdmissionStaleness,
            freshness_code(evidence.freshness),
        ));
    }
    if evidence.compatibility == BasisCompatibilityV1::Incompatible
        || evidence.compatibility == BasisCompatibilityV1::Unknown
        || (evidence.compatibility == BasisCompatibilityV1::CompatibleRefresh
            && !requirement.allow_compatible_refresh)
    {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    let mut required: BTreeSet<_> = requirement.required_basis_components.iter().collect();
    if required.len() != requirement.required_basis_components.len() {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    let profile_components =
        crate::compiler::workspace_profile_components(requirement.required_profile);
    required.extend(profile_components.iter());
    let satisfied: BTreeSet<_> = evidence.satisfied_components.iter().collect();
    if satisfied.len() != evidence.satisfied_components.len() {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    if required.iter().any(|v| !satisfied.contains(v)) {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    if requirement.assurance_requirement == WorkspaceAssuranceRequirementV1::HiveReconciledRequired
        && !evidence
            .satisfied_components
            .iter()
            .any(|component| component == "ProjectAssociation")
    {
        return Err(error(Category::AdmissionStaleness, Code::BasisIncompatible));
    }
    let mut satisfied_components = evidence.satisfied_components.clone();
    satisfied_components.sort();
    let proof_fingerprint = digest((
        &evidence.m02_schema,
        evidence.m02_version,
        &evidence.project_binding_id,
        &evidence.workspace_id,
        evidence.runtime_epoch,
        evidence.generation,
        &evidence.basis_fingerprint,
        evidence.required_profile,
        &satisfied_components,
        evidence.compatibility,
        evidence.freshness,
        &evidence.provenance_fingerprint,
    ))?;
    if proof_fingerprint != evidence.proof_fingerprint {
        return Err(error(
            Category::AdmissionStaleness,
            Code::GovernanceProofMismatch,
        ));
    }
    Ok(())
}

fn expected_context_lock_source_set(
    frozen: &FrozenWorkOrderV1,
) -> Result<crate::identity::EvidenceFingerprintV1, WorkOrderErrorV1> {
    let required: BTreeSet<_> = frozen
        .context_lock_requirement()
        .required_source_ids
        .iter()
        .collect();
    if required.len() != frozen.context_lock_requirement().required_source_ids.len() {
        return Err(error(Category::SourceProvenance, Code::DuplicateId));
    }
    let sources: Vec<_> = frozen
        .source_manifest()
        .iter()
        .filter(|s| required.contains(&s.source_id))
        .collect();
    if sources.len() != required.len() {
        return Err(error(Category::SourceProvenance, Code::SourceMissing));
    }
    digest(sources)
}

fn validate_context_lock(
    frozen: &FrozenWorkOrderV1,
    lock: &ContextLockEvidenceV1,
    expected_source_set: &crate::identity::EvidenceFingerprintV1,
    policy_generation: u64,
) -> Result<(), WorkOrderErrorV1> {
    let requirement = frozen.context_lock_requirement();
    if lock.schema != requirement.required_schema || lock.version != requirement.required_version {
        return Err(error(Category::AdmissionStaleness, Code::ContextLockStale));
    }
    if requirement.require_active && lock.status != ContextLockStateV1::Active {
        return Err(error(Category::AdmissionStaleness, Code::ContextLockStale));
    }
    if lock.work_order_id != *frozen.work_order_id()
        || lock.revision != frozen.revision()
        || lock.work_order_fingerprint != *frozen.fingerprint()
        || crate::scope::authorized_base_fingerprint(&requirement.authorized_base_constraint)?
            != lock.authorized_base_fingerprint
        || lock.source_set_fingerprint != *expected_source_set
        || (requirement.require_implementation_authorized && !lock.implementation_authorized)
        || lock.freshness != EvidenceFreshnessV1::Current
        || lock.policy_generation < policy_generation
    {
        return Err(error(Category::AdmissionStaleness, Code::ContextLockStale));
    }
    let proof = digest((
        &lock.schema,
        lock.version,
        lock.status,
        &lock.lock_fingerprint,
        &lock.work_order_id,
        lock.revision,
        &lock.work_order_fingerprint,
        &lock.authorized_base_fingerprint,
        &lock.source_set_fingerprint,
        lock.implementation_authorized,
        lock.policy_generation,
        lock.freshness,
        &lock.verifier_provenance,
    ))?;
    if proof != lock.proof_fingerprint {
        return Err(error(
            Category::AdmissionStaleness,
            Code::GovernanceProofMismatch,
        ));
    }
    Ok(())
}

fn validate_governance(
    frozen: &FrozenWorkOrderV1,
    proof: &VerifiedGovernanceProofV1,
    request: &WorkOrderAdmissionRequestV1,
) -> Result<(), WorkOrderErrorV1> {
    let requirement = frozen.governance_requirement();
    if proof.schema.is_empty()
        || proof.version == 0
        || proof.work_order_id != *frozen.work_order_id()
        || proof.revision != frozen.revision()
        || proof.work_order_fingerprint != *frozen.fingerprint()
        || proof.verdict != requirement.required_verdict
        || proof.authorized_scope_fingerprint != requirement.authorized_scope_fingerprint
        || proof.policy_generation < requirement.minimum_policy_generation
        || proof.policy_generation != request.policy_generation
        || (requirement.require_current_freshness
            && proof.freshness != EvidenceFreshnessV1::Current)
        || (requirement.require_exact_base
            && proof.exact_base != frozen.context_lock_requirement().authorized_base_constraint)
        || (requirement.require_exact_head
            && proof.exact_head.as_ref().is_none_or(String::is_empty))
    {
        return Err(error(
            Category::AdmissionStaleness,
            Code::GovernanceProofMismatch,
        ));
    }
    let proof_fingerprint = digest((
        &proof.schema,
        proof.version,
        &proof.proof_id,
        &proof.project_repository_fingerprint,
        &proof.work_order_id,
        proof.revision,
        &proof.work_order_fingerprint,
        &proof.exact_base,
        &proof.exact_head,
        proof.verdict,
        &proof.authorized_scope_fingerprint,
        proof.policy_generation,
        proof.freshness,
        &proof.verifier_provenance,
    ))?;
    if proof_fingerprint != proof.proof_fingerprint {
        return Err(error(
            Category::AdmissionStaleness,
            Code::GovernanceProofMismatch,
        ));
    }
    Ok(())
}

fn freshness_code(freshness: EvidenceFreshnessV1) -> Code {
    match freshness {
        EvidenceFreshnessV1::Current => Code::UnknownNotAdmissible,
        EvidenceFreshnessV1::Stale => Code::ContextLockStale,
        EvidenceFreshnessV1::Unknown => Code::UnknownNotAdmissible,
        EvidenceFreshnessV1::Substituted => Code::GovernanceProofMismatch,
    }
}

fn status_for(code: Code) -> WorkOrderAdmissionStatusV1 {
    match code {
        Code::SourceStale | Code::ContextLockStale | Code::PolicyStale | Code::SnapshotStale => {
            WorkOrderAdmissionStatusV1::Stale
        }
        Code::GovernanceProofMismatch | Code::WorkspaceMismatch | Code::SourceSubstituted => {
            WorkOrderAdmissionStatusV1::Rejected
        }
        _ => WorkOrderAdmissionStatusV1::Blocked,
    }
}

fn more_restrictive(
    left: WorkOrderAdmissionStatusV1,
    right: WorkOrderAdmissionStatusV1,
) -> WorkOrderAdmissionStatusV1 {
    fn rank(status: WorkOrderAdmissionStatusV1) -> u8 {
        match status {
            WorkOrderAdmissionStatusV1::Ready => 0,
            WorkOrderAdmissionStatusV1::Superseded => 1,
            WorkOrderAdmissionStatusV1::Blocked => 2,
            WorkOrderAdmissionStatusV1::Stale => 3,
            WorkOrderAdmissionStatusV1::Rejected => 4,
        }
    }
    if rank(left) >= rank(right) {
        left
    } else {
        right
    }
}

pub(crate) fn materialize(
    frozen: &FrozenWorkOrderV1,
    receipt: &WorkOrderAdmissionReceiptV1,
    budget: &M03ResourceBudgetV1,
) -> Result<AdmittedWorkOrderV1, WorkOrderErrorV1> {
    if receipt.status() != WorkOrderAdmissionStatusV1::Ready
        || receipt.work_order_id() != frozen.work_order_id()
        || receipt.revision() != frozen.revision()
        || receipt.work_order_fingerprint() != frozen.fingerprint()
        || receipt.compilation_id() != frozen.compilation_id()
    {
        return Err(error(
            Category::AdmissionStaleness,
            Code::PartialOutputForbidden,
        ));
    }
    let expected_receipt = digest((
        frozen.work_order_id(),
        frozen.revision(),
        frozen.fingerprint(),
        frozen.compilation_id(),
        receipt.status(),
        receipt.source_batch_fingerprint(),
        receipt.workspace_id(),
        receipt.workspace_runtime_epoch(),
        receipt.workspace_generation(),
        receipt.workspace_basis_fingerprint(),
        receipt.context_lock_fingerprint(),
        receipt.governance_proof_fingerprint(),
        receipt.policy_generation(),
        receipt.security_generation(),
        receipt.config_generation(),
        receipt.reason_codes(),
    ))?;
    if expected_receipt != *receipt.receipt_fingerprint() {
        return Err(error(Category::AdmissionStaleness, Code::ReceiptReplay));
    }
    ensure_count(
        frozen.semantic().packet_dag.packets.len(),
        budget.max_packets,
    )?;
    let plans = context::build_packet_context_plans(
        &frozen.semantic().packet_dag,
        frozen.source_manifest(),
        &frozen.semantic().context_budget,
        budget,
    )?;
    let context_plan = plans
        .into_iter()
        .next()
        .ok_or_else(|| error(Category::PacketGraph, Code::DanglingReference))?;
    let revalidation = RunStartRevalidationV1 {
        source_batch_fingerprint: receipt.source_batch_fingerprint().clone(),
        workspace_id: receipt.workspace_id().to_owned(),
        workspace_runtime_epoch: receipt.workspace_runtime_epoch(),
        workspace_generation: receipt.workspace_generation(),
        workspace_basis_fingerprint: receipt.workspace_basis_fingerprint().clone(),
        context_lock_fingerprint: receipt
            .context_lock_fingerprint()
            .cloned()
            .ok_or_else(|| error(Category::AdmissionStaleness, Code::ContextLockStale))?,
        governance_proof_fingerprint: receipt
            .governance_proof_fingerprint()
            .cloned()
            .ok_or_else(|| error(Category::AdmissionStaleness, Code::GovernanceProofMissing))?,
        policy_generation: receipt.policy_generation(),
        security_generation: receipt.security_generation(),
        config_generation: receipt.config_generation(),
    };
    let mut acceptance_ids: Vec<_> = frozen
        .semantic()
        .acceptance
        .criteria
        .iter()
        .map(|c| c.criterion_id.clone())
        .collect();
    let mut evidence_ids: Vec<_> = frozen
        .semantic()
        .acceptance
        .evidence_requirements
        .iter()
        .map(|e| e.evidence_id.clone())
        .collect();
    acceptance_ids.sort();
    evidence_ids.sort();
    Ok(AdmittedWorkOrderV1::new(
        frozen.work_order_id().clone(),
        frozen.revision(),
        frozen.fingerprint().clone(),
        frozen.compilation_id().clone(),
        receipt.receipt_fingerprint().clone(),
        frozen.semantic().packet_dag.clone(),
        frozen.semantic().scope.clone(),
        acceptance_ids,
        evidence_ids,
        context_plan,
        frozen.semantic().stop_condition.clone(),
        revalidation,
    ))
}
