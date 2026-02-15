use crate::app::worktree_wizard::{
    is_worktree_configured, load_worktree_result_from_config, prompt_worktree_wizard,
    save_worktree_config, WorktreeWizardResult,
};
use crate::cli::UpdateArgs;
use crate::cli_error::{to_cli_error, CliResult};
use crate::runtime::Runtime;
use ito_config::ito_dir;
use ito_config::output;
use ito_core::installers::{install_default_templates, InitOptions, InstallMode};
use ito_templates::project_templates::WorktreeTemplateContext;
use std::collections::BTreeSet;
use std::io::IsTerminal;

pub(super) fn handle_update(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["update"], "ito update")
        );
        return Ok(());
    }

    // `--json` is accepted for parity with TS but not implemented yet.
    let _want_json = args.iter().any(|a| a == "--json");
    let target = super::common::last_positional(args).unwrap_or_else(|| ".".to_string());
    let target_path = std::path::Path::new(&target);
    let ctx = rt.ctx();

    // Resolve worktree config BEFORE template installation.
    // During update, run the wizard only if worktrees are not yet configured.
    let ui = output::resolve_ui_options(
        false,
        std::env::var("NO_COLOR").ok().as_deref(),
        false,
        std::env::var("ITO_INTERACTIVE").ok().as_deref(),
    );
    let is_tty = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let is_interactive = ui.interactive && is_tty && !args.iter().any(|a| a == "--no-interactive");

    let (worktree_ctx, post_install_save) =
        resolve_update_worktree_config(ctx, target_path, is_interactive)?;

    let tools: BTreeSet<String> = ito_core::installers::available_tool_ids()
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let opts = InitOptions::new(tools, true, false);

    install_default_templates(
        target_path,
        ctx,
        InstallMode::Update,
        &opts,
        Some(&worktree_ctx),
    )
    .map_err(to_cli_error)?;

    if let Some((path, result)) = post_install_save {
        save_worktree_config(&path, &result)?;
    }

    Ok(())
}

/// Resolve worktree configuration for `ito update`.
///
/// If the user has not configured worktrees yet and we are in interactive mode,
/// runs the wizard. Otherwise loads existing config, defaulting to disabled.
fn resolve_update_worktree_config(
    ctx: &ito_config::ConfigContext,
    target_path: &std::path::Path,
    interactive: bool,
) -> CliResult<(
    WorktreeTemplateContext,
    Option<(std::path::PathBuf, WorktreeWizardResult)>,
)> {
    let ito_path = ito_dir::get_ito_path(target_path, ctx);
    let project_config_path = ito_path.join("config.json");
    let project_local_config_path = ito_path.join("config.local.json");
    let global_config_path = ito_config::global_config_path(ctx);

    let local_configured = is_worktree_configured(&project_local_config_path);
    let project_configured = is_worktree_configured(&project_config_path);
    let global_configured = global_config_path
        .as_ref()
        .is_some_and(|p| is_worktree_configured(p));

    // Wizard runs only when neither project nor global worktree config exists.
    let (result, should_save_to_project) =
        if interactive && !local_configured && !project_configured && !global_configured {
            (prompt_worktree_wizard()?, true)
        } else if local_configured {
            (
                load_worktree_result_from_config(&project_local_config_path),
                false,
            )
        } else if project_configured {
            (
                load_worktree_result_from_config(&project_config_path),
                false,
            )
        } else if global_configured {
            // Backward compatibility: load from global config, then migrate to project.
            let global_path = global_config_path
                .as_ref()
                .expect("global_configured implies global_config_path");
            (load_worktree_result_from_config(global_path), true)
        } else {
            (
                WorktreeWizardResult {
                    ran: false,
                    enabled: false,
                    strategy: None,
                    integration_mode: None,
                },
                false,
            )
        };

    let defaults = ito_core::config::resolve_worktree_template_defaults(target_path, ctx);
    // Derive project root from the already-absolute ito_path rather than
    // target_path which could be a relative subdirectory reference.
    // Defensive absolutize in case get_ito_path contract changes.
    let project_root_path = ito_path.parent().unwrap_or(&ito_path);
    let project_root = ito_config::ito_dir::absolutize_and_normalize(project_root_path)
        .unwrap_or_else(|_| project_root_path.to_path_buf())
        .to_string_lossy()
        .to_string();

    let ctx_out = if result.enabled {
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
    } else {
        WorktreeTemplateContext::default()
    };

    // Save to the per-developer overlay so two developers can have different
    // worktree workflows without churn in committed config.
    let post_install_save = should_save_to_project.then_some((project_local_config_path, result));
    Ok((ctx_out, post_install_save))
}

pub(crate) fn handle_update_clap(rt: &Runtime, args: &UpdateArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();
    if args.json {
        argv.push("--json".to_string());
    }
    if let Some(path) = &args.path {
        argv.push(path.clone());
    }
    handle_update(rt, &argv)
}
