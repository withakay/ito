//! Serde models for Ito configuration.
//!
//! These types are deserialized from `config.json` (and merged across multiple
//! layers) and also used to generate JSON schema for editor validation.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Re-export backend server types from the dedicated submodule.
pub use super::backend_types::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Top-level Ito configuration")]
/// Top-level Ito configuration object.
pub struct ItoConfig {
    #[serde(default, rename = "$schema", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Optional JSON schema reference for editor validation")]
    /// Optional `$schema` reference used by editors.
    pub schema: Option<String>,

    #[serde(default, rename = "projectPath")]
    #[schemars(description = "Ito working directory name (defaults to .ito)")]
    /// Override the Ito working directory name (defaults to `.ito`).
    pub project_path: Option<String>,

    #[serde(default)]
    #[schemars(default, description = "Harness-specific configuration")]
    /// Harness-specific configuration.
    pub harnesses: HarnessesConfig,

    #[serde(default)]
    #[schemars(default, description = "Cache configuration")]
    /// Cache configuration.
    pub cache: CacheConfig,

    #[serde(default)]
    #[schemars(default, description = "Global defaults for workflow and tooling")]
    /// Defaults for workflows and tooling.
    pub defaults: DefaultsConfig,

    #[serde(default)]
    #[schemars(default, description = "Worktree workspace configuration")]
    /// Worktree workspace configuration.
    pub worktrees: WorktreesConfig,

    #[serde(default)]
    #[schemars(default, description = "Per-tool preferences")]
    /// Per-tool preferences.
    pub tools: ToolsConfig,

    #[serde(default)]
    #[schemars(default, description = "Change coordination configuration")]
    /// Change coordination configuration.
    pub changes: ChangesConfig,

    #[serde(default)]
    #[schemars(default, description = "Logging configuration")]
    /// Logging configuration for diagnostics and invalid command tracking.
    pub logging: LoggingConfig,

    #[serde(default)]
    #[schemars(default, description = "Audit logging and mirroring configuration")]
    /// Audit logging and mirroring configuration.
    pub audit: AuditConfig,

    #[serde(default)]
    #[schemars(default, description = "Repository runtime configuration")]
    /// Repository runtime configuration for local persistence.
    ///
    /// When backend mode is enabled, remote runtime selection takes precedence
    /// over these settings.
    pub repository: RepositoryRuntimeConfig,

    #[serde(default)]
    #[schemars(default, description = "Backend state API configuration")]
    /// Backend state API configuration.
    pub backend: BackendApiConfig,

    #[serde(default, rename = "backendServer")]
    #[schemars(default, description = "Backend server configuration (multi-tenant)")]
    /// Backend server configuration for hosting the multi-tenant API.
    pub backend_server: BackendServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Repository runtime configuration")]
/// Repository runtime configuration for local persistence selection.
pub struct RepositoryRuntimeConfig {
    #[serde(default)]
    #[schemars(default, description = "Client-side persistence mode")]
    /// Client-side persistence mode (defaults to filesystem).
    ///
    /// Ignored when backend mode is enabled.
    pub mode: RepositoryPersistenceMode,

    #[serde(default)]
    #[schemars(default, description = "Local SQLite configuration")]
    /// Local SQLite configuration (used only when mode is `sqlite`).
    pub sqlite: RepositorySqliteConfig,
}

impl Default for RepositoryRuntimeConfig {
    fn default() -> Self {
        Self {
            mode: RepositoryPersistenceMode::Filesystem,
            sqlite: RepositorySqliteConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[schemars(description = "Client-side repository persistence mode")]
/// Client-side repository persistence mode (local only).
pub enum RepositoryPersistenceMode {
    /// Filesystem-backed repositories.
    #[default]
    Filesystem,
    /// SQLite-backed repositories.
    Sqlite,
}

impl RepositoryPersistenceMode {
    /// Return a stable string identifier for display.
    pub fn as_str(self) -> &'static str {
        match self {
            RepositoryPersistenceMode::Filesystem => "filesystem",
            RepositoryPersistenceMode::Sqlite => "sqlite",
        }
    }

    /// All supported persistence mode values.
    pub const ALL: &'static [&'static str] = &["filesystem", "sqlite"];

    /// Parse a string into a persistence mode, returning `None` for invalid values.
    pub fn parse_value(s: &str) -> Option<Self> {
        match s {
            "filesystem" => Some(Self::Filesystem),
            "sqlite" => Some(Self::Sqlite),
            _ => None,
        }
    }
}

impl std::fmt::Display for RepositoryPersistenceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Local SQLite repository configuration")]
/// Local SQLite repository configuration for client-side persistence.
pub struct RepositorySqliteConfig {
    #[serde(default, rename = "dbPath", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Path to the local SQLite database file")]
    /// Path to the local SQLite database file.
    ///
    /// Required when `repository.mode` is `sqlite`. Relative paths resolve
    /// against the project root.
    pub db_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Audit configuration")]
/// Audit configuration.
pub struct AuditConfig {
    #[serde(default)]
    #[schemars(default, description = "Remote mirroring settings")]
    /// Remote mirroring settings.
    pub mirror: AuditMirrorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Audit remote mirroring settings")]
/// Audit remote mirroring settings.
pub struct AuditMirrorConfig {
    #[serde(default)]
    #[schemars(default, description = "Enable best-effort audit remote mirroring")]
    /// Enable best-effort audit remote mirroring.
    pub enabled: bool,

    #[serde(default = "AuditMirrorConfig::default_branch")]
    #[schemars(
        default = "AuditMirrorConfig::default_branch",
        description = "Remote branch name used for audit mirroring"
    )]
    /// Remote branch name used for audit mirroring.
    pub branch: String,
}

impl AuditMirrorConfig {
    fn default_branch() -> String {
        "ito/internal/audit".to_string()
    }
}

impl Default for AuditMirrorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            branch: Self::default_branch(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Backend state API configuration")]
/// Backend state API configuration for multi-agent coordination.
pub struct BackendApiConfig {
    #[serde(default)]
    #[schemars(default, description = "Enable backend API integration")]
    /// Whether backend API integration is enabled.
    pub enabled: bool,

    #[serde(default = "BackendApiConfig::default_url")]
    #[schemars(
        default = "BackendApiConfig::default_url",
        description = "Base URL for the backend API"
    )]
    /// Base URL for the backend API.
    pub url: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Authentication token for backend API access")]
    /// Authentication token for backend API access.
    /// When `None`, a deterministic token is generated from hostname + project root.
    pub token: Option<String>,

    #[serde(default = "BackendApiConfig::default_token_env_var")]
    #[schemars(
        default = "BackendApiConfig::default_token_env_var",
        description = "Environment variable name that holds the bearer token"
    )]
    /// Name of the environment variable that holds the bearer token.
    ///
    /// When backend mode is enabled, the token is read from this variable
    /// at runtime unless `token` is explicitly set.
    pub token_env_var: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Per-user backup directory for artifact snapshots")]
    /// Per-user directory for backup snapshots of change artifacts during
    /// sync operations. Defaults to `$HOME/.ito/backups` when not set.
    pub backup_dir: Option<String>,

    #[serde(default = "BackendApiConfig::default_timeout_ms")]
    #[schemars(
        default = "BackendApiConfig::default_timeout_ms",
        description = "Request timeout in milliseconds"
    )]
    /// Request timeout in milliseconds for backend API calls.
    pub timeout_ms: u64,

    #[serde(default = "BackendApiConfig::default_max_retries")]
    #[schemars(
        default = "BackendApiConfig::default_max_retries",
        description = "Maximum retry attempts for transient failures"
    )]
    /// Maximum retry attempts for transient failures.
    pub max_retries: u32,

    #[serde(default)]
    #[schemars(default, description = "Project namespace for backend routing")]
    /// Project namespace used for multi-tenant backend routing.
    pub project: BackendProjectConfig,
}

impl BackendApiConfig {
    fn default_url() -> String {
        "http://127.0.0.1:9010".to_string()
    }

    fn default_token_env_var() -> String {
        "ITO_BACKEND_TOKEN".to_string()
    }

    fn default_timeout_ms() -> u64 {
        30_000
    }

    fn default_max_retries() -> u32 {
        3
    }
}

impl Default for BackendApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: Self::default_url(),
            token: None,
            token_env_var: Self::default_token_env_var(),
            backup_dir: None,
            timeout_ms: Self::default_timeout_ms(),
            max_retries: Self::default_max_retries(),
            project: BackendProjectConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Change coordination settings")]
/// Configuration for change coordination behavior.
pub struct ChangesConfig {
    #[serde(default)]
    #[schemars(default, description = "Coordination branch settings")]
    /// Coordination branch settings.
    pub coordination_branch: CoordinationBranchConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Per-tool preferences")]
/// Configuration for external tool preferences used by Ito workflows.
pub struct ToolsConfig {
    #[serde(default)]
    #[schemars(default, description = "tmux preferences")]
    /// tmux preferences.
    pub tmux: TmuxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "tmux preferences")]
/// Configuration for tmux-backed workflow integrations.
pub struct TmuxConfig {
    #[serde(default = "TmuxConfig::default_enabled")]
    #[schemars(
        default = "TmuxConfig::default_enabled",
        description = "Whether tmux is enabled"
    )]
    /// Whether tmux-backed workflows should be suggested.
    pub enabled: bool,
}

impl TmuxConfig {
    fn default_enabled() -> bool {
        true
    }
}

impl Default for TmuxConfig {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Dedicated branch used for proposal/task coordination")]
/// Configuration for the internal change coordination branch.
pub struct CoordinationBranchConfig {
    #[serde(default = "CoordinationBranchConfig::default_enabled")]
    #[schemars(
        default = "CoordinationBranchConfig::default_enabled",
        description = "Enable change coordination branch synchronization"
    )]
    /// Enable change coordination branch synchronization.
    pub enabled: CoordinationBranchEnabled,

    #[serde(default = "CoordinationBranchConfig::default_name")]
    #[schemars(
        default = "CoordinationBranchConfig::default_name",
        description = "Name of the internal coordination branch"
    )]
    /// Name of the internal coordination branch.
    pub name: String,

    #[serde(default)]
    #[schemars(default, description = "Storage backend for coordination data")]
    /// Storage backend used to persist coordination data.
    pub storage: CoordinationStorage,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Worktree path override for coordination storage")]
    /// Override path for the worktree used by `CoordinationStorage::Worktree`.
    ///
    /// When `None`, the default worktree location is resolved automatically.
    pub worktree_path: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
#[schemars(description = "Boolean wrapper for coordination branch enablement")]
/// Type-safe wrapper for `coordination_branch.enabled`.
pub struct CoordinationBranchEnabled(pub bool);

impl CoordinationBranchConfig {
    fn default_enabled() -> CoordinationBranchEnabled {
        CoordinationBranchEnabled(true)
    }

    fn default_name() -> String {
        "ito/internal/changes".to_string()
    }
}

impl Default for CoordinationBranchConfig {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            name: Self::default_name(),
            storage: CoordinationStorage::default(),
            worktree_path: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
#[schemars(description = "Storage backend for coordination data")]
/// Storage backend used to persist coordination data.
pub enum CoordinationStorage {
    /// Store coordination data in the git worktree (default).
    #[default]
    Worktree,
    /// Store coordination data embedded in the repository.
    Embedded,
}

impl CoordinationStorage {
    /// Return a stable string identifier for display.
    pub fn as_str(&self) -> &'static str {
        match self {
            CoordinationStorage::Worktree => "worktree",
            CoordinationStorage::Embedded => "embedded",
        }
    }

    /// All supported storage variant values.
    pub const ALL: &'static [&'static str] = &["worktree", "embedded"];

    /// Parse a string into a `CoordinationStorage`, returning `None` for invalid values.
    pub fn parse_value(s: &str) -> Option<Self> {
        match s {
            "worktree" => Some(Self::Worktree),
            "embedded" => Some(Self::Embedded),
            _ => None,
        }
    }
}

impl std::fmt::Display for CoordinationStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Cache settings")]
/// Cache configuration for Ito data.
pub struct CacheConfig {
    #[serde(default, rename = "ttl_hours")]
    #[schemars(
        default = "CacheConfig::default_ttl_hours",
        description = "Model registry cache TTL in hours"
    )]
    /// Model registry cache TTL in hours.
    pub ttl_hours: u64,
}

impl CacheConfig {
    fn default_ttl_hours() -> u64 {
        24
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl_hours: Self::default_ttl_hours(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Harness configurations")]
/// Configuration grouped by harness.
pub struct HarnessesConfig {
    #[serde(default, rename = "opencode")]
    #[schemars(default, description = "OpenCode harness settings")]
    /// OpenCode harness settings.
    pub opencode: OpenCodeHarnessConfig,

    #[serde(default, rename = "claude-code")]
    #[schemars(default, description = "Claude Code harness settings")]
    /// Claude Code harness settings.
    pub claude_code: ClaudeCodeHarnessConfig,

    #[serde(default, rename = "codex")]
    #[schemars(default, description = "OpenAI Codex harness settings")]
    /// OpenAI Codex harness settings.
    pub codex: CodexHarnessConfig,

    #[serde(default, rename = "github-copilot")]
    #[schemars(default, description = "GitHub Copilot harness settings")]
    /// GitHub Copilot harness settings.
    pub github_copilot: GitHubCopilotHarnessConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "OpenCode harness configuration")]
/// Configuration for the OpenCode harness.
pub struct OpenCodeHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Optional provider constraint (null/omitted means any provider)")]
    /// Optional provider constraint.
    ///
    /// When omitted, any provider is accepted.
    pub provider: Option<String>,

    #[serde(default = "OpenCodeHarnessConfig::default_agents")]
    #[schemars(
        default = "OpenCodeHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
    /// Ito agent tier model mappings.
    pub agents: AgentTiersConfig,
}

impl Default for OpenCodeHarnessConfig {
    fn default() -> Self {
        Self {
            provider: None,
            agents: Self::default_agents(),
        }
    }
}

impl OpenCodeHarnessConfig {
    fn default_agents() -> AgentTiersConfig {
        AgentTiersConfig {
            ito_quick: AgentModelSetting::Model("anthropic/claude-haiku-4-5".to_string()),
            ito_general: AgentModelSetting::Options(AgentModelOptions {
                model: "openai/gpt-5.2-codex".to_string(),
                variant: Some("high".to_string()),
                temperature: Some(0.3),
                ..AgentModelOptions::default()
            }),
            ito_thinking: AgentModelSetting::Options(AgentModelOptions {
                model: "openai/gpt-5.2-codex".to_string(),
                variant: Some("xhigh".to_string()),
                temperature: Some(0.5),
                ..AgentModelOptions::default()
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Claude Code harness configuration")]
/// Configuration for the Claude Code harness.
pub struct ClaudeCodeHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Provider constraint (if specified, must be anthropic)")]
    /// Provider constraint.
    ///
    /// If specified, must be `anthropic`.
    pub provider: Option<ProviderAnthropic>,

    #[serde(default = "ClaudeCodeHarnessConfig::default_agents")]
    #[schemars(
        default = "ClaudeCodeHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
    /// Ito agent tier model mappings.
    pub agents: AgentTiersConfig,
}

impl Default for ClaudeCodeHarnessConfig {
    fn default() -> Self {
        Self {
            provider: Some(ProviderAnthropic::Anthropic),
            agents: Self::default_agents(),
        }
    }
}

impl ClaudeCodeHarnessConfig {
    fn default_agents() -> AgentTiersConfig {
        AgentTiersConfig {
            ito_quick: AgentModelSetting::Model("haiku".to_string()),
            ito_general: AgentModelSetting::Model("sonnet".to_string()),
            ito_thinking: AgentModelSetting::Model("opus".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Codex harness configuration")]
/// Configuration for the OpenAI Codex harness.
pub struct CodexHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Provider constraint (if specified, must be openai)")]
    /// Provider constraint.
    ///
    /// If specified, must be `openai`.
    pub provider: Option<ProviderOpenAi>,

    #[serde(default = "CodexHarnessConfig::default_agents")]
    #[schemars(
        default = "CodexHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
    /// Ito agent tier model mappings.
    pub agents: AgentTiersConfig,
}

impl Default for CodexHarnessConfig {
    fn default() -> Self {
        Self {
            provider: Some(ProviderOpenAi::OpenAi),
            agents: Self::default_agents(),
        }
    }
}

impl CodexHarnessConfig {
    fn default_agents() -> AgentTiersConfig {
        AgentTiersConfig {
            ito_quick: AgentModelSetting::Model("openai/gpt-5.1-codex-mini".to_string()),
            ito_general: AgentModelSetting::Options(AgentModelOptions {
                model: "openai/gpt-5.2-codex".to_string(),
                reasoning_effort: Some(ReasoningEffort::High),
                ..AgentModelOptions::default()
            }),
            ito_thinking: AgentModelSetting::Options(AgentModelOptions {
                model: "openai/gpt-5.2-codex".to_string(),
                reasoning_effort: Some(ReasoningEffort::XHigh),
                ..AgentModelOptions::default()
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "GitHub Copilot harness configuration")]
/// Configuration for the GitHub Copilot harness.
pub struct GitHubCopilotHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Provider constraint (if specified, must be github-copilot)")]
    /// Provider constraint.
    ///
    /// If specified, must be `github-copilot`.
    pub provider: Option<ProviderGitHubCopilot>,

    #[serde(default = "GitHubCopilotHarnessConfig::default_agents")]
    #[schemars(
        default = "GitHubCopilotHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
    /// Ito agent tier model mappings.
    pub agents: AgentTiersConfig,
}

impl Default for GitHubCopilotHarnessConfig {
    fn default() -> Self {
        Self {
            provider: Some(ProviderGitHubCopilot::GitHubCopilot),
            agents: Self::default_agents(),
        }
    }
}

impl GitHubCopilotHarnessConfig {
    fn default_agents() -> AgentTiersConfig {
        AgentTiersConfig {
            ito_quick: AgentModelSetting::Model("github-copilot/claude-haiku-4.5".to_string()),
            ito_general: AgentModelSetting::Model("github-copilot/gpt-5.2-codex".to_string()),
            ito_thinking: AgentModelSetting::Model("github-copilot/gpt-5.2-codex".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
/// Allowed providers for Anthropic-backed harnesses.
pub enum ProviderAnthropic {
    #[serde(rename = "anthropic")]
    /// Anthropic provider.
    Anthropic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
/// Allowed providers for OpenAI-backed harnesses.
pub enum ProviderOpenAi {
    #[serde(rename = "openai")]
    /// OpenAI provider.
    OpenAi,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
/// Allowed providers for GitHub Copilot-backed harnesses.
pub enum ProviderGitHubCopilot {
    #[serde(rename = "github-copilot")]
    /// GitHub Copilot provider.
    GitHubCopilot,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Agent tier to model mapping")]
/// Mapping from Ito agent tiers to model settings.
pub struct AgentTiersConfig {
    #[serde(rename = "ito-quick")]
    #[schemars(description = "Fast, cheap tier")]
    /// Fast, cheap tier.
    pub ito_quick: AgentModelSetting,

    #[serde(rename = "ito-general")]
    #[schemars(description = "Balanced tier")]
    /// Balanced tier.
    pub ito_general: AgentModelSetting,

    #[serde(rename = "ito-thinking")]
    #[schemars(description = "High-capability tier")]
    /// High-capability tier.
    pub ito_thinking: AgentModelSetting,
}

impl Default for AgentTiersConfig {
    fn default() -> Self {
        let empty = AgentModelSetting::Model(String::new());

        Self {
            ito_quick: empty.clone(),
            ito_general: empty.clone(),
            ito_thinking: empty,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(description = "Agent model setting: shorthand string or options object")]
/// Agent model setting.
///
/// In JSON, this can be either a shorthand string (model id) or a richer
/// options object.
pub enum AgentModelSetting {
    /// Shorthand setting using only a model identifier.
    Model(String),
    /// Extended options object.
    Options(AgentModelOptions),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Extended agent model options")]
/// Extended options for a configured model.
pub struct AgentModelOptions {
    #[schemars(
        description = "Model identifier",
        example = "AgentModelOptions::example_model"
    )]
    /// Model identifier.
    pub model: String,

    #[serde(default)]
    #[schemars(description = "Temperature (0.0-1.0)", range(min = 0.0, max = 1.0))]
    /// Temperature (0.0-1.0).
    pub temperature: Option<f64>,

    #[serde(default)]
    #[schemars(description = "Optional variant selector (OpenCode)")]
    /// Optional variant selector (OpenCode).
    pub variant: Option<String>,

    #[serde(default, rename = "top_p")]
    #[schemars(description = "Top-p sampling (0.0-1.0)", range(min = 0.0, max = 1.0))]
    /// Top-p sampling (0.0-1.0).
    pub top_p: Option<f64>,

    #[serde(default)]
    #[schemars(description = "Optional max steps for tool loops")]
    /// Optional max steps for tool loops.
    pub steps: Option<u64>,

    #[serde(default, rename = "reasoningEffort")]
    #[schemars(description = "Reasoning effort (OpenAI)")]
    /// Reasoning effort (OpenAI).
    pub reasoning_effort: Option<ReasoningEffort>,

    #[serde(default, rename = "textVerbosity")]
    #[schemars(description = "Text verbosity")]
    /// Text verbosity.
    pub text_verbosity: Option<TextVerbosity>,

    #[serde(flatten, default)]
    #[schemars(description = "Additional provider-specific options")]
    /// Additional provider-specific options.
    pub extra: BTreeMap<String, Value>,
}

impl AgentModelOptions {
    fn example_model() -> &'static str {
        "openai/gpt-5.2-codex"
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
/// Preferred verbosity level for text output.
pub enum TextVerbosity {
    /// Minimal output.
    Low,
    /// Balanced output.
    Medium,
    /// Very detailed output.
    High,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
/// Preferred reasoning effort for reasoning-capable models.
pub enum ReasoningEffort {
    /// No explicit reasoning mode.
    None,
    /// Minimal reasoning.
    Minimal,
    /// Low reasoning.
    Low,
    /// Medium reasoning.
    Medium,
    /// High reasoning.
    High,
    #[serde(rename = "xhigh")]
    /// Extra-high reasoning.
    XHigh,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Defaults section")]
/// Defaults applied when a config value is not explicitly set.
pub struct DefaultsConfig {
    #[serde(default)]
    #[schemars(default, description = "Testing-related defaults")]
    /// Testing-related defaults.
    pub testing: TestingDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Worktree workspace configuration")]
/// Configuration for Git worktree-based workspace layouts.
pub struct WorktreesConfig {
    #[serde(default)]
    #[schemars(default, description = "Enable worktree policy features")]
    /// Enable worktree policy features.
    pub enabled: bool,

    #[serde(default = "WorktreesConfig::default_strategy")]
    #[schemars(
        default = "WorktreesConfig::default_strategy",
        description = "Workspace topology strategy"
    )]
    /// Workspace topology strategy.
    pub strategy: WorktreeStrategy,

    #[serde(default)]
    #[schemars(default, description = "Layout path configuration")]
    /// Layout path configuration.
    pub layout: WorktreeLayoutConfig,

    #[serde(default)]
    #[schemars(default, description = "Apply-time behavior configuration")]
    /// Apply-time behavior configuration.
    pub apply: WorktreeApplyConfig,

    #[serde(default)]
    #[schemars(default, description = "Initialization settings for new worktrees")]
    /// Initialization settings applied when a new worktree is created via
    /// `ito worktree ensure`. Controls which files to copy from the main
    /// worktree and which commands to run after creation.
    pub init: WorktreeInitConfig,

    #[serde(default = "WorktreesConfig::default_branch")]
    #[schemars(
        default = "WorktreesConfig::default_branch",
        description = "Branch used when creating/reusing the base worktree"
    )]
    /// Branch used when creating/reusing the base worktree.
    pub default_branch: String,
}

impl Default for WorktreesConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: Self::default_strategy(),
            layout: WorktreeLayoutConfig::default(),
            apply: WorktreeApplyConfig::default(),
            init: WorktreeInitConfig::default(),
            default_branch: Self::default_branch(),
        }
    }
}

impl WorktreesConfig {
    fn default_strategy() -> WorktreeStrategy {
        WorktreeStrategy::CheckoutSubdir
    }

    fn default_branch() -> String {
        "main".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Supported worktree workspace topology strategies")]
/// Supported worktree workspace topology strategies.
pub enum WorktreeStrategy {
    /// Standard checkout with change worktrees under a gitignored subdirectory.
    CheckoutSubdir,
    /// Standard checkout with change worktrees in a sibling directory.
    CheckoutSiblings,
    /// Bare/control repo with `main` as a worktree and change worktrees as siblings.
    BareControlSiblings,
}

impl WorktreeStrategy {
    /// Return a stable string identifier for display.
    pub fn as_str(self) -> &'static str {
        match self {
            WorktreeStrategy::CheckoutSubdir => "checkout_subdir",
            WorktreeStrategy::CheckoutSiblings => "checkout_siblings",
            WorktreeStrategy::BareControlSiblings => "bare_control_siblings",
        }
    }

    /// All supported strategy values.
    pub const ALL: &'static [&'static str] = &[
        "checkout_subdir",
        "checkout_siblings",
        "bare_control_siblings",
    ];

    /// Parse a string into a strategy, returning `None` for invalid values.
    pub fn parse_value(s: &str) -> Option<WorktreeStrategy> {
        match s {
            "checkout_subdir" => Some(WorktreeStrategy::CheckoutSubdir),
            "checkout_siblings" => Some(WorktreeStrategy::CheckoutSiblings),
            "bare_control_siblings" => Some(WorktreeStrategy::BareControlSiblings),
            _ => None,
        }
    }
}

impl std::fmt::Display for WorktreeStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Worktree layout path configuration")]
/// Configuration for worktree directory layout.
pub struct WorktreeLayoutConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Base path override for worktree directory placement")]
    /// Base path override for worktree directory placement.
    pub base_dir: Option<String>,

    #[serde(default = "WorktreeLayoutConfig::default_dir_name")]
    #[schemars(
        default = "WorktreeLayoutConfig::default_dir_name",
        description = "Name of the directory that holds change worktrees"
    )]
    /// Name of the directory that holds change worktrees.
    pub dir_name: String,
}

impl Default for WorktreeLayoutConfig {
    fn default() -> Self {
        Self {
            base_dir: None,
            dir_name: Self::default_dir_name(),
        }
    }
}

impl WorktreeLayoutConfig {
    fn default_dir_name() -> String {
        "ito-worktrees".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Worktree apply-time behavior configuration")]
/// Configuration controlling worktree behavior during apply instructions.
pub struct WorktreeApplyConfig {
    #[serde(default = "WorktreeApplyConfig::default_enabled")]
    #[schemars(
        default = "WorktreeApplyConfig::default_enabled",
        description = "Enable worktree-specific setup in apply instructions"
    )]
    /// Enable worktree-specific setup in apply instructions.
    pub enabled: bool,

    #[serde(default = "WorktreeApplyConfig::default_integration_mode")]
    #[schemars(
        default = "WorktreeApplyConfig::default_integration_mode",
        description = "Integration preference after implementation"
    )]
    /// Integration preference after implementation.
    pub integration_mode: IntegrationMode,

    #[serde(default = "WorktreeApplyConfig::default_copy_from_main")]
    #[schemars(
        default = "WorktreeApplyConfig::default_copy_from_main",
        description = "Glob patterns for files to copy from main into the change worktree"
    )]
    /// Glob patterns for files to copy from main into the change worktree.
    pub copy_from_main: Vec<String>,

    #[serde(default)]
    #[schemars(
        default,
        description = "Ordered shell commands to run in the change worktree before implementation"
    )]
    /// Ordered shell commands to run in the change worktree before implementation.
    pub setup_commands: Vec<String>,
}

impl Default for WorktreeApplyConfig {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            integration_mode: Self::default_integration_mode(),
            copy_from_main: Self::default_copy_from_main(),
            setup_commands: Vec::new(),
        }
    }
}

impl WorktreeApplyConfig {
    fn default_enabled() -> bool {
        true
    }

    fn default_integration_mode() -> IntegrationMode {
        IntegrationMode::CommitPr
    }

    fn default_copy_from_main() -> Vec<String> {
        vec![
            ".env".to_string(),
            ".envrc".to_string(),
            ".mise.local.toml".to_string(),
        ]
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Worktree initialization configuration")]
/// Settings applied when a new worktree is created via `ito worktree ensure`.
///
/// Controls which files to copy from the main worktree and which commands to
/// run inside the new worktree after creation.
pub struct WorktreeInitConfig {
    #[serde(default)]
    #[schemars(
        default,
        description = "Glob patterns for files to copy from the main worktree into a new change worktree"
    )]
    /// Glob patterns for files to copy from the main worktree into a new
    /// change worktree (e.g. `[".env", ".envrc", "*.local.toml"]`).
    ///
    /// Patterns from a `.worktree-include` file at the repo root are merged
    /// with this list at runtime (union semantics).
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Command(s) to run inside the worktree after file copy")]
    /// Optional command or ordered list of commands to run inside the new
    /// worktree after included files have been copied and coordination
    /// symlinks have been created.
    ///
    /// Accepts either a single string (`"make init"`) or a list of strings
    /// (`["npm ci", "npm run build:types"]`).
    pub setup: Option<WorktreeSetupConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(description = "Setup command(s) — a single string or an ordered list of strings")]
/// A single shell command or an ordered list of shell commands to run during
/// worktree initialization.
pub enum WorktreeSetupConfig {
    /// A single command string (e.g. `"make init"`).
    Single(String),
    /// An ordered list of command strings (e.g. `["npm ci", "npm run build"]`).
    Multiple(Vec<String>),
}

impl WorktreeSetupConfig {
    /// Return the commands as a list, regardless of the variant.
    pub fn as_commands(&self) -> Vec<&str> {
        match self {
            WorktreeSetupConfig::Single(cmd) => vec![cmd.as_str()],
            WorktreeSetupConfig::Multiple(cmds) => cmds.iter().map(String::as_str).collect(),
        }
    }

    /// Return true when there are no meaningful commands to run.
    ///
    /// Empty strings are considered non-meaningful — `Multiple(vec![""])`
    /// returns `true`.
    pub fn is_empty(&self) -> bool {
        match self {
            WorktreeSetupConfig::Single(cmd) => cmd.trim().is_empty(),
            WorktreeSetupConfig::Multiple(cmds) => {
                cmds.is_empty() || cmds.iter().all(|c| c.trim().is_empty())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Integration mode after implementation")]
/// Integration mode preference for post-implementation workflow.
pub enum IntegrationMode {
    /// Commit and open a PR workflow.
    CommitPr,
    /// Merge into parent branch workflow.
    MergeParent,
}

impl IntegrationMode {
    /// Return a stable string identifier for display.
    pub fn as_str(self) -> &'static str {
        match self {
            IntegrationMode::CommitPr => "commit_pr",
            IntegrationMode::MergeParent => "merge_parent",
        }
    }

    /// All supported integration mode values.
    pub const ALL: &'static [&'static str] = &["commit_pr", "merge_parent"];

    /// Parse a string into an integration mode, returning `None` for invalid values.
    pub fn parse_value(s: &str) -> Option<IntegrationMode> {
        match s {
            "commit_pr" => Some(IntegrationMode::CommitPr),
            "merge_parent" => Some(IntegrationMode::MergeParent),
            _ => None,
        }
    }
}

impl std::fmt::Display for IntegrationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Testing defaults")]
/// Defaults that affect testing helpers.
pub struct TestingDefaults {
    #[serde(default)]
    #[schemars(default, description = "TDD workflow defaults")]
    /// Test-driven development defaults.
    pub tdd: TddDefaults,

    #[serde(default)]
    #[schemars(default, description = "Coverage defaults")]
    /// Coverage defaults.
    pub coverage: CoverageDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "TDD defaults")]
/// Defaults for the TDD workflow runner.
pub struct TddDefaults {
    #[serde(default)]
    #[schemars(
        default = "TddDefaults::default_workflow",
        description = "TDD workflow name"
    )]
    /// Default workflow name.
    pub workflow: String,
}

impl TddDefaults {
    fn default_workflow() -> String {
        "red-green-refactor".to_string()
    }
}

impl Default for TddDefaults {
    fn default() -> Self {
        Self {
            workflow: Self::default_workflow(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Coverage defaults")]
/// Defaults for coverage tooling.
pub struct CoverageDefaults {
    #[serde(default, rename = "target_percent")]
    #[schemars(
        default = "CoverageDefaults::default_target_percent",
        description = "Target coverage percentage"
    )]
    /// Target coverage percentage.
    pub target_percent: u64,
}

impl CoverageDefaults {
    fn default_target_percent() -> u64 {
        80
    }
}

impl Default for CoverageDefaults {
    fn default() -> Self {
        Self {
            target_percent: Self::default_target_percent(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Logging configuration")]
/// Logging configuration for diagnostics and invalid command tracking.
pub struct LoggingConfig {
    #[serde(default, rename = "invalidCommands")]
    #[schemars(default, description = "Invalid command logging settings")]
    /// Settings for logging invalid or unrecognized commands.
    ///
    /// When enabled, commands that fail due to unknown subcommands, invalid
    /// arguments, or unrecognized artifacts are logged to a JSONL file under
    /// the user's config directory. These logs can be fed back into agent
    /// tooling to understand and improve how agents invoke Ito.
    pub invalid_commands: InvalidCommandsConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Invalid command logging settings")]
/// Configuration for logging invalid or unrecognized CLI commands.
pub struct InvalidCommandsConfig {
    #[serde(default)]
    #[schemars(default, description = "Enable logging of invalid commands")]
    /// Enable logging of invalid commands to a JSONL file.
    pub enabled: bool,
}

#[cfg(test)]
#[path = "coordination_storage_tests.rs"]
mod coordination_storage_tests;

#[cfg(test)]
#[path = "worktree_init_tests.rs"]
mod worktree_init_tests;
