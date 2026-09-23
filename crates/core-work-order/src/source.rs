use crate::budget::{ensure_count, ensure_string_size, M03ResourceBudgetV1};
use crate::canonical::digest;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::EvidenceFingerprintV1;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn validate_manifest(
    refs: &[CanonicalSourceRefV1],
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    ensure_count(refs.len(), budget.max_source_refs)?;
    let mut seen = BTreeSet::new();
    for source in refs {
        if !seen.insert(&source.source_id) {
            return Err(error(Category::PacketGraph, Code::DuplicateId));
        }
        ensure_string_size(source.locator.as_str(), budget.max_string_bytes)?;
        if source.locator.is_empty() || source.locator.chars().any(char::is_control) {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceEvidenceUnknown,
            ));
        }
        if source.locator_kind == LocatorKindV1::RepositoryPath
            && (source.locator.starts_with('/')
                || source.locator.contains('\\')
                || source
                    .locator
                    .split('/')
                    .any(|segment| segment.is_empty() || segment == "." || segment == ".."))
        {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceAuthorityMismatch,
            ));
        }
        if has_uri_user_info(&source.locator) {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceAuthorityMismatch,
            ));
        }
        let mut packet_ids = BTreeSet::new();
        for packet_id in &source.required_packet_ids {
            if !packet_ids.insert(packet_id) {
                return Err(error(Category::PacketGraph, Code::DuplicateId));
            }
        }
        if source.secret_classification == SecretClassificationV1::SecretForbidden
            && source.expansion_policy == SourceExpansionPolicyV1::AlwaysLoad
        {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceEvidenceUnknown,
            ));
        }
        if source.authority_domain == AuthorityDomainV1::HiveAdvisory
            && source.freshness_policy != SourceFreshnessPolicyV1::AdvisoryOnly
        {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceAuthorityMismatch,
            ));
        }
    }
    Ok(())
}

fn has_uri_user_info(locator: &str) -> bool {
    let Some((_, remainder)) = locator.split_once("://") else {
        return false;
    };
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    remainder[..authority_end].contains('@')
}

pub(crate) fn evidence_fingerprint(
    evidence: &SourceResolutionEvidenceV1,
) -> Result<EvidenceFingerprintV1, WorkOrderErrorV1> {
    digest((
        evidence.source_id.as_str(),
        &evidence.requested_fingerprint,
        &evidence.observed_fingerprint,
        evidence.authority_domain,
        &evidence.source_revision,
        &evidence.resolver_schema,
        evidence.resolver_version,
        evidence.freshness_state,
        &evidence.provenance_fingerprint,
    ))
}

pub(crate) fn batch_fingerprint(
    batch: &SourceResolutionBatchV1,
) -> Result<EvidenceFingerprintV1, WorkOrderErrorV1> {
    let mut entries = batch.entries.clone();
    entries.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    for entry in &entries {
        if evidence_fingerprint(entry)? != entry.evidence_fingerprint {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceEvidenceUnknown,
            ));
        }
    }
    digest((&batch.resolver_schema, batch.resolver_version, entries))
}

pub(crate) fn validate_source_batch(
    refs: &[CanonicalSourceRefV1],
    batch: &SourceResolutionBatchV1,
    required_for_admission: bool,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    ensure_count(batch.entries.len(), budget.max_source_refs)?;
    ensure_string_size(&batch.resolver_schema, budget.max_string_bytes)?;
    if batch.resolver_schema.is_empty() || batch.resolver_version == 0 {
        return Err(error(
            Category::SourceProvenance,
            Code::SourceEvidenceUnknown,
        ));
    }
    let expected: BTreeMap<_, _> = refs.iter().map(|r| (&r.source_id, r)).collect();
    if expected.len() != refs.len() {
        return Err(error(Category::PacketGraph, Code::DuplicateId));
    }
    let mut observed = BTreeMap::new();
    for entry in &batch.entries {
        if observed.insert(&entry.source_id, entry).is_some() {
            return Err(error(Category::PacketGraph, Code::DuplicateId));
        }
        let Some(reference) = expected.get(&entry.source_id) else {
            return Err(error(Category::SourceProvenance, Code::SourceSubstituted));
        };
        ensure_string_size(&entry.source_revision, budget.max_string_bytes)?;
        if entry.source_revision.is_empty() || entry.source_revision.chars().any(char::is_control) {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceEvidenceUnknown,
            ));
        }
        if entry.resolver_schema != batch.resolver_schema
            || entry.resolver_version != batch.resolver_version
            || entry.authority_domain != reference.authority_domain
        {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceAuthorityMismatch,
            ));
        }
        if entry.requested_fingerprint != reference.expected_semantic_fingerprint {
            return Err(error(Category::SourceProvenance, Code::SourceSubstituted));
        }
        if entry.observed_fingerprint != reference.expected_semantic_fingerprint {
            return Err(error(Category::SourceProvenance, Code::SourceSubstituted));
        }
        if evidence_fingerprint(entry)? != entry.evidence_fingerprint {
            return Err(error(
                Category::SourceProvenance,
                Code::SourceEvidenceUnknown,
            ));
        }
        if reference.freshness_policy != SourceFreshnessPolicyV1::AdvisoryOnly
            && entry.freshness_state != EvidenceFreshnessV1::Current
        {
            return Err(error(
                Category::SourceProvenance,
                freshness_code(entry.freshness_state),
            ));
        }
        if reference.authority_domain == AuthorityDomainV1::HiveAdvisory
            && entry.freshness_state == EvidenceFreshnessV1::Substituted
        {
            return Err(error(Category::SourceProvenance, Code::SourceSubstituted));
        }
    }
    for reference in refs {
        let required = if required_for_admission {
            reference.required_for_admission
        } else {
            reference.required_for_compile
        };
        if required && !observed.contains_key(&reference.source_id) {
            return Err(error(Category::SourceProvenance, Code::SourceMissing));
        }
    }
    if batch_fingerprint(batch)? != batch.batch_fingerprint {
        return Err(error(
            Category::SourceProvenance,
            Code::SourceEvidenceUnknown,
        ));
    }
    Ok(())
}

fn freshness_code(freshness: EvidenceFreshnessV1) -> Code {
    match freshness {
        EvidenceFreshnessV1::Current => Code::SourceEvidenceUnknown,
        EvidenceFreshnessV1::Stale => Code::SourceStale,
        EvidenceFreshnessV1::Unknown => Code::SourceEvidenceUnknown,
        EvidenceFreshnessV1::Substituted => Code::SourceSubstituted,
    }
}
