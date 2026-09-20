#![no_main]

use core_contracts::RuntimeJournalRecord;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = serde_json::from_slice::<RuntimeJournalRecord>(data);
});
