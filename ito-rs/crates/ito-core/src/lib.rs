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

/// Create new modules/changes and initial scaffolding.
pub mod create;

/// Distribution/build metadata helpers.
pub mod distribution;

/// Installers for project/home templates and harness assets.
pub mod installers;

/// List/query project entities (modules, changes, tasks).
pub mod list;

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
