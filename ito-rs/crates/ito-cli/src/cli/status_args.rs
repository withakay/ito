use clap::Args;

/// Synchronize coordination worktree state.
#[derive(Args, Debug, Clone, Default)]
pub struct SyncArgs {
    /// Bypass redundant-push suppression for this invocation
    #[arg(long)]
    pub force: bool,

    /// Output as JSON when supported by the handler
    #[arg(long)]
    pub json: bool,
}

/// Display artifact completion status for a change.
#[derive(Args, Debug, Clone)]
pub struct StatusArgs {
    /// Change id (directory name)
    #[arg(short = 'c', long)]
    pub change: Option<String>,

    /// Workflow schema name
    #[arg(long)]
    pub schema: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}
