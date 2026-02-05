#![allow(dead_code)]

use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Color, Style};
use clap::{Args, Parser, Subcommand, ValueEnum};

fn cli_styles() -> Styles {
    Styles::styled()
        .header(Style::new().bold())
        .usage(Style::new().bold())
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "ito",
    version,
    about = "AI-native system for spec-driven development",
    long_about = None,
    after_help = "Run 'ito help --all' for the complete CLI reference.",
    styles = cli_styles(),
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Disable color output
    #[arg(long = "no-color", global = true)]
    pub no_color: bool,

    /// Print the full CLI reference (equivalent to `ito help --all`)
    #[arg(long = "help-all", global = true)]
    pub help_all: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    // ─── Change Lifecycle ───────────────────────────────────────────────────────
    /// Create a new module or change proposal
    ///
    /// Modules group related changes and specs. Changes are the unit of work
    /// in Ito - each change has a proposal, spec deltas, and implementation tasks.
    ///
    /// Examples:
    ///   ito create module my-feature
    ///   ito create change add-auth --module 001
    #[command(verbatim_doc_comment)]
    Create(CreateArgs),

    /// List changes, specs, or modules with status summaries
    ///
    /// By default lists changes sorted by most recent. Use --specs or --modules
    /// to list other item types. Use --ready to filter to changes ready for
    /// implementation (have proposal, specs, tasks, and pending work).
    ///
    /// Examples:
    ///   ito list
    ///   ito list --ready
    ///   ito list --pending
    ///   ito list --partial
    ///   ito list --completed
    ///   ito list --specs
    ///   ito list --modules --json
    #[command(verbatim_doc_comment)]
    List(ListArgs),

    /// Display details of a change, spec, or module
    ///
    /// Shows the full content and metadata for an item. For changes, displays
    /// the proposal, spec deltas, and task progress. Use --json for structured output.
    ///
    /// Examples:
    ///   ito show 005-01_add-auth
    ///   ito show --type spec auth-service
    ///   ito show module 005
    #[command(verbatim_doc_comment)]
    Show(ShowArgs),

    /// Check completion status of change artifacts
    ///
    /// Displays which artifacts (proposal, specs, tasks) are complete for a change.
    /// Useful for tracking progress before archiving.
    ///
    /// Examples:
    ///   ito status --change 005-01_add-auth
    #[command(verbatim_doc_comment)]
    Status(StatusArgs),

    /// Check changes, specs, and modules for errors and warnings
    ///
    /// Validates markdown structure, required fields, and cross-references.
    /// Use --strict to treat warnings as errors. Returns non-zero on failure.
    ///
    /// Examples:
    ///   ito validate --all
    ///   ito validate 005-01_add-auth
    ///   ito validate --specs --strict
    #[command(verbatim_doc_comment)]
    Validate(ValidateArgs),

    /// Move a completed change to archive and update main specs
    ///
    /// Archives the change directory and merges spec deltas into the main specs.
    /// Validates the change is complete before archiving. Use -y to skip prompts.
    ///
    /// Examples:
    ///   ito archive 005-01_add-auth
    ///   ito archive 005-01_add-auth -y --skip-specs
    #[command(verbatim_doc_comment)]
    Archive(ArchiveArgs),

    /// Split a large change into smaller changes [not implemented]
    #[command(hide = true)]
    Split(SplitArgs),

    // ─── Task Management ────────────────────────────────────────────────────────
    /// Manage implementation tasks for a change
    ///
    /// Track task progress through status, start, complete, and shelve actions.
    /// Tasks are organized in waves for phased implementation. Use 'ready' to
    /// find actionable tasks across changes.
    ///
    /// Examples:
    ///   ito tasks status 005-01_add-auth
    ///   ito tasks next 005-01_add-auth
    ///   ito tasks ready
    ///   ito tasks ready 005-01_add-auth
    ///   ito tasks start 005-01_add-auth 1.1
    ///   ito tasks complete 005-01_add-auth 1.1
    #[command(verbatim_doc_comment)]
    Tasks(TasksArgs),

    /// Initialize and track project roadmap
    ///
    /// Creates planning structure for milestones and tracks overall progress
    /// across multiple changes and modules.
    ///
    /// Examples:
    ///   ito plan init
    ///   ito plan status
    #[command(verbatim_doc_comment)]
    Plan(PlanArgs),

    /// Track session state and working context
    ///
    /// Record decisions, blockers, notes, and current focus in STATE.md.
    /// Helps maintain context across coding sessions.
    ///
    /// Examples:
    ///   ito state show
    ///   ito state focus "implementing auth flow"
    ///   ito state decision "using JWT for tokens"
    ///   ito state blocker "waiting on API spec"
    #[command(verbatim_doc_comment)]
    State(StateArgs),

    // ─── AI Automation ──────────────────────────────────────────────────────────
    /// Generate instructions and context for AI coding agents
    ///
    /// Produces structured output for AI tools like Claude Code, Codex, or OpenCode.
    /// Includes change proposals, specs, tasks, and implementation guidance.
    ///
    /// Examples:
    ///   ito agent instruction bootstrap --tool claude
    ///   ito agent instruction apply --change 005-01_add-auth
    ///   ito agent instruction tasks --change 005-01_add-auth
    #[command(verbatim_doc_comment)]
    Agent(AgentArgs),

    /// Run an AI agent loop to implement a change
    ///
    /// Iteratively runs an AI coding agent (OpenCode, Claude, etc.) to implement
    /// a change proposal. Monitors progress and commits incrementally.
    ///
    /// Examples:
    ///   ito ralph --change 005-01_add-auth
    ///   ito ralph --change 005-01_add-auth --harness claude --max-iterations 5
    #[command(verbatim_doc_comment)]
    Ralph(RalphArgs),

    /// Deprecated alias for `ralph`
    #[command(hide = true)]
    Loop(RalphArgs),

    // ─── Project Setup ──────────────────────────────────────────────────────────
    /// Set up Ito in a project
    ///
    /// Creates the .ito/ directory structure and optionally configures
    /// AI tool integrations (Claude Code, Codex, OpenCode).
    ///
    /// Examples:
    ///   ito init
    ///   ito init --tools claude,codex
    ///   ito init --tools all --force
    #[command(verbatim_doc_comment)]
    Init(InitArgs),

    /// Refresh Ito instruction files and AI tool configs
    ///
    /// Updates agent instructions, skills, and tool configurations to the
    /// latest versions without reinitializing the project.
    ///
    /// Examples:
    ///   ito update
    #[command(verbatim_doc_comment)]
    Update(UpdateArgs),

    /// Read and write global Ito settings
    ///
    /// Manages configuration in ~/.config/ito/config.json. Settings include
    /// default schemas, tool preferences, and execution options.
    ///
    /// Examples:
    ///   ito config path
    ///   ito config get defaults.schema
    ///   ito config set defaults.schema "spec-driven"
    #[command(verbatim_doc_comment)]
    Config(ConfigArgs),

    /// Initialize and inspect workflow definitions
    ///
    /// Workflows define the artifact structure for changes (proposal, specs, tasks).
    /// Use 'init' to create custom workflows or 'list' to see available schemas.
    ///
    /// Examples:
    ///   ito workflow list
    ///   ito workflow show spec-driven
    ///   ito workflow init
    #[command(verbatim_doc_comment)]
    Workflow(WorkflowArgs),

    // ─── Local Docs Server ──────────────────────────────────────────────────────
    /// Serve local Ito artifacts and docs over HTTP
    ///
    /// Starts a per-project local server for browsing `.ito/` artifacts and
    /// project docs with clickable navigation.
    ///
    /// Examples:
    ///   ito serve start
    ///   ito serve status
    ///   ito serve stop
    #[command(verbatim_doc_comment)]
    Serve(ServeArgs),

    // ─── Utilities ──────────────────────────────────────────────────────────────
    /// Display an interactive dashboard [not implemented]
    #[command(hide = true)]
    Dashboard(DashboardArgs),

    /// Output shell completion scripts
    ///
    /// Generates completion scripts for bash, zsh, fish, or powershell.
    /// Add to your shell config for tab completion of ito commands.
    ///
    /// Examples:
    ///   ito completions bash >> ~/.bashrc
    ///   ito completions zsh >> ~/.zshrc
    ///   ito completions fish > ~/.config/fish/completions/ito.fish
    #[command(verbatim_doc_comment)]
    Completions(CompletionsArgs),

    /// Display command execution counts and history
    ///
    /// Shows statistics about ito command usage in this project.
    /// Useful for understanding workflow patterns.
    Stats(StatsArgs),

    /// Show help for ito commands
    ///
    /// Displays help for a specific command or the full CLI reference.
    /// Use --all to see all commands and options in one view.
    ///
    /// Examples:
    ///   ito help tasks
    ///   ito help --all
    ///   ito help agent instruction
    #[command(verbatim_doc_comment)]
    Help(HelpArgs),

    // ─── Hidden / Deprecated ────────────────────────────────────────────────────
    /// Deprecated alias for `create change`
    #[command(hide = true)]
    New(NewArgs),
}

/// Local docs server with file browser and editor.
#[derive(Args, Debug, Clone)]
pub struct ServeArgs {
    #[command(subcommand)]
    pub action: Option<ServeAction>,

    /// Port to listen on (default: 9009)
    #[arg(short, long, global = true)]
    pub port: Option<u16>,

    /// Address to bind to (default: 127.0.0.1)
    #[arg(short, long, global = true, conflicts_with = "tailscale")]
    pub bind: Option<String>,

    /// Bind to the Tailscale IP address (requires tailscale CLI)
    #[arg(short, long, global = true)]
    pub tailscale: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ServeAction {
    /// Start the server (default if no subcommand)
    Start,
}

/// Deprecated alias for `create change`.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct NewArgs {
    #[command(subcommand)]
    pub action: Option<NewAction>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum NewAction {
    /// Create a change
    Change {
        /// Change name (kebab-case)
        name: Option<String>,

        /// Workflow schema name (default: spec-driven)
        #[arg(long)]
        schema: Option<String>,

        /// Module id (default: 000)
        #[arg(long)]
        module: Option<String>,

        /// Description (writes README.md)
        #[arg(long)]
        description: Option<String>,
    },

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

/// Show local execution usage stats.
#[derive(Args, Debug, Clone)]
pub struct StatsArgs {}

/// Project planning tools.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct PlanArgs {
    #[command(subcommand)]
    pub action: Option<PlanAction>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PlanAction {
    /// Initialize planning structure
    Init,

    /// Show current milestone progress
    Status,
}

/// View and update planning/STATE.md.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct StateArgs {
    #[command(subcommand)]
    pub action: Option<StateAction>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum StateAction {
    /// Show current project state
    Show,

    /// Record a decision
    Decision {
        /// Text to record
        #[arg(value_name = "TEXT", num_args = 0.., trailing_var_arg = true)]
        text: Vec<String>,
    },

    /// Record a blocker
    Blocker {
        /// Text to record
        #[arg(value_name = "TEXT", num_args = 0.., trailing_var_arg = true)]
        text: Vec<String>,
    },

    /// Add a session note
    Note {
        /// Text to record
        #[arg(value_name = "TEXT", num_args = 0.., trailing_var_arg = true)]
        text: Vec<String>,
    },

    /// Set current focus
    Focus {
        /// Text to record
        #[arg(value_name = "TEXT", num_args = 0.., trailing_var_arg = true)]
        text: Vec<String>,
    },

    /// Add an open question
    Question {
        /// Text to record
        #[arg(value_name = "TEXT", num_args = 0.., trailing_var_arg = true)]
        text: Vec<String>,
    },
}

/// Manage and run workflows.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct WorkflowArgs {
    #[command(subcommand)]
    pub action: Option<WorkflowAction>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WorkflowAction {
    /// Initialize workflow templates
    Init,

    /// List available workflows
    List,

    /// Show workflow details
    Show {
        /// Workflow name
        #[arg(value_name = "WORKFLOW", num_args = 0.., trailing_var_arg = true)]
        workflow_name: Vec<String>,
    },
}

/// Initialize Ito instruction files in a project directory.
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Configure AI tools non-interactively (all, none, or comma-separated ids)
    #[arg(long)]
    pub tools: Option<String>,

    /// Overwrite existing tool files without prompting
    #[arg(short = 'f', long)]
    pub force: bool,

    /// Override HOME used for locating global Ito config (for parity/testing)
    #[arg(long, value_name = "HOME")]
    pub home: Option<std::path::PathBuf>,

    /// Target directory (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,
}

/// Display help information.
#[derive(Args, Debug, Clone)]
pub struct HelpArgs {
    /// Show help for all commands
    #[arg(long)]
    pub all: bool,

    /// Output as JSON (with --all)
    #[arg(long)]
    pub json: bool,

    /// Command path to show help for (e.g., `ito help tasks`)
    #[arg(value_name = "COMMAND", num_args = 0..)]
    pub command: Vec<String>,
}

/// Commands that generate machine-readable output for AI agents.
#[derive(Args, Debug, Clone)]
pub struct AgentArgs {
    #[command(subcommand)]
    pub command: Option<AgentCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AgentCommand {
    /// Generate enriched instructions
    Instruction(AgentInstructionArgs),

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Args, Debug, Clone)]
#[command(
    after_help = "Artifacts:\n  bootstrap      Generate a tool bootstrap preamble\n  project-setup  Guide for setting up a new project\n  proposal       Show the change proposal\n  specs          Show the specification deltas\n  tasks          Show the implementation task list\n  apply          Show implementation instructions\n  review         Show review instructions\n  archive        Show archive instructions\n\nExamples:\n  ito agent instruction bootstrap --tool opencode\n  ito agent instruction project-setup\n  ito agent instruction proposal --change 005-08_migrate-cli-to-clap\n  ito agent instruction apply --change 005-08_migrate-cli-to-clap"
)]
pub struct AgentInstructionArgs {
    /// Artifact id (e.g. bootstrap, apply, proposal)
    #[arg(value_name = "ARTIFACT")]
    pub artifact: String,

    /// Change id (directory name)
    #[arg(long)]
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

/// View and modify global Ito configuration.
#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: Option<ConfigCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommand {
    /// Print config file path
    Path(ConfigCommonArgs),

    /// Print config JSON
    List(ConfigCommonArgs),

    /// Read value by path
    Get {
        /// Key path (dot-separated)
        key: String,

        #[command(flatten)]
        common: ConfigCommonArgs,
    },

    /// Set value by path
    Set {
        /// Key path (dot-separated)
        key: String,

        /// Value (JSON or string)
        value: String,

        #[command(flatten)]
        common: ConfigCommonArgs,
    },

    /// Remove value by path
    Unset {
        /// Key path (dot-separated)
        key: String,

        #[command(flatten)]
        common: ConfigCommonArgs,
    },

    /// Print JSON schema for Ito config
    Schema {
        /// Write schema to file instead of stdout
        #[arg(long, value_name = "PATH")]
        output: Option<std::path::PathBuf>,
    },

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Args, Debug, Clone, Default)]
pub struct ConfigCommonArgs {
    /// Treat `value` as a string.
    #[arg(long)]
    pub string: bool,
}

/// Create items.
#[derive(Args, Debug, Clone)]
pub struct CreateArgs {
    #[command(subcommand)]
    pub action: Option<CreateAction>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CreateAction {
    /// Create a module
    Module {
        /// Module name (kebab-case)
        name: Option<String>,

        /// Module scope (comma-separated, default: "*")
        #[arg(long)]
        scope: Option<String>,

        /// Module dependencies (comma-separated module ids)
        #[arg(long = "depends-on")]
        depends_on: Option<String>,
    },

    /// Create a change
    Change {
        /// Change name (kebab-case)
        name: Option<String>,

        /// Workflow schema name (default: spec-driven)
        #[arg(long)]
        schema: Option<String>,

        /// Module id (default: 000)
        #[arg(long)]
        module: Option<String>,

        /// Description (writes README.md)
        #[arg(long)]
        description: Option<String>,
    },

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

/// Show a change, spec, or module.
#[derive(Args, Debug, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ShowArgs {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Type: change or spec
    #[arg(long = "type", value_enum)]
    pub typ: Option<ShowItemType>,

    /// Disable interactive prompts
    #[arg(long = "no-interactive")]
    pub no_interactive: bool,

    /// Change JSON only: only include deltas (deprecated)
    #[arg(long = "deltas-only")]
    pub deltas_only: bool,

    /// Change JSON only: only include deltas (deprecated)
    #[arg(long = "requirements-only")]
    pub requirements_only: bool,

    /// Spec JSON only: exclude scenarios
    #[arg(long)]
    pub requirements: bool,

    /// Spec JSON only: exclude scenarios
    #[arg(long = "no-scenarios")]
    pub no_scenarios: bool,

    /// Spec JSON only: select requirement (1-based)
    #[arg(short = 'r', long = "requirement")]
    pub requirement: Option<usize>,

    #[command(subcommand)]
    pub command: Option<ShowCommand>,

    /// Item name (change id or spec id)
    #[arg(value_name = "ITEM")]
    pub item: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ShowCommand {
    /// Show a module
    Module(ShowModuleArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ShowModuleArgs {
    /// Output as JSON (not implemented)
    #[arg(long)]
    pub json: bool,

    /// Module id
    pub module_id: String,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ShowItemType {
    Change,
    Spec,
}

/// List items (changes by default).
#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    /// List specs instead of changes
    #[arg(long)]
    pub specs: bool,

    /// List changes explicitly (default)
    #[arg(long)]
    pub changes: bool,

    /// List modules instead of changes
    #[arg(long)]
    pub modules: bool,

    /// Filter to changes ready for implementation (has proposal, specs, tasks, and pending work)
    #[arg(long)]
    pub ready: bool,

    /// Filter to completed changes (all tasks done)
    #[arg(long, conflicts_with_all = ["partial", "pending"])]
    pub completed: bool,

    /// Filter to partially complete changes (some but not all tasks done)
    #[arg(long, conflicts_with_all = ["completed", "pending"])]
    pub partial: bool,

    /// Filter to pending changes (no tasks started yet)
    #[arg(long, conflicts_with_all = ["completed", "partial"])]
    pub pending: bool,

    /// Sort order
    #[arg(long, value_enum, default_value_t = ListSortOrder::Recent)]
    pub sort: ListSortOrder,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ListSortOrder {
    Recent,
    Name,
}

#[derive(Args, Debug, Clone)]
pub struct CompletionsArgs {
    /// Shell type
    pub shell: CompletionShell,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    #[value(name = "powershell", alias = "power-shell")]
    PowerShell,
}

/// Track execution tasks for a change.
#[derive(Args, Debug, Clone)]
pub struct TasksArgs {
    #[command(subcommand)]
    pub action: Option<TasksAction>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum TasksAction {
    /// Create enhanced tasks.md
    Init {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
    },

    /// Show task progress
    Status {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,

        /// Wave number (optional)
        #[arg(long)]
        wave: Option<u32>,
    },

    /// Show the next available task
    Next {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
    },

    /// Show all ready tasks (pending tasks in earliest incomplete wave)
    ///
    /// Without a change argument, shows ready tasks across all changes.
    /// With a change argument, shows ready tasks for that specific change.
    ///
    /// Examples:
    ///   ito tasks ready
    ///   ito tasks ready 005-01_my-change
    ///   ito tasks ready --json
    #[command(verbatim_doc_comment)]
    Ready {
        /// Change id (optional - if omitted, shows tasks from all changes)
        change_id: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Mark a task in-progress
    Start {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
        /// Task id (e.g. 1.1)
        task_id: String,
    },

    /// Mark a task complete
    Complete {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
        /// Task id (e.g. 1.1)
        task_id: String,
    },

    /// Shelve a task (reversible)
    Shelve {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
        /// Task id (e.g. 1.1)
        task_id: String,
    },

    /// Restore a shelved task to pending
    Unshelve {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
        /// Task id (e.g. 1.1)
        task_id: String,
    },

    /// Add a new task (enhanced only)
    Add {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
        /// Task name
        task_name: String,
        /// Wave number (default: 1)
        #[arg(long, default_value_t = 1)]
        wave: u32,
    },

    /// Print tasks.md
    Show {
        /// Change id (e.g. 005-08_migrate-cli-to-clap)
        change_id: String,
    },

    /// Forward unknown subcommands to legacy handler
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Args, Debug, Clone)]
pub struct RawArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

/// Display an interactive dashboard of specs and changes.
///
/// Note: This command is currently a stub in `ito-cli`.
#[derive(Args, Debug, Clone)]
pub struct DashboardArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

/// Update Ito instruction files.
#[derive(Args, Debug, Clone)]
pub struct UpdateArgs {
    /// Output as JSON (not implemented yet)
    #[arg(long)]
    pub json: bool,

    /// Target directory (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,
}

/// Archive a completed change and update main specs.
#[derive(Args, Debug, Clone)]
pub struct ArchiveArgs {
    /// Change id (directory name)
    #[arg(value_name = "CHANGE")]
    pub change: Option<String>,

    /// Skip confirmation prompts
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,

    /// Skip spec updates
    #[arg(long = "skip-specs")]
    pub skip_specs: bool,

    /// Skip validation checks
    #[arg(long = "no-validate")]
    pub no_validate: bool,
}

/// Display artifact completion status for a change.
#[derive(Args, Debug, Clone)]
pub struct StatusArgs {
    /// Change id (directory name)
    #[arg(long)]
    pub change: Option<String>,

    /// Workflow schema name
    #[arg(long)]
    pub schema: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

/// Validate changes, specs, and modules.
#[derive(Args, Debug, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ValidateArgs {
    #[command(subcommand)]
    pub command: Option<ValidateCommand>,

    /// Validate everything
    #[arg(long)]
    pub all: bool,

    /// Validate changes
    #[arg(long)]
    pub changes: bool,

    /// Validate specs
    #[arg(long)]
    pub specs: bool,

    /// Validate modules
    #[arg(long)]
    pub modules: bool,

    /// Validate a module by id
    #[arg(long)]
    pub module: Option<String>,

    /// Type: change, spec, or module
    #[arg(long = "type", value_enum)]
    pub typ: Option<ValidateItemType>,

    /// Treat warnings as errors
    #[arg(long)]
    pub strict: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Concurrency (default: 6)
    #[arg(long, default_value_t = 6)]
    pub concurrency: u32,

    /// Disable interactive prompts
    #[arg(long = "no-interactive")]
    pub no_interactive: bool,

    /// Item name (change id or spec id)
    #[arg(value_name = "ITEM")]
    pub item: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ValidateCommand {
    /// Validate a module
    Module {
        /// Module id
        #[arg(value_name = "MODULE")]
        module_id: Option<String>,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ValidateItemType {
    Change,
    Spec,
    Module,
}

/// Run iterative AI loop against a change proposal.
#[derive(Args, Debug, Clone)]
pub struct RalphArgs {
    /// Target a specific change
    #[arg(long)]
    pub change: Option<String>,

    /// Target a module (selects a change)
    #[arg(long)]
    pub module: Option<String>,

    /// Harness to run
    #[arg(long, default_value = "opencode")]
    pub harness: String,

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

    /// Prompt text
    #[arg(value_name = "PROMPT", num_args = 0.., trailing_var_arg = true)]
    pub prompt: Vec<String>,
}

/// Split a large change into smaller changes.
///
/// Note: This command is currently a stub in `ito-cli`.
#[derive(Args, Debug, Clone)]
pub struct SplitArgs {
    /// Change id (directory name)
    #[arg(value_name = "CHANGE")]
    pub change: Option<String>,
}
