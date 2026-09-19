#![no_main]

use core_contracts::{CapabilityProviderDescriptor, CapabilityRequirement, SemVer};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(requirement) = serde_json::from_slice::<CapabilityRequirement>(data) {
        let _ = requirement.contract.compatible_with(SemVer::new(1, 0, 0));
    }
    if let Ok(provider) = serde_json::from_slice::<CapabilityProviderDescriptor>(data) {
        let requirement = CapabilityRequirement::new(&provider.capability, provider.contract);
        let _ = provider.contract.compatible_with(requirement.contract);
    }
});
