use crate::cli_error::{CliError, CliResult, to_cli_error};
use ito_core::ChangeRepository;
use ito_core::tasks::{
    ChangeTargetResolution, TaskDiagnostic, TaskItem, TaskStatus, TaskStatusResult,
    TaskStatusSummary, TasksFormat,
};
use std::path::{Path, PathBuf};

pub(super) fn resolve_change_id(
    change_repo: &dyn ChangeRepository,
    input: &str,
) -> CliResult<String> {
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

pub(super) fn task_status_label(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Pending => "pending",
        TaskStatus::InProgress => "in_progress",
        TaskStatus::Complete => "complete",
        TaskStatus::Shelved => "shelved",
    }
}

pub(super) fn tasks_format_label(format: TasksFormat) -> &'static str {
    match format {
        TasksFormat::Enhanced => "enhanced",
        TasksFormat::Checkbox => "checkbox",
    }
}

pub(super) fn json_task(task: &TaskItem) -> serde_json::Value {
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

pub(super) fn json_diagnostic(path: &Path, d: &TaskDiagnostic) -> serde_json::Value {
    serde_json::json!({
        "level": d.level.as_str(),
        "message": &d.message,
        "task_id": &d.task_id,
        "line": d.line,
        "path": path.display().to_string(),
    })
}

pub(super) fn backend_tasks_path() -> PathBuf {
    PathBuf::from("backend tasks")
}

pub(super) fn missing_tasks_message(path: &Path, change_id: &str) -> String {
    let file = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("tracking file");
    format!("No {file} found for \"{change_id}\". Run \"ito tasks init {change_id}\" first.")
}

pub(super) fn summarize_status(status: TaskStatusResult) -> TaskStatusSummary {
    TaskStatusSummary {
        format: status.format,
        items: status.items,
        progress: status.progress,
        diagnostics: status.diagnostics,
        ready: status.ready,
        blocked: status.blocked,
    }
}

pub(super) fn print_json(value: &serde_json::Value) -> CliResult<()> {
    let rendered = serde_json::to_string_pretty(value).map_err(to_cli_error)?;
    println!("{rendered}");
    Ok(())
}
