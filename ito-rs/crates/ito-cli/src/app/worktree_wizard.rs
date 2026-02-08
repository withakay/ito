//! Interactive worktree setup wizard for `ito init` and `ito update`.
//!
//! Guides the user through enabling worktrees, selecting a strategy, and
//! choosing an integration mode. Choices are persisted to config immediately.
//!
//! The [`WorktreeWizardResult`] carries the resolved config values so that
//! downstream consumers (e.g., template rendering) can use them without
//! re-reading the config file.

use crate::cli_error::{CliError, CliResult};
use ito_core::config as core_config;
use std::path::Path;

/// Result of the worktree setup wizard.
///
/// Carries the resolved worktree configuration values so that callers (e.g.,
/// `init.rs`) can forward them to the template installer for Jinja2 rendering.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct WorktreeWizardResult {
    /// Whether the wizard ran (false if skipped).
    pub ran: bool,
    /// Whether worktrees were enabled by the user.
    pub enabled: bool,
    /// The chosen worktree strategy (e.g., `"checkout_subdir"`).
    ///
    /// `None` when worktrees are disabled.
    pub strategy: Option<String>,
    /// The chosen integration mode (e.g., `"commit_pr"`).
    ///
    /// `None` when worktrees are disabled.
    pub integration_mode: Option<String>,
}

/// Run the interactive worktree setup wizard.
///
/// Asks:
/// 1. Whether to enable worktrees for this project
/// 2. Which strategy to use (if enabled)
/// 3. Which integration mode to prefer (if enabled)
///
/// Persists answers to the config file at `config_path` and prints the
/// config file path and written keys.
///
/// Returns a result indicating what was chosen.
pub(crate) fn run_worktree_wizard(config_path: &Path) -> CliResult<WorktreeWizardResult> {
    println!("\n--- Worktree Configuration ---\n");

    // Question 1: Enable worktrees?
    let enable_items = &["No", "Yes"];
    let enable_idx =
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Enable Git worktree-based workspace layout?")
            .items(enable_items)
            .default(0)
            .interact()
        {
            Ok(v) => v,
            Err(e) => {
                return Err(CliError::msg(format!(
                    "Failed to prompt for worktree enablement: {e}"
                )));
            }
        };

    let enabled = enable_idx == 1;

    if !enabled {
        // Only persist enabled=false and exit
        let mut config = core_config::read_json_config(config_path)
            .map_err(|e| CliError::msg(format!("Failed to read config: {e}")))?;

        let parts = core_config::json_split_path("worktrees.enabled");
        core_config::json_set_path(&mut config, &parts, serde_json::Value::Bool(false))
            .map_err(|e| CliError::msg(format!("Failed to set config: {e}")))?;

        core_config::write_json_config(config_path, &config)
            .map_err(|e| CliError::msg(format!("Failed to write config: {e}")))?;

        println!("\nWorktree mode disabled.");
        println!("Config file: {}", config_path.display());
        println!("  worktrees.enabled = false\n");

        return Ok(WorktreeWizardResult {
            ran: true,
            enabled: false,
            strategy: None,
            integration_mode: None,
        });
    }

    // Question 2: Strategy
    let strategy_items = &[
        "checkout_subdir (Recommended) - worktrees in a gitignored subdirectory",
        "checkout_siblings - worktrees in a sibling directory",
        "bare_control_siblings - bare repo with worktrees as siblings",
    ];
    let strategy_values = &[
        "checkout_subdir",
        "checkout_siblings",
        "bare_control_siblings",
    ];

    let strategy_idx =
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Which worktree strategy?")
            .items(strategy_items)
            .default(0)
            .interact()
        {
            Ok(v) => v,
            Err(e) => {
                return Err(CliError::msg(format!(
                    "Failed to prompt for worktree strategy: {e}"
                )));
            }
        };
    let strategy = strategy_values[strategy_idx];

    // Question 3: Integration mode
    let mode_items = &[
        "commit_pr (Recommended) - commit and open a pull request",
        "merge_parent - merge directly into parent branch",
    ];
    let mode_values = &["commit_pr", "merge_parent"];

    let mode_idx = match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Preferred integration mode after implementation?")
        .items(mode_items)
        .default(0)
        .interact()
    {
        Ok(v) => v,
        Err(e) => {
            return Err(CliError::msg(format!(
                "Failed to prompt for integration mode: {e}"
            )));
        }
    };
    let integration_mode = mode_values[mode_idx];

    // Persist all answers
    let mut config = core_config::read_json_config(config_path)
        .map_err(|e| CliError::msg(format!("Failed to read config: {e}")))?;

    let settings: &[(&str, serde_json::Value)] = &[
        ("worktrees.enabled", serde_json::Value::Bool(true)),
        (
            "worktrees.strategy",
            serde_json::Value::String(strategy.to_string()),
        ),
        (
            "worktrees.apply.integration_mode",
            serde_json::Value::String(integration_mode.to_string()),
        ),
    ];

    for (key, value) in settings {
        let parts = core_config::json_split_path(key);
        core_config::json_set_path(&mut config, &parts, value.clone())
            .map_err(|e| CliError::msg(format!("Failed to set config key '{key}': {e}")))?;
    }

    core_config::write_json_config(config_path, &config)
        .map_err(|e| CliError::msg(format!("Failed to write config: {e}")))?;

    println!("\nWorktree configuration saved.");
    println!("Config file: {}", config_path.display());
    println!("  worktrees.enabled = true");
    println!("  worktrees.strategy = {strategy}");
    println!("  worktrees.apply.integration_mode = {integration_mode}\n");

    Ok(WorktreeWizardResult {
        ran: true,
        enabled: true,
        strategy: Some(strategy.to_string()),
        integration_mode: Some(integration_mode.to_string()),
    })
}

/// Check whether worktree strategy is already configured in the given config file.
pub(crate) fn is_worktree_configured(config_path: &Path) -> bool {
    let Ok(config) = core_config::read_json_config(config_path) else {
        return false;
    };
    let parts = core_config::json_split_path("worktrees.strategy");
    core_config::json_get_path(&config, &parts).is_some()
}

/// Load a [`WorktreeWizardResult`] from an existing config file.
///
/// Returns a "disabled" result if the file does not exist or has no worktree
/// configuration. This is used for non-interactive init and for `ito update`
/// where the wizard does not run but we still need config for template rendering.
#[allow(dead_code)]
pub(crate) fn load_worktree_result_from_config(config_path: &Path) -> WorktreeWizardResult {
    let Ok(config) = core_config::read_json_config(config_path) else {
        return WorktreeWizardResult {
            ran: false,
            enabled: false,
            strategy: None,
            integration_mode: None,
        };
    };

    let enabled =
        core_config::json_get_path(&config, &core_config::json_split_path("worktrees.enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

    if !enabled {
        return WorktreeWizardResult {
            ran: false,
            enabled: false,
            strategy: None,
            integration_mode: None,
        };
    }

    let strategy =
        core_config::json_get_path(&config, &core_config::json_split_path("worktrees.strategy"))
            .and_then(|v| v.as_str().map(|s| s.to_string()));

    let integration_mode = core_config::json_get_path(
        &config,
        &core_config::json_split_path("worktrees.apply.integration_mode"),
    )
    .and_then(|v| v.as_str().map(|s| s.to_string()));

    WorktreeWizardResult {
        ran: false,
        enabled: true,
        strategy,
        integration_mode,
    }
}
