//! End-to-end integration tests for multi-tenant routing, auth, and data isolation.
//!
//! Tests cover:
//! - Two projects in one server instance with independent data
//! - Admin token accesses any project
//! - Derived project token is scoped to one project
//! - Derived token for project A cannot access project B
//! - Allowlist enforcement across multiple orgs

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use serde::Deserialize;
use std::collections::BTreeMap;

// ── Test-facing response types ─────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ApiChangeSummary {
    id: String,
    #[allow(dead_code)]
    work_status: String,
}

#[derive(Debug, Deserialize)]
struct ApiChange {
    id: String,
    #[allow(dead_code)]
    proposal: Option<String>,
    #[allow(dead_code)]
    progress: ApiProgress,
    #[allow(dead_code)]
    last_modified: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ApiProgress {
    total: usize,
    complete: usize,
    shelved: usize,
    in_progress: usize,
    pending: usize,
    remaining: usize,
}

#[derive(Debug, Deserialize)]
struct ApiTaskList {
    change_id: String,
    tasks: Vec<ApiTaskItem>,
    #[allow(dead_code)]
    progress: ApiProgress,
    #[allow(dead_code)]
    format: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ApiTaskItem {
    id: String,
    name: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct ApiModuleSummary {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct ApiModule {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ErrorResponse {
    error: String,
    code: String,
}

// ── Test helper ────────────────────────────────────────────────────

const ORG_A: &str = "acme";
const REPO_A: &str = "widgets";
const ORG_B: &str = "globex";
const REPO_B: &str = "gadgets";
const ADMIN_TOKEN: &str = "mt-admin-token";
const TOKEN_SEED: &str = "mt-seed";

/// Spawn a multi-tenant backend with two allowlisted projects, each
/// pre-seeded with different changes and modules on the filesystem.
///
/// Returns `(base_url, data_dir)`.
async fn spawn_multi_tenant() -> (String, tempfile::TempDir) {
    let data_dir = tempfile::tempdir().unwrap();

    // Seed project A: one change, one module
    seed_project(
        data_dir.path(),
        ORG_A,
        REPO_A,
        &["001-01_alpha-feature"],
        &[("001", "Backend")],
    );

    // Seed project B: two changes, no modules
    seed_project(
        data_dir.path(),
        ORG_B,
        REPO_B,
        &["002-01_beta-fix", "002-02_beta-refactor"],
        &[],
    );

    // Build allowlist for both orgs
    let mut repos = BTreeMap::new();
    repos.insert(ORG_A.to_string(), BackendRepoPolicy::All("*".to_string()));
    repos.insert(
        ORG_B.to_string(),
        BackendRepoPolicy::List(vec![REPO_B.to_string()]),
    );
    let allowlist = BackendAllowlistConfig {
        orgs: vec![ORG_A.to_string(), ORG_B.to_string()],
        repos,
    };

    let auth = BackendAuthConfig {
        admin_tokens: vec![ADMIN_TOKEN.to_string()],
        token_seed: Some(TOKEN_SEED.to_string()),
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
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

/// Create a minimal Ito project directory structure with changes and modules.
fn seed_project(
    data_dir: &std::path::Path,
    org: &str,
    repo: &str,
    change_ids: &[&str],
    modules: &[(&str, &str)], // (module_id, module_name)
) {
    let ito_dir = data_dir.join("projects").join(org).join(repo).join(".ito");
    let changes_dir = ito_dir.join("changes");
    let modules_dir = ito_dir.join("modules");

    std::fs::create_dir_all(&changes_dir).unwrap();

    for change_id in change_ids {
        let change_dir = changes_dir.join(change_id);
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(
            change_dir.join("proposal.md"),
            format!("# Proposal for {change_id}\n\nSome content."),
        )
        .unwrap();
        std::fs::write(
            change_dir.join("tasks.md"),
            format!("## 1. Tasks\n- [ ] 1.1 First task for {change_id}\n- [x] 1.2 Done task"),
        )
        .unwrap();
    }

    for (module_id, module_name) in modules {
        // Module directories use the `{id}_{name}` naming convention
        let dir_name = format!("{module_id}_{module_name}");
        let module_dir = modules_dir.join(&dir_name);
        std::fs::create_dir_all(&module_dir).unwrap();
        std::fs::write(
            module_dir.join("spec.md"),
            format!("# {module_name}\n\n{module_name} module specification."),
        )
        .unwrap();
    }
}

fn project_url(base_url: &str, org: &str, repo: &str, path: &str) -> String {
    format!("{base_url}/api/v1/projects/{org}/{repo}/{path}")
}

// ── Tests ──────────────────────────────────────────────────────────

#[tokio::test]
async fn admin_token_lists_changes_for_project_a() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_A, REPO_A, "changes"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let changes: Vec<ApiChangeSummary> = resp.json().await.unwrap();
    assert_eq!(changes.len(), 1, "project A should have 1 change");
    assert_eq!(changes[0].id, "001-01_alpha-feature");
}

#[tokio::test]
async fn admin_token_lists_changes_for_project_b() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_B, REPO_B, "changes"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let changes: Vec<ApiChangeSummary> = resp.json().await.unwrap();
    assert_eq!(changes.len(), 2, "project B should have 2 changes");
    let ids: Vec<&str> = changes.iter().map(|c| c.id.as_str()).collect();
    assert!(ids.contains(&"002-01_beta-fix"));
    assert!(ids.contains(&"002-02_beta-refactor"));
}

#[tokio::test]
async fn derived_token_for_project_a_accesses_project_a() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let token_a = ito_backend::derive_project_token(TOKEN_SEED, ORG_A, REPO_A);

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_A, REPO_A, "changes"))
        .header("Authorization", format!("Bearer {token_a}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let changes: Vec<ApiChangeSummary> = resp.json().await.unwrap();
    assert_eq!(changes.len(), 1);
}

#[tokio::test]
async fn derived_token_for_project_a_cannot_access_project_b() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let token_a = ito_backend::derive_project_token(TOKEN_SEED, ORG_A, REPO_A);

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_B, REPO_B, "changes"))
        .header("Authorization", format!("Bearer {token_a}"))
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        401,
        "project A token must not access project B"
    );

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "unauthorized");
}

#[tokio::test]
async fn derived_token_for_project_b_cannot_access_project_a() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let token_b = ito_backend::derive_project_token(TOKEN_SEED, ORG_B, REPO_B);

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_A, REPO_A, "changes"))
        .header("Authorization", format!("Bearer {token_b}"))
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        401,
        "project B token must not access project A"
    );
}

#[tokio::test]
async fn non_allowlisted_repo_in_allowed_org_is_rejected() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    // org_b allows only REPO_B ("gadgets"), not "secret-repo"
    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_B, "secret-repo", "changes"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        403,
        "non-allowlisted repo in an allowed org should be forbidden"
    );
}

#[tokio::test]
async fn modules_are_isolated_between_projects() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();

    // Project A has 1 module
    let resp_a = client
        .get(project_url(&base_url, ORG_A, REPO_A, "modules"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp_a.status(), 200);
    let modules_a: Vec<ApiModuleSummary> = resp_a.json().await.unwrap();
    assert_eq!(modules_a.len(), 1, "project A should have 1 module");
    assert_eq!(modules_a[0].id, "001");
    assert_eq!(modules_a[0].name, "Backend");

    // Project B has 0 modules
    let resp_b = client
        .get(project_url(&base_url, ORG_B, REPO_B, "modules"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp_b.status(), 200);
    let modules_b: Vec<ApiModuleSummary> = resp_b.json().await.unwrap();
    assert_eq!(modules_b.len(), 0, "project B should have 0 modules");
}

#[tokio::test]
async fn events_are_isolated_between_projects() {
    let (base_url, dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();

    // Ingest events into project A
    let resp_a = client
        .post(project_url(&base_url, ORG_A, REPO_A, "events"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::json!({
                "events": [{
                    "v": 1,
                    "ts": "2026-02-28T10:00:00.000Z",
                    "entity": "task",
                    "entity_id": "1.1",
                    "scope": "001-01_alpha-feature",
                    "op": "create",
                    "to": "pending",
                    "actor": "cli",
                    "by": "@alice",
                    "ctx": { "session_id": "session-a" }
                }],
                "idempotency_key": "mt-key-a-001"
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp_a.status(), 200);

    // Ingest events into project B
    let resp_b = client
        .post(project_url(&base_url, ORG_B, REPO_B, "events"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::json!({
                "events": [{
                    "v": 1,
                    "ts": "2026-02-28T11:00:00.000Z",
                    "entity": "task",
                    "entity_id": "2.1",
                    "scope": "002-01_beta-fix",
                    "op": "create",
                    "to": "pending",
                    "actor": "cli",
                    "by": "@bob",
                    "ctx": { "session_id": "session-b" }
                }, {
                    "v": 1,
                    "ts": "2026-02-28T11:01:00.000Z",
                    "entity": "task",
                    "entity_id": "2.2",
                    "scope": "002-01_beta-fix",
                    "op": "create",
                    "to": "pending",
                    "actor": "cli",
                    "by": "@bob",
                    "ctx": { "session_id": "session-b" }
                }],
                "idempotency_key": "mt-key-b-001"
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp_b.status(), 200);

    // Verify project A's audit log has exactly 1 event
    let log_a = dir
        .path()
        .join("projects")
        .join(ORG_A)
        .join(REPO_A)
        .join(".ito")
        .join(".state")
        .join("audit")
        .join("events.jsonl");
    let content_a = std::fs::read_to_string(&log_a).unwrap();
    assert_eq!(
        content_a.lines().count(),
        1,
        "project A should have exactly 1 event"
    );

    // Verify project B's audit log has exactly 2 events
    let log_b = dir
        .path()
        .join("projects")
        .join(ORG_B)
        .join(REPO_B)
        .join(".ito")
        .join(".state")
        .join("audit")
        .join("events.jsonl");
    let content_b = std::fs::read_to_string(&log_b).unwrap();
    assert_eq!(
        content_b.lines().count(),
        2,
        "project B should have exactly 2 events"
    );
}

#[tokio::test]
async fn get_single_change_returns_detail() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(
            &base_url,
            ORG_A,
            REPO_A,
            "changes/001-01_alpha-feature",
        ))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let change: ApiChange = resp.json().await.unwrap();
    assert_eq!(change.id, "001-01_alpha-feature");
    assert!(
        change.proposal.is_some(),
        "change should include proposal content"
    );
}

#[tokio::test]
async fn get_nonexistent_change_returns_404() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(
            &base_url,
            ORG_A,
            REPO_A,
            "changes/999-99_does-not-exist",
        ))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404, "nonexistent change should return 404");

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "not_found");
}

#[tokio::test]
async fn get_change_tasks_returns_task_list() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(
            &base_url,
            ORG_A,
            REPO_A,
            "changes/001-01_alpha-feature/tasks",
        ))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let task_list: ApiTaskList = resp.json().await.unwrap();
    assert_eq!(task_list.change_id, "001-01_alpha-feature");
    assert_eq!(task_list.tasks.len(), 2, "seeded change has 2 tasks");

    // Verify one is pending and one is complete
    let pending_count = task_list
        .tasks
        .iter()
        .filter(|t| t.status == "pending")
        .count();
    let complete_count = task_list
        .tasks
        .iter()
        .filter(|t| t.status == "complete")
        .count();
    assert_eq!(pending_count, 1, "one task should be pending");
    assert_eq!(complete_count, 1, "one task should be complete");
}

#[tokio::test]
async fn get_single_module_returns_detail() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_A, REPO_A, "modules/001"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let module: ApiModule = resp.json().await.unwrap();
    assert_eq!(module.id, "001");
    assert_eq!(module.name, "Backend");
}

#[tokio::test]
async fn get_nonexistent_module_returns_404() {
    let (base_url, _dir) = spawn_multi_tenant().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(project_url(&base_url, ORG_A, REPO_A, "modules/999"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404, "nonexistent module should return 404");

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "not_found");
}
