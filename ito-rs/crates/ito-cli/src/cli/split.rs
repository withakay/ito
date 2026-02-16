use clap::Args;

/// Split a large change into smaller changes.
///
/// Note: This command is currently a stub in `ito-cli`.
#[derive(Args, Debug, Clone)]
pub struct SplitArgs {
    /// Change id (directory name)
    #[arg(value_name = "CHANGE")]
    pub change: Option<String>,
}
