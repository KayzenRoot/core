#[path = "../crates/core-work-order/tests/common/mod.rs"]
mod common;

use core_work_order::*;
use std::hint::black_box;
use std::time::Instant;

const SAMPLE_COUNT: usize = 5;
const OPERATIONS_PER_SAMPLE: usize = 10;

fn measure(name: &str, mut operation: impl FnMut()) {
    operation();
    let mut samples = Vec::with_capacity(SAMPLE_COUNT);
    for sample in 0..SAMPLE_COUNT {
        let start = Instant::now();
        for _ in 0..OPERATIONS_PER_SAMPLE {
            operation();
        }
        let elapsed = start.elapsed().as_nanos() / OPERATIONS_PER_SAMPLE as u128;
        samples.push(elapsed);
        println!("scenario={name} sample={} ns_per_op={elapsed}", sample + 1);
    }
    samples.sort_unstable();
    println!(
        "summary scenario={name} warmups=1 measured={} operations_per_sample={} min_ns_per_op={} median_ns_per_op={} max_ns_per_op={}",
        samples.len(),
        OPERATIONS_PER_SAMPLE,
        samples[0],
        samples[samples.len() / 2],
        samples[samples.len() - 1]
    );
}

fn scope_rule_count(scope: &ScopeEnvelopeV1) -> usize {
    scope.allowed_modules.len()
        + scope.denied_modules.len()
        + scope.allowed_crates_or_packages.len()
        + scope.denied_crates_or_packages.len()
        + scope.allowed_path_prefixes.len()
        + scope.denied_path_prefixes.len()
        + scope.allowed_artifact_classes.len()
        + scope.denied_artifact_classes.len()
        + scope.dependency_policy.allowed_direct_dependencies.len()
        + scope.allowed_correction_classes.len()
}

fn scaled_packet_fixture(packet_count: usize, chain: bool) -> common::Fixture {
    let mut fixture = common::fixture();
    assert!(packet_count >= fixture.request.packets.len());
    let shared_source_id = fixture.request.sources[0].source_id.clone();
    for packet in &mut fixture.request.packets {
        packet.required_source_ids = vec![shared_source_id.clone()];
    }
    let template = fixture.request.packets[0].clone();
    let evidence_template = fixture.request.acceptance.evidence_requirements[0].clone();
    for index in fixture.request.packets.len()..packet_count {
        let packet_id = WorkPacketId::new(format!("packet-{}", index + 1)).unwrap();
        let criterion_id = AcceptanceCriterionId::new(format!("criterion-{}", index + 1)).unwrap();
        let evidence_id = EvidenceRequirementId::new(format!("evidence-{}", index + 1)).unwrap();
        let mut packet = template.clone();
        packet.packet_id = packet_id.clone();
        packet.objective = format!("complete synthetic packet {}", index + 1);
        packet.prerequisite_packet_ids = if chain {
            vec![WorkPacketId::new(format!("packet-{index}")).unwrap()]
        } else {
            Vec::new()
        };
        packet.acceptance_criterion_ids = vec![criterion_id.clone()];
        packet.evidence_requirement_ids = vec![evidence_id.clone()];
        packet.stop_condition.required_acceptance_criterion_ids = vec![criterion_id.clone()];
        packet.stop_condition.required_evidence_requirement_ids = vec![evidence_id.clone()];
        fixture.request.packets.push(packet);
        fixture
            .request
            .acceptance
            .criteria
            .push(AcceptanceCriterionV1 {
                criterion_id: criterion_id.clone(),
                statement: format!("synthetic criterion {} is met", index + 1),
                blocking: true,
                required_evidence_ids: vec![evidence_id.clone()],
                packet_ids: vec![packet_id.clone()],
                applicability_policy: ApplicabilityPolicyV1::AlwaysApplicable,
                not_applicable_rationale: None,
            });
        let mut evidence = evidence_template.clone();
        evidence.evidence_id = evidence_id.clone();
        evidence.packet_ids = vec![packet_id];
        fixture
            .request
            .acceptance
            .evidence_requirements
            .push(evidence);
        fixture
            .request
            .acceptance
            .edges
            .push(CriterionEvidenceEdgeV1 {
                criterion_id: criterion_id.clone(),
                evidence_id: evidence_id.clone(),
            });
        fixture
            .request
            .stop_condition
            .required_acceptance_criterion_ids
            .push(criterion_id);
        fixture
            .request
            .stop_condition
            .required_evidence_requirement_ids
            .push(evidence_id);
    }
    fixture.budget.max_packets = packet_count as u64;
    fixture.budget.max_packet_edges = if chain {
        packet_count.saturating_sub(1) as u64
    } else {
        1
    };
    fixture.budget.max_criteria = packet_count as u64;
    fixture.budget.max_evidence_requirements = packet_count as u64;
    fixture.budget.max_acceptance_evidence_edges = packet_count as u64;
    fixture.budget.max_context_refs = packet_count as u64;
    fixture
}

fn scaled_source_fixture(source_count: usize) -> common::Fixture {
    let mut fixture = common::fixture();
    assert!(source_count >= fixture.request.sources.len());
    let template = fixture.request.sources[1].clone();
    let mut entries = fixture.context.sources.entries.clone();
    for index in fixture.request.sources.len()..source_count {
        let source_id = SourceRefId::new(format!("source-{}", index + 1)).unwrap();
        let mut source = template.clone();
        source.source_id = source_id.clone();
        source.locator = format!("synthetic/source-{}.md", index + 1);
        source.expected_semantic_fingerprint = common::fp(&source.locator);
        source.required_packet_ids = vec![WorkPacketId::new("packet-1").unwrap()];
        let mut entry = SourceResolutionEvidenceV1 {
            source_id: source_id.clone(),
            requested_fingerprint: source.expected_semantic_fingerprint.clone(),
            observed_fingerprint: source.expected_semantic_fingerprint.clone(),
            authority_domain: AuthorityDomainV1::CanonicalGit,
            source_revision: "synthetic-revision-v1".into(),
            resolver_schema: fixture.context.sources.resolver_schema.clone(),
            resolver_version: fixture.context.sources.resolver_version,
            freshness_state: EvidenceFreshnessV1::Current,
            provenance_fingerprint: common::fp(&("synthetic-provenance", index)),
            evidence_fingerprint: common::fp(&"synthetic-placeholder"),
        };
        entry.evidence_fingerprint = common::fp(&(
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
        fixture.request.sources.push(source);
        fixture
            .request
            .context_lock
            .required_source_ids
            .push(source_id);
        entries.push(entry);
    }
    fixture
        .request
        .sources
        .sort_by(|a, b| a.source_id.cmp(&b.source_id));
    fixture.request.context_lock.required_source_ids.sort();
    fixture.request.context_budget.max_manifest_entries = source_count as u64;
    for packet in &mut fixture.request.packets {
        packet.context_budget.max_manifest_entries = source_count as u64;
    }
    entries.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    fixture.context.sources.entries = entries;
    fixture.context.sources.batch_fingerprint = common::fp(&(
        &fixture.context.sources.resolver_schema,
        fixture.context.sources.resolver_version,
        fixture.context.sources.entries.clone(),
    ));
    fixture.budget.max_source_refs = source_count as u64;
    fixture.budget.max_context_refs = source_count as u64;
    fixture
}

fn scaled_scope_fixture(rule_count: usize) -> common::Fixture {
    let mut fixture = common::fixture();
    let existing = scope_rule_count(&fixture.request.scope);
    assert!(rule_count >= existing);
    for index in 0..(rule_count - existing) {
        fixture
            .request
            .scope
            .allowed_modules
            .push(format!("M03-SYNTHETIC-{index:03}"));
    }
    fixture.request.governance.authorized_scope_fingerprint =
        scope_fingerprint(&fixture.request.scope).unwrap();
    fixture.budget.max_scope_rules = rule_count as u64;
    fixture
}

fn lineage_fixture(edge_count: usize) -> common::Fixture {
    let mut fixture = common::fixture();
    let work_order_id = fixture.context.lineage.work_order_id.clone();
    let mut superseded = Vec::with_capacity(edge_count);
    let mut edges = Vec::with_capacity(edge_count);
    for index in 1..=edge_count {
        let from_revision = WorkOrderRevision::new(index as u32).unwrap();
        let to_revision = from_revision.next().unwrap();
        let from_fingerprint = WorkOrderFingerprint::new(format!("{index:064x}")).unwrap();
        let to_fingerprint = WorkOrderFingerprint::new(format!("{:064x}", index + 1)).unwrap();
        superseded.push(WorkOrderRevisionRefV1 {
            work_order_id: work_order_id.clone(),
            revision: from_revision,
            fingerprint: from_fingerprint.clone(),
        });
        edges.push(WorkOrderLineageEdgeV1 {
            edge_id: LineageEdgeId::new(format!("lineage-edge-{index:04}")).unwrap(),
            from_revision,
            from_fingerprint,
            to_revision,
            to_fingerprint,
            relation: LineageRelationV1::Supersedes,
        });
    }
    let current_revision = WorkOrderRevision::new((edge_count + 1) as u32).unwrap();
    let current_fingerprint =
        WorkOrderFingerprint::new(format!("{:064x}", edge_count + 1)).unwrap();
    let snapshot = &mut fixture.context.lineage;
    snapshot.current_revision = Some(current_revision);
    snapshot.current_fingerprint = Some(current_fingerprint.clone());
    snapshot.store_generation = edge_count as u64 + 1;
    snapshot.superseded_revisions = superseded;
    snapshot.edges = edges;
    snapshot.snapshot_fingerprint = common::fp(&(
        &snapshot.work_order_id,
        snapshot.current_revision,
        &snapshot.current_fingerprint,
        &snapshot.current_compilation_id,
        snapshot.store_generation,
        snapshot.superseded_revisions.clone(),
        snapshot.edges.clone(),
        &snapshot.provenance_fingerprint,
    ));
    fixture.request.parent = Some(WorkOrderRevisionRefV1 {
        work_order_id,
        revision: current_revision,
        fingerprint: current_fingerprint,
    });
    fixture.budget.max_lineage_edges = edge_count as u64;
    fixture
}

fn next_revision_fixture(current: &FrozenWorkOrderV1) -> common::Fixture {
    let mut fixture = common::fixture();
    fixture.request.parent = Some(WorkOrderRevisionRefV1 {
        work_order_id: current.work_order_id().clone(),
        revision: current.revision(),
        fingerprint: current.fingerprint().clone(),
    });
    let snapshot = &mut fixture.context.lineage;
    snapshot.current_revision = Some(current.revision());
    snapshot.current_fingerprint = Some(current.fingerprint().clone());
    snapshot.current_compilation_id = Some(current.compilation_id().clone());
    snapshot.store_generation = 1;
    snapshot.snapshot_fingerprint = common::fp(&(
        &snapshot.work_order_id,
        snapshot.current_revision,
        &snapshot.current_fingerprint,
        &snapshot.current_compilation_id,
        snapshot.store_generation,
        snapshot.superseded_revisions.clone(),
        snapshot.edges.clone(),
        &snapshot.provenance_fingerprint,
    ));
    fixture
        .request
        .objective
        .push_str(" with a second semantic change");
    fixture.request.risk_assurance.risk_class = "synthetic-calibration-risk".into();
    fixture
}

fn measure_boundaries() {
    let base = common::fixture();
    let request_candidate = scaled_packet_fixture(32, true);
    let envelope = WorkOrderEnvelope::new(
        WorkOrderContractKindV1::Request,
        request_candidate.request.clone(),
    );
    let encoded = serde_json::to_vec(&envelope).unwrap();
    let mut budget = request_candidate.budget.clone();
    budget.max_request_bytes = encoded.len() as u64;
    assert_eq!(parse_request(&encoded, &budget).unwrap(), envelope);
    let mut over_request = scaled_packet_fixture(32, true);
    over_request.request.objective.push('x');
    let over_envelope =
        WorkOrderEnvelope::new(WorkOrderContractKindV1::Request, over_request.request);
    let over_encoded = serde_json::to_vec(&over_envelope).unwrap();
    assert_eq!(over_encoded.len(), encoded.len() + 1);
    assert_eq!(
        parse_request(&over_encoded, &budget).unwrap_err().code,
        WorkOrderErrorCodeV1::RequestTooLarge
    );
    println!(
        "boundary dimension=request_bytes candidate={} cap_plus_one={} result=PASS",
        encoded.len(),
        over_encoded.len()
    );

    let frozen_candidate = compile(
        &request_candidate.request,
        &request_candidate.context,
        &request_candidate.budget,
    )
    .unwrap();
    let frozen_bytes = serde_json::to_vec(&frozen_candidate.frozen).unwrap().len() as u64;
    let mut exact_frozen = request_candidate.budget.clone();
    exact_frozen.max_frozen_bytes = frozen_bytes;
    compile(
        &request_candidate.request,
        &request_candidate.context,
        &exact_frozen,
    )
    .unwrap();
    let mut over_frozen = scaled_packet_fixture(32, true);
    over_frozen.request.objective.push('x');
    assert_eq!(
        compile(&over_frozen.request, &over_frozen.context, &exact_frozen)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::SerializedSizeExceeded
    );
    println!(
        "boundary dimension=frozen_bytes candidate={} cap_plus_one={} result=PASS",
        frozen_bytes,
        frozen_bytes + 1
    );

    let mut exact_string = common::fixture();
    exact_string.request.objective = "x".repeat(4_096);
    exact_string.budget.max_string_bytes = 4_096;
    compile(
        &exact_string.request,
        &exact_string.context,
        &exact_string.budget,
    )
    .unwrap();
    exact_string.request.objective.push('x');
    assert!(compile(
        &exact_string.request,
        &exact_string.context,
        &exact_string.budget
    )
    .is_err());
    println!("boundary dimension=string_bytes candidate=4096 cap_plus_one=4097 result=PASS");

    let sources = scaled_source_fixture(32);
    compile(&sources.request, &sources.context, &sources.budget).unwrap();
    let mut over_sources = scaled_source_fixture(33);
    over_sources.budget.max_source_refs = 32;
    over_sources.budget.max_context_refs = 32;
    assert_eq!(
        compile(
            &over_sources.request,
            &over_sources.context,
            &over_sources.budget
        )
        .unwrap_err()
        .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!("boundary dimension=source_refs candidate=32 cap_plus_one=33 result=PASS");
    let width = scaled_packet_fixture(32, false);
    compile(&width.request, &width.context, &width.budget).unwrap();
    let mut over_packets = scaled_packet_fixture(32, false);
    over_packets.budget.max_packets = 31;
    assert_eq!(
        compile(
            &over_packets.request,
            &over_packets.context,
            &over_packets.budget
        )
        .unwrap_err()
        .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );
    println!("boundary dimension=packets candidate=32 cap_plus_one=33 result=PASS");
    let depth = scaled_packet_fixture(32, true);
    compile(&depth.request, &depth.context, &depth.budget).unwrap();
    let mut over_edges = scaled_packet_fixture(32, true);
    over_edges.budget.max_packet_edges = 30;
    assert_eq!(
        compile(&over_edges.request, &over_edges.context, &over_edges.budget)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!("boundary dimension=packet_edges candidate=31 cap_plus_one=32 result=PASS");
    let scope_rules = scope_rule_count(&base.request.scope);
    let mut exact_scope = common::fixture();
    exact_scope.budget.max_scope_rules = scope_rules as u64;
    compile(
        &exact_scope.request,
        &exact_scope.context,
        &exact_scope.budget,
    )
    .unwrap();
    let mut over_scope = common::fixture();
    over_scope.budget.max_scope_rules = scope_rules as u64 - 1;
    assert_eq!(
        compile(&over_scope.request, &over_scope.context, &over_scope.budget)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );
    println!(
        "boundary dimension=scope_rules candidate={scope_rules} cap_plus_one={} result=PASS",
        scope_rules + 1
    );
    let scaled_scope = scaled_scope_fixture(64);
    compile(
        &scaled_scope.request,
        &scaled_scope.context,
        &scaled_scope.budget,
    )
    .unwrap();
    let mut over_scope = scaled_scope_fixture(65);
    over_scope.budget.max_scope_rules = 64;
    assert!(compile(&over_scope.request, &over_scope.context, &over_scope.budget).is_err());

    println!("boundary dimension=scope_rules_scaled candidate=64 cap_plus_one=65 result=PASS");
    let scaled_aeg = scaled_packet_fixture(32, true);
    compile(&scaled_aeg.request, &scaled_aeg.context, &scaled_aeg.budget).unwrap();
    for (field, expected) in [
        ("criteria", WorkOrderErrorCodeV1::CardinalityLimitExceeded),
        ("evidence", WorkOrderErrorCodeV1::CardinalityLimitExceeded),
        ("edges", WorkOrderErrorCodeV1::CardinalityLimitExceeded),
    ] {
        let mut candidate = scaled_packet_fixture(32, true);
        match field {
            "criteria" => candidate.budget.max_criteria = 31,
            "evidence" => candidate.budget.max_evidence_requirements = 31,
            _ => candidate.budget.max_acceptance_evidence_edges = 31,
        }
        assert_eq!(
            compile(&candidate.request, &candidate.context, &candidate.budget)
                .unwrap_err()
                .code,
            expected,
            "{field} cap-plus-one"
        );
    }

    println!("boundary dimensions=criteria,evidence_nodes,aeg_edges candidate=32 cap_plus_one=33 result=PASS");
    let lineage = lineage_fixture(32);
    compile(&lineage.request, &lineage.context, &lineage.budget).unwrap();
    let mut over_lineage = lineage_fixture(32);
    over_lineage.budget.max_lineage_edges = 31;
    assert_eq!(
        compile(
            &over_lineage.request,
            &over_lineage.context,
            &over_lineage.budget
        )
        .unwrap_err()
        .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!("boundary dimension=lineage_edges candidate=32 cap_plus_one=33 result=PASS");
    let mut context_refs = scaled_source_fixture(32);
    context_refs.budget.max_context_refs = 32;
    compile(
        &context_refs.request,
        &context_refs.context,
        &context_refs.budget,
    )
    .unwrap();
    context_refs.budget.max_context_refs = 31;
    assert_eq!(
        compile(
            &context_refs.request,
            &context_refs.context,
            &context_refs.budget
        )
        .unwrap_err()
        .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!("boundary dimension=context_refs candidate=32 cap_plus_one=33 result=PASS");
    let baseline = compile(&base.request, &base.context, &base.budget).unwrap();
    let frozen = baseline.frozen;
    let correction = ExecutionCorrectionProposalV1 {
        proposal_fingerprint: EvidenceFingerprintV1::new("c".repeat(64)).unwrap(),
        changed_paths: (0..64)
            .map(|index| format!("crates/core-work-order/tests/synthetic-{index:03}.rs"))
            .collect(),
        artifact_classes: Vec::new(),
        requested_classes: vec![CorrectionClassV1::TestOnlyWithinScope],
        added_dependencies: Vec::new(),
        changed_semantic_fields: Vec::new(),
    };
    let mut correction_budget = base.budget.clone();
    correction_budget.max_correction_rules = 64;
    assert_eq!(
        classify_correction(&frozen, &correction, &correction_budget)
            .unwrap()
            .disposition,
        CorrectionDispositionV1::AllowedSameRevision
    );
    correction_budget.max_correction_rules = 63;
    assert_eq!(
        classify_correction(&frozen, &correction, &correction_budget)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!("boundary dimension=correction_rules candidate=64 cap_plus_one=65 result=PASS");
    let next = next_revision_fixture(&frozen);
    let next_frozen = compile(&next.request, &next.context, &next.budget)
        .unwrap()
        .frozen;
    let diff = diff_revision(&frozen, &next_frozen, &base.budget).unwrap();
    let diff_count = diff.changed_semantic_fields.len() as u64;
    let mut diff_budget = base.budget.clone();
    diff_budget.max_diff_entries = diff_count;
    assert!(diff_revision(&frozen, &next_frozen, &diff_budget).is_ok());
    diff_budget.max_diff_entries = diff_count - 1;
    assert_eq!(
        diff_revision(&frozen, &next_frozen, &diff_budget)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!(
        "boundary dimension=diff_entries candidate={diff_count} cap_plus_one={} result=PASS",
        diff_count + 1
    );
    let admission = common::ready_admission(&base.request, &base.context, &frozen);
    let mut blocked = admission.clone();
    blocked.workspace.freshness = EvidenceFreshnessV1::Unknown;
    blocked.context_lock = None;
    blocked.governance = None;
    blocked.policy_generation = 0;
    blocked.security_generation = 0;
    blocked.config_generation = 0;
    let blocked_receipt = evaluate_admission(&frozen, &blocked, &base.budget).unwrap();
    let diagnostic_count = blocked_receipt.reason_codes().len() as u64;
    assert!(diagnostic_count > 1);
    let mut diagnostic_budget = base.budget.clone();
    diagnostic_budget.max_diagnostic_entries = diagnostic_count;
    assert_eq!(
        evaluate_admission(&frozen, &blocked, &diagnostic_budget)
            .unwrap()
            .reason_codes()
            .len() as u64,
        diagnostic_count
    );
    diagnostic_budget.max_diagnostic_entries = diagnostic_count - 1;
    assert_eq!(
        evaluate_admission(&frozen, &blocked, &diagnostic_budget)
            .unwrap_err()
            .code,
        WorkOrderErrorCodeV1::CardinalityLimitExceeded
    );

    println!(
        "boundary dimension=diagnostic_entries candidate={diagnostic_count} cap_plus_one={} result=PASS",
        diagnostic_count + 1
    );
    let depth = json_depth(&encoded);
    let mut depth_budget = base.budget.clone();
    depth_budget.max_parse_depth = depth;
    parse_request(&encoded, &depth_budget).unwrap();
    depth_budget.max_parse_depth -= 1;
    assert_eq!(
        parse_request(&encoded, &depth_budget).unwrap_err().code,
        WorkOrderErrorCodeV1::ContextLimitExceeded
    );
    println!(
        "boundary dimension=parse_depth candidate={depth} cap_plus_one={} result=PASS",
        depth + 1
    );
}

fn json_depth(bytes: &[u8]) -> u32 {
    let mut depth = 0u32;
    let mut maximum = 0u32;
    let mut in_string = false;
    let mut escaped = false;
    for byte in bytes.iter().copied() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
        } else if byte == b'{' || byte == b'[' {
            depth += 1;
            maximum = maximum.max(depth);
        } else if byte == b'}' || byte == b']' {
            depth -= 1;
        }
    }
    maximum
}

fn main() {
    let fixture = common::fixture();
    let envelope =
        WorkOrderEnvelope::new(WorkOrderContractKindV1::Request, fixture.request.clone());
    let encoded_request = serde_json::to_vec(&envelope).unwrap();
    let compilation = common::compiled(&fixture);
    let frozen = compilation.frozen.clone();
    let canonical = canonical_semantic_bytes(&frozen).unwrap();
    let admission_request = common::ready_admission(&fixture.request, &fixture.context, &frozen);
    let admission = evaluate_admission(&frozen, &admission_request, &fixture.budget).unwrap();
    assert_eq!(admission.status(), WorkOrderAdmissionStatusV1::Ready);
    let handoff = materialize_handoff(&frozen, &admission, &fixture.budget).unwrap();
    assert_eq!(handoff.work_order_fingerprint(), frozen.fingerprint());
    let correction = ExecutionCorrectionProposalV1 {
        proposal_fingerprint: EvidenceFingerprintV1::new("c".repeat(64)).unwrap(),
        changed_paths: vec!["crates/core-work-order/tests/synthetic.rs".into()],
        artifact_classes: vec!["test".into()],
        requested_classes: vec![CorrectionClassV1::TestOnlyWithinScope],
        added_dependencies: Vec::new(),
        changed_semantic_fields: Vec::new(),
    };
    let wide = scaled_packet_fixture(32, false);
    let wide_compilation = common::compiled(&wide);
    let deep = scaled_packet_fixture(32, true);
    let deep_compilation = common::compiled(&deep);
    let sources = scaled_source_fixture(32);
    let source_compilation = common::compiled(&sources);
    let scope = scaled_scope_fixture(64);
    let scope_compilation = common::compiled(&scope);
    let large_string = {
        let mut value = common::fixture();
        value.request.objective = "x".repeat(4_096);
        value.budget.max_string_bytes = 4_096;
        value
    };
    let large_string_envelope = WorkOrderEnvelope::new(
        WorkOrderContractKindV1::Request,
        large_string.request.clone(),
    );
    let large_string_request = serde_json::to_vec(&large_string_envelope).unwrap();
    let large_string_compilation = common::compiled(&large_string);
    let next = next_revision_fixture(&frozen);
    let next_frozen = common::compiled(&next).frozen;
    let lineage = lineage_fixture(32);
    let lineage_compilation = common::compiled(&lineage);

    measure("parse_request_2_sources_2_packets", || {
        assert_eq!(
            parse_request(&encoded_request, &fixture.budget).unwrap(),
            envelope
        );
    });
    measure("compile_2_sources_2_packets", || {
        let current = compile(&fixture.request, &fixture.context, &fixture.budget).unwrap();
        assert_eq!(current.frozen.fingerprint(), frozen.fingerprint());
    });
    measure("compile_32_sources", || {
        let current = compile(&sources.request, &sources.context, &sources.budget).unwrap();
        assert_eq!(
            current.frozen.fingerprint(),
            source_compilation.frozen.fingerprint()
        );
    });
    measure("compile_32_packets_32_aeg_width", || {
        let current = compile(&wide.request, &wide.context, &wide.budget).unwrap();
        assert_eq!(
            current.frozen.fingerprint(),
            wide_compilation.frozen.fingerprint()
        );
    });
    measure("compile_32_packets_32_aeg_depth", || {
        let current = compile(&deep.request, &deep.context, &deep.budget).unwrap();
        assert_eq!(
            current.frozen.fingerprint(),
            deep_compilation.frozen.fingerprint()
        );
    });
    measure("compile_scope_64_rules", || {
        let current = compile(&scope.request, &scope.context, &scope.budget).unwrap();
        assert_eq!(
            current.frozen.fingerprint(),
            scope_compilation.frozen.fingerprint()
        );
    });
    measure("parse_request_4096_byte_string", || {
        assert_eq!(
            parse_request(&large_string_request, &large_string.budget).unwrap(),
            large_string_envelope
        );
    });
    measure("compile_4096_byte_string", || {
        let current = compile(
            &large_string.request,
            &large_string.context,
            &large_string.budget,
        )
        .unwrap();
        assert_eq!(
            current.frozen.fingerprint(),
            large_string_compilation.frozen.fingerprint()
        );
    });
    measure("validate_frozen_2_packets", || {
        validate_frozen(&frozen, &fixture.budget).unwrap();
    });
    measure("canonical_semantic_bytes_2_packets", || {
        assert_eq!(canonical_semantic_bytes(&frozen).unwrap(), canonical);
    });
    measure("revision_diff_two_changed_fields", || {
        let diff = diff_revision(&frozen, &next_frozen, &fixture.budget).unwrap();
        assert!(diff.requires_new_revision);
    });
    measure("correction_classification", || {
        assert_eq!(
            classify_correction(&frozen, &correction, &fixture.budget)
                .unwrap()
                .disposition,
            CorrectionDispositionV1::AllowedSameRevision
        );
    });
    measure("lineage_compile_32_edges", || {
        let current = compile(&lineage.request, &lineage.context, &lineage.budget).unwrap();
        assert_eq!(
            current.frozen.fingerprint(),
            lineage_compilation.frozen.fingerprint()
        );
        black_box(current);
    });
    measure("admission_ready", || {
        let receipt = evaluate_admission(&frozen, &admission_request, &fixture.budget).unwrap();
        assert_eq!(receipt.status(), WorkOrderAdmissionStatusV1::Ready);
    });
    measure("handoff_ready", || {
        let current = materialize_handoff(&frozen, &admission, &fixture.budget).unwrap();
        assert_eq!(current.work_order_fingerprint(), frozen.fingerprint());
        black_box(current);
    });

    measure_boundaries();
    println!(
        "fixture_generator=synthetic-core-work-order-v1 source_count=32 packet_width=32 packet_depth=32 acceptance_criteria=32 evidence_nodes=32 lineage_edges=32 scope_rules=64 correction_paths=64 measured_samples={SAMPLE_COUNT} operations_per_sample={OPERATIONS_PER_SAMPLE}"
    );
}
