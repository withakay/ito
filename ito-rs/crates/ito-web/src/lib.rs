//! ito-web: A modern file browser and editor for Ito projects.

mod api;
mod auth;
mod frontend;
mod server;
mod terminal;

pub use server::{ServeConfig, serve};
