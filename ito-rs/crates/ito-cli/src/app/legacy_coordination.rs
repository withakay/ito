use crate::cli::BackendAction;
use crate::cli::{AgentCommand, Commands, ConfigCommand, PlanAction, TasksAction, WorktreeCommand};
use crate::cli_error::{CliResult, to_cli_error};
use crate::commands::audit::AuditAction;
use crate::diagnostics::{
    LegacyCoordinationMutationBlocked, format_legacy_coordination_read_warning,
};
use crate::runtime::Runtime;
use ito_config::types::ItoConfig;
use ito_core::legacy_coordination::{
    LegacyCoordinationClass, expected_coordination_ito_root, inspect_legacy_coordination,
};

/// Safety intent assigned to a parsed Ito command in legacy coordination state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandIntent {
    /// The command only inspects state and may continue with a warning.
    ReadOnly,
    /// The command can mutate local or remote state and must be blocked.
    Mutating,
    /// The command is an explicit escape path from legacy storage.
    Recovery,
}

/// Classify a parsed command before dispatch.
#[must_use]
pub(crate) fn command_intent(command: &Commands) -> CommandIntent {
    match command {
        Commands::List(_)
        | Commands::ListArchive(_)
        | Commands::Show(_)
        | Commands::Status(_)
        | Commands::Validate(_)
        | Commands::Grep(_)
        | Commands::Path(_)
        | Commands::View(_)
        | Commands::Util(_)
        | Commands::Trace(_)
        | Commands::Completions(_)
        | Commands::Stats(_)
        | Commands::Help(_) => CommandIntent::ReadOnly,
        Commands::Tasks(args) => match &args.action {
            Some(
                TasksAction::Status { .. }
                | TasksAction::Next { .. }
                | TasksAction::Ready { .. }
                | TasksAction::Show { .. },
            ) => CommandIntent::ReadOnly,
            Some(
                TasksAction::Init { .. }
                | TasksAction::Start { .. }
                | TasksAction::Complete { .. }
                | TasksAction::Shelve { .. }
                | TasksAction::Unshelve { .. }
                | TasksAction::Add { .. }
                | TasksAction::Claim { .. }
                | TasksAction::Release { .. }
                | TasksAction::Allocate
                | TasksAction::Sync(_)
                | TasksAction::External(_),
            )
            | None => CommandIntent::Mutating,
        },
        Commands::Plan(args) => match &args.action {
            Some(PlanAction::Status) => CommandIntent::ReadOnly,
            Some(PlanAction::Init) | None => CommandIntent::Mutating,
        },
        Commands::Agent(args) => match &args.command {
            Some(AgentCommand::Instruction(args)) if args.sync => CommandIntent::Mutating,
            Some(AgentCommand::Instruction(args)) if args.artifact == "migrate-to-main" => {
                CommandIntent::Recovery
            }
            Some(AgentCommand::Instruction(_)) => CommandIntent::ReadOnly,
            Some(AgentCommand::External(_)) | None => CommandIntent::Mutating,
        },
        Commands::Config(args) => match &args.command {
            None
            | Some(ConfigCommand::Path(_) | ConfigCommand::List(_) | ConfigCommand::Get { .. })
            | Some(ConfigCommand::Schema { output: None }) => CommandIntent::ReadOnly,
            Some(
                ConfigCommand::Set { .. }
                | ConfigCommand::Unset { .. }
                | ConfigCommand::Schema { output: Some(_) }
                | ConfigCommand::External(_),
            ) => CommandIntent::Mutating,
        },
        Commands::Worktree(args) => match &args.command {
            WorktreeCommand::Validate(_) => CommandIntent::ReadOnly,
            WorktreeCommand::Ensure(_) | WorktreeCommand::Setup(_) => CommandIntent::Mutating,
        },
        Commands::Audit(args) => match &args.action {
            Some(
                AuditAction::Log { .. }
                | AuditAction::Validate { .. }
                | AuditAction::Stats { .. }
                | AuditAction::Stream { .. },
            )
            | Some(AuditAction::Reconcile { fix: false, .. }) => CommandIntent::ReadOnly,
            Some(AuditAction::Reconcile { fix: true, .. }) | None => CommandIntent::Mutating,
        },
        Commands::Backend(args) => match &args.action {
            BackendAction::Status { .. }
            | BackendAction::GenerateToken { .. }
            | BackendAction::Import { dry_run: true } => CommandIntent::ReadOnly,
            BackendAction::Serve(_) | BackendAction::Import { dry_run: false } => {
                CommandIntent::Mutating
            }
        },
        Commands::ServeApiRemoved(_) => CommandIntent::ReadOnly,
        Commands::Create(_)
        | Commands::Archive(_)
        | Commands::Patch(_)
        | Commands::Write(_)
        | Commands::Sync(_)
        | Commands::Split(_)
        | Commands::Ralph(_)
        | Commands::Loop(_)
        | Commands::Init(_)
        | Commands::Update(_)
        | Commands::Templates(_)
        | Commands::Dashboard(_)
        | Commands::New(_) => CommandIntent::Mutating,
        #[cfg(feature = "web")]
        Commands::Serve(_) => CommandIntent::Mutating,
    }
}

/// Enforce legacy coordination safety before dispatch can perform side effects.
pub(crate) fn enforce_legacy_coordination_guard(
    runtime: &Runtime,
    command: &Commands,
) -> CliResult<()> {
    enforce_legacy_coordination_intent(runtime, command_intent(command))
}

/// Treat an unparsed or future command as mutating before invalid-command logging.
pub(crate) fn enforce_legacy_coordination_parse_failure_guard(runtime: &Runtime) -> CliResult<()> {
    enforce_legacy_coordination_intent(runtime, CommandIntent::Mutating)
}

fn enforce_legacy_coordination_intent(runtime: &Runtime, intent: CommandIntent) -> CliResult<()> {
    if intent == CommandIntent::Recovery {
        runtime.suppress_command_side_effects();
        return Ok(());
    }

    let ito_root = runtime.ito_path();
    if !ito_root.is_dir() {
        return Ok(());
    }
    let project_root = ito_root.parent().unwrap_or(ito_root);
    let typed: ItoConfig =
        serde_json::from_value(runtime.resolved_config().merged.clone()).map_err(to_cli_error)?;
    let expected_coordination_ito_root = expected_coordination_ito_root(
        project_root,
        ito_root,
        &typed.changes.coordination_branch,
        &typed.backend,
    );
    let report = inspect_legacy_coordination(
        project_root,
        ito_root,
        &typed.changes.coordination_branch,
        expected_coordination_ito_root.as_deref(),
    )
    .map_err(to_cli_error)?;

    match report.classification {
        LegacyCoordinationClass::Absent | LegacyCoordinationClass::Embedded => Ok(()),
        LegacyCoordinationClass::Legacy | LegacyCoordinationClass::Ambiguous => match intent {
            CommandIntent::ReadOnly => {
                runtime.suppress_command_side_effects();
                eprintln!(
                    "{}",
                    format_legacy_coordination_read_warning(report.classification)
                );
                Ok(())
            }
            CommandIntent::Recovery => unreachable!("recovery returned before inspection"),
            CommandIntent::Mutating => Err(to_cli_error(LegacyCoordinationMutationBlocked::new(
                report.classification,
            ))),
        },
    }
}

#[cfg(test)]
#[path = "legacy_coordination_tests.rs"]
mod legacy_coordination_tests;
