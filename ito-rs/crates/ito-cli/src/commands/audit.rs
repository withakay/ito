use clap::{Args, Subcommand};

use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_core::audit::AuditEvent;
use ito_core::audit::{self, EventFilter, read_audit_events, read_audit_events_filtered};

/// Query and manage audit event log.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct AuditArgs {
    #[command(subcommand)]
    pub action: Option<AuditAction>,
}

/// Audit subcommands.
#[derive(Subcommand, Debug, Clone)]
pub enum AuditAction {
    /// Show audit log entries
    #[command(visible_alias = "lo")]
    Log {
        /// Filter to a specific change
        #[arg(long)]
        change: Option<String>,

        /// Filter by entity type (task, change, module, config)
        #[arg(long)]
        entity: Option<String>,

        /// Filter by operation (create, status_change, archive, etc.)
        #[arg(long)]
        op: Option<String>,

        /// Maximum number of events to show
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Detect and optionally fix drift between audit log and file state
    #[command(visible_alias = "re")]
    Reconcile {
        /// Target a specific change (or reconcile all if omitted)
        #[arg(long)]
        change: Option<String>,

        /// Write compensating events to fix detected drift
        #[arg(long)]
        fix: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Validate audit log integrity and consistency
    #[command(visible_alias = "va")]
    Validate {
        /// Target a specific change
        #[arg(long)]
        change: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show audit log statistics
    #[command(visible_alias = "st")]
    Stats {
        /// Filter to a specific change
        #[arg(long)]
        change: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Stream audit events in real-time
    #[command(visible_alias = "sm")]
    Stream {
        /// Include events from all worktrees
        #[arg(long)]
        all_worktrees: bool,

        /// Show last N events on startup
        #[arg(long, default_value_t = 10)]
        last: usize,

        /// Output as JSON lines
        #[arg(long)]
        json: bool,
    },
}

pub(crate) fn handle_audit_clap(rt: &Runtime, args: &AuditArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return fail("Missing required audit subcommand");
    };

    let ito_path = rt.ito_path();

    match action {
        AuditAction::Log {
            change,
            entity,
            op,
            limit,
            json,
        } => {
            let events = if entity.is_some() || change.is_some() || op.is_some() {
                let filter = EventFilter {
                    entity: entity.clone(),
                    scope: change.clone(),
                    op: op.clone(),
                };
                read_audit_events_filtered(ito_path, &filter)
            } else {
                read_audit_events(ito_path)
            };

            let events: Vec<&AuditEvent> = if let Some(n) = limit {
                let start = events.len().saturating_sub(*n);
                events[start..].iter().collect()
            } else {
                events.iter().collect()
            };

            if *json {
                let json_events: Vec<serde_json::Value> = events
                    .iter()
                    .map(|e| serde_json::to_value(e).unwrap())
                    .collect();
                let rendered = serde_json::to_string_pretty(&json_events).map_err(to_cli_error)?;
                println!("{rendered}");
            } else {
                if events.is_empty() {
                    println!("No audit events found.");
                    return Ok(());
                }
                for event in &events {
                    print_event_line(event);
                }
                println!();
                println!("{} events", events.len());
            }

            Ok(())
        }
        AuditAction::Reconcile { change, fix, json } => {
            let report = audit::run_reconcile(ito_path, change.as_deref(), *fix);

            if *json {
                let drifts: Vec<String> = report.drifts.iter().map(|d| d.to_string()).collect();
                let rendered = serde_json::to_string_pretty(&serde_json::json!({
                    "scope": report.scoped_to,
                    "drifts": drifts,
                    "drift_count": report.drifts.len(),
                    "events_written": report.events_written,
                    "fix": fix,
                }))
                .map_err(to_cli_error)?;
                println!("{rendered}");
            } else {
                println!("Reconcile: {}", report.scoped_to);
                println!("──────────────────────────────────────────────────");

                if report.drifts.is_empty() {
                    println!("No drift detected. Audit log and files are in sync.");
                } else {
                    println!("{} drift items found:", report.drifts.len());
                    println!();
                    for drift in &report.drifts {
                        println!("  - {drift}");
                    }
                    println!();
                    if *fix {
                        println!("Wrote {} compensating events.", report.events_written);
                    } else {
                        println!("Run with --fix to write compensating events.");
                    }
                }
            }

            Ok(())
        }
        AuditAction::Validate { change, json } => {
            let report = ito_core::audit::validate::validate_audit_log(ito_path, change.as_deref());

            if *json {
                let issues: Vec<serde_json::Value> = report
                    .issues
                    .iter()
                    .map(|i| {
                        serde_json::json!({
                            "level": i.level,
                            "message": i.message,
                            "event_index": i.event_index,
                        })
                    })
                    .collect();
                let rendered = serde_json::to_string_pretty(&serde_json::json!({
                    "scope": change.as_deref().unwrap_or("project"),
                    "event_count": report.event_count,
                    "issue_count": report.issues.len(),
                    "issues": issues,
                    "valid": report.valid,
                }))
                .map_err(to_cli_error)?;
                println!("{rendered}");
            } else {
                let scope = change.as_deref().unwrap_or("project");
                println!("Audit Validate: {scope}");
                println!("──────────────────────────────────────────────────");
                println!("Events: {}", report.event_count);

                if report.issues.is_empty() {
                    println!("No issues found.");
                } else {
                    println!("{} issues found:", report.issues.len());
                    for issue in &report.issues {
                        println!("  - [{}] {}", issue.level, issue.message);
                    }
                }
            }

            Ok(())
        }
        AuditAction::Stats { change, json } => {
            let events = if let Some(change_id) = change {
                let filter = EventFilter {
                    scope: Some(change_id.clone()),
                    ..Default::default()
                };
                read_audit_events_filtered(ito_path, &filter)
            } else {
                read_audit_events(ito_path)
            };

            // Compute statistics
            let mut entity_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            let mut op_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            let mut actor_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            let mut scope_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();

            for event in &events {
                *entity_counts.entry(event.entity.clone()).or_default() += 1;
                *op_counts.entry(event.op.clone()).or_default() += 1;
                *actor_counts.entry(event.actor.clone()).or_default() += 1;
                if let Some(scope) = &event.scope {
                    *scope_counts.entry(scope.clone()).or_default() += 1;
                }
            }

            if *json {
                let rendered = serde_json::to_string_pretty(&serde_json::json!({
                    "scope": change.as_deref().unwrap_or("project"),
                    "total_events": events.len(),
                    "by_entity": entity_counts,
                    "by_op": op_counts,
                    "by_actor": actor_counts,
                    "by_scope": scope_counts,
                }))
                .map_err(to_cli_error)?;
                println!("{rendered}");
            } else {
                let scope = change.as_deref().unwrap_or("project");
                println!("Audit Stats: {scope}");
                println!("──────────────────────────────────────────────────");
                println!("Total events: {}", events.len());

                if !entity_counts.is_empty() {
                    println!();
                    println!("By entity:");
                    let mut sorted: Vec<_> = entity_counts.iter().collect();
                    sorted.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
                    for (entity, count) in sorted {
                        println!("  {entity}: {count}");
                    }
                }

                if !op_counts.is_empty() {
                    println!();
                    println!("By operation:");
                    let mut sorted: Vec<_> = op_counts.iter().collect();
                    sorted.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
                    for (op, count) in sorted {
                        println!("  {op}: {count}");
                    }
                }

                if !actor_counts.is_empty() {
                    println!();
                    println!("By actor:");
                    let mut sorted: Vec<_> = actor_counts.iter().collect();
                    sorted.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
                    for (actor, count) in sorted {
                        println!("  {actor}: {count}");
                    }
                }

                if !scope_counts.is_empty() {
                    println!();
                    println!("By change:");
                    let mut sorted: Vec<_> = scope_counts.iter().collect();
                    sorted.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
                    for (scope, count) in sorted {
                        println!("  {scope}: {count}");
                    }
                }
            }

            Ok(())
        }
        AuditAction::Stream {
            all_worktrees,
            last,
            json,
        } => {
            // For now, implement a simple one-shot tail (streaming requires a file watcher)
            let events = read_audit_events(ito_path);

            let start = events.len().saturating_sub(*last);
            let tail = &events[start..];

            if *json {
                for event in tail {
                    let line = serde_json::to_string(event).map_err(to_cli_error)?;
                    println!("{line}");
                }
            } else {
                for event in tail {
                    print_event_line(event);
                }
            }

            if *all_worktrees {
                let worktrees = audit::discover_worktrees(ito_path);
                let wt_events = audit::aggregate_worktree_events(&worktrees);
                for (wt, events) in &wt_events {
                    let label = wt.branch.as_deref().unwrap_or("(detached)");
                    let start = events.len().saturating_sub(*last);
                    let tail = &events[start..];
                    if *json {
                        for event in tail {
                            let line = serde_json::to_string(event).map_err(to_cli_error)?;
                            println!("{line}");
                        }
                    } else if !tail.is_empty() {
                        println!();
                        println!("── Worktree: {label} ({}) ──", wt.path.display());
                        for event in tail {
                            print_event_line(event);
                        }
                    }
                }
            }

            Ok(())
        }
    }
}

/// Print a human-readable single line for an audit event.
fn print_event_line(event: &AuditEvent) {
    let scope = event.scope.as_deref().unwrap_or("-");
    let transition = match (&event.from, &event.to) {
        (Some(from), Some(to)) => format!("{from} -> {to}"),
        (None, Some(to)) => format!("-> {to}"),
        (Some(from), None) => format!("{from} ->"),
        (None, None) => String::new(),
    };
    println!(
        "{ts}  {actor:<10} {entity}/{id} ({scope})  {op}  {transition}",
        ts = &event.ts[..19.min(event.ts.len())],
        actor = event.actor,
        entity = event.entity,
        id = event.entity_id,
        scope = scope,
        op = event.op,
        transition = transition,
    );
}
