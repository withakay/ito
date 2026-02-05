//! HTTP server setup and configuration.

use axum::{Router, middleware, routing::get};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::api;
use crate::auth::{self, AuthState};
use crate::frontend;
use crate::terminal::{self, TerminalState};

/// Server configuration.
#[derive(Debug, Clone)]
pub struct ServeConfig {
    /// Root directory to serve (typically the project root).
    pub root: PathBuf,
    /// Address to bind to.
    pub bind: String,
    /// Port to listen on.
    pub port: u16,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            bind: "127.0.0.1".to_string(),
            port: 9009,
        }
    }
}

/// Start the web server.
pub async fn serve(config: ServeConfig) -> miette::Result<()> {
    let root = config.root.canonicalize().unwrap_or(config.root.clone());

    // Generate token for non-loopback addresses
    let token = if auth::is_loopback(&config.bind) {
        None
    } else {
        Some(auth::generate_token(&root))
    };

    let auth_state = Arc::new(AuthState {
        token: token.clone(),
    });
    let terminal_state = Arc::new(TerminalState { root: root.clone() });

    let app = Router::new()
        // Frontend routes
        .route("/", get(frontend::index))
        .route("/app.js", get(frontend::app_js))
        // Terminal WebSocket
        .route("/ws/terminal", get(terminal::ws_handler))
        .with_state(terminal_state)
        // API routes
        .nest("/api", api::router(root.clone()))
        // Auth middleware (checks token for non-loopback)
        .layer(middleware::from_fn_with_state(
            auth_state,
            auth::auth_middleware,
        ))
        // CORS for development
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = format!("{}:{}", config.bind, config.port)
        .parse()
        .map_err(|e| miette::miette!("Invalid address: {e}"))?;

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| miette::miette!("Failed to bind to {addr}: {e}"))?;

    // Print URL with token if required
    let url = if let Some(t) = &token {
        format!("http://{addr}/?token={t}")
    } else {
        format!("http://{addr}/")
    };
    println!("Serving {} at {url}", root.display());

    axum::serve(listener, app)
        .await
        .map_err(|e| miette::miette!("Server error: {e}"))?;

    Ok(())
}
