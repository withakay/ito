use std::collections::BTreeMap;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use serde::Deserialize;

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "task-mutations-admin-token";

#[derive(Debug, Deserialize)]
struct ApiError {
    error: String,
    code: String,
}

#[derive(Debug, Deserialize)]
struct ApiTaskMarkdown {
    change_id: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiTaskMutationResult {
    change_id: String,
    revision: Option<String>,
    task: ApiTaskItem,
}

#[derive(Debug, Deserialize)]
struct ApiTaskItem {
    id: String,
    status: String,
}

async fn spawn_backend() -> (String, tempfile::TempDir) {
    let data_dir = tempfile::tempdir().expect("backend data dir");

    let mut repos = BTreeMap::new();
    repos.insert(
        ORG.to_string(),
        BackendRepoPolicy::List(vec![REPO.to_string()]),
    );
    let allowlist = BackendAllowlistConfig {
        orgs: vec![ORG.to_string()],
        repos,
    };

    let auth = BackendAuthConfig {
        admin_tokens: vec![ADMIN_TOKEN.to_string()],
        token_seed: Some("task-mutations-seed".to_string()),
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind backend port");
    let addr = listener.local_addr().expect("backend addr");
    let base_url = format!("http://{addr}");

    let config = ito_backend::BackendServerConfig {
        enabled: true,
        bind: "127.0.0.1".to_string(),
        port: addr.port(),
        data_dir: Some(data_dir.path().to_string_lossy().to_string()),
        allowed: allowlist,
        auth,
        ..Default::default()
    };

    drop(listener);

    tokio::spawn(async move {
        let _ = ito_backend::serve(config).await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    (base_url, data_dir)
}

fn project_url(base_url: &str, path: &str) -> String {
    format!("{base_url}/api/v1/projects/{ORG}/{REPO}/{path}")
}

fn seed_change(data_dir: &std::path::Path, change_id: &str, tasks: Option<&str>) {
    let change_dir = data_dir
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito")
        .join("changes")
        .join(change_id);
    std::fs::create_dir_all(&change_dir).expect("create change dir");
    std::fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write proposal");
    if let Some(tasks) = tasks {
        std::fs::write(change_dir.join("tasks.md"), tasks).expect("write tasks");
    }
}

#[tokio::test]
async fn tasks_markdown_endpoint_returns_none_for_missing_artifact() {
    let (base_url, data_dir) = spawn_backend().await;
    seed_change(data_dir.path(), "001-01_missing-tasks", None);

    let client = reqwest::Client::new();
    let response = client
        .get(project_url(
            &base_url,
            "changes/001-01_missing-tasks/tasks/raw",
        ))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .expect("send request");

    assert_eq!(response.status(), 200);
    let payload: ApiTaskMarkdown = response.json().await.expect("parse markdown payload");
    assert_eq!(payload.change_id, "001-01_missing-tasks");
    assert!(payload.content.is_none());
}

#[tokio::test]
async fn start_task_endpoint_updates_remote_tasks() {
    let (base_url, data_dir) = spawn_backend().await;
    let change_id = "001-02_remote-start";
    seed_change(
        data_dir.path(),
        change_id,
        Some(
            "# Tasks for: 001-02_remote-start\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: First task\n- **Dependencies**: None\n- **Updated At**: 2026-03-01\n- **Status**: [ ] pending\n",
        ),
    );

    let client = reqwest::Client::new();
    let response = client
        .post(project_url(
            &base_url,
            &format!("changes/{change_id}/tasks/1.1/start"),
        ))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .expect("send request");

    assert_eq!(response.status(), 200);
    let payload: ApiTaskMutationResult = response.json().await.expect("parse mutation payload");
    assert_eq!(payload.change_id, change_id);
    assert_eq!(payload.task.id, "1.1");
    assert_eq!(payload.task.status, "in-progress");
    assert!(payload.revision.is_none());

    let raw = std::fs::read_to_string(
        data_dir
            .path()
            .join("projects")
            .join(ORG)
            .join(REPO)
            .join(".ito")
            .join("changes")
            .join(change_id)
            .join("tasks.md"),
    )
    .expect("read updated tasks");
    assert!(raw.contains("- **Status**: [>] in-progress"), "{raw}");
}

#[tokio::test]
async fn start_task_endpoint_reports_missing_tasks_as_not_found() {
    let (base_url, data_dir) = spawn_backend().await;
    seed_change(data_dir.path(), "001-03_missing-start", None);

    let client = reqwest::Client::new();
    let response = client
        .post(project_url(
            &base_url,
            "changes/001-03_missing-start/tasks/1.1/start",
        ))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .expect("send request");

    assert_eq!(response.status(), 404);
    let payload: ApiError = response.json().await.expect("parse error payload");
    assert_eq!(payload.code, "not_found");
    assert!(
        payload
            .error
            .contains("Run \"ito tasks init 001-03_missing-start\" first")
    );
}
