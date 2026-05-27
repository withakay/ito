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

const STRATEGY_VALUES: &[&str] = &[
    "checkout_subdir",
    "checkout_siblings",
    "bare_control_siblings",
];
const INTEGRATION_MODE_VALUES: &[&str] = &["commit_pr", "merge_parent"];

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

/// Default selections for the interactive worktree wizard.
#[derive(Debug, Clone)]
pub(crate) struct WorktreeWizardDefaults {
    /// Default worktree enablement.
    pub enabled: bool,
    /// Default strategy value when worktrees are enabled.
    pub strategy: String,
    /// Default integration mode when worktrees are enabled.
    pub integration_mode: String,
}

impl Default for WorktreeWizardDefaults {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "checkout_subdir".to_string(),
            integration_mode: "commit_pr".to_string(),
        }
    }
}

impl From<&WorktreeWizardResult> for WorktreeWizardDefaults {
    fn from(result: &WorktreeWizardResult) -> Self {
        Self {
            enabled: result.enabled,
            strategy: result
                .strategy
                .clone()
                .unwrap_or_else(|| "checkout_subdir".to_string()),
            integration_mode: result
                .integration_mode
                .clone()
                .unwrap_or_else(|| "commit_pr".to_string()),
        }
    }
}

/// Explicit worktree config choices supplied by CLI flags.
#[derive(Debug, Clone, Default)]
pub(crate) struct WorktreeWizardOverrides {
    /// Override worktree enablement.
    pub enabled: Option<bool>,
    /// Override worktree strategy.
    pub strategy: Option<String>,
    /// Override worktree integration mode.
    pub integration_mode: Option<String>,
}

impl WorktreeWizardOverrides {
    /// Whether any override was supplied.
    pub fn any(&self) -> bool {
        self.enabled.is_some() || self.strategy.is_some() || self.integration_mode.is_some()
    }
}

/// Parse worktree setup flags shared by `ito init` and `ito update`.
pub(crate) fn parse_worktree_overrides(args: &[String]) -> CliResult<WorktreeWizardOverrides> {
    let enable = args.iter().any(|arg| arg == "--worktrees");
    let disable = args.iter().any(|arg| arg == "--no-worktrees");
    if enable && disable {
        return Err(CliError::msg(
            "--worktrees and --no-worktrees cannot be used together",
        ));
    }

    let strategy = parse_flag_value(args, "--worktree-strategy")?;
    if let Some(value) = strategy.as_deref() {
        validate_choice("--worktree-strategy", value, STRATEGY_VALUES)?;
    }

    let integration_mode = parse_flag_value(args, "--worktree-integration-mode")?;
    if let Some(value) = integration_mode.as_deref() {
        validate_choice(
            "--worktree-integration-mode",
            value,
            INTEGRATION_MODE_VALUES,
        )?;
    }

    Ok(WorktreeWizardOverrides {
        enabled: if enable {
            Some(true)
        } else if disable {
            Some(false)
        } else {
            None
        },
        strategy,
        integration_mode,
    })
}

/// Apply explicit CLI overrides to an existing/default worktree result.
pub(crate) fn apply_worktree_overrides(
    mut result: WorktreeWizardResult,
    overrides: &WorktreeWizardOverrides,
) -> WorktreeWizardResult {
    if let Some(enabled) = overrides.enabled {
        result.enabled = enabled;
    }

    if overrides.strategy.is_some() || overrides.integration_mode.is_some() {
        result.enabled = true;
    }

    if result.enabled {
        if let Some(strategy) = &overrides.strategy {
            result.strategy = Some(strategy.clone());
        } else if result.strategy.is_none() {
            result.strategy = Some("checkout_subdir".to_string());
        }

        if let Some(integration_mode) = &overrides.integration_mode {
            result.integration_mode = Some(integration_mode.clone());
        } else if result.integration_mode.is_none() {
            result.integration_mode = Some("commit_pr".to_string());
        }
    } else {
        result.strategy = None;
        result.integration_mode = None;
    }

    result
}

fn parse_flag_value(args: &[String], flag: &str) -> CliResult<Option<String>> {
    let mut found = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == flag {
            let Some(value) = iter.next() else {
                return Err(CliError::msg(format!("{flag} requires a value")));
            };
            found = Some(value.clone());
        } else if let Some(value) = arg.strip_prefix(&format!("{flag}=")) {
            found = Some(value.to_string());
        }
    }
    Ok(found)
}

fn validate_choice(flag: &str, value: &str, allowed: &[&str]) -> CliResult<()> {
    if allowed.contains(&value) {
        return Ok(());
    }

    Err(CliError::msg(format!(
        "Invalid value for {flag}: '{value}'. Valid values: {}",
        allowed.join(", ")
    )))
}

/// Prompt the user for worktree configuration.
///
/// Returns the resolved worktree configuration choices.
pub(crate) fn prompt_worktree_wizard() -> CliResult<WorktreeWizardResult> {
    prompt_worktree_wizard_with_defaults(&WorktreeWizardDefaults::default())
}

/// Prompt the user for worktree configuration with explicit default selections.
pub(crate) fn prompt_worktree_wizard_with_defaults(
    defaults: &WorktreeWizardDefaults,
) -> CliResult<WorktreeWizardResult> {
    println!("\n--- Worktree Configuration ---\n");

    // Question 1: Enable worktrees?
    let enable_items = &["No", "Yes"];
    let enable_default = usize::from(defaults.enabled);
    let enable_idx =
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Enable Git worktree-based workspace layout?")
            .items(enable_items)
            .default(enable_default)
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
    let strategy_default = STRATEGY_VALUES
        .iter()
        .position(|value| *value == defaults.strategy)
        .unwrap_or(0);

    let strategy_idx =
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Which worktree strategy?")
            .items(strategy_items)
            .default(strategy_default)
            .interact()
        {
            Ok(v) => v,
            Err(e) => {
                return Err(CliError::msg(format!(
                    "Failed to prompt for worktree strategy: {e}"
                )));
            }
        };
    let strategy = STRATEGY_VALUES[strategy_idx];

    // Question 3: Integration mode
    let mode_items = &[
        "commit_pr (Recommended) - commit and open a pull request",
        "merge_parent - merge directly into parent branch",
    ];
    let mode_default = INTEGRATION_MODE_VALUES
        .iter()
        .position(|value| *value == defaults.integration_mode)
        .unwrap_or(0);

    let mode_idx = match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Preferred integration mode after implementation?")
        .items(mode_items)
        .default(mode_default)
        .interact()
    {
        Ok(v) => v,
        Err(e) => {
            return Err(CliError::msg(format!(
                "Failed to prompt for integration mode: {e}"
            )));
        }
    };
    let integration_mode = INTEGRATION_MODE_VALUES[mode_idx];

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

#[cfg(test)]
mod worktree_wizard_tests;
