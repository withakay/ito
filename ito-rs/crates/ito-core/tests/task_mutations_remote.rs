use std::sync::{Arc, Mutex};

use assert_struct::assert_struct;
use ito_core::task_mutations::{RemoteTaskMutationService, TaskMutationService};
use ito_core::tasks::TaskStatus;
use ito_domain::backend::{
    ArtifactBundle, BackendError, BackendSyncClient, PushResult, RevisionConflict,
};
use ito_domain::tasks::enhanced_tasks_template;

#[derive(Debug)]
struct FakeSyncState {
    bundle: ArtifactBundle,
    push_result: PushResult,
    push_error: Option<BackendError>,
}

#[derive(Debug)]
struct FakeSyncClient {
    state: Mutex<FakeSyncState>,
}

impl FakeSyncClient {
    fn new(bundle: ArtifactBundle, push_result: PushResult) -> Self {
        Self {
            state: Mutex::new(FakeSyncState {
                bundle,
                push_result,
                push_error: None,
            }),
        }
    }

    fn with_push_error(mut self, error: BackendError) -> Self {
        self.state.get_mut().expect("lock state").push_error = Some(error);
        self
    }

    fn bundle_tasks(&self) -> Option<String> {
        self.state.lock().expect("lock state").bundle.tasks.clone()
    }
}

impl BackendSyncClient for FakeSyncClient {
    fn pull(&self, _change_id: &str) -> Result<ArtifactBundle, BackendError> {
        Ok(self.state.lock().expect("lock state").bundle.clone())
    }

    fn push(&self, _change_id: &str, bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
        let mut state = self.state.lock().expect("lock state");
        if let Some(error) = state.push_error.clone() {
            return Err(error);
        }
        state.bundle = bundle.clone();
        Ok(state.push_result.clone())
    }
}

fn make_bundle(change_id: &str, tasks: Option<String>, revision: &str) -> ArtifactBundle {
    ArtifactBundle {
        change_id: change_id.to_string(),
        proposal: None,
        design: None,
        tasks,
        specs: Vec::new(),
        revision: revision.to_string(),
    }
}

#[test]
fn remote_init_creates_tasks_bundle() {
    let change_id = "025-02_demo";
    let bundle = make_bundle(change_id, None, "rev-1");
    let client = Arc::new(FakeSyncClient::new(
        bundle,
        PushResult {
            change_id: change_id.to_string(),
            new_revision: "rev-2".to_string(),
        },
    ));

    let service = RemoteTaskMutationService::new(client.clone());
    let result = service.init_tasks(change_id).expect("init tasks");

    assert_struct!(
        result,
        ito_core::task_mutations::TaskInitResult {
            change_id: "025-02_demo",
            existed: false,
            path: None,
            revision: Some("rev-2".to_string()),
            ..
        }
    );

    let markdown = service
        .load_tasks_markdown(change_id)
        .expect("load tasks markdown")
        .expect("tasks markdown");
    assert!(markdown.contains("Tasks for: 025-02_demo"));
}

#[test]
fn remote_start_task_updates_bundle_and_revision() {
    let change_id = "025-02_demo";
    let tasks = enhanced_tasks_template(change_id, chrono::Local::now());
    let bundle = make_bundle(change_id, Some(tasks), "rev-1");
    let client = Arc::new(FakeSyncClient::new(
        bundle,
        PushResult {
            change_id: change_id.to_string(),
            new_revision: "rev-2".to_string(),
        },
    ));

    let service = RemoteTaskMutationService::new(client.clone());
    let result = service.start_task(change_id, "1.1").expect("start task");

    assert_struct!(result, ito_core::task_mutations::TaskMutationResult {
        change_id: "025-02_demo",
        revision: Some("rev-2".to_string()),
        task.status: TaskStatus::InProgress,
        ..
    });

    let updated = client.bundle_tasks().expect("bundle tasks");
    assert!(updated.contains("- **Status**: [>] in-progress"));
}

#[test]
fn remote_start_task_errors_when_missing_tasks() {
    let change_id = "025-02_demo";
    let bundle = make_bundle(change_id, None, "rev-1");
    let client = Arc::new(FakeSyncClient::new(
        bundle,
        PushResult {
            change_id: change_id.to_string(),
            new_revision: "rev-2".to_string(),
        },
    ));

    let service = RemoteTaskMutationService::new(client);
    let err = service
        .start_task(change_id, "1.1")
        .expect_err("should fail");
    let msg = err.to_string();
    assert!(msg.contains("No backend tasks found"), "{msg}");
}

#[test]
fn remote_start_task_surfaces_revision_conflict() {
    let change_id = "025-02_demo";
    let tasks = enhanced_tasks_template(change_id, chrono::Local::now());
    let bundle = make_bundle(change_id, Some(tasks), "rev-1");
    let conflict = BackendError::RevisionConflict(RevisionConflict {
        change_id: change_id.to_string(),
        local_revision: "rev-1".to_string(),
        server_revision: "rev-2".to_string(),
    });
    let client = Arc::new(
        FakeSyncClient::new(
            bundle,
            PushResult {
                change_id: change_id.to_string(),
                new_revision: "rev-2".to_string(),
            },
        )
        .with_push_error(conflict),
    );

    let service = RemoteTaskMutationService::new(client);
    let err = service
        .start_task(change_id, "1.1")
        .expect_err("should surface conflict");
    let msg = err.to_string();
    assert!(msg.contains("Revision conflict"), "{msg}");
    assert!(msg.contains("ito tasks sync pull"), "{msg}");
}
