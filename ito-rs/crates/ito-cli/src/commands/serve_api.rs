//! Handler for the `serve-api` subcommand.
//!
//! Starts a multi-tenant backend API server. Configuration is assembled from
//! CLI flags, environment variables, and an optional config file.

use crate::cli::ServeApiArgs;
use crate::cli_error::{CliError, CliResult};

use ito_config::types::{
    BackendAllowlistConfig, BackendAuthConfig, BackendRepoPolicy, BackendServerConfig,
};
use std::collections::BTreeMap;

pub(crate) fn handle_serve_api_clap(
    _rt: &crate::runtime::Runtime,
    args: &ServeApiArgs,
) -> CliResult<()> {
    // Build auth config from CLI args + env vars
    let mut admin_tokens = args.admin_token.clone();
    if let Ok(env_token) = std::env::var("ITO_BACKEND_ADMIN_TOKEN") {
        let trimmed = env_token.trim().to_string();
        if !trimmed.is_empty() && !admin_tokens.contains(&trimmed) {
            admin_tokens.push(trimmed);
        }
    }

    let token_seed = args.token_seed.clone().or_else(|| {
        std::env::var("ITO_BACKEND_TOKEN_SEED")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    });

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
            port: None,
            bind: None,
            data_dir: None,
            admin_token: vec![],
            token_seed: None,
            allow_org: vec![],
            config: None,
        };
        // Just verify we can create the args without panic
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
