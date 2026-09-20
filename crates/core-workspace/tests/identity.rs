use core_workspace::{
    BvmProfile, ComponentMask, FilesystemSemanticsState, WorkspaceResourceBudget,
};

#[test]
fn bvm_profiles_are_deterministic_and_non_empty() {
    assert_ne!(
        BvmProfile::ReadMetadata.required_mask(),
        ComponentMask::NONE
    );
    assert!(BvmProfile::MutateSource
        .required_mask()
        .contains(core_workspace::BasisComponent::Authority));
}

#[test]
fn conservative_fsc_does_not_guess_windows_case_semantics() {
    assert_ne!(
        FilesystemSemanticsState::CasePreservingUnknown,
        FilesystemSemanticsState::CaseSensitiveVerified
    );
    assert!(WorkspaceResourceBudget::default().validate().is_ok());
}
