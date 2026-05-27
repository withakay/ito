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
        let session_id = session_id.trim();
        if session_id.is_empty() {
            continue;
        }
        let session_id = session_id.to_owned();
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
#[path = "harness_tests.rs"]
mod harness_tests;
