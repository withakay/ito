use crate::cli::ServeArgs;
use crate::cli_error::{CliResult, fail};
use crate::runtime::Runtime;
use std::path::Path;
use std::process::Command;

/// Detect the Tailscale IPv4 address by running `tailscale ip -4`.
fn detect_tailscale_ip() -> CliResult<String> {
    detect_tailscale_ip_with(Path::new("tailscale"))
}

fn detect_tailscale_ip_with(cmd: &Path) -> CliResult<String> {
    let output = Command::new(cmd).args(["ip", "-4"]).output().map_err(|e| {
        crate::cli_error::CliError::msg(format!(
            "Failed to run 'tailscale ip -4': {e}. Is Tailscale installed and on PATH?"
        ))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return fail(format!("Tailscale command failed: {}", stderr.trim()));
    }

    let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if ip.is_empty() {
        return fail("Tailscale returned empty IP. Is Tailscale connected?");
    }

    Ok(ip)
}

fn ensure_ito_dir_exists(ito_path: &Path) -> CliResult<()> {
    if ito_path.is_dir() {
        return Ok(());
    }
    fail("No .ito directory found in this project. Run `ito init` first.")
}

pub(crate) fn handle_serve_clap(rt: &Runtime, args: &ServeArgs) -> CliResult<()> {
    let ito_path = rt.ito_path();
    ensure_ito_dir_exists(ito_path)?;

    // Project root is parent of .ito
    let project_root = ito_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    // Determine bind address
    let bind_addr = if args.tailscale {
        detect_tailscale_ip()?
    } else {
        args.bind.clone().unwrap_or_else(|| "127.0.0.1".to_string())
    };

    let port = args.port.unwrap_or(9009);

    let config = ito_web::ServeConfig {
        root: project_root,
        bind: bind_addr,
        port,
    };

    // Run the async server
    let runtime = tokio::runtime::Runtime::new().map_err(|e| {
        crate::cli_error::CliError::msg(format!("Failed to create tokio runtime: {e}"))
    })?;

    runtime.block_on(async {
        ito_web::serve(config)
            .await
            .map_err(|e| crate::cli_error::CliError::msg(format!("Server error: {e}")))
    })?;

    Ok(())
}

#[cfg(test)]
mod serve_tests;
