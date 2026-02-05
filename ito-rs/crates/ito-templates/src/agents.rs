//! Agent template handling for Ito agent tiers
//!
//! This module provides utilities for loading and rendering agent templates
//! with placeholder resolution for model configuration.

use std::collections::HashMap;

/// Agent configuration with model and optional extended options
#[derive(Debug, Clone, Default)]
pub struct AgentConfig {
    /// Model ID (e.g., "anthropic/claude-haiku-4-5" or "haiku" for Claude Code)
    pub model: String,
    /// Optional variant (e.g., "high", "xhigh")
    pub variant: Option<String>,
    /// Optional temperature
    pub temperature: Option<f64>,
    /// Optional reasoning effort (for OpenAI models)
    pub reasoning_effort: Option<String>,
}

/// Harness identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Harness {
    OpenCode,
    ClaudeCode,
    Codex,
    GitHubCopilot,
}

impl Harness {
    /// Get the directory name in the assets/agents/ directory
    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::OpenCode => "opencode",
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
            Self::GitHubCopilot => "github-copilot",
        }
    }

    /// Get the target installation path for project agents
    pub fn project_agent_path(&self) -> &'static str {
        match self {
            Self::OpenCode => ".opencode/agent",
            Self::ClaudeCode => ".claude/agents",
            Self::Codex => ".agents/skills",
            Self::GitHubCopilot => ".github/agents",
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

/// Agent tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentTier {
    Quick,
    General,
    Thinking,
}

impl AgentTier {
    /// Get the file name (without extension)
    pub fn file_name(&self) -> &'static str {
        match self {
            Self::Quick => "ito-quick",
            Self::General => "ito-general",
            Self::Thinking => "ito-thinking",
        }
    }

    /// All tiers
    pub fn all() -> &'static [AgentTier] {
        &[Self::Quick, Self::General, Self::Thinking]
    }
}

/// Default model configurations per harness and tier
pub fn default_agent_configs() -> HashMap<(Harness, AgentTier), AgentConfig> {
    let mut configs = HashMap::new();

    // OpenCode defaults
    configs.insert(
        (Harness::OpenCode, AgentTier::Quick),
        AgentConfig {
            model: "anthropic/claude-haiku-4-5".to_string(),
            temperature: Some(0.3),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::OpenCode, AgentTier::General),
        AgentConfig {
            model: "openai/gpt-5.2-codex".to_string(),
            variant: Some("high".to_string()),
            temperature: Some(0.3),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::OpenCode, AgentTier::Thinking),
        AgentConfig {
            model: "openai/gpt-5.2-codex".to_string(),
            variant: Some("xhigh".to_string()),
            temperature: Some(0.5),
            ..Default::default()
        },
    );

    // Claude Code defaults (uses simplified model names)
    configs.insert(
        (Harness::ClaudeCode, AgentTier::Quick),
        AgentConfig {
            model: "haiku".to_string(),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::ClaudeCode, AgentTier::General),
        AgentConfig {
            model: "sonnet".to_string(),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::ClaudeCode, AgentTier::Thinking),
        AgentConfig {
            model: "opus".to_string(),
            ..Default::default()
        },
    );

    // Codex defaults
    configs.insert(
        (Harness::Codex, AgentTier::Quick),
        AgentConfig {
            model: "openai/gpt-5.1-codex-mini".to_string(),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::Codex, AgentTier::General),
        AgentConfig {
            model: "openai/gpt-5.2-codex".to_string(),
            reasoning_effort: Some("high".to_string()),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::Codex, AgentTier::Thinking),
        AgentConfig {
            model: "openai/gpt-5.2-codex".to_string(),
            reasoning_effort: Some("xhigh".to_string()),
            ..Default::default()
        },
    );

    // GitHub Copilot defaults
    configs.insert(
        (Harness::GitHubCopilot, AgentTier::Quick),
        AgentConfig {
            model: "github-copilot/claude-haiku-4.5".to_string(),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::GitHubCopilot, AgentTier::General),
        AgentConfig {
            model: "github-copilot/gpt-5.2-codex".to_string(),
            ..Default::default()
        },
    );
    configs.insert(
        (Harness::GitHubCopilot, AgentTier::Thinking),
        AgentConfig {
            model: "github-copilot/gpt-5.2-codex".to_string(),
            ..Default::default()
        },
    );

    configs
}

/// Render an agent template by replacing placeholders with actual values
pub fn render_agent_template(template: &str, config: &AgentConfig) -> String {
    let mut result = template.to_string();

    // Replace model placeholder
    result = result.replace("{{model}}", &config.model);

    // Replace variant placeholder (or remove line if not set)
    if let Some(variant) = &config.variant {
        result = result.replace("{{variant}}", variant);
    } else {
        // Remove lines containing {{variant}} if no variant is set
        result = result
            .lines()
            .filter(|line| !line.contains("{{variant}}"))
            .collect::<Vec<_>>()
            .join("\n");
    }

    result
}

/// Get agent template files for a specific harness
pub fn get_agent_files(harness: Harness) -> Vec<(&'static str, &'static [u8])> {
    let dir_name = harness.dir_name();
    let agents_dir = &super::AGENTS_DIR;

    let mut files = Vec::new();

    if let Some(harness_dir) = agents_dir.get_dir(dir_name) {
        for file in harness_dir.files() {
            if let Some(name) = file.path().file_name().and_then(|n| n.to_str()) {
                files.push((name, file.contents()));
            }
        }

        // Also check subdirectories (for Codex SKILL.md format)
        for subdir in harness_dir.dirs() {
            if let Some(skill_file) = subdir.get_file("SKILL.md") {
                let dir_name = subdir.path().file_name().and_then(|n| n.to_str());
                if let Some(name) = dir_name {
                    // Return as "dirname/SKILL.md"
                    let path = format!("{}/SKILL.md", name);
                    // We need to leak the string to get a static lifetime
                    // This is acceptable since these are loaded once at startup
                    let leaked: &'static str = Box::leak(path.into_boxed_str());
                    files.push((leaked, skill_file.contents()));
                }
            }
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_replaces_model() {
        let template = r#"---
model: "{{model}}"
---
Instructions"#;

        let config = AgentConfig {
            model: "anthropic/claude-haiku-4-5".to_string(),
            ..Default::default()
        };

        let result = render_agent_template(template, &config);
        assert!(result.contains("model: \"anthropic/claude-haiku-4-5\""));
    }

    #[test]
    fn render_template_replaces_variant() {
        let template = r#"---
model: "{{model}}"
variant: "{{variant}}"
---"#;

        let config = AgentConfig {
            model: "openai/gpt-5.2-codex".to_string(),
            variant: Some("high".to_string()),
            ..Default::default()
        };

        let result = render_agent_template(template, &config);
        assert!(result.contains("variant: \"high\""));
    }

    #[test]
    fn render_template_removes_variant_line_if_not_set() {
        let template = r#"---
model: "{{model}}"
variant: "{{variant}}"
---"#;

        let config = AgentConfig {
            model: "anthropic/claude-haiku-4-5".to_string(),
            variant: None,
            ..Default::default()
        };

        let result = render_agent_template(template, &config);
        assert!(!result.contains("variant"));
    }

    #[test]
    fn default_configs_has_all_combinations() {
        let configs = default_agent_configs();

        for harness in Harness::all() {
            for tier in AgentTier::all() {
                assert!(
                    configs.contains_key(&(*harness, *tier)),
                    "Missing config for {:?}/{:?}",
                    harness,
                    tier
                );
            }
        }
    }
}
