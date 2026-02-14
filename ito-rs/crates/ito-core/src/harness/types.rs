use miette::Result;
use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

/// Identifier for a harness implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HarnessName {
    /// The OpenCode harness.
    Opencode,
    /// The Claude Code harness.
    Claude,
    /// The OpenAI Codex harness.
    Codex,
    /// The GitHub Copilot harness.
    GithubCopilot,
    /// The stub harness (testing only, not user-facing).
    Stub,
}

impl HarnessName {
    /// The canonical, user-facing name for this harness.
    ///
    /// Note: some harnesses accept additional aliases (for example,
    /// [`HarnessName::GithubCopilot`] also accepts `github-copilot`).
    pub const fn as_str(self) -> &'static str {
        match self {
            HarnessName::Opencode => "opencode",
            HarnessName::Claude => "claude",
            HarnessName::Codex => "codex",
            HarnessName::GithubCopilot => "copilot",
            HarnessName::Stub => "stub",
        }
    }

    /// Iterator of harnesses intended for user-facing CLI help.
    ///
    /// Does not include [`HarnessName::Stub`] (testing only).
    pub fn user_facing() -> impl Iterator<Item = HarnessName> {
        [
            HarnessName::Opencode,
            HarnessName::Claude,
            HarnessName::Codex,
            HarnessName::GithubCopilot,
        ]
        .into_iter()
    }
}

impl fmt::Display for HarnessName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Parse error for [`HarnessName`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessNameParseError {
    /// The raw value that could not be parsed.
    pub input: String,
}

impl fmt::Display for HarnessNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown harness name: {}", self.input)
    }
}

impl std::error::Error for HarnessNameParseError {}

impl FromStr for HarnessName {
    type Err = HarnessNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opencode" => Ok(HarnessName::Opencode),
            "claude" => Ok(HarnessName::Claude),
            "codex" => Ok(HarnessName::Codex),
            "copilot" | "github-copilot" => Ok(HarnessName::GithubCopilot),
            "stub" => Ok(HarnessName::Stub),
            _ => Err(HarnessNameParseError {
                input: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
/// Inputs for running a single harness invocation.
pub struct HarnessRunConfig {
    /// Prompt text passed to the harness.
    pub prompt: String,
    /// Optional model identifier to use.
    pub model: Option<String>,
    /// Working directory for the harness process.
    pub cwd: PathBuf,
    /// Environment variables to set for the harness process.
    pub env: BTreeMap<String, String>,
    /// Whether the invocation should run in interactive mode.
    pub interactive: bool,
    /// Whether tool approval and permission prompts should be bypassed.
    pub allow_all: bool,
    /// Inactivity timeout - if no output is received for this duration, the harness should terminate.
    pub inactivity_timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
/// Results from running a harness.
pub struct HarnessRunResult {
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
    /// Process exit code (or harness-specific value).
    pub exit_code: i32,
    /// End-to-end duration of the run.
    pub duration: Duration,
    /// True if the harness was terminated due to inactivity timeout.
    pub timed_out: bool,
}

/// Exit codes that indicate a transient process crash (not a logical agent error).
///
/// These are retried automatically without counting against the error threshold,
/// because the harness process itself failed — not the work it was doing.
///
/// - `128` — generic fatal signal on many CLIs
/// - `128 + signal` — killed by signal (e.g. 137 = SIGKILL, 139 = SIGSEGV, 130 = SIGINT)
/// - `-1` — used internally for inactivity timeout (handled separately, listed for completeness)
const RETRIABLE_EXIT_CODES: &[i32] = &[
    128, // Generic fatal signal
    129, // SIGHUP
    130, // SIGINT
    131, // SIGQUIT
    134, // SIGABRT
    136, // SIGFPE
    137, // SIGKILL
    139, // SIGSEGV
    141, // SIGPIPE
    143, // SIGTERM
];

/// Maximum number of consecutive retriable-exit retries before giving up.
///
/// Prevents infinite retry loops when a harness consistently crashes.
pub const MAX_RETRIABLE_RETRIES: u32 = 3;

impl HarnessRunResult {
    /// Whether the exit code indicates a transient process crash that should be retried.
    ///
    /// Signal-based exit codes (128+N) indicate the process was killed by the OS or
    /// a signal, not that the agent's work failed. These are retried without counting
    /// against the error threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::HarnessRunResult;
    /// use std::time::Duration;
    ///
    /// let result = HarnessRunResult {
    ///     stdout: String::new(),
    ///     stderr: String::new(),
    ///     exit_code: 128,
    ///     duration: Duration::from_secs(1),
    ///     timed_out: false,
    /// };
    /// assert!(result.is_retriable());
    ///
    /// let normal_failure = HarnessRunResult {
    ///     exit_code: 1,
    ///     ..result.clone()
    /// };
    /// assert!(!normal_failure.is_retriable());
    /// ```
    pub fn is_retriable(&self) -> bool {
        RETRIABLE_EXIT_CODES.contains(&self.exit_code)
    }
}

/// A runnable harness implementation.
pub trait Harness {
    /// Return the harness identifier.
    fn name(&self) -> HarnessName;

    /// Execute the harness invocation.
    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult>;

    /// Stop any in-flight execution (best-effort).
    fn stop(&mut self);

    /// Indicates whether the harness streams stdout/stderr in real time during `run`.
    ///
    /// When this returns `true`, callers should not print captured stdout or stderr after
    /// `run` completes because output has already been delivered to the caller in real time.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// struct Dummy;
    /// impl super::Harness for Dummy {
    ///     fn name(&self) -> super::HarnessName { super::HarnessName::Stub }
    ///     fn run(&mut self, _config: &super::HarnessRunConfig) -> miette::Result<super::HarnessRunResult> {
    ///         unimplemented!()
    ///     }
    ///     fn streams_output(&self) -> bool { false }
    ///     fn stop(&mut self) {}
    /// }
    ///
    /// let d = Dummy;
    /// assert!(!d.streams_output());
    /// ```
    fn streams_output(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_help_matches_user_facing() {
        let names: Vec<&'static str> = HarnessName::user_facing()
            .map(HarnessName::as_str)
            .collect();
        assert_eq!(names, vec!["opencode", "claude", "codex", "copilot"]);
    }
}
