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
use ito_config::{
    ConfigContext, load_cascading_project_config, resolve_coordination_branch_settings,
};
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
/// when `--setup-coordination-branch` is given, and prints post‑initialization guidance.
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
    let update = args.iter().any(|a| a == "--update" || a == "-u");
    let setup_coordination_branch = args.iter().any(|a| a == "--setup-coordination-branch");
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

        let tool_items: Vec<(&'static str, &str)> = vec![
            (ito_core::installers::TOOL_CLAUDE, "Claude Code"),
            (ito_core::installers::TOOL_CODEX, "Codex"),
            (ito_core::installers::TOOL_GITHUB_COPILOT, "GitHub Copilot"),
            (ito_core::installers::TOOL_OPENCODE, "OpenCode"),
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

    let (worktree_result, worktree_project_config_path, should_persist_worktree) =
        resolve_worktree_config(ctx, target_path, is_interactive)?;
    let worktree_ctx = worktree_template_context(&worktree_result, target_path, ctx);

    let opts = InitOptions::new(tools, force, update);
    install_default_templates(
        target_path,
        ctx,
        InstallMode::Init,
        &opts,
        Some(&worktree_ctx),
    )
    .map_err(to_cli_error)?;

    if should_persist_worktree {
        save_worktree_config(&worktree_project_config_path, &worktree_result)?;
    }

    if setup_coordination_branch {
        let ito_path = ito_dir::get_ito_path(target_path, ctx);
        let project_root = ito_path.parent().ok_or_else(|| CliError::msg(format!("Could not determine project root from Ito path: {}", ito_path.display())))?;
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

    print_post_init_guidance(target_path);

    Ok(())
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
/// print_post_init_guidance(Path::new("."));
/// ```
fn print_post_init_guidance(target_path: &std::path::Path) {
    let target_display = if target_path == std::path::Path::new(".") {
        "current directory".to_string()
    } else {
        format!("{}", target_path.display())
    };

    println!(
        r#"
Ito initialized in {target_display}

Next step: Run /ito-project-setup in your AI assistant to configure the project.

Or manually edit:
  .ito/project.md        Project overview, tech stack, architecture
  .ito/user-prompts/     Shared + artifact-specific instruction guidance
  .ito/config.json       Tool settings and defaults

Learn more: ito --help | ito agent instruction --help
"#
    );
}

/// Convert parsed `InitArgs` into CLI-style argv, optionally override `HOME`, and run the init flow.
///
/// If `args.home` is provided, the `HOME` environment variable is set to that value. The function
/// translates the present `tools`, `force`, `update`, `setup_coordination_branch`, and `path` fields
/// into their corresponding CLI flags and arguments, then delegates to `handle_init`.
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
///     force: true,
///     update: false,
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
    if args.setup_coordination_branch {
        argv.push("--setup-coordination-branch".to_string());
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

/// Convert a [`WorktreeWizardResult`] into a [`WorktreeTemplateContext`].
///
/// Maps the wizard's raw string fields into the context struct that templates
/// consume. Falls back to the disabled default when the wizard result indicates
/// worktrees are not enabled.
fn worktree_template_context(
    result: &WorktreeWizardResult,
    target_path: &std::path::Path,
    ctx: &ConfigContext,
) -> WorktreeTemplateContext {
    if !result.enabled {
        return WorktreeTemplateContext::default();
    }

    let defaults = ito_core::config::resolve_worktree_template_defaults(target_path, ctx);

    WorktreeTemplateContext {
        enabled: true,
        strategy: result.strategy.clone().unwrap_or(defaults.strategy),
        layout_dir_name: defaults.layout_dir_name,
        integration_mode: result
            .integration_mode
            .clone()
            .unwrap_or(defaults.integration_mode),
        default_branch: defaults.default_branch,
    }
}