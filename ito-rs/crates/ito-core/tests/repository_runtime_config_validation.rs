use serde_json::json;
use tempfile::TempDir;

use ito_config::ConfigContext;
use ito_core::repository_runtime::resolve_repository_runtime;

fn write(path: impl AsRef<std::path::Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dirs should exist");
    }
    std::fs::write(path, contents).expect("test fixture should write");
}

#[test]
fn invalid_repository_mode_fails_fast() {
    let repo = TempDir::new().expect("temp repo");
    let ito_path = repo.path().join(".ito");
    let config = json!({ "repository": { "mode": "bogus" } });
    write(ito_path.join("config.json"), &config.to_string());

    let err = resolve_repository_runtime(&ito_path, &ConfigContext::default())
        .err()
        .expect("invalid repository.mode should fail fast");
    let msg = err.to_string();
    assert!(msg.contains("repository.mode"), "unexpected error: {msg}");
    assert!(msg.contains("filesystem"), "unexpected error: {msg}");
    assert!(msg.contains("sqlite"), "unexpected error: {msg}");
}
