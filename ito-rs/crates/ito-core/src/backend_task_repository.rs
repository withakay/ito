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
#[path = "backend_task_repository_tests.rs"]
mod backend_task_repository_tests;
