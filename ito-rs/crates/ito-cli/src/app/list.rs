use crate::cli::{ListArgs, ListSortOrder};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use chrono::{DateTime, Utc};
use ito_common::paths as core_paths;
use ito_domain::changes::{ChangeRepository, ChangeStatus};
use ito_domain::modules::ModuleRepository;

#[derive(Debug, serde::Serialize)]
struct ModulesResponse {
    modules: Vec<ito_core::list::ModuleListItem>,
}

#[derive(Debug, serde::Serialize)]
struct ChangesResponse {
    changes: Vec<ito_core::list::ChangeListItem>,
}

#[derive(Debug, serde::Serialize)]
struct SpecsResponse {
    specs: Vec<ito_core::list::SpecListItem>,
}

pub(crate) fn handle_list(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["list"], "ito list")
        );
        return Ok(());
    }

    let want_specs = args.iter().any(|a| a == "--specs");
    let want_modules = args.iter().any(|a| a == "--modules");
    let want_json = args.iter().any(|a| a == "--json");
    let want_ready = args.iter().any(|a| a == "--ready");
    let want_completed = args.iter().any(|a| a == "--completed");
    let want_partial = args.iter().any(|a| a == "--partial");
    let want_pending = args.iter().any(|a| a == "--pending");

    let progress_filter_count =
        u8::from(want_completed) + u8::from(want_partial) + u8::from(want_pending);
    if progress_filter_count > 1 {
        return fail("Flags --completed, --partial, and --pending are mutually exclusive.");
    }

    let sort = parse_sort_order(args).unwrap_or("recent");
    let mode = if want_specs {
        "specs"
    } else if want_modules {
        "modules"
    } else {
        // default is changes, and `--changes` is a no-op.
        "changes"
    };

    let ito_path = rt.ito_path();

    match mode {
        "modules" => {
            let module_repo = ModuleRepository::new(ito_path);
            let modules = module_repo.list().map_err(to_cli_error)?;

            if want_json {
                // Convert to legacy format for JSON compatibility
                let legacy_modules: Vec<ito_core::list::ModuleListItem> = modules
                    .iter()
                    .map(|m| ito_core::list::ModuleListItem {
                        id: m.id.clone(),
                        name: m.name.clone(),
                        full_name: format!("{}_{}", m.id, m.name),
                        change_count: m.change_count as usize,
                    })
                    .collect();
                let payload = ModulesResponse {
                    modules: legacy_modules,
                };
                let rendered =
                    serde_json::to_string_pretty(&payload).expect("json should serialize");
                println!("{rendered}");
                return Ok(());
            }

            if modules.is_empty() {
                println!("No modules found.");
                println!("Create one with: ito create module <name>");
                return Ok(());
            }

            println!("Modules:\n");
            for m in modules {
                let full_name = format!("{}_{}", m.id, m.name);
                if m.change_count == 0 {
                    println!("  {full_name}");
                    continue;
                }
                let suffix = if m.change_count == 1 {
                    "change"
                } else {
                    "changes"
                };
                println!("  {full_name} ({} {suffix})", m.change_count);
            }
            println!();
        }
        "specs" => {
            let specs = ito_core::list::list_specs(ito_path).unwrap_or_default();
            if specs.is_empty() {
                // TS prints a plain sentence even for `--json`.
                println!("No specs found.");
                return Ok(());
            }

            if want_json {
                let payload = SpecsResponse { specs };
                let rendered =
                    serde_json::to_string_pretty(&payload).expect("json should serialize");
                println!("{rendered}");
                return Ok(());
            }

            println!("Specs:");
            let padding = "  ";
            let name_width = specs.iter().map(|s| s.id.len()).max().unwrap_or(0);
            for s in specs {
                let padded = format!("{id: <width$}", id = s.id, width = name_width);
                println!("{padding}{padded}     requirements {}", s.requirement_count);
            }
        }
        _ => {
            // changes
            let changes_dir = core_paths::changes_dir(ito_path);
            if !changes_dir.exists() {
                return fail("No Ito changes directory found. Run 'ito init' first.");
            }

            let change_repo = ChangeRepository::new(ito_path);
            let mut summaries = change_repo.list().map_err(to_cli_error)?;

            // Filter to ready changes if requested
            if want_ready {
                summaries.retain(|s| s.is_ready());
            }

            // Filter to completed changes if requested
            if want_completed {
                summaries.retain(is_completed);
            }

            // Filter to partially complete changes if requested
            if want_partial {
                summaries.retain(is_partial);
            }

            // Filter to pending changes if requested
            if want_pending {
                summaries.retain(is_pending);
            }

            if summaries.is_empty() {
                if want_json {
                    let rendered =
                        serde_json::to_string_pretty(&serde_json::json!({ "changes": [] }))
                            .expect("json should serialize");
                    println!("{rendered}");
                } else if want_completed {
                    println!("No completed changes found.");
                    println!("Run `ito list` to see all changes.");
                } else if want_partial {
                    println!("No partially complete changes found.");
                    println!("Run `ito list` to see all changes.");
                } else if want_pending {
                    println!("No pending changes found.");
                    println!("Run `ito list` to see all changes.");
                } else {
                    println!("No active changes found.");
                }
                return Ok(());
            }

            // Sort according to preference
            if sort == "name" {
                summaries.sort_by(|a, b| a.id.cmp(&b.id));
            } else {
                summaries.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
            }

            if want_json {
                let changes: Vec<ito_core::list::ChangeListItem> = summaries
                    .iter()
                    .map(|s| {
                        let status = match s.status() {
                            ChangeStatus::NoTasks => "no-tasks",
                            ChangeStatus::InProgress => "in-progress",
                            ChangeStatus::Complete => "complete",
                        };
                        ito_core::list::ChangeListItem {
                            name: s.id.clone(),
                            completed_tasks: s.completed_tasks,
                            shelved_tasks: s.shelved_tasks,
                            in_progress_tasks: s.in_progress_tasks,
                            pending_tasks: s.pending_tasks,
                            total_tasks: s.total_tasks,
                            last_modified: ito_core::list::to_iso_millis(s.last_modified),
                            status: status.to_string(),
                            work_status: s.work_status().to_string(),
                            completed: is_completed(s),
                        }
                    })
                    .collect();
                let payload = ChangesResponse { changes };
                let rendered =
                    serde_json::to_string_pretty(&payload).expect("json should serialize");
                println!("{rendered}");
                return Ok(());
            }

            println!("Changes:");
            let name_width = summaries.iter().map(|s| s.id.len()).max().unwrap_or(0);
            for s in &summaries {
                let status = format_task_status(s);
                let time_ago = format_relative_time(s.last_modified);
                let padded = format!("{: <width$}", s.id, width = name_width);
                println!("  {padded}     {: <20}  {time_ago}", status);
            }
        }
    }

    Ok(())
}

pub(crate) fn handle_list_clap(rt: &Runtime, args: &ListArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();
    if args.specs {
        argv.push("--specs".to_string());
    }
    if args.changes {
        argv.push("--changes".to_string());
    }
    if args.modules {
        argv.push("--modules".to_string());
    }
    if args.ready {
        argv.push("--ready".to_string());
    }
    if args.completed {
        argv.push("--completed".to_string());
    }
    if args.partial {
        argv.push("--partial".to_string());
    }
    if args.pending {
        argv.push("--pending".to_string());
    }
    if args.json {
        argv.push("--json".to_string());
    }

    let sort = match args.sort {
        ListSortOrder::Recent => "recent",
        ListSortOrder::Name => "name",
    };
    argv.push("--sort".to_string());
    argv.push(sort.to_string());

    handle_list(rt, &argv)
}

fn parse_sort_order(args: &[String]) -> Option<&str> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        if a == "--sort" {
            return iter.next().map(|s| s.as_str());
        }
        if let Some(v) = a.strip_prefix("--sort=") {
            return Some(v);
        }
    }
    None
}

fn format_task_status(s: &ito_domain::changes::ChangeSummary) -> String {
    if s.total_tasks == 0 {
        return "No tasks".to_string();
    }

    // Build status string showing all relevant states
    let mut parts = Vec::new();

    if s.completed_tasks > 0 {
        parts.push(format!("{}c", s.completed_tasks));
    }
    if s.shelved_tasks > 0 {
        parts.push(format!("{}s", s.shelved_tasks));
    }
    if s.in_progress_tasks > 0 {
        parts.push(format!("{}i", s.in_progress_tasks));
    }
    if s.pending_tasks > 0 {
        parts.push(format!("{}p", s.pending_tasks));
    }

    let counts = parts.join("/");

    // Add work status indicator
    use ito_domain::changes::ChangeWorkStatus;
    match s.work_status() {
        ChangeWorkStatus::Complete => format!("\u{2713} Complete ({})", counts),
        ChangeWorkStatus::Paused => format!("\u{2016} Paused ({})", counts),
        ChangeWorkStatus::InProgress => format!("\u{25B6} Active ({})", counts),
        ChangeWorkStatus::Ready => counts,
        ChangeWorkStatus::Draft => format!("{} (draft)", counts),
    }
}

fn is_completed(s: &ito_domain::changes::ChangeSummary) -> bool {
    use ito_domain::changes::ChangeWorkStatus;
    matches!(
        s.work_status(),
        ChangeWorkStatus::Complete | ChangeWorkStatus::Paused
    )
}

fn is_partial(s: &ito_domain::changes::ChangeSummary) -> bool {
    use ito_domain::changes::ChangeWorkStatus;
    matches!(
        s.work_status(),
        ChangeWorkStatus::Ready | ChangeWorkStatus::InProgress
    ) && s.total_tasks > 0
        && s.completed_tasks > 0
        && s.completed_tasks < s.total_tasks
}

fn is_pending(s: &ito_domain::changes::ChangeSummary) -> bool {
    use ito_domain::changes::ChangeWorkStatus;
    matches!(
        s.work_status(),
        ChangeWorkStatus::Ready | ChangeWorkStatus::InProgress
    ) && s.total_tasks > 0
        && s.completed_tasks == 0
}

fn format_relative_time(then: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(then);
    let secs = diff.num_seconds();
    if secs <= 0 {
        return "just now".to_string();
    }
    let mins = diff.num_minutes();
    let hours = diff.num_hours();
    let days = diff.num_days();

    if days > 30 {
        // Node's `toLocaleDateString()` is locale-dependent; in our parity harness
        // environment it renders as M/D/YYYY.
        return then.format("%-m/%-d/%Y").to_string();
    }

    if days > 0 {
        format!("{days}d ago")
    } else if hours > 0 {
        format!("{hours}h ago")
    } else if mins > 0 {
        format!("{mins}m ago")
    } else {
        "just now".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        format_relative_time, format_task_status, handle_list, is_completed, is_partial,
        is_pending, parse_sort_order,
    };
    use crate::runtime::Runtime;
    use chrono::{Duration, Utc};

    #[test]
    fn parse_sort_order_supports_separate_and_equals_forms() {
        let args = vec!["--sort".to_string(), "name".to_string()];
        assert_eq!(parse_sort_order(&args), Some("name"));

        let args = vec!["--sort=recent".to_string()];
        assert_eq!(parse_sort_order(&args), Some("recent"));
    }

    #[test]
    fn format_task_status_handles_various_states() {
        use chrono::Utc;
        use ito_domain::changes::ChangeSummary;

        let make_summary = |completed, shelved, in_progress, pending, total| ChangeSummary {
            id: "test".to_string(),
            module_id: None,
            completed_tasks: completed,
            shelved_tasks: shelved,
            in_progress_tasks: in_progress,
            pending_tasks: pending,
            total_tasks: total,
            last_modified: Utc::now(),
            has_proposal: true,
            has_design: false,
            has_specs: true,
            has_tasks: true,
        };

        // No tasks
        let s = make_summary(0, 0, 0, 0, 0);
        assert_eq!(format_task_status(&s), "No tasks");

        // All complete
        let s = make_summary(3, 0, 0, 0, 3);
        assert!(format_task_status(&s).contains("Complete"));
        assert!(format_task_status(&s).contains("3c"));

        // Paused (complete + shelved = total, shelved > 0)
        let s = make_summary(2, 1, 0, 0, 3);
        assert!(format_task_status(&s).contains("Paused"));
        assert!(format_task_status(&s).contains("2c"));
        assert!(format_task_status(&s).contains("1s"));

        // In progress
        let s = make_summary(1, 0, 1, 1, 3);
        assert!(format_task_status(&s).contains("Active"));
        assert!(format_task_status(&s).contains("1i"));

        // Ready (pending work, nothing in progress)
        let s = make_summary(1, 0, 0, 2, 3);
        let status = format_task_status(&s);
        assert!(status.contains("1c"));
        assert!(status.contains("2p"));
    }

    #[test]
    fn progress_predicates_work_with_change_summary() {
        use chrono::Utc;
        use ito_domain::changes::ChangeSummary;

        let make_summary = |completed, shelved, in_progress, pending, total| ChangeSummary {
            id: "test".to_string(),
            module_id: None,
            completed_tasks: completed,
            shelved_tasks: shelved,
            in_progress_tasks: in_progress,
            pending_tasks: pending,
            total_tasks: total,
            last_modified: Utc::now(),
            has_proposal: true,
            has_design: false,
            has_specs: true,
            has_tasks: total > 0,
        };

        // No tasks - none of the predicates match
        let s = make_summary(0, 0, 0, 0, 0);
        assert!(!is_completed(&s));
        assert!(!is_partial(&s));
        assert!(!is_pending(&s));

        // All pending (ready state, no completed)
        let s = make_summary(0, 0, 0, 3, 3);
        assert!(is_pending(&s));
        assert!(!is_partial(&s));
        assert!(!is_completed(&s));

        // Partial (some completed, some pending)
        let s = make_summary(1, 0, 0, 2, 3);
        assert!(is_partial(&s));
        assert!(!is_pending(&s));
        assert!(!is_completed(&s));

        // Not partial when 0 tasks are complete
        let s = make_summary(0, 0, 0, 3, 3);
        assert!(!is_partial(&s));

        // All completed
        let s = make_summary(3, 0, 0, 0, 3);
        assert!(is_completed(&s));
        assert!(!is_partial(&s));
        assert!(!is_pending(&s));

        // Paused (completed + shelved = total) should count as completed
        let s = make_summary(2, 1, 0, 0, 3);
        assert!(is_completed(&s)); // Paused counts as "completed" for filtering
        assert!(!is_partial(&s));
        assert!(!is_pending(&s));
    }

    #[test]
    fn progress_filter_flags_are_mutually_exclusive() {
        let rt = Runtime::new();
        let args = vec!["--pending".to_string(), "--partial".to_string()];
        let err = handle_list(&rt, &args).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Flags --completed, --partial, and --pending are mutually exclusive."
        );
    }

    #[test]
    fn format_relative_time_covers_major_buckets() {
        assert_eq!(
            format_relative_time(Utc::now() + Duration::seconds(1)),
            "just now"
        );

        let then = Utc::now() - Duration::minutes(2);
        assert_eq!(format_relative_time(then), "2m ago");

        let then = Utc::now() - Duration::hours(2);
        assert_eq!(format_relative_time(then), "2h ago");

        let then = Utc::now() - Duration::days(2);
        assert_eq!(format_relative_time(then), "2d ago");

        let then = Utc::now() - Duration::days(40);
        assert_eq!(
            format_relative_time(then),
            then.format("%-m/%-d/%Y").to_string()
        );
    }
}
