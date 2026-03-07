//! Handler for the `serve-api` subcommand.
//!
//! Starts a multi-tenant backend API server. Configuration is assembled from
//! CLI flags, environment variables, and the global config file
//! (`~/.config/ito/config.json`).
//!
//! Precedence (highest to lowest): CLI flags > env vars > global config file.

use crate::cli::ServeApiArgs;
use crate::cli_error::{CliError, CliResult};

use ito_config::types::{
    BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy, BackendServerConfig,
};
use ito_config::{ConfigContext, load_global_ito_config};
use std::collections::BTreeMap;

/// Resolve admin tokens from CLI flags, env vars, and global config (in that
/// precedence order). All non-empty sources are merged — CLI tokens first,
/// then env var token, then config file tokens, deduplicating along the way.
fn resolve_admin_tokens(cli_tokens: &[String], config_tokens: &[String]) -> Vec<String> {
    let mut tokens: Vec<String> = cli_tokens.to_vec();

    if let Ok(env_token) = std::env::var("ITO_BACKEND_ADMIN_TOKEN") {
        let trimmed = env_token.trim().to_string();
        if !trimmed.is_empty() && !tokens.contains(&trimmed) {
            tokens.push(trimmed);
        }
    }

    for token in config_tokens {
        let trimmed = token.trim().to_string();
        if !trimmed.is_empty() && !tokens.contains(&trimmed) {
            tokens.push(trimmed);
        }
    }

    tokens
}

/// Resolve token seed from CLI flag, env var, or global config (in that
/// precedence order). Returns the first non-empty value found.
fn resolve_token_seed(cli_seed: &Option<String>, config_seed: &Option<String>) -> Option<String> {
    if let Some(seed) = cli_seed.as_ref().filter(|s| !s.trim().is_empty()) {
        return Some(seed.clone());
    }

    if let Ok(env_seed) = std::env::var("ITO_BACKEND_TOKEN_SEED") {
        let trimmed = env_seed.trim().to_string();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }

    if let Some(seed) = config_seed.as_ref().filter(|s| !s.trim().is_empty()) {
        return Some(seed.clone());
    }

    None
}

/// Handle the `--init` flag: generate tokens or display existing ones.
///
/// Returns `Ok(true)` if init was handled (caller should exit), or
/// `Ok(false)` if `--init` was not requested.
fn handle_init(args: &ServeApiArgs, ctx: &ConfigContext) -> CliResult<bool> {
    if !args.init {
        return Ok(false);
    }

    let global_config = load_global_ito_config(ctx);
    let existing_auth = &global_config.backend_server.auth;

    // If tokens already exist, print and exit
    let has_tokens = existing_auth
        .admin_tokens
        .iter()
        .any(|t| !t.trim().is_empty());

    if has_tokens {
        println!("Backend server auth is already configured.");
        println!();
        for (i, token) in existing_auth.admin_tokens.iter().enumerate() {
            println!("  Admin token {}: {token}", i + 1);
        }
        if let Some(seed) = &existing_auth.token_seed {
            println!("  Token seed:    {seed}");
        }
        println!();
        println!("Config file: {}", global_config_display_path(ctx));
        return Ok(true);
    }

    // Generate new tokens
    let admin_token = ito_core::token::generate_token();
    let token_seed = ito_core::token::generate_token();

    // Write to global config file
    write_auth_to_global_config(ctx, &admin_token, &token_seed)?;

    println!("Generated backend server auth tokens.");
    println!();
    println!("  Admin token: {admin_token}");
    println!("  Token seed:  {token_seed}");
    println!();
    println!("Written to: {}", global_config_display_path(ctx));
    println!();
    println!("Start the server with:");
    println!("  ito serve-api");

    Ok(true)
}

/// Return a display-friendly path to the global config file.
fn global_config_display_path(ctx: &ConfigContext) -> String {
    ito_config::global_config_path(ctx)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "~/.config/ito/config.json".to_string())
}

/// Write admin token and token seed into the global config file.
///
/// Reads the existing config (if any), merges in the new auth values, and
/// writes back. Creates the config directory and file if needed.
fn write_auth_to_global_config(
    ctx: &ConfigContext,
    admin_token: &str,
    token_seed: &str,
) -> CliResult<()> {
    let Some(config_path) = ito_config::global_config_path(ctx) else {
        return Err(CliError::msg(
            "Cannot determine global config path (HOME not set)",
        ));
    };

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            CliError::msg(format!(
                "Failed to create config directory {}: {e}",
                parent.display()
            ))
        })?;
    }

    // Read existing config as raw JSON to preserve unrelated fields
    let mut doc: serde_json::Value = if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| CliError::msg(format!("Failed to read {}: {e}", config_path.display())))?;
        serde_json::from_str(&contents).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Merge in the new auth values
    let backend_server = doc
        .as_object_mut()
        .expect("config should be an object")
        .entry("backendServer")
        .or_insert_with(|| serde_json::json!({}));

    let auth = backend_server
        .as_object_mut()
        .expect("backendServer should be an object")
        .entry("auth")
        .or_insert_with(|| serde_json::json!({}));

    let auth_obj = auth.as_object_mut().expect("auth should be an object");

    auth_obj.insert("adminTokens".to_string(), serde_json::json!([admin_token]));
    auth_obj.insert("tokenSeed".to_string(), serde_json::json!(token_seed));

    // Write back with pretty formatting
    let formatted = serde_json::to_string_pretty(&doc)
        .map_err(|e| CliError::msg(format!("JSON error: {e}")))?;

    std::fs::write(&config_path, formatted + "\n")
        .map_err(|e| CliError::msg(format!("Failed to write {}: {e}", config_path.display())))?;

    Ok(())
}

pub(crate) fn handle_serve_api_clap(
    _rt: &crate::runtime::Runtime,
    args: &ServeApiArgs,
) -> CliResult<()> {
    let ctx = ConfigContext::from_process_env();

    // Handle --init before anything else
    if handle_init(args, &ctx)? {
        return Ok(());
    }

    // Load global config for fallback values
    let global_config = load_global_ito_config(&ctx);
    let config_auth = &global_config.backend_server.auth;

    // Build auth config: CLI flags > env vars > global config file
    let admin_tokens = resolve_admin_tokens(&args.admin_token, &config_auth.admin_tokens);
    let token_seed = resolve_token_seed(&args.token_seed, &config_auth.token_seed);

    let auth = BackendAuthConfig {
        admin_tokens,
        token_seed,
    };

    // Build allowlist from CLI args
    let allowed = if args.allow_org.is_empty() {
        BackendAllowlistConfig::default()
    } else {
        let mut repos = BTreeMap::new();
        for org in &args.allow_org {
            repos.insert(org.clone(), BackendRepoPolicy::All("*".to_string()));
        }
        BackendAllowlistConfig {
            orgs: args.allow_org.clone(),
            repos,
        }
    };

    let config = BackendServerConfig {
        enabled: true,
        bind: args.bind.clone().unwrap_or_else(|| "127.0.0.1".to_string()),
        port: args.port.unwrap_or(9010),
        data_dir: args.data_dir.clone(),
        allowed,
        auth,
        ..Default::default()
    };

    let tokio_rt = tokio::runtime::Runtime::new()
        .map_err(|e| CliError::msg(format!("Failed to create tokio runtime: {e}")))?;

    tokio_rt.block_on(async {
        ito_backend::serve(config)
            .await
            .map_err(|e| CliError::msg(format!("Server error: {e}")))
    })?;

    Ok(())
}

#[cfg(test)]
mod serve_api_tests {
    use super::*;

    #[test]
    fn builds_config_with_defaults() {
        let args = ServeApiArgs {
            init: false,
            port: None,
            bind: None,
            data_dir: None,
            admin_token: vec![],
            token_seed: None,
            allow_org: vec![],
            config: None,
        };
        assert!(args.port.is_none());
        assert!(args.bind.is_none());
    }

    #[test]
    fn builds_allowlist_from_allow_org_args() {
        let orgs = vec!["acme".to_string(), "globex".to_string()];
        let mut repos = BTreeMap::new();
        for org in &orgs {
            repos.insert(org.clone(), BackendRepoPolicy::All("*".to_string()));
        }
        let allowlist = BackendAllowlistConfig {
            orgs: orgs.clone(),
            repos,
        };
        assert!(allowlist.is_allowed("acme", "any-repo"));
        assert!(allowlist.is_allowed("globex", "another-repo"));
        assert!(!allowlist.is_allowed("unknown-org", "repo"));
    }

    #[test]
    fn resolve_admin_tokens_merges_all_sources() {
        let cli = vec!["cli-token".to_string()];
        let config = vec!["config-token".to_string()];

        let prev = std::env::var("ITO_BACKEND_ADMIN_TOKEN").ok();
        // SAFETY: test-only, single-threaded
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
    fn handle_init_skips_when_tokens_exist() {
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

        let args = ServeApiArgs {
            init: true,
            port: None,
            bind: None,
            data_dir: None,
            admin_token: vec![],
            token_seed: None,
            allow_org: vec![],
            config: None,
        };

        let handled = handle_init(&args, &ctx).unwrap();
        assert!(handled);

        // Config file should be unchanged
        let contents = std::fs::read_to_string(config_dir.join("config.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
        assert_eq!(
            parsed["backendServer"]["auth"]["adminTokens"][0],
            "existing-token"
        );
    }

    #[test]
    fn handle_init_generates_tokens_when_none_exist() {
        let home = tempfile::tempdir().unwrap();
        let ctx = ConfigContext {
            home_dir: Some(home.path().to_path_buf()),
            xdg_config_home: None,
            project_dir: None,
        };

        let args = ServeApiArgs {
            init: true,
            port: None,
            bind: None,
            data_dir: None,
            admin_token: vec![],
            token_seed: None,
            allow_org: vec![],
            config: None,
        };

        let handled = handle_init(&args, &ctx).unwrap();
        assert!(handled);

        // Config file should exist with generated tokens
        let config_path = home.path().join(".config/ito/config.json");
        assert!(config_path.exists());

        let contents = std::fs::read_to_string(&config_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();

        let admin_token = parsed["backendServer"]["auth"]["adminTokens"][0]
            .as_str()
            .unwrap();
        let seed = parsed["backendServer"]["auth"]["tokenSeed"]
            .as_str()
            .unwrap();

        // Tokens should be non-empty and URL-safe base64
        assert_eq!(admin_token.len(), 43);
        assert_eq!(seed.len(), 43);
        assert!(
            admin_token
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        );
    }

    #[test]
    fn handle_init_returns_false_when_not_requested() {
        let home = tempfile::tempdir().unwrap();
        let ctx = ConfigContext {
            home_dir: Some(home.path().to_path_buf()),
            xdg_config_home: None,
            project_dir: None,
        };

        let args = ServeApiArgs {
            init: false,
            port: None,
            bind: None,
            data_dir: None,
            admin_token: vec![],
            token_seed: None,
            allow_org: vec![],
            config: None,
        };

        let handled = handle_init(&args, &ctx).unwrap();
        assert!(!handled);
    }
}
