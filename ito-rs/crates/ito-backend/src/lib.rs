//! Backend state API for Ito projects.
//!
//! `ito-backend` is a **Layer 3 adapter** that exposes Ito project state
//! (changes, tasks, modules) via a RESTful HTTP API. It delegates all business
//! logic to [`ito_core`] and communicates exclusively in JSON.
//!
//! The public surface is intentionally minimal: call [`serve`] with a
//! [`BackendConfig`] to start the server.

#![warn(missing_docs)]

mod api;
mod auth;
mod error;
mod server;
mod state;

pub use server::{BackendConfig, serve};
