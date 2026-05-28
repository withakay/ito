use super::{
    HARNESS_SESSION_ENV_VARS, HARNESS_SESSION_PROBES, HarnessSessionEnv,
    resolve_harness_session_env_with,
};

fn resolve_from(values: &[(&str, &str)]) -> Option<HarnessSessionEnv> {
    resolve_harness_session_env_with(|env_var| {
        values
            .iter()
            .find(|(key, _value)| *key == env_var)
            .map(|(_key, value)| (*value).to_string())
    })
}

#[test]
fn detects_canonical_harness_names() {
    let cases = [
        ("ITO_HARNESS_SESSION_ID", "generic"),
        ("CLAUDE_SESSION_ID", "claude-code"),
        ("OPENCODE_SESSION_ID", "opencode"),
        ("CODEX_SESSION_ID", "codex"),
        ("GITHUB_COPILOT_CHAT_SESSION_ID", "github-copilot"),
        ("PI_SESSION_ID", "pi"),
    ];

    for (env_var, expected) in cases {
        let env = resolve_from(&[(env_var, "test-session")]).expect("harness env");
        assert_eq!(env.harness.canonical_name(), expected);
        assert_eq!(env.env_var, env_var);
    }
}

#[test]
fn generic_session_id_takes_precedence() {
    let env = resolve_from(&[
        ("OPENCODE_SESSION_ID", "opencode-session"),
        ("ITO_HARNESS_SESSION_ID", "generic-session"),
    ])
    .expect("harness env");
    assert_eq!(env.env_var, "ITO_HARNESS_SESSION_ID");
    assert_eq!(env.session_id, "generic-session");
    assert_eq!(env.harness.canonical_name(), "generic");
}

#[test]
fn returns_unknown_without_harness_env() {
    assert_eq!(resolve_from(&[]), None);
}

#[test]
fn public_env_var_list_matches_probe_order() {
    let probe_env_vars: Vec<&str> = HARNESS_SESSION_PROBES
        .iter()
        .map(|probe| probe.env_var)
        .collect();
    assert_eq!(probe_env_vars, HARNESS_SESSION_ENV_VARS);
}
