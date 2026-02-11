//! Filesystem-backed task repository implementation.

use std::path::Path;

use ito_common::fs::{FileSystem, StdFs};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::{
    TaskRepository as DomainTaskRepository, TasksParseResult, parse_tasks_tracking_file,
    tasks_path_checked,
};

/// Filesystem-backed implementation of the domain `TaskRepository` port.
pub struct FsTaskRepository<'a, F: FileSystem = StdFs> {
    ito_path: &'a Path,
    fs: F,
}

impl<'a> FsTaskRepository<'a, StdFs> {
    /// Create a filesystem-backed task repository using the standard filesystem.
    pub fn new(ito_path: &'a Path) -> Self {
        Self::with_fs(ito_path, StdFs)
    }
}

impl<'a, F: FileSystem> FsTaskRepository<'a, F> {
    /// Create a filesystem-backed task repository with a custom filesystem.
    pub fn with_fs(ito_path: &'a Path, fs: F) -> Self {
        Self { ito_path, fs }
    }
}

impl<F: FileSystem> DomainTaskRepository for FsTaskRepository<'_, F> {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        let Some(path) = tasks_path_checked(self.ito_path, change_id) else {
            return Ok(TasksParseResult::empty());
        };
        if !self.fs.is_file(&path) {
            return Ok(TasksParseResult::empty());
        }
        let contents = self
            .fs
            .read_to_string(&path)
            .map_err(|source| DomainError::io("reading tasks file", source))?;
        Ok(parse_tasks_tracking_file(&contents))
    }
}

/// Backward-compatible alias for the default filesystem-backed task repository.
pub type TaskRepository<'a> = FsTaskRepository<'a, StdFs>;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;

    use super::TaskRepository;
    use ito_domain::tasks::TaskRepository as DomainTaskRepository;

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
