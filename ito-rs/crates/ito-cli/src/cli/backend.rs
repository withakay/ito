use clap::{Args, Subcommand};

/// Hidden deprecated top-level `serve-api` argument capture.
#[derive(Args, Debug, Clone)]
#[command(disable_help_flag = true, disable_help_subcommand = true)]
pub struct RemovedServeApiArgs {
    /// Trailing arguments passed to the removed command.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

/// Arguments for `ito backend serve`.
#[derive(Args, Debug, Clone)]
pub struct ServeArgs {
    /// Generate auth tokens and write them to the global config file, then exit.
    #[arg(long, conflicts_with = "service")]
    pub init: bool,

    /// Ensure backend auth exists for unattended service startup, then run.
    #[arg(long, conflicts_with = "init")]
    pub service: bool,

    /// Port to listen on (default: 9010)
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Address to bind to (default: 127.0.0.1)
    #[arg(short, long)]
    pub bind: Option<String>,

    /// Root directory for backend-managed project data.
    #[arg(long)]
    pub data_dir: Option<String>,

    /// Admin bearer token with full access to all projects.
    #[arg(long)]
    pub admin_token: Vec<String>,

    /// Secret seed for deriving per-project tokens via HMAC-SHA256.
    #[arg(long)]
    pub token_seed: Option<String>,

    /// Allow an organization (can be specified multiple times).
    #[arg(long)]
    pub allow_org: Vec<String>,

    /// Path to a TOML/JSON config file for full backend server configuration.
    #[arg(long)]
    pub config: Option<String>,
}

/// Backend client management commands.
#[derive(Args, Debug, Clone)]
pub struct BackendArgs {
    #[command(subcommand)]
    pub action: BackendAction,
}

/// Backend subcommands.
#[derive(Subcommand, Debug, Clone)]
pub enum BackendAction {
    /// Start the backend state API server
    ///
    /// Starts the multi-tenant backend server using the canonical backend
    /// command path.
    ///
    /// Examples:
    ///   ito backend serve
    ///   ito backend serve --service
    ///   ito backend serve --port 8080 --bind 0.0.0.0
    ///   ito backend serve --admin-token my-secret
    #[command(verbatim_doc_comment)]
    Serve(ServeArgs),

    /// Check backend configuration, connectivity, and authentication
    ///
    /// Validates that:
    ///   1. Backend mode is enabled in config
    ///   2. Required fields (token, org, repo) are configured
    ///   3. Server health and readiness endpoints respond
    ///   4. Authentication token is valid for the configured project
    ///
    /// Exit codes:
    ///   0 = backend disabled (informational) or fully healthy
    ///   1 = configuration error, server unreachable, or auth failure
    ///
    /// Examples:
    ///   ito backend status
    ///   ito backend status --json
    #[command(verbatim_doc_comment, visible_alias = "st")]
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Generate a project-scoped bearer token from an HMAC seed
    ///
    /// Derives a token using HMAC-SHA256(seed, "{org}/{repo}"). The token
    /// is printed to stdout; guidance is printed to stderr.
    ///
    /// Resolution order for seed: ITO_BACKEND_TOKEN_SEED env > --seed flag > global config
    /// Resolution order for org/repo: env vars > flags > project config > interactive prompt
    ///
    /// Examples:
    ///   ito backend generate-token
    ///   ito backend generate-token --seed my-seed --org acme --repo widgets
    #[command(verbatim_doc_comment, visible_alias = "gt")]
    GenerateToken {
        /// HMAC seed for token derivation (overrides config)
        #[arg(long)]
        seed: Option<String>,

        /// Organization namespace
        #[arg(long)]
        org: Option<String>,

        /// Repository namespace
        #[arg(long)]
        repo: Option<String>,
    },

    /// Import local active and archived changes into backend-managed state
    ///
    /// Scans local `.ito/changes/` and `.ito/changes/archive/`, then imports
    /// active and archived change artifacts into the configured backend project.
    ///
    /// Examples:
    ///   ito backend import
    ///   ito backend import --dry-run
    #[command(verbatim_doc_comment, visible_alias = "im")]
    Import {
        /// Preview import scope without mutating backend state
        #[arg(long)]
        dry_run: bool,
    },
}
