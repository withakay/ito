//! Backend-backed task repository adapter.
//!
//! Delegates task reads to a [`BackendTaskReader`] when backend mode is
//! enabled. Falls back to empty results when the backend has no tasks
//! artifact for a change.

use ito_domain::backend::BackendTaskReader;
use ito_domain::errors::DomainResult;
use ito_domain::tasks::{TaskRepository as DomainTaskRepository, TasksParseResult};

/// Backend-backed task repository.
///
/// Wraps a [`BackendTaskReader`] and parses the tasks markdown content
/// returned by the backend using the standard task parser.
pub struct BackendTaskRepository<R: BackendTaskReader> {
    reader: R,
}

impl<R: BackendTaskReader> BackendTaskRepository<R> {
    /// Create a backend-backed task repository.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BackendTaskReader> DomainTaskRepository for BackendTaskRepository<R> {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        let content = self.reader.load_tasks_content(change_id)?;
        let Some(content) = content else {
            return Ok(TasksParseResult::empty());
        };

        Ok(ito_domain::tasks::parse_tasks_tracking_file(&content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_domain::errors::DomainResult;

    /// In-memory backend task reader for tests.
    struct FakeTaskReader {
        content: Option<String>,
    }

    impl FakeTaskReader {
        fn with_content(content: &str) -> Self {
            Self {
                content: Some(content.to_string()),
            }
        }

        fn empty() -> Self {
            Self { content: None }
        }
    }

    impl BackendTaskReader for FakeTaskReader {
        fn load_tasks_content(&self, _change_id: &str) -> DomainResult<Option<String>> {
            Ok(self.content.clone())
        }
    }

    #[test]
    fn missing_tasks_returns_empty() {
        let reader = FakeTaskReader::empty();
        let repo = BackendTaskRepository::new(reader);

        let result = repo.load_tasks("test-change").unwrap();
        assert_eq!(result.progress.total, 0);
        assert_eq!(result.progress.complete, 0);
    }

    #[test]
    fn checkbox_tasks_parsed_correctly() {
        let reader = FakeTaskReader::with_content("# Tasks\n- [x] Done\n- [ ] Pending\n");
        let repo = BackendTaskRepository::new(reader);

        let result = repo.load_tasks("test-change").unwrap();
        assert_eq!(result.progress.total, 2);
        assert_eq!(result.progress.complete, 1);
    }

    #[test]
    fn get_task_counts_from_backend() {
        let reader = FakeTaskReader::with_content("# Tasks\n- [x] A\n- [x] B\n- [ ] C\n");
        let repo = BackendTaskRepository::new(reader);

        let (completed, total) = repo.get_task_counts("test-change").unwrap();
        assert_eq!(completed, 2);
        assert_eq!(total, 3);
    }

    #[test]
    fn has_tasks_detects_content() {
        let reader = FakeTaskReader::with_content("# Tasks\n- [ ] A\n");
        let repo = BackendTaskRepository::new(reader);

        assert!(repo.has_tasks("test-change").unwrap());
    }

    #[test]
    fn has_tasks_empty_content() {
        let reader = FakeTaskReader::empty();
        let repo = BackendTaskRepository::new(reader);

        assert!(!repo.has_tasks("test-change").unwrap());
    }
}
