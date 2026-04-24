use clap::{Args, Subcommand};

/// Arguments for the `ito worktree` command group.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct WorktreeArgs {
    /// The worktree sub-command to run.
    #[command(subcommand)]
    pub command: WorktreeCommand,
}

/// Sub-commands under `ito worktree`.
#[derive(Subcommand, Debug, Clone)]
pub enum WorktreeCommand {
    /// Ensure the correct change worktree exists and is initialized
    ///
    /// Resolves the expected worktree path for a change, creates it if absent
    /// (branching from the configured default branch), copies include files,
    /// runs setup commands, and prints the resolved path to stdout.
    ///
    /// When worktrees are disabled, prints the current working directory.
    #[command(verbatim_doc_comment)]
    Ensure(WorktreeChangeArgs),

    /// Re-run setup commands in an existing change worktree
    ///
    /// Runs the configured setup command(s) from `worktrees.init.setup` inside
    /// an existing worktree without recreating it or re-copying files.
    #[command(verbatim_doc_comment)]
    Setup(WorktreeChangeArgs),
}

/// Arguments shared by worktree sub-commands that target a change.
#[derive(Args, Debug, Clone)]
pub struct WorktreeChangeArgs {
    /// Change ID to target (e.g. `012-05_my-change`)
    #[arg(long)]
    pub change: String,
}
