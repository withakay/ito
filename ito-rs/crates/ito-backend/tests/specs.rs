use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
struct ApiSpecSummary {
    id: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct ApiSpecDocument {
    id: String,
    path: String,
    markdown: String,
}

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "spec-admin-token";
const TOKEN_SEED: &str = "spec-seed";

async fn spawn_backend_with_specs() -> String {
    let data_dir = tempfile::tempdir().unwrap();
    let ito_dir = data_dir.path().join("projects").join(ORG).join(REPO).join(".ito");
    std::fs::create_dir_all(ito_dir.join("specs/alpha")).unwrap();
    std::fs::create_dir_all(ito_dir.join("specs/beta")).unwrap();
    std::fs::write(ito_dir.join("specs/alpha/spec.md"), "# Alpha\n").unwrap();
    std::fs::write(ito_dir.join("specs/beta/spec.md"), "# Beta\n").unwrap();

    let mut repos = BTreeMap::new();
    repos.insert(ORG.to_string(), BackendRepoPolicy::List(vec![REPO.to_string()]));
    let allowlist = BackendAllowlistConfig {
        orgs: vec![ORG.to_string()],
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

    std::mem::forget(data_dir);
    base_url
}

fn project_url(base_url: &str, path: &str) -> String {
    format!("{base_url}/api/v1/projects/{ORG}/{REPO}/{path}")
}

#[tokio::test]
async fn list_specs_returns_promoted_specs() {
    let base_url = spawn_backend_with_specs().await;
    let client = reqwest::Client::new();

    let response = client
        .get(project_url(&base_url, "specs"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let specs: Vec<ApiSpecSummary> = response.json().await.unwrap();
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0].id, "alpha");
    assert_eq!(specs[1].id, "beta");
    assert!(specs[0].path.ends_with(".ito/specs/alpha/spec.md"));
}

#[tokio::test]
async fn get_spec_returns_markdown() {
    let base_url = spawn_backend_with_specs().await;
    let client = reqwest::Client::new();

    let response = client
        .get(project_url(&base_url, "specs/beta"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let spec: ApiSpecDocument = response.json().await.unwrap();
    assert_eq!(spec.id, "beta");
    assert!(spec.path.ends_with(".ito/specs/beta/spec.md"));
    assert_eq!(spec.markdown, "# Beta\n");
}
