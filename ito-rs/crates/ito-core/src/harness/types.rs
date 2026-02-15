use miette::Result;
use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Identifier for a harness implementation.
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
    /// The canonical user-facing string for this harness.
    ///
    /// This yields the single canonical identifier shown to users (aliases such as
    /// `github-copilot` are accepted by parsing but map to the canonical `"copilot"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::harness::types::HarnessName;
    ///
    /// assert_eq!(HarnessName::Opencode.as_str(), "opencode");
    /// assert_eq!(HarnessName::GithubCopilot.as_str(), "copilot");
    /// ```
    pub const fn as_str(self) -> &'static str {
        match self {
            HarnessName::Opencode => "opencode",
            HarnessName::Claude => "claude",
            HarnessName::Codex => "codex",
            HarnessName::GithubCopilot => "copilot",
            HarnessName::Stub => "stub",
        }
    }

    /// Iterator over harness variants exposed to users.
    ///
    /// This excludes `HarnessName::Stub` (testing-only harness).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::harness::types::HarnessName;
    ///
    /// let names: Vec<_> = HarnessName::user_facing().map(HarnessName::as_str).collect();
    /// assert_eq!(names, ["opencode", "claude", "codex", "copilot"]);
    /// ```
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
    /// Formats the harness name as its canonical user-facing string.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = format!("{}", HarnessName::Opencode);
    /// assert_eq!(s, "opencode");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parse error for [`HarnessName`].
pub struct HarnessNameParseError {
    /// The raw value that could not be parsed.
    pub input: String,
}

impl fmt::Display for HarnessNameParseError {
    /// Formats a human-readable error message for an unknown harness name.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// let err = crate::HarnessNameParseError { input: "unknown-harness".into() };
    /// let s = format!("{}", err);
    /// assert_eq!(s, "Unknown harness name: unknown-harness");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown harness name: {}", self.input)
    }
}

impl std::error::Error for HarnessNameParseError {}

impl FromStr for HarnessName {
    type Err = HarnessNameParseError;

    /// Parses a string into the corresponding `HarnessName` variant.
    ///
    /// Recognized, case-sensitive inputs are:
    /// - `"opencode"` -> `HarnessName::Opencode`
    /// - `"claude"` -> `HarnessName::Claude`
    /// - `"codex"` -> `HarnessName::Codex`
    /// - `"copilot"` or `"github-copilot"` -> `HarnessName::GithubCopilot`
    /// - `"stub"` -> `HarnessName::Stub`
    ///
    /// # Returns
    ///
    /// `Ok(HarnessName)` for recognized inputs; `Err(HarnessNameParseError)` containing the original input for unrecognized strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// assert_eq!(HarnessName::from_str("opencode").unwrap(), HarnessName::Opencode);
    /// assert_eq!(HarnessName::from_str("github-copilot").unwrap(), HarnessName::GithubCopilot);
    /// assert!(HarnessName::from_str("invalid-name").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opencode" => Ok(HarnessName::Opencode),
            "claude" => Ok(HarnessName::Claude),
            "codex" => Ok(HarnessName::Codex),
            "copilot" | "github-copilot" => Ok(HarnessName::GithubCopilot),
            "stub" => Ok(HarnessName::Stub),
            other => Err(HarnessNameParseError {
                input: other.to_string(),
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
    132, // SIGILL
    134, // SIGABRT
    135, // SIGBUS
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
    /// Determines whether the exit code indicates a transient process crash that should be retried.
    ///
    /// Signal-based exit codes (values of 128 + N) represent termination by an OS signal and are
    /// treated as transient; they are considered retriable and are not counted toward persistent error thresholds.
    /// Other exit codes are treated as normal failures.
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
        let mut names = Vec::new();
        for name in HarnessName::user_facing() {
            names.push(name.as_str());
        }
        assert_eq!(names, vec!["opencode", "claude", "codex", "copilot"]);
    }

    #[test]
    fn from_str_valid_variants() {
        assert_eq!(
            "opencode".parse::<HarnessName>().unwrap(),
            HarnessName::Opencode
        );
        assert_eq!(
            "claude".parse::<HarnessName>().unwrap(),
            HarnessName::Claude
        );
        assert_eq!("codex".parse::<HarnessName>().unwrap(), HarnessName::Codex);
        assert_eq!(
            "copilot".parse::<HarnessName>().unwrap(),
            HarnessName::GithubCopilot
        );
        assert_eq!(
            "github-copilot".parse::<HarnessName>().unwrap(),
            HarnessName::GithubCopilot
        );
        assert_eq!("stub".parse::<HarnessName>().unwrap(), HarnessName::Stub);
    }

    #[test]
    fn from_str_invalid_returns_error() {
        let invalid_inputs = vec!["invalid", "", "OPENCODE"];
        for input in invalid_inputs {
            let result = input.parse::<HarnessName>();
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err.input, input);
        }
    }

    #[test]
    fn as_str_all_variants() {
        assert_eq!(HarnessName::Opencode.as_str(), "opencode");
        assert_eq!(HarnessName::Claude.as_str(), "claude");
        assert_eq!(HarnessName::Codex.as_str(), "codex");
        assert_eq!(HarnessName::GithubCopilot.as_str(), "copilot");
        assert_eq!(HarnessName::Stub.as_str(), "stub");
    }

    #[test]
    fn display_matches_as_str() {
        let variants = vec![
            HarnessName::Opencode,
            HarnessName::Claude,
            HarnessName::Codex,
            HarnessName::GithubCopilot,
            HarnessName::Stub,
        ];
        for variant in variants {
            assert_eq!(format!("{}", variant), variant.as_str());
        }
    }

    #[test]
    fn parse_error_display() {
        let err = HarnessNameParseError {
            input: "foo".to_string(),
        };
        assert_eq!(format!("{}", err), "Unknown harness name: foo");
    }

    /// Creates a HarnessRunResult with empty stdout/stderr, the provided exit code, a 1 second duration, and timed_out set to false.
    ///
    /// # Examples
    ///
    /// ```
    /// let r = make_result(42);
    /// assert_eq!(r.exit_code, 42);
    /// assert_eq!(r.stdout, "");
    /// assert_eq!(r.stderr, "");
    /// assert_eq!(r.duration.as_secs(), 1);
    /// assert!(!r.timed_out);
    /// ```
    fn make_result(exit_code: i32) -> HarnessRunResult {
        HarnessRunResult {
            stdout: String::new(),
            stderr: String::new(),
            exit_code,
            duration: Duration::from_secs(1),
            timed_out: false,
        }
    }

    #[test]
    fn is_retriable_for_all_retriable_codes() {
        let retriable_codes = vec![128, 129, 130, 131, 132, 134, 135, 136, 137, 139, 141, 143];
        for code in retriable_codes {
            let result = make_result(code);
            assert!(
                result.is_retriable(),
                "Exit code {} should be retriable",
                code
            );
        }
    }

    #[test]
    fn is_not_retriable_for_normal_codes() {
        let normal_codes = vec![0, 1, 2, 127, 133, 144, 255, -1];
        for code in normal_codes {
            let result = make_result(code);
            assert!(
                !result.is_retriable(),
                "Exit code {} should not be retriable",
                code
            );
        }
    }
}