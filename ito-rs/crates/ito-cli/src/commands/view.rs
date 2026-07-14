use crate::cli::{ViewArgs, ViewCommand, ViewProposalArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use dialoguer::{Select, theme::ColorfulTheme};
use ito_core::viewer::{ViewerBackend, ViewerRegistry, collect_proposal_artifacts};

pub(crate) fn handle_view_clap(rt: &Runtime, args: &ViewArgs) -> CliResult<()> {
    let Some(command) = &args.command else {
        return fail("Missing required subcommand");
    };

    match command {
        ViewCommand::Proposal(args) => handle_view_proposal(rt, args),
    }
}

fn handle_view_proposal(rt: &Runtime, args: &ViewProposalArgs) -> CliResult<()> {
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let change_repo = runtime.repositories().changes.as_ref();
    let resolved_change = crate::app::common::resolve_change_target(change_repo, &args.change_id)
        .map_err(crate::cli_error::CliError::msg)?;
    let content =
        collect_proposal_artifacts(&resolved_change, rt.ito_path()).map_err(to_cli_error)?;

    if args.json {
        let output = serde_json::json!({
            "change_id": resolved_change,
            "content": content,
        });
        let rendered = serde_json::to_string_pretty(&output).map_err(to_cli_error)?;
        println!("{rendered}");
        return Ok(());
    }

    let registry = ViewerRegistry::for_proposals();
    let viewer = match &args.viewer {
        Some(name) => resolve_named_viewer(&registry, name)?,
        None => prompt_for_viewer(&registry)?,
    };

    viewer.open(&content).map_err(to_cli_error)
}

fn resolve_named_viewer<'a>(
    registry: &'a ViewerRegistry,
    name: &str,
) -> CliResult<&'a dyn ViewerBackend> {
    let Some(viewer) = registry.find_by_name(name) else {
        return fail(format!("Unknown viewer '{name}'"));
    };
    if !viewer.is_available() {
        let msg = viewer.availability_hint().unwrap_or_else(|| {
            format!("Viewer '{name}' is unavailable. Install its backing tool and try again.")
        });
        return fail(msg);
    }
    Ok(viewer)
}

fn prompt_for_viewer(registry: &ViewerRegistry) -> CliResult<&dyn ViewerBackend> {
    let available = registry.available_viewers();
    if available.is_empty() {
        return fail("No proposal viewers are available. Install one of: bat, glow, or pandoc.");
    }

    let mut items = Vec::new();
    for viewer in &available {
        items.push(format!("{} - {}", viewer.name(), viewer.description()));
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a proposal viewer")
        .items(&items)
        .default(0)
        .interact()
        .map_err(to_cli_error)?;
    Ok(available[selection])
}
