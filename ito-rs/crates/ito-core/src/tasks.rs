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

/// Computes and validated filesystem path to a change's tasks.md file.
///
/// # Arguments
///
/// * `ito_path` - Root repository path containing change directories.
/// * `change_id` - Change identifier used as a path segment; must not contain invalid traversal or path characters.
///
/// # Returns
///
/// `PathBuf` pointing to the change's tasks.md on success. Returns `CoreError::validation` when `change_id` is an invalid path segment.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let p = checked_tasks_path(Path::new("repo"), "1.1").unwrap();
/// assert!(p.ends_with("tasks.md"));
/// ```
fn checked_tasks_path(ito_path: &Path, change_id: &str) -> CoreResult<PathBuf> {
    let Some(path) = ito_domain::tasks::tasks_path_checked(ito_path, change_id) else {
        return Err(CoreError::validation(format!(
            "invalid change id path segment: \"{change_id}\""
        )));
    };
    Ok(path)
}

/// Resolves a user-supplied task identifier to the canonical task id used in the parsed tasks.
///
/// For non-checkbox task formats the input `task_id` is returned unchanged. For checkbox-format
/// task lists this will:
/// - return `task_id` unchanged if it already matches a task's canonical id, or
/// - treat a numeric `task_id` as a 1-based index and return the canonical id of that indexed task.
///
/// # Returns
///
/// `Ok(&str)` containing the canonical task id when resolution succeeds, `Err(CoreError::not_found)`
/// when the provided `task_id` does not match any task and cannot be resolved as a valid index.
///
/// # Examples
///
/// ```
/// // Checkbox format: "1" resolves to the first task's canonical id "1.1"
/// let parsed = /* TasksParseResult with format Checkbox and tasks[0].id == "1.1" */;
/// let resolved = resolve_task_id(&parsed, "1").unwrap();
/// assert_eq!(resolved, "1.1");
///
/// // Enhanced format: id is returned unchanged
/// let parsed_enh = /* TasksParseResult with format Enhanced */;
/// let id = resolve_task_id(&parsed_enh, "task-abc").unwrap();
/// assert_eq!(id, "task-abc");
/// ```
fn resolve_task_id<'a>(parsed: &'a TasksParseResult, task_id: &'a str) -> CoreResult<&'a str> {
    if parsed.format != TasksFormat::Checkbox {
        return Ok(task_id);
    }

    if parsed.tasks.iter().any(|t| t.id == task_id) {
        return Ok(task_id);
    }

    let Ok(idx) = task_id.parse::<usize>() else {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    };
    if idx == 0 || idx > parsed.tasks.len() {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    }

    Ok(parsed.tasks[idx - 1].id.as_str())
}

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

        let Ok(path) = checked_tasks_path(ito_path, &summary.id) else {
            continue;
        };
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
    let path = checked_tasks_path(ito_path, change_id)?;

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
    let path = checked_tasks_path(ito_path, change_id)?;

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

/// Mark a task as in-progress in a change's tasks.md.
///
/// Validates parsing diagnostics and task preconditions, updates the tasks.md file on disk,
/// and returns the updated TaskItem with its status set to `InProgress`.
///
/// Parameters:
/// - `ito_path`: root repository path used to resolve the change's tasks.md.
/// - `change_id`: canonical change identifier whose tasks.md will be modified.
/// - `task_id`: task identifier to start; for checkbox-format files this may be a numeric index
///   that will be resolved to the canonical task id.
///
/// Errors:
/// Returns a `CoreError` when the tasks.md cannot be read/written, when parsing diagnostics
/// contain errors, when the task cannot be resolved or located, or when preconditions for
/// transitioning the task to `InProgress` are not met (including blocked, already in-progress,
/// completed, or shelved states).
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // Start task "1.1" for change "1" in the repository at "/repo"
/// let repo = Path::new("/repo");
/// let _ = ito_core::tasks::start_task(repo, "1", "1.1");
/// ```
pub fn start_task(ito_path: &Path, change_id: &str, task_id: &str) -> CoreResult<TaskItem> {
    let path = checked_tasks_path(ito_path, change_id)?;
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

    let resolved_task_id = resolve_task_id(&parsed, task_id)?;

    // Find the task
    let Some(task) = parsed.tasks.iter().find(|t| t.id == resolved_task_id) else {
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
        && current.id != resolved_task_id
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
                    "Task \"{resolved_task_id}\" is already in-progress"
                )));
            }
            TaskStatus::Complete => {
                return Err(CoreError::validation(format!(
                    "Task \"{resolved_task_id}\" is already complete"
                )));
            }
            TaskStatus::Shelved => {
                return Err(CoreError::validation(
                    "Checkbox-only tasks.md does not support shelving".to_string(),
                ));
            }
        }

        let updated =
            update_checkbox_task_status(&contents, resolved_task_id, TaskStatus::InProgress)
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

/// Mark a task in a change's tasks.md as complete.
///
/// Reads and validates the change's tasks.md, resolves the provided task identifier
/// (supports enhanced ids and numeric indexes for checkbox format), updates the file
/// setting the task's status to `Complete`, and returns the updated task item.
///
/// # Returns
///
/// `TaskItem` representing the task with its status set to `Complete`.
///
/// # Errors
///
/// Returns a `CoreError::validation` if the tasks.md contains parse errors or the update
/// operation is rejected; `CoreError::not_found` if the specified task cannot be located;
/// and `CoreError::io` for filesystem read/write failures.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use ito_core::tasks::complete_task;
/// # use ito_core::error::CoreResult;
/// // Attempt to mark task "1.1" complete for change "1" in the repository at "."
/// let res: CoreResult<_> = complete_task(Path::new("."), "1", "1.1", None);
/// // `res` will be `Ok(task)` on success or an error describing the failure.
/// ```
pub fn complete_task(
    ito_path: &Path,
    change_id: &str,
    task_id: &str,
    _note: Option<String>,
) -> CoreResult<TaskItem> {
    let path = checked_tasks_path(ito_path, change_id)?;
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

    let resolved_task_id = resolve_task_id(&parsed, task_id)?;

    // Find the task
    let Some(task) = parsed.tasks.iter().find(|t| t.id == resolved_task_id) else {
        return Err(CoreError::not_found(format!(
            "Task \"{task_id}\" not found in tasks.md"
        )));
    };

    let updated = if parsed.format == TasksFormat::Checkbox {
        update_checkbox_task_status(&contents, resolved_task_id, TaskStatus::Complete)
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
    let path = checked_tasks_path(ito_path, change_id)?;
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
    let path = checked_tasks_path(ito_path, change_id)?;
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
    let path = checked_tasks_path(ito_path, change_id)?;
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
    let path = checked_tasks_path(ito_path, change_id)?;
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
    let path = checked_tasks_path(ito_path, change_id)?;
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
        assert_eq!(ready[0].ready_tasks[0].id, "1.1");
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

    #[test]
    fn read_tasks_markdown_rejects_traversal_like_change_id() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");

        let result = super::read_tasks_markdown(&ito_path, "../escape");
        assert!(result.is_err(), "traversal-like ids should fail");
    }
}
