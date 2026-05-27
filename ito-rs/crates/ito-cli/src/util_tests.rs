use super::*;

#[test]
fn split_csv_trims_parts() {
    assert_eq!(split_csv("a, b ,c"), vec!["a", "b", "c"]);
}

#[test]
fn command_id_uses_positional_args_and_normalizes_hyphens() {
    let args = vec![
        "agent".to_string(),
        "instruction".to_string(),
        "apply".to_string(),
    ];
    assert_eq!(command_id_from_args(&args), "ito.agent.instruction");

    let args = vec!["agent-config".to_string(), "summary".to_string()];
    assert_eq!(command_id_from_args(&args), "ito.agent_config.summary");
}

#[test]
fn command_id_maps_x_templates_to_templates() {
    let args = vec!["x-templates".to_string(), "--json".to_string()];
    assert_eq!(command_id_from_args(&args), "ito.templates");
}

#[test]
fn command_id_maps_gr_to_grep() {
    let args = vec!["gr".to_string(), "--all".to_string(), "pattern".to_string()];
    assert_eq!(command_id_from_args(&args), "ito.grep");
}

#[test]
fn sanitize_args_redacts_sensitive_flags() {
    let args = vec![
        "agent".to_string(),
        "--token".to_string(),
        "my-secret-token".to_string(),
        "--change".to_string(),
        "foo".to_string(),
    ];
    let sanitized = sanitize_args_for_logging(&args);
    assert_eq!(
        sanitized,
        vec!["agent", "--token", "<redacted>", "--change", "foo"]
    );
}

#[test]
fn sanitize_args_redacts_equals_form() {
    let args = vec!["--api-key=sk-1234".to_string(), "command".to_string()];
    let sanitized = sanitize_args_for_logging(&args);
    assert_eq!(sanitized, vec!["--api-key=<redacted>", "command"]);
}

#[test]
fn sanitize_args_replaces_paths() {
    let args = vec![
        "agent".to_string(),
        "/home/user/project".to_string(),
        "instruction".to_string(),
    ];
    let sanitized = sanitize_args_for_logging(&args);
    assert_eq!(sanitized, vec!["agent", "<path>", "instruction"]);
}
