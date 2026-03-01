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

/// Backend API client factory, runtime, and coordination services.
pub mod backend_client;

/// Backend-backed change repository adapter.
pub mod backend_change_repository;

/// Backend-backed task repository adapter.
pub mod backend_task_repository;

/// Backend coordination use-cases (claim, release, allocate, sync).
pub mod backend_coordination;

/// Artifact synchronization (pull/push) for backend mode.
pub mod backend_sync;

/// Audit log infrastructure: writer, reader, reconciliation, worktree discovery.
pub mod audit;

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

/// Grep-style search over Ito change artifacts using ripgrep crates.
pub mod grep;

/// Client-side forwarding of local audit events to the backend.
pub mod event_forwarder;

/// Filesystem-backed backend project store implementation.
pub mod fs_project_store;

/// SQLite-backed backend project store proof-of-concept.
pub mod sqlite_project_store;

/// YAML front matter parsing, writing, and metadata utilities for artifacts.
pub mod front_matter;

/// Git synchronization helpers for coordination workflows.
pub mod git;

/// Resolve repository and worktree path roots.
pub mod repo_paths;

/// Infer Ito change/module target context for harness sessions.
pub mod harness_context;

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

/// Schema templates execution and planning.
pub mod templates;

// Re-export domain types for CLI and adapter convenience
pub use ito_domain::backend::{
    AllocateResult, ArchiveResult, ArtifactBundle, BackendArchiveClient, BackendChangeReader,
    BackendError, BackendEventIngestClient, BackendLeaseClient, BackendProjectStore,
    BackendSyncClient, BackendTaskReader, ClaimResult, EventBatch, EventIngestResult,
    LeaseConflict, PushResult, ReleaseResult, RevisionConflict,
};
pub use ito_domain::changes::{Change, ChangeRepository, ChangeSummary, ChangeTargetResolution};
pub use ito_domain::errors::DomainError;
pub use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
pub use ito_domain::tasks::{
    ProgressInfo, TaskItem, TaskRepository as DomainTaskRepository, TaskStatus, TasksFormat,
    TasksParseResult,
};

/// Harness integrations for running AI-assisted workflows.
pub mod harness;

/// Re-exported schema types from [`ito_domain::schemas`].
pub mod schemas {
    pub use ito_domain::schemas::*;
}

/// Re-exported domain modules
pub mod domain {
    /// Planning domain module
    pub use ito_domain::planning;
}

// Re-export utility functions for CLI convenience
pub use ito_common::id::parse_change_id;
pub use ito_common::id::parse_module_id;
pub use ito_common::match_::nearest_matches;

/// Re-exported path utilities from [`ito_common::paths`].
pub mod paths {
    pub use ito_common::paths::changes_dir;
    pub use ito_common::paths::spec_markdown_path;
    pub use ito_common::paths::specs_dir;
}
