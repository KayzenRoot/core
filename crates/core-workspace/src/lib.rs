//! M02 deterministic Project / Workspace Adapter.
//!
//! This crate is intentionally read-oriented: it establishes proof-carrying
//! workspace state, but it never mutates source, Git history, remotes or HIVE.

mod authority;
mod basis;
mod cache;
mod contracts;
mod hashing;
mod identity;
mod invalidation;
mod reconcile;
mod repository;
mod service;
mod state;

pub mod git;

pub use authority::*;
pub use basis::*;
pub use cache::*;
pub use contracts::*;
pub use core_config::WorkspaceResourceBudget;
pub use identity::*;
pub use invalidation::*;
pub use reconcile::*;
pub use repository::*;
pub use state::*;

pub const M02_SCHEMA: &str = "nexlabs.core.workspace";
pub const M02_VERSION: u16 = 1;
