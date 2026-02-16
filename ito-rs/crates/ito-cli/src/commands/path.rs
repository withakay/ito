use crate::cli::{PathArgs, PathCommand, PathCommonArgs, PathRootsArgs, PathWorktreeArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_config::ito_dir::{absolutize_and_normalize, get_ito_path, lexical_normalize};
use ito_config::{load_cascading_project_config, types::ItoConfig};
use std::path::{Path, PathBuf};

pub(crate) fn handle_path_clap(rt: &Runtime, args: &PathArgs) -> CliResult<()> {
    let Some(cmd) = &args.command else {
        return fail("Missing required subcommand");
    };

    match cmd {
        PathCommand::ProjectRoot(common) => {
            let env = resolve_env(rt)?;
            print_path(&env.project_root, common)
        }
        PathCommand::WorktreeRoot(common) => {
            let env = resolve_env(rt)?;
            if env.is_bare_git_repo {
                return fail(bare_repo_error_message(&env));
            }
            print_path(&env.worktree_root, common)
        }
        PathCommand::ItoRoot(common) => {
            let env = resolve_env(rt)?;
            print_path(&env.ito_root, common)
        }
        PathCommand::WorktreesRoot(common) => {
            let env = resolve_env(rt)?;
            let paths = resolve_worktree_paths(&env)?;
            let Some(worktrees_root) = paths.worktrees_root else {
                return fail("Worktrees are not enabled for this project");
            };
            print_path(&worktrees_root, common)
        }
        PathCommand::Worktree(args) => {
            let env = resolve_env(rt)?;
            let paths = resolve_worktree_paths(&env)?;
            let selector = selector_from_args(args)?;
            let Some(out) = paths.path_for_selector(&selector) else {
                return fail("Worktrees are not enabled for this project");
            };
            print_path(&out, &args.common)
        }
        PathCommand::Roots(args) => {
            let env = resolve_env(rt)?;
            let worktree_paths = resolve_worktree_paths(&env).ok();
            print_roots(&env, worktree_paths.as_ref(), args)
        }
    }
}

#[derive(Debug, Clone)]
enum WorktreeSelector {
    Main,
    Branch(String),
    Change(String),
}

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
            "enabled": wt.map(|w| w.enabled),
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
        println!("worktrees_enabled: {}", wt.enabled);
        println!("worktree_strategy: {}", wt.strategy.as_str());
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct ResolvedEnv {
    worktree_root: PathBuf,
    project_root: PathBuf,
    ito_root: PathBuf,
    is_bare_git_repo: bool,
}

fn resolve_env(rt: &Runtime) -> CliResult<ResolvedEnv> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let is_bare = git_is_bare_repo(&cwd).unwrap_or(false);

    // If we're inside a git worktree, prefer the actual worktree root.
    let worktree_root = git_show_toplevel(&cwd)
        .unwrap_or_else(|| find_nearest_ito_root(&cwd).unwrap_or_else(|| cwd.clone()));
    let worktree_root = absolutize_and_normalize(&worktree_root)
        .unwrap_or_else(|_| lexical_normalize(&worktree_root));

    let ito_root = get_ito_path(&worktree_root, rt.ctx());
    if !ito_root.is_dir() {
        if is_bare {
            return fail(bare_repo_error_message_raw(&cwd));
        }
        return fail(format!(
            "No Ito directory found (expected {}). Run `ito init <project-dir>` or cd into an initialized worktree.",
            ito_root.to_string_lossy()
        ));
    }

    let project_root = git_common_root(&worktree_root).unwrap_or_else(|| worktree_root.clone());
    let project_root = absolutize_and_normalize(&project_root)
        .unwrap_or_else(|_| lexical_normalize(&project_root));

    Ok(ResolvedEnv {
        worktree_root,
        project_root,
        ito_root,
        is_bare_git_repo: is_bare,
    })
}

fn bare_repo_error_message(env: &ResolvedEnv) -> String {
    bare_repo_error_message_raw(&env.project_root)
}

fn bare_repo_error_message_raw(cwd: &Path) -> String {
    // Best-effort hint: in bare/control layouts, `main/` is typically the primary worktree.
    let main = cwd.join("main");
    let master = cwd.join("master");
    let hint = if main.is_dir() {
        format!("cd \"{}\"", main.to_string_lossy())
    } else if master.is_dir() {
        format!("cd \"{}\"", master.to_string_lossy())
    } else {
        "cd <worktree-dir>".to_string()
    };

    format!("Ito must be run from a git worktree (not the bare repository). Try: {hint}")
}

fn find_nearest_ito_root(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join(".ito").is_dir() {
            return Some(cur);
        }
        let parent = cur.parent()?.to_path_buf();
        cur = parent;
    }
}

fn git_show_toplevel(cwd: &Path) -> Option<PathBuf> {
    let out = git_output(
        cwd,
        &["rev-parse", "--path-format=absolute", "--show-toplevel"],
    )?;
    let out = out.trim();
    if out.is_empty() {
        return None;
    }
    Some(PathBuf::from(out))
}

fn git_common_root(worktree_root: &Path) -> Option<PathBuf> {
    let common = git_output(
        worktree_root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    let common = common.trim();
    if common.is_empty() {
        return None;
    }
    Path::new(common).parent().map(Path::to_path_buf)
}

fn git_is_bare_repo(cwd: &Path) -> Option<bool> {
    let out = git_output(cwd, &["rev-parse", "--is-bare-repository"])?;
    let out = out.trim().to_ascii_lowercase();
    if out == "true" {
        return Some(true);
    }
    if out == "false" {
        return Some(false);
    }
    None
}

fn git_output(cwd: &Path, args: &[&str]) -> Option<String> {
    let mut command = std::process::Command::new("git");
    command.args(args).current_dir(cwd);

    // Ignore injected git environment variables to avoid surprises.
    for (k, _v) in std::env::vars_os() {
        let k = k.to_string_lossy();
        if k.starts_with("GIT_") {
            command.env_remove(k.as_ref());
        }
    }

    let output = command.output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

#[derive(Debug, Clone)]
struct ResolvedWorktreePaths {
    enabled: bool,
    strategy: ito_config::types::WorktreeStrategy,
    worktrees_root: Option<PathBuf>,
    main_worktree_root: Option<PathBuf>,
}

impl ResolvedWorktreePaths {
    fn path_for_selector(&self, selector: &WorktreeSelector) -> Option<PathBuf> {
        if !self.enabled {
            return None;
        }

        match selector {
            WorktreeSelector::Main => self.main_worktree_root.clone(),
            WorktreeSelector::Branch(branch) => {
                self.worktrees_root.as_ref().map(|p| p.join(branch))
            }
            WorktreeSelector::Change(change) => {
                self.worktrees_root.as_ref().map(|p| p.join(change))
            }
        }
    }
}

fn resolve_worktree_paths(env: &ResolvedEnv) -> CliResult<ResolvedWorktreePaths> {
    let ctx = ito_config::ConfigContext::from_process_env();
    // Load config relative to the current worktree root so repo-local sources
    // like `ito.json` and `.ito.json` resolve within the working checkout.
    let cfg = load_cascading_project_config(&env.worktree_root, &env.ito_root, &ctx);
    let typed: ItoConfig = serde_json::from_value(cfg.merged).unwrap_or_default();

    let wt = typed.worktrees;
    let enabled = wt.enabled;
    let strategy = wt.strategy;
    let default_branch = wt.default_branch;
    let dir_name = wt.layout.dir_name;

    let base = resolve_base_dir(env, &wt.layout.base_dir);

    let (worktrees_root, main_worktree_root) = if enabled {
        match strategy {
            ito_config::types::WorktreeStrategy::CheckoutSubdir => {
                let wt_root = base.join(format!(".{dir_name}"));
                (Some(wt_root), Some(base))
            }
            ito_config::types::WorktreeStrategy::CheckoutSiblings => {
                let project_name = base
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("project");
                let parent = base.parent().unwrap_or(&base);
                let wt_root = parent.join(format!("{project_name}-{dir_name}"));
                (Some(wt_root), Some(base))
            }
            ito_config::types::WorktreeStrategy::BareControlSiblings => {
                let wt_root = base.join(&dir_name);
                let main = base.join(&default_branch);
                (Some(wt_root), Some(main))
            }
        }
    } else {
        (None, None)
    };

    Ok(ResolvedWorktreePaths {
        enabled,
        strategy,
        worktrees_root,
        main_worktree_root,
    })
}

fn resolve_base_dir(env: &ResolvedEnv, configured: &Option<String>) -> PathBuf {
    let Some(raw) = configured
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    else {
        return env.project_root.clone();
    };

    let p = PathBuf::from(raw);
    let p = if p.is_absolute() {
        p
    } else {
        env.project_root.join(p)
    };
    absolutize_and_normalize(&p).unwrap_or_else(|_| lexical_normalize(&p))
}
