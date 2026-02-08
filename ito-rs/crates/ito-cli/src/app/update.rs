use crate::app::worktree_wizard::{
    is_worktree_configured, load_worktree_result_from_config, run_worktree_wizard,
};
use crate::cli::UpdateArgs;
use crate::cli_error::{CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_config::output;
use ito_core::installers::{InitOptions, InstallMode, install_default_templates};
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

    let worktree_ctx = resolve_update_worktree_config(ctx, is_interactive);

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

    Ok(())
}

/// Resolve worktree configuration for `ito update`.
///
/// If the user has not configured worktrees yet and we are in interactive mode,
/// runs the wizard. Otherwise loads existing config, defaulting to disabled.
fn resolve_update_worktree_config(
    ctx: &ito_config::ConfigContext,
    interactive: bool,
) -> WorktreeTemplateContext {
    let Some(config_path) = ito_config::global_config_path(ctx) else {
        return WorktreeTemplateContext::default();
    };

    // Interactive + not yet configured: run the wizard.
    if interactive && !is_worktree_configured(&config_path) {
        if let Some(parent) = config_path.parent() {
            let _ = ito_common::io::create_dir_all_std(parent);
        }
        if let Ok(result) = run_worktree_wizard(&config_path) {
            if result.enabled {
                return WorktreeTemplateContext {
                    enabled: true,
                    strategy: result.strategy.unwrap_or_default(),
                    layout_dir_name: "ito-worktrees".to_string(),
                    integration_mode: result.integration_mode.unwrap_or_default(),
                    default_branch: "main".to_string(),
                };
            }
            return WorktreeTemplateContext::default();
        }
    }

    // Non-interactive or already configured: load from config.
    let result = load_worktree_result_from_config(&config_path);
    if result.enabled {
        return WorktreeTemplateContext {
            enabled: true,
            strategy: result.strategy.unwrap_or_default(),
            layout_dir_name: "ito-worktrees".to_string(),
            integration_mode: result.integration_mode.unwrap_or_default(),
            default_branch: "main".to_string(),
        };
    }

    WorktreeTemplateContext::default()
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
