//! Handler for `ito backend` subcommands.

use crate::cli::{BackendAction, BackendArgs};
use crate::cli_error::{CliError, CliResult};
use crate::runtime::Runtime;
use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;
use ito_core::backend_client::resolve_backend_runtime;
use ito_core::backend_health::check_backend_health;
use ito_core::backend_http::BackendHttpClient;
use ito_core::backend_import::{RepositoryBackedImportSink, import_local_changes_with_options};
use std::path::Path;

/// Dispatch `ito backend` subcommands.
pub fn handle_backend_clap(rt: &Runtime, args: &BackendArgs) -> CliResult<()> {
    match &args.action {
        BackendAction::Status { json } => handle_status(rt, *json),
        BackendAction::Import { dry_run } => handle_import(rt, *dry_run),
        BackendAction::GenerateToken { seed, org, repo } => {
            handle_generate_token(rt, seed.clone(), org.clone(), repo.clone())
        }
    }
}

fn handle_import(rt: &Runtime, dry_run: bool) -> CliResult<()> {
    let config = load_project_config(rt)?;
    let runtime = resolve_backend_runtime(&config.backend)
        .map_err(|e| CliError::msg(format!("Invalid backend config: {e}")))?
        .ok_or_else(|| CliError::msg("backend mode is required for `ito backend import`"))?;

    let client = BackendHttpClient::new(runtime);
    let sink = RepositoryBackedImportSink::new(&client, &client, &client);
    let summary = import_local_changes_with_options(&sink, rt.ito_path(), dry_run)
        .map_err(|e| CliError::msg(format!("Backend import failed: {e}")))?;

    if dry_run {
        println!(
            "Would import {} change(s); skip {} and fail {} during preview.",
            summary.previewed, summary.skipped, summary.failed
        );
    } else {
        println!(
            "Imported {} change(s); skipped {}; failed {}.",
            summary.imported, summary.skipped, summary.failed
        );
    }

    if summary.failed > 0 {
        return Err(CliError::silent());
    }
    Ok(())
}

fn load_project_config(rt: &Runtime) -> CliResult<ItoConfig> {
    let ito_path = rt.ito_path();
    let project_root = resolve_project_root(ito_path)?;
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    serde_json::from_value(merged)
        .map_err(|e| CliError::msg(format!("Invalid merged Ito config: {e}")))
}

fn resolve_project_root(ito_path: &Path) -> CliResult<&Path> {
    ito_path.parent().ok_or_else(|| {
        CliError::msg(format!(
            "Invalid Ito root without parent directory: {}",
            ito_path.display()
        ))
    })
}

fn handle_status(rt: &Runtime, json: bool) -> CliResult<()> {
    let config = load_project_config(rt)?;

    // Check if backend is enabled
    if !config.backend.enabled {
        if json {
            let output = serde_json::json!({
                "enabled": false,
                "url": config.backend.url,
                "config_valid": true,
                "config_errors": [],
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        } else {
            println!("Backend Status");
            println!("──────────────");
            println!("Enabled:        false");
            println!();
            println!(
                "Backend mode is disabled. Set 'backend.enabled=true' in your Ito config to enable."
            );
        }
        return Ok(());
    }

    // Try to resolve BackendRuntime
    let runtime = match resolve_backend_runtime(&config.backend) {
        Ok(Some(runtime)) => Ok(runtime),
        Ok(None) => {
            // This shouldn't happen since we checked enabled=true above
            Err(CliError::msg(
                "Backend mode is enabled but runtime could not be resolved.",
            ))
        }
        Err(e) => {
            // Config validation failed
            let error_msg = e.to_string();
            if json {
                let output = serde_json::json!({
                    "enabled": true,
                    "url": config.backend.url,
                    "config_valid": false,
                    "config_errors": [error_msg],
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            } else {
                println!("Backend Status");
                println!("──────────────");
                println!("Enabled:        true");
                println!("URL:            {}", config.backend.url);
                println!("Config:         invalid");
                println!();
                eprintln!("Configuration error: {error_msg}");
            }
            Err(CliError::silent())
        }
    }?;

    // Check for token security warning
    if config.backend.token.is_some() {
        eprintln!("Warning: Backend token is set directly in config file.");
        eprintln!(
            "         Consider using the {} environment variable or .ito/config.local.json instead.",
            config.backend.token_env_var
        );
        eprintln!();
    }

    // Check backend health
    let health = check_backend_health(&runtime);

    // Determine overall status
    let is_healthy = health.server_reachable
        && health.server_healthy
        && health.server_ready
        && health.auth_verified;

    if json {
        // JSON output
        let output = serde_json::json!({
            "enabled": true,
            "url": runtime.base_url,
            "config_valid": true,
            "config_errors": [],
            "server_reachable": health.server_reachable,
            "server_healthy": health.server_healthy,
            "server_ready": health.server_ready,
            "server_version": health.server_version,
            "ready_reason": health.ready_reason,
            "auth_verified": health.auth_verified,
            "token_scope": health.token_scope,
            "error": health.error,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        // Human-readable output
        println!("Backend Status");
        println!("──────────────");
        println!("Enabled:        true");
        println!("URL:            {}", runtime.base_url);
        println!("Config:         valid");
        println!(
            "Server:         {}",
            if health.server_reachable {
                "reachable"
            } else {
                "unreachable"
            }
        );

        if let Some(version) = &health.server_version {
            println!(
                "Health:         {} (v{})",
                if health.server_healthy {
                    "ok"
                } else {
                    "unhealthy"
                },
                version
            );
        } else {
            println!(
                "Health:         {}",
                if health.server_healthy {
                    "ok"
                } else {
                    "unhealthy"
                }
            );
        }

        if health.server_ready {
            println!("Ready:          ready");
        } else if let Some(reason) = &health.ready_reason {
            println!("Ready:          not ready ({})", reason);
        } else {
            println!("Ready:          not ready");
        }

        if health.auth_verified {
            if let Some(scope) = &health.token_scope {
                println!("Auth:           verified (scope: {})", scope);
            } else {
                println!("Auth:           verified");
            }
        } else {
            println!("Auth:           failed");
        }

        if let Some(error) = &health.error {
            println!();
            eprintln!("Error: {error}");
        }
    }

    // Exit with appropriate code
    if is_healthy {
        Ok(())
    } else {
        Err(CliError::silent())
    }
}

fn handle_generate_token(
    rt: &Runtime,
    seed_flag: Option<String>,
    org_flag: Option<String>,
    repo_flag: Option<String>,
) -> CliResult<()> {
    // ── Step 1: Resolve seed (env > flag > config) ──
    let seed = resolve_seed(rt, &seed_flag)?;

    // ── Step 2: Resolve org/repo (env > flag > config > interactive) ──
    let (org, repo) = resolve_org_repo(rt, &org_flag, &repo_flag)?;

    // ── Step 3: Derive token ──
    let token = ito_backend::derive_project_token(&seed, &org, &repo);

    // ── Step 4: Output ──
    // Print token to stdout (for piping/capturing)
    println!("{}", token);

    // Print guidance to stderr
    eprintln!();
    eprintln!("Token derived for: {}/{}", org, repo);
    eprintln!();
    eprintln!("To use this token, set it as an environment variable:");
    eprintln!("  export ITO_BACKEND_TOKEN={}", token);
    eprintln!();
    eprintln!("Or add to .ito/config.local.json (gitignored):");
    eprintln!("  {{\"backend\": {{\"token\": \"{}\"}}}}", token);
    eprintln!();

    Ok(())
}

/// Resolve the HMAC seed from env > flag > global config.
fn resolve_seed(rt: &Runtime, seed_flag: &Option<String>) -> CliResult<String> {
    // Check env var first
    if let Ok(env_seed) = std::env::var("ITO_BACKEND_TOKEN_SEED") {
        let trimmed = env_seed.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    // Check flag
    if let Some(seed) = seed_flag.as_ref().filter(|s| !s.trim().is_empty()) {
        return Ok(seed.clone());
    }

    // Check global config
    let global_config = ito_config::load_global_ito_config(rt.ctx());
    if let Some(seed) = &global_config.backend_server.auth.token_seed {
        let trimmed = seed.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    // No seed found
    Err(CliError::msg(
        "No token seed configured. Run 'ito serve-api --init' to generate one, \
         or set ITO_BACKEND_TOKEN_SEED environment variable.",
    ))
}

/// Resolve org and repo from env > flag > config > interactive prompt.
fn resolve_org_repo(
    rt: &Runtime,
    org_flag: &Option<String>,
    repo_flag: &Option<String>,
) -> CliResult<(String, String)> {
    const ENV_PROJECT_ORG: &str = "ITO_BACKEND_PROJECT_ORG";
    const ENV_PROJECT_REPO: &str = "ITO_BACKEND_PROJECT_REPO";

    // Load project config for fallback values
    let config = load_project_config(rt)?;

    // Resolve org: env > flag > config
    let mut org = std::env::var(ENV_PROJECT_ORG)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| org_flag.clone())
        .or_else(|| config.backend.project.org.clone().filter(|s| !s.is_empty()));

    // Resolve repo: env > flag > config
    let mut repo = std::env::var(ENV_PROJECT_REPO)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| repo_flag.clone())
        .or_else(|| {
            config
                .backend
                .project
                .repo
                .clone()
                .filter(|s| !s.is_empty())
        });

    // Track if we prompted interactively
    let mut prompted = false;

    // Prompt for missing values
    if org.is_none() {
        org = Some(prompt_for_value("Organization (e.g., 'acme')")?);
        prompted = true;
    }

    if repo.is_none() {
        repo = Some(prompt_for_value("Repository (e.g., 'widgets')")?);
        prompted = true;
    }

    // Print tip if we prompted
    if prompted {
        eprintln!();
        eprintln!("Tip: Set 'backend.project.org' and 'backend.project.repo' in your");
        eprintln!("     project config (.ito/config.json) to skip this prompt.");
        eprintln!();
    }

    Ok((org.unwrap(), repo.unwrap()))
}

/// Prompt the user for a value interactively.
fn prompt_for_value(prompt: &str) -> CliResult<String> {
    use dialoguer::Input;

    let value: String = Input::new()
        .with_prompt(prompt)
        .interact_text()
        .map_err(|e| CliError::msg(format!("Failed to read input: {}", e)))?;

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(CliError::msg("Value cannot be empty"));
    }

    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::resolve_project_root;
    use std::path::Path;

    #[test]
    fn resolve_project_root_returns_parent_directory() {
        let root = resolve_project_root(Path::new("/tmp/project/.ito")).unwrap();
        assert_eq!(root, Path::new("/tmp/project"));
    }

    #[test]
    fn resolve_project_root_rejects_parentless_paths() {
        let err = resolve_project_root(Path::new("/")).unwrap_err();
        assert!(
            err.to_string()
                .contains("Invalid Ito root without parent directory")
        );
    }
}
