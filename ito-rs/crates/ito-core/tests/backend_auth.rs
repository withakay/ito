//! Tests for `ito_core::backend_auth` — token resolution, config persistence,
//! and `--init` orchestration.

use ito_config::ConfigContext;
use ito_core::backend_auth::{
    InitAuthResult, init_backend_auth, resolve_admin_tokens, resolve_token_seed,
    write_auth_to_global_config,
};

#[test]
fn resolve_admin_tokens_merges_all_sources() {
    let cli = vec!["cli-token".to_string()];
    let config = vec!["config-token".to_string()];

    let prev = std::env::var("ITO_BACKEND_ADMIN_TOKEN").ok();
    unsafe { std::env::remove_var("ITO_BACKEND_ADMIN_TOKEN") };

    let result = resolve_admin_tokens(&cli, &config);
    assert_eq!(result, vec!["cli-token", "config-token"]);

    if let Some(v) = prev {
        unsafe { std::env::set_var("ITO_BACKEND_ADMIN_TOKEN", v) };
    }
}

#[test]
fn resolve_admin_tokens_deduplicates() {
    let cli = vec!["same-token".to_string()];
    let config = vec!["same-token".to_string()];

    let prev = std::env::var("ITO_BACKEND_ADMIN_TOKEN").ok();
    unsafe { std::env::remove_var("ITO_BACKEND_ADMIN_TOKEN") };

    let result = resolve_admin_tokens(&cli, &config);
    assert_eq!(result, vec!["same-token"]);

    if let Some(v) = prev {
        unsafe { std::env::set_var("ITO_BACKEND_ADMIN_TOKEN", v) };
    }
}

#[test]
fn resolve_admin_tokens_skips_empty_config_entries() {
    let cli: Vec<String> = vec![];
    let config = vec!["".to_string(), "  ".to_string(), "valid".to_string()];

    let prev = std::env::var("ITO_BACKEND_ADMIN_TOKEN").ok();
    unsafe { std::env::remove_var("ITO_BACKEND_ADMIN_TOKEN") };

    let result = resolve_admin_tokens(&cli, &config);
    assert_eq!(result, vec!["valid"]);

    if let Some(v) = prev {
        unsafe { std::env::set_var("ITO_BACKEND_ADMIN_TOKEN", v) };
    }
}

#[test]
fn resolve_token_seed_cli_takes_precedence() {
    let cli = Some("cli-seed".to_string());
    let config = Some("config-seed".to_string());
    assert_eq!(
        resolve_token_seed(&cli, &config),
        Some("cli-seed".to_string())
    );
}

#[test]
fn resolve_token_seed_falls_back_to_config() {
    let prev = std::env::var("ITO_BACKEND_TOKEN_SEED").ok();
    unsafe { std::env::remove_var("ITO_BACKEND_TOKEN_SEED") };

    let result = resolve_token_seed(&None, &Some("config-seed".to_string()));
    assert_eq!(result, Some("config-seed".to_string()));

    if let Some(v) = prev {
        unsafe { std::env::set_var("ITO_BACKEND_TOKEN_SEED", v) };
    }
}

#[test]
fn resolve_token_seed_returns_none_when_all_empty() {
    let prev = std::env::var("ITO_BACKEND_TOKEN_SEED").ok();
    unsafe { std::env::remove_var("ITO_BACKEND_TOKEN_SEED") };

    let result = resolve_token_seed(&None, &None);
    assert_eq!(result, None);

    if let Some(v) = prev {
        unsafe { std::env::set_var("ITO_BACKEND_TOKEN_SEED", v) };
    }
}

#[test]
fn write_auth_creates_config_file() {
    let home = tempfile::tempdir().unwrap();
    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    write_auth_to_global_config(&ctx, "my-admin-token", "my-seed").unwrap();

    let config_path = home.path().join(".config/ito/config.json");
    assert!(config_path.exists());

    let contents = std::fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let auth = &parsed["backendServer"]["auth"];
    assert_eq!(auth["adminTokens"][0], "my-admin-token");
    assert_eq!(auth["tokenSeed"], "my-seed");
}

#[test]
fn write_auth_preserves_existing_config() {
    let home = tempfile::tempdir().unwrap();
    let config_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Write pre-existing config with unrelated field
    std::fs::write(
        config_dir.join("config.json"),
        r#"{"projectPath": ".ito", "worktrees": {"enabled": true}}"#,
    )
    .unwrap();

    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    write_auth_to_global_config(&ctx, "new-token", "new-seed").unwrap();

    let contents = std::fs::read_to_string(config_dir.join("config.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();

    // Auth was written
    assert_eq!(
        parsed["backendServer"]["auth"]["adminTokens"][0],
        "new-token"
    );
    assert_eq!(parsed["backendServer"]["auth"]["tokenSeed"], "new-seed");

    // Pre-existing fields preserved
    assert_eq!(parsed["projectPath"], ".ito");
    assert_eq!(parsed["worktrees"]["enabled"], true);
}

#[test]
fn init_generates_tokens_when_none_exist() {
    let home = tempfile::tempdir().unwrap();
    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let result = init_backend_auth(&ctx).unwrap();
    let InitAuthResult::Generated { .. } = result else {
        panic!("expected Generated variant");
    };

    // Config file should exist with generated tokens
    let file_path = home.path().join(".config/ito/config.json");
    assert!(file_path.exists());

    let contents = std::fs::read_to_string(&file_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let admin_token = parsed["backendServer"]["auth"]["adminTokens"][0]
        .as_str()
        .unwrap();
    let seed = parsed["backendServer"]["auth"]["tokenSeed"]
        .as_str()
        .unwrap();

    assert_eq!(admin_token.len(), 43);
    assert_eq!(seed.len(), 43);
    assert!(
        admin_token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    );
}

#[test]
fn init_skips_when_tokens_exist() {
    let home = tempfile::tempdir().unwrap();
    let config_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&config_dir).unwrap();

    std::fs::write(
        config_dir.join("config.json"),
        r#"{"backendServer": {"auth": {"adminTokens": ["existing-token"]}}}"#,
    )
    .unwrap();

    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let result = init_backend_auth(&ctx).unwrap();
    let InitAuthResult::AlreadyConfigured { .. } = result else {
        panic!("expected AlreadyConfigured variant");
    };

    // Config file should be unchanged
    let contents = std::fs::read_to_string(config_dir.join("config.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(
        parsed["backendServer"]["auth"]["adminTokens"][0],
        "existing-token"
    );
}

#[test]
fn write_auth_rejects_non_object_root() {
    let home = tempfile::tempdir().unwrap();
    let config_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.json"), "42").unwrap();

    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let err = write_auth_to_global_config(&ctx, "t", "s").unwrap_err();
    assert!(
        err.to_string().contains("not a JSON object"),
        "unexpected error: {err}"
    );
}

#[test]
fn write_auth_rejects_non_object_backend_server() {
    let home = tempfile::tempdir().unwrap();
    let config_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("config.json"),
        r#"{"backendServer": "not-an-object"}"#,
    )
    .unwrap();

    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let err = write_auth_to_global_config(&ctx, "t", "s").unwrap_err();
    assert!(
        err.to_string()
            .contains("'backendServer' must be a JSON object"),
        "unexpected error: {err}"
    );
}

#[cfg(unix)]
#[test]
fn write_auth_sets_restrictive_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let home = tempfile::tempdir().unwrap();
    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    write_auth_to_global_config(&ctx, "tok", "seed").unwrap();

    let config_path = home.path().join(".config/ito/config.json");
    let file_mode = std::fs::metadata(&config_path)
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(file_mode, 0o600, "config file should be 0600");

    let dir_mode = std::fs::metadata(config_path.parent().unwrap())
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(dir_mode, 0o700, "config dir should be 0700");
}
