//! Interactive worktree setup wizard for `ito init` and `ito update`.
//!
//! This module is split into two parts:
//! - prompting (collecting user choices)
//! - persistence (writing choices to a config file)
//!
//! This separation lets `ito init` render templates with the chosen worktree
//! context *before* `.ito/config.json` exists, then persist to the project
//! config after template installation.

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

/// Prompt the user for worktree configuration.
///
/// Returns the resolved worktree configuration choices.
pub(crate) fn prompt_worktree_wizard() -> CliResult<WorktreeWizardResult> {
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

    Ok(WorktreeWizardResult {
        ran: true,
        enabled: true,
        strategy: Some(strategy.to_string()),
        integration_mode: Some(integration_mode.to_string()),
    })
}

/// Persist a [`WorktreeWizardResult`] to a JSON config file.
///
/// This only writes the `worktrees.*` keys and preserves any other existing
/// keys in the file.
pub(crate) fn persist_worktree_config(
    config_path: &Path,
    result: &WorktreeWizardResult,
) -> CliResult<()> {
    if let Some(parent) = config_path.parent() {
        let _ = ito_common::io::create_dir_all_std(parent);
    }

    let mut config = core_config::read_json_config(config_path)
        .map_err(|e| CliError::msg(format!("Failed to read config: {e}")))?;

    let enabled = serde_json::Value::Bool(result.enabled);
    let enabled_parts = core_config::json_split_path("worktrees.enabled");
    core_config::json_set_path(&mut config, &enabled_parts, enabled)
        .map_err(|e| CliError::msg(format!("Failed to set config: {e}")))?;

    if result.enabled {
        let Some(strategy) = result.strategy.as_deref() else {
            return Err(CliError::msg("Worktree wizard result missing strategy"));
        };
        let Some(integration_mode) = result.integration_mode.as_deref() else {
            return Err(CliError::msg(
                "Worktree wizard result missing integration_mode",
            ));
        };

        let settings: &[(&str, serde_json::Value)] = &[
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
    }

    core_config::write_json_config(config_path, &config)
        .map_err(|e| CliError::msg(format!("Failed to write config: {e}")))?;

    Ok(())
}

pub(crate) fn print_worktree_config_written(config_path: &Path, result: &WorktreeWizardResult) {
    if !result.enabled {
        println!("\nWorktree mode disabled.");
        println!("Config file: {}", config_path.display());
        println!("  worktrees.enabled = false\n");
        return;
    }

    let strategy = result.strategy.as_deref().unwrap_or_default();
    let integration_mode = result.integration_mode.as_deref().unwrap_or_default();

    println!("\nWorktree configuration saved.");
    println!("Config file: {}", config_path.display());
    println!("  worktrees.enabled = true");
    println!("  worktrees.strategy = {strategy}");
    println!("  worktrees.apply.integration_mode = {integration_mode}\n");
}

pub(crate) fn save_worktree_config(
    config_path: &Path,
    result: &WorktreeWizardResult,
) -> CliResult<()> {
    persist_worktree_config(config_path, result)?;
    print_worktree_config_written(config_path, result);
    Ok(())
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
