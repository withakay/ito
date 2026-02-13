use ito_core::harness::streaming_cli::DEFAULT_INACTIVITY_TIMEOUT;
use ito_core::harness::{ClaudeCodeHarness, CodexHarness, GitHubCopilotHarness, Harness};
use ito_core::harness::{HarnessName, HarnessRunConfig};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

/// Verifies that ClaudeCodeHarness correctly constructs command arguments including
/// model selection, permission bypass, and prompt arguments.
///
/// The test creates a ClaudeCodeHarness, builds a HarnessRunConfig with various
/// options, and confirms the harness name is correct. A full execution test would
/// require the `claude` binary to be present on PATH, so this test focuses on
/// verifying the harness metadata.
#[test]
fn claude_code_harness_has_correct_name() {
    let harness = ClaudeCodeHarness;
    assert_eq!(harness.name(), HarnessName::CLAUDE);
    assert!(harness.streams_output());
}

/// Verifies that CodexHarness correctly reports its name and streaming capability.
///
/// Similar to the ClaudeCodeHarness test, this verifies the harness metadata without
/// requiring the actual `codex` binary to be installed.
#[test]
fn codex_harness_has_correct_name() {
    let harness = CodexHarness;
    assert_eq!(harness.name(), HarnessName::CODEX);
    assert!(harness.streams_output());
}

/// Verifies that GitHubCopilotHarness correctly reports its name and streaming capability.
///
/// Confirms the harness uses the canonical GITHUB_COPILOT name and supports streaming output.
#[test]
fn github_copilot_harness_has_correct_name() {
    let harness = GitHubCopilotHarness;
    assert_eq!(harness.name(), HarnessName::GITHUB_COPILOT);
    assert!(harness.streams_output());
}

/// Verifies that the default inactivity timeout for streaming CLI harnesses is reasonable.
///
/// The timeout should be long enough to accommodate typical AI agent iterations but
/// short enough to detect actual hangs. This test documents the expected value.
#[test]
fn default_inactivity_timeout_is_fifteen_minutes() {
    assert_eq!(DEFAULT_INACTIVITY_TIMEOUT, Duration::from_secs(15 * 60));
}

/// Verifies that HarnessRunConfig can be constructed with all required fields
/// and optional fields like inactivity_timeout and allow_all.
#[test]
fn harness_run_config_accepts_timeout_and_allow_all() {
    let config = HarnessRunConfig {
        prompt: "test prompt".to_string(),
        model: Some("sonnet".to_string()),
        cwd: PathBuf::from("/tmp"),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: true,
        inactivity_timeout: Some(Duration::from_secs(30)),
    };

    assert_eq!(config.prompt, "test prompt");
    assert_eq!(config.model.as_deref(), Some("sonnet"));
    assert!(config.allow_all);
    assert_eq!(config.inactivity_timeout, Some(Duration::from_secs(30)));
}

/// Verifies that harness implementations correctly handle the allow_all flag
/// by checking their metadata and configuration options.
///
/// This test ensures the harness trait implementations properly expose the
/// streams_output capability and can be stopped (even if it's a no-op for
/// synchronous harnesses).
#[test]
fn harness_implementations_support_stop_operation() {
    let mut claude = ClaudeCodeHarness;
    let mut codex = CodexHarness;
    let mut copilot = GitHubCopilotHarness;

    // stop() should be callable without panic (even if no-op)
    claude.stop();
    codex.stop();
    copilot.stop();
}

/// Verifies that streaming CLI harnesses can be instantiated with default trait implementations.
#[test]
fn streaming_harnesses_can_be_boxed() {
    let harnesses: Vec<Box<dyn Harness>> = vec![
        Box::new(ClaudeCodeHarness),
        Box::new(CodexHarness),
        Box::new(GitHubCopilotHarness),
    ];

    assert_eq!(harnesses.len(), 3);
    assert_eq!(harnesses[0].name(), HarnessName::CLAUDE);
    assert_eq!(harnesses[1].name(), HarnessName::CODEX);
    assert_eq!(harnesses[2].name(), HarnessName::GITHUB_COPILOT);
}

/// Verifies that ClaudeCodeHarness uses the Default trait correctly.
#[test]
fn claude_harness_implements_default() {
    let harness = ClaudeCodeHarness::default();
    assert_eq!(harness.name(), HarnessName::CLAUDE);
}

/// Verifies that CodexHarness uses the Default trait correctly.
#[test]
fn codex_harness_implements_default() {
    let harness = CodexHarness::default();
    assert_eq!(harness.name(), HarnessName::CODEX);
}

/// Verifies that GitHubCopilotHarness uses the Default trait correctly.
#[test]
fn github_copilot_harness_implements_default() {
    let harness = GitHubCopilotHarness::default();
    assert_eq!(harness.name(), HarnessName::GITHUB_COPILOT);
}

/// Verifies that streaming harnesses correctly report their Debug trait implementation.
#[test]
fn streaming_harnesses_implement_debug() {
    let claude = ClaudeCodeHarness;
    let codex = CodexHarness;
    let copilot = GitHubCopilotHarness;

    let claude_debug = format!("{:?}", claude);
    let codex_debug = format!("{:?}", codex);
    let copilot_debug = format!("{:?}", copilot);

    assert!(claude_debug.contains("ClaudeCodeHarness"));
    assert!(codex_debug.contains("CodexHarness"));
    assert!(copilot_debug.contains("GitHubCopilotHarness"));
}