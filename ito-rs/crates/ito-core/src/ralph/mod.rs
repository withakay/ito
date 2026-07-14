//! Ralph Wiggum iterative development loop.
//!
//! The Ralph loop repeatedly runs a harness with a prompt until a completion
//! promise is detected, persisting lightweight state between iterations.

/// Duration parsing/formatting helpers.
pub mod duration;

/// Prompt construction for Ralph iterations.
pub mod prompt;

mod readiness;

/// Loop runner and iteration bookkeeping.
pub mod runner;

/// On-disk state for the Ralph loop.
pub mod state;

/// External task source resolution.
pub mod task_sources;

/// Completion validation for Ralph.
pub mod validation;

pub use duration::{format_duration, parse_duration};
pub use readiness::{RalphReadinessGate, ResolvedCwd, run_ralph};
pub use runner::{
    DEFAULT_ERROR_THRESHOLD, RalphOptions, WorktreeConfig, resolve_effective_cwd,
    run_ralph_with_readiness,
};
pub use task_sources::{
    RalphTaskSource, resolve_github_task_sources, resolve_markdown_task_sources,
    resolve_yaml_task_sources,
};
