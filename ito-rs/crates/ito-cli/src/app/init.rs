use crate::app::worktree_wizard::{
    WorktreeWizardResult, load_worktree_result_from_config, prompt_worktree_wizard,
    save_worktree_config,
};
use crate::cli::InitArgs;
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_config::ito_dir;
use ito_config::output;
use ito_config::types::CoordinationStorage;
use ito_config::{
    ConfigContext, load_cascading_project_config, resolve_coordination_branch_settings,
};
use ito_core::config as core_config;
use ito_core::coordination_worktree::provision_coordination_worktree;
use ito_core::git::{CoordinationBranchSetupStatus, ensure_coordination_branch_on_origin};
use ito_core::installers::{InitOptions, InstallMode, install_default_templates};
use ito_templates::project_templates::WorktreeTemplateContext;
use std::collections::BTreeSet;
use std::io::IsTerminal;

/// Initialize a project using the `ito init` command, handling interactive prompts or explicit CLI flags.
///
/// This parses the provided `args` for flags and a target path, determines which tooling to configure
/// (from `--tools` or an interactive selection), resolves and optionally persists worktree configuration,
/// installs the default templates, optionally ensures the coordination branch exists on origin
/// when `--setup-coordination-branch` is given, and prints post-initialization guidance.
///
/// # Examples
///
/// ```ignore
/// // Print help and exit
/// let rt = obtain_runtime();
/// let args = vec!["--help".to_string()];
/// handle_init(&rt, &args).unwrap();
/// ```
pub(super) fn handle_init(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["init"], "ito init")
        );
        return Ok(());
    }

    let force = args.iter().any(|a| a == "--force" || a == "-f");
    let upgrade = args.iter().any(|a| a == "--upgrade");
    // --update activates non-destructive update semantics; --upgrade implies update semantics.
    let update = args.iter().any(|a| a == "--update" || a == "-u");
    let setup_coordination_branch = args.iter().any(|a| a == "--setup-coordination-branch");
    let no_coordination_worktree = args.iter().any(|a| a == "--no-coordination-worktree");
    let no_tmux = args.iter().any(|a| a == "--no-tmux");
    let tools_arg = parse_string_flag(args, "--tools");

    // Positional path (defaults to current directory).
    let target = super::common::last_positional(args).unwrap_or_else(|| ".".to_string());
    let target_path = std::path::Path::new(&target);
    let ctx = rt.ctx();

    let all_ids = ito_core::installers::available_tool_ids();

    let tools: BTreeSet<String> = if let Some(raw) = tools_arg.as_deref() {
        let raw = raw.trim();
        if raw.is_empty() {
            return fail("--tools cannot be empty");
        }

        if raw == "none" {
            BTreeSet::new()
        } else if raw == "all" {
            all_ids.iter().map(|s| (*s).to_string()).collect()
        } else {
            let valid = all_ids.join(", ");
            let mut selected: BTreeSet<String> = BTreeSet::new();
            for part in raw.split(',') {
                let id = part.trim();
                if id.is_empty() {
                    continue;
                }
                if all_ids.contains(&id) {
                    selected.insert(id.to_string());
                } else {
                    return fail(format!("Unknown tool id '{id}'. Valid tool ids: {valid}"));
                }
            }
            selected
        }
    } else {
        use std::io::BufRead;
        use std::io::{IsTerminal, stdin, stdout};

        // Match TS semantics: prompt only when interactive; otherwise require explicit --tools.
        let ui = output::resolve_ui_options(
            false,
            std::env::var("NO_COLOR").ok().as_deref(),
            false,
            std::env::var("ITO_INTERACTIVE").ok().as_deref(),
        );
        let is_tty = stdin().is_terminal() && stdout().is_terminal();
        if !(ui.interactive && is_tty) {
            return fail(
                "Non-interactive init requires --tools (all, none, or comma-separated ids).",
            );
        }

        println!(
            "Welcome to Ito!\n\nStep 1/3\n\nConfigure your Ito tooling\nPress Enter to continue."
        );
        {
            let mut line = String::new();
            let mut locked = stdin().lock();
            let _ = locked.read_line(&mut line);
        }

        println!(
            "\nStep 2/3\n\nWhich natively supported AI tools do you use?\nUse ↑/↓ to move · Space to toggle · Enter reviews.\n"
        );

        let mut detected: BTreeSet<&'static str> = BTreeSet::new();
        if target_path.join("CLAUDE.md").exists() || target_path.join(".claude").exists() {
            detected.insert(ito_core::installers::TOOL_CLAUDE);
        }
        if target_path.join(".opencode").exists() {
            detected.insert(ito_core::installers::TOOL_OPENCODE);
        }
        if target_path.join(".github").exists() {
            detected.insert(ito_core::installers::TOOL_GITHUB_COPILOT);
        }
        if target_path.join(".codex").exists() {
            detected.insert(ito_core::installers::TOOL_CODEX);
        }
        if target_path.join(".pi").exists() {
            detected.insert(ito_core::installers::TOOL_PI);
        }

        let tool_items: Vec<(&'static str, &str)> = vec![
            (ito_core::installers::TOOL_CLAUDE, "Claude Code"),
            (ito_core::installers::TOOL_CODEX, "Codex"),
            (ito_core::installers::TOOL_GITHUB_COPILOT, "GitHub Copilot"),
            (ito_core::installers::TOOL_OPENCODE, "OpenCode"),
            (ito_core::installers::TOOL_PI, "Pi"),
        ];
        let labels: Vec<String> = tool_items
            .iter()
            .map(|(id, label)| format!("{label} ({id})"))
            .collect();
        let defaults: Vec<bool> = tool_items
            .iter()
            .map(|(id, _)| detected.contains(id))
            .collect();

        let indices =
            match dialoguer::MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("Select AI tools to configure")
                .items(&labels)
                .defaults(&defaults)
                .interact()
            {
                Ok(v) => v,
                Err(e) => {
                    return Err(CliError::msg(format!("Failed to prompt for tools: {e}")));
                }
            };

        println!("\nStep 3/3\n\nReview selections\nPress Enter to confirm.");
        {
            let mut line = String::new();
            let mut locked = stdin().lock();
            let _ = locked.read_line(&mut line);
        }

        indices
            .into_iter()
            .map(|i| tool_items[i].0.to_string())
            .collect()
    };

    // Resolve worktree config BEFORE template installation so that templates
    // can be rendered with the user's worktree preferences.
    let ui = output::resolve_ui_options(
        false,
        std::env::var("NO_COLOR").ok().as_deref(),
        false,
        std::env::var("ITO_INTERACTIVE").ok().as_deref(),
    );
    let is_tty = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let is_interactive = ui.interactive && is_tty && !args.iter().any(|a| a == "--no-interactive");
    let existing_tmux_preference = load_tmux_preference(target_path, ctx)?;
    let tmux_enabled = resolve_tmux_preference(is_interactive, no_tmux, existing_tmux_preference)?;

    let (worktree_result, worktree_project_config_path, should_persist_worktree) =
        resolve_worktree_config(ctx, target_path, is_interactive)?;
    let worktree_ctx = worktree_template_context(&worktree_result, target_path, ctx);

    let opts = if upgrade {
        InitOptions::new_upgrade(tools)
    } else {
        InitOptions::new(tools, force, update)
    };
    install_default_templates(
        target_path,
        ctx,
        InstallMode::Init,
        &opts,
        Some(&worktree_ctx),
    )
    .map_err(to_cli_error)?;

    persist_tmux_preference(target_path, ctx, tmux_enabled)?;

    if should_persist_worktree {
        save_worktree_config(&worktree_project_config_path, &worktree_result)?;
    }

    if setup_coordination_branch {
        let ito_path = ito_dir::get_ito_path(target_path, ctx);
        let project_root = ito_path.parent().ok_or_else(|| {
            CliError::msg(format!(
                "Could not determine project root from Ito path: {}",
                ito_path.display()
            ))
        })?;
        let merged = load_cascading_project_config(project_root, &ito_path, ctx).merged;
        let (_, coord_branch) = resolve_coordination_branch_settings(&merged);
        let setup_result = ensure_coordination_branch_on_origin(project_root, &coord_branch)
            .map_err(|err| {
                CliError::msg(format!(
                    "Failed to set up coordination branch '{}': {}",
                    coord_branch, err.message
                ))
            })?;

        match setup_result {
            CoordinationBranchSetupStatus::Ready => {
                println!("Coordination branch ready on origin: {coord_branch}");
            }
            CoordinationBranchSetupStatus::Created => {
                println!("Coordination branch created on origin: {coord_branch}");
            }
        }
    }

    // Coordination worktree setup: only for fresh init (not --upgrade), and only
    // when backend mode is not enabled. Failures are non-fatal — init still succeeds
    // but falls back to embedded storage mode.
    if !upgrade {
        setup_coordination_worktree(target_path, ctx, no_coordination_worktree);
    }

    print_post_init_guidance(target_path, ctx);

    print_repo_validation_advisory(target_path, ctx);

    Ok(())
}

/// Print a brief advisory pointing at the `ito-update-repo` skill when at
/// least one repository validation rule is active and no `ito validate repo`
/// pre-commit hook is detected.
///
/// The advisory is **purely informational**: it never modifies
/// `.pre-commit-config.yaml`, `.husky/`, `lefthook.yml`, or any other file.
/// Wiring is left to the `ito-update-repo` skill so downstream projects opt
/// in by running the skill explicitly.
fn print_repo_validation_advisory(target_path: &std::path::Path, ctx: &ConfigContext) {
    let ito_path = ito_dir::get_ito_path(target_path, ctx);

    // Best-effort: if the config cannot be loaded or deserialized, suppress
    // the advisory rather than fail init.
    let cfg_value = load_cascading_project_config(target_path, &ito_path, ctx).merged;
    let Ok(config) = serde_json::from_value::<ito_config::types::ItoConfig>(cfg_value) else {
        return;
    };

    let rules = ito_core::validate_repo::list_active_rules(&config);
    let any_active = rules.iter().any(|r| r.active);
    if !any_active {
        return;
    }

    if pre_commit_config_already_wires_ito_validate_repo(target_path) {
        return;
    }

    let detected = ito_core::validate_repo::detect_pre_commit_system(target_path);

    eprintln!();
    eprintln!("Tip: `ito validate repo` is not wired into your pre-commit hook.");
    eprintln!(
        "    Detected pre-commit system: {detected}. {} active validation rule(s) would run.",
        rules.iter().filter(|r| r.active).count(),
    );
    eprintln!(
        "    Run the `ito-update-repo` skill to wire it (proposes a diff, asks before applying)."
    );
}

/// True when the project already has a hook entry for `ito validate repo`
/// in any of the supported pre-commit framework config files.
///
/// Substring scan: cheap and good enough for an advisory that errs on the
/// side of staying silent.
fn pre_commit_config_already_wires_ito_validate_repo(target_path: &std::path::Path) -> bool {
    let candidates = [
        target_path.join(".pre-commit-config.yaml"),
        target_path.join(".pre-commit-config.yml"),
        target_path.join("lefthook.yml"),
        target_path.join("lefthook.yaml"),
        target_path.join(".lefthook.yml"),
        target_path.join(".lefthook.yaml"),
        target_path.join(".husky").join("pre-commit"),
    ];
    for path in &candidates {
        let Ok(body) = std::fs::read_to_string(path) else {
            continue;
        };
        // Match either the canonical hook id or a literal command line.
        if body.contains("ito-validate-repo") || body.contains("ito validate repo") {
            return true;
        }
    }
    false
}

/// Prints post-initialization guidance showing where Ito was initialized and suggested next steps.
///
/// The message lists the target location, instructions for running the project setup via the AI assistant,
/// and paths to common configuration and documentation files.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // Print guidance for the current directory
/// # use ito_config::ConfigContext;
/// print_post_init_guidance(Path::new("."), &ConfigContext::default());
/// ```
fn print_post_init_guidance(target_path: &std::path::Path, ctx: &ConfigContext) {
    let abs_target = ito_config::ito_dir::absolutize_and_normalize(target_path)
        .unwrap_or_else(|_| target_path.to_path_buf());

    let ito_dir = ito_dir::get_ito_path(target_path, ctx);

    let needs_project_setup = project_setup_is_incomplete(&ito_dir);

    println!("\nIto initialized in {}\n", abs_target.display());

    if needs_project_setup {
        println!(
            "Next step: Run /ito-project-setup in your AI assistant to configure the project.\n"
        );
    }

    println!(
        r#"Or manually edit:
  {}/project.md        Project overview, tech stack, architecture
  {}/user-prompts/     Shared + artifact-specific instruction guidance
  {}/config.json       Tool settings and defaults

Learn more: ito --help | ito agent instruction --help
"#,
        ito_dir.display(),
        ito_dir.display(),
        ito_dir.display(),
    );
}

fn project_setup_is_incomplete(ito_dir: &std::path::Path) -> bool {
    const COMPLETE: &str = "<!-- ITO:PROJECT_SETUP:COMPLETE -->";
    const INCOMPLETE: &str = "<!-- ITO:PROJECT_SETUP:INCOMPLETE -->";

    let Ok(contents) = std::fs::read_to_string(ito_dir.join("project.md")) else {
        return false;
    };

    if contents.contains(COMPLETE) {
        return false;
    }

    contents.contains(INCOMPLETE)
}

/// Convert parsed `InitArgs` into CLI-style argv, optionally override `HOME`, and run the init flow.
///
/// If `args.home` is provided, the `HOME` environment variable is set to that value. The function
/// translates the present `tools`, `force`, `update`, `upgrade`, `setup_coordination_branch`, and
/// `path` fields into their corresponding CLI flags and arguments, then delegates to `handle_init`.
///
/// # Examples
///
/// ```
/// # use crate::{Runtime, InitArgs, handle_init_clap};
/// # fn make_runtime() -> Runtime { unimplemented!() }
/// let rt = make_runtime();
/// let args = InitArgs {
///     home: None,
///     tools: Some("all".to_string()),
///     force: false,
///     update: false,
///     upgrade: true,
///     setup_coordination_branch: false,
///     path: Some(".".to_string()),
/// };
/// let _ = handle_init_clap(&rt, &args);
/// ```
///
/// # Returns
///
/// `Ok(())` on success, or a `CliError` if the init flow fails.
pub(crate) fn handle_init_clap(rt: &Runtime, args: &InitArgs) -> CliResult<()> {
    if let Some(home) = &args.home {
        // For parity/testing.
        unsafe {
            std::env::set_var("HOME", home);
        }
    }

    let mut argv: Vec<String> = Vec::new();
    if let Some(tools) = &args.tools {
        argv.push("--tools".to_string());
        argv.push(tools.clone());
    }
    if args.force {
        argv.push("--force".to_string());
    }
    if args.update {
        argv.push("--update".to_string());
    }
    if args.upgrade {
        argv.push("--upgrade".to_string());
    }
    if args.setup_coordination_branch {
        argv.push("--setup-coordination-branch".to_string());
    }
    if args.no_coordination_worktree {
        argv.push("--no-coordination-worktree".to_string());
    }
    if args.no_tmux {
        argv.push("--no-tmux".to_string());
    }
    if let Some(path) = &args.path {
        argv.push(path.clone());
    }
    handle_init(rt, &argv)
}

/// Resolve worktree configuration for template rendering.
///
/// In interactive mode, runs the worktree setup wizard and returns the user's
/// choices. In non-interactive mode, loads existing config from the global
/// config file, defaulting to "disabled" if no config exists.
fn resolve_worktree_config(
    ctx: &ConfigContext,
    target_path: &std::path::Path,
    interactive: bool,
) -> CliResult<(WorktreeWizardResult, std::path::PathBuf, bool)> {
    let ito_path = ito_dir::get_ito_path(target_path, ctx);
    let project_config_path = ito_path.join("config.json");
    let project_local_config_path = ito_path.join("config.local.json");
    let global_config_path = ito_config::global_config_path(ctx);

    if interactive {
        let result = prompt_worktree_wizard()?;
        // Worktree workflow is a per-developer preference; persist to the
        // project-local (gitignored) config overlay by default.
        return Ok((result, project_local_config_path, true));
    }

    // Non-interactive init: prefer per-dev project-local config overlay, then
    // project config, then global config for backward compatibility.
    let local_result = load_worktree_result_from_config(&project_local_config_path);
    if local_result.enabled
        || crate::app::worktree_wizard::is_worktree_configured(&project_local_config_path)
    {
        return Ok((local_result, project_local_config_path, false));
    }

    let project_result = load_worktree_result_from_config(&project_config_path);
    if project_result.enabled
        || crate::app::worktree_wizard::is_worktree_configured(&project_config_path)
    {
        return Ok((project_result, project_config_path, false));
    }

    if let Some(global_path) = global_config_path {
        let global_result = load_worktree_result_from_config(&global_path);
        if global_result.enabled
            || crate::app::worktree_wizard::is_worktree_configured(&global_path)
        {
            return Ok((global_result, project_local_config_path, false));
        }
    }

    Ok((
        WorktreeWizardResult {
            ran: false,
            enabled: false,
            strategy: None,
            integration_mode: None,
        },
        project_local_config_path,
        false,
    ))
}

fn persist_tmux_preference(
    target_path: &std::path::Path,
    ctx: &ConfigContext,
    enabled: bool,
) -> CliResult<()> {
    let ito_path = ito_dir::get_ito_path(target_path, ctx);
    let config_path = ito_path.join("config.json");
    let mut config = core_config::read_json_config(&config_path)
        .map_err(|e| CliError::msg(format!("Failed to read config: {e}")))?;

    let parts = core_config::json_split_path("tools.tmux.enabled");
    core_config::json_set_path(&mut config, &parts, serde_json::Value::Bool(enabled))
        .map_err(|e| CliError::msg(format!("Failed to set tmux config: {e}")))?;

    core_config::write_json_config(&config_path, &config)
        .map_err(|e| CliError::msg(format!("Failed to write config: {e}")))?;

    Ok(())
}

fn load_tmux_preference(
    target_path: &std::path::Path,
    ctx: &ConfigContext,
) -> CliResult<Option<bool>> {
    let ito_path = ito_dir::get_ito_path(target_path, ctx);
    let merged = load_cascading_project_config(target_path, &ito_path, ctx);
    Ok(merged
        .merged
        .pointer("/tools/tmux/enabled")
        .and_then(|value| value.as_bool()))
}

fn resolve_tmux_preference(
    interactive: bool,
    no_tmux: bool,
    existing_preference: Option<bool>,
) -> CliResult<bool> {
    if no_tmux {
        return Ok(false);
    }

    if !interactive {
        return Ok(existing_preference.unwrap_or(true));
    }

    dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Do you use tmux?")
        .default(existing_preference.unwrap_or(true))
        .interact()
        .map_err(|e| CliError::msg(format!("Failed to prompt for tmux preference: {e}")))
}

/// Set up the coordination worktree for a fresh `ito init`.
///
/// Delegates all business logic to [`provision_coordination_worktree`] in
/// `ito-core` and handles the result:
///
/// - `Ok(Some(storage))` — writes `storage` to the project config.
/// - `Ok(None)` — backend mode is active; nothing to do.
/// - `Err(err)` — logs a warning and falls back to embedded storage mode.
///
/// All failures are non-fatal: a warning is printed and init continues.
fn setup_coordination_worktree(target_path: &std::path::Path, ctx: &ConfigContext, skip: bool) {
    let ito_path = ito_dir::get_ito_path(target_path, ctx);
    let config_path = ito_path.join("config.json");

    let Some(project_root) = ito_path.parent() else {
        eprintln!(
            "warning: could not determine project root from Ito path '{}'; \
             skipping coordination worktree setup.\n\
             Continuing with embedded storage mode.\n\
             Fix: ensure the .ito/ directory is inside a valid project directory.",
            ito_path.display()
        );
        if let Err(e) = write_coordination_storage(&config_path, CoordinationStorage::Embedded) {
            eprintln!(
                "warning: could not write coordination storage config to {}: {e}\n\
                 Fix: manually set `changes.coordination_branch.storage` to \"embedded\" in {}",
                config_path.display(),
                config_path.display(),
            );
        }
        return;
    };

    match provision_coordination_worktree(project_root, &ito_path, skip) {
        Ok(Some(storage)) => {
            if let Err(e) = write_coordination_storage(&config_path, storage) {
                eprintln!(
                    "warning: could not write coordination storage config to {}: {e}\n\
                     Fix: manually set `changes.coordination_branch.storage` in {}",
                    config_path.display(),
                    config_path.display(),
                );
            }
        }
        Ok(None) => {
            // Backend mode is active — backend owns coordination; nothing to do.
        }
        Err(err) => {
            eprintln!("Warning: coordination worktree setup failed: {err}");
            eprintln!("Continuing with embedded storage mode.");
            if let Err(e) = write_coordination_storage(&config_path, CoordinationStorage::Embedded)
            {
                eprintln!(
                    "warning: could not write coordination storage config to {}: {e}\n\
                     Fix: manually set `changes.coordination_branch.storage` to \"embedded\" in {}",
                    config_path.display(),
                    config_path.display(),
                );
            }
        }
    }
}

/// Write the `changes.coordination_branch.storage` field to the project config file.
///
/// Reads the existing config (or starts from an empty object), sets the storage
/// field, and writes the result back.
fn write_coordination_storage(
    config_path: &std::path::Path,
    storage: CoordinationStorage,
) -> CliResult<()> {
    let mut config = core_config::read_json_config(config_path)
        .map_err(|e| CliError::msg(format!("Failed to read config: {e}")))?;

    let parts = core_config::json_split_path("changes.coordination_branch.storage");
    core_config::json_set_path(
        &mut config,
        &parts,
        serde_json::Value::String(storage.to_string()),
    )
    .map_err(|e| CliError::msg(format!("Failed to set coordination storage config: {e}")))?;

    core_config::write_json_config(config_path, &config)
        .map_err(|e| CliError::msg(format!("Failed to write config: {e}")))?;

    Ok(())
}

/// Create a WorktreeTemplateContext from a WorktreeWizardResult for template rendering.
///
/// If `result.enabled` is false, returns the disabled/default `WorktreeTemplateContext`.
/// When enabled, maps string and option fields from the wizard result into the context,
/// using template defaults resolved via `ito_core::config::resolve_worktree_template_defaults`.
/// The returned context's `project_root` is the absolute, normalized string form of
/// `target_path`; if normalization fails, `target_path` is used as-is.
///
/// # Examples
///
/// ```
/// // Construct a wizard result (disabled for this simple example) and a config context,
/// // then obtain the template context for the current directory.
/// let result = WorktreeWizardResult { enabled: false, ..Default::default() };
/// let ctx = ConfigContext::default();
/// let tpl_ctx = worktree_template_context(&result, std::path::Path::new("."), &ctx);
/// assert!(!tpl_ctx.enabled);
/// ```
fn worktree_template_context(
    result: &WorktreeWizardResult,
    target_path: &std::path::Path,
    ctx: &ConfigContext,
) -> WorktreeTemplateContext {
    if !result.enabled {
        return WorktreeTemplateContext::default();
    }

    let defaults = ito_core::config::resolve_worktree_template_defaults(target_path, ctx);

    let project_root = ito_config::ito_dir::absolutize_and_normalize(target_path)
        .unwrap_or_else(|_| target_path.to_path_buf())
        .to_string_lossy()
        .to_string();

    WorktreeTemplateContext {
        enabled: true,
        strategy: result.strategy.clone().unwrap_or(defaults.strategy),
        layout_dir_name: defaults.layout_dir_name,
        integration_mode: result
            .integration_mode
            .clone()
            .unwrap_or(defaults.integration_mode),
        default_branch: defaults.default_branch,
        project_root,
    }
}

#[cfg(test)]
mod repo_validation_advisory_tests {
    //! Unit tests for the `ito init` repo-validation advisory helpers.
    //!
    //! These cover the cheap, pure-filesystem helpers; full advisory
    //! triggering is exercised by integration tests that invoke `ito init`.

    use super::pre_commit_config_already_wires_ito_validate_repo as already_wired;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn returns_false_for_empty_project() {
        let tmp = TempDir::new().unwrap();
        assert!(!already_wired(tmp.path()));
    }

    #[test]
    fn detects_pre_commit_yaml_with_hook_id() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join(".pre-commit-config.yaml"),
            "repos:\n  - repo: local\n    hooks:\n      - id: ito-validate-repo\n",
        )
        .unwrap();
        assert!(already_wired(tmp.path()));
    }

    #[test]
    fn detects_husky_pre_commit_script_with_command_line() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".husky")).unwrap();
        fs::write(
            tmp.path().join(".husky").join("pre-commit"),
            "#!/usr/bin/env bash\nito validate repo --staged --strict\n",
        )
        .unwrap();
        assert!(already_wired(tmp.path()));
    }

    #[test]
    fn detects_lefthook_config() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("lefthook.yml"),
            "pre-commit:\n  commands:\n    ito-validate-repo:\n      run: ito validate repo --staged --strict\n",
        )
        .unwrap();
        assert!(already_wired(tmp.path()));
    }

    #[test]
    fn ignores_unrelated_pre_commit_yaml() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join(".pre-commit-config.yaml"),
            "repos:\n  - repo: local\n    hooks:\n      - id: cargo-fmt\n",
        )
        .unwrap();
        assert!(!already_wired(tmp.path()));
    }
}
