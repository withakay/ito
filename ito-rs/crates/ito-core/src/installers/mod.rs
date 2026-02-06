use std::collections::BTreeSet;
use std::path::Path;

use chrono::Utc;

use crate::errors::{CoreError, CoreResult};

use markers::update_file_with_markers;

mod markers;

use ito_config::ConfigContext;
use ito_config::ito_dir::get_ito_dir_name;

/// Tool id for Claude Code.
pub const TOOL_CLAUDE: &str = "claude";
/// Tool id for Codex.
pub const TOOL_CODEX: &str = "codex";
/// Tool id for GitHub Copilot.
pub const TOOL_GITHUB_COPILOT: &str = "github-copilot";
/// Tool id for OpenCode.
pub const TOOL_OPENCODE: &str = "opencode";

/// Return the set of supported tool ids.
pub fn available_tool_ids() -> &'static [&'static str] {
    &[TOOL_CLAUDE, TOOL_CODEX, TOOL_GITHUB_COPILOT, TOOL_OPENCODE]
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options that control template installation behavior.
pub struct InitOptions {
    /// Selected tool ids.
    pub tools: BTreeSet<String>,
    /// Overwrite existing files when `true`.
    pub force: bool,
}

impl InitOptions {
    /// Create new init options.
    pub fn new(tools: BTreeSet<String>, force: bool) -> Self {
        Self { tools, force }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Installation mode used by the installer.
pub enum InstallMode {
    /// Initial installation (`ito init`).
    Init,
    /// Update installation (`ito update`).
    Update,
}

/// Install the default project templates and selected tool adapters.
pub fn install_default_templates(
    project_root: &Path,
    ctx: &ConfigContext,
    mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    let ito_dir_name = get_ito_dir_name(project_root, ctx);
    let ito_dir = ito_templates::normalize_ito_dir(&ito_dir_name);

    install_project_templates(project_root, &ito_dir, mode, opts)?;

    // Repository-local ignore rule for session state.
    // This is not a templated file: we update `.gitignore` directly to preserve existing content.
    if mode == InstallMode::Init {
        ensure_repo_gitignore_ignores_session_json(project_root, &ito_dir)?;
    }

    install_adapter_files(project_root, mode, opts)?;
    install_agent_templates(project_root, mode, opts)?;
    Ok(())
}

fn ensure_repo_gitignore_ignores_session_json(
    project_root: &Path,
    ito_dir: &str,
) -> CoreResult<()> {
    let entry = format!("{ito_dir}/session.json");
    ensure_gitignore_contains_line(project_root, &entry)
}

fn ensure_gitignore_contains_line(project_root: &Path, entry: &str) -> CoreResult<()> {
    let path = project_root.join(".gitignore");
    let existing = match ito_common::io::read_to_string_std(&path) {
        Ok(s) => Some(s),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(CoreError::io(format!("reading {}", path.display()), e)),
    };

    let Some(mut s) = existing else {
        ito_common::io::write_std(&path, format!("{entry}\n"))
            .map_err(|e| CoreError::io(format!("writing {}", path.display()), e))?;
        return Ok(());
    };

    if gitignore_has_exact_line(&s, entry) {
        return Ok(());
    }

    if !s.ends_with('\n') {
        s.push('\n');
    }
    s.push_str(entry);
    s.push('\n');

    ito_common::io::write_std(&path, s)
        .map_err(|e| CoreError::io(format!("writing {}", path.display()), e))?;
    Ok(())
}

fn gitignore_has_exact_line(contents: &str, entry: &str) -> bool {
    contents.lines().map(|l| l.trim()).any(|l| l == entry)
}

fn install_project_templates(
    project_root: &Path,
    ito_dir: &str,
    mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    let selected = &opts.tools;
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let state_rel = format!("{ito_dir}/planning/STATE.md");

    for f in ito_templates::default_project_files() {
        let rel = ito_templates::render_rel_path(f.relative_path, ito_dir);
        if !should_install_project_rel(rel.as_ref(), selected) {
            continue;
        }

        let mut bytes = ito_templates::render_bytes(f.contents, ito_dir).into_owned();
        if rel.as_ref() == state_rel
            && let Ok(s) = std::str::from_utf8(&bytes)
        {
            bytes = s.replace("__CURRENT_DATE__", &current_date).into_bytes();
        }
        let target = project_root.join(rel.as_ref());
        write_one(&target, &bytes, mode, opts)?;
    }

    Ok(())
}

fn should_install_project_rel(rel: &str, tools: &BTreeSet<String>) -> bool {
    // Always install Ito project assets.
    if rel == "AGENTS.md" {
        return true;
    }
    if rel.starts_with(".ito/") {
        return true;
    }

    // Tool-specific assets.
    if rel == "CLAUDE.md" || rel.starts_with(".claude/") {
        return tools.contains(TOOL_CLAUDE);
    }
    if rel.starts_with(".opencode/") {
        return tools.contains(TOOL_OPENCODE);
    }
    if rel.starts_with(".github/") {
        return tools.contains(TOOL_GITHUB_COPILOT);
    }
    if rel.starts_with(".codex/") {
        return tools.contains(TOOL_CODEX);
    }

    // Unknown/unclassified: only install when tools=all (caller controls via set contents).
    false
}

fn write_one(
    target: &Path,
    rendered_bytes: &[u8],
    mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    if let Some(parent) = target.parent() {
        ito_common::io::create_dir_all_std(parent)
            .map_err(|e| CoreError::io(format!("creating directory {}", parent.display()), e))?;
    }

    // Marker-managed files: template contains markers; we extract the inner block.
    if let Ok(text) = std::str::from_utf8(rendered_bytes)
        && let Some(block) = ito_templates::extract_managed_block(text)
    {
        if target.exists() {
            if mode == InstallMode::Init && !opts.force {
                // If the file exists but doesn't contain Ito markers, mimic TS init behavior:
                // refuse to overwrite without --force.
                let existing = ito_common::io::read_to_string_or_default(target);
                let has_start = existing.contains(ito_templates::ITO_START_MARKER);
                let has_end = existing.contains(ito_templates::ITO_END_MARKER);
                if !(has_start && has_end) {
                    return Err(CoreError::Validation(format!(
                        "Refusing to overwrite existing file without markers: {} (re-run with --force)",
                        target.display()
                    )));
                }
            }

            update_file_with_markers(
                target,
                block,
                ito_templates::ITO_START_MARKER,
                ito_templates::ITO_END_MARKER,
            )
            .map_err(|e| match e {
                markers::FsEditError::Io(io_err) => {
                    CoreError::io(format!("updating markers in {}", target.display()), io_err)
                }
                markers::FsEditError::Marker(marker_err) => CoreError::Validation(format!(
                    "Failed to update markers in {}: {}",
                    target.display(),
                    marker_err
                )),
            })?;
        } else {
            // New file: write the template bytes verbatim so output matches embedded assets.
            ito_common::io::write_std(target, rendered_bytes)
                .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
        }

        return Ok(());
    }

    // Non-marker-managed files: init refuses to overwrite unless --force.
    if mode == InstallMode::Init && target.exists() && !opts.force {
        return Err(CoreError::Validation(format!(
            "Refusing to overwrite existing file without markers: {} (re-run with --force)",
            target.display()
        )));
    }

    ito_common::io::write_std(target, rendered_bytes)
        .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
    Ok(())
}

fn install_adapter_files(
    project_root: &Path,
    _mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    for tool in &opts.tools {
        match tool.as_str() {
            TOOL_OPENCODE => {
                let config_dir = project_root.join(".opencode");
                let manifests = crate::distribution::opencode_manifests(&config_dir);
                crate::distribution::install_manifests(&manifests)?;
            }
            TOOL_CLAUDE => {
                let manifests = crate::distribution::claude_manifests(project_root);
                crate::distribution::install_manifests(&manifests)?;
            }
            TOOL_CODEX => {
                let manifests = crate::distribution::codex_manifests(project_root);
                crate::distribution::install_manifests(&manifests)?;
            }
            TOOL_GITHUB_COPILOT => {
                let manifests = crate::distribution::github_manifests(project_root);
                crate::distribution::install_manifests(&manifests)?;
            }
            _ => {}
        }
    }

    Ok(())
}

/// Install Ito agent templates (ito-quick, ito-general, ito-thinking)
fn install_agent_templates(
    project_root: &Path,
    mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    use ito_templates::agents::{
        AgentTier, Harness, default_agent_configs, get_agent_files, render_agent_template,
    };

    let configs = default_agent_configs();

    // Map tool names to harnesses
    let tool_harness_map = [
        (TOOL_OPENCODE, Harness::OpenCode),
        (TOOL_CLAUDE, Harness::ClaudeCode),
        (TOOL_CODEX, Harness::Codex),
        (TOOL_GITHUB_COPILOT, Harness::GitHubCopilot),
    ];

    for (tool_id, harness) in tool_harness_map {
        if !opts.tools.contains(tool_id) {
            continue;
        }

        let agent_dir = project_root.join(harness.project_agent_path());

        // Get agent template files for this harness
        let files = get_agent_files(harness);

        for (rel_path, contents) in files {
            let target = agent_dir.join(rel_path);

            // Parse the template and determine which tier it is
            let tier = if rel_path.contains("ito-quick") || rel_path.contains("quick") {
                Some(AgentTier::Quick)
            } else if rel_path.contains("ito-general") || rel_path.contains("general") {
                Some(AgentTier::General)
            } else if rel_path.contains("ito-thinking") || rel_path.contains("thinking") {
                Some(AgentTier::Thinking)
            } else {
                None
            };

            // Get config for this tier
            let config = tier.and_then(|t| configs.get(&(harness, t)));

            match mode {
                InstallMode::Init => {
                    // During init: skip if exists and not forced
                    if target.exists() && !opts.force {
                        continue;
                    }

                    // Render full template
                    let rendered = if let Some(cfg) = config {
                        if let Ok(template_str) = std::str::from_utf8(contents) {
                            render_agent_template(template_str, cfg).into_bytes()
                        } else {
                            contents.to_vec()
                        }
                    } else {
                        contents.to_vec()
                    };

                    // Ensure parent directory exists
                    if let Some(parent) = target.parent() {
                        ito_common::io::create_dir_all_std(parent).map_err(|e| {
                            CoreError::io(format!("creating directory {}", parent.display()), e)
                        })?;
                    }

                    ito_common::io::write_std(&target, rendered)
                        .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
                }
                InstallMode::Update => {
                    // During update: only update model in existing ito agent files
                    if !target.exists() {
                        // File doesn't exist, create it
                        let rendered = if let Some(cfg) = config {
                            if let Ok(template_str) = std::str::from_utf8(contents) {
                                render_agent_template(template_str, cfg).into_bytes()
                            } else {
                                contents.to_vec()
                            }
                        } else {
                            contents.to_vec()
                        };

                        if let Some(parent) = target.parent() {
                            ito_common::io::create_dir_all_std(parent).map_err(|e| {
                                CoreError::io(format!("creating directory {}", parent.display()), e)
                            })?;
                        }
                        ito_common::io::write_std(&target, rendered).map_err(|e| {
                            CoreError::io(format!("writing {}", target.display()), e)
                        })?;
                    } else if let Some(cfg) = config {
                        // File exists, only update model field in frontmatter
                        update_agent_model_field(&target, &cfg.model)?;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Update only the model field in an existing agent file's frontmatter
fn update_agent_model_field(path: &Path, new_model: &str) -> CoreResult<()> {
    let content = ito_common::io::read_to_string_or_default(path);

    // Only update files with frontmatter
    if !content.starts_with("---") {
        return Ok(());
    }

    // Find frontmatter boundaries
    let rest = &content[3..];
    let Some(end_idx) = rest.find("\n---") else {
        return Ok(());
    };

    let frontmatter = &rest[..end_idx];
    let body = &rest[end_idx + 4..]; // Skip "\n---"

    // Update model field in frontmatter using simple string replacement
    let updated_frontmatter = update_model_in_yaml(frontmatter, new_model);

    // Reconstruct file
    let updated = format!("---{}\n---{}", updated_frontmatter, body);
    ito_common::io::write_std(path, updated)
        .map_err(|e| CoreError::io(format!("writing {}", path.display()), e))?;

    Ok(())
}

/// Update the model field in YAML frontmatter string
fn update_model_in_yaml(yaml: &str, new_model: &str) -> String {
    let mut lines: Vec<String> = yaml.lines().map(|l| l.to_string()).collect();
    let mut found = false;

    for line in &mut lines {
        if line.trim_start().starts_with("model:") {
            *line = format!("model: \"{}\"", new_model);
            found = true;
            break;
        }
    }

    // If no model field found, add it
    if !found {
        lines.push(format!("model: \"{}\"", new_model));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gitignore_created_when_missing() {
        let td = tempfile::tempdir().unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert_eq!(s, ".ito/session.json\n");
    }

    #[test]
    fn gitignore_noop_when_already_present() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(td.path().join(".gitignore"), ".ito/session.json\n").unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert_eq!(s, ".ito/session.json\n");
    }

    #[test]
    fn gitignore_does_not_duplicate_on_repeated_calls() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(td.path().join(".gitignore"), "node_modules\n").unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert_eq!(s, "node_modules\n.ito/session.json\n");
    }

    #[test]
    fn gitignore_preserves_existing_content_and_adds_newline_if_missing() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(td.path().join(".gitignore"), "node_modules").unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert_eq!(s, "node_modules\n.ito/session.json\n");
    }
}
