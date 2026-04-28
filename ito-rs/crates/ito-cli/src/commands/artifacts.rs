use std::io::{IsTerminal, Read};

use ito_core::{ChangeArtifactKind, ChangeArtifactRef};

use crate::app::common::resolve_change_target;
use crate::cli::{
    ChangeArtifactSelector, ChangeArtifactTargetArgs, ChangeArtifactTargetCommand, PatchArgs,
    WriteArgs,
};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_core::repository_runtime::RepositoryRuntime;

pub(crate) fn handle_patch_clap(rt: &Runtime, args: &PatchArgs) -> CliResult<()> {
    handle_change_artifact_mutation(rt, &args.target, MutationMode::Patch)
}

pub(crate) fn handle_write_clap(rt: &Runtime, args: &WriteArgs) -> CliResult<()> {
    handle_change_artifact_mutation(rt, &args.target, MutationMode::Write)
}

#[derive(Debug, Clone, Copy)]
enum MutationMode {
    Patch,
    Write,
}

fn handle_change_artifact_mutation(
    rt: &Runtime,
    target: &ChangeArtifactTargetCommand,
    mode: MutationMode,
) -> CliResult<()> {
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    match target {
        ChangeArtifactTargetCommand::Change(args) => handle_change_target(runtime, args, mode),
    }
}

fn handle_change_target(
    runtime: &RepositoryRuntime,
    args: &ChangeArtifactTargetArgs,
    mode: MutationMode,
) -> CliResult<()> {
    let change_id =
        match resolve_change_target(runtime.repositories().changes.as_ref(), &args.change) {
            Ok(change_id) => change_id,
            Err(message) => return fail(message),
        };
    let artifact = artifact_kind_from_selector(&args.artifact);
    let target = ChangeArtifactRef {
        change_id,
        artifact,
    };
    let input = read_mutation_input()?;

    let result = match mode {
        MutationMode::Patch => runtime
            .change_artifact_mutations()
            .patch_artifact(&target, &input),
        MutationMode::Write => runtime
            .change_artifact_mutations()
            .write_artifact(&target, &input),
    };
    let result = match result {
        Ok(result) => result,
        Err(err) => return fail(err.to_string()),
    };

    let action = match (mode, result.existed) {
        (MutationMode::Patch, _) => "patched",
        (MutationMode::Write, true) => "updated",
        (MutationMode::Write, false) => "created",
    };
    println!("Successfully {action} artifact '{}'", result.target.label());
    if let Some(revision) = result.revision {
        println!("Revision: {revision}");
    }
    Ok(())
}

fn artifact_kind_from_selector(selector: &ChangeArtifactSelector) -> ChangeArtifactKind {
    match selector {
        ChangeArtifactSelector::Proposal => ChangeArtifactKind::Proposal,
        ChangeArtifactSelector::Design => ChangeArtifactKind::Design,
        ChangeArtifactSelector::Tasks => ChangeArtifactKind::Tasks,
        ChangeArtifactSelector::Spec { capability } => ChangeArtifactKind::SpecDelta {
            capability: capability.clone(),
        },
    }
}

fn read_mutation_input() -> CliResult<String> {
    let stdin_is_tty = std::io::stdin().is_terminal();
    if !stdin_is_tty {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|err| to_cli_error(format!("failed to read stdin: {err}")))?;
        if buf.is_empty() {
            return fail("No mutation input provided. Pipe data on stdin.");
        }
        return Ok(buf);
    }

    fail("No mutation input provided. Pipe data on stdin.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_from_selector_maps_expected_variants() {
        assert!(matches!(
            artifact_kind_from_selector(&ChangeArtifactSelector::Proposal),
            ChangeArtifactKind::Proposal
        ));
        assert!(matches!(
            artifact_kind_from_selector(&ChangeArtifactSelector::Design),
            ChangeArtifactKind::Design
        ));
        assert!(matches!(
            artifact_kind_from_selector(&ChangeArtifactSelector::Tasks),
            ChangeArtifactKind::Tasks
        ));
        assert_eq!(
            artifact_kind_from_selector(&ChangeArtifactSelector::Spec {
                capability: "backend-agent-instructions".to_string(),
            }),
            ChangeArtifactKind::SpecDelta {
                capability: "backend-agent-instructions".to_string(),
            }
        );
    }
}
