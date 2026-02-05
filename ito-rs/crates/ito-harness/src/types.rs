use miette::Result;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Identifier for a harness implementation.
pub struct HarnessName(pub &'static str);

impl HarnessName {
    /// The OpenCode harness.
    pub const OPENCODE: HarnessName = HarnessName("opencode");
    /// The stub harness.
    pub const STUB: HarnessName = HarnessName("stub");
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

/// A runnable harness implementation.
pub trait Harness {
    /// Return the harness identifier.
    fn name(&self) -> HarnessName;

    /// Execute the harness invocation.
    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult>;

    /// Stop any in-flight execution (best-effort).
    fn stop(&mut self);

    /// Returns true if the harness streams output in real-time during `run()`.
    /// When true, the caller should NOT print stdout/stderr after run completes
    /// as it has already been streamed.
    fn streams_output(&self) -> bool {
        false
    }
}
