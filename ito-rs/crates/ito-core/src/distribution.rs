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

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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
    let mut out = vec![
        FileManifest {
            source: "claude/session-start.sh".to_string(),
            dest: project_root.join(".claude").join("session-start.sh"),
            asset_type: AssetType::Adapter,
        },
        FileManifest {
            source: "claude/hooks/ito-audit.sh".to_string(),
            dest: project_root
                .join(".claude")
                .join("hooks")
                .join("ito-audit.sh"),
            asset_type: AssetType::Adapter,
        },
    ];

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

/// Return manifest entries for Pi coding agent template installation.
///
/// Pi gets its own copy of skills and commands under `.pi/` so it is fully
/// self-contained — users can install Pi without OpenCode. The skills and
/// commands are read from the same shared embedded assets used by every harness.
pub fn pi_manifests(project_root: &Path) -> Vec<FileManifest> {
    let mut out = vec![FileManifest {
        source: "pi/ito-skills.ts".to_string(),
        dest: project_root
            .join(".pi")
            .join("extensions")
            .join("ito-skills.ts"),
        asset_type: AssetType::Adapter,
    }];

    // Skills go under .pi/skills/ (flat structure with ito- prefix)
    let skills_dir = project_root.join(".pi").join("skills");
    out.extend(ito_skills_manifests(&skills_dir));

    // Commands go under .pi/commands/
    let commands_dir = project_root.join(".pi").join("commands");
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
///
/// When `worktree_ctx` is `Some`, the `using-git-worktrees` skill template is
/// rendered with the given worktree configuration before writing. Other skill
/// files (which may contain `{{` as user-facing prompt placeholders) are written
/// as-is.
pub fn install_manifests(
    manifests: &[FileManifest],
    worktree_ctx: Option<&ito_templates::project_templates::WorktreeTemplateContext>,
) -> CoreResult<()> {
    use ito_templates::project_templates::{WorktreeTemplateContext, render_project_template};

    let default_ctx = WorktreeTemplateContext::default();
    let ctx = worktree_ctx.unwrap_or(&default_ctx);

    for manifest in manifests {
        let raw_bytes = match manifest.asset_type {
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

        // Render skill templates that opt into worktree Jinja2 variables. We
        // intentionally avoid rendering arbitrary `{{ ... }}` placeholders used
        // by non-template skills (e.g. research prompts).
        let mut should_render_skill = false;
        if manifest.asset_type == AssetType::Skill {
            for line in raw_bytes.split(|b| *b == b'\n') {
                let Ok(line) = std::str::from_utf8(line) else {
                    continue;
                };
                if skill_line_uses_worktree_template_syntax(line) {
                    should_render_skill = true;
                    break;
                }
            }
        }

        let bytes = if should_render_skill {
            render_project_template(raw_bytes, ctx).map_err(|e| {
                CoreError::Validation(format!(
                    "Failed to render skill template {}: {}",
                    manifest.source, e
                ))
            })?
        } else {
            raw_bytes.to_vec()
        };

        if let Some(parent) = manifest.dest.parent() {
            ito_common::io::create_dir_all_std(parent).map_err(|e| {
                CoreError::io(format!("creating directory {}", parent.display()), e)
            })?;
        }
        ito_common::io::write_std(&manifest.dest, &bytes)
            .map_err(|e| CoreError::io(format!("writing {}", manifest.dest.display()), e))?;
        ensure_manifest_script_is_executable(manifest)?;
    }
    Ok(())
}

fn ensure_manifest_script_is_executable(manifest: &FileManifest) -> CoreResult<()> {
    #[cfg(unix)]
    {
        let is_skill_script = manifest.asset_type == AssetType::Skill
            && manifest.source.ends_with(".sh")
            && manifest.source.contains("/scripts/");

        if is_skill_script {
            let metadata = std::fs::metadata(&manifest.dest).map_err(|e| {
                CoreError::io(
                    format!("reading metadata for {}", manifest.dest.display()),
                    e,
                )
            })?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(permissions.mode() | 0o755);
            std::fs::set_permissions(&manifest.dest, permissions).map_err(|e| {
                CoreError::io(
                    format!(
                        "setting executable permissions on {}",
                        manifest.dest.display()
                    ),
                    e,
                )
            })?;
        }
    }

    Ok(())
}

fn skill_line_uses_worktree_template_syntax(line: &str) -> bool {
    if line.contains("{%") {
        return true;
    }

    // Variable-only templates are supported for the worktree context keys.
    const WORKTREE_VARS: &[&str] = &[
        "{{ enabled",
        "{{ strategy",
        "{{ layout_dir_name",
        "{{ integration_mode",
        "{{ default_branch",
    ];

    for var in WORKTREE_VARS {
        if line.contains(var) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pi_manifests_includes_adapter_skills_and_commands() {
        let root = Path::new("/tmp/project");
        let manifests = pi_manifests(root);

        // Must contain the adapter extension.
        let adapter = manifests
            .iter()
            .find(|m| m.asset_type == AssetType::Adapter);
        assert!(adapter.is_some(), "pi_manifests must include the adapter");
        let adapter = adapter.unwrap();
        assert_eq!(adapter.source, "pi/ito-skills.ts");
        assert!(
            adapter.dest.ends_with(".pi/extensions/ito-skills.ts"),
            "adapter dest should end with .pi/extensions/ito-skills.ts, got {:?}",
            adapter.dest
        );

        // Must contain skill entries under .pi/skills/.
        let skills: Vec<_> = manifests
            .iter()
            .filter(|m| m.asset_type == AssetType::Skill)
            .collect();
        assert!(
            !skills.is_empty(),
            "pi_manifests must include skill entries"
        );
        for skill in &skills {
            let dest_str = skill.dest.to_string_lossy();
            assert!(
                dest_str.contains(".pi/skills/"),
                "skill dest should be under .pi/skills/, got: {}",
                dest_str
            );
        }

        // Must contain command entries under .pi/commands/.
        let commands: Vec<_> = manifests
            .iter()
            .filter(|m| m.asset_type == AssetType::Command)
            .collect();
        assert!(
            !commands.is_empty(),
            "pi_manifests must include command entries"
        );
        for cmd in &commands {
            let dest_str = cmd.dest.to_string_lossy();
            assert!(
                dest_str.contains(".pi/commands/"),
                "command dest should be under .pi/commands/, got: {}",
                dest_str
            );
        }
    }

    #[test]
    fn pi_adapter_asset_exists_in_embedded_templates() {
        let contents = ito_templates::get_adapter_file("pi/ito-skills.ts");
        assert!(
            contents.is_some(),
            "pi/ito-skills.ts must be present in embedded adapter assets"
        );
        let bytes = contents.unwrap();
        assert!(!bytes.is_empty());
        let text = std::str::from_utf8(bytes).expect("adapter should be valid UTF-8");
        assert!(
            text.contains("ExtensionAPI"),
            "Pi adapter should import the Pi ExtensionAPI type"
        );
        assert!(
            text.contains(r#""--tool", "pi""#),
            "Pi adapter must request bootstrap with --tool pi (not opencode or other)"
        );
        assert!(
            !text.contains(r#""--tool", "opencode""#),
            "Pi adapter must not reference opencode tool type"
        );
        // Verify unused imports are not present.
        assert!(
            !text.contains("import path from"),
            "Pi adapter should not have unused path import"
        );
        assert!(
            !text.contains("import fs from"),
            "Pi adapter should not have unused fs import"
        );
    }

    #[test]
    fn pi_manifests_skills_match_opencode_skills() {
        // Pi and OpenCode should install the same set of skills from the
        // shared embedded source — only the destination directory differs.
        let root = Path::new("/home/user/myproject");
        let pi = pi_manifests(root);
        let oc_dir = root.join(".opencode");
        let oc = opencode_manifests(&oc_dir);

        let pi_skill_sources: std::collections::BTreeSet<_> = pi
            .iter()
            .filter(|m| m.asset_type == AssetType::Skill)
            .map(|m| m.source.clone())
            .collect();
        let oc_skill_sources: std::collections::BTreeSet<_> = oc
            .iter()
            .filter(|m| m.asset_type == AssetType::Skill)
            .map(|m| m.source.clone())
            .collect();

        assert_eq!(
            pi_skill_sources, oc_skill_sources,
            "Pi and OpenCode should install identical skill sources"
        );
    }

    #[test]
    fn pi_agent_templates_discoverable() {
        use ito_templates::agents::{Harness, get_agent_files};
        let files = get_agent_files(Harness::Pi);
        let names: Vec<_> = files.iter().map(|(name, _)| *name).collect();
        assert!(
            names.contains(&"ito-quick.md"),
            "Pi agent templates must include ito-quick.md, got: {:?}",
            names
        );
        assert!(
            names.contains(&"ito-general.md"),
            "Pi agent templates must include ito-general.md, got: {:?}",
            names
        );
        assert!(
            names.contains(&"ito-thinking.md"),
            "Pi agent templates must include ito-thinking.md, got: {:?}",
            names
        );
    }

    #[test]
    fn pi_manifests_commands_match_opencode_commands() {
        // Pi and OpenCode should install the same set of commands from the
        // shared embedded source — only the destination directory differs.
        let root = Path::new("/home/user/myproject");
        let pi = pi_manifests(root);
        let oc_dir = root.join(".opencode");
        let oc = opencode_manifests(&oc_dir);

        let pi_cmd_sources: std::collections::BTreeSet<_> = pi
            .iter()
            .filter(|m| m.asset_type == AssetType::Command)
            .map(|m| m.source.clone())
            .collect();
        let oc_cmd_sources: std::collections::BTreeSet<_> = oc
            .iter()
            .filter(|m| m.asset_type == AssetType::Command)
            .map(|m| m.source.clone())
            .collect();

        assert_eq!(
            pi_cmd_sources, oc_cmd_sources,
            "Pi and OpenCode should install identical command sources"
        );
    }
}
