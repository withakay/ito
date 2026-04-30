use clap::{Args, Subcommand, ValueEnum};

#[derive(Subcommand, Debug, Clone)]
pub enum ValidateCommand {
    /// Validate a module
    Module {
        /// Module id
        #[arg(value_name = "MODULE")]
        module_id: Option<String>,
    },

    /// Run repository-level validation rules driven by `.ito/config.json`.
    ///
    /// See `ito validate repo --help` for the full flag list. This is the
    /// command invoked by the `pre-commit` hook installed via the
    /// `ito-update-repo` skill.
    Repo(RepoValidateArgs),
}

/// Arguments for `ito validate repo`.
#[derive(Args, Debug, Clone, Default)]
pub struct RepoValidateArgs {
    /// Read staged files from `git diff --cached --name-only -z` and pass
    /// them to staged-aware rules (e.g. `coordination/staged-symlinked-paths`,
    /// `worktrees/no-write-on-control`).
    #[arg(long)]
    pub staged: bool,

    /// Treat `WARNING`-level issues as failures (exit code 1).
    #[arg(long)]
    pub strict: bool,

    /// Emit the result as JSON matching the standard `ValidationReport`
    /// envelope.
    #[arg(long)]
    pub json: bool,

    /// Run only the named rule(s); repeat for multiple. Mutually exclusive
    /// with `--no-rule` (enforced at runtime so the error exits with the
    /// documented code 2).
    #[arg(long = "rule", value_name = "RULE_ID")]
    pub rule: Vec<String>,

    /// Skip the named rule(s); repeat for multiple. Mutually exclusive
    /// with `--rule`.
    #[arg(long = "no-rule", value_name = "RULE_ID")]
    pub no_rule: Vec<String>,

    /// Print every built-in rule with its activation status and gate, then
    /// exit 0 without running any check.
    #[arg(long = "list-rules")]
    pub list_rules: bool,

    /// Print the metadata for a single rule (id, severity, gate, description),
    /// then exit 0 without running any check.
    #[arg(long, value_name = "RULE_ID")]
    pub explain: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ValidateItemType {
    Change,
    Spec,
    Module,
}
