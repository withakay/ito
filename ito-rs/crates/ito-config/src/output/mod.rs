//! UI-related option resolution.
//!
//! These helpers translate CLI flags + environment variables into a single
//! `UiOptions` struct used by higher-level crates.

use std::io::IsTerminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// UI behavior flags.
pub struct UiOptions {
    /// Disable ANSI color output.
    pub no_color: bool,
    /// Whether interactive prompts are allowed.
    pub interactive: bool,
}

/// Return `true` if stdout is a TTY.
pub fn stdout_is_tty() -> bool {
    std::io::stdout().is_terminal()
}

/// Resolve UI options from CLI flags and environment variables.
pub fn resolve_ui_options(
    cli_no_color: bool,
    no_color_env: Option<&str>,
    cli_no_interactive: bool,
    ito_interactive_env: Option<&str>,
) -> UiOptions {
    resolve_ui_options_with_tty(
        cli_no_color,
        no_color_env,
        cli_no_interactive,
        ito_interactive_env,
        stdout_is_tty(),
    )
}

/// Resolve UI options, allowing tests to inject TTY detection.
pub fn resolve_ui_options_with_tty(
    cli_no_color: bool,
    no_color_env: Option<&str>,
    cli_no_interactive: bool,
    ito_interactive_env: Option<&str>,
    stdout_is_tty: bool,
) -> UiOptions {
    let interactive = resolve_interactive(cli_no_interactive, ito_interactive_env, stdout_is_tty);
    UiOptions {
        no_color: cli_no_color || no_color_env_set(no_color_env),
        interactive,
    }
}

/// Interpret the `NO_COLOR` env var value.
pub fn no_color_env_set(value: Option<&str>) -> bool {
    match value {
        Some("1") => true,
        Some("true") => true,
        Some(_) => false,
        None => false,
    }
}

/// Resolve whether prompts should run interactively.
pub fn resolve_interactive(
    cli_no_interactive: bool,
    env: Option<&str>,
    stdout_is_tty: bool,
) -> bool {
    if cli_no_interactive {
        return false;
    }

    match env {
        Some("1") => true,
        Some("0") => false,
        Some(_) => true,
        None => stdout_is_tty,
    }
}

#[cfg(test)]
mod output_tests;
