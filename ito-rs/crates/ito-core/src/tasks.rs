//! Task-oriented orchestration use-cases for adapters.

use std::path::{Path, PathBuf};

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use ito_domain::changes::ChangeRepository as DomainChangeRepository;

// Re-export domain types and functions for CLI convenience
pub use ito_domain::changes::ChangeTargetResolution;
pub use ito_domain::tasks::{
    DiagnosticLevel, ProgressInfo, TaskDiagnostic, TaskItem, TaskKind, TaskStatus, TasksFormat,
    TasksParseResult, WaveInfo, compute_ready_and_blocked, enhanced_tasks_template,
    parse_tasks_tracking_file, tasks_path, update_checkbox_task_status,
    update_enhanced_task_status,
};

/// Ready task list for a single change.
#[derive(Debug, Clone)]
pub struct ReadyTasksForChange {
    /// Canonical change id.
    pub change_id: String,
    /// Ready tasks from `tasks.md` after dependency computation.
    pub ready_tasks: Vec<TaskItem>,
}

/// Collect ready tasks across all currently ready changes.
///
/// This use-case keeps repository traversal and task orchestration in core,
/// while adapters remain focused on argument parsing and presentation.
pub fn list_ready_tasks_across_changes(
    change_repo: &impl DomainChangeRepository,
    ito_path: &Path,
) -> CoreResult<Vec<ReadyTasksForChange>> {
    let summaries = change_repo.list().into_core()?;

    let mut results: Vec<ReadyTasksForChange> = Vec::new();
    for summary in &summaries {
        if !summary.is_ready() {
            continue;
        }

        let path = tasks_path(ito_path, &summary.id);
        let Ok(contents) = ito_common::io::read_to_string(&path) else {
            continue;
        };

        let parsed = parse_tasks_tracking_file(&contents);
        if parsed
            .diagnostics
            .iter()
            .any(|d| d.level == ito_domain::tasks::DiagnosticLevel::Error)
        {
            continue;
        }

        let (ready, _blocked) = compute_ready_and_blocked(&parsed);
        if ready.is_empty() {
            continue;
        }

        results.push(ReadyTasksForChange {
            change_id: summary.id.clone(),
            ready_tasks: ready,
        });
    }

    Ok(results)
}

/// Result of getting task status for a change.
#[derive(Debug, Clone)]
pub struct TaskStatusResult {
    /// Detected file format.
    pub format: TasksFormat,
    /// All parsed tasks.
    pub items: Vec<TaskItem>,
    /// Progress summary.
    pub progress: ProgressInfo,
    /// Parse diagnostics.
    pub diagnostics: Vec<TaskDiagnostic>,
    /// Ready tasks (computed).
    pub ready: Vec<TaskItem>,
    /// Blocked tasks with their blockers.
    pub blocked: Vec<(TaskItem, Vec<String>)>,
}

/// Initialize a tasks.md file for a change.
///
/// Returns the path to the created file and whether it already existed.
pub fn init_tasks(ito_path: &Path, change_id: &str) -> CoreResult<(PathBuf, bool)> {
    let path = tasks_path(ito_path, change_id);

    if path.exists() {
        return Ok((path, true));
    }

    let now = chrono::Local::now();
    let contents = enhanced_tasks_template(change_id, now);

    if let Some(parent) = path.parent() {
        ito_common::io::create_dir_all_std(parent)
            .map_err(|e| CoreError::io("create tasks.md parent directory", e))?;
    }

    ito_common::io::write_std(&path, contents.as_bytes())
        .map_err(|e| CoreError::io("write tasks.md", e))?;

    Ok((path, false))
}

/// Get task status for a change.
///
/// Reads and parses the tasks.md file, computes ready/blocked tasks.
pub fn get_task_status(ito_path: &Path, change_id: &str) -> CoreResult<TaskStatusResult> {
    let path = tasks_path(ito_path, change_id);

    if !path.exists() {
        return Err(CoreError::not_found(format!(
            "No tasks.md found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
        )));
    }

    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);
    let (ready, blocked) = compute_ready_and_blocked(&parsed);

    Ok(TaskStatusResult {
        format: parsed.format,
        items: parsed.tasks,
        progress: parsed.progress,
        diagnostics: parsed.diagnostics,
        ready,
        blocked,
    })
}

/// Get the next actionable task for a change.
///
/// Returns None if all tasks are complete or if no tasks are ready.
pub fn get_next_task(ito_path: &Path, change_id: &str) -> CoreResult<Option<TaskItem>> {
    let status = get_task_status(ito_path, change_id)?;

    // Check for errors
    if status
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    // All complete?
    if status.progress.remaining == 0 {
        return Ok(None);
    }

    match status.format {
        TasksFormat::Checkbox => {
            // Check for current in-progress task
            if let Some(current) = status
                .items
                .iter()
                .find(|t| t.status == TaskStatus::InProgress)
            {
                return Ok(Some(current.clone()));
            }

            // Find first pending task
            Ok(status
                .items
                .iter()
                .find(|t| t.status == TaskStatus::Pending)
                .cloned())
        }
        TasksFormat::Enhanced => {
            // Return first ready task
            Ok(status.ready.first().cloned())
        }
    }
}

/// Start a task (transition to in_progress).
///
/// Validates preconditions and updates the tasks.md file.
pub fn start_task(ito_path: &Path, change_id: &str, task_id: &str) -> CoreResult<TaskItem> {
    let path = tasks_path(ito_path, change_id);
    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);

    // Check for errors
    if parsed
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    // Find the task
    let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    };

    // Checkbox format: check for existing in-progress task
    if parsed.format == TasksFormat::Checkbox
        && let Some(current) = parsed
            .tasks
            .iter()
            .find(|t| t.status == TaskStatus::InProgress)
        && current.id != task_id
    {
        return Err(CoreError::validation(format!(
            "Task \"{}\" is already in-progress (complete it before starting another task)",
            current.id
        )));
    }

    if parsed.format == TasksFormat::Checkbox {
        // Validate status
        match task.status {
            TaskStatus::Pending => {}
            TaskStatus::InProgress => {
                return Err(CoreError::validation(format!(
                    "Task \"{task_id}\" is already in-progress"
                )));
            }
            TaskStatus::Complete => {
                return Err(CoreError::validation(format!(
                    "Task \"{task_id}\" is already complete"
                )));
            }
            TaskStatus::Shelved => {
                return Err(CoreError::validation(
                    "Checkbox-only tasks.md does not support shelving".to_string(),
                ));
            }
        }

        let updated = update_checkbox_task_status(&contents, task_id, TaskStatus::InProgress)
            .map_err(CoreError::validation)?;
        ito_common::io::write_std(&path, updated.as_bytes())
            .map_err(|e| CoreError::io("write tasks.md", e))?;

        let mut result = task.clone();
        result.status = TaskStatus::InProgress;
        return Ok(result);
    }

    // Enhanced format: validate status and check if ready
    if task.status == TaskStatus::Shelved {
        return Err(CoreError::validation(format!(
            "Task \"{task_id}\" is shelved (run \"ito tasks unshelve {change_id} {task_id}\" first)"
        )));
    }

    if task.status != TaskStatus::Pending {
        return Err(CoreError::validation(format!(
            "Task \"{task_id}\" is not pending (current: {})",
            task.status.as_enhanced_label()
        )));
    }

    let (ready, blocked) = compute_ready_and_blocked(&parsed);
    if !ready.iter().any(|t| t.id == task_id) {
        if let Some((_, blockers)) = blocked.iter().find(|(t, _)| t.id == task_id) {
            let mut msg = String::from("Task is blocked:");
            for b in blockers {
                msg.push_str("\n- ");
                msg.push_str(b);
            }
            return Err(CoreError::validation(msg));
        }
        return Err(CoreError::validation("Task is blocked"));
    }

    let updated = update_enhanced_task_status(
        &contents,
        task_id,
        TaskStatus::InProgress,
        chrono::Local::now(),
    );
    ito_common::io::write_std(&path, updated.as_bytes())
        .map_err(|e| CoreError::io("write tasks.md", e))?;

    let mut result = task.clone();
    result.status = TaskStatus::InProgress;
    Ok(result)
}

/// Complete a task (transition to complete).
///
/// Validates preconditions and updates the tasks.md file.
pub fn complete_task(
    ito_path: &Path,
    change_id: &str,
    task_id: &str,
    _note: Option<String>,
) -> CoreResult<TaskItem> {
    let path = tasks_path(ito_path, change_id);
    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);

    // Check for errors
    if parsed
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    // Find the task
    let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    };

    let updated = if parsed.format == TasksFormat::Checkbox {
        update_checkbox_task_status(&contents, task_id, TaskStatus::Complete)
            .map_err(CoreError::validation)?
    } else {
        update_enhanced_task_status(
            &contents,
            task_id,
            TaskStatus::Complete,
            chrono::Local::now(),
        )
    };

    ito_common::io::write_std(&path, updated.as_bytes())
        .map_err(|e| CoreError::io("write tasks.md", e))?;

    let mut result = task.clone();
    result.status = TaskStatus::Complete;
    Ok(result)
}

/// Shelve a task (transition to shelved).
///
/// Only supported for enhanced format. Validates preconditions and updates the tasks.md file.
pub fn shelve_task(
    ito_path: &Path,
    change_id: &str,
    task_id: &str,
    _reason: Option<String>,
) -> CoreResult<TaskItem> {
    let path = tasks_path(ito_path, change_id);
    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);

    if parsed.format == TasksFormat::Checkbox {
        return Err(CoreError::validation(
            "Checkbox-only tasks.md does not support shelving",
        ));
    }

    // Check for errors
    if parsed
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    // Find the task
    let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    };

    if task.status == TaskStatus::Complete {
        return Err(CoreError::validation(format!(
            "Task \"{task_id}\" is already complete"
        )));
    }

    let updated = update_enhanced_task_status(
        &contents,
        task_id,
        TaskStatus::Shelved,
        chrono::Local::now(),
    );

    ito_common::io::write_std(&path, updated.as_bytes())
        .map_err(|e| CoreError::io("write tasks.md", e))?;

    let mut result = task.clone();
    result.status = TaskStatus::Shelved;
    Ok(result)
}

/// Unshelve a task (transition back to pending).
///
/// Only supported for enhanced format. Validates preconditions and updates the tasks.md file.
pub fn unshelve_task(ito_path: &Path, change_id: &str, task_id: &str) -> CoreResult<TaskItem> {
    let path = tasks_path(ito_path, change_id);
    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);

    if parsed.format == TasksFormat::Checkbox {
        return Err(CoreError::validation(
            "Checkbox-only tasks.md does not support shelving",
        ));
    }

    // Check for errors
    if parsed
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    // Find the task
    let Some(task) = parsed.tasks.iter().find(|t| t.id == task_id) else {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    };

    if task.status != TaskStatus::Shelved {
        return Err(CoreError::validation(format!(
            "Task \"{task_id}\" is not shelved"
        )));
    }

    let updated = update_enhanced_task_status(
        &contents,
        task_id,
        TaskStatus::Pending,
        chrono::Local::now(),
    );

    ito_common::io::write_std(&path, updated.as_bytes())
        .map_err(|e| CoreError::io("write tasks.md", e))?;

    let mut result = task.clone();
    result.status = TaskStatus::Pending;
    Ok(result)
}

/// Add a new task to a change's tasks.md.
///
/// Only supported for enhanced format. Computes the next task ID and inserts the task.
pub fn add_task(
    ito_path: &Path,
    change_id: &str,
    title: &str,
    wave: Option<u32>,
) -> CoreResult<TaskItem> {
    let wave = wave.unwrap_or(1);
    let path = tasks_path(ito_path, change_id);
    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);

    if parsed.format != TasksFormat::Enhanced {
        return Err(CoreError::validation(
            "Cannot add tasks to checkbox-only tracking file. Convert to enhanced format first.",
        ));
    }

    // Check for errors
    if parsed
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    // Compute next task ID for this wave
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
        "\n### Task {new_id}: {title}\n- **Files**: `path/to/file.rs`\n- **Dependencies**: None\n- **Action**:\n  [Describe what needs to be done]\n- **Verify**: `cargo test --workspace`\n- **Done When**: [Success criteria]\n- **Updated At**: {date}\n- **Status**: [ ] pending\n"
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

    ito_common::io::write_std(&path, out.as_bytes())
        .map_err(|e| CoreError::io("write tasks.md", e))?;

    Ok(TaskItem {
        id: new_id,
        name: title.to_string(),
        wave: Some(wave),
        status: TaskStatus::Pending,
        updated_at: Some(date),
        dependencies: Vec::new(),
        files: vec!["path/to/file.rs".to_string()],
        action: "[Describe what needs to be done]".to_string(),
        verify: Some("cargo test --workspace".to_string()),
        done_when: Some("[Success criteria]".to_string()),
        kind: TaskKind::Normal,
        header_line_index: 0,
    })
}

/// Show a specific task by ID.
///
/// Returns the full task details.
pub fn show_task(ito_path: &Path, change_id: &str, task_id: &str) -> CoreResult<TaskItem> {
    let path = tasks_path(ito_path, change_id);
    let contents = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;

    let parsed = parse_tasks_tracking_file(&contents);

    // Check for errors
    if parsed
        .diagnostics
        .iter()
        .any(|d| d.level == DiagnosticLevel::Error)
    {
        return Err(CoreError::validation("tasks.md contains errors"));
    }

    parsed
        .tasks
        .iter()
        .find(|t| t.id == task_id)
        .cloned()
        .ok_or_else(|| CoreError::not_found(format!("Task \"{task_id}\" not found")))
}

/// Read the raw markdown contents of a change's tasks.md file.
pub fn read_tasks_markdown(ito_path: &Path, change_id: &str) -> CoreResult<String> {
    let path = tasks_path(ito_path, change_id);
    ito_common::io::read_to_string(&path).map_err(|e| {
        CoreError::io(
            format!("reading tasks.md for \"{change_id}\""),
            std::io::Error::other(e),
        )
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::change_repository::FsChangeRepository;

    use super::list_ready_tasks_across_changes;

    fn write(path: impl AsRef<Path>, contents: &str) {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("parent dirs should exist");
        }
        std::fs::write(path, contents).expect("test fixture should write");
    }

    fn make_ready_change(root: &Path, id: &str) {
        write(
            root.join(".ito/changes").join(id).join("proposal.md"),
            "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
        );
        write(
            root.join(".ito/changes")
                .join(id)
                .join("specs")
                .join("alpha")
                .join("spec.md"),
            "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
        );
        write(
            root.join(".ito/changes").join(id).join("tasks.md"),
            "## 1. Implementation\n- [ ] 1.1 pending\n",
        );
    }

    fn make_complete_change(root: &Path, id: &str) {
        write(
            root.join(".ito/changes").join(id).join("proposal.md"),
            "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
        );
        write(
            root.join(".ito/changes")
                .join(id)
                .join("specs")
                .join("alpha")
                .join("spec.md"),
            "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
        );
        write(
            root.join(".ito/changes").join(id).join("tasks.md"),
            "## 1. Implementation\n- [x] 1.1 done\n",
        );
    }

    #[test]
    fn returns_ready_tasks_for_ready_changes() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");
        make_ready_change(repo.path(), "000-01_alpha");
        make_complete_change(repo.path(), "000-02_beta");

        let change_repo = FsChangeRepository::new(&ito_path);
        let ready =
            list_ready_tasks_across_changes(&change_repo, &ito_path).expect("ready task listing");

        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].change_id, "000-01_alpha");
        assert_eq!(ready[0].ready_tasks.len(), 1);
        assert_eq!(ready[0].ready_tasks[0].id, "1");
    }

    #[test]
    fn returns_empty_when_no_ready_tasks_exist() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");
        make_complete_change(repo.path(), "000-01_alpha");

        let change_repo = FsChangeRepository::new(&ito_path);
        let ready =
            list_ready_tasks_across_changes(&change_repo, &ito_path).expect("ready task listing");

        assert!(ready.is_empty());
    }

    #[test]
    fn read_tasks_markdown_returns_contents_for_existing_file() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");
        let change_id = "000-01_alpha";
        let tasks_content = "## 1. Implementation\n- [ ] 1.1 pending\n";
        write(
            ito_path.join("changes").join(change_id).join("tasks.md"),
            tasks_content,
        );

        let result =
            super::read_tasks_markdown(&ito_path, change_id).expect("should read tasks.md");
        assert_eq!(result, tasks_content);
    }

    #[test]
    fn read_tasks_markdown_returns_error_for_missing_file() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");

        let result = super::read_tasks_markdown(&ito_path, "nonexistent-change");
        assert!(result.is_err(), "should fail for missing tasks.md");
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("tasks.md"),
            "error should mention tasks.md, got: {msg}"
        );
    }
}
