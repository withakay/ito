//! Handler for the `serve-api` subcommand.

use crate::cli::ServeApiArgs;
use crate::cli_error::{CliResult, fail};
use crate::runtime::Runtime;
use std::path::Path;

fn ensure_ito_dir_exists(ito_path: &Path) -> CliResult<()> {
    if ito_path.is_dir() {
        return Ok(());
    }
    fail("No .ito directory found in this project. Run `ito init` first.")
}

pub(crate) fn handle_serve_api_clap(rt: &Runtime, args: &ServeApiArgs) -> CliResult<()> {
    let ito_path = rt.ito_path();
    ensure_ito_dir_exists(ito_path)?;

    let project_root = ito_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let config = ito_backend::BackendConfig {
        project_root,
        ito_path: Some(ito_path.to_path_buf()),
        bind: args.bind.clone().unwrap_or_else(|| "127.0.0.1".to_string()),
        port: args.port.unwrap_or(9010),
        token: args.token.clone(),
        cors_origins: None,
    };

    let runtime = tokio::runtime::Runtime::new().map_err(|e| {
        crate::cli_error::CliError::msg(format!("Failed to create tokio runtime: {e}"))
    })?;

    runtime.block_on(async {
        ito_backend::serve(config)
            .await
            .map_err(|e| crate::cli_error::CliError::msg(format!("Server error: {e}")))
    })?;

    Ok(())
}

#[cfg(test)]
mod serve_api_tests {
    use super::*;

    #[test]
    fn ensure_ito_dir_exists_errors_when_missing() {
        let result = ensure_ito_dir_exists(Path::new("/nonexistent/.ito"));
        assert!(result.is_err());
    }

    #[test]
    fn ensure_ito_dir_exists_ok_when_present() {
        let dir = tempfile::tempdir().unwrap();
        let ito_path = dir.path().join(".ito");
        std::fs::create_dir(&ito_path).unwrap();
        let result = ensure_ito_dir_exists(&ito_path);
        assert!(result.is_ok());
    }

    #[test]
    fn ensure_ito_dir_exists_errors_when_path_is_file() {
        let dir = tempfile::tempdir().unwrap();
        let ito_path = dir.path().join(".ito");
        std::fs::write(&ito_path, "not a dir").unwrap();
        let result = ensure_ito_dir_exists(&ito_path);
        assert!(result.is_err());
    }
}
