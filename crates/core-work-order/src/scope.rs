use crate::budget::{ensure_count, ensure_string_size, M03ResourceBudgetV1};
use crate::canonical::digest;
use crate::canonical::normalize_scope;
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::EvidenceFingerprintV1;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeDecisionV1 {
    Allowed,
    Denied,
    Ambiguous,
}

pub fn scope_fingerprint(
    scope: &ScopeEnvelopeV1,
) -> Result<EvidenceFingerprintV1, WorkOrderErrorV1> {
    let mut normalized = scope.clone();
    normalize_scope(&mut normalized);
    digest(("nexlabs.core.work-order.scope.v1", normalized))
}

pub fn authorized_base_fingerprint(
    authorized_base: &str,
) -> Result<EvidenceFingerprintV1, WorkOrderErrorV1> {
    if authorized_base.is_empty() || authorized_base.chars().any(char::is_control) {
        return Err(error(Category::SchemaVersion, Code::InvalidEnvelope));
    }
    digest((
        "nexlabs.core.work-order.authorized-base.v1",
        authorized_base,
    ))
}

pub(crate) fn validate_scope(
    scope: &ScopeEnvelopeV1,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    if scope.allowed_modules.is_empty()
        || scope.allowed_crates_or_packages.is_empty()
        || scope.allowed_path_prefixes.is_empty()
        || scope.allowed_artifact_classes.is_empty()
    {
        return Err(error(Category::ScopeDelta, Code::AmbiguousScope));
    }
    let count = scope.allowed_modules.len()
        + scope.denied_modules.len()
        + scope.allowed_crates_or_packages.len()
        + scope.denied_crates_or_packages.len()
        + scope.allowed_path_prefixes.len()
        + scope.denied_path_prefixes.len()
        + scope.allowed_artifact_classes.len()
        + scope.denied_artifact_classes.len()
        + scope.dependency_policy.allowed_direct_dependencies.len()
        + scope.allowed_correction_classes.len();
    ensure_count(count, budget.max_scope_rules)?;
    validate_unique_strings(&scope.allowed_modules, budget)?;
    validate_unique_strings(&scope.denied_modules, budget)?;
    validate_unique_strings(&scope.allowed_crates_or_packages, budget)?;
    validate_unique_strings(&scope.denied_crates_or_packages, budget)?;
    validate_unique_strings(&scope.allowed_artifact_classes, budget)?;
    validate_unique_strings(&scope.denied_artifact_classes, budget)?;
    validate_unique_strings(&scope.dependency_policy.allowed_direct_dependencies, budget)?;
    validate_path_rules(&scope.allowed_path_prefixes, budget)?;
    validate_path_rules(&scope.denied_path_prefixes, budget)?;
    if scope
        .allowed_correction_classes
        .iter()
        .collect::<BTreeSet<_>>()
        .len()
        != scope.allowed_correction_classes.len()
    {
        return Err(error(Category::ScopeDelta, Code::DuplicateId));
    }
    Ok(())
}

fn validate_unique_strings(
    values: &[String],
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    let mut seen = BTreeSet::new();
    for value in values {
        ensure_string_size(value, budget.max_string_bytes)?;
        if value.is_empty() || value.chars().any(char::is_control) || !seen.insert(value) {
            return Err(error(Category::ScopeDelta, Code::AmbiguousScope));
        }
    }
    Ok(())
}

fn validate_path_rules(
    values: &[String],
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    validate_unique_strings(values, budget)?;
    if values.iter().any(|path| {
        path.starts_with('/')
            || path.contains('\\')
            || path
                .split('/')
                .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    }) {
        return Err(error(Category::ScopeDelta, Code::AmbiguousScope));
    }
    Ok(())
}

fn path_matches(rule: &str, path: &str) -> bool {
    path == rule
        || path
            .strip_prefix(rule)
            .is_some_and(|rest| rest.starts_with('/'))
}

pub fn evaluate_scope(
    scope: &ScopeEnvelopeV1,
    module: &str,
    crate_or_package: &str,
    path: &str,
    artifact_class: &str,
) -> ScopeDecisionV1 {
    if scope.denied_modules.iter().any(|v| v == module)
        || scope
            .denied_crates_or_packages
            .iter()
            .any(|v| v == crate_or_package)
        || scope
            .denied_path_prefixes
            .iter()
            .any(|v| path_matches(v, path))
        || scope
            .denied_artifact_classes
            .iter()
            .any(|v| v == artifact_class)
    {
        return ScopeDecisionV1::Denied;
    }
    if scope.allowed_modules.is_empty()
        || scope.allowed_crates_or_packages.is_empty()
        || scope.allowed_path_prefixes.is_empty()
        || scope.allowed_artifact_classes.is_empty()
    {
        return ScopeDecisionV1::Ambiguous;
    }
    if scope.allowed_modules.iter().any(|v| v == module)
        && scope
            .allowed_crates_or_packages
            .iter()
            .any(|v| v == crate_or_package)
        && scope
            .allowed_path_prefixes
            .iter()
            .any(|v| path_matches(v, path))
        && scope
            .allowed_artifact_classes
            .iter()
            .any(|v| v == artifact_class)
    {
        ScopeDecisionV1::Allowed
    } else {
        ScopeDecisionV1::Denied
    }
}

pub(crate) fn is_scope_subset(child: &ScopeEnvelopeV1, parent: &ScopeEnvelopeV1) -> bool {
    child
        .allowed_modules
        .iter()
        .all(|v| parent.allowed_modules.contains(v))
        && child
            .allowed_crates_or_packages
            .iter()
            .all(|v| parent.allowed_crates_or_packages.contains(v))
        && child.allowed_path_prefixes.iter().all(|v| {
            parent
                .allowed_path_prefixes
                .iter()
                .any(|p| path_matches(p, v))
        })
        && child
            .allowed_artifact_classes
            .iter()
            .all(|v| parent.allowed_artifact_classes.contains(v))
        && child
            .dependency_policy
            .allowed_direct_dependencies
            .iter()
            .all(|v| {
                parent
                    .dependency_policy
                    .allowed_direct_dependencies
                    .contains(v)
            })
        && child
            .allowed_correction_classes
            .iter()
            .all(|v| parent.allowed_correction_classes.contains(v))
        && mutation_subset(child.source_mutation_policy, parent.source_mutation_policy)
        && mutation_subset(
            child.documentation_mutation_policy,
            parent.documentation_mutation_policy,
        )
        && mutation_subset(
            child.evidence_mutation_policy,
            parent.evidence_mutation_policy,
        )
        && mutation_subset(
            child.generated_artifact_policy,
            parent.generated_artifact_policy,
        )
        && child.maximum_scope_class <= parent.maximum_scope_class
        && child.dependency_policy.change_policy <= parent.dependency_policy.change_policy
}

fn mutation_subset(child: MutationPolicyV1, parent: MutationPolicyV1) -> bool {
    child == MutationPolicyV1::Deny
        || parent == MutationPolicyV1::WithinAllowedScope
        || child == parent
}

fn intersect_mutation(child: MutationPolicyV1, parent: MutationPolicyV1) -> MutationPolicyV1 {
    if child == MutationPolicyV1::Deny || parent == MutationPolicyV1::Deny {
        MutationPolicyV1::Deny
    } else if parent == MutationPolicyV1::WithinAllowedScope {
        child
    } else if child == MutationPolicyV1::WithinAllowedScope || child == parent {
        parent
    } else {
        MutationPolicyV1::Deny
    }
}

pub(crate) fn intersect_scope(
    parent: &ScopeEnvelopeV1,
    child: &ScopeEnvelopeV1,
) -> ScopeEnvelopeV1 {
    let mut result = child.clone();
    intersect_values(&mut result.allowed_modules, &parent.allowed_modules);
    intersect_values(
        &mut result.allowed_crates_or_packages,
        &parent.allowed_crates_or_packages,
    );
    let mut path_intersection = Vec::new();
    for child_path in &result.allowed_path_prefixes {
        for parent_path in &parent.allowed_path_prefixes {
            if path_matches(parent_path, child_path) {
                path_intersection.push(child_path.clone());
            } else if path_matches(child_path, parent_path) {
                path_intersection.push(parent_path.clone());
            }
        }
    }
    result.allowed_path_prefixes = path_intersection;
    intersect_values(
        &mut result.allowed_artifact_classes,
        &parent.allowed_artifact_classes,
    );
    result.denied_modules.extend(parent.denied_modules.clone());
    result
        .denied_crates_or_packages
        .extend(parent.denied_crates_or_packages.clone());
    result
        .denied_path_prefixes
        .extend(parent.denied_path_prefixes.clone());
    result
        .denied_artifact_classes
        .extend(parent.denied_artifact_classes.clone());
    result
        .dependency_policy
        .allowed_direct_dependencies
        .retain(|dep| {
            parent
                .dependency_policy
                .allowed_direct_dependencies
                .contains(dep)
        });
    result
        .allowed_correction_classes
        .retain(|class| parent.allowed_correction_classes.contains(class));
    result.source_mutation_policy =
        intersect_mutation(result.source_mutation_policy, parent.source_mutation_policy);
    result.documentation_mutation_policy = intersect_mutation(
        result.documentation_mutation_policy,
        parent.documentation_mutation_policy,
    );
    result.evidence_mutation_policy = intersect_mutation(
        result.evidence_mutation_policy,
        parent.evidence_mutation_policy,
    );
    result.generated_artifact_policy = intersect_mutation(
        result.generated_artifact_policy,
        parent.generated_artifact_policy,
    );
    result.maximum_scope_class = result.maximum_scope_class.min(parent.maximum_scope_class);
    result.dependency_policy.change_policy = result
        .dependency_policy
        .change_policy
        .min(parent.dependency_policy.change_policy);
    normalize_scope(&mut result);
    result
}

fn intersect_values(values: &mut Vec<String>, parent: &[String]) {
    values.retain(|value| parent.contains(value));
}

pub(crate) fn validate_correction_paths(
    scope: &ScopeEnvelopeV1,
    paths: &[String],
    artifact_classes: &[String],
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    ensure_count(
        paths.len() + artifact_classes.len(),
        budget.max_diff_entries,
    )?;
    for path in paths {
        ensure_string_size(path, budget.max_string_bytes)?;
        if scope
            .denied_path_prefixes
            .iter()
            .any(|rule| path_matches(rule, path))
        {
            return Err(error(Category::ScopeDelta, Code::DenyOverridesAllow));
        }
        if !scope
            .allowed_path_prefixes
            .iter()
            .any(|rule| path_matches(rule, path))
        {
            return Err(error(Category::ScopeDelta, Code::AmbiguousScope));
        }
    }
    for artifact in artifact_classes {
        ensure_string_size(artifact, budget.max_string_bytes)?;
        if scope.denied_artifact_classes.contains(artifact) {
            return Err(error(Category::ScopeDelta, Code::DenyOverridesAllow));
        }
        if !scope.allowed_artifact_classes.contains(artifact) {
            return Err(error(Category::ScopeDelta, Code::AmbiguousScope));
        }
    }
    Ok(())
}
