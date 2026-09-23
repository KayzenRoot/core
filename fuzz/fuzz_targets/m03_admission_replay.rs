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
    let Some(frozen_value) = value.get("frozen").cloned() else {
        return;
    };
    let Some(request_value) = value.get("request").cloned() else {
        return;
    };
    let Ok(frozen) = serde_json::from_value::<FrozenWorkOrderV1>(frozen_value) else {
        return;
    };
    let Ok(request) = serde_json::from_value::<WorkOrderAdmissionRequestV1>(request_value) else {
        return;
    };
    let budget = budget();
    if let Ok(receipt) = evaluate_admission(&frozen, &request, &budget) {
        if receipt.status() == WorkOrderAdmissionStatusV1::Ready {
            let _ = materialize_handoff(&frozen, &receipt, &budget);
        }
    }

    let mut changed = request;
    changed.workspace.generation = changed.workspace.generation.saturating_add(1);
    changed.workspace.freshness = match data.first().copied().unwrap_or_default() % 4 {
        0 => EvidenceFreshnessV1::Current,
        1 => EvidenceFreshnessV1::Stale,
        2 => EvidenceFreshnessV1::Unknown,
        _ => EvidenceFreshnessV1::Substituted,
    };
    let _ = evaluate_admission(&frozen, &changed, &budget);
});
