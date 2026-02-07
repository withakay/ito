use crate::cli::{WorkflowAction, WorkflowArgs};
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::domain::workflow as wf_workflow;
use ito_core::workflow_templates as wf_io;

pub(crate) fn handle_workflow_clap(rt: &Runtime, args: &WorkflowArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return Err(CliError::msg("Missing required workflow subcommand"));
    };

    let ito_path = rt.ito_path();

    match action {
        WorkflowAction::Init => {
            wf_io::init_workflow_structure(ito_path).map_err(to_cli_error)?;
            println!("Created workflows directory with example workflows:");
            println!("  - research.yaml  (domain investigation)");
            println!("  - execute.yaml   (task execution)");
            println!("  - review.yaml    (adversarial review)");
            println!();
            println!("Prompt templates are installed via `ito init`.");
            Ok(())
        }
        WorkflowAction::List => {
            let workflows = wf_io::list_workflows(ito_path);
            if workflows.is_empty() {
                println!("No workflows found. Run `ito workflow init` to create examples.");
                return Ok(());
            }
            println!("Available workflows:");
            println!();
            for name in workflows {
                match wf_io::load_workflow(ito_path, &name) {
                    Ok(wf) => {
                        println!("  {name}");
                        println!("    {}", wf.description);
                        println!(
                            "    Waves: {}, Tasks: {}",
                            wf.waves.len(),
                            wf_workflow::count_tasks(&wf)
                        );
                        println!();
                    }
                    Err(e) => {
                        println!("  {name} (invalid: {e})");
                    }
                }
            }
            Ok(())
        }
        WorkflowAction::Show { workflow_name } => {
            let workflow_name = workflow_name.join(" ");
            if workflow_name.trim().is_empty() {
                return Err(CliError::msg("Missing required argument <workflow-name>"));
            }

            let wf = wf_io::load_workflow(ito_path, &workflow_name)
                .map_err(|e| CliError::msg(format!("Invalid workflow: {e}")))?;

            fn agent_label(a: &ito_core::schemas::AgentType) -> &'static str {
                match a {
                    ito_core::schemas::AgentType::Research => "research",
                    ito_core::schemas::AgentType::Execution => "execution",
                    ito_core::schemas::AgentType::Review => "review",
                    ito_core::schemas::AgentType::Planning => "planning",
                }
            }

            println!("# Workflow: {}", wf.name);
            println!("ID: {}", wf.id);
            println!("Description: {}", wf.description);
            println!();
            if let Some(req) = &wf.requires {
                println!("## Requirements");
                if let Some(files) = &req.files {
                    println!("Files: {}", files.join(", "));
                }
                if let Some(vars) = &req.variables {
                    println!("Variables: {}", vars.join(", "));
                }
                println!();
            }
            println!("## Waves");
            println!();
            for (idx, wave) in wf.waves.iter().enumerate() {
                let cp = if wave.checkpoint.unwrap_or(false) {
                    " (checkpoint)"
                } else {
                    ""
                };
                println!("### Wave {}: {}{cp}", idx + 1, wave.id);
                println!();
                for task in &wave.tasks {
                    println!("  - [{}] {}", agent_label(&task.agent), task.name);
                    println!("    Prompt: {}", task.prompt);
                    if let Some(out) = &task.output {
                        println!("    Output: {out}");
                    }
                }
                println!();
            }
            Ok(())
        }
    }
}
