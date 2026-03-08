//! Task mutation services for filesystem and backend persistence.

use std::path::PathBuf;
use std::sync::Arc;

use crate::backend_sync::map_backend_error;
use crate::errors::{CoreError, CoreResult};
use crate::tasks::{
    apply_add_task, apply_complete_task, apply_shelve_task, apply_start_task, apply_unshelve_task,
    enhanced_tasks_template, tracking_file_path, TaskItem,
};
use ito_domain::backend::{ArtifactBundle, BackendError, BackendSyncClient, PushResult};

const BACKEND_TASKS_LABEL: &str = "backend tasks";

/// Outcome of a task mutation.
#[derive(Debug, Clone)]
pub struct TaskMutationResult {
    /// Change identifier the mutation applied to.
    pub change_id: String,
    /// Updated task item.
    pub task: TaskItem,
    /// Backend revision after the mutation, when applicable.
    pub revision: Option<String>,
}

/// Outcome of initializing a tracking file or artifact.
#[derive(Debug, Clone)]
pub struct TaskInitResult {
    /// Change identifier the init applied to.
    pub change_id: String,
    /// Tracking path when filesystem-backed.
    pub path: Option<PathBuf>,
    /// Whether the tracking file already existed.
    pub existed: bool,
    /// Backend revision after the mutation, when applicable.
    pub revision: Option<String>,
}

/// Service interface for task mutations.
pub trait TaskMutationService: Send + Sync {
    /// Load raw task tracking markdown, if available.
    fn load_tasks_markdown(&self, change_id: &str) -> CoreResult<Option<String>>;
    /// Initialize a tracking file/artifact for a change.
    fn init_tasks(&self, change_id: &str) -> CoreResult<TaskInitResult>;
    /// Mark a task as in-progress.
    fn start_task(&self, change_id: &str, task_id: &str) -> CoreResult<TaskMutationResult>;
    /// Mark a task as complete.
    fn complete_task(
        &self,
        change_id: &str,
        task_id: &str,
        note: Option<String>,
    ) -> CoreResult<TaskMutationResult>;
    /// Shelve a task.
    fn shelve_task(
        &self,
        change_id: &str,
        task_id: &str,
        reason: Option<String>,
    ) -> CoreResult<TaskMutationResult>;
    /// Unshelve a task.
    fn unshelve_task(&self, change_id: &str, task_id: &str) -> CoreResult<TaskMutationResult>;
    /// Add a new task.
    fn add_task(
        &self,
        change_id: &str,
        title: &str,
        wave: Option<u32>,
    ) -> CoreResult<TaskMutationResult>;
}

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
}

impl TaskMutationService for FsTaskMutationService {
    fn load_tasks_markdown(&self, change_id: &str) -> CoreResult<Option<String>> {
        let path = tracking_file_path(&self.ito_path, change_id)?;
        if !path.exists() {
            return Ok(None);
        }
        let contents = ito_common::io::read_to_string_std(&path)
            .map_err(|e| CoreError::io(format!("read {}", path.display()), e))?;
        Ok(Some(contents))
    }

    fn init_tasks(&self, change_id: &str) -> CoreResult<TaskInitResult> {
        let (path, existed) = crate::tasks::init_tasks(&self.ito_path, change_id)?;
        Ok(TaskInitResult {
            change_id: change_id.to_string(),
            path: Some(path),
            existed,
            revision: None,
        })
    }

    fn start_task(&self, change_id: &str, task_id: &str) -> CoreResult<TaskMutationResult> {
        let task = crate::tasks::start_task(&self.ito_path, change_id, task_id)?;
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
    ) -> CoreResult<TaskMutationResult> {
        let task = crate::tasks::complete_task(&self.ito_path, change_id, task_id, note)?;
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
    ) -> CoreResult<TaskMutationResult> {
        let task = crate::tasks::shelve_task(&self.ito_path, change_id, task_id, reason)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }

    fn unshelve_task(&self, change_id: &str, task_id: &str) -> CoreResult<TaskMutationResult> {
        let task = crate::tasks::unshelve_task(&self.ito_path, change_id, task_id)?;
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
    ) -> CoreResult<TaskMutationResult> {
        let task = crate::tasks::add_task(&self.ito_path, change_id, title, wave)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task,
            revision: None,
        })
    }
}

/// Backend-backed task mutation service using artifact sync.
#[derive(Debug, Clone)]
pub struct RemoteTaskMutationService {
    sync_client: Arc<dyn BackendSyncClient + Send + Sync>,
}

impl RemoteTaskMutationService {
    /// Create a backend-backed task mutation service.
    pub fn new(sync_client: Arc<dyn BackendSyncClient + Send + Sync>) -> Self {
        Self { sync_client }
    }

    fn pull_bundle(&self, change_id: &str) -> CoreResult<ArtifactBundle> {
        self.sync_client
            .pull(change_id)
            .map_err(|err| map_backend_error(err, "pull"))
    }

    fn push_bundle(&self, change_id: &str, bundle: &ArtifactBundle) -> CoreResult<PushResult> {
        self.sync_client
            .push(change_id, bundle)
            .map_err(|err| map_backend_error(err, "push"))
    }

    fn mutate_tasks<F>(&self, change_id: &str, op: F) -> CoreResult<TaskMutationResult>
    where
        F: FnOnce(&str) -> CoreResult<crate::tasks::TaskMutationOutcome>,
    {
        let mut bundle = self.pull_bundle(change_id)?;
        let tasks = bundle.tasks.as_deref().ok_or_else(|| {
            CoreError::not_found(format!(
                "No {BACKEND_TASKS_LABEL} found for \"{change_id}\". Run \"ito tasks init {change_id}\" first."
            ))
        })?;

        let outcome = op(tasks)?;
        bundle.tasks = Some(outcome.updated_content);

        let result = self.push_bundle(change_id, &bundle)?;
        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task: outcome.task,
            revision: Some(result.new_revision),
        })
    }
}

impl TaskMutationService for RemoteTaskMutationService {
    fn load_tasks_markdown(&self, change_id: &str) -> CoreResult<Option<String>> {
        let bundle = self.pull_bundle(change_id)?;
        Ok(bundle.tasks)
    }

    fn init_tasks(&self, change_id: &str) -> CoreResult<TaskInitResult> {
        let mut bundle = self.pull_bundle(change_id)?;
        if bundle.tasks.is_some() {
            return Ok(TaskInitResult {
                change_id: change_id.to_string(),
                path: None,
                existed: true,
                revision: None,
            });
        }

        let now = chrono::Local::now();
        let contents = enhanced_tasks_template(change_id, now);
        bundle.tasks = Some(contents);
        let result = self.push_bundle(change_id, &bundle)?;

        Ok(TaskInitResult {
            change_id: change_id.to_string(),
            path: None,
            existed: false,
            revision: Some(result.new_revision),
        })
    }

    fn start_task(&self, change_id: &str, task_id: &str) -> CoreResult<TaskMutationResult> {
        self.mutate_tasks(change_id, |tasks| {
            apply_start_task(tasks, change_id, task_id, BACKEND_TASKS_LABEL)
        })
    }

    fn complete_task(
        &self,
        change_id: &str,
        task_id: &str,
        _note: Option<String>,
    ) -> CoreResult<TaskMutationResult> {
        self.mutate_tasks(change_id, |tasks| {
            apply_complete_task(tasks, task_id, BACKEND_TASKS_LABEL)
        })
    }

    fn shelve_task(
        &self,
        change_id: &str,
        task_id: &str,
        _reason: Option<String>,
    ) -> CoreResult<TaskMutationResult> {
        self.mutate_tasks(change_id, |tasks| {
            apply_shelve_task(tasks, task_id, BACKEND_TASKS_LABEL)
        })
    }

    fn unshelve_task(&self, change_id: &str, task_id: &str) -> CoreResult<TaskMutationResult> {
        self.mutate_tasks(change_id, |tasks| {
            apply_unshelve_task(tasks, task_id, BACKEND_TASKS_LABEL)
        })
    }

    fn add_task(
        &self,
        change_id: &str,
        title: &str,
        wave: Option<u32>,
    ) -> CoreResult<TaskMutationResult> {
        self.mutate_tasks(change_id, |tasks| {
            apply_add_task(tasks, title, wave, BACKEND_TASKS_LABEL)
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StubBackendSyncClient;

impl BackendSyncClient for StubBackendSyncClient {
    fn pull(&self, change_id: &str) -> Result<ArtifactBundle, BackendError> {
        Err(BackendError::Other(format!(
            "Sync endpoints not yet available on backend for change '{change_id}'"
        )))
    }

    fn push(&self, change_id: &str, _bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
        Err(BackendError::Other(format!(
            "Sync endpoints not yet available on backend for change '{change_id}'"
        )))
    }
}

pub(crate) fn stub_backend_sync_client() -> Arc<dyn BackendSyncClient + Send + Sync> {
    Arc::new(StubBackendSyncClient)
}
