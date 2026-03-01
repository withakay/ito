//! Backend API client factory and runtime.
//!
//! Creates and configures an HTTP client for the Ito backend API when
//! backend mode is enabled in the resolved configuration. The client
//! handles authentication, timeouts, and retry logic for transient failures.

use std::path::PathBuf;
use std::time::Duration;

use ito_config::types::BackendApiConfig;

use crate::errors::{CoreError, CoreResult};

/// Resolved backend runtime settings ready for client construction.
///
/// Constructed from [`BackendApiConfig`] with environment variable resolution
/// and validation applied. This type is only created when backend mode is
/// enabled and all required settings are present.
#[derive(Debug, Clone)]
pub struct BackendRuntime {
    /// Base URL for the backend API.
    pub base_url: String,
    /// Resolved bearer token for authentication.
    pub token: String,
    /// Request timeout.
    pub timeout: Duration,
    /// Maximum retry attempts for transient failures.
    pub max_retries: u32,
    /// Directory for artifact backup snapshots.
    pub backup_dir: PathBuf,
    /// Organization namespace for project-scoped routes.
    pub org: String,
    /// Repository namespace for project-scoped routes.
    pub repo: String,
}

impl BackendRuntime {
    /// Returns the project-scoped API path prefix: `/api/v1/projects/{org}/{repo}`.
    pub fn project_api_prefix(&self) -> String {
        format!(
            "{}/api/v1/projects/{}/{}",
            self.base_url, self.org, self.repo
        )
    }
}

/// Resolve backend runtime settings from config.
///
/// Returns `Ok(None)` when backend mode is disabled. Returns `Err` when
/// backend mode is enabled but required values (e.g., token) are missing.
pub fn resolve_backend_runtime(config: &BackendApiConfig) -> CoreResult<Option<BackendRuntime>> {
    if !config.enabled {
        return Ok(None);
    }

    let token = resolve_token(config)?;
    let backup_dir = resolve_backup_dir(config);
    let timeout = Duration::from_millis(config.timeout_ms);
    let (org, repo) = resolve_project_namespace(config)?;

    Ok(Some(BackendRuntime {
        base_url: config.url.clone(),
        token,
        timeout,
        max_retries: config.max_retries,
        backup_dir,
        org,
        repo,
    }))
}

/// Resolve the bearer token from explicit config or environment variable.
fn resolve_token(config: &BackendApiConfig) -> CoreResult<String> {
    if let Some(token) = &config.token {
        let token = token.trim();
        if !token.is_empty() {
            return Ok(token.to_string());
        }
    }

    let env_var = &config.token_env_var;
    match std::env::var(env_var) {
        Ok(val) if !val.trim().is_empty() => Ok(val.trim().to_string()),
        Ok(_) => Err(CoreError::validation(format!(
            "Backend mode is enabled but environment variable '{env_var}' is empty. \
             Set the token via '{env_var}' or 'backend.token' in config."
        ))),
        Err(_) => Err(CoreError::validation(format!(
            "Backend mode is enabled but environment variable '{env_var}' is not set. \
             Set the token via '{env_var}' or 'backend.token' in config."
        ))),
    }
}

/// Resolve the backup directory, falling back to `$HOME/.ito/backups`.
fn resolve_backup_dir(config: &BackendApiConfig) -> PathBuf {
    if let Some(dir) = &config.backup_dir {
        return PathBuf::from(dir);
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/tmp".to_string());

    PathBuf::from(home).join(".ito").join("backups")
}

/// Environment variable name for overriding the project organization namespace.
const ENV_PROJECT_ORG: &str = "ITO_BACKEND_PROJECT_ORG";
/// Environment variable name for overriding the project repository namespace.
const ENV_PROJECT_REPO: &str = "ITO_BACKEND_PROJECT_REPO";

/// Resolve the project namespace (org, repo) from config with env var fallbacks.
///
/// Resolution order for each field:
/// 1. Explicit config value (`backend.project.org` / `backend.project.repo`)
/// 2. Environment variable (`ITO_BACKEND_PROJECT_ORG` / `ITO_BACKEND_PROJECT_REPO`)
///
/// Returns `Err` if either value is missing after fallback resolution.
fn resolve_project_namespace(config: &BackendApiConfig) -> CoreResult<(String, String)> {
    resolve_project_namespace_with_env(config, ENV_PROJECT_ORG, ENV_PROJECT_REPO)
}

/// Inner implementation that accepts env var names for testability.
fn resolve_project_namespace_with_env(
    config: &BackendApiConfig,
    org_env_var: &str,
    repo_env_var: &str,
) -> CoreResult<(String, String)> {
    let org = config
        .project
        .org
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .or_else(|| {
            std::env::var(org_env_var)
                .ok()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
        });

    let repo = config
        .project
        .repo
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .or_else(|| {
            std::env::var(repo_env_var)
                .ok()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
        });

    let Some(org) = org else {
        return Err(CoreError::validation(format!(
            "Backend mode is enabled but 'backend.project.org' is not set. \
             Set it in config or via the {org_env_var} environment variable."
        )));
    };

    let Some(repo) = repo else {
        return Err(CoreError::validation(format!(
            "Backend mode is enabled but 'backend.project.repo' is not set. \
             Set it in config or via the {repo_env_var} environment variable."
        )));
    };

    Ok((org, repo))
}

/// Determines whether a backend error status code is retriable.
///
/// Returns `true` for server errors (5xx) and rate limiting (429).
/// Client errors (4xx other than 429) are not retriable.
pub fn is_retriable_status(status: u16) -> bool {
    match status {
        429 => true,
        s if s >= 500 => true,
        _ => false,
    }
}

/// Generate a unique idempotency key for a backend operation.
///
/// The key combines a UUID v4 prefix with the operation name for
/// traceability in server logs.
pub fn idempotency_key(operation: &str) -> String {
    format!("{}-{operation}", uuid::Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_config::types::BackendProjectConfig;

    /// Create an enabled config with token, org, and repo pre-populated.
    fn enabled_config() -> BackendApiConfig {
        BackendApiConfig {
            enabled: true,
            token: Some("test-token-123".to_string()),
            project: BackendProjectConfig {
                org: Some("acme".to_string()),
                repo: Some("widgets".to_string()),
            },
            ..BackendApiConfig::default()
        }
    }

    #[test]
    fn disabled_backend_returns_none() {
        let config = BackendApiConfig::default();
        assert!(!config.enabled);

        let result = resolve_backend_runtime(&config).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn enabled_backend_with_explicit_token_resolves() {
        let config = enabled_config();

        let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
        assert_eq!(runtime.token, "test-token-123");
        assert_eq!(runtime.base_url, "http://127.0.0.1:9010");
        assert_eq!(runtime.max_retries, 3);
        assert_eq!(runtime.timeout, Duration::from_millis(30_000));
        assert_eq!(runtime.org, "acme");
        assert_eq!(runtime.repo, "widgets");
    }

    #[test]
    fn enabled_backend_with_env_var_token_resolves() {
        let env_var = "ITO_TEST_BACKEND_TOKEN_1";
        // SAFETY: test-only, single-threaded access to this unique env var.
        unsafe { std::env::set_var(env_var, "env-token-456") };

        let config = BackendApiConfig {
            token: None,
            token_env_var: env_var.to_string(),
            ..enabled_config()
        };

        let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
        assert_eq!(runtime.token, "env-token-456");

        // SAFETY: test-only cleanup.
        unsafe { std::env::remove_var(env_var) };
    }

    #[test]
    fn enabled_backend_missing_token_fails() {
        let env_var = "ITO_TEST_NONEXISTENT_TOKEN_VAR";
        // SAFETY: test-only cleanup of unique env var.
        unsafe { std::env::remove_var(env_var) };

        let config = BackendApiConfig {
            token: None,
            token_env_var: env_var.to_string(),
            ..enabled_config()
        };

        let err = resolve_backend_runtime(&config).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(env_var), "error should mention env var: {msg}");
        assert!(
            msg.contains("not set"),
            "error should mention 'not set': {msg}"
        );
    }

    #[test]
    fn enabled_backend_empty_token_fails() {
        let env_var = "ITO_TEST_EMPTY_TOKEN_VAR";
        // SAFETY: test-only, single-threaded access to this unique env var.
        unsafe { std::env::set_var(env_var, "") };

        let config = BackendApiConfig {
            token: None,
            token_env_var: env_var.to_string(),
            ..enabled_config()
        };

        let err = resolve_backend_runtime(&config).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("empty"), "error should mention 'empty': {msg}");

        // SAFETY: test-only cleanup.
        unsafe { std::env::remove_var(env_var) };
    }

    #[test]
    fn custom_backup_dir_is_used() {
        let config = BackendApiConfig {
            backup_dir: Some("/custom/backups".to_string()),
            ..enabled_config()
        };

        let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
        assert_eq!(runtime.backup_dir, PathBuf::from("/custom/backups"));
    }

    #[test]
    fn default_backup_dir_uses_home() {
        let config = BackendApiConfig {
            backup_dir: None,
            ..enabled_config()
        };

        let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
        // Should end with .ito/backups regardless of the home directory
        let path_str = runtime.backup_dir.to_string_lossy();
        assert!(
            path_str.ends_with(".ito/backups"),
            "unexpected backup dir: {path_str}"
        );
    }

    // ── Project namespace resolution tests ──────────────────────────
    //
    // These tests use `resolve_project_namespace_with_env` with unique
    // env var names per test to avoid parallel-test races on global
    // process environment.

    #[test]
    fn project_namespace_from_config() {
        let config = enabled_config();
        let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
        assert_eq!(runtime.org, "acme");
        assert_eq!(runtime.repo, "widgets");
    }

    #[test]
    fn project_namespace_from_env_vars() {
        let org_var = "ITO_TEST_NS_FROM_ENV_ORG";
        let repo_var = "ITO_TEST_NS_FROM_ENV_REPO";
        // SAFETY: test-only, single-threaded access to unique env vars.
        unsafe {
            std::env::set_var(org_var, "env-org");
            std::env::set_var(repo_var, "env-repo");
        }

        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: None,
                repo: None,
            },
            ..enabled_config()
        };

        let (org, repo) = resolve_project_namespace_with_env(&config, org_var, repo_var).unwrap();
        assert_eq!(org, "env-org");
        assert_eq!(repo, "env-repo");

        // SAFETY: test-only cleanup.
        unsafe {
            std::env::remove_var(org_var);
            std::env::remove_var(repo_var);
        }
    }

    #[test]
    fn project_namespace_config_takes_precedence_over_env() {
        let org_var = "ITO_TEST_NS_PREC_ORG";
        let repo_var = "ITO_TEST_NS_PREC_REPO";
        // SAFETY: test-only, single-threaded access to unique env vars.
        unsafe {
            std::env::set_var(org_var, "env-org");
            std::env::set_var(repo_var, "env-repo");
        }

        let config = enabled_config(); // has org=acme, repo=widgets
        let (org, repo) = resolve_project_namespace_with_env(&config, org_var, repo_var).unwrap();
        assert_eq!(org, "acme");
        assert_eq!(repo, "widgets");

        // SAFETY: test-only cleanup.
        unsafe {
            std::env::remove_var(org_var);
            std::env::remove_var(repo_var);
        }
    }

    #[test]
    fn project_namespace_missing_org_fails() {
        let org_var = "ITO_TEST_NS_MISS_ORG";
        let repo_var = "ITO_TEST_NS_MISS_ORG_REPO";
        // SAFETY: test-only cleanup.
        unsafe {
            std::env::remove_var(org_var);
            std::env::remove_var(repo_var);
        }

        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: None,
                repo: Some("widgets".to_string()),
            },
            ..enabled_config()
        };

        let err = resolve_project_namespace_with_env(&config, org_var, repo_var).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("project.org"),
            "error should mention project.org: {msg}"
        );
    }

    #[test]
    fn project_namespace_missing_repo_fails() {
        let org_var = "ITO_TEST_NS_MISS_REPO_ORG";
        let repo_var = "ITO_TEST_NS_MISS_REPO";
        // SAFETY: test-only cleanup.
        unsafe {
            std::env::remove_var(org_var);
            std::env::remove_var(repo_var);
        }

        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: Some("acme".to_string()),
                repo: None,
            },
            ..enabled_config()
        };

        let err = resolve_project_namespace_with_env(&config, org_var, repo_var).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("project.repo"),
            "error should mention project.repo: {msg}"
        );
    }

    #[test]
    fn project_namespace_empty_string_falls_through_to_env() {
        let org_var = "ITO_TEST_NS_EMPTY_ORG";
        let repo_var = "ITO_TEST_NS_EMPTY_REPO";
        // SAFETY: test-only, single-threaded access to unique env vars.
        unsafe {
            std::env::set_var(org_var, "env-org");
            std::env::set_var(repo_var, "env-repo");
        }

        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: Some("".to_string()),
                repo: Some("".to_string()),
            },
            ..enabled_config()
        };

        let (org, repo) = resolve_project_namespace_with_env(&config, org_var, repo_var).unwrap();
        assert_eq!(org, "env-org");
        assert_eq!(repo, "env-repo");

        // SAFETY: test-only cleanup.
        unsafe {
            std::env::remove_var(org_var);
            std::env::remove_var(repo_var);
        }
    }

    #[test]
    fn project_api_prefix_formats_correctly() {
        let config = enabled_config();
        let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
        assert_eq!(
            runtime.project_api_prefix(),
            "http://127.0.0.1:9010/api/v1/projects/acme/widgets"
        );
    }

    #[test]
    fn is_retriable_status_checks() {
        assert!(is_retriable_status(429));
        assert!(is_retriable_status(500));
        assert!(is_retriable_status(502));
        assert!(is_retriable_status(503));
        assert!(!is_retriable_status(400));
        assert!(!is_retriable_status(401));
        assert!(!is_retriable_status(404));
        assert!(!is_retriable_status(200));
    }

    #[test]
    fn idempotency_key_includes_operation() {
        let key = idempotency_key("push");
        assert!(key.ends_with("-push"));
        assert!(key.len() > 5); // UUID prefix + separator + operation
    }
}
