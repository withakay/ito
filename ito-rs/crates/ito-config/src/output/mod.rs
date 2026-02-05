use std::io::IsTerminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiOptions {
    pub no_color: bool,
    pub interactive: bool,
}

pub fn stdout_is_tty() -> bool {
    std::io::stdout().is_terminal()
}

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

pub fn no_color_env_set(value: Option<&str>) -> bool {
    match value {
        Some("1") => true,
        Some("true") => true,
        Some(_) => false,
        None => false,
    }
}

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
mod tests {
    use super::*;

    #[test]
    fn no_color_env_set_matches_ts_values() {
        assert!(!no_color_env_set(None));
        assert!(!no_color_env_set(Some("0")));
        assert!(no_color_env_set(Some("1")));
        assert!(no_color_env_set(Some("true")));
        assert!(!no_color_env_set(Some("TRUE")));
    }

    #[test]
    fn resolve_interactive_respects_cli_and_env() {
        assert!(resolve_interactive(false, None, true));
        assert!(!resolve_interactive(false, None, false));
        assert!(resolve_interactive(false, Some("1"), false));
        assert!(!resolve_interactive(false, Some("0"), true));
        assert!(!resolve_interactive(true, None, true));
        assert!(!resolve_interactive(true, Some("1"), true));
    }

    #[test]
    fn resolve_ui_options_combines_sources() {
        assert_eq!(
            resolve_ui_options_with_tty(false, None, false, None, false),
            UiOptions {
                no_color: false,
                interactive: false
            }
        );
        assert_eq!(
            resolve_ui_options_with_tty(true, None, false, None, true),
            UiOptions {
                no_color: true,
                interactive: true
            }
        );
        assert_eq!(
            resolve_ui_options_with_tty(false, Some("1"), false, Some("0"), true),
            UiOptions {
                no_color: true,
                interactive: false
            }
        );
    }
}
