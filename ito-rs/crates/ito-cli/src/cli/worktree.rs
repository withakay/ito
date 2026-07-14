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
    /// Resolves the expected worktree path for a change, proves prepare
    /// readiness, creates it from the captured authority commit if absent,
    /// then proves execute readiness before copying files or running setup.
    ///
    /// When worktrees are disabled, prints the current working directory.
    #[command(verbatim_doc_comment)]
    Ensure(WorktreeChangeArgs),

    /// Re-run setup commands in an existing change worktree
    ///
    /// Proves that the existing worktree descends from the proposal integration
    /// commit, then runs configured `worktrees.init.setup` commands without
    /// recreating it or re-copying files.
    #[command(verbatim_doc_comment)]
    Setup(WorktreeChangeArgs),

    /// Validate that the current checkout is an acceptable worktree for a change
    ///
    /// This fast read-only check is designed for humans and hook callers. When
    /// worktrees are enabled, it rejects the main/control checkout, reports
    /// mismatches when the current branch/path does not include the full change
    /// ID, and accepts same-change suffix worktrees such as `foo-review`.
    #[command(verbatim_doc_comment)]
    Validate(WorktreeValidateArgs),
}

/// Arguments shared by worktree sub-commands that target a change.
#[derive(Args, Debug, Clone)]
pub struct WorktreeChangeArgs {
    /// Change ID to target (e.g. `012-05_my-change`)
    #[arg(long)]
    pub change: String,
}

/// Arguments for validating the current checkout against a change worktree.
#[derive(Args, Debug, Clone)]
pub struct WorktreeValidateArgs {
    #[command(flatten)]
    pub change_args: WorktreeChangeArgs,

    /// Emit machine-readable JSON for hook callers.
    #[arg(long)]
    pub json: bool,
}
