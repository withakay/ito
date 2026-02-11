use crate::cli::{WorkflowAction, WorkflowArgs};
use crate::cli_error::CliResult;
use crate::runtime::Runtime;

pub(crate) fn handle_workflow_clap(_rt: &Runtime, args: &WorkflowArgs) -> CliResult<()> {
    match &args.action {
        Some(WorkflowAction::Init)
        | Some(WorkflowAction::List)
        | Some(WorkflowAction::Show { .. })
        | Some(WorkflowAction::Run { .. })
        | Some(WorkflowAction::Status { .. })
        | None => Ok(()),
    }
}
