use crate::contracts::*;
use crate::errors::WorkOrderErrorV1;
use crate::M03ResourceBudgetV1;

pub fn parse_request(
    input: &[u8],
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderEnvelope<WorkOrderRequestV1>, WorkOrderErrorV1> {
    crate::compiler::parse_request(input, budget)
}

pub fn compile(
    request: &WorkOrderRequestV1,
    context: &CompilationContextV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderCompilationV1, WorkOrderErrorV1> {
    crate::compiler::compile(request, context, budget)
}

pub fn validate_frozen(
    frozen: &FrozenWorkOrderV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderValidationReceiptV1, WorkOrderErrorV1> {
    crate::compiler::validate_frozen(frozen, budget)
}

pub fn diff_revision(
    before: &FrozenWorkOrderV1,
    after: &FrozenWorkOrderV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderRevisionDiffV1, WorkOrderErrorV1> {
    crate::compiler::validate_frozen(before, budget)?;
    crate::compiler::validate_frozen(after, budget)?;
    crate::delta::diff(before, after, budget)
}

pub fn classify_correction(
    frozen: &FrozenWorkOrderV1,
    proposal: &ExecutionCorrectionProposalV1,
    budget: &M03ResourceBudgetV1,
) -> Result<CorrectionClassificationReceiptV1, WorkOrderErrorV1> {
    crate::compiler::validate_frozen(frozen, budget)?;
    crate::delta::classify(frozen, proposal, budget)
}

pub fn evaluate_admission(
    frozen: &FrozenWorkOrderV1,
    request: &WorkOrderAdmissionRequestV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkOrderAdmissionReceiptV1, WorkOrderErrorV1> {
    crate::compiler::validate_frozen(frozen, budget)?;
    crate::admission::evaluate(frozen, request, budget)
}

pub fn materialize_handoff(
    frozen: &FrozenWorkOrderV1,
    receipt: &WorkOrderAdmissionReceiptV1,
    budget: &M03ResourceBudgetV1,
) -> Result<AdmittedWorkOrderV1, WorkOrderErrorV1> {
    crate::compiler::validate_frozen(frozen, budget)?;
    crate::admission::materialize(frozen, receipt, budget)
}

pub fn canonical_semantic_bytes(frozen: &FrozenWorkOrderV1) -> Result<Vec<u8>, WorkOrderErrorV1> {
    crate::canonical::semantic_projection_bytes(frozen)
}
