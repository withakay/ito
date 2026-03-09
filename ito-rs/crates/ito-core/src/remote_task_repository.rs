//! Remote task repository backed by the backend API.

use ito_domain::errors::DomainResult;
use ito_domain::tasks::{TaskRepository as DomainTaskRepository, TasksParseResult};

use crate::backend_http::BackendHttpClient;

/// Task repository implementation that reads tasks via the backend API.
pub struct RemoteTaskRepository {
    client: BackendHttpClient,
}

impl RemoteTaskRepository {
    /// Create a remote-backed task repository.
    pub(crate) fn new(client: BackendHttpClient) -> Self {
        Self { client }
    }
}

impl DomainTaskRepository for RemoteTaskRepository {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        self.client.load_tasks_parse_result(change_id)
    }
}
