use crate::{M02_SCHEMA, M02_VERSION};
use core_config::WorkspaceResourceBudget;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum M02Error {
    #[error("unsupported M02 schema {schema} version {version}")]
    UnsupportedVersion { schema: String, version: u16 },
    #[error("invalid M02 input: {0}")]
    InvalidInput(String),
    #[error("invalid state transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: BindingState,
        to: BindingState,
    },
    #[error("workspace authority violation: {0}")]
    AuthorityViolation(String),
    #[error("filesystem semantics are unsupported or unsafe: {0}")]
    FilesystemSemantics(String),
    #[error("Git inspection failed: {0}")]
    GitInspection(String),
    #[error("resource budget exceeded: {0}")]
    ResourceBudgetExceeded(String),
    #[error("workspace handle is stale: {0}")]
    StaleHandle(String),
    #[error("a durable receipt cannot mint a live handle without fresh validation")]
    ReceiptCannotMintHandle,
    #[error("HIVE association conflict: {0}")]
    AssociationConflict(String),
    #[error("operation was cancelled")]
    Cancelled,
    #[error("I/O failure: {0}")]
    Io(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M02Envelope<T> {
    pub schema: String,
    pub version: u16,
    pub kind: String,
    pub payload: T,
}

impl<T> M02Envelope<T> {
    pub fn new(kind: impl Into<String>, payload: T) -> Self {
        Self {
            schema: M02_SCHEMA.to_owned(),
            version: M02_VERSION,
            kind: kind.into(),
            payload,
        }
    }

    pub fn validate(&self) -> Result<(), M02Error> {
        if self.schema != M02_SCHEMA || self.version != M02_VERSION {
            return Err(M02Error::UnsupportedVersion {
                schema: self.schema.clone(),
                version: self.version,
            });
        }
        Ok(())
    }
}

macro_rules! identity_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $name {
            pub value: String,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.value.fmt(f)
            }
        }
    };
}

identity_type!(ProjectBindingId);
identity_type!(WorkspaceId);
identity_type!(RepositoryId);
identity_type!(WorktreeId);
identity_type!(AuthorityRootId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WorkspaceGeneration {
    pub runtime_epoch: u64,
    pub number: u64,
}

impl WorkspaceGeneration {
    pub const fn new(runtime_epoch: u64, number: u64) -> Self {
        Self {
            runtime_epoch,
            number,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum BasisComponent {
    Identity = 0,
    Authority = 1,
    RepositoryGraph = 2,
    WorktreeIdentity = 3,
    HeadState = 4,
    IndexState = 5,
    TrackedWorktreeState = 6,
    UntrackedWorktreeState = 7,
    FilesystemSemantics = 8,
    ConfigGeneration = 9,
    SecurityPolicy = 10,
    ProjectAssociation = 11,
    SubmoduleState = 12,
    SparseCheckoutState = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ComponentMask(pub u64);

impl ComponentMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u64::MAX);

    pub const fn only(component: BasisComponent) -> Self {
        Self(1u64 << component as u8)
    }

    pub fn contains(self, component: BasisComponent) -> bool {
        self.0 & Self::only(component).0 != 0
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingState {
    Unbound,
    Discovering,
    Validating,
    Bound,
    Drifted,
    Revalidating,
    Blocked,
    Detaching,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssuranceRequirement {
    Standalone,
    HiveReconciled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UntrackedPolicy {
    ExcludedByPolicy,
    NamesOnly,
    ContentHashed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NestedRepositoryPolicy {
    Admit,
    Ignore,
    Conflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalObjectPolicy {
    Deny,
    AdmitWithProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BvmProfile {
    ReadMetadata,
    ReadSource,
    PlanWork,
    ExecuteToolReadonly,
    MutateSource,
    GitDelivery,
    HiveReconciledOperation,
}

impl BvmProfile {
    pub fn required_mask(self) -> ComponentMask {
        use BasisComponent as C;
        let base = |items: &[BasisComponent]| {
            items.iter().fold(ComponentMask::NONE, |mask, item| {
                mask.union(ComponentMask::only(*item))
            })
        };
        match self {
            Self::ReadMetadata => base(&[
                C::Identity,
                C::RepositoryGraph,
                C::ConfigGeneration,
                C::SecurityPolicy,
            ]),
            Self::ReadSource => base(&[
                C::Identity,
                C::Authority,
                C::FilesystemSemantics,
                C::TrackedWorktreeState,
                C::UntrackedWorktreeState,
                C::ConfigGeneration,
                C::SecurityPolicy,
            ]),
            Self::PlanWork => Self::ReadSource.required_mask().union(base(&[
                C::RepositoryGraph,
                C::HeadState,
                C::IndexState,
            ])),
            Self::ExecuteToolReadonly => Self::PlanWork
                .required_mask()
                .union(ComponentMask::only(C::Authority)),
            Self::MutateSource => Self::PlanWork
                .required_mask()
                .union(base(&[C::Authority, C::FilesystemSemantics])),
            Self::GitDelivery => base(&[
                C::Identity,
                C::Authority,
                C::RepositoryGraph,
                C::HeadState,
                C::IndexState,
                C::TrackedWorktreeState,
                C::UntrackedWorktreeState,
                C::ConfigGeneration,
                C::SecurityPolicy,
            ]),
            Self::HiveReconciledOperation => ComponentMask::only(C::ProjectAssociation),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAttachRequestV1 {
    pub schema_version: u16,
    pub workspace_root: PathBuf,
    pub expected_project_binding: Option<ProjectBindingId>,
    pub expected_repository: Option<RepositoryId>,
    pub expected_worktree: Option<WorktreeId>,
    pub assurance: AssuranceRequirement,
    pub untracked_policy: UntrackedPolicy,
    pub nested_repository_policy: NestedRepositoryPolicy,
    pub external_object_policy: ExternalObjectPolicy,
    pub resource_budget: WorkspaceResourceBudget,
    pub policy_generation: u64,
    pub security_generation: u64,
    pub runtime_epoch: u64,
    pub hive_project_reference: Option<String>,
}

impl WorkspaceAttachRequestV1 {
    pub fn new(workspace_root: impl Into<PathBuf>, runtime_epoch: u64) -> Self {
        Self {
            schema_version: M02_VERSION,
            workspace_root: workspace_root.into(),
            expected_project_binding: None,
            expected_repository: None,
            expected_worktree: None,
            assurance: AssuranceRequirement::Standalone,
            untracked_policy: UntrackedPolicy::ContentHashed,
            nested_repository_policy: NestedRepositoryPolicy::Ignore,
            external_object_policy: ExternalObjectPolicy::Deny,
            resource_budget: WorkspaceResourceBudget::default(),
            policy_generation: 1,
            security_generation: 1,
            runtime_epoch,
            hive_project_reference: None,
        }
    }

    pub fn validate(&self) -> Result<(), M02Error> {
        if self.schema_version != M02_VERSION {
            return Err(M02Error::UnsupportedVersion {
                schema: M02_SCHEMA.to_owned(),
                version: self.schema_version,
            });
        }
        self.resource_budget
            .validate()
            .map_err(|error| M02Error::InvalidInput(error.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRootV1 {
    pub id: AuthorityRootId,
    pub class: AuthorityClass,
    pub logical_root: PathBuf,
    pub canonical_existing_root: PathBuf,
    pub physical_identity: String,
    pub filesystem_semantics_fingerprint: String,
    pub provenance: String,
    pub policy_generation: u64,
    pub externality: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityClass {
    Source,
    GitMetadata,
    ExternalObject,
    InternalCoreTemp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilesystemSemanticsState {
    CaseSensitiveVerified,
    CaseInsensitiveVerified,
    CasePreservingUnknown,
    UnknownUnsafeForAliasDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemSemanticsCapsuleV1 {
    pub root_physical_identity: String,
    pub namespace_class: String,
    pub canonical_path_evidence: String,
    pub case_state: FilesystemSemanticsState,
    pub symlink_or_reparse_supported: bool,
    pub normalization_policy_version: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedPathReceiptV1 {
    pub requested: PathBuf,
    pub lexical_normalized: PathBuf,
    pub authority_root: AuthorityRootId,
    pub nearest_existing_ancestor: PathBuf,
    pub resolved_target: Option<PathBuf>,
    pub filesystem_semantics_fingerprint: String,
    pub use_time_revalidation_required: bool,
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathOperation {
    ReadMetadata,
    ReadSource,
    MutateSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathValidationRequestV1 {
    pub requested: PathBuf,
    pub operation: PathOperation,
    pub root: AuthorityRootId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryNodeV1 {
    pub id: String,
    pub kind: RepositoryNodeKind,
    pub path: PathBuf,
    pub semantic_fingerprint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RepositoryNodeKind {
    WorkspaceRoot,
    Repository,
    Worktree,
    SubmoduleDeclaration,
    SubmoduleMaterialization,
    NestedRepository,
    GitCommonDir,
    ExternalObjectStore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryEdgeV1 {
    pub kind: RepositoryEdgeKind,
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RepositoryEdgeKind {
    Contains,
    CheckoutOf,
    UsesCommonDir,
    DeclaresSubmodule,
    Materializes,
    NestedWithin,
    UsesObjectStore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryGraphV1 {
    pub schema_version: u16,
    pub nodes: Vec<RepositoryNodeV1>,
    pub edges: Vec<RepositoryEdgeV1>,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitEvidenceV1 {
    pub provider: String,
    pub provider_version: String,
    pub repository_id: RepositoryId,
    pub worktree_id: Option<WorktreeId>,
    pub is_bare: bool,
    pub object_format: String,
    pub head: String,
    pub symbolic_head: Option<String>,
    pub index_fingerprint: String,
    pub tracked_delta_fingerprint: String,
    pub untracked_fingerprint: String,
    pub submodule_fingerprint: String,
    pub sparse_checkout_fingerprint: String,
    pub common_dir: PathBuf,
    pub git_dir: PathBuf,
    pub redacted_remote_hints: Vec<String>,
    pub parsed_records: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectAssociationEvidence {
    pub provider_origin: String,
    pub provider_version: String,
    pub project_reference: Option<String>,
    pub asserted_workspace: Option<WorkspaceId>,
    pub asserted_repository: Option<RepositoryId>,
    pub generation: u64,
    pub fingerprint: String,
    pub status: AssociationStatus,
    pub provenance: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssociationStatus {
    Match,
    Stale,
    Conflict,
    Unavailable,
    NotRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconciliationStatus {
    Consistent,
    StandaloneVerified,
    Partial,
    Conflict,
    Insufficient,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationReceiptV1 {
    pub status: ReconciliationStatus,
    pub association: AssociationStatus,
    pub reason: String,
    pub local_fingerprint: String,
    pub hive_fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceBasisV1 {
    pub schema_version: u16,
    pub project_binding_id: ProjectBindingId,
    pub workspace_id: WorkspaceId,
    pub repository_id: Option<RepositoryId>,
    pub worktree_id: Option<WorktreeId>,
    pub authority_roots: Vec<AuthorityRootV1>,
    pub filesystem_semantics: FilesystemSemanticsCapsuleV1,
    pub repository_graph: RepositoryGraphV1,
    pub git: Option<GitEvidenceV1>,
    pub association: ProjectAssociationEvidence,
    pub reconciliation: ReconciliationReceiptV1,
    pub untracked_policy: UntrackedPolicy,
    pub config_generation: u64,
    pub security_generation: u64,
    pub component_fingerprints: BTreeMap<BasisComponent, String>,
}

impl WorkspaceBasisV1 {
    pub fn fingerprint(&self) -> Result<String, M02Error> {
        core_identity::fingerprint(self).map_err(|error| M02Error::InvalidInput(error.to_string()))
    }

    pub fn validate_schema(&self) -> Result<(), M02Error> {
        if self.schema_version != M02_VERSION {
            return Err(M02Error::UnsupportedVersion {
                schema: M02_SCHEMA.to_owned(),
                version: self.schema_version,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceBasisDiffV1 {
    pub from_generation: WorkspaceGeneration,
    pub to_generation: WorkspaceGeneration,
    pub changed_components: ComponentMask,
    pub before_fingerprint: String,
    pub after_fingerprint: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceBindingReceiptV1 {
    pub envelope: M02Envelope<ReceiptPayloadV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptPayloadV1 {
    pub project_binding_id: ProjectBindingId,
    pub workspace_id: WorkspaceId,
    pub repository_id: Option<RepositoryId>,
    pub worktree_id: Option<WorktreeId>,
    pub generation: WorkspaceGeneration,
    pub basis_fingerprint: String,
    pub authority_roots: Vec<AuthorityRootV1>,
    pub reconciliation: ReconciliationReceiptV1,
    pub requires_use_time_revalidation: bool,
}

impl WorkspaceBindingReceiptV1 {
    pub fn validate(&self) -> Result<(), M02Error> {
        self.envelope.validate()
    }

    pub fn to_handle_without_revalidation(&self) -> Result<WorkspaceHandleV1, M02Error> {
        Err(M02Error::ReceiptCannotMintHandle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceHandleV1 {
    pub runtime_epoch: u64,
    pub project_binding_id: ProjectBindingId,
    pub workspace_id: WorkspaceId,
    pub generation: WorkspaceGeneration,
    pub basis_fingerprint: String,
    pub valid_mask: ComponentMask,
}

impl WorkspaceHandleV1 {
    pub fn is_for_epoch(&self, epoch: u64) -> bool {
        self.runtime_epoch == epoch && self.generation.runtime_epoch == epoch
    }

    pub fn freshness_mask(&self) -> ComponentMask {
        self.valid_mask
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevalidationOutcome {
    StillValid,
    UpdatedCompatible,
    RebindRequired,
    BlockedConflict,
    InsufficientEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevalidationResult {
    pub outcome: RevalidationOutcome,
    pub diff: Option<WorkspaceBasisDiffV1>,
    pub handle: Option<WorkspaceHandleV1>,
}
