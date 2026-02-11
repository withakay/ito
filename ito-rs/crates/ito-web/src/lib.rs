//! Web adapter for browsing and editing Ito projects.
//!
//! `ito-web` is a **Layer 3 adapter** — it owns HTTP routing, authentication,
//! WebSocket terminal emulation, and frontend asset serving. All business logic
//! is delegated to [`ito_core`].
//!
//! The public surface is intentionally minimal: call [`serve`] with a
//! [`ServeConfig`] to start the server.

#![warn(missing_docs)]

mod api;
mod auth;
mod frontend;
mod server;
mod terminal;

pub use server::{ServeConfig, serve};
