use clap::{Args, Subcommand};

/// Print resolved paths for the current project/worktree.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct PathArgs {
    #[command(subcommand)]
    pub command: Option<PathCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PathCommand {
    /// Print the stable project root (common repo root across worktrees).
    ProjectRoot(PathCommonArgs),
    /// Print the current working worktree root (errors in bare repos).
    WorktreeRoot(PathCommonArgs),
    /// Print the resolved Ito directory path (e.g. <root>/.ito).
    ItoRoot(PathCommonArgs),
    /// Print the configured worktrees root directory (errors if worktrees disabled).
    WorktreesRoot(PathCommonArgs),
    /// Print a specific worktree path derived from config.
    Worktree(PathWorktreeArgs),
    /// Print a bundle of resolved roots.
    Roots(PathRootsArgs),
}

#[derive(Args, Debug, Clone)]
pub struct PathCommonArgs {
    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug, Clone)]
pub struct PathWorktreeArgs {
    /// Select the main/default worktree directory.
    ///
    /// For `bare_control_siblings`, this uses `worktrees.default_branch`.
    #[arg(long, conflicts_with_all = ["branch", "change"])]
    pub main: bool,

    /// Select a worktree by branch name.
    #[arg(long, value_name = "BRANCH", conflicts_with_all = ["main", "change"])]
    pub branch: Option<String>,

    /// Select a worktree by Ito change id.
    #[arg(long, value_name = "CHANGE", conflicts_with_all = ["main", "branch"])]
    pub change: Option<String>,

    #[command(flatten)]
    pub common: PathCommonArgs,
}

#[derive(Args, Debug, Clone)]
pub struct PathRootsArgs {
    #[command(flatten)]
    pub common: PathCommonArgs,
}
