use clap::{Args, Subcommand, ValueEnum};

/// Readiness and inspection commands for one Ito change.
#[derive(Args, Debug, Clone)]
pub struct ChangeArgs {
    #[command(subcommand)]
    pub command: ChangeCommand,
}

/// Supported `ito change` operations.
#[derive(Subcommand, Debug, Clone)]
pub enum ChangeCommand {
    /// Prove that a proposal is ready for preparation or implementation.
    Preflight(ChangePreflightArgs),
}

/// Arguments for `ito change preflight`.
#[derive(Args, Debug, Clone)]
pub struct ChangePreflightArgs {
    /// Full canonical Ito change ID.
    pub change_id: String,

    /// Readiness phase to evaluate.
    #[arg(long = "for", value_enum)]
    pub phase: ReadinessPhaseArg,

    /// Refresh the configured pull-request authority before evaluation.
    #[arg(long)]
    pub refresh: bool,

    /// Emit the stable readiness report as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Readiness phase accepted by the preflight CLI.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadinessPhaseArg {
    Prepare,
    Execute,
}
