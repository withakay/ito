//! Task Repository - Clean abstraction over task storage
//!
//! This module provides a repository pattern for loading and querying tasks,
//! hiding the markdown storage implementation from consumers.

use miette::{IntoDiagnostic, Result};
use std::path::Path;

use super::parse::{ProgressInfo, TaskItem, TasksParseResult, parse_tasks_tracking_file};
use super::tasks_path;

/// Repository for accessing task data.
///
/// This abstraction hides the markdown storage format from consumers.
/// All task queries should go through this interface rather than
/// directly parsing markdown.
pub struct TaskRepository<'a> {
    ito_path: &'a Path,
}

impl<'a> TaskRepository<'a> {
    /// Create a new task repository for the given ito directory.
    pub fn new(ito_path: &'a Path) -> Self {
        Self { ito_path }
    }

    /// Load all tasks for a change.
    ///
    /// Returns the full parse result including diagnostics.
    pub fn load_tasks(&self, change_id: &str) -> Result<TasksParseResult> {
        let path = tasks_path(self.ito_path, change_id);
        if !path.exists() {
            return Ok(TasksParseResult::empty());
        }
        let contents = std::fs::read_to_string(&path).into_diagnostic()?;
        Ok(parse_tasks_tracking_file(&contents))
    }

    /// Get task progress for a change.
    ///
    /// This is a convenience method that returns just the progress info.
    pub fn get_progress(&self, change_id: &str) -> Result<ProgressInfo> {
        let result = self.load_tasks(change_id)?;
        Ok(result.progress)
    }

    /// Get task counts (completed, total) for a change.
    ///
    /// Returns (0, 0) if the tasks file doesn't exist.
    pub fn get_task_counts(&self, change_id: &str) -> Result<(u32, u32)> {
        let progress = self.get_progress(change_id)?;
        Ok((progress.complete as u32, progress.total as u32))
    }

    /// Check if a change has any tasks defined.
    pub fn has_tasks(&self, change_id: &str) -> Result<bool> {
        let path = tasks_path(self.ito_path, change_id);
        if !path.exists() {
            return Ok(false);
        }
        let progress = self.get_progress(change_id)?;
        Ok(progress.total > 0)
    }

    /// Get all tasks for a change.
    pub fn get_tasks(&self, change_id: &str) -> Result<Vec<TaskItem>> {
        let result = self.load_tasks(change_id)?;
        Ok(result.tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_change(ito_dir: &Path, change_id: &str, tasks_content: &str) {
        let change_dir = ito_dir.join("changes").join(change_id);
        fs::create_dir_all(&change_dir).unwrap();
        fs::write(change_dir.join("tasks.md"), tasks_content).unwrap();
    }

    #[test]
    fn test_get_task_counts_checkbox_format() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        fs::create_dir_all(&ito_path).unwrap();

        setup_test_change(
            &ito_path,
            "001-01_test",
            r#"# Tasks

- [x] Task 1
- [x] Task 2
- [ ] Task 3
- [ ] Task 4
"#,
        );

        let repo = TaskRepository::new(&ito_path);
        let (completed, total) = repo.get_task_counts("001-01_test").unwrap();

        assert_eq!(completed, 2);
        assert_eq!(total, 4);
    }

    #[test]
    fn test_get_task_counts_enhanced_format() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        fs::create_dir_all(&ito_path).unwrap();

        setup_test_change(
            &ito_path,
            "001-02_enhanced",
            r#"# Tasks

## Wave 1
- **Depends On**: None

### Task 1.1: First task
- **Status**: [x] complete
- **Updated At**: 2024-01-01

### Task 1.2: Second task
- **Status**: [ ] pending
- **Updated At**: 2024-01-01

### Task 1.3: Third task
- **Status**: [x] complete
- **Updated At**: 2024-01-01
"#,
        );

        let repo = TaskRepository::new(&ito_path);
        let (completed, total) = repo.get_task_counts("001-02_enhanced").unwrap();

        assert_eq!(completed, 2);
        assert_eq!(total, 3);
    }

    #[test]
    fn test_missing_tasks_file_returns_zero() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        fs::create_dir_all(&ito_path).unwrap();

        let repo = TaskRepository::new(&ito_path);
        let (completed, total) = repo.get_task_counts("nonexistent").unwrap();

        assert_eq!(completed, 0);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_has_tasks() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        fs::create_dir_all(&ito_path).unwrap();

        setup_test_change(&ito_path, "001-01_with-tasks", "# Tasks\n- [ ] Task 1\n");
        setup_test_change(&ito_path, "001-02_no-tasks", "# Tasks\n\nNo tasks yet.\n");

        let repo = TaskRepository::new(&ito_path);

        assert!(repo.has_tasks("001-01_with-tasks").unwrap());
        assert!(!repo.has_tasks("001-02_no-tasks").unwrap());
        assert!(!repo.has_tasks("nonexistent").unwrap());
    }
}
