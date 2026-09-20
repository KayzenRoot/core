use core_workspace::{
    CacheDecisionReason, ComponentMask, ProofCache, ProofCacheKey, WorkspaceGeneration,
    WorkspaceId, WorkspaceResourceBudget,
};

#[test]
fn cache_is_disposable_derived_evidence() {
    let mut cache = ProofCache::new(&WorkspaceResourceBudget::default()).unwrap();
    let key = ProofCacheKey {
        schema_version: 1,
        workspace_id: WorkspaceId::new("w"),
        runtime_epoch: 1,
        generation: WorkspaceGeneration::new(1, 1),
        component_mask: ComponentMask::ALL,
        authority_generation: 1,
        policy_generation: 1,
        provider_version: "v1".into(),
        filesystem_semantics_fingerprint: "fsc".into(),
        proof_kind: "basis".into(),
    };
    cache.insert(key.clone(), "derived".to_owned()).unwrap();
    assert_eq!(cache.get(&key).reason, CacheDecisionReason::HitValidated);
    cache.invalidate_generation(WorkspaceGeneration::new(1, 2));
    assert_eq!(cache.get(&key).reason, CacheDecisionReason::MissNotFound);
}
