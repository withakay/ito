//! Serde models for Ito's on-disk formats.
//!
//! This crate exists to keep YAML/JSON schema structs shared across crates
//! (e.g. `ito-domain`, `ito-core`, `ito-cli`).

#![warn(missing_docs)]

/// Workflow definition schema (`workflow.yaml`).
pub mod workflow;

/// Execution plan schema derived from a workflow definition.
pub mod workflow_plan;

/// Workflow execution state schema.
pub mod workflow_state;

/// Re-export of workflow definition schema types.
pub use workflow::*;
/// Re-export of execution plan schema types.
pub use workflow_plan::*;
/// Re-export of workflow state schema types.
pub use workflow_state::*;
