#![no_main]

use core_config::{ConfigOverrides, CoreConfig};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    let _ = CoreConfig::from_sources(
        Some(&text),
        None,
        std::iter::empty(),
        &ConfigOverrides::default(),
        1,
    );
});
