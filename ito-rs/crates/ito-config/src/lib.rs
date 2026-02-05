//! Ito configuration loading and normalization.
//!
//! `ito-config` owns the logic for reading configuration files (repo-local and
//! global), applying precedence rules, and exposing a single resolved view to
//! the rest of the workspace.
//!
//! This crate is intentionally small: it does not perform domain operations.
//! It only answers questions like "where is the Ito directory?" and "what are
//! the effective settings for this run?".

#![warn(missing_docs)]

/// Resolve the Ito working directory name and path.
pub mod ito_dir;

/// Console/UI behavior (color and interactivity) derived from CLI + env.
pub mod output;

mod config;
mod context;

/// Configuration loading and schema helpers.
pub use config::*;

/// Resolved context for a single invocation.
pub use context::ItoContext;

pub use config::{defaults, schema, types};
