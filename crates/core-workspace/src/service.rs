//! M02 attach/revalidate/detach service facade.

use crate::basis::{association_for_request, build_basis, diff, required_components_fresh};
use crate::cache::{CacheDecisionReason, ProofCache, ProofCacheKey};
use crate::reconcile::{NoopAssociationProvider, ProjectAssociationProvider};
use crate::repository::{empty_graph_with_policy, inspect_repository_with_conveyor};
use crate::{
    source_authority_root, workspace_identity, BindingState, BindingStateMachine, BvmProfile,
    ComponentMask, DeltaWorkspaceSnapshot, EventHint, EventInvalidationSpine, HashConveyor,
    M02Error, PathValidationRequestV1, RevalidationOutcome, RevalidationResult,
    ValidatedPathReceiptV1, WorkspaceAttachRequestV1, WorkspaceBasisV1, WorkspaceBindingReceiptV1,
    WorkspaceGeneration, WorkspaceHandleV1, WorkspaceResourceBudget,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AttachResult {
    pub receipt: WorkspaceBindingReceiptV1,
    pub handle: WorkspaceHandleV1,
    pub basis: WorkspaceBasisV1,
}

#[derive(Debug)]
struct CurrentBinding {
    request: WorkspaceAttachRequestV1,
    basis: WorkspaceBasisV1,
    handle: WorkspaceHandleV1,
}

pub struct WorkspaceService {
    runtime_epoch: u64,
    next_generation: u64,
    state: BindingStateMachine,
    current: Option<CurrentBinding>,
    association: Arc<dyn ProjectAssociationProvider>,
    budget: WorkspaceResourceBudget,
    invalidation: EventInvalidationSpine,
    dirty_components: ComponentMask,
    broad_invalidation: bool,
    proof_cache: ProofCache<WorkspaceBasisV1>,
    hash_conveyor: HashConveyor,
}

impl std::fmt::Debug for WorkspaceService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorkspaceService")
            .field("runtime_epoch", &self.runtime_epoch)
            .field("next_generation", &self.next_generation)
            .field("state", &self.state.state())
            .finish_non_exhaustive()
    }
}

impl WorkspaceService {
    pub fn new(runtime_epoch: u64) -> Self {
        Self::with_association_provider(runtime_epoch, Arc::new(NoopAssociationProvider))
    }

    pub fn with_association_provider(
        runtime_epoch: u64,
        association: Arc<dyn ProjectAssociationProvider>,
    ) -> Self {
        Self {
            runtime_epoch,
            next_generation: 0,
            state: BindingStateMachine::default(),
            current: None,
            association,
            budget: WorkspaceResourceBudget::default(),
            invalidation: EventInvalidationSpine::new(&WorkspaceResourceBudget::default())
                .expect("finalized M02 budget is valid"),
            dirty_components: ComponentMask::NONE,
            broad_invalidation: false,
            proof_cache: ProofCache::new(&WorkspaceResourceBudget::default())
                .expect("finalized M02 budget is valid"),
            hash_conveyor: HashConveyor::default(),
        }
    }

    pub fn state(&self) -> BindingState {
        self.state.state()
    }

    pub fn set_budget(&mut self, budget: WorkspaceResourceBudget) -> Result<(), M02Error> {
        budget
            .validate()
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        self.budget = budget;
        self.invalidation = EventInvalidationSpine::new(&self.budget)
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        self.proof_cache = ProofCache::new(&self.budget)?;
        self.dirty_components = ComponentMask::ALL;
        self.broad_invalidation = true;
        Ok(())
    }

    pub fn push_event_hint(&mut self, hint: EventHint) -> crate::InvalidationResult {
        let result = self.invalidation.push(hint);
        self.dirty_components = self.dirty_components.union(result.dirty_components);
        self.broad_invalidation |= result.broad;
        result
    }

    pub fn dirty_components(&self) -> ComponentMask {
        self.dirty_components
    }

    pub fn hash_operation_count(&self) -> u64 {
        self.hash_conveyor.hash_operation_count()
    }

    fn collect_basis(
        &self,
        request: &WorkspaceAttachRequestV1,
    ) -> Result<WorkspaceBasisV1, M02Error> {
        let authority = source_authority_root(&request.workspace_root, request.policy_generation)?;
        let (workspace_id, _) = workspace_identity(&request.workspace_root)?;
        let inspected = inspect_repository_with_conveyor(
            request.workspace_root.clone(),
            request.untracked_policy,
            request.resource_budget.clone(),
            Some(&self.hash_conveyor),
        )?;
        let (git, graph) = match inspected {
            Some((git, graph)) => (Some(git), graph),
            None => (
                None,
                empty_graph_with_policy(
                    &request.workspace_root,
                    request.untracked_policy,
                    &request.resource_budget,
                )?,
            ),
        };
        if let Some(expected) = &request.expected_repository {
            if git.as_ref().map(|value| &value.repository_id) != Some(expected) {
                return Err(M02Error::InvalidInput(
                    "expected RepositoryId does not match local Git".into(),
                ));
            }
        }
        if let Some(expected) = &request.expected_worktree {
            if git.as_ref().and_then(|value| value.worktree_id.as_ref()) != Some(expected) {
                return Err(M02Error::InvalidInput(
                    "expected WorktreeId does not match local Git".into(),
                ));
            }
        }
        let repository_id = git.as_ref().map(|value| value.repository_id.clone());
        let local_fingerprint = core_identity::fingerprint(&(
            &workspace_id,
            &repository_id,
            &graph.fingerprint,
            request.policy_generation,
            request.security_generation,
        ))
        .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        let association = association_for_request(
            &*self.association,
            workspace_id,
            repository_id,
            request,
            local_fingerprint,
        )?;
        build_basis(request, authority, graph, git, association)
    }

    fn proof_cache_key(
        &self,
        basis: &WorkspaceBasisV1,
        generation: WorkspaceGeneration,
    ) -> Result<ProofCacheKey, M02Error> {
        let authority = basis
            .authority_roots
            .first()
            .ok_or_else(|| M02Error::InvalidInput("basis has no authority root".into()))?;
        let provider_version = basis
            .git
            .as_ref()
            .map(|git| format!("{}:{}", git.provider, git.provider_version))
            .unwrap_or_else(|| {
                format!(
                    "{}:{}",
                    basis.association.provider_origin, basis.association.provider_version
                )
            });
        let semantic_proof_identity = core_identity::fingerprint(&(
            &basis.component_fingerprints,
            &basis.untracked_policy,
            &basis.filesystem_semantics,
        ))
        .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        let content_hash = basis.fingerprint()?;
        Ok(ProofCacheKey {
            schema_version: crate::M02_VERSION,
            workspace_id: basis.workspace_id.clone(),
            runtime_epoch: generation.runtime_epoch,
            generation,
            component_mask: ComponentMask::ALL,
            authority_generation: authority.policy_generation,
            policy_generation: basis.config_generation,
            security_generation: basis.security_generation,
            provider_version,
            filesystem_semantics_fingerprint: authority.filesystem_semantics_fingerprint.clone(),
            semantic_proof_identity,
            content_hash,
            proof_kind: "workspace-basis-v1".into(),
        })
    }

    fn issue_handle(
        &self,
        basis: &WorkspaceBasisV1,
        generation: WorkspaceGeneration,
    ) -> Result<WorkspaceHandleV1, M02Error> {
        Ok(WorkspaceHandleV1 {
            runtime_epoch: self.runtime_epoch,
            project_binding_id: basis.project_binding_id.clone(),
            workspace_id: basis.workspace_id.clone(),
            generation,
            basis_fingerprint: basis.fingerprint()?,
            valid_mask: ComponentMask::ALL,
        })
    }

    fn receipt(
        &self,
        basis: &WorkspaceBasisV1,
        generation: WorkspaceGeneration,
    ) -> Result<WorkspaceBindingReceiptV1, M02Error> {
        Ok(WorkspaceBindingReceiptV1 {
            envelope: crate::M02Envelope::new(
                "workspace-binding-receipt",
                crate::ReceiptPayloadV1 {
                    project_binding_id: basis.project_binding_id.clone(),
                    workspace_id: basis.workspace_id.clone(),
                    repository_id: basis.repository_id.clone(),
                    worktree_id: basis.worktree_id.clone(),
                    generation,
                    basis_fingerprint: basis.fingerprint()?,
                    authority_roots: basis.authority_roots.clone(),
                    reconciliation: basis.reconciliation.clone(),
                    requires_use_time_revalidation: true,
                },
            ),
        })
    }

    pub fn attach(
        &mut self,
        mut request: WorkspaceAttachRequestV1,
    ) -> Result<AttachResult, M02Error> {
        request.validate()?;
        if request.runtime_epoch != self.runtime_epoch {
            return Err(M02Error::StaleHandle(
                "attach runtime epoch mismatch".into(),
            ));
        }
        if !request.workspace_root.is_dir() {
            return Err(M02Error::InvalidInput(
                "workspace root must be an existing directory".into(),
            ));
        }
        if matches!(
            self.state.state(),
            BindingState::Blocked | BindingState::Unbound
        ) {
            self.state.transition(BindingState::Discovering)?;
        }
        self.state.transition(BindingState::Validating)?;
        if request.resource_budget == WorkspaceResourceBudget::default() {
            request.resource_budget = self.budget.clone();
        }
        let basis = self.collect_basis(&request)?;
        self.next_generation = self.next_generation.saturating_add(1);
        let generation = WorkspaceGeneration::new(self.runtime_epoch, self.next_generation);
        let handle = self.issue_handle(&basis, generation)?;
        let receipt = self.receipt(&basis, generation)?;
        self.state.transition(BindingState::Bound)?;
        self.current = Some(CurrentBinding {
            request,
            basis: basis.clone(),
            handle: handle.clone(),
        });
        self.proof_cache
            .insert(self.proof_cache_key(&basis, generation)?, basis.clone())?;
        self.invalidation.drain();
        self.dirty_components = ComponentMask::NONE;
        self.broad_invalidation = false;
        Ok(AttachResult {
            receipt,
            handle,
            basis,
        })
    }

    pub fn ensure_fresh(
        &self,
        handle: &WorkspaceHandleV1,
        profile: BvmProfile,
    ) -> Result<(), M02Error> {
        if !handle.is_for_epoch(self.runtime_epoch) {
            return Err(M02Error::StaleHandle("runtime epoch mismatch".into()));
        }
        let current = self
            .current
            .as_ref()
            .ok_or_else(|| M02Error::StaleHandle("no current binding".into()))?;
        if current.handle.basis_fingerprint != handle.basis_fingerprint
            || current.handle.generation != handle.generation
        {
            return Err(M02Error::StaleHandle(
                "handle is not the current generation".into(),
            ));
        }
        required_components_fresh(
            &current.basis,
            profile.required_mask(),
            self.dirty_components,
        )
    }

    pub fn validate_path(
        &self,
        handle: &WorkspaceHandleV1,
        request: &PathValidationRequestV1,
    ) -> Result<ValidatedPathReceiptV1, M02Error> {
        self.ensure_fresh(handle, BvmProfile::ReadSource)?;
        let current = self
            .current
            .as_ref()
            .ok_or_else(|| M02Error::StaleHandle("no current binding".into()))?;
        let root = current
            .basis
            .authority_roots
            .iter()
            .find(|root| root.id == request.root)
            .ok_or_else(|| M02Error::AuthorityViolation("unknown authority root".into()))?;
        crate::validate_path(root, request)
    }

    pub fn revalidate(
        &mut self,
        handle: &WorkspaceHandleV1,
    ) -> Result<RevalidationResult, M02Error> {
        self.revalidate_for(handle, BvmProfile::ReadMetadata)
    }

    pub fn revalidate_for(
        &mut self,
        handle: &WorkspaceHandleV1,
        profile: BvmProfile,
    ) -> Result<RevalidationResult, M02Error> {
        self.validate_handle(handle)?;
        let hints = self.invalidation.drain();
        let required = profile.required_mask();
        if !hints.is_empty()
            && !self.broad_invalidation
            && self.dirty_components.0 & required.0 == 0
        {
            let current = self
                .current
                .as_ref()
                .ok_or_else(|| M02Error::StaleHandle("no current binding".into()))?;
            let key = self.proof_cache_key(&current.basis, current.handle.generation)?;
            if self.proof_cache.get(&key).reason == CacheDecisionReason::HitValidated {
                let fingerprint = current.basis.fingerprint()?;
                let snapshot = DeltaWorkspaceSnapshot {
                    base_fingerprint: fingerprint.clone(),
                    changed_components: self.dirty_components,
                    resulting_fingerprint: fingerprint,
                    evidence_references: vec!["pec-l1:event-provenance".into()],
                };
                if snapshot.proves_required_mask(required) {
                    // Preserve dirty components that were not required by this
                    // profile so a later stronger profile cannot consume a
                    // metadata-only proof as source/content freshness.
                    return Ok(RevalidationResult {
                        outcome: RevalidationOutcome::StillValid,
                        diff: None,
                        handle: Some(handle.clone()),
                    });
                }
            }
        }
        let current = self
            .current
            .take()
            .ok_or_else(|| M02Error::StaleHandle("no current binding".into()))?;
        self.state.transition(BindingState::Drifted)?;
        self.state.transition(BindingState::Revalidating)?;
        let basis = match self.collect_basis(&current.request) {
            Ok(basis) => basis,
            Err(error) => {
                self.state.transition(BindingState::Blocked)?;
                self.current = Some(current);
                return Err(error);
            }
        };
        let old_fingerprint = current.basis.fingerprint()?;
        let new_fingerprint = basis.fingerprint()?;
        if old_fingerprint == new_fingerprint {
            let generation = current.handle.generation;
            let cache_key = self.proof_cache_key(&basis, generation)?;
            self.state.transition(BindingState::Bound)?;
            self.current = Some(current);
            self.proof_cache.insert(cache_key, basis)?;
            self.dirty_components = ComponentMask::NONE;
            self.broad_invalidation = false;
            return Ok(RevalidationResult {
                outcome: RevalidationOutcome::StillValid,
                diff: None,
                handle: Some(handle.clone()),
            });
        }
        if basis.workspace_id != current.basis.workspace_id
            || basis.repository_id != current.basis.repository_id
        {
            self.state.transition(BindingState::Blocked)?;
            self.current = Some(current);
            return Ok(RevalidationResult {
                outcome: RevalidationOutcome::RebindRequired,
                diff: None,
                handle: None,
            });
        }
        self.next_generation = self.next_generation.saturating_add(1);
        let generation = WorkspaceGeneration::new(self.runtime_epoch, self.next_generation);
        let new_handle = self.issue_handle(&basis, generation)?;
        let delta = diff(
            &current.basis,
            &basis,
            current.handle.generation,
            generation,
        )?;
        self.state.transition(BindingState::Bound)?;
        let cache_key = self.proof_cache_key(&basis, generation)?;
        self.current = Some(CurrentBinding {
            request: current.request,
            basis: basis.clone(),
            handle: new_handle.clone(),
        });
        self.proof_cache.insert(cache_key, basis)?;
        self.dirty_components = ComponentMask::NONE;
        self.broad_invalidation = false;
        Ok(RevalidationResult {
            outcome: RevalidationOutcome::UpdatedCompatible,
            diff: Some(delta),
            handle: Some(new_handle),
        })
    }

    pub fn detach(&mut self, handle: &WorkspaceHandleV1) -> Result<(), M02Error> {
        self.ensure_fresh(handle, BvmProfile::ReadMetadata)?;
        self.state.transition(BindingState::Detaching)?;
        self.current = None;
        self.state.transition(BindingState::Unbound)?;
        Ok(())
    }

    fn validate_handle(&self, handle: &WorkspaceHandleV1) -> Result<(), M02Error> {
        if !handle.is_for_epoch(self.runtime_epoch) {
            return Err(M02Error::StaleHandle("runtime epoch mismatch".into()));
        }
        let current = self
            .current
            .as_ref()
            .ok_or_else(|| M02Error::StaleHandle("no current binding".into()))?;
        if current.handle.basis_fingerprint != handle.basis_fingerprint
            || current.handle.generation != handle.generation
        {
            return Err(M02Error::StaleHandle(
                "handle is not the current generation".into(),
            ));
        }
        Ok(())
    }
}
