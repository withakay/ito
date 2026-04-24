use crate::cli::{TasksAction, TasksArgs};
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::commands::sync::best_effort_sync_coordination;
use crate::diagnostics;
use crate::runtime::Runtime;
use ito_core::audit::{Actor, AuditEventBuilder, EntityType, ops};
use ito_core::coordination_worktree::maybe_auto_commit_coordination;
use ito_core::repository_runtime::PersistenceMode;
use ito_core::tasks as core_tasks;
use ito_core::tasks::{DiagnosticLevel, TaskStatus, TasksFormat};

mod backend;
mod support;

use backend::{
    handle_backend_allocate, handle_backend_claim, handle_backend_release, handle_backend_sync,
    sync_after_mutation,
};
use support::{
    backend_tasks_path, json_diagnostic, json_task, missing_tasks_message, print_json,
    resolve_change_id, summarize_status, task_status_label, tasks_format_label,
};

/// Attempt to auto-commit the coordination worktree after a task mutation.
///
/// Failures are printed as warnings to stderr and never propagate — the
/// primary task operation has already succeeded at this point.
fn auto_commit_after_task_mutation(rt: &Runtime, change_id: &str, action: &str) {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let message = format!("chore: {action} task for change {change_id}");
    if let Err(err) = maybe_auto_commit_coordination(project_root, ito_path, &message) {
        eprintln!("Warning: auto-commit to coordination worktree failed: {err}");
    }
}

pub(crate) fn handle_tasks_clap(rt: &Runtime, args: &TasksArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        // Preserve legacy behavior: `ito tasks` errors.
        return fail("Missing required argument <change-id>");
    };

    // Handle backend coordination commands directly (they don't use the legacy handler)
    match action {
        TasksAction::Claim { change_id } => {
            return handle_backend_claim(rt, change_id, args.json);
        }
        TasksAction::Release { change_id } => {
            return handle_backend_release(rt, change_id, args.json);
        }
        TasksAction::Allocate => {
            return handle_backend_allocate(rt, args.json);
        }
        TasksAction::Sync(sync_action) => {
            return handle_backend_sync(rt, sync_action, args.json);
        }
        // All other actions fall through to the legacy forwarding handler below
        TasksAction::Init { .. }
        | TasksAction::Status { .. }
        | TasksAction::Next { .. }
        | TasksAction::Ready { .. }
        | TasksAction::Start { .. }
        | TasksAction::Complete { .. }
        | TasksAction::Shelve { .. }
        | TasksAction::Unshelve { .. }
        | TasksAction::Add { .. }
        | TasksAction::Show { .. }
        | TasksAction::External(_) => {}
    }

    let mut forwarded: Vec<String> = match action {
        TasksAction::Init { change_id } => vec!["init".to_string(), change_id.clone()],
        TasksAction::Status { change_id, wave } => {
            let mut out = vec!["status".to_string(), change_id.clone()];
            if let Some(wave) = wave {
                out.push("--wave".to_string());
                out.push(wave.to_string());
            }
            out
        }
        TasksAction::Next { change_id } => vec!["next".to_string(), change_id.clone()],
        TasksAction::Ready { change_id } => {
            let mut out = vec!["ready".to_string()];
            if let Some(id) = change_id {
                out.push(id.clone());
            }
            out
        }
        TasksAction::Start { change_id, task_id } => {
            vec!["start".to_string(), change_id.clone(), task_id.clone()]
        }
        TasksAction::Complete { change_id, task_id } => {
            vec!["complete".to_string(), change_id.clone(), task_id.clone()]
        }
        TasksAction::Shelve { change_id, task_id } => {
            vec!["shelve".to_string(), change_id.clone(), task_id.clone()]
        }
        TasksAction::Unshelve { change_id, task_id } => {
            vec!["unshelve".to_string(), change_id.clone(), task_id.clone()]
        }
        TasksAction::Add {
            change_id,
            task_name,
            wave,
        } => vec![
            "add".to_string(),
            change_id.clone(),
            task_name.clone(),
            "--wave".to_string(),
            wave.to_string(),
        ],
        TasksAction::Show { change_id } => vec!["show".to_string(), change_id.clone()],
        TasksAction::External(rest) => rest.clone(),
        // Backend commands handled above
        TasksAction::Claim { .. }
        | TasksAction::Release { .. }
        | TasksAction::Allocate
        | TasksAction::Sync(_) => unreachable!(),
    };

    if args.json {
        forwarded.push("--json".to_string());
    }
    handle_tasks(rt, &forwarded)
}

pub(crate) fn handle_tasks(rt: &Runtime, args: &[String]) -> CliResult<()> {
    fn parse_wave_flag(args: &[String]) -> u32 {
        args.iter()
            .enumerate()
            .find(|(_, a)| *a == "--wave")
            .and_then(|(i, _)| args.get(i + 1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(1)
    }

    fn format_blockers(blockers: &[String]) -> String {
        if blockers.is_empty() {
            return "Task is blocked".to_string();
        }
        let mut out = String::from("Task is blocked:");
        for b in blockers {
            out.push_str("\n- ");
            out.push_str(b);
        }
        out
    }

    let sub = args.first().map(|s| s.as_str()).unwrap_or("");
    let want_json = args.iter().any(|a| a == "--json");
    let ito_path = rt.ito_path();
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let repos = runtime.repositories();
    let change_repo = repos.changes.as_ref();
    let task_repo = repos.tasks.as_ref();
    let task_mutations = repos.task_mutations.as_ref();

    // Handle "ready" specially since change_id is optional
    if sub == "ready" {
        return handle_tasks_ready(rt, args);
    }
    let input_change_id = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if input_change_id.is_empty() || input_change_id.starts_with('-') {
        return fail("Missing required argument <change-id>");
    }

    let change_id = resolve_change_id(change_repo, input_change_id)?;

    match sub {
        "init" => {
            if runtime.mode() == PersistenceMode::Filesystem {
                let change_dir = ito_path.join("changes").join(&change_id);
                if !change_dir.exists() {
                    return fail(format!("Change '{change_id}' not found"));
                }
            }

            let result = task_mutations
                .init_tasks(&change_id)
                .map_err(to_cli_error)?;

            if result.existed {
                let file = result
                    .path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str())
                    .unwrap_or("backend tasks");
                return fail(format!(
                    "{file} already exists for \"{change_id}\". Use \"tasks add\" to add tasks."
                ));
            }

            if want_json {
                let path = result.path.as_ref().map(|p| p.display().to_string());
                return print_json(&serde_json::json!({
                    "action": "init",
                    "change_id": change_id,
                    "path": path,
                    "created": true,
                }));
            }
            let file = result
                .path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("backend tasks");
            eprintln!("✔ Enhanced {file} created for \"{change_id}\"");
            Ok(())
        }
        "status" => {
            let (path, status) = if runtime.mode() == PersistenceMode::Remote {
                let path = backend_tasks_path();
                let raw = task_mutations
                    .load_tasks_markdown(&change_id)
                    .map_err(to_cli_error)?;
                if raw.is_none() {
                    let message = missing_tasks_message(&path, &change_id);
                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "status",
                            "change_id": change_id,
                            "path": path.display().to_string(),
                            "exists": false,
                            "message": message,
                        }));
                    }
                    println!("{message}");
                    return Ok(());
                }
                let status = core_tasks::get_task_status_from_repository(task_repo, &change_id)
                    .map_err(to_cli_error)?;
                (path, status)
            } else {
                let path =
                    core_tasks::tracking_file_path(ito_path, &change_id).map_err(to_cli_error)?;

                if !path.exists() {
                    let message = missing_tasks_message(&path, &change_id);
                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "status",
                            "change_id": change_id,
                            "path": path.display().to_string(),
                            "exists": false,
                            "message": message,
                        }));
                    }
                    println!("{message}");
                    return Ok(());
                }

                let status =
                    core_tasks::get_task_status(ito_path, &change_id).map_err(to_cli_error)?;
                (path, summarize_status(status))
            };

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &status.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            if want_json {
                let warnings: Vec<serde_json::Value> = status
                    .diagnostics
                    .iter()
                    .filter(|d| d.level == DiagnosticLevel::Warning)
                    .map(|d| json_diagnostic(&path, d))
                    .collect();
                let ready_tasks: Vec<serde_json::Value> =
                    status.ready.iter().map(json_task).collect();
                let blocked_tasks: Vec<serde_json::Value> = status
                    .blocked
                    .iter()
                    .map(|(task, blockers)| {
                        serde_json::json!({
                            "task": json_task(task),
                            "blockers": blockers,
                        })
                    })
                    .collect();

                return print_json(&serde_json::json!({
                    "action": "status",
                    "change_id": change_id,
                    "path": path.display().to_string(),
                    "format": tasks_format_label(status.format),
                    "progress": {
                        "total": status.progress.total,
                        "complete": status.progress.complete,
                        "shelved": status.progress.shelved,
                        "in_progress": status.progress.in_progress,
                        "pending": status.progress.pending,
                        "remaining": status.progress.remaining,
                    },
                    "warnings": warnings,
                    "ready_tasks": ready_tasks,
                    "blocked_tasks": blocked_tasks,
                }));
            }

            println!("Tasks for: {change_id}");
            println!("──────────────────────────────────────────────────");
            println!();

            let warnings = diagnostics::render_task_diagnostics(
                &path,
                &status.diagnostics,
                DiagnosticLevel::Warning,
            );
            if !warnings.is_empty() {
                println!("Warnings");
                print!("{warnings}");
                println!();
            }

            match status.format {
                TasksFormat::Enhanced => {
                    let done = status.progress.complete + status.progress.shelved;
                    println!(
                        "Progress: {}/{} done ({} complete, {} shelved), {} in-progress, {} pending",
                        done,
                        status.progress.total,
                        status.progress.complete,
                        status.progress.shelved,
                        status.progress.in_progress,
                        status.progress.pending
                    );
                }
                TasksFormat::Checkbox => {
                    println!(
                        "Progress (compat): {}/{} complete, {} in-progress, {} pending",
                        status.progress.complete,
                        status.progress.total,
                        status.progress.in_progress,
                        status.progress.pending
                    );
                }
            }

            println!();
            println!("Ready");
            for t in &status.ready {
                println!("  - {}: {}", t.id, t.name);
            }
            println!();
            println!("Blocked");
            for (t, blockers) in &status.blocked {
                println!("  - {}: {}", t.id, t.name);
                for b in blockers {
                    println!("    - {b}");
                }
            }

            Ok(())
        }
        "next" => {
            let (path, status) = if runtime.mode() == PersistenceMode::Remote {
                let path = backend_tasks_path();
                let raw = task_mutations
                    .load_tasks_markdown(&change_id)
                    .map_err(to_cli_error)?;
                if raw.is_none() {
                    let message = missing_tasks_message(&path, &change_id);
                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "next",
                            "change_id": change_id,
                            "path": path.display().to_string(),
                            "exists": false,
                            "message": message,
                        }));
                    }
                    println!("{message}");
                    return Ok(());
                }
                let status = core_tasks::get_task_status_from_repository(task_repo, &change_id)
                    .map_err(to_cli_error)?;
                (path, status)
            } else {
                let path =
                    core_tasks::tracking_file_path(ito_path, &change_id).map_err(to_cli_error)?;
                let status =
                    core_tasks::get_task_status(ito_path, &change_id).map_err(to_cli_error)?;
                (path, summarize_status(status))
            };

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &status.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let file_label = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("tracking file");
            let next_task = core_tasks::get_next_task_from_summary(&status, file_label)
                .map_err(to_cli_error)?;

            match status.format {
                TasksFormat::Checkbox => {
                    if let Some(t) = next_task {
                        let state = if t.status == TaskStatus::InProgress {
                            "current"
                        } else {
                            "next"
                        };

                        if want_json {
                            return print_json(&serde_json::json!({
                                "action": "next",
                                "change_id": change_id,
                                "format": "checkbox",
                                "state": state,
                                "task": json_task(&t),
                            }));
                        }

                        if t.status == TaskStatus::InProgress {
                            println!("Current Task (compat)");
                            println!("──────────────────────────────────────────────────");
                            println!("Task {}: {}", t.id, t.name);
                            println!("Run \"ito tasks complete {change_id} {}\" when done", t.id);
                        } else {
                            println!("Next Task (compat)");
                            println!("──────────────────────────────────────────────────");
                            println!("Task {}: {}", t.id, t.name);
                            println!("Run \"ito tasks start {change_id} {}\" to begin", t.id);
                            println!("Run \"ito tasks complete {change_id} {}\" when done", t.id);
                        }
                        return Ok(());
                    }

                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "next",
                            "change_id": change_id,
                            "format": "checkbox",
                            "state": "complete",
                            "message": "All tasks complete!",
                        }));
                    }
                    println!("All tasks complete!");
                    Ok(())
                }
                TasksFormat::Enhanced => {
                    if status.progress.remaining == 0 {
                        if want_json {
                            return print_json(&serde_json::json!({
                                "action": "next",
                                "change_id": change_id,
                                "format": "enhanced",
                                "state": "complete",
                                "message": "All tasks complete!",
                            }));
                        }
                        println!("All tasks complete!");
                        return Ok(());
                    }

                    if status.ready.is_empty() {
                        if want_json {
                            let first_blocked = status.blocked.first().map(|(task, blockers)| {
                                serde_json::json!({
                                    "task": json_task(task),
                                    "blockers": blockers,
                                })
                            });
                            return print_json(&serde_json::json!({
                                "action": "next",
                                "change_id": change_id,
                                "format": "enhanced",
                                "state": "blocked",
                                "message": "No ready tasks.",
                                "first_blocked": first_blocked,
                            }));
                        }
                        println!("No ready tasks.");
                        if let Some((t, blockers)) = status.blocked.first() {
                            println!("First blocked task: {} - {}", t.id, t.name);
                            println!("{}", format_blockers(blockers));
                        }
                        return Ok(());
                    }

                    let t = &status.ready[0];
                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "next",
                            "change_id": change_id,
                            "format": "enhanced",
                            "state": "next",
                            "task": json_task(t),
                        }));
                    }
                    println!("Next Task");
                    println!("──────────────────────────────────────────────────");
                    println!("Task {}: {}", t.id, t.name);
                    println!();
                    if !t.files.is_empty() {
                        println!("Files: {}", t.files.join(", "));
                    }
                    if !t.action.trim().is_empty() {
                        println!("Action:");
                        for line in t.action.lines() {
                            println!("  {line}");
                        }
                    }
                    if let Some(v) = &t.verify {
                        println!("Verify: {v}");
                    }
                    if let Some(v) = &t.done_when {
                        println!("Done When: {v}");
                    }
                    println!();
                    println!("Run \"ito tasks start {change_id} {}\" to begin", t.id);
                    Ok(())
                }
            }
        }
        "start" => {
            let task_id = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if task_id.is_empty() || task_id.starts_with('-') {
                return fail("Missing required argument <task-id>");
            }

            best_effort_sync_coordination(rt, "before task start");

            let _task = task_mutations
                .start_task(&change_id, task_id)
                .map_err(to_cli_error)?;

            // Emit audit event for task start
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Task)
                .entity_id(task_id)
                .scope(&change_id)
                .op(ops::TASK_STATUS_CHANGE)
                .from("pending")
                .to("in-progress")
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            // Best-effort backend sync after mutation
            sync_after_mutation(rt, &change_id);
            // Best-effort auto-commit to coordination worktree
            auto_commit_after_task_mutation(rt, &change_id, "start");
            best_effort_sync_coordination(rt, "after task start");

            if want_json {
                let status = if runtime.mode() == PersistenceMode::Remote {
                    core_tasks::get_task_status_from_repository(task_repo, &change_id)
                        .map_err(to_cli_error)?
                } else {
                    let status =
                        core_tasks::get_task_status(ito_path, &change_id).map_err(to_cli_error)?;
                    summarize_status(status)
                };
                return print_json(&serde_json::json!({
                    "action": "start",
                    "change_id": change_id,
                    "task_id": task_id,
                    "format": tasks_format_label(status.format),
                    "status": "in_progress",
                }));
            }
            eprintln!("✔ Task \"{task_id}\" marked as in-progress");
            Ok(())
        }
        "complete" => {
            let task_id = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if task_id.is_empty() || task_id.starts_with('-') {
                return fail("Missing required argument <task-id>");
            }

            let _task = task_mutations
                .complete_task(&change_id, task_id, None)
                .map_err(to_cli_error)?;

            // Emit audit event for task completion
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Task)
                .entity_id(task_id)
                .scope(&change_id)
                .op(ops::TASK_STATUS_CHANGE)
                .to("complete")
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            // Best-effort backend sync after mutation
            sync_after_mutation(rt, &change_id);
            // Best-effort auto-commit to coordination worktree
            auto_commit_after_task_mutation(rt, &change_id, "complete");
            best_effort_sync_coordination(rt, "after task complete");

            if want_json {
                let status = if runtime.mode() == PersistenceMode::Remote {
                    core_tasks::get_task_status_from_repository(task_repo, &change_id)
                        .map_err(to_cli_error)?
                } else {
                    let status =
                        core_tasks::get_task_status(ito_path, &change_id).map_err(to_cli_error)?;
                    summarize_status(status)
                };
                return print_json(&serde_json::json!({
                    "action": "complete",
                    "change_id": change_id,
                    "task_id": task_id,
                    "format": tasks_format_label(status.format),
                    "status": "complete",
                }));
            }
            eprintln!("✔ Task \"{task_id}\" marked as complete");
            Ok(())
        }
        "shelve" => {
            let task_id = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if task_id.is_empty() || task_id.starts_with('-') {
                return fail("Missing required argument <task-id>");
            }

            let _task = task_mutations
                .shelve_task(&change_id, task_id, None)
                .map_err(to_cli_error)?;

            // Emit audit event for task shelve
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Task)
                .entity_id(task_id)
                .scope(&change_id)
                .op(ops::TASK_STATUS_CHANGE)
                .to("shelved")
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            // Best-effort backend sync after mutation
            sync_after_mutation(rt, &change_id);
            // Best-effort auto-commit to coordination worktree
            auto_commit_after_task_mutation(rt, &change_id, "shelve");
            best_effort_sync_coordination(rt, "after task shelve");

            if want_json {
                return print_json(&serde_json::json!({
                    "action": "shelve",
                    "change_id": change_id,
                    "task_id": task_id,
                    "status": "shelved",
                }));
            }
            eprintln!("✔ Task \"{task_id}\" shelved");
            Ok(())
        }
        "unshelve" => {
            let task_id = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if task_id.is_empty() || task_id.starts_with('-') {
                return fail("Missing required argument <task-id>");
            }

            let _task = task_mutations
                .unshelve_task(&change_id, task_id)
                .map_err(to_cli_error)?;

            // Emit audit event for task unshelve
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Task)
                .entity_id(task_id)
                .scope(&change_id)
                .op(ops::TASK_STATUS_CHANGE)
                .from("shelved")
                .to("pending")
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            // Best-effort backend sync after mutation
            sync_after_mutation(rt, &change_id);
            // Best-effort auto-commit to coordination worktree
            auto_commit_after_task_mutation(rt, &change_id, "unshelve");
            best_effort_sync_coordination(rt, "after task unshelve");

            if want_json {
                return print_json(&serde_json::json!({
                    "action": "unshelve",
                    "change_id": change_id,
                    "task_id": task_id,
                    "status": "pending",
                }));
            }
            eprintln!("✔ Task \"{task_id}\" unshelved (pending)");
            Ok(())
        }
        "add" => {
            let task_name = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if task_name.is_empty() || task_name.starts_with('-') {
                return fail("Missing required argument <task-name>");
            }
            let wave = parse_wave_flag(args);

            let task = task_mutations
                .add_task(&change_id, task_name, Some(wave))
                .map_err(to_cli_error)?
                .task;

            // Emit audit event for task add
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Task)
                .entity_id(&task.id)
                .scope(&change_id)
                .op(ops::TASK_ADD)
                .to("pending")
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .meta(serde_json::json!({
                    "wave": wave,
                    "name": task_name,
                }))
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            // Best-effort backend sync after mutation
            sync_after_mutation(rt, &change_id);
            // Best-effort auto-commit to coordination worktree
            auto_commit_after_task_mutation(rt, &change_id, "add");
            best_effort_sync_coordination(rt, "after task add");

            if want_json {
                return print_json(&serde_json::json!({
                    "action": "add",
                    "change_id": change_id,
                    "task_id": task.id,
                    "task_name": task_name,
                    "wave": wave,
                }));
            }
            eprintln!("✔ Task {} \"{task_name}\" added to Wave {wave}", task.id);
            Ok(())
        }
        "show" => {
            if runtime.mode() == PersistenceMode::Remote {
                let path = backend_tasks_path();
                let raw = task_mutations
                    .load_tasks_markdown(&change_id)
                    .map_err(to_cli_error)?;
                if raw.is_none() {
                    let message = missing_tasks_message(&path, &change_id);
                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "show",
                            "change_id": change_id,
                            "path": path.display().to_string(),
                            "exists": false,
                            "message": message,
                        }));
                    }
                    println!("{message}");
                    return Ok(());
                }
                let parsed = task_repo.load_tasks(&change_id).map_err(to_cli_error)?;

                if let Some(msg) =
                    diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
                {
                    return Err(CliError::msg(msg));
                }

                if want_json {
                    let tasks: Vec<serde_json::Value> =
                        parsed.tasks.iter().map(json_task).collect();
                    let mut wave_refs: Vec<_> = parsed.waves.iter().collect();
                    wave_refs.sort_by_key(|wave| wave.wave);
                    let waves: Vec<serde_json::Value> = wave_refs
                        .iter()
                        .map(|wave| {
                            serde_json::json!({
                                "wave": wave.wave,
                                "depends_on": wave.depends_on,
                                "header_line_index": wave.header_line_index,
                                "depends_on_line_index": wave.depends_on_line_index,
                            })
                        })
                        .collect();
                    let warnings: Vec<serde_json::Value> = parsed
                        .diagnostics
                        .iter()
                        .filter(|d| d.level == DiagnosticLevel::Warning)
                        .map(|d| json_diagnostic(&path, d))
                        .collect();
                    return print_json(&serde_json::json!({
                        "action": "show",
                        "change_id": change_id,
                        "path": path.display().to_string(),
                        "format": tasks_format_label(parsed.format),
                        "progress": {
                            "total": parsed.progress.total,
                            "complete": parsed.progress.complete,
                            "shelved": parsed.progress.shelved,
                            "in_progress": parsed.progress.in_progress,
                            "pending": parsed.progress.pending,
                            "remaining": parsed.progress.remaining,
                        },
                        "warnings": warnings,
                        "waves": waves,
                        "tasks": tasks,
                        "raw": raw,
                    }));
                }

                if let Some(contents) = raw {
                    print!("{contents}");
                    return Ok(());
                }

                println!("Tasks for: {change_id}");
                println!("──────────────────────────────────────────────────");
                for task in parsed.tasks {
                    println!(
                        "{} [{}] {}",
                        task.id,
                        task_status_label(task.status),
                        task.name
                    );
                }
                println!();
                println!("Backend tasks markdown is not available.");
                Ok(())
            } else {
                let path =
                    core_tasks::tracking_file_path(ito_path, &change_id).map_err(to_cli_error)?;
                let status =
                    core_tasks::get_task_status(ito_path, &change_id).map_err(to_cli_error)?;

                if let Some(msg) =
                    diagnostics::blocking_task_error_message(&path, &status.diagnostics)
                {
                    return Err(CliError::msg(msg));
                }

                if want_json {
                    let contents = core_tasks::read_tasks_markdown(ito_path, &change_id)
                        .map_err(to_cli_error)?;
                    let parsed = core_tasks::parse_tasks_tracking_file(&contents);

                    let tasks: Vec<serde_json::Value> =
                        status.items.iter().map(json_task).collect();
                    let mut wave_refs: Vec<_> = parsed.waves.iter().collect();
                    wave_refs.sort_by_key(|wave| wave.wave);
                    let waves: Vec<serde_json::Value> = wave_refs
                        .iter()
                        .map(|wave| {
                            serde_json::json!({
                                "wave": wave.wave,
                                "depends_on": wave.depends_on,
                                "header_line_index": wave.header_line_index,
                                "depends_on_line_index": wave.depends_on_line_index,
                            })
                        })
                        .collect();
                    let warnings: Vec<serde_json::Value> = status
                        .diagnostics
                        .iter()
                        .filter(|d| d.level == DiagnosticLevel::Warning)
                        .map(|d| json_diagnostic(&path, d))
                        .collect();
                    return print_json(&serde_json::json!({
                        "action": "show",
                        "change_id": change_id,
                        "path": path.display().to_string(),
                        "format": tasks_format_label(status.format),
                        "progress": {
                            "total": status.progress.total,
                            "complete": status.progress.complete,
                            "shelved": status.progress.shelved,
                            "in_progress": status.progress.in_progress,
                            "pending": status.progress.pending,
                            "remaining": status.progress.remaining,
                        },
                        "warnings": warnings,
                        "waves": waves,
                        "tasks": tasks,
                        "raw": contents,
                    }));
                }

                let contents =
                    core_tasks::read_tasks_markdown(ito_path, &change_id).map_err(to_cli_error)?;
                print!("{contents}");
                Ok(())
            }
        }
        _ => fail(format!("Unknown tasks subcommand '{sub}'")),
    }
}

/// Handle `tasks ready [change_id] [--json]`
fn handle_tasks_ready(rt: &Runtime, args: &[String]) -> CliResult<()> {
    let want_json = args.iter().any(|a| a == "--json");

    // Check if we have a change_id (arg after "ready" that doesn't start with -)
    let change_id = args
        .get(1)
        .filter(|s| !s.starts_with('-'))
        .map(|s| s.as_str());

    if let Some(change_id) = change_id {
        // Single change mode
        handle_tasks_ready_single(rt, change_id, want_json)
    } else {
        // All changes mode
        handle_tasks_ready_all(rt, want_json)
    }
}

/// Show ready tasks for a single change
fn handle_tasks_ready_single(rt: &Runtime, change_id: &str, want_json: bool) -> CliResult<()> {
    let ito_path = rt.ito_path();
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let repos = runtime.repositories();
    let change_repo = repos.changes.as_ref();
    let task_repo = repos.tasks.as_ref();
    let task_mutations = repos.task_mutations.as_ref();
    let change_id = resolve_change_id(change_repo, change_id)?;
    let (path, status) = if runtime.mode() == PersistenceMode::Remote {
        let path = backend_tasks_path();
        let raw = task_mutations
            .load_tasks_markdown(&change_id)
            .map_err(to_cli_error)?;
        if raw.is_none() {
            let message = missing_tasks_message(&path, &change_id);
            if want_json {
                return print_json(&serde_json::json!({
                    "action": "ready",
                    "change_id": change_id,
                    "path": path.display().to_string(),
                    "exists": false,
                    "message": message,
                    "ready_tasks": [],
                }));
            }
            println!("{message}");
            return Ok(());
        }
        let status = core_tasks::get_task_status_from_repository(task_repo, &change_id)
            .map_err(to_cli_error)?;
        (path, status)
    } else {
        let path = core_tasks::tracking_file_path(ito_path, &change_id).map_err(to_cli_error)?;
        let status = core_tasks::get_task_status(ito_path, &change_id).map_err(to_cli_error)?;
        (path, summarize_status(status))
    };

    if let Some(msg) = diagnostics::blocking_task_error_message(&path, &status.diagnostics) {
        return Err(CliError::msg(msg));
    }

    if want_json {
        let json_tasks: Vec<serde_json::Value> = status.ready.iter().map(json_task).collect();
        return print_json(&serde_json::json!({
            "action": "ready",
            "change_id": change_id,
            "path": path.display().to_string(),
            "ready_tasks": json_tasks,
        }));
    }

    if status.ready.is_empty() {
        if status.progress.remaining == 0 {
            println!("All tasks complete for \"{change_id}\"!");
        } else {
            println!("No ready tasks for \"{change_id}\" (tasks may be blocked or shelved).");
        }
        return Ok(());
    }

    println!("Ready Tasks for: {change_id}");
    println!("──────────────────────────────────────────────────");
    println!();

    for t in &status.ready {
        println!("Task {}: {}", t.id, t.name);
        if !t.files.is_empty() {
            println!("  Files: {}", t.files.join(", "));
        }
    }

    println!();
    println!("Run \"ito tasks start {change_id} <task-id>\" to begin a task");

    Ok(())
}

/// Show ready tasks across all changes
fn handle_tasks_ready_all(rt: &Runtime, want_json: bool) -> CliResult<()> {
    let ito_path = rt.ito_path();
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let repos = runtime.repositories();
    let change_repo = repos.changes.as_ref();
    let ready_changes = if runtime.mode() == PersistenceMode::Remote {
        core_tasks::list_ready_tasks_across_changes_with_repo(change_repo, repos.tasks.as_ref())
            .map_err(to_cli_error)?
    } else {
        core_tasks::list_ready_tasks_across_changes(change_repo, ito_path).map_err(to_cli_error)?
    };

    if ready_changes.is_empty() {
        if want_json {
            return print_json(&serde_json::json!([]));
        } else {
            println!("No ready changes found.");
        }
        return Ok(());
    }

    let mut all_results: Vec<serde_json::Value> = Vec::new();

    for change in &ready_changes {
        if want_json {
            let json_tasks: Vec<serde_json::Value> =
                change.ready_tasks.iter().map(json_task).collect();
            all_results.push(serde_json::json!({
                "action": "ready",
                "change_id": change.change_id,
                "ready_tasks": json_tasks,
            }));
        } else {
            println!("{}:", change.change_id);
            for t in &change.ready_tasks {
                println!("  {} - {}", t.id, t.name);
            }
            println!();
        }
    }

    if want_json {
        return print_json(&serde_json::json!(all_results));
    }

    Ok(())
}
