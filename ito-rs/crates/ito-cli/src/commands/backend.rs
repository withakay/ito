//! Handler for `ito backend` subcommands.

use crate::cli::{BackendAction, BackendArgs};
use crate::cli_error::CliResult;
use crate::runtime::Runtime;

/// Dispatch `ito backend` subcommands.
pub fn handle_backend_clap(rt: &Runtime, args: &BackendArgs) -> CliResult<()> {
    match &args.action {
        BackendAction::Status { json } => handle_status(rt, *json),
        BackendAction::GenerateToken { seed, org, repo } => {
            handle_generate_token(rt, seed.clone(), org.clone(), repo.clone())
        }
    }
}

fn handle_status(_rt: &Runtime, _json: bool) -> CliResult<()> {
    eprintln!("ito backend status: not yet implemented");
    Ok(())
}

fn handle_generate_token(
    _rt: &Runtime,
    _seed: Option<String>,
    _org: Option<String>,
    _repo: Option<String>,
) -> CliResult<()> {
    eprintln!("ito backend generate-token: not yet implemented");
    Ok(())
}
