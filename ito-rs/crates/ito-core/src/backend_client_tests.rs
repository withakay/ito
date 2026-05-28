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
fn env_var_token_takes_precedence_over_config_token() {
    let env_var = "ITO_TEST_BACKEND_TOKEN_PREC";
    // SAFETY: test-only, single-threaded access to this unique env var.
    unsafe { std::env::set_var(env_var, "env-token-override") };

    let config = BackendApiConfig {
        token: Some("config-token".to_string()),
        token_env_var: env_var.to_string(),
        ..enabled_config()
    };

    let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
    assert_eq!(runtime.token, "env-token-override");

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
fn project_namespace_env_takes_precedence_over_config() {
    let org_var = "ITO_TEST_NS_PREC_ORG";
    let repo_var = "ITO_TEST_NS_PREC_REPO";
    // SAFETY: test-only, single-threaded access to unique env vars.
    unsafe {
        std::env::set_var(org_var, "env-org");
        std::env::set_var(repo_var, "env-repo");
    }

    let config = enabled_config(); // has org=acme, repo=widgets
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
