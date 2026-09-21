#![no_main]

use core_workspace::lexical_normalize;
use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;

fuzz_target!(|data: &[u8]| {
    let candidate = String::from_utf8_lossy(data);
    let _ = lexical_normalize(&PathBuf::from(candidate.as_ref()));
});
