//! Integration tests for the multi-tenant event ingest endpoint.
//!
//! Tests cover:
//! - Successful batch ingestion via project-scoped route
//! - Idempotent retry (same idempotency key returns duplicates)
//! - Empty batch accepted
//! - Missing idempotency key rejected
//! - Authentication required

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ── Test-facing types ──────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct IngestRequest {
    events: Vec<serde_json::Value>,
    idempotency_key: String,
}

#[derive(Debug, Deserialize)]
struct IngestResponse {
    accepted: usize,
    duplicates: usize,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ErrorResponse {
    error: String,
    code: String,
}

// ── Test helper ────────────────────────────────────────────────────

const TEST_ORG: &str = "test-org";
const TEST_REPO: &str = "test-repo";

async fn spawn_backend() -> (String, String, tempfile::TempDir) {
    let data_dir = tempfile::tempdir().unwrap();
    let admin_token = "ingest-admin-token".to_string();
    let token_seed = "ingest-seed".to_string();

    // Create allowlisted project directory
    let project_ito = data_dir
        .path()
        .join("projects")
        .join(TEST_ORG)
        .join(TEST_REPO)
        .join(".ito");
    std::fs::create_dir_all(&project_ito).unwrap();

    let mut repos = BTreeMap::new();
    repos.insert(
        TEST_ORG.to_string(),
        BackendRepoPolicy::All("*".to_string()),
    );
    let allowlist = BackendAllowlistConfig {
        orgs: vec![TEST_ORG.to_string()],
        repos,
    };

    let auth = BackendAuthConfig {
        admin_tokens: vec![admin_token.clone()],
        token_seed: Some(token_seed),
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

    (base_url, admin_token, data_dir)
}

fn events_url(base_url: &str) -> String {
    format!("{base_url}/api/v1/projects/{TEST_ORG}/{TEST_REPO}/events")
}

fn make_event(entity_id: &str) -> serde_json::Value {
    serde_json::json!({
        "v": 1,
        "ts": "2026-02-28T10:00:00.000Z",
        "entity": "task",
        "entity_id": entity_id,
        "scope": "test-change",
        "op": "create",
        "to": "pending",
        "actor": "cli",
        "by": "@test",
        "ctx": {
            "session_id": "test-session"
        }
    })
}

// ── Tests ──────────────────────────────────────────────────────────

#[tokio::test]
async fn ingest_accepts_event_batch() {
    let (base_url, token, dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .post(events_url(&base_url))
        .header("Authorization", format!("Bearer {token}"))
        .json(&IngestRequest {
            events: vec![make_event("1.1"), make_event("1.2")],
            idempotency_key: "key-abc-001".to_string(),
        })
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: IngestResponse = resp.json().await.unwrap();
    assert_eq!(body.accepted, 2);
    assert_eq!(body.duplicates, 0);

    // Verify events were written to the audit log
    let log_path = dir
        .path()
        .join("projects")
        .join(TEST_ORG)
        .join(TEST_REPO)
        .join(".ito")
        .join(".state")
        .join("audit")
        .join("events.jsonl");
    assert!(log_path.is_file(), "audit log should exist");
    let content = std::fs::read_to_string(&log_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);
}

#[tokio::test]
async fn ingest_idempotent_retry_returns_duplicates() {
    let (base_url, token, dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let request_body = IngestRequest {
        events: vec![make_event("2.1")],
        idempotency_key: "key-idempotent-001".to_string(),
    };

    // First request: accepted
    let resp1 = client
        .post(events_url(&base_url))
        .header("Authorization", format!("Bearer {token}"))
        .json(&request_body)
        .send()
        .await
        .unwrap();

    assert_eq!(resp1.status(), 200);
    let body1: IngestResponse = resp1.json().await.unwrap();
    assert_eq!(body1.accepted, 1);
    assert_eq!(body1.duplicates, 0);

    // Second request with same idempotency key: duplicates
    let resp2 = client
        .post(events_url(&base_url))
        .header("Authorization", format!("Bearer {token}"))
        .json(&request_body)
        .send()
        .await
        .unwrap();

    assert_eq!(resp2.status(), 200);
    let body2: IngestResponse = resp2.json().await.unwrap();
    assert_eq!(body2.accepted, 0);
    assert_eq!(body2.duplicates, 1);

    // Verify only one event was written (not duplicated)
    let log_path = dir
        .path()
        .join("projects")
        .join(TEST_ORG)
        .join(TEST_REPO)
        .join(".ito")
        .join(".state")
        .join("audit")
        .join("events.jsonl");
    let content = std::fs::read_to_string(&log_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "idempotent retry should not duplicate events"
    );
}

#[tokio::test]
async fn ingest_empty_batch_accepted() {
    let (base_url, token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .post(events_url(&base_url))
        .header("Authorization", format!("Bearer {token}"))
        .json(&IngestRequest {
            events: vec![],
            idempotency_key: "key-empty-001".to_string(),
        })
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: IngestResponse = resp.json().await.unwrap();
    assert_eq!(body.accepted, 0);
    assert_eq!(body.duplicates, 0);
}

#[tokio::test]
async fn ingest_missing_idempotency_key_rejected() {
    let (base_url, token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .post(events_url(&base_url))
        .header("Authorization", format!("Bearer {token}"))
        .json(&IngestRequest {
            events: vec![make_event("3.1")],
            idempotency_key: String::new(),
        })
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "bad_request");
    assert!(body.error.contains("idempotency_key"));
}

#[tokio::test]
async fn ingest_requires_authentication() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .post(events_url(&base_url))
        .json(&IngestRequest {
            events: vec![make_event("4.1")],
            idempotency_key: "key-noauth-001".to_string(),
        })
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);
}
