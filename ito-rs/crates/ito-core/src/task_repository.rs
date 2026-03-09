//! Filesystem-backed task repository implementation.

use std::path::Path;

use ito_common::fs::{FileSystem, StdFs};
use ito_config::ConfigContext;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::{
    TaskRepository as DomainTaskRepository, TasksParseResult, is_safe_tracking_filename,
    parse_tasks_tracking_file, tasks_path_checked, tracking_path_checked,
};

use crate::templates::{default_schema_name, read_change_schema, resolve_schema};

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

    /// Load tasks from an explicit change directory.
    pub(crate) fn load_tasks_from_dir(&self, change_dir: &Path) -> DomainResult<TasksParseResult> {
        let schema_name = read_schema_from_dir(&self.fs, change_dir);
        let mut ctx = ConfigContext::from_process_env();
        ctx.project_dir = self.ito_path.parent().map(|p| p.to_path_buf());

        let mut tracking_file = "tasks.md".to_string();
        if let Ok(resolved) = resolve_schema(Some(&schema_name), &ctx)
            && let Some(apply) = resolved.schema.apply.as_ref()
            && let Some(tracks) = apply.tracks.as_deref()
            && is_safe_tracking_filename(tracks)
        {
            tracking_file = tracks.to_string();
        }

        if !is_safe_tracking_filename(&tracking_file) {
            return Ok(TasksParseResult::empty());
        }

        let path = change_dir.join(&tracking_file);
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

fn read_schema_from_dir<F: FileSystem>(fs: &F, change_dir: &Path) -> String {
    let meta = change_dir.join(".ito.yaml");
    if fs.is_file(&meta)
        && let Ok(contents) = fs.read_to_string(&meta)
    {
        for line in contents.lines() {
            let l = line.trim();
            if let Some(rest) = l.strip_prefix("schema:") {
                let v = rest.trim();
                if !v.is_empty() {
                    return v.to_string();
                }
            }
        }
    }

    default_schema_name().to_string()
}

impl<F: FileSystem> DomainTaskRepository for FsTaskRepository<'_, F> {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        // `read_change_schema` uses `change_id` as a path segment; reject traversal.
        if tasks_path_checked(self.ito_path, change_id).is_none() {
            return Ok(TasksParseResult::empty());
        }

        let schema_name = read_change_schema(self.ito_path, change_id);
        let mut ctx = ConfigContext::from_process_env();
        ctx.project_dir = self.ito_path.parent().map(|p| p.to_path_buf());

        let mut tracking_file = "tasks.md".to_string();
        if let Ok(resolved) = resolve_schema(Some(&schema_name), &ctx)
            && let Some(apply) = resolved.schema.apply.as_ref()
            && let Some(tracks) = apply.tracks.as_deref()
        {
            tracking_file = tracks.to_string();
        }

        let Some(path) = tracking_path_checked(self.ito_path, change_id, &tracking_file) else {
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
    fn load_tasks_uses_schema_apply_tracks_when_set() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let ito_path = root.join(".ito");
        fs::create_dir_all(&ito_path).unwrap();

        // Override the project schema to point tracking at todo.md.
        let schema_dir = root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("spec-driven");
        fs::create_dir_all(&schema_dir).unwrap();
        fs::write(
            schema_dir.join("schema.yaml"),
            "name: spec-driven\nversion: 1\nartifacts: []\napply:\n  tracks: todo.md\n",
        )
        .unwrap();

        let change_id = "001-03_tracks";
        let change_dir = ito_path.join("changes").join(change_id);
        fs::create_dir_all(&change_dir).unwrap();
        fs::write(change_dir.join(".ito.yaml"), "schema: spec-driven\n").unwrap();
        fs::write(
            change_dir.join("todo.md"),
            "## Tasks\n- [x] one\n- [ ] two\n",
        )
        .unwrap();

        let repo = TaskRepository::new(&ito_path);
        let (completed, total) = repo.get_task_counts(change_id).unwrap();

        assert_eq!(completed, 1);
        assert_eq!(total, 2);
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
