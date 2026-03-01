//! Multi-tenant backend state API for Ito projects.
//!
//! `ito-backend` is a **Layer 3 adapter** that exposes Ito project state
//! (changes, tasks, modules) via a RESTful HTTP API. It delegates all business
//! logic to [`ito_core`] and communicates exclusively in JSON.
//!
//! All project-scoped routes are nested under `/api/v1/projects/{org}/{repo}/`.
//! Authentication uses admin tokens and HMAC-SHA256 derived per-project tokens.
//! An organization/repository allowlist is enforced before token validation.
//!
//! The public surface is intentionally minimal: call [`serve`] with a
//! [`BackendServerConfig`] to
//! start the server.

#![warn(missing_docs)]

mod api;
mod auth;
mod error;
mod server;
mod state;

pub use server::serve;

/// Re-export the configuration type callers need to start the server.
pub use ito_config::types::BackendServerConfig;

/// Re-export auth utilities for token derivation.
pub use auth::derive_project_token;
