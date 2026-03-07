//! Backend server authentication setup and token resolution.
//!
//! This module owns the precedence logic for assembling auth credentials
//! (CLI flags > environment variables > global config file) and the `--init`
//! orchestration that generates and persists tokens on first run.

use std::path::Path;

use ito_config::{global_config_path, load_global_ito_config, ConfigContext};

use crate::errors::{CoreError, CoreResult};

// ---------------------------------------------------------------------------
// Init result
// ---------------------------------------------------------------------------

/// Outcome of [`init_backend_auth`].
#[derive(Debug)]
pub enum InitAuthResult {
    /// Auth was already configured — no changes were made.
    AlreadyConfigured {
        /// Display-friendly path to the global config file.
        config_path: String,
    },
    /// New tokens were generated and persisted.
    Generated {
        /// Display-friendly path to the global config file.
        config_path: String,
    },
}

// ---------------------------------------------------------------------------
// Init orchestration
// ---------------------------------------------------------------------------

/// Check for existing backend auth in the global config; if absent, generate
/// new tokens and persist them.
///
/// Returns an [`InitAuthResult`] describing what happened.
pub fn init_backend_auth(ctx: &ConfigContext) -> CoreResult<InitAuthResult> {
    let global_config = load_global_ito_config(ctx);
    let existing_auth = &global_config.backend_server.auth;

    let has_tokens = existing_auth
        .admin_tokens
        .iter()
        .any(|t| !t.trim().is_empty());

    let config_display = global_config_display_path(ctx);

    if has_tokens {
        return Ok(InitAuthResult::AlreadyConfigured {
            config_path: config_display,
        });
    }

    let admin_token = crate::token::generate_token();
    let token_seed = crate::token::generate_token();

    write_auth_to_global_config(ctx, &admin_token, &token_seed)?;

    Ok(InitAuthResult::Generated {
        config_path: config_display,
    })
}

// ---------------------------------------------------------------------------
// Token resolution (CLI > env > config)
// ---------------------------------------------------------------------------

/// Resolve admin tokens by merging CLI flags, the `ITO_BACKEND_ADMIN_TOKEN`
/// env var, and global config entries — in that precedence order.
///
/// All non-empty sources contribute; duplicates are removed.
pub fn resolve_admin_tokens(cli_tokens: &[String], config_tokens: &[String]) -> Vec<String> {
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

/// Resolve the HMAC token seed from CLI flag, the `ITO_BACKEND_TOKEN_SEED`
/// env var, or global config — returning the first non-empty value found.
pub fn resolve_token_seed(
    cli_seed: &Option<String>,
    config_seed: &Option<String>,
) -> Option<String> {
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

// ---------------------------------------------------------------------------
// Config persistence
// ---------------------------------------------------------------------------

/// Return a display-friendly path to the global config file.
pub fn global_config_display_path(ctx: &ConfigContext) -> String {
    global_config_path(ctx)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "~/.config/ito/config.json".to_string())
}

/// Write admin token and token seed into the global config file.
///
/// Reads the existing config (if any), merges in the new auth values, and
/// writes back. Creates the config directory and file if needed, with
/// restrictive permissions (directory 0700, file 0600 on Unix).
pub fn write_auth_to_global_config(
    ctx: &ConfigContext,
    admin_token: &str,
    token_seed: &str,
) -> CoreResult<()> {
    let Some(config_path) = global_config_path(ctx) else {
        return Err(CoreError::validation(
            "Cannot determine global config path (HOME not set)",
        ));
    };

    // Ensure parent directory exists with restrictive permissions (0700)
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| CoreError::io(format!("create config dir {}", parent.display()), e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700)).map_err(
                |e| CoreError::io(format!("set permissions on {}", parent.display()), e),
            )?;
        }
    }

    // Read existing config as raw JSON to preserve unrelated fields
    let mut doc: serde_json::Value = if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| CoreError::io(format!("read {}", config_path.display()), e))?;
        serde_json::from_str(&contents).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Merge in the new auth values
    let root = doc.as_object_mut().ok_or_else(|| {
        CoreError::validation(format!(
            "{} is not a JSON object; delete it and re-run --init",
            config_path.display()
        ))
    })?;

    let backend_server = root
        .entry("backendServer")
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .ok_or_else(|| CoreError::validation("config key 'backendServer' must be a JSON object"))?;

    let auth_obj = backend_server
        .entry("auth")
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .ok_or_else(|| {
            CoreError::validation("config key 'backendServer.auth' must be a JSON object")
        })?;

    auth_obj.insert("adminTokens".to_string(), serde_json::json!([admin_token]));
    auth_obj.insert("tokenSeed".to_string(), serde_json::json!(token_seed));

    // Write back with pretty formatting and restrictive permissions (0600)
    let formatted = serde_json::to_string_pretty(&doc)
        .map_err(|e| CoreError::serde("serialize global config", e.to_string()))?;

    write_config_file(&config_path, &(formatted + "\n"))?;

    Ok(())
}

/// Write content to a file with restrictive permissions (0600 on Unix).
///
/// On non-Unix platforms, falls back to `std::fs::write` which uses the
/// process umask.
fn write_config_file(path: &Path, content: &str) -> CoreResult<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)
            .map_err(|e| CoreError::io(format!("write {}", path.display()), e))?;

        file.write_all(content.as_bytes())
            .map_err(|e| CoreError::io(format!("write {}", path.display()), e))?;
    }

    #[cfg(not(unix))]
    {
        std::fs::write(path, content)
            .map_err(|e| CoreError::io(format!("write {}", path.display()), e))?;
    }

    Ok(())
}
