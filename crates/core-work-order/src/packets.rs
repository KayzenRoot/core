use crate::budget::{ensure_count, M03ResourceBudgetV1};
use crate::contracts::*;
use crate::errors::{
    error, WorkOrderErrorCategoryV1 as Category, WorkOrderErrorCodeV1 as Code, WorkOrderErrorV1,
};
use crate::identity::WorkPacketId;
use crate::scope::is_scope_subset;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn build_dag(
    packets: Vec<WorkPacketSpecV1>,
    parent_scope: &ScopeEnvelopeV1,
    budget: &M03ResourceBudgetV1,
) -> Result<WorkPacketDagV1, WorkOrderErrorV1> {
    ensure_count(packets.len(), budget.max_packets)?;
    let mut by_id = BTreeMap::new();
    for packet in packets {
        if by_id.insert(packet.packet_id.clone(), packet).is_some() {
            return Err(error(Category::PacketGraph, Code::DuplicateId));
        }
    }
    for packet in by_id.values_mut() {
        crate::scope::validate_scope(&packet.scope, budget)?;
        packet.scope = crate::scope::intersect_scope(parent_scope, &packet.scope);
        crate::scope::validate_scope(&packet.scope, budget)?;
        if !is_scope_subset(&packet.scope, parent_scope) {
            return Err(error(Category::ScopeDelta, Code::PacketScopeWidening));
        }
    }
    let mut incoming = BTreeMap::<WorkPacketId, usize>::new();
    let mut outgoing = BTreeMap::<WorkPacketId, Vec<WorkPacketId>>::new();
    let mut edge_count = 0usize;
    for id in by_id.keys() {
        incoming.insert(id.clone(), 0);
    }
    for (id, packet) in &by_id {
        let mut unique = BTreeSet::new();
        for prerequisite in &packet.prerequisite_packet_ids {
            edge_count = edge_count
                .checked_add(1)
                .ok_or_else(|| error(Category::Resource, Code::GraphLimitExceeded))?;
            if prerequisite == id || !unique.insert(prerequisite) {
                return Err(error(Category::PacketGraph, Code::DuplicateId));
            }
            if !by_id.contains_key(prerequisite) {
                return Err(error(Category::PacketGraph, Code::DanglingReference));
            }
            *incoming.get_mut(id).expect("inserted above") += 1;
            outgoing
                .entry(prerequisite.clone())
                .or_default()
                .push(id.clone());
        }
    }
    ensure_count(edge_count, budget.max_packet_edges)?;
    let mut ready: BTreeSet<_> = incoming
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(id.clone()))
        .collect();
    let mut order = Vec::with_capacity(by_id.len());
    while let Some(next) = ready.pop_first() {
        order.push(next.clone());
        if let Some(children) = outgoing.get(&next) {
            for child in children {
                let count = incoming.get_mut(child).expect("known child");
                *count -= 1;
                if *count == 0 {
                    ready.insert(child.clone());
                }
            }
        }
    }
    if order.len() != by_id.len() {
        return Err(error(Category::PacketGraph, Code::Cycle));
    }
    Ok(WorkPacketDagV1 {
        packets: by_id.into_values().collect(),
        topological_order: order,
    })
}

pub(crate) fn validate_dag(
    dag: &WorkPacketDagV1,
    parent_scope: &ScopeEnvelopeV1,
    budget: &M03ResourceBudgetV1,
) -> Result<(), WorkOrderErrorV1> {
    let rebuilt = build_dag(dag.packets.clone(), parent_scope, budget)?;
    if rebuilt.topological_order != dag.topological_order || rebuilt.packets != dag.packets {
        return Err(error(Category::PacketGraph, Code::NondeterministicOrder));
    }
    Ok(())
}
