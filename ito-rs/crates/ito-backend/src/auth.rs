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
#[path = "auth_tests.rs"]
mod auth_tests;
