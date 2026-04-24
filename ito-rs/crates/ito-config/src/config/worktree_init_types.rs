//! Worktree initialization configuration types.
//!
//! Extracted into a dedicated submodule to keep `types.rs` within the
//! 1200-line limit while keeping the public API unchanged via re-exports.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Worktree initialization configuration")]
/// Settings applied when a new worktree is created via `ito worktree ensure`.
///
/// Controls which files to copy from the main worktree and which commands to
/// run inside the new worktree after creation.
pub struct WorktreeInitConfig {
    #[serde(default)]
    #[schemars(
        default,
        description = "Glob patterns for files to copy from the main worktree into a new change worktree"
    )]
    /// Glob patterns for files to copy from the main worktree into a new
    /// change worktree (e.g. `[".env", ".envrc", "*.local.toml"]`).
    ///
    /// Patterns from a `.worktree-include` file at the repo root are merged
    /// with this list at runtime (union semantics).
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Command(s) to run inside the worktree after file copy")]
    /// Optional command or ordered list of commands to run inside the new
    /// worktree after included files have been copied and coordination
    /// symlinks have been created.
    ///
    /// Accepts either a single string (`"make init"`) or a list of strings
    /// (`["npm ci", "npm run build:types"]`).
    pub setup: Option<WorktreeSetupConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(description = "Setup command(s) — a single string or an ordered list of strings")]
/// A single shell command or an ordered list of shell commands to run during
/// worktree initialization.
pub enum WorktreeSetupConfig {
    /// A single command string (e.g. `"make init"`).
    Single(String),
    /// An ordered list of command strings (e.g. `["npm ci", "npm run build"]`).
    Multiple(Vec<String>),
}

impl WorktreeSetupConfig {
    /// Return the commands as a list, regardless of the variant.
    pub fn as_commands(&self) -> Vec<&str> {
        match self {
            WorktreeSetupConfig::Single(cmd) => vec![cmd.as_str()],
            WorktreeSetupConfig::Multiple(cmds) => cmds.iter().map(String::as_str).collect(),
        }
    }

    /// Return true when there are no meaningful commands to run.
    ///
    /// Empty strings are considered non-meaningful — `Multiple(vec![""])`
    /// returns `true`.
    pub fn is_empty(&self) -> bool {
        match self {
            WorktreeSetupConfig::Single(cmd) => cmd.trim().is_empty(),
            WorktreeSetupConfig::Multiple(cmds) => {
                cmds.is_empty() || cmds.iter().all(|c| c.trim().is_empty())
            }
        }
    }
}
