//! Agent file discovery across harnesses

use crate::agent::{AgentFile, AgentFrontmatter, AgentScope, Harness};
use std::fs;
use std::path::{Path, PathBuf};

/// Options for agent discovery
#[derive(Debug, Clone, Default)]
pub struct DiscoveryOptions {
    /// Only discover agents for specific harnesses
    pub harnesses: Option<Vec<Harness>>,
    /// Only discover global agents
    pub global_only: bool,
    /// Only discover project agents
    pub project_only: bool,
    /// Project root directory (for project-level agents)
    pub project_root: Option<PathBuf>,
}

/// Discover all agent files across configured harnesses
pub fn discover_agents(options: &DiscoveryOptions) -> Vec<AgentFile> {
    let harnesses = options
        .harnesses
        .as_deref()
        .unwrap_or_else(|| Harness::all());

    let mut agents = Vec::new();

    for harness in harnesses {
        // Discover global agents
        if !options.project_only {
            for path in harness.global_agent_paths() {
                if path.exists() {
                    agents.extend(discover_in_directory(&path, *harness, AgentScope::Global));
                }
            }
        }

        // Discover project agents
        if !options.global_only
            && let Some(project_root) = &options.project_root
        {
            for rel_path in harness.project_agent_paths() {
                let path = project_root.join(rel_path);
                if path.exists() {
                    agents.extend(discover_in_directory(&path, *harness, AgentScope::Project));
                }
            }
        }
    }

    agents
}

/// Discover agent files in a specific directory
fn discover_in_directory(dir: &Path, harness: Harness, scope: AgentScope) -> Vec<AgentFile> {
    let mut agents = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return agents,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip directories and non-markdown files
        if path.is_dir() {
            // For Codex skills, check for SKILL.md inside directories
            if harness == Harness::Codex {
                let skill_md = path.join("SKILL.md");
                if skill_md.exists()
                    && let Some(agent) = parse_agent_file(&skill_md, harness, scope)
                {
                    agents.push(agent);
                }
            }
            continue;
        }

        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }

        if let Some(agent) = parse_agent_file(&path, harness, scope) {
            agents.push(agent);
        }
    }

    agents
}

/// Parse an agent file and extract frontmatter
fn parse_agent_file(path: &Path, harness: Harness, scope: AgentScope) -> Option<AgentFile> {
    let content = fs::read_to_string(path).ok()?;

    // Extract name from filename (without extension)
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        // For SKILL.md files, use parent directory name
        .or_else(|| {
            if path.file_name()?.to_str()? == "SKILL.md" {
                path.parent()?.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })?;

    let frontmatter = parse_frontmatter(&content);

    Some(AgentFile {
        name,
        harness,
        path: path.to_path_buf(),
        scope,
        frontmatter,
    })
}

/// Parse YAML frontmatter from markdown content
fn parse_frontmatter(content: &str) -> Option<AgentFrontmatter> {
    // Check for frontmatter delimiters
    if !content.starts_with("---") {
        return None;
    }

    // Find the closing delimiter
    let rest = &content[3..];
    let end_idx = rest.find("\n---")?;
    let yaml_content = &rest[..end_idx].trim();

    // Parse YAML
    serde_yaml::from_str(yaml_content).ok()
}

/// Filter agents to only Ito-managed agents (ito-quick, ito-general, ito-thinking)
pub fn filter_ito_agents(agents: &[AgentFile]) -> Vec<&AgentFile> {
    agents.iter().filter(|a| a.is_ito_agent()).collect()
}

/// Filter agents by harness
pub fn filter_by_harness(agents: &[AgentFile], harness: Harness) -> Vec<&AgentFile> {
    agents.iter().filter(|a| a.harness == harness).collect()
}

/// Find agents with outdated models (compared to a target model)
#[allow(dead_code)]
pub fn find_outdated_agents<'a>(agents: &'a [AgentFile], target_model: &str) -> Vec<&'a AgentFile> {
    agents
        .iter()
        .filter(|a| a.current_model().is_some_and(|m| m != target_model))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parse_frontmatter_extracts_model() {
        let content = r#"---
name: test-agent
description: A test agent
model: anthropic/claude-sonnet-4-5
temperature: 0.3
---

Agent instructions here.
"#;

        let fm = parse_frontmatter(content).unwrap();
        assert_eq!(fm.name, Some("test-agent".to_string()));
        assert_eq!(fm.model, Some("anthropic/claude-sonnet-4-5".to_string()));
        assert_eq!(fm.temperature, Some(0.3));
    }

    #[test]
    fn parse_frontmatter_handles_missing_fields() {
        let content = r#"---
description: A minimal agent
---

Instructions.
"#;

        let fm = parse_frontmatter(content).unwrap();
        assert_eq!(fm.name, None);
        assert_eq!(fm.model, None);
        assert!(fm.description.is_some());
    }

    #[test]
    fn parse_frontmatter_returns_none_for_no_frontmatter() {
        let content = "Just some markdown without frontmatter.";
        assert!(parse_frontmatter(content).is_none());
    }

    #[test]
    fn discover_agents_in_directory() {
        let dir = tempdir().unwrap();
        let agent_path = dir.path().join("test-agent.md");

        fs::write(
            &agent_path,
            r#"---
description: Test agent
model: anthropic/claude-sonnet-4-5
---

Instructions.
"#,
        )
        .unwrap();

        let agents = discover_in_directory(dir.path(), Harness::OpenCode, AgentScope::Project);

        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "test-agent");
        assert_eq!(agents[0].harness, Harness::OpenCode);
        assert_eq!(
            agents[0].current_model(),
            Some("anthropic/claude-sonnet-4-5")
        );
    }

    #[test]
    fn filter_ito_agents_works() {
        let agents = vec![
            AgentFile {
                name: "ito-quick".to_string(),
                harness: Harness::OpenCode,
                path: PathBuf::from("ito-quick.md"),
                scope: AgentScope::Project,
                frontmatter: None,
            },
            AgentFile {
                name: "custom-agent".to_string(),
                harness: Harness::OpenCode,
                path: PathBuf::from("custom-agent.md"),
                scope: AgentScope::Project,
                frontmatter: None,
            },
        ];

        let ito_agents = filter_ito_agents(&agents);
        assert_eq!(ito_agents.len(), 1);
        assert_eq!(ito_agents[0].name, "ito-quick");
    }
}
