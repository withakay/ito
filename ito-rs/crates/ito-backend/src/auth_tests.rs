use super::*;
use ito_core::fs_project_store::FsBackendProjectStore;

fn test_app_state(auth: ito_config::types::BackendAuthConfig) -> AppState {
    let store = Arc::new(FsBackendProjectStore::new("/data"));
    AppState::new(
        std::path::PathBuf::from("/data"),
        store,
        ito_config::types::BackendAllowlistConfig::default(),
        auth,
    )
}

#[test]
fn derive_project_token_is_deterministic() {
    let t1 = derive_project_token("secret", "acme", "repo1");
    let t2 = derive_project_token("secret", "acme", "repo1");
    assert_eq!(t1, t2);
}

#[test]
fn derive_project_token_differs_by_project() {
    let t1 = derive_project_token("secret", "acme", "repo1");
    let t2 = derive_project_token("secret", "acme", "repo2");
    assert_ne!(t1, t2);
}

#[test]
fn derive_project_token_differs_by_seed() {
    let t1 = derive_project_token("seed1", "acme", "repo1");
    let t2 = derive_project_token("seed2", "acme", "repo1");
    assert_ne!(t1, t2);
}

#[test]
fn derive_project_token_is_64_hex_chars() {
    let token = derive_project_token("secret", "org", "repo");
    assert_eq!(token.len(), 64);
    assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn extract_org_repo_valid_path() {
    let result = extract_org_repo("/api/v1/projects/acme/infra/changes");
    assert_eq!(result, Some(("acme", "infra")));
}

#[test]
fn extract_org_repo_no_trailing() {
    let result = extract_org_repo("/api/v1/projects/acme/infra");
    assert_eq!(result, Some(("acme", "infra")));
}

#[test]
fn extract_org_repo_non_project_path() {
    let result = extract_org_repo("/api/v1/health");
    assert!(result.is_none());
}

#[test]
fn validate_token_admin_matches() {
    let state = test_app_state(ito_config::types::BackendAuthConfig {
        admin_tokens: vec!["admin-secret".to_string()],
        token_seed: None,
    });
    let result = validate_token(&state, "admin-secret", "any", "project");
    assert_eq!(result, Some(TokenScope::Admin));
}

#[test]
fn validate_token_project_matches() {
    let state = test_app_state(ito_config::types::BackendAuthConfig {
        admin_tokens: vec![],
        token_seed: Some("my-seed".to_string()),
    });
    let expected_token = derive_project_token("my-seed", "acme", "repo1");
    let result = validate_token(&state, &expected_token, "acme", "repo1");
    assert_eq!(
        result,
        Some(TokenScope::Project {
            org: "acme".to_string(),
            repo: "repo1".to_string(),
        })
    );
}

#[test]
fn validate_token_wrong_project_fails() {
    let state = test_app_state(ito_config::types::BackendAuthConfig {
        admin_tokens: vec![],
        token_seed: Some("my-seed".to_string()),
    });
    let token = derive_project_token("my-seed", "acme", "repo1");
    // Try to use token for a different project
    let result = validate_token(&state, &token, "acme", "repo2");
    assert!(result.is_none());
}

#[test]
fn validate_token_invalid_fails() {
    let state = test_app_state(ito_config::types::BackendAuthConfig {
        admin_tokens: vec!["admin".to_string()],
        token_seed: Some("seed".to_string()),
    });
    let result = validate_token(&state, "bogus-token", "acme", "repo1");
    assert!(result.is_none());
}

#[test]
fn exempt_paths_are_health_and_ready() {
    assert!(EXEMPT_PATHS.contains(&"/api/v1/health"));
    assert!(EXEMPT_PATHS.contains(&"/api/v1/ready"));
}

#[test]
fn token_scope_serializes_admin() {
    let scope = TokenScope::Admin;
    let json = serde_json::to_value(&scope).unwrap();
    assert_eq!(json["scope"], "admin");
}

#[test]
fn token_scope_serializes_project() {
    let scope = TokenScope::Project {
        org: "acme".to_string(),
        repo: "infra".to_string(),
    };
    let json = serde_json::to_value(&scope).unwrap();
    assert_eq!(json["scope"], "project");
    assert_eq!(json["org"], "acme");
    assert_eq!(json["repo"], "infra");
}
