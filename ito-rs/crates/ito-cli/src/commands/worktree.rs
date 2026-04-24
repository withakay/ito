//! Handler for `ito worktree <sub-command>`.

use ito_config::types::ItoConfig;
use ito_config::{ConfigContext, load_cascading_project_config};
use ito_core::repo_paths::{resolve_env, resolve_worktree_paths};
use ito_core::worktree_ensure::ensure_worktree;
use ito_core::worktree_init::run_worktree_setup;

use crate::cli::{WorktreeArgs, WorktreeCommand};
use crate::cli_error::{CliError, CliResult};
use crate::runtime::Runtime;

/// Dispatch `ito worktree` sub-commands.
pub(crate) fn handle_worktree_clap(rt: &Runtime, args: &WorktreeArgs) -> CliResult<()> {
    match &args.command {
        WorktreeCommand::Ensure(change_args) => handle_ensure(rt, &change_args.change),
        WorktreeCommand::Setup(change_args) => handle_setup(rt, &change_args.change),
    }
}

/// Handle `ito worktree ensure --change <id>`.
///
/// Prints the resolved worktree path to stdout (a single absolute path on one
/// line). All progress/informational output goes to stderr.
fn handle_ensure(rt: &Runtime, change_id: &str) -> CliResult<()> {
    let env = resolve_env(rt.ctx()).map_err(|e| CliError::msg(e.to_string()))?;
    let worktree_paths =
        resolve_worktree_paths(&env, rt.ctx()).map_err(|e| CliError::msg(e.to_string()))?;

    let config = load_resolved_config(&env.worktree_root, &env.ito_root, rt.ctx())?;

    let cwd = rt.cwd();

    let path = ensure_worktree(change_id, &config, &env, &worktree_paths, cwd)
        .map_err(|e| CliError::msg(e.to_string()))?;

    // Only the path goes to stdout — everything else goes to stderr.
    println!("{}", path.display());

    Ok(())
}

/// Handle `ito worktree setup --change <id>`.
///
/// Re-runs setup commands in an existing worktree. Errors if the worktree
/// does not exist.
fn handle_setup(rt: &Runtime, change_id: &str) -> CliResult<()> {
    let env = resolve_env(rt.ctx()).map_err(|e| CliError::msg(e.to_string()))?;
    let worktree_paths =
        resolve_worktree_paths(&env, rt.ctx()).map_err(|e| CliError::msg(e.to_string()))?;

    let config = load_resolved_config(&env.worktree_root, &env.ito_root, rt.ctx())?;

    use ito_core::repo_paths::WorktreeSelector;
    let selector = WorktreeSelector::Change(change_id.to_string());
    let Some(worktree_path) = worktree_paths.path_for_selector(&selector) else {
        return Err(CliError::msg(format!(
            "Cannot resolve worktree path for change '{change_id}'.\n\
             Worktrees may not be enabled. Check 'worktrees.enabled' in .ito/config.json.",
        )));
    };

    if !worktree_path.is_dir() {
        return Err(CliError::msg(format!(
            "Worktree for change '{change_id}' does not exist at '{}'.\n\
             Run `ito worktree ensure --change {change_id}` first to create it.",
            worktree_path.display(),
        )));
    }

    if config.worktrees.init.setup.is_none() {
        eprintln!("No setup commands configured in worktrees.init.setup — nothing to do.");
        return Ok(());
    }

    run_worktree_setup(&worktree_path, &config.worktrees)
        .map_err(|e| CliError::msg(e.to_string()))?;

    eprintln!("Setup complete for change '{change_id}'.");

    Ok(())
}

/// Load and deserialize the resolved Ito config.
fn load_resolved_config(
    worktree_root: &std::path::Path,
    ito_root: &std::path::Path,
    ctx: &ConfigContext,
) -> CliResult<ItoConfig> {
    let cfg_value = load_cascading_project_config(worktree_root, ito_root, ctx).merged;
    let config: ItoConfig = serde_json::from_value(cfg_value).map_err(|e| {
        CliError::msg(format!(
            "Cannot parse Ito configuration: {e}\n\
             Fix: check .ito/config.json for syntax errors.",
        ))
    })?;
    Ok(config)
}
