//! Repository and worktree path resolution.
//!
//! This module contains business logic for computing repository roots and
//! worktree layout paths. Adapter layers (CLI, web) should call these APIs and
//! only format output.

use crate::errors::{CoreError, CoreResult};
use ito_config::ConfigContext;
use ito_config::ito_dir::{absolutize_and_normalize, get_ito_path, lexical_normalize};
use ito_config::load_cascading_project_config;
use ito_config::types::{ItoConfig, WorktreeStrategy};
use std::path::{Path, PathBuf};

/// Distinguishes bare repositories from non-bare working trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitRepoKind {
    /// The current directory resolves to a bare Git repository (no worktree).
    Bare,
    /// The current directory resolves to a non-bare Git repository.
    NonBare,
}

impl GitRepoKind {
    /// Returns true when this represents a bare repository.
    pub fn is_bare(self) -> bool {
        match self {
            Self::Bare => true,
            Self::NonBare => false,
        }
    }
}

/// Whether worktrees are enabled for the current project configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorktreeFeature {
    /// Worktrees are enabled.
    Enabled,
    /// Worktrees are disabled.
    Disabled,
}

impl WorktreeFeature {
    /// Returns true when worktrees are enabled.
    pub fn is_enabled(self) -> bool {
        match self {
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}

/// Resolved repository and Ito roots for a given invocation.
#[derive(Debug, Clone)]
pub struct ResolvedEnv {
    /// The selected working-tree root (Git top-level if inside a worktree, or
    /// nearest Ito root/cwd fallback), normalized to an absolute path when possible.
    pub worktree_root: PathBuf,
    /// The repository's common project root (when available), otherwise the worktree root.
    pub project_root: PathBuf,
    /// The resolved Ito directory path for this invocation.
    pub ito_root: PathBuf,
    /// Whether the repository is bare.
    pub git_repo_kind: GitRepoKind,
}

/// Selector for a specific worktree path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeSelector {
    /// The main/default worktree.
    Main,
    /// A worktree for a branch name.
    Branch(String),
    /// A worktree for a change ID/name.
    Change(String),
}

/// Derived worktree layout paths for the current project.
#[derive(Debug, Clone)]
pub struct ResolvedWorktreePaths {
    /// Whether worktrees are enabled.
    pub feature: WorktreeFeature,
    /// Configured worktree strategy.
    pub strategy: WorktreeStrategy,
    /// Root directory that contains branch/change worktrees.
    pub worktrees_root: Option<PathBuf>,
    /// Root directory of the main worktree.
    pub main_worktree_root: Option<PathBuf>,
}

impl ResolvedWorktreePaths {
    /// Resolves a worktree path for the given selector.
    pub fn path_for_selector(&self, selector: &WorktreeSelector) -> Option<PathBuf> {
        if !self.feature.is_enabled() {
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

/// Resolve repository and Ito-related roots for the current working directory.
///
/// This function determines the worktree root, project root, Ito directory, and
/// whether the current repository is bare.
pub fn resolve_env(ctx: &ConfigContext) -> CoreResult<ResolvedEnv> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    resolve_env_from_cwd(&cwd, ctx)
}

/// Resolve repository and Ito-related roots for a specific `cwd`.
pub fn resolve_env_from_cwd(cwd: &Path, ctx: &ConfigContext) -> CoreResult<ResolvedEnv> {
    let is_bare = git_is_bare_repo(cwd).unwrap_or(false);
    let git_repo_kind = if is_bare {
        GitRepoKind::Bare
    } else {
        GitRepoKind::NonBare
    };

    // If we're inside a git worktree, prefer the actual worktree root.
    let worktree_root = git_show_toplevel(cwd)
        .or_else(|| find_nearest_ito_root(cwd))
        .unwrap_or_else(|| cwd.to_path_buf());
    let worktree_root = absolutize_and_normalize(&worktree_root)
        .unwrap_or_else(|_| lexical_normalize(&worktree_root));

    let ito_root = get_ito_path(&worktree_root, ctx);
    if !ito_root.is_dir() {
        if git_repo_kind.is_bare() {
            return Err(CoreError::validation(bare_repo_error_message_raw(cwd)));
        }
        return Err(CoreError::not_found(format!(
            "No Ito directory found (expected {}). Run `ito init <project-dir>` or cd into an initialized worktree.",
            ito_root.to_string_lossy()
        )));
    }

    let project_root = git_common_root(&worktree_root).unwrap_or_else(|| worktree_root.clone());
    let project_root = absolutize_and_normalize(&project_root)
        .unwrap_or_else(|_| lexical_normalize(&project_root));

    Ok(ResolvedEnv {
        worktree_root,
        project_root,
        ito_root,
        git_repo_kind,
    })
}

/// Computes worktree layout paths from the resolved environment and repository-local configuration.
pub fn resolve_worktree_paths(
    env: &ResolvedEnv,
    ctx: &ConfigContext,
) -> CoreResult<ResolvedWorktreePaths> {
    // Load config relative to the current worktree root so repo-local sources
    // like `ito.json` and `.ito.json` resolve within the working checkout.
    let cfg = load_cascading_project_config(&env.worktree_root, &env.ito_root, ctx);
    let typed: ItoConfig = serde_json::from_value(cfg.merged)
        .map_err(|e| CoreError::serde("parse Ito configuration", e.to_string()))?;

    let wt = typed.worktrees;
    let feature = if wt.enabled {
        WorktreeFeature::Enabled
    } else {
        WorktreeFeature::Disabled
    };
    let strategy = wt.strategy;
    let default_branch = wt.default_branch;
    let dir_name = wt.layout.dir_name;

    let base = resolve_base_dir(env, &wt.layout.base_dir);

    let (worktrees_root, main_worktree_root) = if feature.is_enabled() {
        match strategy {
            WorktreeStrategy::CheckoutSubdir => {
                let wt_root = base.join(format!(".{dir_name}"));
                (Some(wt_root), Some(base))
            }
            WorktreeStrategy::CheckoutSiblings => {
                let project_name = base
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("project");
                let parent = base.parent().unwrap_or(&base);
                let wt_root = parent.join(format!("{project_name}-{dir_name}"));
                (Some(wt_root), Some(base))
            }
            WorktreeStrategy::BareControlSiblings => {
                let wt_root = base.join(&dir_name);
                let main = base.join(&default_branch);
                (Some(wt_root), Some(main))
            }
        }
    } else {
        (None, None)
    };

    Ok(ResolvedWorktreePaths {
        feature,
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
    )
    .or_else(|| git_output(cwd, &["rev-parse", "--show-toplevel"]))?;
    let out = out.trim();
    if out.is_empty() {
        return None;
    }

    let p = PathBuf::from(out);
    let p = if p.is_absolute() { p } else { cwd.join(p) };
    Some(absolutize_and_normalize(&p).unwrap_or_else(|_| lexical_normalize(&p)))
}

fn git_common_root(worktree_root: &Path) -> Option<PathBuf> {
    let common = git_output(
        worktree_root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
    .or_else(|| git_output(worktree_root, &["rev-parse", "--git-common-dir"]))?;
    let common = common.trim();
    if common.is_empty() {
        return None;
    }

    let common = PathBuf::from(common);
    let common = if common.is_absolute() {
        common
    } else {
        worktree_root.join(common)
    };
    let common = absolutize_and_normalize(&common).unwrap_or_else(|_| lexical_normalize(&common));

    common.parent().map(Path::to_path_buf)
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
