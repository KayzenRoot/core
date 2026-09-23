#![no_main]

use core_work_order::{parse_request, M03ResourceBudgetV1};
use libfuzzer_sys::fuzz_target;

fn budget() -> M03ResourceBudgetV1 {
    M03ResourceBudgetV1::CALIBRATED_V1
}

fuzz_target!(|data: &[u8]| {
    if data.len() > 1_000_000 {
        return;
    }
    let budget = budget();
    if let Ok(envelope) = parse_request(data, &budget) {
        if let Ok(encoded) = serde_json::to_vec(&envelope) {
            let replay = parse_request(&encoded, &budget);
            assert_eq!(replay.unwrap(), envelope);
        }
    }
});
