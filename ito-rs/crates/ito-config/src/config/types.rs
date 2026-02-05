use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Top-level Ito configuration")]
pub struct ItoConfig {
    #[serde(default, rename = "$schema", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Optional JSON schema reference for editor validation")]
    pub schema: Option<String>,

    #[serde(default, rename = "projectPath")]
    #[schemars(description = "Ito working directory name (defaults to .ito)")]
    pub project_path: Option<String>,

    #[serde(default)]
    #[schemars(default, description = "Harness-specific configuration")]
    pub harnesses: HarnessesConfig,

    #[serde(default)]
    #[schemars(default, description = "Cache configuration")]
    pub cache: CacheConfig,

    #[serde(default)]
    #[schemars(default, description = "Global defaults for workflow and tooling")]
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Cache settings")]
pub struct CacheConfig {
    #[serde(default, rename = "ttl_hours")]
    #[schemars(
        default = "CacheConfig::default_ttl_hours",
        description = "Model registry cache TTL in hours"
    )]
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
pub struct HarnessesConfig {
    #[serde(default, rename = "opencode")]
    #[schemars(default, description = "OpenCode harness settings")]
    pub opencode: OpenCodeHarnessConfig,

    #[serde(default, rename = "claude-code")]
    #[schemars(default, description = "Claude Code harness settings")]
    pub claude_code: ClaudeCodeHarnessConfig,

    #[serde(default, rename = "codex")]
    #[schemars(default, description = "OpenAI Codex harness settings")]
    pub codex: CodexHarnessConfig,

    #[serde(default, rename = "github-copilot")]
    #[schemars(default, description = "GitHub Copilot harness settings")]
    pub github_copilot: GitHubCopilotHarnessConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "OpenCode harness configuration")]
pub struct OpenCodeHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Optional provider constraint (null/omitted means any provider)")]
    pub provider: Option<String>,

    #[serde(default = "OpenCodeHarnessConfig::default_agents")]
    #[schemars(
        default = "OpenCodeHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
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
pub struct ClaudeCodeHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Provider constraint (if specified, must be anthropic)")]
    pub provider: Option<ProviderAnthropic>,

    #[serde(default = "ClaudeCodeHarnessConfig::default_agents")]
    #[schemars(
        default = "ClaudeCodeHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
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
pub struct CodexHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Provider constraint (if specified, must be openai)")]
    pub provider: Option<ProviderOpenAi>,

    #[serde(default = "CodexHarnessConfig::default_agents")]
    #[schemars(
        default = "CodexHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
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
pub struct GitHubCopilotHarnessConfig {
    #[serde(default)]
    #[schemars(description = "Provider constraint (if specified, must be github-copilot)")]
    pub provider: Option<ProviderGitHubCopilot>,

    #[serde(default = "GitHubCopilotHarnessConfig::default_agents")]
    #[schemars(
        default = "GitHubCopilotHarnessConfig::default_agents",
        description = "Ito agent tier model mappings"
    )]
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
pub enum ProviderAnthropic {
    #[serde(rename = "anthropic")]
    Anthropic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderOpenAi {
    #[serde(rename = "openai")]
    OpenAi,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderGitHubCopilot {
    #[serde(rename = "github-copilot")]
    GitHubCopilot,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Agent tier to model mapping")]
pub struct AgentTiersConfig {
    #[serde(rename = "ito-quick")]
    #[schemars(description = "Fast, cheap tier")]
    pub ito_quick: AgentModelSetting,

    #[serde(rename = "ito-general")]
    #[schemars(description = "Balanced tier")]
    pub ito_general: AgentModelSetting,

    #[serde(rename = "ito-thinking")]
    #[schemars(description = "High-capability tier")]
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
pub enum AgentModelSetting {
    Model(String),
    Options(AgentModelOptions),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Extended agent model options")]
pub struct AgentModelOptions {
    #[schemars(
        description = "Model identifier",
        example = "AgentModelOptions::example_model"
    )]
    pub model: String,

    #[serde(default)]
    #[schemars(description = "Temperature (0.0-1.0)", range(min = 0.0, max = 1.0))]
    pub temperature: Option<f64>,

    #[serde(default)]
    #[schemars(description = "Optional variant selector (OpenCode)")]
    pub variant: Option<String>,

    #[serde(default, rename = "top_p")]
    #[schemars(description = "Top-p sampling (0.0-1.0)", range(min = 0.0, max = 1.0))]
    pub top_p: Option<f64>,

    #[serde(default)]
    #[schemars(description = "Optional max steps for tool loops")]
    pub steps: Option<u64>,

    #[serde(default, rename = "reasoningEffort")]
    #[schemars(description = "Reasoning effort (OpenAI)")]
    pub reasoning_effort: Option<ReasoningEffort>,

    #[serde(default, rename = "textVerbosity")]
    #[schemars(description = "Text verbosity")]
    pub text_verbosity: Option<TextVerbosity>,

    #[serde(flatten, default)]
    #[schemars(description = "Additional provider-specific options")]
    pub extra: BTreeMap<String, Value>,
}

impl AgentModelOptions {
    fn example_model() -> &'static str {
        "openai/gpt-5.2-codex"
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TextVerbosity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    #[serde(rename = "xhigh")]
    XHigh,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Defaults section")]
pub struct DefaultsConfig {
    #[serde(default)]
    #[schemars(default, description = "Testing-related defaults")]
    pub testing: TestingDefaults,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Testing defaults")]
pub struct TestingDefaults {
    #[serde(default)]
    #[schemars(default, description = "TDD workflow defaults")]
    pub tdd: TddDefaults,

    #[serde(default)]
    #[schemars(default, description = "Coverage defaults")]
    pub coverage: CoverageDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "TDD defaults")]
pub struct TddDefaults {
    #[serde(default)]
    #[schemars(
        default = "TddDefaults::default_workflow",
        description = "TDD workflow name"
    )]
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
pub struct CoverageDefaults {
    #[serde(default, rename = "target_percent")]
    #[schemars(
        default = "CoverageDefaults::default_target_percent",
        description = "Target coverage percentage"
    )]
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
