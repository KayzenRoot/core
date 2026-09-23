use crate::budget::{ensure_count, M03ResourceBudgetV1};
use crate::canonical::digest;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::SemanticFieldIdV1;
use crate::scope::validate_correction_paths;
use core_identity::canonical_bytes;
use serde::Serialize;

fn field_bytes<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, WorkOrderErrorV1> {
    canonical_bytes(&value).map_err(|_| {
        error(
            Category::InternalInvariant,
            Code::InternalInvariantViolation,
        )
    })
}

pub(crate) fn changed_fields(
    before: &FrozenWorkOrderV1,
    after: &FrozenWorkOrderV1,
) -> Result<Vec<SemanticFieldIdV1>, WorkOrderErrorV1> {
    let b = before.semantic();
    let a = after.semantic();
    let fields = [
        (
            "objective",
            field_bytes(&b.objective)?,
            field_bytes(&a.objective)?,
        ),
        ("scope", field_bytes(&b.scope)?, field_bytes(&a.scope)?),
        (
            "packets",
            field_bytes(&b.packet_dag)?,
            field_bytes(&a.packet_dag)?,
        ),
        (
            "acceptance",
            field_bytes(&b.acceptance)?,
            field_bytes(&a.acceptance)?,
        ),
        (
            "context_budget",
            field_bytes(&b.context_budget)?,
            field_bytes(&a.context_budget)?,
        ),
        (
            "correction_policy",
            field_bytes(&b.correction_policy)?,
            field_bytes(&a.correction_policy)?,
        ),
        (
            "stop_condition",
            field_bytes(&b.stop_condition)?,
            field_bytes(&a.stop_condition)?,
        ),
        (
            "risk_assurance",
            field_bytes(&b.risk_assurance)?,
            field_bytes(&a.risk_assurance)?,
        ),
        (
            "source_manifest",
            field_bytes(before.source_manifest())?,
            field_bytes(after.source_manifest())?,
        ),
        (
            "workspace_requirement",
            field_bytes(before.workspace_requirement())?,
            field_bytes(after.workspace_requirement())?,
        ),
        (
            "context_lock_requirement",
            field_bytes(before.context_lock_requirement())?,
            field_bytes(after.context_lock_requirement())?,
        ),
        (
            "governance_requirement",
            field_bytes(before.governance_requirement())?,
            field_bytes(after.governance_requirement())?,
        ),
        (
            "lineage",
            field_bytes(&before.lineage().parent)?,
            field_bytes(&after.lineage().parent)?,
        ),
    ];
    Ok(fields
        .into_iter()
        .filter_map(|(name, left, right)| {
            if left != right {
                Some(SemanticFieldIdV1::new(name).ok()?)
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}

pub(crate) fn diff(
    before: &FrozenWorkOrderV1,
    after: &FrozenWorkOrderV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderRevisionDiffV1, WorkOrderErrorV1> {
    if before.work_order_id() != after.work_order_id() {
        return Err(error(Category::Lineage, Code::LineageConflict));
    }
    let fields = changed_fields(before, after)?;
    ensure_count(fields.len(), budget.max_diff_entries)?;
    let changed = !fields.is_empty();
    let mut forbidden = Vec::new();
    if (changed && after.revision() != before.revision().next()?)
        || (!changed && after.revision() != before.revision())
    {
        forbidden.push(Code::RevisionNotNext);
    }
    if before.revision() == after.revision() && changed {
        forbidden.push(Code::RevisionNotNext);
    }
    let diff_fingerprint = digest((
        before.work_order_id(),
        before.revision(),
        before.fingerprint(),
        after.revision(),
        after.fingerprint(),
        &fields,
        changed,
        &forbidden,
    ))?;
    Ok(WorkOrderRevisionDiffV1 {
        work_order_id: before.work_order_id().clone(),
        before_revision: before.revision(),
        before_fingerprint: before.fingerprint().clone(),
        after_revision: after.revision(),
        after_fingerprint: after.fingerprint().clone(),
        changed_semantic_fields: fields,
        requires_new_revision: changed,
        forbidden_reason_codes: forbidden,
        diff_fingerprint,
    })
}

pub(crate) fn classify(
    frozen: &FrozenWorkOrderV1,
    proposal: &ExecutionCorrectionProposalV1,
    budget: &M03ResourceBudgetV1,
) -> Result<CorrectionClassificationReceiptV1, WorkOrderErrorV1> {
    ensure_count(
        proposal.changed_paths.len()
            + proposal.artifact_classes.len()
            + proposal.added_dependencies.len(),
        budget.max_correction_rules,
    )?;
    ensure_count(
        proposal.changed_semantic_fields.len(),
        budget.max_diff_entries,
    )?;
    let mut codes = Vec::new();
    let policy = &frozen.semantic().correction_policy;
    let scope = &frozen.semantic().scope;
    let forbidden_requested = proposal
        .requested_classes
        .iter()
        .any(|class| policy.forbidden_classes.contains(class));
    if forbidden_requested
        || proposal.requested_classes.iter().any(|class| {
            matches!(
                class,
                CorrectionClassV1::AcceptanceWeakening
                    | CorrectionClassV1::StopConditionWeakening
                    | CorrectionClassV1::ScopeExpansion
                    | CorrectionClassV1::ArchitectureChange
                    | CorrectionClassV1::SecurityPolicyChange
            )
        })
    {
        codes.push(Code::ForbiddenDelta);
    }
    if !proposal.changed_paths.is_empty() || !proposal.artifact_classes.is_empty() {
        if let Err(reason) = validate_correction_paths(
            scope,
            &proposal.changed_paths,
            &proposal.artifact_classes,
            budget,
        ) {
            codes.push(reason.code);
        }
    }
    if !proposal.added_dependencies.is_empty() {
        let dependency_policy = &scope.dependency_policy;
        let allowed = match dependency_policy.change_policy {
            DependencyChangePolicyV1::NoChanges => false,
            DependencyChangePolicyV1::NamedDependenciesOnly => proposal
                .added_dependencies
                .iter()
                .all(|dep| dependency_policy.allowed_direct_dependencies.contains(dep)),
            DependencyChangePolicyV1::GovernedAdmissionRequired => false,
        };
        if !allowed
            || !proposal
                .requested_classes
                .contains(&CorrectionClassV1::DependencyAdmission)
        {
            codes.push(Code::DependencyAdmissionRequired);
        }
    }
    for field in &proposal.changed_semantic_fields {
        if !SemanticFieldIdV1::FIELDS.contains(&field.as_str()) {
            codes.push(Code::ForbiddenDelta);
            break;
        }
    }
    if !proposal.changed_semantic_fields.is_empty() {
        codes.push(Code::RevisionNotNext);
    }
    let forbidden = codes.contains(&Code::ForbiddenDelta)
        || codes.contains(&Code::DenyOverridesAllow)
        || codes.contains(&Code::AmbiguousScope);
    let requires_revision = !proposal.changed_semantic_fields.is_empty()
        || codes.contains(&Code::DependencyAdmissionRequired)
        || proposal
            .requested_classes
            .iter()
            .any(|c| !policy.same_revision_classes.contains(c));
    let disposition = if forbidden {
        CorrectionDispositionV1::Forbidden
    } else if requires_revision {
        CorrectionDispositionV1::RequiresNewRevision
    } else if proposal.requested_classes.is_empty() {
        codes.push(Code::ForbiddenDelta);
        CorrectionDispositionV1::Forbidden
    } else {
        CorrectionDispositionV1::AllowedSameRevision
    };
    codes.sort();
    codes.dedup();
    let receipt_fingerprint = digest((
        frozen.work_order_id(),
        frozen.revision(),
        frozen.fingerprint(),
        &proposal.proposal_fingerprint,
        disposition,
        &codes,
    ))?;
    Ok(CorrectionClassificationReceiptV1 {
        work_order_id: frozen.work_order_id().clone(),
        revision: frozen.revision(),
        work_order_fingerprint: frozen.fingerprint().clone(),
        proposal_fingerprint: proposal.proposal_fingerprint.clone(),
        disposition,
        reason_codes: codes,
        receipt_fingerprint,
    })
}
