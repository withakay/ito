//! Integration tests for the project bootstrap endpoints.
//!
//! Tests cover:
//! - Health/version endpoint (unauthenticated)
//! - Token introspection (`/api/v1/auth/whoami`) with valid and invalid tokens

use std::path::PathBuf;

use serde::Deserialize;

// ── Test-facing response types ─────────────────────────────────────

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct WhoamiResponse {
    project_id: String,
    project_root: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ErrorResponse {
    error: String,
    code: String,
}

// ── Test helper ────────────────────────────────────────────────────

/// Spin up a real backend server on a random port.
///
/// Returns `(base_url, token)`. The server runs in a background task and
/// is dropped when the test's tokio runtime shuts down.
async fn spawn_backend(project_dir: PathBuf) -> (String, String) {
    let token = "integration-test-token".to_string();

    // Bind to port 0 to get a random available port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{addr}");

    let config = ito_backend::BackendConfig {
        project_root: project_dir,
        ito_path: None,
        bind: "127.0.0.1".to_string(),
        port: addr.port(),
        token: Some(token.clone()),
        cors_origins: None,
    };

    // `serve` binds its own listener, so we use the same port we reserved.
    // Drop the pre-bound listener first so `serve` can bind to it.
    drop(listener);

    tokio::spawn(async move {
        let _ = ito_backend::serve(config).await;
    });

    // Give the server a moment to bind
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    (base_url, token)
}

// ── Health endpoint tests ──────────────────────────────────────────

#[tokio::test]
async fn health_endpoint_returns_status_and_version() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join(".ito")).unwrap();

    let (base_url, _token) = spawn_backend(dir.path().to_path_buf()).await;

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
    // Version should be a valid semver-ish string (e.g. "0.1.11")
    assert!(
        body.version.contains('.'),
        "version should contain a dot: {}",
        body.version
    );
}

#[tokio::test]
async fn health_endpoint_does_not_require_auth() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join(".ito")).unwrap();

    let (base_url, _token) = spawn_backend(dir.path().to_path_buf()).await;

    // Request without any Authorization header should still succeed
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/health"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

// ── Whoami endpoint tests ──────────────────────────────────────────

#[tokio::test]
async fn whoami_returns_project_identity_with_valid_token() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join(".ito")).unwrap();

    let (base_url, token) = spawn_backend(dir.path().to_path_buf()).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/auth/whoami"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: WhoamiResponse = resp.json().await.unwrap();
    // The project_id should be the directory name of the canonicalized path
    let expected_dir_name = dir
        .path()
        .canonicalize()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert_eq!(body.project_id, expected_dir_name);
    assert!(!body.project_root.is_empty());
}

#[tokio::test]
async fn whoami_rejects_missing_token() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join(".ito")).unwrap();

    let (base_url, _token) = spawn_backend(dir.path().to_path_buf()).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/auth/whoami"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "unauthorized");
}

#[tokio::test]
async fn whoami_rejects_invalid_token() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join(".ito")).unwrap();

    let (base_url, _token) = spawn_backend(dir.path().to_path_buf()).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base_url}/api/v1/auth/whoami"))
        .header("Authorization", "Bearer wrong-token-value")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);

    let body: ErrorResponse = resp.json().await.unwrap();
    assert_eq!(body.code, "unauthorized");
}
