//! Embedded asset distribution helpers.
//!
//! This module builds install manifests for the various harnesses Ito supports.
//! The manifests map a file embedded in `ito-templates` to a destination path on
//! disk.

use crate::errors::{CoreError, CoreResult};
use ito_templates::{
    commands_files, get_adapter_file, get_command_file, get_skill_file, skills_files,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
/// One file to be installed from embedded assets.
pub struct FileManifest {
    /// Source path relative to embedded assets (e.g., "brainstorming/SKILL.md" for skills)
    pub source: String,
    /// Destination path on disk
    pub dest: PathBuf,
    /// Asset type determines which embedded directory to read from
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Category of embedded asset.
pub enum AssetType {
    /// A skill markdown file.
    Skill,
    /// A tool-specific adapter/bootstrap file.
    Adapter,
    /// A command/prompt template.
    Command,
}

/// Returns manifest entries for all ito-skills.
/// Source paths are relative to assets/skills/ (e.g., "brainstorming/SKILL.md")
/// Dest paths have ito- prefix added if not already present
/// (e.g., "brainstorming/SKILL.md" -> "ito-brainstorming/SKILL.md")
/// (e.g., "ito/SKILL.md" -> "ito/SKILL.md" - no double prefix)
fn ito_skills_manifests(skills_dir: &Path) -> Vec<FileManifest> {
    let mut manifests = Vec::new();

    // Get all skill files from embedded assets
    for file in skills_files() {
        let rel_path = file.relative_path;
        // Extract skill name from path (e.g., "brainstorming/SKILL.md" -> "brainstorming")
        let parts: Vec<&str> = rel_path.split('/').collect();
        if parts.is_empty() {
            continue;
        }
        let skill_name = parts[0];

        // Build destination path, adding ito- prefix only if not already present
        let dest_skill_name = if skill_name.starts_with("ito") {
            skill_name.to_string()
        } else {
            format!("ito-{}", skill_name)
        };

        let rest = if parts.len() > 1 {
            parts[1..].join("/")
        } else {
            rel_path.to_string()
        };
        let dest = skills_dir.join(format!("{}/{}", dest_skill_name, rest));

        manifests.push(FileManifest {
            source: rel_path.to_string(),
            dest,
            asset_type: AssetType::Skill,
        });
    }

    manifests
}

/// Returns manifest entries for all ito commands.
/// Commands are copied directly to the commands directory with their original names.
fn ito_commands_manifests(commands_dir: &Path) -> Vec<FileManifest> {
    let mut manifests = Vec::new();

    for file in commands_files() {
        let rel_path = file.relative_path;
        manifests.push(FileManifest {
            source: rel_path.to_string(),
            dest: commands_dir.join(rel_path),
            asset_type: AssetType::Command,
        });
    }

    manifests
}

/// Return manifest entries for OpenCode template installation.
///
/// OpenCode stores its configuration under a single directory (typically
/// `~/.config/opencode/`). We install an Ito plugin along with a flat list of
/// skills and commands.
pub fn opencode_manifests(config_dir: &Path) -> Vec<FileManifest> {
    let mut out = Vec::new();

    out.push(FileManifest {
        source: "opencode/ito-skills.js".to_string(),
        dest: config_dir.join("plugins").join("ito-skills.js"),
        asset_type: AssetType::Adapter,
    });

    // Skills go directly under skills/ (flat structure with ito- prefix)
    let skills_dir = config_dir.join("skills");
    out.extend(ito_skills_manifests(&skills_dir));

    // Commands go under commands/
    let commands_dir = config_dir.join("commands");
    out.extend(ito_commands_manifests(&commands_dir));

    out
}

/// Return manifest entries for Claude Code template installation.
pub fn claude_manifests(project_root: &Path) -> Vec<FileManifest> {
    let mut out = vec![FileManifest {
        source: "claude/session-start.sh".to_string(),
        dest: project_root.join(".claude").join("session-start.sh"),
        asset_type: AssetType::Adapter,
    }];

    // Skills go directly under .claude/skills/ (flat structure with ito- prefix)
    let skills_dir = project_root.join(".claude").join("skills");
    out.extend(ito_skills_manifests(&skills_dir));

    // Commands go under .claude/commands/
    let commands_dir = project_root.join(".claude").join("commands");
    out.extend(ito_commands_manifests(&commands_dir));

    out
}

/// Return manifest entries for Codex template installation.
pub fn codex_manifests(project_root: &Path) -> Vec<FileManifest> {
    let mut out = vec![FileManifest {
        source: "codex/ito-skills-bootstrap.md".to_string(),
        dest: project_root
            .join(".codex")
            .join("instructions")
            .join("ito-skills-bootstrap.md"),
        asset_type: AssetType::Adapter,
    }];

    // Skills go directly under .codex/skills/ (flat structure with ito- prefix)
    let skills_dir = project_root.join(".codex").join("skills");
    out.extend(ito_skills_manifests(&skills_dir));

    // Commands go under .codex/prompts/ (Codex uses "prompts" terminology)
    let commands_dir = project_root.join(".codex").join("prompts");
    out.extend(ito_commands_manifests(&commands_dir));

    out
}

/// Return manifest entries for GitHub Copilot template installation.
pub fn github_manifests(project_root: &Path) -> Vec<FileManifest> {
    // Skills go directly under .github/skills/ (flat structure with ito- prefix)
    let skills_dir = project_root.join(".github").join("skills");
    let mut out = ito_skills_manifests(&skills_dir);

    // Commands go under .github/prompts/ (GitHub uses "prompts" terminology)
    // Note: GitHub Copilot uses .prompt.md suffix convention
    let prompts_dir = project_root.join(".github").join("prompts");
    for file in commands_files() {
        let rel_path = file.relative_path;
        // Convert ito-apply.md -> ito-apply.prompt.md for GitHub
        let dest_name = if let Some(stripped) = rel_path.strip_suffix(".md") {
            format!("{stripped}.prompt.md")
        } else {
            rel_path.to_string()
        };
        out.push(FileManifest {
            source: rel_path.to_string(),
            dest: prompts_dir.join(dest_name),
            asset_type: AssetType::Command,
        });
    }

    out
}

/// Install manifests from embedded assets to disk.
pub fn install_manifests(manifests: &[FileManifest]) -> CoreResult<()> {
    for manifest in manifests {
        let bytes = match manifest.asset_type {
            AssetType::Skill => get_skill_file(&manifest.source).ok_or_else(|| {
                CoreError::NotFound(format!(
                    "Skill file not found in embedded assets: {}",
                    manifest.source
                ))
            })?,
            AssetType::Adapter => get_adapter_file(&manifest.source).ok_or_else(|| {
                CoreError::NotFound(format!(
                    "Adapter file not found in embedded assets: {}",
                    manifest.source
                ))
            })?,
            AssetType::Command => get_command_file(&manifest.source).ok_or_else(|| {
                CoreError::NotFound(format!(
                    "Command file not found in embedded assets: {}",
                    manifest.source
                ))
            })?,
        };

        if let Some(parent) = manifest.dest.parent() {
            ito_common::io::create_dir_all_std(parent).map_err(|e| {
                CoreError::io(format!("creating directory {}", parent.display()), e)
            })?;
        }
        ito_common::io::write_std(&manifest.dest, bytes)
            .map_err(|e| CoreError::io(format!("writing {}", manifest.dest.display()), e))?;
    }
    Ok(())
}
