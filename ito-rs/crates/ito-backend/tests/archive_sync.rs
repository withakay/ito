use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const ORG: &str = "acme";
const REPO: &str = "widgets";
const ADMIN_TOKEN: &str = "archive-sync-admin-token";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ArtifactBundle {
    change_id: String,
    proposal: Option<String>,
    design: Option<String>,
    tasks: Option<String>,
    specs: Vec<(String, String)>,
    revision: String,
}

#[derive(Debug, Serialize)]
struct PushRequest {
    proposal: Option<String>,
    design: Option<String>,
    tasks: Option<String>,
    specs: Vec<(String, String)>,
    revision: String,
}

#[derive(Debug, Deserialize)]
struct PushResult {
    change_id: String,
    new_revision: String,
}

#[derive(Debug, Deserialize)]
struct ArchiveResult {
    change_id: String,
    archived_at: String,
}

async fn spawn_backend() -> (String, tempfile::TempDir) {
    let data_dir = tempfile::tempdir().unwrap();
    seed_project(data_dir.path());

    let mut repos = BTreeMap::new();
    repos.insert(ORG.to_string(), BackendRepoPolicy::List(vec![REPO.to_string()]));
    let allowlist = BackendAllowlistConfig {
        orgs: vec![ORG.to_string()],
        repos,
    };
    let auth = BackendAuthConfig {
        admin_tokens: vec![ADMIN_TOKEN.to_string()],
        token_seed: Some("archive-sync-seed".to_string()),
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

fn seed_project(data_dir: &std::path::Path) {
    let change_dir = data_dir
        .join("projects")
        .join(ORG)
        .join(REPO)
        .join(".ito")
        .join("changes")
        .join("025-05_archive-me");
    std::fs::create_dir_all(change_dir.join("specs/spec-one")).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "# Proposal\n").unwrap();
    std::fs::write(change_dir.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "- [x] done\n").unwrap();
    std::fs::write(change_dir.join("specs/spec-one/spec.md"), "## ADDED\n").unwrap();
}

fn project_url(base_url: &str, path: &str) -> String {
    format!("{base_url}/api/v1/projects/{ORG}/{REPO}/{path}")
}

#[tokio::test]
async fn sync_pull_returns_artifact_bundle() {
    let (base_url, _dir) = spawn_backend().await;
    let client = reqwest::Client::new();

    let response = client
        .get(project_url(&base_url, "changes/025-05_archive-me/sync"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let bundle: ArtifactBundle = response.json().await.unwrap();
    assert_eq!(bundle.change_id, "025-05_archive-me");
    assert!(bundle.proposal.as_deref().unwrap_or_default().contains("Proposal"));
    assert_eq!(bundle.specs.len(), 1);
}

#[tokio::test]
async fn sync_push_updates_backend_artifacts() {
    let (base_url, dir) = spawn_backend().await;
    let client = reqwest::Client::new();

    let response = client
        .post(project_url(&base_url, "changes/025-05_archive-me/sync"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .json(&PushRequest {
            proposal: Some("# Updated Proposal\n".to_string()),
            design: Some("# Updated Design\n".to_string()),
            tasks: Some("- [x] shipped\n".to_string()),
            specs: vec![("spec-one".to_string(), "## MODIFIED\n".to_string())],
            revision: String::new(),
        })
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let result: PushResult = response.json().await.unwrap();
    assert_eq!(result.change_id, "025-05_archive-me");
    assert!(!result.new_revision.is_empty());

    let proposal = std::fs::read_to_string(
        dir.path()
            .join("projects")
            .join(ORG)
            .join(REPO)
            .join(".ito/changes/025-05_archive-me/proposal.md"),
    )
    .unwrap();
    assert_eq!(proposal, "# Updated Proposal\n");
}

#[tokio::test]
async fn archive_endpoint_promotes_specs_and_moves_change() {
    let (base_url, dir) = spawn_backend().await;
    let client = reqwest::Client::new();

    let response = client
        .post(project_url(&base_url, "changes/025-05_archive-me/archive"))
        .header("Authorization", format!("Bearer {ADMIN_TOKEN}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let result: ArchiveResult = response.json().await.unwrap();
    assert_eq!(result.change_id, "025-05_archive-me");
    assert!(!result.archived_at.is_empty());

    let ito_dir = dir.path().join("projects").join(ORG).join(REPO).join(".ito");
    assert!(ito_dir.join("specs/spec-one/spec.md").exists());
    assert!(!ito_dir.join("changes/025-05_archive-me").exists());
    let archived = ito_dir.join("changes/archive");
    let entries: Vec<_> = std::fs::read_dir(&archived)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .collect();
    assert_eq!(entries.len(), 1);
}
