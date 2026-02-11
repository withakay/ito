//! Token-based authentication for non-loopback access.
//!
//! When the server binds to a non-loopback address (e.g. `0.0.0.0`), every
//! request must carry a valid token. The token is a deterministic SHA-256 hash
//! of `hostname + project_root + salt`, which gives:
//!
//! - **Stable across restarts** — no need to re-authenticate after a server bounce.
//! - **Host-scoped** — a token from one machine cannot be reused on another.
//! - **Project-scoped** — separate projects get separate tokens.
//!
//! Loopback connections bypass authentication entirely.

use axum::{
    extract::{Query, Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;

const COOKIE_NAME: &str = "ito_token";
const SALT: &str = "ito-web-auth-v1";

/// Derive a deterministic authentication token for `project_root`.
///
/// The token is a 32-hex-char truncation of `SHA-256(salt ‖ hostname ‖ canonical_root)`.
/// It is safe to display in URLs and cookies.
pub fn generate_token(project_root: &std::path::Path) -> String {
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    let root = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf())
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

/// Return `true` when `bind` resolves to a loopback address.
///
/// Loopback connections are trusted and skip token authentication.
#[allow(clippy::match_like_matches_macro)]
pub fn is_loopback(bind: &str) -> bool {
    match bind {
        "127.0.0.1" => true,
        "localhost" => true,
        "::1" => true,
        "0:0:0:0:0:0:0:1" => true,
        _ => false,
    }
}

/// Shared state for the authentication middleware.
///
/// When `token` is `None` (loopback bind), all requests pass through
/// unauthenticated.
#[derive(Clone)]
pub struct AuthState {
    /// Expected token, or `None` when authentication is disabled (loopback).
    pub token: Option<String>,
}

/// Query-string parameters for token-based authentication.
#[derive(Deserialize)]
pub struct TokenQuery {
    token: Option<String>,
}

/// Axum middleware that enforces token authentication.
///
/// The token may be supplied via:
/// 1. The `ito_token` cookie (set automatically on first valid request).
/// 2. The `?token=…` query parameter.
///
/// On a valid query-string token the middleware sets an `HttpOnly` cookie so
/// subsequent requests authenticate transparently. Unauthenticated requests
/// receive a `403 Forbidden` HTML page with instructions.
pub async fn auth_middleware(
    State(auth): State<Arc<AuthState>>,
    jar: CookieJar,
    Query(query): Query<TokenQuery>,
    request: Request,
    next: Next,
) -> Response {
    // No auth required if no token configured (loopback)
    let Some(expected_token) = &auth.token else {
        return next.run(request).await;
    };

    // Check cookie first
    if let Some(cookie) = jar.get(COOKIE_NAME)
        && cookie.value() == expected_token
    {
        return next.run(request).await;
    }

    // Check query string
    if let Some(provided_token) = &query.token
        && provided_token == expected_token
    {
        // Valid token - run request and set cookie in response
        let response = next.run(request).await;

        // Add Set-Cookie header
        let cookie_value = format!(
            "{}={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=86400",
            COOKIE_NAME, expected_token
        );

        let (mut parts, body) = response.into_parts();
        if let Ok(cookie_header) = cookie_value.parse() {
            parts.headers.insert(header::SET_COOKIE, cookie_header);
        }

        return Response::from_parts(parts, body);
    }

    // No valid token - return 403 with helpful message
    let body = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Access Denied</title>
<style>
body {{ font-family: system-ui; background: #1a1b26; color: #c0caf5; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }}
.box {{ text-align: center; padding: 2rem; }}
h1 {{ color: #f7768e; }}
code {{ background: #24283b; padding: 0.5rem 1rem; border-radius: 4px; display: block; margin: 1rem 0; }}
</style>
</head>
<body>
<div class="box">
<h1>Access Denied</h1>
<p>This server requires a token for remote access.</p>
<p>Add the token to your URL:</p>
<code>?token={}</code>
</div>
</body>
</html>"#,
        expected_token
    );

    (
        StatusCode::FORBIDDEN,
        [(header::CONTENT_TYPE, "text/html")],
        body,
    )
        .into_response()
}
