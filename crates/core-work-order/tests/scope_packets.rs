mod common;

use common::{compiled, fixture};
use core_work_order::*;

#[test]
fn denied_rules_override_allow_and_empty_allow_sets_are_ambiguous() {
    let fixture = fixture();
    let mut denied = fixture.request.scope.clone();
    denied
        .denied_path_prefixes
        .push("crates/core-work-order/src".into());
    assert_eq!(
        evaluate_scope(
            &denied,
            "M03",
            "core-work-order",
            "crates/core-work-order/src/lib.rs",
            "source",
        ),
        ScopeDecisionV1::Denied
    );

    let mut ambiguous = fixture.request.scope;
    ambiguous.allowed_path_prefixes.clear();
    assert_eq!(
        evaluate_scope(
            &ambiguous,
            "M03",
            "core-work-order",
            "crates/core-work-order/src/lib.rs",
            "source",
        ),
        ScopeDecisionV1::Ambiguous
    );
}

#[test]
fn path_prefix_matching_respects_segment_boundaries() {
    let fixture = fixture();
    assert_eq!(
        evaluate_scope(
            &fixture.request.scope,
            "M03",
            "core-work-order",
            "crates/core-work-order-extra/src/lib.rs",
            "source",
        ),
        ScopeDecisionV1::Denied
    );
}

#[test]
fn packet_scope_is_intersected_with_parent_and_dag_order_is_stable() {
    let mut fixture = fixture();
    fixture.request.packets[0].scope.allowed_path_prefixes =
        vec!["crates/core-work-order/src".into()];
    fixture.request.packets[1].scope.allowed_path_prefixes =
        vec!["crates/core-work-order/tests".into()];
    let compilation = compiled(&fixture);
    let dag = &compilation.frozen.semantic().packet_dag;

    assert_eq!(dag.packets[0].packet_id.as_str(), "packet-1");
    assert_eq!(
        dag.packets[0].scope.allowed_path_prefixes,
        ["crates/core-work-order/src"]
    );
    assert_eq!(
        dag.packets[1].scope.allowed_path_prefixes,
        ["crates/core-work-order/tests"]
    );
    assert_eq!(
        dag.topological_order
            .iter()
            .map(WorkPacketId::as_str)
            .collect::<Vec<_>>(),
        ["packet-1", "packet-2"]
    );
}

#[test]
fn dangling_edges_cycles_and_widening_context_are_rejected() {
    let mut dangling = fixture();
    dangling.request.packets[0]
        .prerequisite_packet_ids
        .push(WorkPacketId::new("missing-packet").unwrap());
    assert!(compile(&dangling.request, &dangling.context, &dangling.budget).is_err());

    let mut cycle = fixture();
    let second_packet_id = cycle.request.packets[1].packet_id.clone();
    cycle.request.packets[0]
        .prerequisite_packet_ids
        .push(second_packet_id);
    assert!(compile(&cycle.request, &cycle.context, &cycle.budget).is_err());

    let mut widening = fixture();
    widening.request.packets[0].scope.allowed_path_prefixes = vec!["docs".into()];
    let result = compile(&widening.request, &widening.context, &widening.budget);
    assert!(result.is_err());
}
