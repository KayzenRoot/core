#![no_main]

use core_work_order::*;
use libfuzzer_sys::fuzz_target;

fn budget() -> M03ResourceBudgetV1 {
    M03ResourceBudgetV1::CALIBRATED_V1
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
    let _ = compile(&envelope.payload, &context, &budget);

    if !context.sources.entries.is_empty() {
        let mut unknown = context.clone();
        unknown.sources.entries[0].freshness_state = EvidenceFreshnessV1::Unknown;
        let _ = compile(&envelope.payload, &unknown, &budget);

        let mut substituted = context;
        let nibble = data.first().copied().unwrap_or_default() % 16;
        substituted.sources.entries[0].observed_fingerprint =
            EvidenceFingerprintV1::new(format!("{nibble:x}").repeat(64)).unwrap();
        let _ = compile(&envelope.payload, &substituted, &budget);
    }
});
