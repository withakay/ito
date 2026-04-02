use clap::{Args, Subcommand};

/// Commands that generate machine-readable output for AI agents.
#[derive(Args, Debug, Clone)]
pub struct AgentArgs {
    #[command(subcommand)]
    pub command: Option<AgentCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AgentCommand {
    /// Generate enriched instructions
    #[command(visible_alias = "in")]
    Instruction(AgentInstructionArgs),

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

/// Parsed arguments for `ito agent instruction <artifact>`.
///
/// Used by both the clap-based and legacy string-based instruction handlers.
/// See [`to_argv`](Self::to_argv) for round-tripping back to a raw argument vector.
#[derive(Args, Debug, Clone)]
#[command(
    after_help = "Artifacts:\n  bootstrap      Generate a tool bootstrap preamble\n  project-setup  Guide for setting up a new project\n  backend        Backend server and client configuration guide\n  worktrees      Guide for git worktree workflow (config-driven)\n  repo-sweep     Scan for old-only ID format assumptions in prompts and templates\n  proposal       Show the change proposal\n  specs          Show the specification deltas\n  tasks          Show the implementation task list\n  apply          Show implementation instructions\n  review         Show review instructions\n  archive        Show archive instructions\n  finish         Cleanup worktrees and branches after merge\n\nExamples:\n  ito agent instruction bootstrap --tool opencode\n  ito agent instruction project-setup\n  ito agent instruction backend\n  ito agent instruction worktrees\n  ito agent instruction repo-sweep\n  ito agent instruction proposal --change 005-08_migrate-cli-to-clap\n  ito agent instruction apply --change 005-08_migrate-cli-to-clap\n  ito agent instruction archive\n  ito agent instruction archive --change 005-08_migrate-cli-to-clap\n  ito agent instruction finish --change 005-08_migrate-cli-to-clap"
)]
pub struct AgentInstructionArgs {
    /// Artifact id (e.g. bootstrap, apply, proposal)
    #[arg(value_name = "ARTIFACT")]
    pub artifact: String,

    /// Change id (directory name)
    #[arg(short = 'c', long)]
    pub change: Option<String>,

    /// Tool name for bootstrap (opencode|claude|codex)
    #[arg(long)]
    pub tool: Option<String>,

    /// Workflow schema name
    #[arg(long)]
    pub schema: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

impl AgentInstructionArgs {
    /// Convert back to a raw argument vector suitable for the legacy string-based handler.
    ///
    /// Uses exhaustive destructuring so adding a field to this struct produces a
    /// compile error here, forcing the new field to be handled.
    pub(crate) fn to_argv(&self) -> Vec<String> {
        let AgentInstructionArgs {
            artifact,
            change,
            tool,
            schema,
            json,
        } = self;

        let mut argv = vec![artifact.clone()];
        if let Some(v) = change {
            argv.push("--change".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = tool {
            argv.push("--tool".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = schema {
            argv.push("--schema".to_string());
            argv.push(v.clone());
        }
        if *json {
            argv.push("--json".to_string());
        }
        argv
    }
}
