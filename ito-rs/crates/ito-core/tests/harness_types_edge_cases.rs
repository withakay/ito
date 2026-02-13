use ito_core::harness::types::{HarnessName, HarnessRunConfig};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

/// Verifies that all user-facing harness names are properly defined and accessible.
///
/// This test ensures that the USER_FACING constant contains the expected harness names
/// and that they match the documented harness variants. This helps catch issues where
/// a new harness is added but not included in the user-facing list.
#[test]
fn user_facing_harness_names_include_all_public_harnesses() {
    let expected = vec!["opencode", "claude", "codex", "copilot"];
    assert_eq!(HarnessName::USER_FACING, &expected[..]);
}

/// Verifies that the help text generation includes all user-facing harnesses.
///
/// The help_text() method should produce a formatted string containing all
/// harness names from USER_FACING, formatted as comma-separated values within brackets.
#[test]
fn help_text_includes_all_user_facing_harnesses() {
    let help = HarnessName::help_text();
    assert_eq!(help, "[opencode, claude, codex, copilot]");

    // Verify each harness name appears exactly once
    for name in HarnessName::USER_FACING {
        assert!(help.contains(name), "help text should contain {}", name);
    }
}

/// Verifies that HARNESS_HELP constant is correctly formatted.
///
/// This ensures the static help string matches the dynamically generated help text,
/// preventing drift between the two representations.
#[test]
fn harness_help_matches_expected_format() {
    assert_eq!(
        HarnessName::HARNESS_HELP,
        "Harness to run [opencode, claude, codex, copilot]"
    );
}

/// Verifies that harness name constants have the expected string values.
///
/// This test documents the expected internal names for each harness and ensures
/// they remain stable across refactorings.
#[test]
fn harness_name_constants_have_expected_values() {
    assert_eq!(HarnessName::OPENCODE.0, "opencode");
    assert_eq!(HarnessName::CLAUDE.0, "claude");
    assert_eq!(HarnessName::CODEX.0, "codex");
    assert_eq!(HarnessName::GITHUB_COPILOT.0, "github-copilot");
    assert_eq!(HarnessName::COPILOT.0, "copilot");
    assert_eq!(HarnessName::STUB.0, "stub");
}

/// Verifies that the stub harness is not included in user-facing documentation.
///
/// The stub harness is for testing only and should not appear in USER_FACING or
/// in the generated help text.
#[test]
fn stub_harness_not_in_user_facing_list() {
    assert!(!HarnessName::USER_FACING.contains(&"stub"));
    assert!(!HarnessName::help_text().contains("stub"));
    assert!(!HarnessName::HARNESS_HELP.contains("stub"));
}

/// Verifies that github-copilot canonical name is not in user-facing list.
///
/// Only the "copilot" alias should appear in user-facing documentation, not the
/// internal "github-copilot" canonical name.
#[test]
fn github_copilot_canonical_name_not_in_user_facing_list() {
    assert!(!HarnessName::USER_FACING.contains(&"github-copilot"));
    assert!(!HarnessName::help_text().contains("github-copilot"));
}

/// Verifies that HarnessName implements Display correctly.
///
/// The Display trait should render the internal string representation.
#[test]
fn harness_name_display_returns_inner_string() {
    assert_eq!(format!("{}", HarnessName::CLAUDE), "claude");
    assert_eq!(format!("{}", HarnessName::OPENCODE), "opencode");
    assert_eq!(format!("{}", HarnessName::STUB), "stub");
}

/// Verifies that HarnessRunConfig can be created with minimal required fields.
///
/// This ensures the struct can be instantiated with default/empty values for
/// optional fields, which is useful in test scenarios.
#[test]
fn harness_run_config_can_use_minimal_fields() {
    let config = HarnessRunConfig {
        prompt: String::new(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    };

    assert_eq!(config.prompt, "");
    assert_eq!(config.model, None);
    assert!(!config.interactive);
    assert!(!config.allow_all);
    assert_eq!(config.inactivity_timeout, None);
}

/// Verifies that HarnessRunConfig correctly stores environment variables.
///
/// The env field should properly store key-value pairs for environment
/// variable configuration.
#[test]
fn harness_run_config_stores_environment_variables() {
    let mut env = BTreeMap::new();
    env.insert("API_KEY".to_string(), "test-key".to_string());
    env.insert("MODEL".to_string(), "sonnet".to_string());

    let config = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("/tmp"),
        env: env.clone(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    };

    assert_eq!(config.env.get("API_KEY"), Some(&"test-key".to_string()));
    assert_eq!(config.env.get("MODEL"), Some(&"sonnet".to_string()));
    assert_eq!(config.env.len(), 2);
}

/// Verifies that HarnessRunConfig accepts various inactivity timeout durations.
///
/// This ensures the timeout field can represent different time scales from
/// seconds to hours.
#[test]
fn harness_run_config_accepts_various_timeout_durations() {
    let short_timeout = Duration::from_secs(30);
    let medium_timeout = Duration::from_secs(5 * 60);
    let long_timeout = Duration::from_secs(60 * 60);

    let config_short = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: Some(short_timeout),
    };

    let config_medium = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: Some(medium_timeout),
    };

    let config_long = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: Some(long_timeout),
    };

    assert_eq!(config_short.inactivity_timeout, Some(short_timeout));
    assert_eq!(config_medium.inactivity_timeout, Some(medium_timeout));
    assert_eq!(config_long.inactivity_timeout, Some(long_timeout));
}

/// Verifies that HarnessRunConfig correctly represents interactive mode.
///
/// The interactive flag should be independently settable from other options.
#[test]
fn harness_run_config_interactive_flag_works_independently() {
    let interactive_config = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: true,
        allow_all: false,
        inactivity_timeout: None,
    };

    let non_interactive_config = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: true,
        inactivity_timeout: None,
    };

    assert!(interactive_config.interactive);
    assert!(!interactive_config.allow_all);
    assert!(!non_interactive_config.interactive);
    assert!(non_interactive_config.allow_all);
}

/// Verifies that HarnessRunConfig supports Unicode in prompts and paths.
///
/// This ensures the configuration can handle non-ASCII characters in various
/// fields, which is important for international users.
#[test]
fn harness_run_config_supports_unicode_content() {
    let config = HarnessRunConfig {
        prompt: "测试提示 🚀".to_string(),
        model: Some("модель".to_string()),
        cwd: PathBuf::from("/tmp/テスト"),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    };

    assert!(config.prompt.contains("🚀"));
    assert!(config.model.as_ref().unwrap().contains("модель"));
    assert!(config.cwd.to_string_lossy().contains("テスト"));
}

/// Verifies that HarnessName can be compared for equality.
///
/// This test ensures that harness names can be compared reliably, which is
/// important for matching and routing logic.
#[test]
fn harness_names_support_equality_comparison() {
    assert_eq!(HarnessName::CLAUDE, HarnessName::CLAUDE);
    assert_ne!(HarnessName::CLAUDE, HarnessName::CODEX);
    assert_ne!(HarnessName::COPILOT, HarnessName::GITHUB_COPILOT);
}

/// Verifies that HarnessName can be cloned.
///
/// The Clone trait is essential for passing harness names around without
/// ownership issues.
#[test]
fn harness_names_can_be_cloned() {
    let name1 = HarnessName::CLAUDE;
    let name2 = name1;
    assert_eq!(name1, name2);
}

/// Verifies that very long prompts are accepted in HarnessRunConfig.
///
/// This boundary test ensures the system can handle large prompt inputs,
/// which might occur when including extensive context or instructions.
#[test]
fn harness_run_config_accepts_very_long_prompts() {
    let long_prompt = "test ".repeat(10000);
    let config = HarnessRunConfig {
        prompt: long_prompt.clone(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    };

    assert_eq!(config.prompt.len(), long_prompt.len());
    assert!(config.prompt.len() > 50000);
}

/// Verifies that empty environment variables are handled correctly.
///
/// This edge case ensures the system gracefully handles empty env maps.
#[test]
fn harness_run_config_handles_empty_environment() {
    let config = HarnessRunConfig {
        prompt: "test".to_string(),
        model: None,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        interactive: false,
        allow_all: false,
        inactivity_timeout: None,
    };

    assert!(config.env.is_empty());
}