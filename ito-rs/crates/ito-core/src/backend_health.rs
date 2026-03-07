//! Backend health-check client for validating connectivity and auth.
//!
//! Provides a reusable health-check function that probes the backend server's
//! health, readiness, and auth verify endpoints. Used by `ito backend status`
//! and available for programmatic consumers.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::backend_client::BackendRuntime;

/// Status report from a backend health check.
///
/// Contains results from probing `/api/v1/health`, `/api/v1/ready`, and
/// `/api/v1/projects/{org}/{repo}/auth/verify`.
#[derive(Debug, Clone, Serialize)]
pub struct BackendHealthStatus {
    /// Whether the server responded to the health endpoint.
    pub server_reachable: bool,
    /// Whether the health endpoint returned `"ok"`.
    pub server_healthy: bool,
    /// Whether the ready endpoint returned `"ready"`.
    pub server_ready: bool,
    /// Server version from the health response.
    pub server_version: Option<String>,
    /// Reason from the ready endpoint when not ready.
    pub ready_reason: Option<String>,
    /// Whether the auth verify endpoint returned 200.
    pub auth_verified: bool,
    /// Token scope from auth verify (e.g. "admin", "project").
    pub token_scope: Option<String>,
    /// Error message if any check failed.
    pub error: Option<String>,
}

/// Health endpoint response shape.
#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

/// Ready endpoint response shape.
#[derive(Debug, Deserialize)]
struct ReadyResponse {
    status: String,
    reason: Option<String>,
}

/// Auth verify endpoint response shape.
#[derive(Debug, Deserialize)]
struct AuthVerifyResponse {
    #[allow(dead_code)]
    valid: bool,
    scope: String,
}

/// Default timeout for health-check requests (5 seconds).
const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

/// Check backend health, readiness, and auth verification.
///
/// Makes three HTTP requests:
/// 1. `GET /api/v1/health` — server is alive and responding
/// 2. `GET /api/v1/ready` — server data directory is accessible
/// 3. `GET /api/v1/projects/{org}/{repo}/auth/verify` — token is valid
///
/// Uses a 5-second timeout per request, independent of the runtime's
/// configured timeout for data operations.
pub fn check_backend_health(runtime: &BackendRuntime) -> BackendHealthStatus {
    let mut status = BackendHealthStatus {
        server_reachable: false,
        server_healthy: false,
        server_ready: false,
        server_version: None,
        ready_reason: None,
        auth_verified: false,
        token_scope: None,
        error: None,
    };

    let config = ureq::Agent::config_builder()
        .timeout_global(Some(HEALTH_CHECK_TIMEOUT))
        // Disable automatic error on 4xx/5xx so we can map status codes
        .http_status_as_error(false)
        .build();
    let agent: ureq::Agent = config.into();

    // 1. Health check
    let health_url = format!("{}/api/v1/health", runtime.base_url);
    match agent.get(&health_url).call() {
        Ok(mut response) => {
            status.server_reachable = true;
            let status_code = response.status().as_u16();
            if status_code != 200 {
                status.error = Some(format!("Health endpoint returned HTTP {status_code}"));
                return status;
            }
            let text = response
                .body_mut()
                .read_to_string()
                .unwrap_or_else(|_| String::new());
            match serde_json::from_str::<HealthResponse>(&text) {
                Ok(health) => {
                    status.server_healthy = health.status == "ok";
                    status.server_version = Some(health.version);
                }
                Err(e) => {
                    status.error = Some(format!("Failed to parse health response: {e}"));
                    return status;
                }
            }
        }
        Err(e) => {
            status.error = Some(format!("Server unreachable: {e}"));
            return status;
        }
    }

    // 2. Ready check
    let ready_url = format!("{}/api/v1/ready", runtime.base_url);
    match agent.get(&ready_url).call() {
        Ok(mut response) => {
            let status_code = response.status().as_u16();
            let text = response
                .body_mut()
                .read_to_string()
                .unwrap_or_else(|_| String::new());
            if status_code == 200 {
                match serde_json::from_str::<ReadyResponse>(&text) {
                    Ok(ready) => {
                        status.server_ready = ready.status == "ready";
                        status.ready_reason = ready.reason;
                    }
                    Err(e) => {
                        status.error = Some(format!("Failed to parse ready response: {e}"));
                        return status;
                    }
                }
            } else if status_code == 503 {
                // Not ready is a valid state, try to parse the body
                status.server_ready = false;
                match serde_json::from_str::<ReadyResponse>(&text) {
                    Ok(ready) => {
                        status.ready_reason = ready.reason;
                    }
                    Err(_) => {
                        status.ready_reason = Some("Server returned 503".to_string());
                    }
                }
            } else {
                status.error = Some(format!("Ready endpoint returned HTTP {status_code}"));
                return status;
            }
        }
        Err(e) => {
            status.error = Some(format!("Ready check failed: {e}"));
            return status;
        }
    }

    // 3. Auth verify
    let auth_url = format!(
        "{}/api/v1/projects/{}/{}/auth/verify",
        runtime.base_url, runtime.org, runtime.repo
    );
    match agent
        .get(&auth_url)
        .header("Authorization", &format!("Bearer {}", runtime.token))
        .call()
    {
        Ok(mut response) => {
            let status_code = response.status().as_u16();
            if status_code == 200 {
                status.auth_verified = true;
                let text = response
                    .body_mut()
                    .read_to_string()
                    .unwrap_or_else(|_| String::new());
                match serde_json::from_str::<AuthVerifyResponse>(&text) {
                    Ok(verify) => {
                        status.token_scope = Some(verify.scope);
                    }
                    Err(_) => {
                        // Auth passed but couldn't parse scope - still considered verified
                    }
                }
            } else if status_code == 401 {
                status.auth_verified = false;
                status.error = Some(
                    "Authentication failed. Check your token or seed. \
                     Use 'ito backend generate-token' to derive a project token from the server seed."
                        .to_string(),
                );
            } else if status_code == 403 {
                status.auth_verified = false;
                status.error = Some(format!(
                    "Organization/repository '{}/{}' is not in the server allowlist.",
                    runtime.org, runtime.repo
                ));
            } else {
                status.error = Some(format!("Auth verify returned HTTP {status_code}"));
            }
        }
        Err(e) => {
            status.error = Some(format!("Auth verify failed: {e}"));
        }
    }

    status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_health_status_default_is_all_false() {
        let status = BackendHealthStatus {
            server_reachable: false,
            server_healthy: false,
            server_ready: false,
            server_version: None,
            ready_reason: None,
            auth_verified: false,
            token_scope: None,
            error: None,
        };

        assert!(!status.server_reachable);
        assert!(!status.server_healthy);
        assert!(!status.server_ready);
        assert!(!status.auth_verified);
        assert!(status.server_version.is_none());
        assert!(status.ready_reason.is_none());
        assert!(status.token_scope.is_none());
        assert!(status.error.is_none());
    }

    #[test]
    fn backend_health_status_serializes_to_json() {
        let status = BackendHealthStatus {
            server_reachable: true,
            server_healthy: true,
            server_ready: true,
            server_version: Some("0.1.0".to_string()),
            ready_reason: None,
            auth_verified: true,
            token_scope: Some("project".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&status).expect("should serialize");
        assert!(json.contains("\"server_reachable\":true"));
        assert!(json.contains("\"server_healthy\":true"));
        assert!(json.contains("\"server_ready\":true"));
        assert!(json.contains("\"server_version\":\"0.1.0\""));
        assert!(json.contains("\"auth_verified\":true"));
        assert!(json.contains("\"token_scope\":\"project\""));
    }

    #[test]
    fn backend_health_status_serializes_error_state() {
        let status = BackendHealthStatus {
            server_reachable: false,
            server_healthy: false,
            server_ready: false,
            server_version: None,
            ready_reason: None,
            auth_verified: false,
            token_scope: None,
            error: Some("Connection refused".to_string()),
        };

        let json = serde_json::to_string(&status).expect("should serialize");
        assert!(json.contains("\"server_reachable\":false"));
        assert!(json.contains("\"error\":\"Connection refused\""));
    }
}
