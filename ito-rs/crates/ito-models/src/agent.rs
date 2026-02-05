//! Agent and harness type definitions

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Supported agent harnesses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Harness {
    /// OpenCode - supports any provider
    OpenCode,
    /// Claude Code - Anthropic models only
    ClaudeCode,
    /// OpenAI Codex CLI - OpenAI models only
    Codex,
    /// GitHub Copilot - github-copilot provider only
    GitHubCopilot,
}

impl Harness {
    /// Get the provider constraint for this harness (None = any provider allowed)
    pub fn provider_constraint(&self) -> Option<&'static str> {
        match self {
            Self::OpenCode => None,
            Self::ClaudeCode => Some("anthropic"),
            Self::Codex => Some("openai"),
            Self::GitHubCopilot => Some("github-copilot"),
        }
    }

    /// Get project-level agent directory path patterns for this harness
    pub fn project_agent_paths(&self) -> Vec<&'static str> {
        match self {
            Self::OpenCode => vec![".opencode/agent"],
            Self::ClaudeCode => vec![".claude/agents"],
            Self::Codex => vec![".agents/skills", ".codex/agents"],
            Self::GitHubCopilot => vec![".github/agents"],
        }
    }

    /// Get global agent directory path patterns for this harness
    pub fn global_agent_paths(&self) -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        let config = dirs::config_dir().unwrap_or_else(|| home.join(".config"));

        match self {
            Self::OpenCode => vec![config.join("opencode/agent")],
            Self::ClaudeCode => vec![home.join(".claude/agents")],
            Self::Codex => vec![config.join("codex/agents"), home.join(".agents/skills")],
            Self::GitHubCopilot => vec![config.join("copilot/skills")],
        }
    }

    /// Get the file extension for agent files in this harness
    pub fn file_extension(&self) -> &'static str {
        ".md"
    }

    /// Check if this harness supports direct model configuration in agent files
    pub fn supports_model_in_frontmatter(&self) -> bool {
        match self {
            Self::OpenCode => true,
            Self::ClaudeCode => true,
            Self::Codex => false,         // Model configured at harness level
            Self::GitHubCopilot => false, // Model not directly configurable
        }
    }

    /// All supported harnesses
    pub fn all() -> &'static [Harness] {
        &[
            Self::OpenCode,
            Self::ClaudeCode,
            Self::Codex,
            Self::GitHubCopilot,
        ]
    }
}

impl fmt::Display for Harness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenCode => write!(f, "opencode"),
            Self::ClaudeCode => write!(f, "claude-code"),
            Self::Codex => write!(f, "codex"),
            Self::GitHubCopilot => write!(f, "github-copilot"),
        }
    }
}

impl std::str::FromStr for Harness {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "opencode" | "open-code" => Ok(Self::OpenCode),
            "claude-code" | "claudecode" | "claude" => Ok(Self::ClaudeCode),
            "codex" => Ok(Self::Codex),
            "github-copilot" | "githubcopilot" | "copilot" => Ok(Self::GitHubCopilot),
            _ => Err(format!("Unknown harness: {}", s)),
        }
    }
}

/// Ito agent tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentTier {
    /// Fast, cost-effective agent for simple tasks
    ItoQuick,
    /// Balanced agent for typical development tasks
    ItoGeneral,
    /// High-capability agent for complex reasoning
    ItoThinking,
}

impl AgentTier {
    /// Get the agent file name (without extension)
    pub fn file_name(&self) -> &'static str {
        match self {
            Self::ItoQuick => "ito-quick",
            Self::ItoGeneral => "ito-general",
            Self::ItoThinking => "ito-thinking",
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ItoQuick => {
                "Fast, cost-effective agent for simple tasks, quick queries, and small code changes"
            }
            Self::ItoGeneral => {
                "Balanced agent for typical development tasks, code review, and implementation work"
            }
            Self::ItoThinking => {
                "High-capability agent for complex reasoning, architecture decisions, and difficult problems"
            }
        }
    }

    /// All agent tiers
    pub fn all() -> &'static [AgentTier] {
        &[Self::ItoQuick, Self::ItoGeneral, Self::ItoThinking]
    }
}

impl fmt::Display for AgentTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name())
    }
}

impl std::str::FromStr for AgentTier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ito-quick" | "quick" => Ok(Self::ItoQuick),
            "ito-general" | "general" => Ok(Self::ItoGeneral),
            "ito-thinking" | "thinking" => Ok(Self::ItoThinking),
            _ => Err(format!("Unknown agent tier: {}", s)),
        }
    }
}

/// Discovered agent file
#[derive(Debug, Clone)]
pub struct AgentFile {
    /// Agent name (derived from filename)
    pub name: String,
    /// Which harness this agent belongs to
    pub harness: Harness,
    /// Full path to the agent file
    pub path: PathBuf,
    /// Whether this is a global or project agent
    pub scope: AgentScope,
    /// Parsed frontmatter (if available)
    pub frontmatter: Option<AgentFrontmatter>,
}

/// Scope of an agent file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentScope {
    /// Global agent in user config directory
    Global,
    /// Project-level agent
    Project,
}

impl fmt::Display for AgentScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "global"),
            Self::Project => write!(f, "project"),
        }
    }
}

/// Parsed agent frontmatter (common fields across harnesses)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentFrontmatter {
    /// Agent name (explicit or derived)
    #[serde(default)]
    pub name: Option<String>,
    /// Description of what the agent does
    #[serde(default)]
    pub description: Option<String>,
    /// Model identifier (format varies by harness)
    #[serde(default)]
    pub model: Option<String>,
    /// Temperature setting (OpenCode only)
    #[serde(default)]
    pub temperature: Option<f64>,
    /// Variant setting (OpenCode only)
    #[serde(default)]
    pub variant: Option<String>,
    /// Mode (OpenCode: primary/subagent/all)
    #[serde(default)]
    pub mode: Option<String>,
    /// Tools configuration (format varies by harness)
    #[serde(default)]
    pub tools: Option<serde_json::Value>,
    /// Reasoning effort (OpenAI models)
    #[serde(default, rename = "reasoningEffort")]
    pub reasoning_effort: Option<String>,
}

impl AgentFile {
    /// Check if this is a Ito-managed agent (ito-quick, ito-general, ito-thinking)
    pub fn is_ito_agent(&self) -> bool {
        self.name.starts_with("ito-")
    }

    /// Get the Ito agent tier if this is a Ito agent
    pub fn ito_tier(&self) -> Option<AgentTier> {
        self.name.parse().ok()
    }

    /// Get the current model from frontmatter
    pub fn current_model(&self) -> Option<&str> {
        self.frontmatter.as_ref()?.model.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_provider_constraints() {
        assert_eq!(Harness::OpenCode.provider_constraint(), None);
        assert_eq!(Harness::ClaudeCode.provider_constraint(), Some("anthropic"));
        assert_eq!(Harness::Codex.provider_constraint(), Some("openai"));
        assert_eq!(
            Harness::GitHubCopilot.provider_constraint(),
            Some("github-copilot")
        );
    }

    #[test]
    fn harness_from_str() {
        assert_eq!("opencode".parse::<Harness>().unwrap(), Harness::OpenCode);
        assert_eq!(
            "claude-code".parse::<Harness>().unwrap(),
            Harness::ClaudeCode
        );
        assert_eq!("codex".parse::<Harness>().unwrap(), Harness::Codex);
        assert_eq!(
            "github-copilot".parse::<Harness>().unwrap(),
            Harness::GitHubCopilot
        );
    }

    #[test]
    fn agent_tier_file_names() {
        assert_eq!(AgentTier::ItoQuick.file_name(), "ito-quick");
        assert_eq!(AgentTier::ItoGeneral.file_name(), "ito-general");
        assert_eq!(AgentTier::ItoThinking.file_name(), "ito-thinking");
    }

    #[test]
    fn agent_tier_from_str() {
        assert_eq!(
            "ito-quick".parse::<AgentTier>().unwrap(),
            AgentTier::ItoQuick
        );
        assert_eq!("quick".parse::<AgentTier>().unwrap(), AgentTier::ItoQuick);
        assert_eq!(
            "ito-general".parse::<AgentTier>().unwrap(),
            AgentTier::ItoGeneral
        );
        assert_eq!(
            "ito-thinking".parse::<AgentTier>().unwrap(),
            AgentTier::ItoThinking
        );
    }
}
