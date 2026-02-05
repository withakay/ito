//! Model registry and agent management for Ito
//!
//! This crate provides:
//! - Integration with models.dev for fetching AI model information
//! - Local caching of model data
//! - Agent file discovery and management across harnesses
//! - Model comparison utilities

#![warn(missing_docs)]

mod agent;
mod cache;
mod client;
mod compare;
mod discovery;
mod registry;
mod rollback;
mod types;
mod update;

/// Agent file parsing and metadata.
pub use agent::{AgentFile, AgentFrontmatter, AgentScope, AgentTier, Harness};

/// Compare two models and summarize differences.
pub use compare::{ComparisonResult, ModelComparison, compare_models};

/// Discover agent configuration files on disk.
pub use discovery::{DiscoveryOptions, discover_agents, filter_by_harness, filter_ito_agents};

/// Local model registry with caching and update support.
pub use registry::{CostTier, ModelRegistry, RegistryError};

/// Restore agent files from on-disk backups.
pub use rollback::{
    RollbackError, RollbackResult, find_backup_files, restore_from_backup, rollback_all,
};

/// Core model types and registry load configuration.
pub use types::{
    LoadOptions, Modality, Model, ModelCapability, ModelModalities, ModelSource, Provider,
    RegistryLoad,
};

/// Batch update helpers for rewriting agent model pins.
pub use update::{
    BatchUpdateOptions, UpdateError, UpdateResult, update_agent_model, update_agents_batch,
};
