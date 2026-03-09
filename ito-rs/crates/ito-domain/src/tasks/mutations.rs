//! Task mutation port definitions.

use std::io;
use std::path::PathBuf;

use thiserror::Error;

use super::parse::TaskItem;

/// Result alias for task mutation operations.
pub type TaskMutationServiceResult<T> = Result<T, TaskMutationError>;

/// Outcome of a task mutation.
#[derive(Debug, Clone)]
pub struct TaskMutationResult {
    /// Change identifier the mutation applied to.
    pub change_id: String,
    /// Updated task item.
    pub task: TaskItem,
    /// Backend or store revision after the mutation, when applicable.
    pub revision: Option<String>,
}

/// Outcome of initializing task tracking.
#[derive(Debug, Clone)]
pub struct TaskInitResult {
    /// Change identifier the init applied to.
    pub change_id: String,
    /// Tracking path when filesystem-backed.
    pub path: Option<PathBuf>,
    /// Whether the tracking artifact already existed.
    pub existed: bool,
    /// Backend or store revision after the mutation, when applicable.
    pub revision: Option<String>,
}

/// Error type for task mutation ports.
#[derive(Debug, Error)]
pub enum TaskMutationError {
    /// Filesystem or transport failure.
    #[error("I/O failure while {context}: {source}")]
    Io {
        /// Short operation context.
        context: String,
        /// Source error.
        #[source]
        source: io::Error,
    },

    /// Validation or precondition failure.
    #[error("{0}")]
    Validation(String),

    /// Requested task artifact or item was not found.
    #[error("{0}")]
    NotFound(String),

    /// Unexpected transport or backend failure.
    #[error("{0}")]
    Other(String),
}

impl TaskMutationError {
    /// Build an I/O flavored error.
    pub fn io(context: impl Into<String>, source: io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }

    /// Build a validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Build a not-found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    /// Build a catch-all error.
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

/// Port for task mutations and raw markdown access.
pub trait TaskMutationService: Send + Sync {
    /// Load raw task tracking markdown, if available.
    fn load_tasks_markdown(&self, change_id: &str) -> TaskMutationServiceResult<Option<String>>;
    /// Initialize a tracking file or remote artifact for a change.
    fn init_tasks(&self, change_id: &str) -> TaskMutationServiceResult<TaskInitResult>;
    /// Mark a task as in-progress.
    fn start_task(&self, change_id: &str, task_id: &str)
    -> TaskMutationServiceResult<TaskMutationResult>;
    /// Mark a task as complete.
    fn complete_task(
        &self,
        change_id: &str,
        task_id: &str,
        note: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult>;
    /// Shelve a task.
    fn shelve_task(
        &self,
        change_id: &str,
        task_id: &str,
        reason: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult>;
    /// Unshelve a task.
    fn unshelve_task(
        &self,
        change_id: &str,
        task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult>;
    /// Add a new task.
    fn add_task(
        &self,
        change_id: &str,
        title: &str,
        wave: Option<u32>,
    ) -> TaskMutationServiceResult<TaskMutationResult>;
}
