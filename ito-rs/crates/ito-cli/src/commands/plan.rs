use crate::cli::{PlanAction, PlanArgs};
use crate::cli_error::{CliError, CliResult};
use crate::runtime::Runtime;
use ito_core::audit::{Actor, AuditEventBuilder, EntityType, ops};
use ito_core::planning_init;

pub(crate) fn handle_plan_clap(rt: &Runtime, args: &PlanArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return Err(CliError::msg("Missing required plan subcommand"));
    };

    let ito_path = rt.ito_path();
    match action {
        PlanAction::Init => {
            let status = planning_init::read_planning_workspace_status(ito_path).map_err(|e| {
                CliError::msg(format!(
                    "Could not inspect planning workspace: {e}. Check directory permissions and disk space."
                ))
            })?;

            if status.planning_invalid {
                return Err(CliError::msg(format!(
                    "Could not create planning workspace: {} exists but is not a directory. Rename or remove it, then run `ito plan init`.",
                    status.planning_dir.display()
                )));
            }

            planning_init::init_planning_structure(ito_path).map_err(|e| {
                CliError::msg(format!(
                    "Could not create planning workspace: {e}. Check directory permissions and disk space."
                ))
            })?;

            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Planning)
                .entity_id("planning-structure")
                .op(ops::PLANNING_NOTE)
                .to("initialized")
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            eprintln!("✔ Planning workspace available");
            if status.research_invalid {
                eprintln!(
                    "Warning: {} exists but is not a directory. Rename or remove it before storing deep-dive research.",
                    status.research_dir.display()
                );
            }
            println!("Planning workspace:");
            println!("  - {}", status.planning_dir.display());
            println!(
                "Create plan documents with /ito-plan; supporting research can live under {}.",
                status.research_dir.display()
            );
            Ok(())
        }
        PlanAction::Status => {
            let status = planning_init::read_planning_workspace_status(ito_path).map_err(|e| {
                CliError::msg(format!(
                    "Could not read planning workspace status: {e}. Check directory permissions and disk space."
                ))
            })?;

            println!("Planning Workspace");
            println!("────────────────────────────────────────");
            println!(
                "Planning: {}",
                if status.planning_exists {
                    "available"
                } else if status.planning_invalid {
                    "invalid"
                } else {
                    "missing"
                }
            );
            println!("Path: {}", status.planning_dir.display());
            println!(
                "Research: {}",
                if status.research_exists {
                    "available"
                } else if status.research_invalid {
                    "invalid"
                } else {
                    "missing"
                }
            );
            println!("Research path: {}", status.research_dir.display());
            println!();

            if status.planning_invalid {
                println!(
                    "Planning path is not a directory. Rename or remove it, then run `ito plan init`."
                );
                return Ok(());
            }

            if status.research_invalid {
                println!(
                    "Research path is not a directory. Rename or remove it before storing deep-dive research."
                );
                println!();
            }

            println!("Planning Documents");
            println!("────────────────────────────────────────");

            if !status.planning_exists {
                println!("No planning workspace found. Run `ito plan init` to create one.");
                return Ok(());
            }

            if status.planning_documents.is_empty() {
                println!("No planning documents yet. Use /ito-plan to create the first plan.");
                return Ok(());
            }

            for document in &status.planning_documents {
                let name = document
                    .file_name()
                    .unwrap_or_else(|| document.as_os_str())
                    .to_string_lossy();
                println!("  - {name}");
            }

            Ok(())
        }
    }
}
