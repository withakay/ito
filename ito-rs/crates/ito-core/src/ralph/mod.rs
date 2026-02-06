//! Ralph Wiggum iterative development loop.
//!
//! The Ralph loop repeatedly runs a harness with a prompt until a completion
//! promise is detected, persisting lightweight state between iterations.

/// Duration parsing/formatting helpers.
pub mod duration;

/// Prompt construction for Ralph iterations.
pub mod prompt;

/// Loop runner and iteration bookkeeping.
pub mod runner;

/// On-disk state for the Ralph loop.
pub mod state;

/// Completion validation for Ralph.
pub mod validation;

pub use duration::{format_duration, parse_duration};
pub use runner::{DEFAULT_ERROR_THRESHOLD, RalphOptions, run_ralph};
