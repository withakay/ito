//! Bearer token authentication for the backend API.
//!
//! All endpoints except `/api/v1/health` and `/api/v1/ready` require a valid
//! bearer token in the `Authorization` header.
//!
//! The default token is a deterministic SHA-256 hash of
//! `hostname + project_root + salt`, providing stable tokens across restarts
//! that are scoped to a specific host and project. An explicit token override
//! is also supported via configuration or CLI flag.

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Arc;

use crate::error::ApiErrorResponse;

const SALT: &str = "ito-backend-auth-v1";

/// Derive a deterministic authentication token for a project root.
///
/// The token is a 32-hex-char truncation of
/// `SHA-256(salt || hostname || canonical_root)`.
pub fn generate_token(project_root: &Path) -> String {
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    let root = project_root
        .canonicalize()
        .unwrap_or_else(|e| {
            eprintln!(
                "warning: could not canonicalize project root '{}': {e}. Token will be based on non-canonical path.",
                project_root.display()
    let root = project_root
        .canonicalize()
        .unwrap_or_else(|e| {
            eprintln!(
                "warning: could not canonicalize project root '{}': {}. Token will be based on non-canonical path.",
                project_root.display(),
                e
            );
            project_root.to_path_buf()
        })
        .to_string_lossy()
        .to_string();

    let mut hasher = Sha256::new();
    hasher.update(SALT.as_bytes());
    hasher.update(hostname.as_bytes());
    hasher.update(root.as_bytes());
    let result = hasher.finalize();

    // Use first 16 bytes (32 hex chars) for a shorter but still secure token
    hex::encode(&result[..16])
}

/// Shared state for the authentication middleware.
#[derive(Debug, Clone)]
pub struct AuthState {
    /// Expected bearer token.
    pub token: String,
}

/// Paths that bypass authentication.
const EXEMPT_PATHS: &[&str] = &["/api/v1/health", "/api/v1/ready"];

/// Axum middleware that enforces bearer token authentication.
///
/// Extracts the token from `Authorization: Bearer <token>` and compares it
/// to the expected value. Health and readiness endpoints are exempt.
pub async fn auth_middleware(
    State(auth): State<Arc<AuthState>>,
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path();

    // Exempt health and readiness endpoints
    for exempt in EXEMPT_PATHS {
        if path == *exempt {
            return next.run(request).await;
        }
    }

    // Extract bearer token from Authorization header
    let authorized = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .is_some_and(|token| token == auth.token);

    if authorized {
        return next.run(request).await;
    }

    ApiErrorResponse::unauthorized("Missing or invalid bearer token").into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn generate_token_is_deterministic() {
        let root = PathBuf::from("/tmp/test-project");
        let t1 = generate_token(&root);
        let t2 = generate_token(&root);
        assert_eq!(t1, t2);
    }

    #[test]
    fn generate_token_is_32_hex_chars() {
        let root = PathBuf::from("/tmp/test-project");
        let token = generate_token(&root);
        assert_eq!(token.len(), 32);
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn different_roots_produce_different_tokens() {
        let t1 = generate_token(&PathBuf::from("/tmp/project-a"));
        let t2 = generate_token(&PathBuf::from("/tmp/project-b"));
        assert_ne!(t1, t2);
    }

    #[test]
    fn exempt_paths_are_health_and_ready() {
        assert!(EXEMPT_PATHS.contains(&"/api/v1/health"));
        assert!(EXEMPT_PATHS.contains(&"/api/v1/ready"));
    }
}
