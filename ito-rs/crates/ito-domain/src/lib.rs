//! Domain models and repositories for Ito.
//!
//! `ito-domain` defines the stable "shape" of Ito data (modules, changes, tasks,
//! plans) and provides repository abstractions for reading and writing the
//! on-disk representation.
//!
//! Consumers should prefer repository APIs over direct file I/O so storage
//! formats can evolve without rewriting higher-level logic.

#![warn(missing_docs)]

/// Change definitions and computed status.
pub mod changes;

/// Domain-level error types.
pub mod errors;

/// Project discovery and filesystem traversal.
pub mod discovery;

/// Module definitions and dependency graph helpers.
pub mod modules;

/// Planning primitives and execution plan construction.
pub mod planning;

/// Persisted state for workflows and runs.
pub mod state;

/// Task models and task list parsing.
pub mod tasks;

/// Workflow models and execution helpers.
pub mod workflow;

/// Serde schema types for workflow definitions, plans, and execution state.
///
/// Re-exported from the former `ito-schemas` crate.
pub mod schemas;
