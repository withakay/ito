use clap::{Args, Subcommand};

/// Apply a targeted patch to an active change artifact.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub struct PatchArgs {
    #[command(subcommand)]
    pub target: ChangeArtifactTargetCommand,
}

/// Replace an active change artifact completely.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub struct WriteArgs {
    #[command(subcommand)]
    pub target: ChangeArtifactTargetCommand,
}

/// Supported mutation targets for active change artifact commands.
#[derive(Subcommand, Debug, Clone)]
pub enum ChangeArtifactTargetCommand {
    /// Mutate an artifact inside an active change.
    Change(ChangeArtifactTargetArgs),
}

/// Arguments for change-scoped artifact mutation.
#[derive(Args, Debug, Clone)]
pub struct ChangeArtifactTargetArgs {
    /// Change ID or unique prefix.
    pub change: String,

    #[command(subcommand)]
    pub artifact: ChangeArtifactSelector,
}

/// Addressable active-change artifacts.
#[derive(Subcommand, Debug, Clone)]
pub enum ChangeArtifactSelector {
    /// `proposal.md`
    Proposal,
    /// `design.md`
    Design,
    /// The change's tracking artifact (usually `tasks.md`).
    Tasks,
    /// `specs/<capability>/spec.md`
    Spec {
        /// Capability directory name under the change's `specs/` directory.
        capability: String,
    },
}
