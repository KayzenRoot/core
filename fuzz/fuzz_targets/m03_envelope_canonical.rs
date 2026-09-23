#![no_main]

use core_work_order::{parse_request, M03ResourceBudgetV1, ResourceCalibrationStateV1};
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
    let budget = budget();
    if let Ok(envelope) = parse_request(data, &budget) {
        if let Ok(encoded) = serde_json::to_vec(&envelope) {
            let replay = parse_request(&encoded, &budget);
            assert_eq!(replay.unwrap(), envelope);
        }
    }
});
