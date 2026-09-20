#![no_main]

use core_ipc::decode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = decode(data, 1024 * 1024, 1);
});
