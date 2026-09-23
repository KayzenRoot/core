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
    let Ok(encoded) = serde_json::to_vec(&envelope) else {
        return;
    };
    let Ok(parsed) = parse_request(&encoded, &budget) else {
        return;
    };
    let mut request = parsed.payload;
    if let Some(first) = request.packets.first_mut() {
        match data.first().copied().unwrap_or_default() % 3 {
            0 => first.prerequisite_packet_ids.push(first.packet_id.clone()),
            1 => first
                .prerequisite_packet_ids
                .push(WorkPacketId::new("dangling-fuzz-edge").unwrap()),
            _ => request.packets.reverse(),
        }
    }
    let candidate = WorkOrderEnvelope::new(WorkOrderContractKindV1::Request, request);
    if let Ok(candidate_json) = serde_json::to_vec(&candidate) {
        if let Ok(candidate) = parse_request(&candidate_json, &budget) {
            let _ = compile(&candidate.payload, &context, &budget);
        }
    }
});
