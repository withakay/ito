//! Model registry and agent management for Ito
//!
//! This crate provides:
//! - Integration with models.dev for fetching AI model information
//! - Local caching of model data
//! - Agent file discovery and management across harnesses
//! - Model comparison utilities

mod agent;
mod cache;
mod client;
mod compare;
mod discovery;
mod registry;
mod rollback;
mod types;
mod update;

pub use agent::{AgentFile, AgentFrontmatter, AgentScope, AgentTier, Harness};
pub use compare::{ComparisonResult, ModelComparison, compare_models};
pub use discovery::{DiscoveryOptions, discover_agents, filter_by_harness, filter_ito_agents};
pub use registry::{CostTier, ModelRegistry, RegistryError};
pub use rollback::{
    RollbackError, RollbackResult, find_backup_files, restore_from_backup, rollback_all,
};
pub use types::{
    LoadOptions, Modality, Model, ModelCapability, ModelModalities, ModelSource, Provider,
    RegistryLoad,
};
pub use update::{
    BatchUpdateOptions, UpdateError, UpdateResult, update_agent_model, update_agents_batch,
};
