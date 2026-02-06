//! Core Ito application behavior.
//!
//! `ito-core` implements the main orchestration logic behind the CLI: reading and
//! writing Ito state on disk, running workflows, validating inputs, and
//! delegating to installers and harness integrations.
//!
//! This crate is intentionally "policy heavy" but "UI light": it defines the
//! core semantics of commands without owning the CLI argument surface.

#![warn(missing_docs)]

/// Archive completed changes and update specifications.
pub mod archive;

/// Filesystem-backed change repository implementation.
pub mod change_repository;

/// JSON configuration file CRUD operations.
pub mod config;

/// Create new modules/changes and initial scaffolding.
pub mod create;

/// Distribution/build metadata helpers.
pub mod distribution;

/// Core-layer error types and result alias.
pub mod errors;

mod error_bridge;

/// Process execution boundary and default runner.
pub mod process;

/// Installers for project/home templates and harness assets.
pub mod installers;

/// List/query project entities (modules, changes, tasks).
pub mod list;

/// Filesystem-backed module repository implementation.
pub mod module_repository;

/// Planning directory initialization (filesystem I/O).
pub mod planning_init;

/// Filesystem-backed task repository implementation.
pub mod task_repository;

/// Clock helpers (`now_time`, `now_date`).
pub mod time;

/// Task-focused orchestration use-cases.
pub mod tasks;

/// Ralph Wiggum loop support.
pub mod ralph;

/// Indexing helpers for repository contents.
pub mod repo_index;

/// Display and inspection commands.
pub mod show;

/// Statistics collection and computation for command usage.
pub mod stats;

/// Validation utilities for on-disk state.
pub mod validate;

/// Workflow execution and planning.
pub mod workflow;

/// Workflow template I/O: init, list, load workflow files from disk.
pub mod workflow_templates;

// Re-export domain types for CLI convenience
pub use ito_domain::changes::{ChangeRepository, ChangeTargetResolution};
pub use ito_domain::tasks::TaskRepository as DomainTaskRepository;

/// Re-exported workflow domain modules
pub mod domain {
    /// Planning domain module
    pub use ito_domain::planning;
    /// State domain module
    pub use ito_domain::state;
    /// Workflow domain module
    pub use ito_domain::workflow;
}
