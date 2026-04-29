//! Harness session environment detection shared across Ito crates.

/// Ito harness identifiers inferred from session environment variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessKind {
    /// Generic Ito harness session override.
    Generic,
    /// Claude Code harness.
    ClaudeCode,
    /// OpenCode harness.
    OpenCode,
    /// Codex harness.
    Codex,
    /// GitHub Copilot harness.
    GitHubCopilot,
    /// Pi coding agent harness.
    Pi,
}

impl HarnessKind {
    /// Return the canonical harness identifier used in generated Ito assets.
    pub fn canonical_name(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::ClaudeCode => "claude-code",
            Self::OpenCode => "opencode",
            Self::Codex => "codex",
            Self::GitHubCopilot => "github-copilot",
            Self::Pi => "pi",
        }
    }
}

/// Resolved harness session environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessSessionEnv {
    /// Environment variable that supplied the session id.
    pub env_var: &'static str,
    /// Session id value from the environment.
    pub session_id: String,
    /// Harness kind inferred from the environment variable.
    pub harness: HarnessKind,
}

#[derive(Debug, Clone, Copy)]
struct HarnessSessionProbe {
    env_var: &'static str,
    harness: HarnessKind,
}

const HARNESS_SESSION_PROBES: &[HarnessSessionProbe] = &[
    HarnessSessionProbe {
        env_var: "ITO_HARNESS_SESSION_ID",
        harness: HarnessKind::Generic,
    },
    HarnessSessionProbe {
        env_var: "CLAUDE_SESSION_ID",
        harness: HarnessKind::ClaudeCode,
    },
    HarnessSessionProbe {
        env_var: "OPENCODE_SESSION_ID",
        harness: HarnessKind::OpenCode,
    },
    HarnessSessionProbe {
        env_var: "CODEX_SESSION_ID",
        harness: HarnessKind::Codex,
    },
    HarnessSessionProbe {
        env_var: "GITHUB_COPILOT_CHAT_SESSION_ID",
        harness: HarnessKind::GitHubCopilot,
    },
    HarnessSessionProbe {
        env_var: "PI_SESSION_ID",
        harness: HarnessKind::Pi,
    },
];

/// Environment variables that Ito checks for harness session context, in precedence order.
pub const HARNESS_SESSION_ENV_VARS: &[&str] = &[
    "ITO_HARNESS_SESSION_ID",
    "CLAUDE_SESSION_ID",
    "OPENCODE_SESSION_ID",
    "CODEX_SESSION_ID",
    "GITHUB_COPILOT_CHAT_SESSION_ID",
    "PI_SESSION_ID",
];

/// Resolve harness session environment from the current process.
pub fn resolve_harness_session_env() -> Option<HarnessSessionEnv> {
    resolve_harness_session_env_with(|env_var| std::env::var(env_var).ok())
}

fn resolve_harness_session_env_with(
    mut get_env: impl FnMut(&str) -> Option<String>,
) -> Option<HarnessSessionEnv> {
    for HarnessSessionProbe { env_var, harness } in HARNESS_SESSION_PROBES {
        let Some(session_id) = get_env(env_var) else {
            continue;
        };
        if session_id.is_empty() {
            continue;
        }
        return Some(HarnessSessionEnv {
            env_var,
            session_id,
            harness: *harness,
        });
    }

    None
}

/// Resolve the first configured harness session id from the current process.
pub fn resolve_harness_session_id() -> Option<String> {
    resolve_harness_session_env().map(|env| env.session_id)
}

/// Detect the current harness canonical name from session environment variables.
pub fn detect_harness_name() -> &'static str {
    resolve_harness_session_env()
        .map(|env| env.harness.canonical_name())
        .unwrap_or("unknown")
}

#[cfg(test)]
mod tests {
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
}
