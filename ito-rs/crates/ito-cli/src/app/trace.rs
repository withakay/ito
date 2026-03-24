//! CLI adapter for the `ito trace` command.

use clap::Args;

use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_core::trace::compute_trace_output;

/// Show requirement traceability for a change.
#[derive(Args, Debug, Clone)]
pub struct TraceArgs {
    /// Change id (positional or via --change flag)
    #[arg(
        value_name = "CHANGE",
        required_unless_present = "change_flag",
        conflicts_with = "change_flag"
    )]
    pub change: Option<String>,
    /// Change id (flag form)
    #[arg(
        short = 'c',
        long = "change",
        value_name = "CHANGE",
        conflicts_with = "change"
    )]
    pub change_flag: Option<String>,
    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

/// Handles the `ito trace` CLI subcommand.
///
/// Validates and resolves the requested change target, computes the trace output for that
/// change, and prints either pretty JSON (when `args.json` is true) or a human-readable report
/// describing lifecycle, status, coverage, declared requirements, covered/uncovered items,
/// unresolved task references, and diagnostics.
///
/// # Examples
///
/// ```text
/// // Construct a Runtime and TraceArgs appropriate for your environment, then:
/// // handle_trace_clap(&runtime, &args).unwrap();
/// ```
pub(crate) fn handle_trace_clap(rt: &Runtime, args: &TraceArgs) -> CliResult<()> {
    let change_input = args
        .change
        .as_deref()
        .or(args.change_flag.as_deref())
        .unwrap_or("");

    if change_input.is_empty() {
        return fail(
            "Change id is required. Try:\n  ito trace <change-id>\n  ito trace --change <change-id>",
        );
    }

    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let change_repo = runtime.repositories().changes.as_ref();

    let actual = match super::common::resolve_change_target(change_repo, change_input) {
        Ok(id) => id,
        Err(msg) => return fail(msg),
    };

    let output = compute_trace_output(change_repo, &actual).map_err(to_cli_error)?;

    if args.json {
        let rendered = serde_json::to_string_pretty(&output).expect("json should serialize");
        println!("{rendered}");
        return Ok(());
    }

    // Human-readable output.
    let lifecycle_label = if output.lifecycle == "archived" {
        " (historical)"
    } else {
        ""
    };
    println!(
        "Trace: {} ({}{})",
        output.change_id, output.lifecycle, lifecycle_label
    );

    match output.status.as_str() {
        "unavailable" => {
            let reason = output.reason.as_deref().unwrap_or("unknown");
            println!("Status: Unavailable — {reason}");
        }
        "invalid" => {
            let reason = output.reason.as_deref().unwrap_or("unknown");
            println!("Status: Invalid — {reason}");
        }
        "ready" => {
            let total = output.declared_requirements.len();
            let covered = output.covered.len();
            let uncovered = output.uncovered.len();
            println!(
                "Status: Ready — {covered}/{total} requirements covered, {uncovered} uncovered"
            );

            if !output.declared_requirements.is_empty() {
                println!("\nDeclared Requirements:");
                for req_id in &output.declared_requirements {
                    println!("  {req_id}");
                }
            }

            if !output.covered.is_empty() {
                println!("\nCovered:");
                for entry in &output.covered {
                    println!(
                        "  {} → tasks: {}",
                        entry.requirement_id,
                        entry.covering_tasks.join(", ")
                    );
                }
            }

            if !output.uncovered.is_empty() {
                println!("\nUncovered (no active task):");
                for req_id in &output.uncovered {
                    println!("  {req_id}");
                }
            }

            if !output.unresolved.is_empty() {
                println!("\nUnresolved Task References:");
                for entry in &output.unresolved {
                    println!(
                        "  task {} → unknown requirement {}",
                        entry.task_id, entry.requirement_id
                    );
                }
            }

            if !output.diagnostics.is_empty() {
                println!("\nDiagnostics:");
                for d in &output.diagnostics {
                    println!("  {d}");
                }
            }
        }
        other => {
            println!("Status: {other}");
        }
    }

    Ok(())
}
