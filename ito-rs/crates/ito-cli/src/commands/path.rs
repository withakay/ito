use crate::cli::{PathArgs, PathCommand, PathCommonArgs, PathRootsArgs, PathWorktreeArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_config::ito_dir::{absolutize_and_normalize, get_ito_path, lexical_normalize};
use ito_config::{load_cascading_project_config, types::ItoConfig};
use std::path::{Path, PathBuf};

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
/// # // Construct minimal example values matching the function's expected types.
/// # let env = ResolvedEnv {
/// #     worktree_root: PathBuf::from("/repo"),
/// #     project_root: PathBuf::from("/repo"),
/// #     ito_root: PathBuf::from("/repo/.ito"),
/// #     is_bare_git_repo: false,
/// # };
/// # let wt = Some(ResolvedWorktreePaths {
/// #     enabled: true,
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

/// Resolve repository and Ito-related roots for the current working directory.
///
/// Determines the worktree root, project root, Ito directory, and whether the
/// current repository is a bare Git repository, and returns them in a
/// `ResolvedEnv`.
///
/// The returned `ResolvedEnv` contains:
/// - `worktree_root`: the chosen worktree directory (Git top-level if inside a
///   worktree, otherwise the nearest Ito root or the current directory), normalized
///   to an absolute path when possible.
/// - `project_root`: the repository's common project root (when available) or the
///   worktree root, normalized to an absolute path when possible.
/// - `ito_root`: the resolved Ito directory for the worktree (must exist or the
///   function will return an error).
/// - `is_bare_git_repo`: `true` when the current repository is detected as a bare
///   Git repository, `false` otherwise.
///
/// # Returns
///
/// A `CliResult<ResolvedEnv>` containing the resolved environment on success,
/// or a CLI error if the Ito directory cannot be found or other validation fails.
///
/// # Examples
///
/// ```no_run
/// use crate::runtime::Runtime;
///
/// // Construct a runtime (example; actual construction may vary).
/// let rt = Runtime::new();
/// let env = crate::commands::path::resolve_env(&rt).expect("failed to resolve env");
/// println!("ito_root: {}", env.ito_root.display());
/// ```
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

/// Builds an error message indicating Ito must be run from a git worktree, using the environment's project root to produce a helpful hint.
///
/// # Returns
///
/// An error message string advising the user that Ito must be run from a git worktree and suggesting a `cd` hint derived from the resolved project root.
///
/// # Examples
///
/// ```
/// let env = ResolvedEnv {
///     worktree_root: std::path::PathBuf::from("/repo"),
///     project_root: std::path::PathBuf::from("/repo"),
///     ito_root: std::path::PathBuf::from("/repo/.ito"),
///     is_bare_git_repo: true,
/// };
/// let msg = bare_repo_error_message(&env);
/// assert!(msg.contains("Ito must be run from a git worktree"));
/// ```
fn bare_repo_error_message(env: &ResolvedEnv) -> String {
    bare_repo_error_message_raw(&env.project_root)
}

/// Constructs a user-facing error message indicating Ito must be run from a git worktree.
///
/// Chooses a best-effort shell hint: if a `main` subdirectory exists under `cwd` it suggests
/// `cd "<cwd>/main"`, otherwise if `master` exists it suggests that, and if neither exist it
/// suggests the generic `cd <worktree-dir>`. The returned `String` contains the message and hint.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let msg = super::bare_repo_error_message_raw(Path::new("/does/not/exist"));
/// assert!(msg.contains("Ito must be run from a git worktree"));
/// ```
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

/// Searches upward from `start` for the nearest ancestor directory that contains a `.ito` subdirectory.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::path::Path;
/// use std::env;
///
/// // Create a temporary directory structure: {temp}/ito_find_example/project/.ito
/// let base = env::temp_dir().join("ito_find_example");
/// let project = base.join("project");
/// let ito_dir = project.join(".ito");
/// let _ = fs::remove_dir_all(&base); // ignore errors if not present
/// fs::create_dir_all(&ito_dir).unwrap();
///
/// // Start search from the project directory; should find `project` as the nearest Ito root.
/// let found = crate::commands::path::find_nearest_ito_root(Path::new(&project));
/// assert_eq!(found.unwrap(), project);
///
/// // Clean up
/// let _ = fs::remove_dir_all(&base);
/// ```
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

/// Obtain the repository top-level directory as an absolute path for a Git repository at or above `cwd`.
///
/// # Returns
///
/// `Some(PathBuf)` containing the absolute top-level path if Git recognizes a repository at or above `cwd`, `None` otherwise.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let top = crate::git_show_toplevel(Path::new("."));
/// match top {
///     Some(path) => println!("repo top: {}", path.display()),
///     None => println!("not inside a git repository"),
/// }
/// ```
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

/// Determines the repository's common root directory for the repository containing `worktree_root`.
///
/// Returns the parent directory of Git's common directory (the repository root) when Git can
/// report it for the given path, or `None` if Git does not provide a common directory.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// // Returns `Some(path)` when `path` is inside a Git repository; otherwise `None`.
/// let root = crate::git_common_root(Path::new("."));
/// println!("{:?}", root);
/// ```
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

/// Determines whether the Git repository at the specified path is a bare repository.
///
/// # Returns
///
/// `Some(true)` if Git reports the repository is bare, `Some(false)` if Git reports it is not,
/// or `None` if the command output could not be parsed or the Git command failed.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // Inspect current directory's repository (if any).
/// if let Some(is_bare) = git_is_bare_repo(Path::new(".")) {
///     println!("bare: {}", is_bare);
/// } else {
///     eprintln!("could not determine repository bare status");
/// }
/// ```
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

/// Execute a `git` command in the given directory and return its stdout when the command exits successfully.

///

/// This function runs `git` with the provided arguments in `cwd`, removing any environment variables

/// that start with `GIT_` for the child process to avoid injected behavior. If the command exits with

/// a non-zero status or cannot be spawned, `None` is returned.

///

/// # Examples

///

/// ```

/// use std::path::Path;

/// let out = crate::git_output(Path::new("."), &["--version"]).expect("git available");

/// assert!(out.to_lowercase().contains("git version"));

/// ```
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
    /// Resolve the filesystem path corresponding to a worktree selector when worktrees are enabled.
    ///
    /// Returns `Some(PathBuf)` with the selected worktree path when worktrees are enabled and a path
    /// can be determined for the given selector, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// let paths = ResolvedWorktreePaths {
    ///     enabled: true,
    ///     strategy: WorktreeStrategy::CheckoutSubdir,
    ///     worktrees_root: Some(PathBuf::from("/repo/worktrees")),
    ///     main_worktree_root: Some(PathBuf::from("/repo")),
    /// };
    ///
    /// let branch_path = paths.path_for_selector(&WorktreeSelector::Branch("feature".into()));
    /// assert_eq!(branch_path, Some(PathBuf::from("/repo/worktrees/feature")));
    ///
    /// let main_path = paths.path_for_selector(&WorktreeSelector::Main);
    /// assert_eq!(main_path, Some(PathBuf::from("/repo")));
    /// ```
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

/// Computes worktree layout paths from the resolved environment and repository-local Ito configuration.
///
/// Reads the merged Ito project configuration (relative to `env.worktree_root` and `env.ito_root`),
/// evaluates the `worktrees` configuration (enabled flag, strategy, layout, and default branch),
/// resolves the effective base directory, and returns the derived `worktrees_root` and
/// `main_worktree_root` according to the configured strategy. If worktrees are disabled, both
/// roots are returned as `None`.
///
/// # Parameters
///
/// `env` - Resolved repository environment used to locate project and Ito roots.
///
/// # Returns
///
/// `ResolvedWorktreePaths` containing `enabled`, `strategy`, an optional `worktrees_root`, and an optional `main_worktree_root`.
///
/// # Examples
///
/// ```rust
/// # // Example disabled from running because it depends on repository-local config.
/// # use std::path::PathBuf;
/// # use crate::commands::path::{ResolvedEnv, resolve_worktree_paths};
/// # // Construct a minimal ResolvedEnv for illustration.
/// # let cwd = std::env::current_dir().unwrap();
/// # let env = ResolvedEnv {
/// #     worktree_root: cwd.clone(),
/// #     project_root: cwd.clone(),
/// #     ito_root: cwd,
/// #     is_bare_git_repo: false,
/// # };
/// let _paths = resolve_worktree_paths(&env).unwrap();
/// ```
fn resolve_worktree_paths(env: &ResolvedEnv) -> CliResult<ResolvedWorktreePaths> {
    let ctx = ito_config::ConfigContext::from_process_env();
    // Load config relative to the current worktree root so repo-local sources
    // like `ito.json` and `.ito.json` resolve within the working checkout.
    let cfg = load_cascading_project_config(&env.worktree_root, &env.ito_root, &ctx);
    let typed: ItoConfig = serde_json::from_value(cfg.merged)
        .map_err(|e| to_cli_error(format!("Failed to parse Ito configuration: {e}")))?;

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

/// Resolve the base directory for worktree layouts.
///
/// If `configured` is `None` or an empty/whitespace string, returns `env.project_root`.
/// Otherwise treats `configured` as a path: if it's absolute, that path is used;
/// if it's relative, it is resolved against `env.project_root`. The resulting
/// path is normalized and absolutized; if that process fails, a lexical
/// normalization of the computed path is returned.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// // Construct a minimal ResolvedEnv for the example.
/// let env = ResolvedEnv {
///     worktree_root: PathBuf::from("/tmp/proj/.git/worktree"),
///     project_root: PathBuf::from("/tmp/proj"),
///     ito_root: PathBuf::from("/tmp/proj/.ito"),
///     is_bare_git_repo: false,
/// };
///
/// // No configured base -> fallback to project_root
/// assert_eq!(resolve_base_dir(&env, &None), PathBuf::from("/tmp/proj"));
///
/// // Relative configured path -> resolved against project_root
/// assert_eq!(
///     resolve_base_dir(&env, &Some("sub/dir".to_string())),
///     env.project_root.join("sub/dir")
/// );
///
/// // Absolute configured path -> used as-is
/// assert_eq!(
///     resolve_base_dir(&env, &Some("/var/data".to_string())),
///     PathBuf::from("/var/data")
/// );
/// ```
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