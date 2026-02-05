//! Harness integrations for running AI-assisted workflows.
//!
//! A *harness* is an adapter around a specific agent runtime (e.g. OpenCode) that
//! can execute Ito workflows and return structured results.

#![warn(missing_docs)]

/// OpenCode harness implementation.
pub mod opencode;

/// No-op/stub harness used for testing.
pub mod stub;

/// Shared harness types.
pub mod types;

/// Run workflows via the OpenCode harness.
pub use opencode::OpencodeHarness;

/// Core harness trait + configuration and result types.
pub use types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
