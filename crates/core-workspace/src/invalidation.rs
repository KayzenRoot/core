//! Watcher-independent event hints and deterministic causal invalidation.

use crate::{BasisComponent, BvmProfile, ComponentMask, M02Error, WorkspaceResourceBudget};
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    PathCreated,
    PathRemoved,
    PathRenamed,
    PathContentChanged,
    PathMetadataChanged,
    GitHeadHint,
    GitIndexHint,
    GitRefsHint,
    GitMetadataHint,
    AuthorityRootHint,
    FilesystemSemanticsHint,
    AssociationHint,
    OverflowOrLoss,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventHint {
    pub workspace_id: crate::WorkspaceId,
    pub repository_id: Option<crate::RepositoryId>,
    pub path: Option<PathBuf>,
    pub kind: EventKind,
    pub provider: String,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationResult {
    pub dirty_components: ComponentMask,
    pub reason: String,
    pub broad: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CausalInvalidationGraph;

impl CausalInvalidationGraph {
    pub fn map(&self, hint: &EventHint) -> InvalidationResult {
        use BasisComponent as C;
        let (mask, broad, reason) = match hint.kind {
            EventKind::PathCreated
            | EventKind::PathRemoved
            | EventKind::PathRenamed
            | EventKind::PathContentChanged => (
                ComponentMask::only(C::TrackedWorktreeState)
                    .union(ComponentMask::only(C::UntrackedWorktreeState)),
                false,
                "path-content",
            ),
            EventKind::PathMetadataChanged => (
                ComponentMask::only(C::TrackedWorktreeState),
                false,
                "path-metadata",
            ),
            EventKind::GitHeadHint | EventKind::GitRefsHint => (
                ComponentMask::only(C::HeadState).union(ComponentMask::only(C::RepositoryGraph)),
                false,
                "git-head-or-ref",
            ),
            EventKind::GitIndexHint => (ComponentMask::only(C::IndexState), false, "git-index"),
            EventKind::GitMetadataHint => (
                ComponentMask::only(C::RepositoryGraph)
                    .union(ComponentMask::only(C::WorktreeIdentity)),
                false,
                "git-metadata",
            ),
            EventKind::AuthorityRootHint => (
                ComponentMask::only(C::Authority)
                    .union(ComponentMask::only(C::FilesystemSemantics))
                    .union(ComponentMask::only(C::SecurityPolicy)),
                false,
                "authority-root",
            ),
            EventKind::FilesystemSemanticsHint => (
                ComponentMask::only(C::FilesystemSemantics)
                    .union(ComponentMask::only(C::SecurityPolicy)),
                false,
                "filesystem-semantics",
            ),
            EventKind::AssociationHint => (
                ComponentMask::only(C::ProjectAssociation),
                false,
                "association",
            ),
            EventKind::OverflowOrLoss => (ComponentMask::ALL, true, "event-overflow-or-loss"),
        };
        InvalidationResult {
            dirty_components: mask,
            reason: reason.into(),
            broad,
        }
    }
}

#[derive(Debug)]
pub struct EventInvalidationSpine {
    queue: VecDeque<EventHint>,
    budget: u64,
    graph: CausalInvalidationGraph,
}

impl EventInvalidationSpine {
    pub fn new(budget: &WorkspaceResourceBudget) -> Result<Self, M02Error> {
        budget
            .validate()
            .map_err(|error| M02Error::InvalidInput(error.to_string()))?;
        Ok(Self {
            queue: VecDeque::new(),
            budget: budget.max_event_hint_backlog,
            graph: CausalInvalidationGraph,
        })
    }

    pub fn push(&mut self, hint: EventHint) -> InvalidationResult {
        if self.queue.len() as u64 >= self.budget {
            self.queue.clear();
            let overflow = EventHint {
                kind: EventKind::OverflowOrLoss,
                ..hint
            };
            let result = self.graph.map(&overflow);
            self.queue.push_back(overflow);
            return result;
        }
        let result = self.graph.map(&hint);
        self.queue.push_back(hint);
        result
    }

    pub fn drain(&mut self) -> Vec<EventHint> {
        self.queue.drain(..).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaWorkspaceSnapshot {
    pub base_fingerprint: String,
    pub changed_components: ComponentMask,
    pub resulting_fingerprint: String,
    pub evidence_references: Vec<String>,
}

impl DeltaWorkspaceSnapshot {
    pub fn proves_required_mask(&self, required: ComponentMask) -> bool {
        self.changed_components.0 & required.0 == 0
    }
}

pub fn required_mask(profile: BvmProfile) -> ComponentMask {
    profile.required_mask()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hint(kind: EventKind) -> EventHint {
        EventHint {
            workspace_id: crate::WorkspaceId::new("w"),
            repository_id: None,
            path: None,
            kind,
            provider: "test".into(),
            sequence: 1,
        }
    }

    #[test]
    fn overflow_broadens_invalidation() {
        let result = CausalInvalidationGraph.map(&hint(EventKind::OverflowOrLoss));
        assert!(result.broad);
        assert_eq!(result.dirty_components, ComponentMask::ALL);
    }

    #[test]
    fn zero_events_do_not_create_freshness() {
        let snapshot = DeltaWorkspaceSnapshot {
            base_fingerprint: "a".into(),
            changed_components: ComponentMask::only(BasisComponent::HeadState),
            resulting_fingerprint: "b".into(),
            evidence_references: Vec::new(),
        };
        assert!(!snapshot.proves_required_mask(BvmProfile::PlanWork.required_mask()));
    }
}
