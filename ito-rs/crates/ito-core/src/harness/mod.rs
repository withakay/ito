//! Harness integrations for running AI-assisted workflows.
//!
//! A *harness* is an adapter around a specific agent runtime (e.g. OpenCode) that
//! can execute Ito workflows and return structured results.

/// Claude Code harness implementation.
pub mod claude_code;

/// OpenAI Codex harness implementation.
pub mod codex;

/// GitHub Copilot harness implementation.
pub mod github_copilot;

/// OpenCode harness implementation.
pub mod opencode;

/// No-op/stub harness used for testing.
pub mod stub;

mod streaming_cli;

/// Shared harness types.
pub mod types;

/// Run workflows via the Claude Code harness.
pub use claude_code::ClaudeCodeHarness;

/// Run workflows via the OpenAI Codex harness.
pub use codex::CodexHarness;

/// Run workflows via the GitHub Copilot harness.
pub use github_copilot::GitHubCopilotHarness;

/// Run workflows via the OpenCode harness.
pub use opencode::OpencodeHarness;

/// Core harness trait + configuration and result types.
pub use types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
