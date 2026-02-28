//! HTTP server bootstrap and route assembly.
//!
//! [`serve`] is the single entry point: it wires up the v1 API router,
//! authentication middleware, and CORS, then binds to the configured address.
//! All business logic is in `ito-core`; this module handles transport concerns.

use axum::{Router, middleware};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::api;
use crate::auth::{self, AuthState};
use crate::state::AppState;

/// Configuration for starting the backend server.
#[derive(Debug, Clone)]
pub struct BackendConfig {
    /// Project root directory (parent of `.ito/`).
    pub project_root: PathBuf,
    /// Path to the `.ito/` directory. Defaults to `project_root.join(".ito")`.
    pub ito_path: Option<PathBuf>,
    /// Address to bind to (e.g., `"127.0.0.1"`).
    pub bind: String,
    /// Port to listen on.
    pub port: u16,
    /// Authentication token. If `None`, a deterministic token is generated.
    pub token: Option<String>,
    /// CORS allowed origins. `None` means permissive.
    pub cors_origins: Option<Vec<String>>,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            project_root: PathBuf::from("."),
            ito_path: None,
            bind: "127.0.0.1".to_string(),
            port: 9010,
            token: None,
            cors_origins: None,
        }
    }
}

/// Start the backend API server and block until it shuts down.
///
/// Assembles routes, auth middleware, and CORS, then binds to the configured
/// address. Prints the listening address and token to stderr on startup.
pub async fn serve(config: BackendConfig) -> miette::Result<()> {
    let root = config
        .project_root
        .canonicalize()
        .unwrap_or(config.project_root.clone());

    // Resolve authentication token
    let token = config.token.unwrap_or_else(|| auth::generate_token(&root));

    let app_state = Arc::new(match config.ito_path {
        Some(ito_path) => AppState::with_ito_path(root.clone(), ito_path),
        None => AppState::new(root.clone()),
    });

    let auth_state = Arc::new(AuthState {
        token: token.clone(),
    });

    // Build CORS layer
    let cors = match config.cors_origins {
        Some(origins) => {
            let mut layer = CorsLayer::new();
            for origin in origins {
                let Ok(origin): Result<axum::http::HeaderValue, _> = origin.parse() else {
                    continue;
                };
                layer = layer.allow_origin(origin);
            }
            layer
        }
        None => CorsLayer::permissive(),
    };

    let app = Router::new()
        .nest("/api/v1", api::v1_router())
        .with_state(app_state)
        .layer(middleware::from_fn_with_state(
            auth_state,
            auth::auth_middleware,
        ))
        .layer(cors);

    let addr: SocketAddr = format!("{}:{}", config.bind, config.port)
        .parse()
        .map_err(|e| miette::miette!("Invalid address: {e}"))?;

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| miette::miette!("Failed to bind to {addr}: {e}"))?;

    eprintln!("ito-backend serving {} at http://{addr}/", root.display());
    eprintln!("Auth token: {token}");

    axum::serve(listener, app)
        .await
        .map_err(|e| miette::miette!("Server error: {e}"))?;

    Ok(())
}
