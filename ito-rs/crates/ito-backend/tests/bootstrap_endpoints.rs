//! Integration tests for the multi-tenant bootstrap endpoints.
//!
//! Tests cover:
//! - Health/version endpoint (unauthenticated)
//! - Ready endpoint (unauthenticated)
//! - Auth enforcement on project-scoped routes
//! - Admin token and derived project token access

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use serde::Deserialize;
use std::collections::BTreeMap;

// ── Test-facing response types ─────────────────────────────────────

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct ReadyResponse {
    status: String,
    #[allow(dead_code)]
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ErrorResponse {
    error: String,
    code: String,
}

// ── Test helper ────────────────────────────────────────────────────

/// Spin up a real multi-tenant backend server on a random port.
///
/// Creates a data directory with a project `test-org/test-repo`.
/// Returns `(base_url, admin_token, data_dir)`.
async fn spawn_backend() -> (String, String, tempfile::TempDir) {
    let data_dir = tempfile::tempdir().unwrap();
    let admin_token = "test-admin-token".to_string();
    let token_seed = "test-seed".to_string();

    // Create allowlisted project directory structure
    let project_ito = data_dir
        .path()
        .join("projects")
        .join("test-org")
        .join("test-repo")
        .join(".ito");
    std::fs::create_dir_all(&project_ito).unwrap();

    // Build allowlist
    let mut repos = BTreeMap::new();
    repos.insert(
        "test-org".to_string(),
        BackendRepoPolicy::All("*".to_string()),
    );
    let allowlist = BackendAllowlistConfig {
        orgs: vec!["test-org".to_string()],
        repos,
    };

    let auth = BackendAuthConfig {
        admin_tokens: vec![admin_token.clone()],
        token_seed: Some(token_seed),
    };

    // Bind to port 0 to get a random available port
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

    // Drop the pre-bound listener so `serve` can bind to the same port.
    drop(listener);

    tokio::spawn(async move {
        let _ = ito_backend::serve(config).await;
    });

    // Give the server a moment to bind
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    (base_url, admin_token, data_dir)
}

// ── Health endpoint tests ──────────────────────────────────────────

#[tokio::test]
async fn health_endpoint_returns_status_and_version() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/health"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: HealthResponse = resp.json().await.unwrap();
    assert_eq!(body.status, "ok");
    assert!(!body.version.is_empty(), "version must be non-empty");
    assert!(
        body.version.contains('.'),
        "version should contain a dot: {}",
        body.version
    );
}

#[tokio::test]
async fn health_endpoint_does_not_require_auth() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/health"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

// ── Ready endpoint tests ───────────────────────────────────────────

#[tokio::test]
async fn ready_endpoint_returns_ready_when_data_dir_exists() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/ready"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: ReadyResponse = resp.json().await.unwrap();
    assert_eq!(body.status, "ready");
}

#[tokio::test]
async fn ready_endpoint_does_not_require_auth() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/ready"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

// ── Auth enforcement tests ─────────────────────────────────────────

#[tokio::test]
async fn project_route_rejects_missing_token() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v1/projects/test-org/test-repo/changes"
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "unauthorized");
}

#[tokio::test]
async fn project_route_rejects_invalid_token() {
    let (base_url, _token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v1/projects/test-org/test-repo/changes"
        ))
        .header("Authorization", "Bearer wrong-token-value")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "unauthorized");
}

#[tokio::test]
async fn project_route_accepts_admin_token() {
    let (base_url, admin_token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v1/projects/test-org/test-repo/changes"
        ))
        .header("Authorization", format!("Bearer {admin_token}"))
        .send()
        .await
        .unwrap();

    // Should succeed (200) — empty changes list since no changes exist
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn project_route_accepts_derived_project_token() {
    let (base_url, _admin_token, _dir) = spawn_backend().await;

    let project_token = ito_backend::derive_project_token("test-seed", "test-org", "test-repo");

    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v1/projects/test-org/test-repo/changes"
        ))
        .header("Authorization", format!("Bearer {project_token}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn project_route_rejects_non_allowlisted_org() {
    let (base_url, admin_token, _dir) = spawn_backend().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v1/projects/forbidden-org/some-repo/changes"
        ))
        .header("Authorization", format!("Bearer {admin_token}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 403);

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "forbidden");
}
