#![no_main]

use core_work_order::*;
use libfuzzer_sys::fuzz_target;

fn budget() -> M03ResourceBudgetV1 {
    M03ResourceBudgetV1::new(
        1_000_000,
        1_000_000,
        16_384,
        256,
        128,
        1_024,
        512,
        256,
        256,
        512,
        512,
        512,
        256,
        256,
        256,
        64,
        ResourceCalibrationStateV1::Uncalibrated,
    )
    .unwrap()
}

fuzz_target!(|data: &[u8]| {
    if data.len() > 1_000_000 {
        return;
    }
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(data) else {
        return;
    };
    let Some(request_value) = value.get("request").cloned() else {
        return;
    };
    let Some(context_value) = value.get("context").cloned() else {
        return;
    };
    let Ok(envelope) =
        serde_json::from_value::<WorkOrderEnvelope<WorkOrderRequestV1>>(request_value)
    else {
        return;
    };
    let Ok(context) = serde_json::from_value::<CompilationContextV1>(context_value) else {
        return;
    };
    let budget = budget();
    let Ok(frozen) = compile(&envelope.payload, &context, &budget) else {
        return;
    };
    let scope = &frozen.frozen.semantic().scope;
    if let (Some(module), Some(package), Some(prefix), Some(artifact)) = (
        scope.allowed_modules.first(),
        scope.allowed_crates_or_packages.first(),
        scope.allowed_path_prefixes.first(),
        scope.allowed_artifact_classes.first(),
    ) {
        let path = format!("{prefix}/fuzz-target.rs");
        let decision = evaluate_scope(scope, module, package, &path, artifact);
        if scope.denied_path_prefixes.iter().any(|deny| path == *deny) {
            assert_eq!(decision, ScopeDecisionV1::Denied);
        }
    }
    let class = if data.first().copied().unwrap_or_default() & 1 == 0 {
        CorrectionClassV1::TestOnlyWithinScope
    } else {
        CorrectionClassV1::ScopeExpansion
    };
    let proposal = ExecutionCorrectionProposalV1 {
        proposal_fingerprint: EvidenceFingerprintV1::new("a".repeat(64)).unwrap(),
        changed_paths: vec!["crates/core-work-order/tests/fuzz-target.rs".into()],
        artifact_classes: vec!["test".into()],
        requested_classes: vec![class],
        added_dependencies: if data.first().copied().unwrap_or_default() & 2 == 0 {
            vec![]
        } else {
            vec!["fuzz-only-dependency".into()]
        },
        changed_semantic_fields: if data.first().copied().unwrap_or_default() & 4 == 0 {
            vec![]
        } else {
            vec![SemanticFieldIdV1::new("scope").unwrap()]
        },
    };
    let _ = classify_correction(&frozen.frozen, &proposal, &budget);
});
