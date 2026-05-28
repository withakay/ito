//! Expected Ito-managed file manifest generation.

use crate::{commands_files, default_project_files, skills_files};

/// Tool-specific harness surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessTool {
    /// Claude Code project assets.
    ClaudeCode,
    /// OpenCode project assets.
    OpenCode,
    /// Codex project assets.
    Codex,
    /// GitHub Copilot project assets.
    GitHubCopilot,
    /// Pi coding-agent project assets.
    Pi,
}

/// Source category for an expected manifest entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestSource {
    /// Default project file installed by Ito.
    Project,
    /// Shared skill installed into a harness skill directory.
    Skill,
    /// Shared command installed into a harness command/prompt directory.
    Command,
    /// Harness-specific adapter or bootstrap asset.
    Adapter,
}

/// One expected Ito-managed installed file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntry {
    /// Destination path relative to the project root or harness config root.
    pub relative_path: String,
    /// Embedded source path relative to its asset category.
    pub source_path: String,
    /// Category of embedded source.
    pub source: ManifestSource,
    /// Harness that owns the entry, or `None` for project-level files.
    pub harness: Option<HarnessTool>,
}

/// Generate the expected Ito-managed install manifest for selected harness tools.
#[must_use]
pub fn generate_manifest(tools: &[HarnessTool]) -> Vec<ManifestEntry> {
    let mut entries = Vec::new();

    for file in default_project_files() {
        entries.push(ManifestEntry {
            relative_path: file.relative_path.to_string(),
            source_path: file.relative_path.to_string(),
            source: ManifestSource::Project,
            harness: None,
        });
    }

    for tool in tools {
        entries.extend(harness_entries(*tool));
    }

    entries.sort_by(|a, b| {
        a.relative_path
            .cmp(&b.relative_path)
            .then(a.source_path.cmp(&b.source_path))
    });
    entries
}

fn harness_entries(tool: HarnessTool) -> Vec<ManifestEntry> {
    let mut entries = Vec::new();
    match tool {
        HarnessTool::ClaudeCode => {
            adapter(
                &mut entries,
                tool,
                "claude/session-start.sh",
                ".claude/session-start.sh",
            );
            adapter(
                &mut entries,
                tool,
                "claude/hooks/ito-audit.sh",
                ".claude/hooks/ito-audit.sh",
            );
            skills(&mut entries, tool, ".claude/skills");
            commands(&mut entries, tool, ".claude/commands", CommandNaming::Plain);
        }
        HarnessTool::OpenCode => {
            adapter(
                &mut entries,
                tool,
                "opencode/ito-skills.js",
                ".opencode/plugins/ito-skills.js",
            );
            skills(&mut entries, tool, ".opencode/skills");
            commands(
                &mut entries,
                tool,
                ".opencode/commands",
                CommandNaming::Plain,
            );
        }
        HarnessTool::Codex => {
            adapter(
                &mut entries,
                tool,
                "codex/ito-skills-bootstrap.md",
                ".codex/instructions/ito-skills-bootstrap.md",
            );
            skills(&mut entries, tool, ".codex/skills");
            commands(&mut entries, tool, ".codex/prompts", CommandNaming::Plain);
        }
        HarnessTool::GitHubCopilot => {
            skills(&mut entries, tool, ".github/skills");
            commands(
                &mut entries,
                tool,
                ".github/prompts",
                CommandNaming::PromptSuffix,
            );
        }
        HarnessTool::Pi => {
            adapter(
                &mut entries,
                tool,
                "pi/ito-skills.ts",
                ".pi/extensions/ito-skills.ts",
            );
            skills(&mut entries, tool, ".pi/skills");
            commands(&mut entries, tool, ".pi/commands", CommandNaming::Plain);
        }
    }
    entries
}

#[derive(Debug, Clone, Copy)]
enum CommandNaming {
    Plain,
    PromptSuffix,
}

fn adapter(entries: &mut Vec<ManifestEntry>, tool: HarnessTool, source_path: &str, dest: &str) {
    entries.push(ManifestEntry {
        relative_path: dest.to_string(),
        source_path: source_path.to_string(),
        source: ManifestSource::Adapter,
        harness: Some(tool),
    });
}

fn skills(entries: &mut Vec<ManifestEntry>, tool: HarnessTool, skills_dir: &str) {
    for file in skills_files() {
        let rel_path = file.relative_path;
        let mut parts = rel_path.split('/');
        let Some(skill_name) = parts.next() else {
            continue;
        };
        let dest_skill_name = if skill_name.starts_with("ito") {
            skill_name.to_string()
        } else {
            format!("ito-{skill_name}")
        };
        let rest = parts.collect::<Vec<_>>().join("/");
        let relative_path = if rest.is_empty() {
            format!("{skills_dir}/{dest_skill_name}")
        } else {
            format!("{skills_dir}/{dest_skill_name}/{rest}")
        };
        entries.push(ManifestEntry {
            relative_path,
            source_path: rel_path.to_string(),
            source: ManifestSource::Skill,
            harness: Some(tool),
        });
    }
}

fn commands(
    entries: &mut Vec<ManifestEntry>,
    tool: HarnessTool,
    commands_dir: &str,
    naming: CommandNaming,
) {
    for file in commands_files() {
        let rel_path = file.relative_path;
        let dest_name = match naming {
            CommandNaming::Plain => rel_path.to_string(),
            CommandNaming::PromptSuffix => rel_path
                .strip_suffix(".md")
                .map(|name| format!("{name}.prompt.md"))
                .unwrap_or_else(|| rel_path.to_string()),
        };
        entries.push(ManifestEntry {
            relative_path: format!("{commands_dir}/{dest_name}"),
            source_path: rel_path.to_string(),
            source: ManifestSource::Command,
            harness: Some(tool),
        });
    }
}

#[cfg(test)]
#[path = "manifest_tests.rs"]
mod manifest_tests;
