//! Multi-tenant bearer token authentication for the backend API.
//!
//! Two token tiers are supported:
//!
//! - **Admin tokens**: authorize access to any project namespace.
//! - **Derived project tokens**: authorize exactly one `{org}/{repo}`,
//!   computed as `HMAC-SHA256(token_seed, "{org}/{repo}")`.
//!
//! Health and readiness endpoints bypass authentication.

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;
use std::sync::Arc;

use crate::error::ApiErrorResponse;
use crate::state::AppState;

type HmacSha256 = Hmac<Sha256>;

/// Paths that bypass authentication entirely.
const EXEMPT_PATHS: &[&str] = &["/api/v1/health", "/api/v1/ready"];

/// Derive a per-project token from a seed and a project key.
///
/// The project key is `"{org}/{repo}"`. The token is the lowercase hex
/// representation of `HMAC-SHA256(seed, project_key)`.
pub fn derive_project_token(seed: &str, org: &str, repo: &str) -> String {
    let project_key = format!("{org}/{repo}");
    let mut mac = HmacSha256::new_from_slice(seed.as_bytes()).expect("HMAC accepts any key length");
    mac.update(project_key.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Constant-time byte comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Token validation result indicating which tier matched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "scope", rename_all = "lowercase")]
pub enum TokenScope {
    /// Admin token — authorized for any project.
    Admin,
    /// Per-project token — authorized for a specific `{org}/{repo}`.
    Project {
        /// Organization.
        org: String,
        /// Repository.
        repo: String,
    },
}

/// Validate a bearer token against the backend auth configuration.
///
/// Returns the matched [`TokenScope`] or `None` if the token is invalid.
pub fn validate_token(state: &AppState, token: &str, org: &str, repo: &str) -> Option<TokenScope> {
    // Check admin tokens first
    for admin_token in &state.auth.admin_tokens {
        if constant_time_eq(token.as_bytes(), admin_token.as_bytes()) {
            return Some(TokenScope::Admin);
        }
    }

    // Check derived project token
    let Some(seed) = &state.auth.token_seed else {
        return None;
    };

    let expected = derive_project_token(seed, org, repo);
    if constant_time_eq(token.as_bytes(), expected.as_bytes()) {
        return Some(TokenScope::Project {
            org: org.to_string(),
            repo: repo.to_string(),
        });
    }

    None
}

/// Extract `{org}` and `{repo}` from the request path.
///
/// Expected prefix: `/api/v1/projects/{org}/{repo}/...`
fn extract_org_repo(path: &str) -> Option<(&str, &str)> {
    let rest = path.strip_prefix("/api/v1/projects/")?;
    let (org, rest) = rest.split_once('/')?;
    // repo is the next segment (until next `/` or end)
    let repo = rest.split('/').next()?;
    if org.is_empty() || repo.is_empty() {
        return None;
    }
    Some((org, repo))
}

/// Axum middleware that enforces multi-tenant bearer token authentication.
///
/// Health and readiness endpoints are exempt. All project-scoped routes
/// must include a valid `Authorization: Bearer <token>` header.
///
/// Allowlist checks run before token validation.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path();
    let normalized_path = path.trim_end_matches('/');

    // Exempt health and readiness endpoints
    for exempt in EXEMPT_PATHS {
        if normalized_path == exempt.trim_end_matches('/') {
            return next.run(request).await;
        }
    }

    // Extract org/repo from the path
    let Some((org, repo)) = extract_org_repo(path) else {
        // Non-project routes that aren't exempt are also passed through
        // (e.g., unknown paths will 404 via the router)
        return next.run(request).await;
    };

    // Enforce allowlist before token check
    if !state.allowlist.is_allowed(org, repo) {
        return ApiErrorResponse::forbidden(format!(
            "Organization/repository '{org}/{repo}' is not allowed"
        ))
        .into_response();
    }

    // Extract bearer token
    let bearer_token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let Some(token) = bearer_token else {
        return ApiErrorResponse::unauthorized("Missing bearer token").into_response();
    };

    let Some(scope) = validate_token(&state, token, org, repo) else {
        return ApiErrorResponse::unauthorized("Invalid bearer token").into_response();
    };

    // Store the token scope in request extensions for downstream handlers
    request.extensions_mut().insert(scope);

    next.run(request).await
}

#[cfg(test)]
mod tests {
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
}
