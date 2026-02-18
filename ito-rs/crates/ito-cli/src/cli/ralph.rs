use clap::{Args, ValueEnum};

/// CLI-facing harness selector for `ito ralph --harness`.
///
/// This is a bridge type between `ito-cli` (adapter) and `ito-core` (domain
/// orchestration): `HarnessArg` derives `clap::ValueEnum` for parsing and help
/// generation, while `ito_core::harness::HarnessName` stays `clap`-free.
#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum HarnessArg {
    Opencode,
    Claude,
    Codex,
    #[value(alias = "github-copilot")]
    Copilot,
    #[value(hide = true)]
    Stub,
}

impl From<HarnessArg> for ito_core::harness::HarnessName {
    fn from(value: HarnessArg) -> Self {
        match value {
            HarnessArg::Opencode => ito_core::harness::HarnessName::Opencode,
            HarnessArg::Claude => ito_core::harness::HarnessName::Claude,
            HarnessArg::Codex => ito_core::harness::HarnessName::Codex,
            HarnessArg::Copilot => ito_core::harness::HarnessName::GithubCopilot,
            HarnessArg::Stub => ito_core::harness::HarnessName::Stub,
        }
    }
}

/// Run iterative AI loop against a change proposal.
#[derive(Args, Debug, Clone)]
pub struct RalphArgs {
    /// Target a specific change
    #[arg(short = 'c', long)]
    pub change: Option<String>,

    /// Target a module.
    ///
    /// Note: when running with `--no-interactive`, `--module` implies `--continue-module`.
    #[arg(short = 'm', long)]
    pub module: Option<String>,

    /// When using --module, keep working through ready changes until module work is complete
    #[arg(long = "continue-module")]
    pub continue_module: bool,

    /// Keep working through eligible changes across the repo until work is complete
    #[arg(long = "continue-ready")]
    pub continue_ready: bool,

    /// Harness to run
    #[arg(long, value_enum, default_value_t = HarnessArg::Opencode)]
    pub harness: HarnessArg,

    /// Model id for the harness
    #[arg(long)]
    pub model: Option<String>,
    /// Minimum iterations before stopping
    #[arg(long = "min-iterations", default_value_t = 1)]
    pub min_iterations: u32,
    /// Maximum iterations (default: unlimited)
    #[arg(long = "max-iterations")]
    pub max_iterations: Option<u32>,
    /// Completion promise token
    #[arg(long = "completion-promise", default_value = "COMPLETE")]
    pub completion_promise: String,
    /// Skip completion validation (tasks + project checks/tests)
    ///
    /// When set, Ralph trusts the completion promise and exits immediately.
    #[arg(long = "skip-validation")]
    pub skip_validation: bool,
    /// Extra validation command to run on completion promise
    ///
    /// Runs after the project validation steps.
    #[arg(long = "validation-command")]
    pub validation_command: Option<String>,
    /// Exit Ralph immediately if harness exits non-zero
    #[arg(long = "exit-on-error")]
    pub exit_on_error: bool,
    /// Maximum non-zero harness exits before failing (default: 10)
    #[arg(long = "error-threshold")]
    pub error_threshold: Option<u32>,
    /// Allow all tool actions (dangerous)
    #[arg(long = "allow-all", alias = "yolo", alias = "dangerously-allow-all")]
    pub allow_all: bool,
    /// Do not create git commits per iteration
    #[arg(long = "no-commit")]
    pub no_commit: bool,
    /// Show current Ralph state for the change
    #[arg(long)]
    pub status: bool,
    /// Append extra context to the Ralph loop
    #[arg(long = "add-context")]
    pub add_context: Option<String>,
    /// Clear the Ralph loop context file
    #[arg(long = "clear-context")]
    pub clear_context: bool,
    /// Do not prompt for selections
    #[arg(long = "no-interactive")]
    pub no_interactive: bool,
    /// Verbose output
    #[arg(short = 'v', long)]
    pub verbose: bool,
    /// Hidden testing flag
    #[arg(long = "stub-script", hide = true)]
    pub stub_script: Option<String>,
    /// Inactivity timeout (e.g. 15m)
    #[arg(long = "timeout")]
    pub timeout: Option<String>,
    /// Read prompt text from a file
    #[arg(long = "file", value_name = "FILE")]
    pub file: Option<String>,
    /// Prompt text
    #[arg(value_name = "PROMPT", num_args = 0.., trailing_var_arg = true)]
    pub prompt: Vec<String>,
}

#[cfg(test)]
mod ralph_tests;
