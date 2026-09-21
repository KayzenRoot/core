#![no_main]

use core_workspace::git::redacted_remote_for_test;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let candidate = String::from_utf8_lossy(data);
    let redacted = redacted_remote_for_test(candidate.as_ref());
    assert!(!redacted.contains("token=") || redacted == "<redacted-remote>");
    assert!(!redacted.contains("password=") || redacted == "<redacted-remote>");
});
