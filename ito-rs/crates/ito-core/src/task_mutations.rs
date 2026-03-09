//! Task mutation services for filesystem-backed persistence.

use std::path::PathBuf;

use crate::errors::CoreError;
use ito_domain::errors::DomainError;
use ito_domain::tasks::{
    TaskInitResult, TaskMutationError, TaskMutationResult, TaskMutationService,
    TaskMutationServiceResult,
};

/// Filesystem-backed task mutation service.
#[derive(Debug, Clone)]
pub struct FsTaskMutationService {
    ito_path: PathBuf,
}

impl FsTaskMutationService {
    /// Create a filesystem-backed task mutation service for a `.ito/` path.
    pub fn new(ito_path: impl Into<PathBuf>) -> Self {
        Self {
            ito_path: ito_path.into(),
        }
    }

    /// Return a `NotFound` error when the tasks file is absent, with a helpful init hint.
    fn missing_tasks_error(change_id: &str) -> TaskMutationError {
        TaskMutationError::not_found(format!(
            "No backend tasks found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
        ))
    }

    /// Resolve the tracking file path and verify it exists.
    ///
    /// Returns `Err(NotFound)` when the file is absent so callers get a consistent
    /// 404-class error rather than an opaque IO failure.
    fn require_tasks_path(&self, change_id: &str) -> TaskMutationServiceResult<std::path::PathBuf> {
        let path = crate::tasks::tracking_file_path(&self.ito_path, change_id)
            .map_err(task_mutation_error_from_core)?;
        if !path.exists() {
            return Err(Self::missing_tasks_error(change_id));
        }
        Ok(path)
    }
}

impl TaskMutationService for FsTaskMutationService {
    fn load_tasks_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        let path = crate::tasks::tracking_file_path(&self.ito_path, change_id)
            .map_err(task_mutation_error_from_core)?;
        if !path.exists() {
            return Ok(None);
        }
        let contents = ito_common::io::read_to_string_std(&path)
            .map_err(|e| TaskMutationError::io("reading tasks markdown", e))?;
        Ok(Some(contents))
    }

    fn init_tasks(&self, change_id: &str) -> TaskMutationServiceResult<TaskInitResult> {
        let (path, existed) = crate::tasks::init_tasks(&self.ito_path, change_id)
            .map_err(task_mutation_error_from_core)?;
        Ok(TaskInitResult {
            change_id: change_id.to_string(),
            path: Some(path),
            existed,
            revision: None,
        })
    }

    fn start_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        // Verify tasks file exists before delegating to core (gives a 404 instead of IO error).
        let _ = self.require_tasks_path(change_id)?;
        let task = crate::tasks::start_task(&self.ito_path, change_id, task_id)
            .map_err(task_mutation_error_from_core)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }

    fn complete_task(
        &self,
        change_id: &str,
        task_id: &str,
        note: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let _ = self.require_tasks_path(change_id)?;
        let task = crate::tasks::complete_task(&self.ito_path, change_id, task_id, note)
            .map_err(task_mutation_error_from_core)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }

    fn shelve_task(
        &self,
        change_id: &str,
        task_id: &str,
        reason: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let _ = self.require_tasks_path(change_id)?;
        let task = crate::tasks::shelve_task(&self.ito_path, change_id, task_id, reason)
            .map_err(task_mutation_error_from_core)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }

    fn unshelve_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let _ = self.require_tasks_path(change_id)?;
        let task = crate::tasks::unshelve_task(&self.ito_path, change_id, task_id)
            .map_err(task_mutation_error_from_core)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }

    fn add_task(
        &self,
        change_id: &str,
        title: &str,
        wave: Option<u32>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        let _ = self.require_tasks_path(change_id)?;
        let task = crate::tasks::add_task(&self.ito_path, change_id, title, wave)
            .map_err(task_mutation_error_from_core)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }
}

pub(crate) fn boxed_fs_task_mutation_service(
    ito_path: PathBuf,
) -> Box<dyn TaskMutationService + Send> {
    Box::new(FsTaskMutationService::new(ito_path))
}

pub(crate) fn task_mutation_error_from_core(err: CoreError) -> TaskMutationError {
    match err {
        CoreError::Domain(domain) => match domain {
            DomainError::Io { context, source } => TaskMutationError::io(context, source),
            DomainError::NotFound { entity, id } => {
                TaskMutationError::not_found(format!("{entity} not found: {id}"))
            }
            DomainError::AmbiguousTarget {
                entity,
                input,
                matches,
            } => TaskMutationError::validation(format!(
                "Ambiguous {entity} target '{input}'. Matches: {matches}"
            )),
        },
        CoreError::Io { context, source } => TaskMutationError::io(context, source),
        CoreError::Validation(message) => TaskMutationError::validation(message),
        CoreError::Parse(message) => TaskMutationError::validation(message),
        CoreError::Process(message) => TaskMutationError::other(message),
        CoreError::Sqlite(message) => TaskMutationError::other(format!("sqlite error: {message}")),
        CoreError::NotFound(message) => TaskMutationError::not_found(message),
        CoreError::Serde { context, message } => {
            TaskMutationError::other(format!("{context}: {message}"))
        }
    }
}
