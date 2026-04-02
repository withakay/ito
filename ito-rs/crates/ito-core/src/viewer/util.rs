//! Shared utilities for viewer backends.

use std::process::{Command, Stdio};

use crate::errors::{CoreError, CoreResult};

pub(crate) fn command_on_path(binary: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join(binary).is_file()))
}

pub(crate) fn run_with_stdin(binary: &str, args: &[&str], content: &str) -> CoreResult<()> {
    if !command_on_path(binary) {
        return Err(CoreError::not_found(format!(
            "{binary} is not installed or not on PATH"
        )));
    }

    let mut child = Command::new(binary)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| CoreError::io(format!("spawning {binary}"), e))?;

    if let Some(mut stdin) = child.stdin.take() {
        std::io::Write::write_all(&mut stdin, content.as_bytes())
            .map_err(|e| CoreError::io(format!("writing to {binary} stdin"), e))?;
    }

    let status = child
        .wait()
        .map_err(|e| CoreError::io(format!("waiting for {binary}"), e))?;

    if status.success() {
        Ok(())
    } else {
        Err(CoreError::process(format!(
            "{binary} exited with status {status}"
        )))
    }
}
