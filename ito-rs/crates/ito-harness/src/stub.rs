use crate::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::{Result, miette};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StubStep {
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub exit_code: i32,
}

#[derive(Debug, Clone)]
pub struct StubHarness {
    steps: Vec<StubStep>,
    idx: usize,
}

impl StubHarness {
    pub fn new(steps: Vec<StubStep>) -> Self {
        Self { steps, idx: 0 }
    }

    pub fn from_json_path(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .map_err(|e| miette!("Failed to read stub script {p}: {e}", p = path.display()))?;
        let steps: Vec<StubStep> = serde_json::from_str(&raw)
            .map_err(|e| miette!("Invalid stub script JSON in {p}: {e}", p = path.display()))?;
        Ok(Self::new(steps))
    }

    pub fn from_env_or_default(script_path: Option<PathBuf>) -> Result<Self> {
        let from_env = std::env::var("ITO_STUB_SCRIPT").ok().map(PathBuf::from);
        let path = script_path.or(from_env);
        if let Some(p) = path {
            return Self::from_json_path(&p);
        }

        // Default: single successful completion.
        Ok(Self::new(vec![StubStep {
            stdout: "<promise>COMPLETE</promise>\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        }]))
    }

    fn next_step(&mut self) -> Option<StubStep> {
        if self.steps.is_empty() {
            return None;
        }
        let step = self
            .steps
            .get(self.idx)
            .cloned()
            .or_else(|| self.steps.last().cloned());
        self.idx = self.idx.saturating_add(1);
        step
    }
}

impl Harness for StubHarness {
    fn name(&self) -> HarnessName {
        HarnessName::STUB
    }

    fn run(&mut self, _config: &HarnessRunConfig) -> Result<HarnessRunResult> {
        let started = Instant::now();
        let step = self
            .next_step()
            .ok_or_else(|| miette!("Stub harness has no steps"))?;

        Ok(HarnessRunResult {
            stdout: step.stdout,
            stderr: step.stderr,
            exit_code: step.exit_code,
            duration: started.elapsed().max(Duration::from_millis(1)),
            timed_out: false,
        })
    }

    fn stop(&mut self) {
        // No-op
    }
}
