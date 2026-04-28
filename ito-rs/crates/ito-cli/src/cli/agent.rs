use clap::{Args, Subcommand};

const AGENT_INSTRUCTION_AFTER_HELP: &str = concat!(
    "Artifacts:\n",
    "  bootstrap                          Generate a tool bootstrap preamble\n",
    "  project-setup                      Guide for setting up a new project\n",
    "  backend                            Backend server and client configuration guide\n",
    "  worktrees                          Guide for git worktree workflow (config-driven)\n",
    "  repo-sweep                         Scan for old-only ID format assumptions in prompts and templates\n",
    "  migrate-to-coordination-worktree   Guide for migrating from embedded to worktree storage\n",
    "  orchestrate                        Orchestrate applying a set of changes via an orchestrator agent\n",
    "  manifesto                          Generate a strict Ito manifesto for prompt-only execution\n",
    "  proposal                           Show the change proposal\n",
    "  specs                              Show the specification deltas\n",
    "  tasks                              Show the implementation task list\n",
    "  apply                              Show implementation instructions\n",
    "  review                             Show review instructions\n",
    "  archive                            Show archive instructions\n",
    "  finish                             Cleanup worktrees and branches after merge\n",
    "  memory-capture                     Capture durable knowledge through configured memory\n",
    "  memory-search                      Search configured memory for ranked matches\n",
    "  memory-query                       Query configured memory for a synthesized answer\n",
    "\n",
    "Examples:\n",
    "  ito agent instruction bootstrap --tool opencode\n",
    "  ito agent instruction project-setup\n",
    "  ito agent instruction backend\n",
    "  ito agent instruction worktrees\n",
    "  ito agent instruction repo-sweep\n",
    "  ito agent instruction migrate-to-coordination-worktree\n",
    "  ito agent instruction orchestrate\n",
    "  ito agent instruction manifesto\n",
    "  ito agent instruction manifesto --variant full --profile proposal-only\n",
    "  ito agent instruction manifesto --change 005-08_migrate-cli-to-clap --variant full --operation apply\n",
    "  ito agent instruction proposal --change 005-08_migrate-cli-to-clap\n",
    "  ito agent instruction apply --change 005-08_migrate-cli-to-clap\n",
    "  ito agent instruction archive\n",
    "  ito agent instruction archive --change 005-08_migrate-cli-to-clap\n",
    "  ito agent instruction finish --change 005-08_migrate-cli-to-clap\n",
    "  ito agent instruction memory-capture --context \"Decision and rationale\" --file docs/config.md\n",
    "  ito agent instruction memory-search --query \"archive workflow\" --limit 5\n",
    "  ito agent instruction memory-query --query \"How should agents capture memories?\"",
);

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
    Instruction(Box<AgentInstructionArgs>),

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

/// Parsed arguments for `ito agent instruction <artifact>`.
///
/// Used by both the clap-based and legacy string-based instruction handlers.
/// See [`to_argv`](Self::to_argv) for round-tripping back to a raw argument vector.
#[derive(Args, Debug, Clone)]
#[command(after_help = AGENT_INSTRUCTION_AFTER_HELP)]
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

    /// Manifesto output variant (light|full)
    #[arg(long)]
    pub variant: Option<String>,

    /// Manifesto capability profile (planning|proposal-only|review-only|apply|archive|full)
    #[arg(long)]
    pub profile: Option<String>,

    /// Manifesto operation selector for full renders
    #[arg(long)]
    pub operation: Option<String>,

    // ---- memory-* artifact inputs --------------------------------------
    /// Free-form context for `memory-capture`
    #[arg(long)]
    pub context: Option<String>,

    /// File path for `memory-capture` (repeatable)
    #[arg(long = "file", action = clap::ArgAction::Append)]
    pub file: Vec<String>,

    /// Folder path for `memory-capture` (repeatable)
    #[arg(long = "folder", action = clap::ArgAction::Append)]
    pub folder: Vec<String>,

    /// Search/query input for `memory-search` and `memory-query`
    #[arg(long)]
    pub query: Option<String>,

    /// Limit for `memory-search` (positive integer)
    #[arg(long)]
    pub limit: Option<u64>,

    /// Scope for `memory-search`
    #[arg(long)]
    pub scope: Option<String>,
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
            variant,
            profile,
            operation,
            context,
            file,
            folder,
            query,
            limit,
            scope,
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
        if let Some(v) = variant {
            argv.push("--variant".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = profile {
            argv.push("--profile".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = operation {
            argv.push("--operation".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = context {
            argv.push("--context".to_string());
            argv.push(v.clone());
        }
        for v in file {
            argv.push("--file".to_string());
            argv.push(v.clone());
        }
        for v in folder {
            argv.push("--folder".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = query {
            argv.push("--query".to_string());
            argv.push(v.clone());
        }
        if let Some(v) = limit {
            argv.push("--limit".to_string());
            argv.push(v.to_string());
        }
        if let Some(v) = scope {
            argv.push("--scope".to_string());
            argv.push(v.clone());
        }
        argv
    }
}
