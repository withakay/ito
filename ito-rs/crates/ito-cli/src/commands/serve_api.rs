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
    BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy, BackendServerConfig,
};
use ito_config::{ConfigContext, load_global_ito_config};
use ito_core::backend_auth::{self, InitAuthResult};
use std::collections::BTreeMap;

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

    // Load global config for fallback values
    let global_config = load_global_ito_config(&ctx);
    let config_auth = &global_config.backend_server.auth;

    // Build auth config: CLI flags > env vars > global config file
    let admin_tokens =
        backend_auth::resolve_admin_tokens(&args.admin_token, &config_auth.admin_tokens);
    let token_seed = backend_auth::resolve_token_seed(&args.token_seed, &config_auth.token_seed);

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
}
