#![no_main]

use core_work_order::*;
use libfuzzer_sys::fuzz_target;

fn budget() -> M03ResourceBudgetV1 {
    M03ResourceBudgetV1::new(
        78_333,
        78_898,
        4_096,
        32,
        32,
        31,
        64,
        32,
        32,
        32,
        32,
        32,
        64,
        3,
        4,
        7,
        ResourceCalibrationStateV1::Calibrated,
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
    if let Ok(compilation) = compile(&envelope.payload, &context, &budget) {
        if let Some(capsule) = compilation.lineage_precondition {
            assert_eq!(capsule.proposed_revision, compilation.frozen.revision());
            let _ = capsule.precondition_fingerprint.as_str();
        }
        assert!(classify_lineage_cas(LineageCasResultV1::Applied).is_ok());
        let _ = classify_lineage_cas(LineageCasResultV1::Conflict);
    }

    let mut stale_context = context;
    stale_context.lineage.store_generation =
        stale_context.lineage.store_generation.saturating_add(1);
    let _ = compile(&envelope.payload, &stale_context, &budget);
    let revision = WorkOrderRevision::new(u32::from(data.first().copied().unwrap_or_default()))
        .unwrap_or_else(|_| WorkOrderRevision::new(1).unwrap());
    let _ = revision.next();
});
