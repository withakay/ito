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
