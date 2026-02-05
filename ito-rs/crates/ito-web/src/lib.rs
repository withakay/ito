//! Web server for browsing and editing Ito projects.
//!
//! `ito-web` provides the server-side pieces behind the Ito web UI.

#![warn(missing_docs)]

mod api;
mod auth;
mod frontend;
mod server;
mod terminal;

/// Start the web server with the given configuration.
pub use server::{ServeConfig, serve};
