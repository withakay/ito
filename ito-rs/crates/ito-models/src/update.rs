//! Agent file update utilities

use crate::agent::{AgentFile, Harness};
use std::fs;
use std::io;
use std::path::Path;

/// Result of an agent update operation
#[derive(Debug, Clone)]
pub struct UpdateResult {
    /// Path to the updated file
    pub path: std::path::PathBuf,
    /// Previous model (if any)
    pub old_model: Option<String>,
    /// New model
    pub new_model: String,
    /// Whether a backup was created
    pub backup_created: bool,
}

/// Error during agent update
#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("Failed to read agent file: {0}")]
    ReadError(#[from] io::Error),

    #[error("Agent file has no frontmatter")]
    NoFrontmatter,

    #[error("Failed to parse frontmatter: {0}")]
    ParseError(String),

    #[error("Model '{0}' is not valid for harness '{1}' (requires provider: {2})")]
    InvalidProvider(String, Harness, String),

    #[error("Harness '{0}' does not support model configuration in agent files")]
    ModelNotSupported(Harness),
}

/// Update the model in an agent file
pub fn update_agent_model(
    agent: &AgentFile,
    new_model: &str,
    create_backup: bool,
) -> Result<UpdateResult, UpdateError> {
    // Validate the model is appropriate for the harness
    validate_model_for_harness(new_model, agent.harness)?;

    // Read the current file content
    let content = fs::read_to_string(&agent.path)?;

    // Create backup if requested
    let backup_created = if create_backup {
        create_backup_file(&agent.path)?;
        true
    } else {
        false
    };

    // Update the frontmatter
    let updated_content = update_model_in_content(&content, new_model, agent.harness)?;

    // Write the updated content
    fs::write(&agent.path, updated_content)?;

    let old_model = agent.current_model().map(|s| s.to_string());

    Ok(UpdateResult {
        path: agent.path.clone(),
        old_model,
        new_model: new_model.to_string(),
        backup_created,
    })
}

/// Validate that a model ID is valid for a given harness
fn validate_model_for_harness(model: &str, harness: Harness) -> Result<(), UpdateError> {
    // Check if harness supports model in frontmatter
    if !harness.supports_model_in_frontmatter() {
        return Err(UpdateError::ModelNotSupported(harness));
    }

    // Check provider constraint
    if let Some(required_provider) = harness.provider_constraint() {
        // For Claude Code, model is just "haiku", "sonnet", "opus", "inherit"
        if harness == Harness::ClaudeCode {
            let valid_models = ["haiku", "sonnet", "opus", "inherit"];
            if !valid_models.contains(&model) {
                return Err(UpdateError::InvalidProvider(
                    model.to_string(),
                    harness,
                    required_provider.to_string(),
                ));
            }
        } else {
            // For other harnesses, check provider prefix
            let provider = model.split('/').next().unwrap_or("");
            if provider != required_provider {
                return Err(UpdateError::InvalidProvider(
                    model.to_string(),
                    harness,
                    required_provider.to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// Create a backup of the file with .bak extension
fn create_backup_file(path: &Path) -> Result<(), io::Error> {
    let backup_path = path.with_extension("md.bak");
    fs::copy(path, backup_path)?;
    Ok(())
}

/// Update the model field in the markdown content
fn update_model_in_content(
    content: &str,
    new_model: &str,
    harness: Harness,
) -> Result<String, UpdateError> {
    // Check for frontmatter
    if !content.starts_with("---") {
        return Err(UpdateError::NoFrontmatter);
    }

    // Find the frontmatter boundaries
    let rest = &content[3..];
    let end_idx = rest
        .find("\n---")
        .ok_or_else(|| UpdateError::ParseError("No closing frontmatter delimiter".to_string()))?;

    let frontmatter_yaml = &rest[..end_idx];
    let body = &rest[end_idx + 4..]; // Skip "\n---"

    // Parse the frontmatter
    let mut frontmatter: serde_yaml::Value = serde_yaml::from_str(frontmatter_yaml)
        .map_err(|e| UpdateError::ParseError(e.to_string()))?;

    // Update the model field
    if let serde_yaml::Value::Mapping(ref mut map) = frontmatter {
        map.insert(
            serde_yaml::Value::String("model".to_string()),
            serde_yaml::Value::String(new_model.to_string()),
        );
    }

    // Serialize back to YAML
    let updated_frontmatter =
        serde_yaml::to_string(&frontmatter).map_err(|e| UpdateError::ParseError(e.to_string()))?;

    // Handle harness-specific formatting
    let formatted_frontmatter = format_frontmatter_for_harness(&updated_frontmatter, harness);

    // Reconstruct the file
    Ok(format!("---\n{}---{}", formatted_frontmatter, body))
}

/// Format frontmatter according to harness conventions
fn format_frontmatter_for_harness(frontmatter: &str, _harness: Harness) -> String {
    // For now, just ensure it ends with a newline
    let mut result = frontmatter.to_string();
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result
}

/// Batch update multiple agents
pub fn update_agents_batch(
    agents: &[AgentFile],
    new_model: &str,
    create_backup: bool,
) -> Vec<Result<UpdateResult, UpdateError>> {
    agents
        .iter()
        .map(|agent| update_agent_model(agent, new_model, create_backup))
        .collect()
}

/// Options for batch updates
#[derive(Debug, Clone, Default)]
pub struct BatchUpdateOptions {
    /// Create backups before modifying files
    pub create_backup: bool,
    /// Only perform a dry run (don't actually modify files)
    pub dry_run: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentFrontmatter, AgentScope};
    use tempfile::tempdir;

    #[test]
    fn validate_model_opencode_accepts_any_provider() {
        assert!(
            validate_model_for_harness("anthropic/claude-sonnet-4-5", Harness::OpenCode).is_ok()
        );
        assert!(validate_model_for_harness("openai/gpt-5.2-codex", Harness::OpenCode).is_ok());
        assert!(validate_model_for_harness("google/gemini-2.0", Harness::OpenCode).is_ok());
    }

    #[test]
    fn validate_model_claude_code_requires_simple_names() {
        assert!(validate_model_for_harness("sonnet", Harness::ClaudeCode).is_ok());
        assert!(validate_model_for_harness("haiku", Harness::ClaudeCode).is_ok());
        assert!(validate_model_for_harness("opus", Harness::ClaudeCode).is_ok());
        assert!(validate_model_for_harness("inherit", Harness::ClaudeCode).is_ok());
        assert!(
            validate_model_for_harness("anthropic/claude-sonnet-4-5", Harness::ClaudeCode).is_err()
        );
    }

    #[test]
    fn validate_model_codex_not_supported() {
        assert!(matches!(
            validate_model_for_harness("openai/gpt-5.2-codex", Harness::Codex),
            Err(UpdateError::ModelNotSupported(_))
        ));
    }

    #[test]
    fn update_model_in_content_works() {
        let content = r#"---
name: test-agent
description: A test agent
model: anthropic/claude-haiku-4-5
temperature: 0.3
---

Agent instructions here.
"#;

        let updated =
            update_model_in_content(content, "openai/gpt-5.2-codex", Harness::OpenCode).unwrap();
        assert!(updated.contains("model: openai/gpt-5.2-codex"));
        assert!(updated.contains("Agent instructions here."));
        assert!(updated.contains("temperature:"));
    }

    #[test]
    fn update_agent_model_creates_backup() {
        let dir = tempdir().unwrap();
        let agent_path = dir.path().join("test-agent.md");

        fs::write(
            &agent_path,
            r#"---
description: Test agent
model: anthropic/claude-haiku-4-5
---

Instructions.
"#,
        )
        .unwrap();

        let agent = AgentFile {
            name: "test-agent".to_string(),
            harness: Harness::OpenCode,
            path: agent_path.clone(),
            scope: AgentScope::Project,
            frontmatter: Some(AgentFrontmatter {
                model: Some("anthropic/claude-haiku-4-5".to_string()),
                ..Default::default()
            }),
        };

        let result = update_agent_model(&agent, "openai/gpt-5.2-codex", true).unwrap();

        assert!(result.backup_created);
        assert_eq!(
            result.old_model,
            Some("anthropic/claude-haiku-4-5".to_string())
        );
        assert_eq!(result.new_model, "openai/gpt-5.2-codex");

        // Check backup exists
        let backup_path = agent_path.with_extension("md.bak");
        assert!(backup_path.exists());

        // Check updated content
        let updated_content = fs::read_to_string(&agent_path).unwrap();
        assert!(updated_content.contains("model: openai/gpt-5.2-codex"));
    }
}
