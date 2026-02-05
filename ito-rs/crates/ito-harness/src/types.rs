use miette::Result;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessName(pub &'static str);

impl HarnessName {
    pub const OPENCODE: HarnessName = HarnessName("opencode");
    pub const STUB: HarnessName = HarnessName("stub");
}

#[derive(Debug, Clone)]
pub struct HarnessRunConfig {
    pub prompt: String,
    pub model: Option<String>,
    pub cwd: PathBuf,
    pub env: BTreeMap<String, String>,
    pub interactive: bool,
    /// Inactivity timeout - if no output is received for this duration, the harness should terminate.
    pub inactivity_timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct HarnessRunResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration: Duration,
    /// True if the harness was terminated due to inactivity timeout.
    pub timed_out: bool,
}

pub trait Harness {
    fn name(&self) -> HarnessName;
    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult>;
    fn stop(&mut self);

    /// Returns true if the harness streams output in real-time during `run()`.
    /// When true, the caller should NOT print stdout/stderr after run completes
    /// as it has already been streamed.
    fn streams_output(&self) -> bool {
        false
    }
}
