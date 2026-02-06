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

/// Create new modules/changes and initial scaffolding.
pub mod create;

/// Distribution/build metadata helpers.
pub mod distribution;

mod error_bridge;

/// Process execution boundary and default runner.
pub mod process;

/// Installers for project/home templates and harness assets.
pub mod installers;

/// List/query project entities (modules, changes, tasks).
pub mod list;

/// Filesystem-backed module repository implementation.
pub mod module_repository;

/// Filesystem-backed task repository implementation.
pub mod task_repository;

/// Task-focused orchestration use-cases.
pub mod tasks;

/// Ralph Wiggum loop support.
pub mod ralph;

/// Indexing helpers for repository contents.
pub mod repo_index;

/// Display and inspection commands.
pub mod show;

/// Validation utilities for on-disk state.
pub mod validate;

/// Workflow execution and planning.
pub mod workflow;
