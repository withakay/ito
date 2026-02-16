use crate::cli::{PathArgs, PathCommand, PathCommonArgs, PathRootsArgs, PathWorktreeArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_core::repo_paths::{
    ResolvedEnv, ResolvedWorktreePaths, WorktreeSelector, resolve_env, resolve_worktree_paths,
};
use std::path::Path;

/// Dispatches the path-related CLI subcommands, resolves repository and Ito environment state, and prints the requested filesystem path(s).
///
/// This function handles each `PathCommand` variant by resolving the repository/Ito environment, computing any worktree paths when required, and emitting the selected path(s) or a structured roots summary. It will produce user-facing errors when a required selector is missing, when invoked from a bare Git repository for commands that require a worktree, or when worktrees are not enabled but requested.
///
/// # Returns
///
/// `CliResult<()>` — `Ok(())` on success, or an error describing why the requested path(s) could not be resolved.
///
/// # Examples
///
/// ```no_run
/// // Typical usage from a CLI entrypoint:
/// let rt = Runtime::new();
/// let args = PathArgs { command: Some(PathCommand::ProjectRoot(PathCommonArgs { json: false })) };
/// handle_path_clap(&rt, &args)?;
/// ```
pub(crate) fn handle_path_clap(rt: &Runtime, args: &PathArgs) -> CliResult<()> {
    let Some(cmd) = &args.command else {
        return fail("Missing required subcommand");
    };

    match cmd {
        PathCommand::ProjectRoot(common) => {
            let env = resolve_env(rt.ctx()).map_err(to_cli_error)?;
            print_path(&env.project_root, common)
        }
        PathCommand::WorktreeRoot(common) => {
            let env = resolve_env(rt.ctx()).map_err(to_cli_error)?;
            print_path(&env.worktree_root, common)
        }
        PathCommand::ItoRoot(common) => {
            let env = resolve_env(rt.ctx()).map_err(to_cli_error)?;
            print_path(&env.ito_root, common)
        }
        PathCommand::WorktreesRoot(common) => {
            let env = resolve_env(rt.ctx()).map_err(to_cli_error)?;
            let paths = resolve_worktree_paths(&env, rt.ctx()).map_err(to_cli_error)?;
            let Some(worktrees_root) = paths.worktrees_root else {
                return fail("Worktrees are not enabled for this project");
            };
            print_path(&worktrees_root, common)
        }
        PathCommand::Worktree(args) => {
            let env = resolve_env(rt.ctx()).map_err(to_cli_error)?;
            let paths = resolve_worktree_paths(&env, rt.ctx()).map_err(to_cli_error)?;
            let selector = selector_from_args(args)?;
            let Some(out) = paths.path_for_selector(&selector) else {
                return fail("Worktrees are not enabled for this project");
            };
            print_path(&out, &args.common)
        }
        PathCommand::Roots(args) => {
            let env = resolve_env(rt.ctx()).map_err(to_cli_error)?;
            let worktree_paths = resolve_worktree_paths(&env, rt.ctx()).ok();
            print_roots(&env, worktree_paths.as_ref(), args)
        }
    }
}

/// Determine which worktree selector the given CLI arguments specify.
///
/// # Returns
///
/// `Ok(WorktreeSelector)` corresponding to the first selector flag found (`--main`, `--branch`, or `--change`), or an `Err` if no selector was provided.
///
/// # Examples
///
/// ```
/// let args = PathWorktreeArgs { main: true, branch: None, change: None };
/// let sel = selector_from_args(&args).unwrap();
/// assert!(matches!(sel, WorktreeSelector::Main));
/// ```
fn selector_from_args(args: &PathWorktreeArgs) -> CliResult<WorktreeSelector> {
    if args.main {
        return Ok(WorktreeSelector::Main);
    }
    if let Some(b) = &args.branch {
        return Ok(WorktreeSelector::Branch(b.clone()));
    }
    if let Some(c) = &args.change {
        return Ok(WorktreeSelector::Change(c.clone()));
    }
    fail("Missing selector (use one of: --main, --branch <name>, --change <id>)")
}

/// Render and print a filesystem path, using JSON output when requested.
///
/// When `common.json` is true, prints a pretty-printed JSON object of the form:
/// `{"path": "<rendered>"}`. Otherwise prints the rendered path as a plain string.
///
/// # Parameters
///
/// - `path`: the path to render and print.
/// - `common`: controls output formatting; when `common.json` is true, JSON output is produced.
///
/// # Errors
///
/// Returns a `CliResult::Err` if JSON serialization fails.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # // `PathCommonArgs` and `print_path` are assumed to be available in scope for this example.
/// let path = Path::new("/tmp/example");
/// let common = PathCommonArgs { json: false };
/// let _ = print_path(path, &common);
/// ```
fn print_path(path: &Path, common: &PathCommonArgs) -> CliResult<()> {
    let rendered = path.to_string_lossy().to_string();
    if common.json {
        let v = serde_json::json!({ "path": rendered });
        let rendered = serde_json::to_string_pretty(&v).map_err(to_cli_error)?;
        println!("{rendered}");
        return Ok(());
    }
    println!("{rendered}");
    Ok(())
}

/// Prints project, worktree, and Ito root paths.
///
/// When `args.common.json` is true, emits a JSON object with the fields
/// `projectRoot`, `worktreeRoot`, `itoRoot`, and, when available, `worktreesRoot`,
/// `mainWorktreeRoot`, `strategy`, and `enabled`. When `args.common.json` is false,
/// prints a human-readable list of the same information. If `wt` is `None`,
/// worktree-specific fields are omitted (or not printed).
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// # use ito_core::repo_paths::{GitRepoKind, ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature};
/// # use ito_config::types::WorktreeStrategy;
/// # // Construct minimal example values matching the function's expected types.
/// # let env = ResolvedEnv {
/// #     worktree_root: PathBuf::from("/repo"),
/// #     project_root: PathBuf::from("/repo"),
/// #     ito_root: PathBuf::from("/repo/.ito"),
/// #     git_repo_kind: GitRepoKind::NonBare,
/// # };
/// # let wt = Some(ResolvedWorktreePaths {
/// #     feature: WorktreeFeature::Enabled,
/// #     strategy: WorktreeStrategy::CheckoutSubdir,
/// #     worktrees_root: Some(PathBuf::from("/repo/.worktrees")),
/// #     main_worktree_root: Some(PathBuf::from("/repo/main")),
/// # });
/// # let args = PathRootsArgs { common: PathCommonArgs { json: false } };
/// // Print human-readable roots
/// let _ = print_roots(&env, wt.as_ref(), &args);
/// ```
fn print_roots(
    env: &ResolvedEnv,
    wt: Option<&ResolvedWorktreePaths>,
    args: &PathRootsArgs,
) -> CliResult<()> {
    if args.common.json {
        let v = serde_json::json!({
            "projectRoot": env.project_root.to_string_lossy(),
            "worktreeRoot": env.worktree_root.to_string_lossy(),
            "itoRoot": env.ito_root.to_string_lossy(),
            "worktreesRoot": wt.and_then(|w| w.worktrees_root.as_ref()).map(|p| p.to_string_lossy().to_string()),
            "mainWorktreeRoot": wt.and_then(|w| w.main_worktree_root.as_ref()).map(|p| p.to_string_lossy().to_string()),
            "strategy": wt.map(|w| w.strategy.as_str()),
            "enabled": wt.map(|w| w.feature.is_enabled()),
        });
        let rendered = serde_json::to_string_pretty(&v).map_err(to_cli_error)?;
        println!("{rendered}");
        return Ok(());
    }

    println!("project_root: {}", env.project_root.to_string_lossy());
    println!("worktree_root: {}", env.worktree_root.to_string_lossy());
    println!("ito_root: {}", env.ito_root.to_string_lossy());
    if let Some(wt) = wt {
        if let Some(p) = &wt.worktrees_root {
            println!("worktrees_root: {}", p.to_string_lossy());
        } else {
            println!("worktrees_root: (none)");
        }
        if let Some(p) = &wt.main_worktree_root {
            println!("main_worktree_root: {}", p.to_string_lossy());
        }
        println!("worktrees_enabled: {}", wt.feature.is_enabled());
        println!("worktree_strategy: {}", wt.strategy.as_str());
    }
    Ok(())
}
