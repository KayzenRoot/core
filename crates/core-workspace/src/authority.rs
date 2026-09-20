//! Lexical + physical path authority proof and conservative FSC.

use crate::identity::{authority_root_id, normalized_path_string, workspace_id};
use crate::{
    AuthorityClass, AuthorityRootV1, FilesystemSemanticsCapsuleV1, FilesystemSemanticsState,
    M02Error, PathValidationRequestV1, ValidatedPathReceiptV1,
};
use core_identity::fingerprint;
use std::fs;
use std::path::{Component, Path, PathBuf, Prefix};

pub fn lexical_normalize(path: &Path) -> Result<PathBuf, M02Error> {
    let raw = path.to_string_lossy();
    if raw.contains('\0') {
        return Err(M02Error::InvalidInput("path contains NUL".into()));
    }
    #[cfg(windows)]
    {
        let bytes = raw.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' && !matches!(bytes.get(2), Some(b'\\' | b'/')) {
            return Err(M02Error::AuthorityViolation(
                "drive-relative path denied".into(),
            ));
        }
        if raw.starts_with(r"\\?\") || raw.starts_with(r"\\.\") {
            return Err(M02Error::AuthorityViolation(
                "device namespace denied".into(),
            ));
        }
    }
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => {
                if matches!(
                    prefix.kind(),
                    Prefix::Verbatim(_) | Prefix::VerbatimUNC(_, _)
                ) {
                    return Err(M02Error::AuthorityViolation(
                        "verbatim namespace denied".into(),
                    ));
                }
                output.push(prefix.as_os_str());
            }
            Component::RootDir => output.push(Path::new(std::path::MAIN_SEPARATOR_STR)),
            Component::CurDir => {}
            Component::ParentDir => {
                if !output.pop() {
                    return Err(M02Error::AuthorityViolation(
                        "lexical traversal escaped root".into(),
                    ));
                }
            }
            Component::Normal(value) => output.push(value),
        }
    }
    Ok(output)
}

pub fn probe_filesystem_semantics(root: &Path) -> Result<FilesystemSemanticsCapsuleV1, M02Error> {
    let canonical = root
        .canonicalize()
        .map_err(|error| M02Error::Io(format!("canonicalize {}: {error}", root.display())))?;
    let metadata = fs::metadata(&canonical)
        .map_err(|error| M02Error::Io(format!("metadata {}: {error}", canonical.display())))?;
    let physical_identity = fingerprint(&(
        normalized_path_string(&canonical),
        metadata.len(),
        metadata.permissions().readonly(),
    ))
    .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
    let case_state = if cfg!(unix) {
        FilesystemSemanticsState::CaseSensitiveVerified
    } else {
        FilesystemSemanticsState::CasePreservingUnknown
    };
    Ok(FilesystemSemanticsCapsuleV1 {
        root_physical_identity: physical_identity,
        namespace_class: if cfg!(windows) {
            "windows-drive-or-unc"
        } else {
            "posix"
        }
        .into(),
        canonical_path_evidence: normalized_path_string(&canonical),
        case_state,
        symlink_or_reparse_supported: true,
        normalization_policy_version: "m02-fsc-v1".into(),
        provenance: "std-fs-read-only".into(),
    })
}

pub fn source_authority_root(
    root: &Path,
    policy_generation: u64,
) -> Result<AuthorityRootV1, M02Error> {
    let canonical = root
        .canonicalize()
        .map_err(|error| M02Error::Io(format!("canonicalize {}: {error}", root.display())))?;
    let capsule = probe_filesystem_semantics(&canonical)?;
    let id = authority_root_id(
        AuthorityClass::Source,
        &canonical,
        &capsule.root_physical_identity,
    );
    Ok(AuthorityRootV1 {
        id,
        class: AuthorityClass::Source,
        logical_root: lexical_normalize(root)?,
        canonical_existing_root: canonical,
        physical_identity: capsule.root_physical_identity.clone(),
        filesystem_semantics_fingerprint: fingerprint(&capsule)
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?,
        provenance: capsule.provenance,
        policy_generation,
        externality: false,
    })
}

pub fn validate_path(
    authority: &AuthorityRootV1,
    request: &PathValidationRequestV1,
) -> Result<ValidatedPathReceiptV1, M02Error> {
    if request.root != authority.id {
        return Err(M02Error::AuthorityViolation(
            "authority root mismatch".into(),
        ));
    }
    let requested = &request.requested;
    let lexical = if requested.is_absolute() {
        lexical_normalize(requested)?
    } else {
        lexical_normalize(&authority.logical_root.join(requested))?
    };
    let logical_root = lexical_normalize(&authority.logical_root)?;
    if !lexical.starts_with(&logical_root) {
        return Err(M02Error::AuthorityViolation(format!(
            "{} is outside {}",
            lexical.display(),
            logical_root.display()
        )));
    }

    let mut nearest = lexical.clone();
    while !nearest.exists() {
        if !nearest.pop() {
            return Err(M02Error::AuthorityViolation("no existing ancestor".into()));
        }
    }
    let physical_root = authority
        .canonical_existing_root
        .canonicalize()
        .map_err(|error| M02Error::Io(error.to_string()))?;
    let physical_ancestor = nearest
        .canonicalize()
        .map_err(|error| M02Error::Io(error.to_string()))?;
    if !physical_ancestor.starts_with(&physical_root) {
        return Err(M02Error::AuthorityViolation(
            "physical path escaped source authority".into(),
        ));
    }
    let target = if lexical.exists() {
        let physical = lexical
            .canonicalize()
            .map_err(|error| M02Error::Io(error.to_string()))?;
        if !physical.starts_with(&physical_root) {
            return Err(M02Error::AuthorityViolation(
                "resolved target escaped source authority".into(),
            ));
        }
        Some(physical)
    } else {
        None
    };
    Ok(ValidatedPathReceiptV1 {
        requested: requested.clone(),
        lexical_normalized: lexical,
        authority_root: authority.id.clone(),
        nearest_existing_ancestor: physical_ancestor,
        resolved_target: target,
        filesystem_semantics_fingerprint: authority.filesystem_semantics_fingerprint.clone(),
        use_time_revalidation_required: !requested.exists(),
        allowed: true,
        reason: "lexical-and-physical-containment-proven".into(),
    })
}

pub fn workspace_identity(
    root: &Path,
) -> Result<(crate::WorkspaceId, FilesystemSemanticsCapsuleV1), M02Error> {
    let capsule = probe_filesystem_semantics(root)?;
    let canonical = root
        .canonicalize()
        .map_err(|error| M02Error::Io(error.to_string()))?;
    let id = workspace_id(
        &canonical,
        &capsule.root_physical_identity,
        &fingerprint(&capsule).unwrap(),
    );
    Ok((id, capsule))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn traversal_is_rejected_without_writing_source() {
        let root = std::env::temp_dir().join(format!("m02-authority-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let authority = source_authority_root(&root, 1).unwrap();
        let request = PathValidationRequestV1 {
            requested: PathBuf::from("..\\outside"),
            operation: crate::PathOperation::ReadSource,
            root: authority.id.clone(),
        };
        assert!(validate_path(&authority, &request).is_err());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
        let _ = fs::remove_dir(&root);
    }

    #[test]
    fn non_existing_path_requires_use_time_revalidation() {
        let root =
            std::env::temp_dir().join(format!("m02-authority-nonexistent-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let authority = source_authority_root(&root, 1).unwrap();
        let request = PathValidationRequestV1 {
            requested: PathBuf::from("new-file.txt"),
            operation: crate::PathOperation::MutateSource,
            root: authority.id.clone(),
        };
        let receipt = validate_path(&authority, &request).unwrap();
        assert!(receipt.use_time_revalidation_required);
        let _ = fs::remove_dir(&root);
    }
}
