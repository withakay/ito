//! HTTP server bootstrap and route assembly for the multi-tenant backend.
//!
//! [`serve`] is the single entry point: it wires up the v1 API router,
//! authentication middleware, and CORS, then binds to the configured address.
//! All business logic is in `ito-core`; this module handles transport concerns.

use axum::{Router, middleware};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use ito_config::types::BackendServerConfig;

use crate::api;
use crate::auth;
use crate::state::AppState;

/// Resolve the data directory from the server config.
///
/// Priority: explicit `data_dir` in config → `$XDG_DATA_HOME/ito/backend`
/// → `$HOME/.local/share/ito/backend`.
fn resolve_data_dir(config: &BackendServerConfig) -> miette::Result<PathBuf> {
    if let Some(dir) = &config.data_dir {
        return Ok(PathBuf::from(dir));
    }

    // XDG_DATA_HOME fallback
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg).join("ito").join("backend"));
    }

    // $HOME fallback
    let home = std::env::var("HOME").map_err(|_| {
        miette::miette!(
            "Cannot determine data directory: neither data_dir, XDG_DATA_HOME, nor HOME is set"
        )
    })?;

    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("ito")
        .join("backend"))
}

/// Start the multi-tenant backend API server and block until it shuts down.
///
/// Assembles routes, auth middleware, and CORS, then binds to the configured
/// address. Prints the listening address and auth info to stderr on startup.
pub async fn serve(config: BackendServerConfig) -> miette::Result<()> {
    let data_dir = resolve_data_dir(&config)?;

    // Ensure data directory exists
    std::fs::create_dir_all(&data_dir).map_err(|e| {
        miette::miette!(
            "Failed to create data directory {}: {e}",
            data_dir.display()
        )
    })?;

    let data_dir = data_dir.canonicalize().unwrap_or(data_dir);

    let app_state = Arc::new(AppState::new(
        data_dir.clone(),
        config.allowed.clone(),
        config.auth.clone(),
    ));

    // Build CORS layer
    let cors = match &config.cors.origins {
        Some(origins) => {
            let mut layer = CorsLayer::new();
            for origin in origins {
                let Ok(header_val) = origin.parse::<axum::http::HeaderValue>() else {
                    eprintln!("warning: invalid CORS origin skipped: {origin}");
                    continue;
                };
                layer = layer.allow_origin(header_val);
            }
            layer
        }
        None => CorsLayer::permissive(),
    };

    let app = Router::new()
        .nest("/api/v1", api::v1_router())
        .with_state(app_state.clone())
        .layer(middleware::from_fn_with_state(
            app_state,
            auth::auth_middleware,
        ))
        .layer(cors);

    let addr: SocketAddr = format!("{}:{}", config.bind, config.port)
        .parse()
        .map_err(|e| miette::miette!("Invalid address: {e}"))?;

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| miette::miette!("Failed to bind to {addr}: {e}"))?;

    let admin_count = config.auth.admin_tokens.len();
    let has_seed = config.auth.token_seed.is_some();
    let allowed_orgs = config.allowed.orgs.len();

    eprintln!("ito-backend (multi-tenant) listening at http://{addr}/");
    eprintln!("  data_dir: {}", data_dir.display());
    eprintln!("  admin_tokens: {admin_count}, token_seed: {has_seed}");
    eprintln!("  allowed orgs: {allowed_orgs}");

    axum::serve(listener, app)
        .await
        .map_err(|e| miette::miette!("Server error: {e}"))?;

    Ok(())
}
