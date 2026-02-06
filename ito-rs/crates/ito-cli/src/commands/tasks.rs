use crate::cli::{TasksAction, TasksArgs};
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::diagnostics;
use crate::runtime::Runtime;
use ito_common::paths as core_paths;
use ito_core::change_repository::FsChangeRepository;
use ito_domain::changes::ChangeTargetResolution;
use ito_domain::tasks as wf_tasks;

fn resolve_change_id(ito_path: &std::path::Path, input: &str) -> CliResult<String> {
    let change_repo = FsChangeRepository::new(ito_path);
    match change_repo.resolve_target(input) {
        ChangeTargetResolution::Unique(id) => Ok(id),
        ChangeTargetResolution::Ambiguous(matches) => {
            let mut msg = format!("Change '{input}' is ambiguous. Matches:\n");
            for id in matches.iter().take(8) {
                msg.push_str(&format!("  {id}\n"));
            }
            if matches.len() > 8 {
                msg.push_str(&format!("  ... and {} more\n", matches.len() - 8));
            }
            msg.push_str("Use a longer prefix or the full canonical change ID.");
            Err(CliError::msg(msg))
        }
        ChangeTargetResolution::NotFound => {
            let mut msg = format!("Change '{input}' not found");
            let suggestions = change_repo.suggest_targets(input, 5);
            if !suggestions.is_empty() {
                msg.push_str("\n\nDid you mean:\n");
                for suggestion in suggestions {
                    msg.push_str(&format!("  {suggestion}\n"));
                }
            }
            Err(CliError::msg(msg))
        }
    }
}

fn task_status_label(status: wf_tasks::TaskStatus) -> &'static str {
    match status {
        wf_tasks::TaskStatus::Pending => "pending",
        wf_tasks::TaskStatus::InProgress => "in_progress",
        wf_tasks::TaskStatus::Complete => "complete",
        wf_tasks::TaskStatus::Shelved => "shelved",
    }
}

fn tasks_format_label(format: wf_tasks::TasksFormat) -> &'static str {
    match format {
        wf_tasks::TasksFormat::Enhanced => "enhanced",
        wf_tasks::TasksFormat::Checkbox => "checkbox",
    }
}

fn json_task(task: &wf_tasks::TaskItem) -> serde_json::Value {
    serde_json::json!({
        "id": &task.id,
        "name": &task.name,
        "wave": task.wave,
        "status": task_status_label(task.status),
        "updated_at": &task.updated_at,
        "dependencies": &task.dependencies,
        "files": &task.files,
        "action": &task.action,
        "verify": &task.verify,
        "done_when": &task.done_when,
        "kind": format!("{:?}", task.kind).to_lowercase(),
        "header_line_index": task.header_line_index,
    })
}

fn json_diagnostic(path: &std::path::Path, d: &wf_tasks::TaskDiagnostic) -> serde_json::Value {
    serde_json::json!({
        "level": d.level.as_str(),
        "message": &d.message,
        "task_id": &d.task_id,
        "line": d.line,
        "path": path.display().to_string(),
    })
}

fn print_json(value: &serde_json::Value) -> CliResult<()> {
    let rendered = serde_json::to_string_pretty(value).map_err(to_cli_error)?;
    println!("{rendered}");
    Ok(())
}

pub(crate) fn handle_tasks_clap(rt: &Runtime, args: &TasksArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        // Preserve legacy behavior: `ito tasks` errors.
        return fail("Missing required argument <change-id>");
    };

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

    // Handle "ready" specially since change_id is optional
    if sub == "ready" {
        return handle_tasks_ready(rt, args);
    }

    let input_change_id = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if input_change_id.is_empty() || input_change_id.starts_with('-') {
        return fail("Missing required argument <change-id>");
    }

    let change_id = resolve_change_id(ito_path, input_change_id)?;

    let change_dir = core_paths::change_dir(ito_path, &change_id);

    match sub {
        "init" => {
            if !change_dir.exists() {
                return fail(format!("Change '{change_id}' not found"));
            }
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            if path.exists() {
                return fail(format!(
                    "tasks.md already exists for \"{change_id}\". Use \"tasks add\" to add tasks."
                ));
            }

            let now = chrono::Local::now();
            let contents = wf_tasks::enhanced_tasks_template(&change_id, now);
            if let Some(parent) = path.parent() {
                ito_common::io::create_dir_all(parent).map_err(to_cli_error)?;
            }
            ito_common::io::write(&path, contents.as_bytes()).map_err(to_cli_error)?;
            if want_json {
                return print_json(&serde_json::json!({
                    "action": "init",
                    "change_id": change_id,
                    "path": path.display().to_string(),
                    "created": true,
                }));
            }
            eprintln!("✔ Enhanced tasks.md created for \"{change_id}\"");
            Ok(())
        }
        "status" => {
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            if !path.exists() {
                if want_json {
                    return print_json(&serde_json::json!({
                        "action": "status",
                        "change_id": change_id,
                        "path": path.display().to_string(),
                        "exists": false,
                        "message": format!("No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."),
                    }));
                }
                println!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                );
                return Ok(());
            }

            let contents = ito_common::io::read_to_string(&path)
                .map_err(|_| CliError::msg(format!("Failed to read {}", path.display())))?;

            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let (ready, blocked) = wf_tasks::compute_ready_and_blocked(&parsed);
            if want_json {
                let warnings: Vec<serde_json::Value> = parsed
                    .diagnostics
                    .iter()
                    .filter(|d| d.level == wf_tasks::DiagnosticLevel::Warning)
                    .map(|d| json_diagnostic(&path, d))
                    .collect();
                let ready_tasks: Vec<serde_json::Value> = ready.iter().map(json_task).collect();
                let blocked_tasks: Vec<serde_json::Value> = blocked
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
                    "ready_tasks": ready_tasks,
                    "blocked_tasks": blocked_tasks,
                }));
            }

            println!("Tasks for: {change_id}");
            println!("──────────────────────────────────────────────────");
            println!();

            let warnings = diagnostics::render_task_diagnostics(
                &path,
                &parsed.diagnostics,
                wf_tasks::DiagnosticLevel::Warning,
            );
            if !warnings.is_empty() {
                println!("Warnings");
                print!("{warnings}");
                println!();
            }

            match parsed.format {
                wf_tasks::TasksFormat::Enhanced => {
                    let done = parsed.progress.complete + parsed.progress.shelved;
                    println!(
                        "Progress: {}/{} done ({} complete, {} shelved), {} in-progress, {} pending",
                        done,
                        parsed.progress.total,
                        parsed.progress.complete,
                        parsed.progress.shelved,
                        parsed.progress.in_progress,
                        parsed.progress.pending
                    );
                }
                wf_tasks::TasksFormat::Checkbox => {
                    println!(
                        "Progress (compat): {}/{} complete, {} in-progress, {} pending",
                        parsed.progress.complete,
                        parsed.progress.total,
                        parsed.progress.in_progress,
                        parsed.progress.pending
                    );
                }
            }

            println!();
            println!("Ready");
            for t in &ready {
                println!("  - {}: {}", t.id, t.name);
            }
            println!();
            println!("Blocked");
            for (t, blockers) in &blocked {
                println!("  - {}: {}", t.id, t.name);
                for b in blockers {
                    println!("    - {b}");
                }
            }

            Ok(())
        }
        "next" => {
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path).map_err(|_| {
                CliError::msg(format!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            match parsed.format {
                wf_tasks::TasksFormat::Checkbox => {
                    let current = parsed
                        .tasks
                        .iter()
                        .find(|t| t.status == wf_tasks::TaskStatus::InProgress);
                    if let Some(t) = current {
                        if want_json {
                            return print_json(&serde_json::json!({
                                "action": "next",
                                "change_id": change_id,
                                "format": "checkbox",
                                "state": "current",
                                "task": json_task(t),
                            }));
                        }
                        println!("Current Task (compat)");
                        println!("──────────────────────────────────────────────────");
                        println!("Task {}: {}", t.id, t.name);
                        println!("Run \"ito tasks complete {change_id} {}\" when done", t.id);
                        return Ok(());
                    }

                    let next = parsed
                        .tasks
                        .iter()
                        .find(|t| t.status == wf_tasks::TaskStatus::Pending);
                    if let Some(t) = next {
                        if want_json {
                            return print_json(&serde_json::json!({
                                "action": "next",
                                "change_id": change_id,
                                "format": "checkbox",
                                "state": "next",
                                "task": json_task(t),
                            }));
                        }
                        println!("Next Task (compat)");
                        println!("──────────────────────────────────────────────────");
                        println!("Task {}: {}", t.id, t.name);
                        println!("Run \"ito tasks start {change_id} {}\" to begin", t.id);
                        println!("Run \"ito tasks complete {change_id} {}\" when done", t.id);
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
                wf_tasks::TasksFormat::Enhanced => {
                    if parsed.progress.remaining == 0 {
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

                    let (ready, blocked) = wf_tasks::compute_ready_and_blocked(&parsed);
                    if ready.is_empty() {
                        if want_json {
                            let first_blocked = blocked.first().map(|(task, blockers)| {
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
                        if let Some((t, blockers)) = blocked.first() {
                            println!("First blocked task: {} - {}", t.id, t.name);
                            println!("{}", format_blockers(blockers));
                        }
                        return Ok(());
                    }

                    let t = &ready[0];
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
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path).map_err(|_| {
                CliError::msg(format!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);
            if parsed.format == wf_tasks::TasksFormat::Checkbox {
                let Some(current) = parsed
                    .tasks
                    .iter()
                    .find(|t| t.status == wf_tasks::TaskStatus::InProgress)
                else {
                    let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
                        return fail(format!("Task \"{task_id}\" not found"));
                    };
                    match task.status {
                        wf_tasks::TaskStatus::Pending => {}
                        wf_tasks::TaskStatus::InProgress => {
                            return fail(format!("Task \"{task_id}\" is already in-progress"));
                        }
                        wf_tasks::TaskStatus::Complete => {
                            return fail(format!("Task \"{task_id}\" is already complete"));
                        }
                        wf_tasks::TaskStatus::Shelved => {
                            return fail("Checkbox-only tasks.md does not support shelving.");
                        }
                    }

                    let updated = wf_tasks::update_checkbox_task_status(
                        &contents,
                        task_id,
                        wf_tasks::TaskStatus::InProgress,
                    )
                    .map_err(CliError::msg)?;
                    ito_common::io::write(&path, updated.as_bytes()).map_err(to_cli_error)?;
                    if want_json {
                        return print_json(&serde_json::json!({
                            "action": "start",
                            "change_id": change_id,
                            "task_id": task_id,
                            "format": "checkbox",
                            "status": "in_progress",
                        }));
                    }
                    eprintln!("✔ Task \"{task_id}\" marked as in-progress");
                    return Ok(());
                };

                return fail(format!(
                    "Task \"{}\" is already in-progress (complete it before starting another task)",
                    current.id
                ));
            }

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
                return fail(format!("Task \"{task_id}\" not found in tasks.md"));
            };
            let status_label = match task.status {
                wf_tasks::TaskStatus::Pending => "pending",
                wf_tasks::TaskStatus::InProgress => "in_progress",
                wf_tasks::TaskStatus::Complete => "complete",
                wf_tasks::TaskStatus::Shelved => "shelved",
            };
            if task.status == wf_tasks::TaskStatus::Shelved {
                return fail(format!(
                    "Task \"{task_id}\" is shelved (run \"ito tasks unshelve {change_id} {task_id}\" first)"
                ));
            }
            if task.status != wf_tasks::TaskStatus::Pending {
                return fail(format!(
                    "Task \"{task_id}\" is not pending (current: {status_label})"
                ));
            }

            let (ready, blocked) = wf_tasks::compute_ready_and_blocked(&parsed);
            if !ready.iter().any(|t| t.id == task_id) {
                if let Some((_, blockers)) = blocked.iter().find(|(t, _)| t.id == task_id) {
                    return fail(format_blockers(blockers));
                }
                return fail("Task is blocked");
            }

            let updated = wf_tasks::update_enhanced_task_status(
                &contents,
                task_id,
                wf_tasks::TaskStatus::InProgress,
                chrono::Local::now(),
            );
            ito_common::io::write(&path, updated.as_bytes()).map_err(to_cli_error)?;
            if want_json {
                return print_json(&serde_json::json!({
                    "action": "start",
                    "change_id": change_id,
                    "task_id": task_id,
                    "format": "enhanced",
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
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path).map_err(|_| {
                CliError::msg(format!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);
            if parsed.format == wf_tasks::TasksFormat::Checkbox {
                let updated = wf_tasks::update_checkbox_task_status(
                    &contents,
                    task_id,
                    wf_tasks::TaskStatus::Complete,
                )
                .map_err(CliError::msg)?;
                ito_common::io::write(&path, updated.as_bytes()).map_err(to_cli_error)?;
                if want_json {
                    return print_json(&serde_json::json!({
                        "action": "complete",
                        "change_id": change_id,
                        "task_id": task_id,
                        "format": "checkbox",
                        "status": "complete",
                    }));
                }
                eprintln!("✔ Task \"{task_id}\" marked as complete");
                return Ok(());
            }

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let updated = wf_tasks::update_enhanced_task_status(
                &contents,
                task_id,
                wf_tasks::TaskStatus::Complete,
                chrono::Local::now(),
            );
            ito_common::io::write(&path, updated.as_bytes()).map_err(to_cli_error)?;
            if want_json {
                return print_json(&serde_json::json!({
                    "action": "complete",
                    "change_id": change_id,
                    "task_id": task_id,
                    "format": "enhanced",
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
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path).map_err(|_| {
                CliError::msg(format!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);
            if parsed.format == wf_tasks::TasksFormat::Checkbox {
                return fail("Checkbox-only tasks.md does not support shelving.");
            }

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
                return fail(format!("Task \"{task_id}\" not found in tasks.md"));
            };
            if task.status == wf_tasks::TaskStatus::Complete {
                return fail(format!("Task \"{task_id}\" is already complete"));
            }

            let updated = wf_tasks::update_enhanced_task_status(
                &contents,
                task_id,
                wf_tasks::TaskStatus::Shelved,
                chrono::Local::now(),
            );
            ito_common::io::write(&path, updated.as_bytes()).map_err(to_cli_error)?;
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
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path).map_err(|_| {
                CliError::msg(format!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);
            if parsed.format == wf_tasks::TasksFormat::Checkbox {
                return fail("Checkbox-only tasks.md does not support shelving.");
            }

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
                return fail(format!("Task \"{task_id}\" not found in tasks.md"));
            };
            if task.status != wf_tasks::TaskStatus::Shelved {
                return fail(format!("Task \"{task_id}\" is not shelved"));
            }

            let updated = wf_tasks::update_enhanced_task_status(
                &contents,
                task_id,
                wf_tasks::TaskStatus::Pending,
                chrono::Local::now(),
            );
            ito_common::io::write(&path, updated.as_bytes()).map_err(to_cli_error)?;
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
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path).map_err(|_| {
                CliError::msg(format!(
                    "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
                ))
            })?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);
            if parsed.format != wf_tasks::TasksFormat::Enhanced {
                return fail(
                    "Cannot add tasks to checkbox-only tracking file. Convert to enhanced format first.",
                );
            }

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }

            let mut max_n = 0u32;
            for t in &parsed.tasks {
                if let Some((w, n)) = t.id.split_once('.')
                    && let (Ok(w), Ok(n)) = (w.parse::<u32>(), n.parse::<u32>())
                    && w == wave
                {
                    max_n = max_n.max(n);
                }
            }
            let new_id = format!("{wave}.{}", max_n + 1);

            let date = chrono::Local::now().format("%Y-%m-%d").to_string();
            let block = format!(
                "\n### Task {new_id}: {task_name}\n- **Files**: `path/to/file.rs`\n- **Dependencies**: None\n- **Action**:\n  [Describe what needs to be done]\n- **Verify**: `cargo test --workspace`\n- **Done When**: [Success criteria]\n- **Updated At**: {date}\n- **Status**: [ ] pending\n"
            );

            let mut out = contents.clone();
            if out.contains(&format!("## Wave {wave}")) {
                // Insert before the next major section after this wave.
                if let Some(pos) = out.find("## Checkpoints") {
                    out.insert_str(pos, &block);
                } else {
                    out.push_str(&block);
                }
            } else {
                // Create wave section before checkpoints (or at end).
                if let Some(pos) = out.find("## Checkpoints") {
                    out.insert_str(
                        pos,
                        &format!("\n---\n\n## Wave {wave}\n- **Depends On**: None\n"),
                    );
                    let pos2 = out.find("## Checkpoints").unwrap_or(out.len());
                    out.insert_str(pos2, &block);
                } else {
                    out.push_str(&format!(
                        "\n---\n\n## Wave {wave}\n- **Depends On**: None\n"
                    ));
                    out.push_str(&block);
                }
            }

            ito_common::io::write(&path, out.as_bytes()).map_err(to_cli_error)?;
            if want_json {
                return print_json(&serde_json::json!({
                    "action": "add",
                    "change_id": change_id,
                    "task_id": new_id,
                    "task_name": task_name,
                    "wave": wave,
                }));
            }
            eprintln!("✔ Task {new_id} \"{task_name}\" added to Wave {wave}");
            Ok(())
        }
        "show" => {
            let path = wf_tasks::tasks_path(ito_path, &change_id);
            let contents = ito_common::io::read_to_string(&path)
                .map_err(|_| CliError::msg(format!("tasks.md not found for \"{change_id}\"")))?;
            let parsed = wf_tasks::parse_tasks_tracking_file(&contents);

            if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics)
            {
                return Err(CliError::msg(msg));
            }
            if want_json {
                let tasks: Vec<serde_json::Value> = parsed.tasks.iter().map(json_task).collect();
                let waves: Vec<serde_json::Value> = parsed
                    .waves
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
                    .filter(|d| d.level == wf_tasks::DiagnosticLevel::Warning)
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
                    "raw": contents,
                }));
            }
            print!("{contents}");
            Ok(())
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
    let change_id = resolve_change_id(ito_path, change_id)?;
    let path = wf_tasks::tasks_path(ito_path, &change_id);

    let contents = ito_common::io::read_to_string(&path).map_err(|_| {
        CliError::msg(format!(
            "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
        ))
    })?;

    let parsed = wf_tasks::parse_tasks_tracking_file(&contents);

    if let Some(msg) = diagnostics::blocking_task_error_message(&path, &parsed.diagnostics) {
        return Err(CliError::msg(msg));
    }

    let (ready, _blocked) = wf_tasks::compute_ready_and_blocked(&parsed);

    if want_json {
        let json_tasks: Vec<serde_json::Value> = ready.iter().map(json_task).collect();
        return print_json(&serde_json::json!({
            "action": "ready",
            "change_id": change_id,
            "path": path.display().to_string(),
            "ready_tasks": json_tasks,
        }));
    }

    if ready.is_empty() {
        if parsed.progress.remaining == 0 {
            println!("All tasks complete for \"{change_id}\"!");
        } else {
            println!("No ready tasks for \"{change_id}\" (tasks may be blocked or shelved).");
        }
        return Ok(());
    }

    println!("Ready Tasks for: {change_id}");
    println!("──────────────────────────────────────────────────");
    println!();

    for t in &ready {
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
    let change_repo = FsChangeRepository::new(ito_path);
    let summaries = change_repo.list().map_err(to_cli_error)?;

    // Only process changes that are ready (have proposal, specs, tasks, and pending work)
    let ready_changes: Vec<_> = summaries.iter().filter(|s| s.is_ready()).collect();

    if ready_changes.is_empty() {
        if want_json {
            return print_json(&serde_json::json!([]));
        } else {
            println!("No ready changes found.");
        }
        return Ok(());
    }

    let mut all_results: Vec<serde_json::Value> = Vec::new();
    let mut has_any_tasks = false;

    for summary in &ready_changes {
        let path = wf_tasks::tasks_path(ito_path, &summary.id);
        let Ok(contents) = ito_common::io::read_to_string(&path) else {
            continue;
        };

        let parsed = wf_tasks::parse_tasks_tracking_file(&contents);

        // Skip if there are blocking errors
        if diagnostics::blocking_task_error_message(&path, &parsed.diagnostics).is_some() {
            continue;
        }

        let (ready, _blocked) = wf_tasks::compute_ready_and_blocked(&parsed);

        if ready.is_empty() {
            continue;
        }

        has_any_tasks = true;

        if want_json {
            let json_tasks: Vec<serde_json::Value> = ready.iter().map(json_task).collect();
            all_results.push(serde_json::json!({
                "action": "ready",
                "change_id": summary.id,
                "ready_tasks": json_tasks,
            }));
        } else {
            println!("{}:", summary.id);
            for t in &ready {
                println!("  {} - {}", t.id, t.name);
            }
            println!();
        }
    }

    if want_json {
        return print_json(&serde_json::json!(all_results));
    } else if !has_any_tasks {
        println!("No ready tasks found across any changes.");
    }

    Ok(())
}
