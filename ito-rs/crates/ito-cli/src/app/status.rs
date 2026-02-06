use crate::cli::StatusArgs;
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_core::workflow as core_workflow;
use ito_domain::changes::ChangeRepository;

pub(crate) fn handle_status(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["status"], "ito status")
        );
        return Ok(());
    }

    let want_json = args.iter().any(|a| a == "--json");
    let change = parse_string_flag(args, "--change");
    if change.as_deref().unwrap_or("").is_empty() {
        let change_repo = ChangeRepository::new(rt.ito_path());
        let changes = change_repo.list().unwrap_or_default();
        let mut msg = "Missing required option --change".to_string();
        if !changes.is_empty() {
            msg.push_str("\n\nAvailable changes:\n");
            for c in changes {
                msg.push_str(&format!("  {}\n", c.id));
            }
        }
        return fail(msg);
    }

    let schema = parse_string_flag(args, "--schema");
    let ctx = rt.ctx();
    let ito_path = rt.ito_path();

    // Match TS/ora: spinner output is written to stderr.
    eprintln!("- Loading change status...");

    let change = change.expect("checked above");
    let change = match super::common::resolve_change_target(ito_path, &change) {
        Ok(resolved) => resolved,
        Err(msg) => return fail(msg),
    };
    let status =
        match core_workflow::compute_change_status(ito_path, &change, schema.as_deref(), ctx) {
            Ok(s) => s,
            Err(core_workflow::WorkflowError::InvalidChangeName) => {
                return fail("Invalid change name");
            }
            Err(core_workflow::WorkflowError::ChangeNotFound(name)) => {
                return fail(format!("Change '{name}' not found"));
            }
            Err(core_workflow::WorkflowError::SchemaNotFound(name)) => {
                return fail(super::common::schema_not_found_message(ctx, &name));
            }
            Err(e) => {
                return Err(to_cli_error(e));
            }
        };

    if want_json {
        let rendered = serde_json::to_string_pretty(&status).expect("json should serialize");
        println!("{rendered}");
        return Ok(());
    }

    let total = status.artifacts.len();
    let done = status
        .artifacts
        .iter()
        .filter(|a| a.status == "done")
        .count();

    println!("Change: {}", status.change_name);
    println!("Schema: {}", status.schema_name);
    println!("Progress: {done}/{total} artifacts complete\n");
    for a in &status.artifacts {
        let mark = if a.status == "done" {
            "[x]"
        } else if a.status == "blocked" {
            "[-]"
        } else {
            "[ ]"
        };

        if a.status == "blocked" && !a.missing_deps.is_empty() {
            println!(
                "{mark} {} (blocked by: {})",
                a.id,
                a.missing_deps.join(", ")
            );
        } else {
            println!("{mark} {}", a.id);
        }
    }
    if status.is_complete {
        println!("\nAll artifacts complete!");
    }

    Ok(())
}

pub(crate) fn handle_status_clap(rt: &Runtime, args: &StatusArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();
    if let Some(change) = &args.change {
        argv.push("--change".to_string());
        argv.push(change.clone());
    }
    if let Some(schema) = &args.schema {
        argv.push("--schema".to_string());
        argv.push(schema.clone());
    }
    if args.json {
        argv.push("--json".to_string());
    }
    handle_status(rt, &argv)
}
