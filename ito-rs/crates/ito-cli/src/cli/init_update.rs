use clap::Args;

/// Initialize Ito instruction files in a project directory.
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Configure AI tools non-interactively (all, none, or comma-separated ids)
    #[arg(long)]
    pub tools: Option<String>,

    /// Overwrite existing tool files without prompting
    #[arg(short = 'f', long)]
    pub force: bool,

    /// Update managed files while preserving user-edited files (project.md, user-guidance.md, etc.)
    #[arg(short = 'u', long)]
    pub update: bool,

    /// Refresh managed prompt/template content (marker-scoped upgrade; preserves user content outside markers)
    #[arg(long)]
    pub upgrade: bool,

    /// Remove known legacy Ito-managed paths during --upgrade
    #[arg(long)]
    pub cleanup: bool,

    /// Ensure coordination branch exists on origin after init
    #[arg(long = "setup-coordination-branch")]
    #[cfg_attr(not(feature = "coordination-branch"), arg(hide = true))]
    pub setup_coordination_branch: bool,

    /// Skip coordination worktree setup and use embedded storage mode instead
    #[arg(long = "no-coordination-worktree")]
    #[cfg_attr(not(feature = "coordination-branch"), arg(hide = true))]
    pub no_coordination_worktree: bool,

    /// Enable Git worktree-based workspace layout
    #[arg(long = "worktrees", conflicts_with = "no_worktrees")]
    pub worktrees: bool,

    /// Disable Git worktree-based workspace layout
    #[arg(long = "no-worktrees", conflicts_with = "worktrees")]
    pub no_worktrees: bool,

    /// Worktree topology strategy
    #[arg(long = "worktree-strategy", value_name = "STRATEGY")]
    pub worktree_strategy: Option<String>,

    /// Preferred integration mode after implementation
    #[arg(long = "worktree-integration-mode", value_name = "MODE")]
    pub worktree_integration_mode: Option<String>,

    /// Override HOME used for locating global Ito config (for parity/testing)
    #[arg(long, value_name = "HOME")]
    pub home: Option<std::path::PathBuf>,

    /// Target directory (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,
}

/// Update Ito instruction files.
#[derive(Args, Debug, Clone)]
pub struct UpdateArgs {
    /// Output as JSON (not implemented yet)
    #[arg(long)]
    pub json: bool,

    /// Enable Git worktree-based workspace layout
    #[arg(long = "worktrees", conflicts_with = "no_worktrees")]
    pub worktrees: bool,

    /// Disable Git worktree-based workspace layout
    #[arg(long = "no-worktrees", conflicts_with = "worktrees")]
    pub no_worktrees: bool,

    /// Worktree topology strategy
    #[arg(long = "worktree-strategy", value_name = "STRATEGY")]
    pub worktree_strategy: Option<String>,

    /// Preferred integration mode after implementation
    #[arg(long = "worktree-integration-mode", value_name = "MODE")]
    pub worktree_integration_mode: Option<String>,

    /// Target directory (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,
}
