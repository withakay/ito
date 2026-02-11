//! Task repository port definitions.

use super::parse::{ProgressInfo, TaskItem, TasksParseResult};
use crate::errors::DomainResult;

/// Port for accessing task data.
///
/// This trait defines the interface for reading task tracking information.
/// Implementations (in the core layer) handle the file I/O and parsing details.
///
/// Consumers in the domain or adapter layers should rely on this trait to remain
/// decoupled from storage specifics.
pub trait TaskRepository {
    /// Load all tasks for a change.
    ///
    /// Returns the full parse result including diagnostics.
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult>;

    /// Get task progress for a change.
    ///
    /// This is a convenience method that returns just the progress info.
    fn get_progress(&self, change_id: &str) -> DomainResult<ProgressInfo> {
        let result = self.load_tasks(change_id)?;
        Ok(result.progress)
    }

    /// Get task counts (completed, total) for a change.
    ///
    /// Implementations should return `(0, 0)` when the tasks file doesn't exist.
    fn get_task_counts(&self, change_id: &str) -> DomainResult<(u32, u32)> {
        let progress = self.get_progress(change_id)?;
        Ok((progress.complete as u32, progress.total as u32))
    }

    /// Check if a change has any tasks defined.
    fn has_tasks(&self, change_id: &str) -> DomainResult<bool> {
        let progress = self.get_progress(change_id)?;
        Ok(progress.total > 0)
    }

    /// Get all tasks for a change.
    fn get_tasks(&self, change_id: &str) -> DomainResult<Vec<TaskItem>> {
        let result = self.load_tasks(change_id)?;
        Ok(result.tasks)
    }
}
