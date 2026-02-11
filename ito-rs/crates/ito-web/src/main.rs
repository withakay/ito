//! Standalone binary for running the web UI outside of `ito-cli`.
//!
//! Useful during development — it launches the same [`ito_web::serve`] server
//! but with its own CLI argument parser so it can be invoked directly via
//! `cargo run -p ito-web`.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ito-web", about = "Modern file browser and editor")]
struct Args {
    /// Root directory to serve
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

    /// Port to listen on
    #[arg(short, long, default_value = "9009")]
    port: u16,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    ito_web::serve(ito_web::ServeConfig {
        root: args.root,
        bind: args.bind,
        port: args.port,
    })
    .await
}
