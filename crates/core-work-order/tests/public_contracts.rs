mod common;

use common::{compiled, fixture, fp};
use core_work_order::*;

#[test]
fn request_envelope_round_trips_and_services_are_public() {
    let fixture = fixture();
    let envelope =
        WorkOrderEnvelope::new(WorkOrderContractKindV1::Request, fixture.request.clone());
    let encoded = serde_json::to_vec(&envelope).unwrap();
    let parsed = parse_request(&encoded, &fixture.budget).unwrap();

    assert_eq!(M03_SCHEMA, "nexlabs.core.work-order");
    assert_eq!(M03_VERSION, 1);
    assert_eq!(parsed, envelope);

    let compilation = compiled(&fixture);
    let frozen = compilation.frozen;
    assert_eq!(frozen.revision().get(), 1);
    validate_frozen(&frozen, &fixture.budget).unwrap();
    assert_eq!(
        canonical_semantic_bytes(&frozen).unwrap(),
        canonical_semantic_bytes(&frozen).unwrap()
    );
}

#[test]
fn parser_rejects_unknown_schema_version_kind_and_enum() {
    let fixture = fixture();
    let envelope =
        WorkOrderEnvelope::new(WorkOrderContractKindV1::Request, fixture.request.clone());
    let encoded = serde_json::to_value(&envelope).unwrap();

    for (field, value) in [
        ("schema", serde_json::Value::String("unknown.schema".into())),
        ("version", serde_json::Value::from(2)),
        ("kind", serde_json::Value::String("future_kind".into())),
    ] {
        let mut candidate = encoded.clone();
        candidate[field] = value;
        let bytes = serde_json::to_vec(&candidate).unwrap();
        assert!(parse_request(&bytes, &fixture.budget).is_err(), "{field}");
    }

    let mut candidate = encoded;
    candidate["payload"]["workspace"]["required_profile"] =
        serde_json::Value::String("future_profile".into());
    let bytes = serde_json::to_vec(&candidate).unwrap();
    assert!(parse_request(&bytes, &fixture.budget).is_err());
}

#[test]
fn typed_ids_and_revisions_are_bounded_and_positive() {
    assert!(WorkOrderId::new("valid-id/01").is_ok());
    assert!(WorkOrderId::new("").is_err());
    assert!(WorkOrderId::new("contains spaces").is_err());
    assert!(WorkOrderId::new("x".repeat(257)).is_err());
    assert!(WorkOrderRevision::new(0).is_err());
    assert_eq!(WorkOrderRevision::new(1).unwrap().next().unwrap().get(), 2);
    assert!(WorkOrderRevision::new(u32::MAX).unwrap().next().is_err());
    assert!(WorkOrderFingerprint::new("A".repeat(64)).is_err());
}

#[test]
fn logical_key_allocates_a_repeatable_work_order_id() {
    let mut first = fixture();
    first.request.requested_work_order_id = None;
    first.request.logical_key = Some(WorkOrderLogicalKeyV1 {
        project_namespace: "nexlabs".into(),
        module_namespace: "m03".into(),
        stable_key: "work-order-engine".into(),
    });
    let allocated_id =
        WorkOrderId::from_logical_key(first.request.logical_key.as_ref().unwrap()).unwrap();
    first.context.lineage.work_order_id = allocated_id;
    first.context.lineage.snapshot_fingerprint = fp(&(
        &first.context.lineage.work_order_id,
        first.context.lineage.current_revision,
        &first.context.lineage.current_fingerprint,
        &first.context.lineage.current_compilation_id,
        first.context.lineage.store_generation,
        first.context.lineage.superseded_revisions.clone(),
        first.context.lineage.edges.clone(),
        &first.context.lineage.provenance_fingerprint,
    ));
    let a = compile(&first.request, &first.context, &first.budget).unwrap();

    let mut second = fixture();
    second.request.requested_work_order_id = None;
    second.request.logical_key = first.request.logical_key.clone();
    second.context.lineage.work_order_id = a.frozen.work_order_id().clone();
    second.context.lineage.snapshot_fingerprint = fp(&(
        &second.context.lineage.work_order_id,
        second.context.lineage.current_revision,
        &second.context.lineage.current_fingerprint,
        &second.context.lineage.current_compilation_id,
        second.context.lineage.store_generation,
        second.context.lineage.superseded_revisions.clone(),
        second.context.lineage.edges.clone(),
        &second.context.lineage.provenance_fingerprint,
    ));
    let b = compile(&second.request, &second.context, &second.budget).unwrap();

    assert_eq!(a.frozen.work_order_id(), b.frozen.work_order_id());
    assert!(a.frozen.work_order_id().as_str().starts_with("woid.v1."));
}

#[test]
fn malformed_duplicate_packet_ids_fail_without_partial_compilation() {
    let mut fixture = fixture();
    fixture.request.packets[1].packet_id = fixture.request.packets[0].packet_id.clone();

    let result = compile(&fixture.request, &fixture.context, &fixture.budget);
    assert!(result.is_err());
}
