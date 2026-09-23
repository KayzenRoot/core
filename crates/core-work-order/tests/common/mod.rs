#![allow(dead_code)]

use core_identity::fingerprint;
use core_work_order::*;
use serde::Serialize;

const BASE: &str = "ac90b1f48c5551e65ecadace95c59f7f0647062f";
const RESOLVER_SCHEMA: &str = "nexlabs.core.source-resolver";

pub struct Fixture {
    pub budget: M03ResourceBudgetV1,
    pub request: WorkOrderRequestV1,
    pub context: CompilationContextV1,
}

pub fn fp<T: Serialize>(value: &T) -> EvidenceFingerprintV1 {
    EvidenceFingerprintV1::new(fingerprint(value).expect("fixture serializes")).unwrap()
}

pub fn budget() -> M03ResourceBudgetV1 {
    M03ResourceBudgetV1::new(
        128_000,
        128_000,
        16_000,
        32,
        32,
        128,
        256,
        64,
        64,
        256,
        128,
        256,
        64,
        256,
        64,
        64,
        ResourceCalibrationStateV1::Uncalibrated,
    )
    .unwrap()
}

fn scope() -> ScopeEnvelopeV1 {
    ScopeEnvelopeV1 {
        allowed_modules: vec!["M03".into()],
        denied_modules: vec![],
        allowed_crates_or_packages: vec!["core-work-order".into()],
        denied_crates_or_packages: vec![],
        allowed_path_prefixes: vec!["crates/core-work-order".into()],
        denied_path_prefixes: vec![],
        allowed_artifact_classes: vec!["source".into(), "test".into()],
        denied_artifact_classes: vec![],
        source_mutation_policy: MutationPolicyV1::WithinAllowedScope,
        documentation_mutation_policy: MutationPolicyV1::WithinAllowedScope,
        evidence_mutation_policy: MutationPolicyV1::WithinAllowedScope,
        generated_artifact_policy: MutationPolicyV1::WithinAllowedScope,
        dependency_policy: DependencyPolicyV1 {
            change_policy: DependencyChangePolicyV1::NoChanges,
            allowed_direct_dependencies: vec![],
        },
        allowed_correction_classes: vec![
            CorrectionClassV1::DocumentationWithinScope,
            CorrectionClassV1::EvidenceRegeneration,
            CorrectionClassV1::TestOnlyWithinScope,
        ],
        maximum_scope_class: ScopeClassV1::Repository,
    }
}

fn context_budget(mandatory: Vec<SourceRefId>) -> ContextBudgetEnvelopeV1 {
    ContextBudgetEnvelopeV1 {
        max_manifest_entries: 8,
        max_inline_bytes: 32_000,
        max_expanded_source_bytes: 64_000,
        max_packet_inline_bytes: 16_000,
        max_packet_expanded_bytes: 32_000,
        max_hive_refs: 8,
        max_prior_evidence_refs: 8,
        mandatory_source_ids: mandatory,
        expansion_policy: SourceExpansionPolicyV1::PacketOnDemand,
        allowed_expansion_reasons: vec!["explicit_packet_requirement".into()],
    }
}

fn stop_condition(
    criteria: Vec<AcceptanceCriterionId>,
    evidence: Vec<EvidenceRequirementId>,
) -> StopConditionV1 {
    StopConditionV1 {
        success_predicates: vec!["all_acceptance_and_evidence_complete".into()],
        blocked_predicates: vec!["any_required_gate_unproven".into()],
        prohibited_early_exit_conditions: vec!["do_not_stop_before_all_gates".into()],
        required_acceptance_criterion_ids: criteria,
        required_evidence_requirement_ids: evidence,
        reviewer_verdict_required: true,
        checkpoint_promotion_allowed: false,
    }
}

fn source(id: &str, locator: &str, packet: &str, fingerprint_hex: char) -> CanonicalSourceRefV1 {
    CanonicalSourceRefV1 {
        source_id: SourceRefId::new(id).unwrap(),
        source_class: if id == "source-1" {
            SourceClassV1::Checkpoint
        } else {
            SourceClassV1::DecisionLedger
        },
        authority_domain: AuthorityDomainV1::CanonicalGit,
        locator_kind: LocatorKindV1::RepositoryPath,
        locator: locator.into(),
        expected_semantic_fingerprint: EvidenceFingerprintV1::new(
            fingerprint_hex.to_string().repeat(64),
        )
        .unwrap(),
        required_for_compile: true,
        required_for_admission: true,
        required_packet_ids: vec![WorkPacketId::new(packet).unwrap()],
        freshness_policy: SourceFreshnessPolicyV1::RequireFingerprintMatch,
        provenance_class: ProvenanceClassV1::CanonicalGit,
        secret_classification: SecretClassificationV1::Internal,
        expansion_policy: SourceExpansionPolicyV1::AlwaysLoad,
    }
}

pub fn fixture() -> Fixture {
    let work_order_id = WorkOrderId::new("work-order-1").unwrap();
    let s1 = source("source-1", "synthetic/checkpoint.md", "packet-1", '1');
    let s2 = source("source-2", "synthetic/decision-ledger.md", "packet-2", '2');
    let source_ids = vec![s1.source_id.clone(), s2.source_id.clone()];
    let criteria = vec![
        AcceptanceCriterionId::new("criterion-1").unwrap(),
        AcceptanceCriterionId::new("criterion-2").unwrap(),
    ];
    let evidence = vec![
        EvidenceRequirementId::new("evidence-1").unwrap(),
        EvidenceRequirementId::new("evidence-2").unwrap(),
    ];
    let common_scope = scope();
    let packet_ids = [
        WorkPacketId::new("packet-1").unwrap(),
        WorkPacketId::new("packet-2").unwrap(),
    ];
    let packets = packet_ids
        .iter()
        .enumerate()
        .map(|(index, packet_id)| WorkPacketSpecV1 {
            packet_id: packet_id.clone(),
            objective: format!("complete bounded packet {}", index + 1),
            prerequisite_packet_ids: if index == 0 {
                vec![]
            } else {
                vec![packet_ids[0].clone()]
            },
            required_source_ids: vec![source_ids[index].clone()],
            workspace_freshness_profile: WorkspaceFreshnessProfileV1::PlanWork,
            scope: common_scope.clone(),
            acceptance_criterion_ids: vec![criteria[index].clone()],
            evidence_requirement_ids: vec![evidence[index].clone()],
            context_budget: context_budget(vec![source_ids[0].clone()]),
            stop_condition: stop_condition(
                vec![criteria[index].clone()],
                vec![evidence[index].clone()],
            ),
        })
        .collect();
    let acceptance = AcceptanceEvidenceGraphV1 {
        criteria: criteria
            .iter()
            .enumerate()
            .map(|(index, criterion_id)| AcceptanceCriterionV1 {
                criterion_id: criterion_id.clone(),
                statement: format!("criterion {} is met", index + 1),
                blocking: true,
                required_evidence_ids: vec![evidence[index].clone()],
                packet_ids: vec![packet_ids[index].clone()],
                applicability_policy: ApplicabilityPolicyV1::AlwaysApplicable,
                not_applicable_rationale: None,
            })
            .collect(),
        evidence_requirements: evidence
            .iter()
            .enumerate()
            .map(|(index, evidence_id)| EvidenceRequirementV1 {
                evidence_id: evidence_id.clone(),
                evidence_class: "exact_head_test_result".into(),
                producer_module_hint: "M03".into(),
                exact_head_required: true,
                platform_requirements: vec!["windows".into(), "ubuntu".into()],
                freshness_policy: SourceFreshnessPolicyV1::RequireCurrent,
                packet_ids: vec![packet_ids[index].clone()],
                security_classification: SecretClassificationV1::Internal,
                global: false,
            })
            .collect(),
        edges: criteria
            .iter()
            .enumerate()
            .map(|(index, criterion_id)| CriterionEvidenceEdgeV1 {
                criterion_id: criterion_id.clone(),
                evidence_id: evidence[index].clone(),
            })
            .collect(),
    };
    let mut workspace_components: Vec<String> = vec![
        "Identity".into(),
        "Authority".into(),
        "FilesystemSemantics".into(),
        "TrackedWorktreeState".into(),
        "UntrackedWorktreeState".into(),
        "ConfigGeneration".into(),
        "SecurityPolicy".into(),
        "RepositoryGraph".into(),
        "HeadState".into(),
        "IndexState".into(),
    ];
    workspace_components.sort();
    let authorized_scope_fingerprint = scope_fingerprint(&common_scope).unwrap();
    let request = WorkOrderRequestV1 {
        requested_work_order_id: Some(work_order_id.clone()),
        logical_key: None,
        objective: "compile a deterministic, bounded two-packet work order".into(),
        sources: vec![s1.clone(), s2.clone()],
        workspace: WorkspaceRequirementV1 {
            expected_project_binding_id: Some("project-binding-1".into()),
            expected_workspace_id: Some("workspace-1".into()),
            required_profile: WorkspaceFreshnessProfileV1::PlanWork,
            required_basis_components: vec!["Identity".into()],
            allow_compatible_refresh: false,
            assurance_requirement: WorkspaceAssuranceRequirementV1::StandaloneRequired,
            dirty_untracked_policy: DirtyUntrackedPolicyV1::RequireClean,
            required_m02_schema: "nexlabs.core.workspace".into(),
            required_m02_version: 1,
        },
        context_lock: ContextLockRequirementV1 {
            required_schema: "nexlabs.core.context-lock".into(),
            required_version: 1,
            require_active: true,
            authorized_base_constraint: BASE.into(),
            required_source_ids: source_ids.clone(),
            require_implementation_authorized: true,
            staleness_policy: SourceFreshnessPolicyV1::RequireCurrent,
        },
        governance: GovernanceRequirementV1 {
            required_verdict: ExternalGovernanceVerdictV1::Accepted,
            require_exact_base: true,
            require_exact_head: true,
            authorized_scope_fingerprint,
            minimum_policy_generation: 1,
            require_current_freshness: true,
        },
        scope: common_scope,
        packets,
        acceptance,
        context_budget: context_budget(vec![source_ids[0].clone()]),
        correction_policy: CorrectionPolicyV1 {
            same_revision_classes: vec![
                CorrectionClassV1::DocumentationWithinScope,
                CorrectionClassV1::EvidenceRegeneration,
                CorrectionClassV1::TestOnlyWithinScope,
            ],
            forbidden_classes: vec![
                CorrectionClassV1::AcceptanceWeakening,
                CorrectionClassV1::ScopeExpansion,
                CorrectionClassV1::StopConditionWeakening,
            ],
        },
        stop_condition: stop_condition(criteria.clone(), evidence.clone()),
        risk_assurance: RiskAssuranceProfileV1 {
            risk_class: "elevated".into(),
            assurance_mode: "independent_review_required".into(),
            high_critical_blocking: true,
        },
        parent: None,
    };
    let resolver_schema = RESOLVER_SCHEMA.to_owned();
    let mut source_entries: Vec<SourceResolutionEvidenceV1> = vec![s1, s2]
        .into_iter()
        .map(|source_ref| {
            let mut entry = SourceResolutionEvidenceV1 {
                source_id: source_ref.source_id,
                requested_fingerprint: source_ref.expected_semantic_fingerprint.clone(),
                observed_fingerprint: source_ref.expected_semantic_fingerprint,
                authority_domain: AuthorityDomainV1::CanonicalGit,
                source_revision: "git-blob-v1".into(),
                resolver_schema: resolver_schema.clone(),
                resolver_version: 1,
                freshness_state: EvidenceFreshnessV1::Current,
                provenance_fingerprint: EvidenceFingerprintV1::new("4".repeat(64)).unwrap(),
                evidence_fingerprint: EvidenceFingerprintV1::new("0".repeat(64)).unwrap(),
            };
            entry.evidence_fingerprint = fp(&(
                entry.source_id.as_str(),
                &entry.requested_fingerprint,
                &entry.observed_fingerprint,
                entry.authority_domain,
                &entry.source_revision,
                &entry.resolver_schema,
                entry.resolver_version,
                entry.freshness_state,
                &entry.provenance_fingerprint,
            ));
            entry
        })
        .collect();
    source_entries.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    let batch_fingerprint = fp(&(&resolver_schema, 1u16, source_entries.clone()));
    let sources = SourceResolutionBatchV1 {
        resolver_schema,
        resolver_version: 1,
        entries: source_entries,
        batch_fingerprint,
    };
    let current_revision: Option<WorkOrderRevision> = None;
    let current_fingerprint: Option<WorkOrderFingerprint> = None;
    let current_compilation_id: Option<WorkOrderCompilationId> = None;
    let superseded_revisions: Vec<WorkOrderRevisionRefV1> = vec![];
    let lineage_edges: Vec<WorkOrderLineageEdgeV1> = vec![];
    let provenance_fingerprint = EvidenceFingerprintV1::new("5".repeat(64)).unwrap();
    let snapshot_fingerprint = fp(&(
        &work_order_id,
        current_revision,
        &current_fingerprint,
        &current_compilation_id,
        0u64,
        superseded_revisions.clone(),
        lineage_edges.clone(),
        &provenance_fingerprint,
    ));
    Fixture {
        budget: budget(),
        request,
        context: CompilationContextV1 {
            compiler_contract_version: 1,
            algorithm_version: "m03-woc-v1".into(),
            policy_generation: 1,
            security_generation: 1,
            config_generation: 1,
            sources,
            lineage: LineageSnapshotV1 {
                work_order_id,
                current_revision,
                current_fingerprint,
                current_compilation_id,
                store_generation: 0,
                superseded_revisions,
                edges: lineage_edges,
                snapshot_fingerprint,
                provenance_fingerprint,
            },
            hive_context_refs: vec![],
        },
    }
}

pub fn ready_admission(
    request: &WorkOrderRequestV1,
    context: &CompilationContextV1,
    frozen: &FrozenWorkOrderV1,
) -> WorkOrderAdmissionRequestV1 {
    let profile = request.workspace.required_profile;
    let mut components: Vec<String> = vec![
        "Identity".into(),
        "Authority".into(),
        "FilesystemSemantics".into(),
        "TrackedWorktreeState".into(),
        "UntrackedWorktreeState".into(),
        "ConfigGeneration".into(),
        "SecurityPolicy".into(),
        "RepositoryGraph".into(),
        "HeadState".into(),
        "IndexState".into(),
    ];
    components.sort();
    let workspace_provenance = EvidenceFingerprintV1::new("6".repeat(64)).unwrap();
    let basis_fingerprint = EvidenceFingerprintV1::new("7".repeat(64)).unwrap();
    let mut workspace = WorkspaceAdmissionEvidenceV1 {
        m02_schema: "nexlabs.core.workspace".into(),
        m02_version: 1,
        project_binding_id: Some("project-binding-1".into()),
        workspace_id: "workspace-1".into(),
        runtime_epoch: 3,
        generation: 9,
        basis_fingerprint,
        required_profile: profile,
        satisfied_components: components,
        compatibility: BasisCompatibilityV1::ExactMatch,
        freshness: EvidenceFreshnessV1::Current,
        provenance_fingerprint: workspace_provenance,
        proof_fingerprint: EvidenceFingerprintV1::new("0".repeat(64)).unwrap(),
    };
    workspace.proof_fingerprint = fp(&(
        &workspace.m02_schema,
        workspace.m02_version,
        &workspace.project_binding_id,
        &workspace.workspace_id,
        workspace.runtime_epoch,
        workspace.generation,
        &workspace.basis_fingerprint,
        workspace.required_profile,
        &workspace.satisfied_components,
        workspace.compatibility,
        workspace.freshness,
        &workspace.provenance_fingerprint,
    ));
    let required: std::collections::BTreeSet<_> =
        request.context_lock.required_source_ids.iter().collect();
    let mut selected_sources: Vec<_> = request
        .sources
        .iter()
        .filter(|s| required.contains(&s.source_id))
        .collect();
    selected_sources.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    let source_set_fingerprint = fp(&selected_sources);
    let base_fingerprint =
        authorized_base_fingerprint(&request.context_lock.authorized_base_constraint).unwrap();
    let lock_verifier = EvidenceFingerprintV1::new("8".repeat(64)).unwrap();
    let mut lock = ContextLockEvidenceV1 {
        schema: request.context_lock.required_schema.clone(),
        version: request.context_lock.required_version,
        status: ContextLockStateV1::Active,
        lock_fingerprint: EvidenceFingerprintV1::new("9".repeat(64)).unwrap(),
        work_order_id: frozen.work_order_id().clone(),
        revision: frozen.revision(),
        work_order_fingerprint: frozen.fingerprint().clone(),
        authorized_base_fingerprint: base_fingerprint,
        source_set_fingerprint,
        implementation_authorized: true,
        policy_generation: 1,
        freshness: EvidenceFreshnessV1::Current,
        verifier_provenance: lock_verifier,
        proof_fingerprint: EvidenceFingerprintV1::new("0".repeat(64)).unwrap(),
    };
    lock.proof_fingerprint = fp(&(
        &lock.schema,
        lock.version,
        lock.status,
        &lock.lock_fingerprint,
        &lock.work_order_id,
        lock.revision,
        &lock.work_order_fingerprint,
        &lock.authorized_base_fingerprint,
        &lock.source_set_fingerprint,
        lock.implementation_authorized,
        lock.policy_generation,
        lock.freshness,
        &lock.verifier_provenance,
    ));
    let mut governance = VerifiedGovernanceProofV1 {
        schema: "nexlabs.core.gef-governance-proof".into(),
        version: 1,
        proof_id: GovernanceProofId::new("proof-1").unwrap(),
        project_repository_fingerprint: EvidenceFingerprintV1::new("a".repeat(64)).unwrap(),
        work_order_id: frozen.work_order_id().clone(),
        revision: frozen.revision(),
        work_order_fingerprint: frozen.fingerprint().clone(),
        exact_base: request.context_lock.authorized_base_constraint.clone(),
        exact_head: Some("frozen-head-1".into()),
        verdict: ExternalGovernanceVerdictV1::Accepted,
        authorized_scope_fingerprint: request.governance.authorized_scope_fingerprint.clone(),
        policy_generation: 1,
        freshness: EvidenceFreshnessV1::Current,
        verifier_provenance: EvidenceFingerprintV1::new("b".repeat(64)).unwrap(),
        proof_fingerprint: EvidenceFingerprintV1::new("0".repeat(64)).unwrap(),
    };
    governance.proof_fingerprint = fp(&(
        &governance.schema,
        governance.version,
        &governance.proof_id,
        &governance.project_repository_fingerprint,
        &governance.work_order_id,
        governance.revision,
        &governance.work_order_fingerprint,
        &governance.exact_base,
        &governance.exact_head,
        governance.verdict,
        &governance.authorized_scope_fingerprint,
        governance.policy_generation,
        governance.freshness,
        &governance.verifier_provenance,
    ));
    WorkOrderAdmissionRequestV1 {
        work_order_id: frozen.work_order_id().clone(),
        revision: frozen.revision(),
        work_order_fingerprint: frozen.fingerprint().clone(),
        requested_mode: AdmissionModeV1::Execution,
        sources: context.sources.clone(),
        workspace,
        context_lock: Some(lock),
        governance: Some(governance),
        policy_generation: 1,
        security_generation: 1,
        config_generation: 1,
    }
}

pub fn compiled(fixture: &Fixture) -> WorkOrderCompilationV1 {
    compile(&fixture.request, &fixture.context, &fixture.budget).unwrap()
}
