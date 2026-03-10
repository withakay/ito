//! Handler for the `serve-api` subcommand.
//!
//! Starts a multi-tenant backend API server. Configuration is assembled from
//! CLI flags, environment variables, and the global config file
//! (`~/.config/ito/config.json`).
//!
//! Precedence (highest to lowest): CLI flags > env vars > global config file.
//!
//! Business logic (token resolution, init orchestration, config persistence)
//! lives in [`ito_core::backend_auth`]; this module handles only argument
//! parsing and output formatting.

use crate::cli::ServeApiArgs;
use crate::cli_error::{CliError, CliResult};

use ito_config::types::{
    BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy, BackendServerConfig, ItoConfig,
};
use ito_config::{ConfigContext, load_global_ito_config};
use ito_core::backend_auth::{self, InitAuthResult};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::path::Path;

pub(crate) fn handle_serve_api_clap(
    _rt: &crate::runtime::Runtime,
    args: &ServeApiArgs,
) -> CliResult<()> {
    let ctx = ConfigContext::from_process_env();
    let bootstrap_result = if args.init || args.service {
        Some(backend_auth::init_backend_auth(&ctx).map_err(|e| CliError::msg(e.to_string()))?)
    } else {
        None
    };

    // Handle auth bootstrap before anything else.
    if args.init {
        let result = bootstrap_result.expect("bootstrap result must exist for --init");

        match result {
            InitAuthResult::AlreadyConfigured { config_path } => {
                println!("Backend server auth is already configured.");
                println!();
                println!("  Config file: {config_path}");
                println!();
                println!("To view your tokens, open the config file directly.");
            }
            InitAuthResult::Generated { config_path } => {
                println!("Generated backend server auth tokens.");
                println!();
                println!("  Config file: {config_path}");
                println!();
                println!("Tokens are stored in the config file (not printed for security).");
                println!();
                println!("Start the server with:");
                println!("  ito serve-api");
            }
        }

        return Ok(());
    }

    let mut config = load_backend_server_config(&ctx, args)?;
    let config_auth = &config.auth;

    // Build auth config: CLI flags > env vars > global config file
    let admin_tokens =
        backend_auth::resolve_admin_tokens(&args.admin_token, &config_auth.admin_tokens);
    let token_seed = backend_auth::resolve_token_seed(&args.token_seed, &config_auth.token_seed);

    config.auth = BackendAuthConfig {
        admin_tokens,
        token_seed,
    };

    // Build allowlist from CLI args
    if !args.allow_org.is_empty() {
        let mut repos = BTreeMap::new();
        for org in &args.allow_org {
            repos.insert(org.clone(), BackendRepoPolicy::All("*".to_string()));
        }
        config.allowed = BackendAllowlistConfig {
            orgs: args.allow_org.clone(),
            repos,
        };
    }
    config.enabled = true;
    if let Some(bind) = &args.bind {
        config.bind = bind.clone();
    }
    if let Some(port) = args.port {
        config.port = port;
    }
    if let Some(data_dir) = &args.data_dir {
        config.data_dir = Some(data_dir.clone());
    }

    let tokio_rt = tokio::runtime::Runtime::new()
        .map_err(|e| CliError::msg(format!("Failed to create tokio runtime: {e}")))?;

    tokio_rt.block_on(async {
        ito_backend::serve(config)
            .await
            .map_err(|e| CliError::msg(format!("Server error: {e}")))
    })?;

    Ok(())
}

fn load_backend_server_config(
    ctx: &ConfigContext,
    args: &ServeApiArgs,
) -> CliResult<BackendServerConfig> {
    if let Some(path) = &args.config {
        return load_backend_server_config_file(Path::new(path));
    }

    Ok(load_global_ito_config(ctx).backend_server)
}

fn load_backend_server_config_file(path: &Path) -> CliResult<BackendServerConfig> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| CliError::msg(format!("Failed to read {}: {e}", path.display())))?;

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => load_backend_server_json_config(&contents, path),
        Some("toml") => load_backend_server_toml_config(&contents, path),
        Some(other) => Err(CliError::msg(format!(
            "Unsupported config format for {}: expected .json or .toml, got .{}",
            path.display(),
            other
        ))),
        None => Err(CliError::msg(format!(
            "Unsupported config format for {}: expected a .json or .toml file",
            path.display()
        ))),
    }
}

fn load_backend_server_json_config(contents: &str, path: &Path) -> CliResult<BackendServerConfig> {
    let value: serde_json::Value = serde_json::from_str(contents).map_err(|e| {
        CliError::msg(format!(
            "Invalid backend server config in {}: {e}",
            path.display()
        ))
    })?;

    if value.get("backendServer").is_some() {
        let parsed: ItoConfig = deserialize_json_with_unknown_check(contents, path, "Ito config")?;
        return Ok(parsed.backend_server);
    }

    deserialize_json_with_unknown_check(contents, path, "backend server config")
}

fn load_backend_server_toml_config(contents: &str, path: &Path) -> CliResult<BackendServerConfig> {
    let value: toml::Value = toml::from_str(contents).map_err(|e| {
        CliError::msg(format!(
            "Invalid backend server config in {}: {e}",
            path.display()
        ))
    })?;

    if value.get("backendServer").is_some() {
        let parsed: ItoConfig = deserialize_toml_with_unknown_check(contents, path, "Ito config")?;
        return Ok(parsed.backend_server);
    }

    deserialize_toml_with_unknown_check(contents, path, "backend server config")
}

fn deserialize_json_with_unknown_check<T: DeserializeOwned>(
    contents: &str,
    path: &Path,
    kind: &str,
) -> CliResult<T> {
    let mut ignored = Vec::new();
    let mut deserializer = serde_json::Deserializer::from_str(contents);
    let parsed =
        serde_ignored::deserialize(&mut deserializer, |field| ignored.push(field.to_string()))
            .map_err(|e| CliError::msg(format!("Invalid {kind} in {}: {e}", path.display())))?;
    reject_unknown_fields(path, kind, &ignored)?;
    Ok(parsed)
}

fn deserialize_toml_with_unknown_check<T: DeserializeOwned>(
    contents: &str,
    path: &Path,
    kind: &str,
) -> CliResult<T> {
    let mut ignored = Vec::new();
    let deserializer = toml::Deserializer::new(contents);
    let parsed = serde_ignored::deserialize(deserializer, |field| ignored.push(field.to_string()))
        .map_err(|e| CliError::msg(format!("Invalid {kind} in {}: {e}", path.display())))?;
    reject_unknown_fields(path, kind, &ignored)?;
    Ok(parsed)
}

fn reject_unknown_fields(path: &Path, kind: &str, ignored: &[String]) -> CliResult<()> {
    if ignored.is_empty() {
        return Ok(());
    }

    Err(CliError::msg(format!(
        "Invalid {kind} in {}: unknown field(s): {}",
        path.display(),
        ignored.join(", ")
    )))
}

#[cfg(test)]
mod serve_api_tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn builds_config_with_defaults() {
        let args = ServeApiArgs {
            init: false,
            service: false,
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
    fn load_backend_server_config_file_rejects_unknown_json_fields() {
        let temp = tempdir().unwrap();
        let path = temp.path().join("backend.json");
        std::fs::write(&path, r#"{"server":{"auth":{}}}"#).unwrap();

        let err = load_backend_server_config_file(&path).unwrap_err();
        assert!(err.to_string().contains("unknown field(s): server"));
    }

    #[test]
    fn load_backend_server_config_file_reads_toml() {
        let temp = tempdir().unwrap();
        let path = temp.path().join("backend.toml");
        std::fs::write(
            &path,
            "bind = \"0.0.0.0\"\nport = 9020\n[auth]\nadminTokens = [\"token\"]\n",
        )
        .unwrap();

        let config = load_backend_server_config_file(&path).unwrap();
        assert_eq!(config.bind, "0.0.0.0");
        assert_eq!(config.port, 9020);
        assert_eq!(config.auth.admin_tokens, vec!["token".to_string()]);
    }

    #[test]
    fn load_backend_server_config_file_accepts_full_ito_json_config() {
        let temp = tempdir().unwrap();
        let path = temp.path().join("config.json");
        std::fs::write(
            &path,
            r#"{
                "backendServer": {
                    "bind": "0.0.0.0",
                    "port": 9030,
                    "auth": {
                        "adminTokens": ["token"]
                    }
                },
                "changes": {
                    "coordination_branch": {
                        "enabled": false
                    }
                }
            }"#,
        )
        .unwrap();

        let config = load_backend_server_config_file(&path).unwrap();
        assert_eq!(config.bind, "0.0.0.0");
        assert_eq!(config.port, 9030);
        assert_eq!(config.auth.admin_tokens, vec!["token".to_string()]);
    }
}
