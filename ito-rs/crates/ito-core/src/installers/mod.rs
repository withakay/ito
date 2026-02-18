use std::collections::BTreeSet;
use std::path::Path;

use chrono::Utc;
use serde_json::{Map, Value};

use crate::errors::{CoreError, CoreResult};

use markers::update_file_with_markers;

mod markers;

use ito_config::ConfigContext;
use ito_config::ito_dir::get_ito_dir_name;
use ito_templates::project_templates::WorktreeTemplateContext;

/// Tool id for Claude Code.
pub const TOOL_CLAUDE: &str = "claude";
/// Tool id for Codex.
pub const TOOL_CODEX: &str = "codex";
/// Tool id for GitHub Copilot.
pub const TOOL_GITHUB_COPILOT: &str = "github-copilot";
/// Tool id for OpenCode.
pub const TOOL_OPENCODE: &str = "opencode";

const CONFIG_SCHEMA_RELEASE_TAG_PLACEHOLDER: &str = "__ITO_RELEASE_TAG__";

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
    /// When `true`, update managed files while preserving user-edited files.
    ///
    /// In this mode, non-marker files that already exist are silently skipped
    /// instead of triggering an error. Marker-managed files still get their
    /// managed blocks updated. Adapter files, skills, and commands are
    /// overwritten as usual.
    pub update: bool,
}

impl InitOptions {
    /// Create new init options.
    pub fn new(tools: BTreeSet<String>, force: bool, update: bool) -> Self {
        Self {
            tools,
            force,
            update,
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileOwnership {
    ItoManaged,
    UserOwned,
}

/// Install the default project templates and selected tool adapters.
///
/// When `worktree_ctx` is `Some`, templates containing Jinja2 syntax will be
/// rendered with the given worktree configuration. When `None`, a disabled
/// default context is used.
pub fn install_default_templates(
    project_root: &Path,
    ctx: &ConfigContext,
    mode: InstallMode,
    opts: &InitOptions,
    worktree_ctx: Option<&WorktreeTemplateContext>,
) -> CoreResult<()> {
    let ito_dir_name = get_ito_dir_name(project_root, ctx);
    let ito_dir = ito_templates::normalize_ito_dir(&ito_dir_name);

    install_project_templates(project_root, &ito_dir, mode, opts, worktree_ctx)?;

    // Repository-local ignore rules for per-worktree state.
    // This is not a templated file: we update `.gitignore` directly to preserve existing content.
    if mode == InstallMode::Init {
        ensure_repo_gitignore_ignores_session_json(project_root, &ito_dir)?;
        ensure_repo_gitignore_ignores_audit_session(project_root, &ito_dir)?;
        // Un-ignore audit event log so it is git-tracked even if .state/ is broadly ignored.
        ensure_repo_gitignore_unignores_audit_events(project_root, &ito_dir)?;
    }

    // Local (per-developer) config overlays should never be committed.
    ensure_repo_gitignore_ignores_local_configs(project_root, &ito_dir)?;

    install_adapter_files(project_root, mode, opts, worktree_ctx)?;
    install_agent_templates(project_root, mode, opts)?;
    Ok(())
}

fn ensure_repo_gitignore_ignores_local_configs(
    project_root: &Path,
    ito_dir: &str,
) -> CoreResult<()> {
    // Strategy/worktree settings are often personal preferences; users can keep
    // them in a local overlay file.
    let entry = format!("{ito_dir}/config.local.json");
    ensure_gitignore_contains_line(project_root, &entry)?;

    // Optional convention: keep local configs under `.local/`.
    let entry = ".local/ito/config.json";
    ensure_gitignore_contains_line(project_root, entry)?;
    Ok(())
}

fn ensure_repo_gitignore_ignores_session_json(
    project_root: &Path,
    ito_dir: &str,
) -> CoreResult<()> {
    let entry = format!("{ito_dir}/session.json");
    ensure_gitignore_contains_line(project_root, &entry)
}

/// Ensure `.ito/.state/audit/.session` is gitignored (per-worktree UUID).
fn ensure_repo_gitignore_ignores_audit_session(
    project_root: &Path,
    ito_dir: &str,
) -> CoreResult<()> {
    let entry = format!("{ito_dir}/.state/audit/.session");
    ensure_gitignore_contains_line(project_root, &entry)
}

/// Un-ignore the audit events directory so `events.jsonl` is git-tracked.
///
/// If `.ito/.state/` is broadly gitignored (e.g., by a user rule or template),
/// we add `!.ito/.state/audit/` to override the ignore and ensure the audit
/// event log is committed alongside other project artifacts.
fn ensure_repo_gitignore_unignores_audit_events(
    project_root: &Path,
    ito_dir: &str,
) -> CoreResult<()> {
    let entry = format!("!{ito_dir}/.state/audit/");
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
    worktree_ctx: Option<&WorktreeTemplateContext>,
) -> CoreResult<()> {
    use ito_templates::project_templates::render_project_template;

    let selected = &opts.tools;
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let state_rel = format!("{ito_dir}/planning/STATE.md");
    let config_json_rel = format!("{ito_dir}/config.json");
    let release_tag = release_tag();
    let default_ctx = WorktreeTemplateContext::default();
    let ctx = worktree_ctx.unwrap_or(&default_ctx);

    for f in ito_templates::default_project_files() {
        let rel = ito_templates::render_rel_path(f.relative_path, ito_dir);
        let rel = rel.as_ref();

        if !should_install_project_rel(rel, selected) {
            continue;
        }

        let mut bytes = ito_templates::render_bytes(f.contents, ito_dir).into_owned();
        if let Ok(s) = std::str::from_utf8(&bytes) {
            if rel == state_rel {
                bytes = s.replace("__CURRENT_DATE__", &current_date).into_bytes();
            } else if rel == config_json_rel {
                bytes = s
                    .replace(CONFIG_SCHEMA_RELEASE_TAG_PLACEHOLDER, &release_tag)
                    .into_bytes();
            }
        }

        // Render worktree-aware project templates (AGENTS.md) with worktree
        // config. Only AGENTS.md uses Jinja2 for worktree rendering; other
        // files (e.g., .ito/commands/) may contain `{{` as user-facing prompt
        // placeholders that must NOT be processed by minijinja.
        if rel == "AGENTS.md" {
            bytes = render_project_template(&bytes, ctx).map_err(|e| {
                CoreError::Validation(format!("Failed to render template {rel}: {e}"))
            })?;
        }

        let ownership = classify_project_file_ownership(rel, ito_dir);

        let target = project_root.join(rel);
        if rel == ".claude/settings.json" {
            write_claude_settings(&target, &bytes, mode, opts)?;
            continue;
        }
        write_one(&target, &bytes, mode, opts, ownership)?;
    }

    Ok(())
}

fn release_tag() -> String {
    let version = option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    if version.starts_with('v') {
        return version.to_string();
    }

    format!("v{version}")
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

fn classify_project_file_ownership(rel: &str, ito_dir: &str) -> FileOwnership {
    let project_md_rel = format!("{ito_dir}/project.md");
    if rel == project_md_rel {
        return FileOwnership::UserOwned;
    }

    let config_json_rel = format!("{ito_dir}/config.json");
    if rel == config_json_rel {
        return FileOwnership::UserOwned;
    }

    let user_guidance_rel = format!("{ito_dir}/user-guidance.md");
    if rel == user_guidance_rel {
        return FileOwnership::UserOwned;
    }

    let user_prompts_prefix = format!("{ito_dir}/user-prompts/");
    if rel.starts_with(&user_prompts_prefix) {
        return FileOwnership::UserOwned;
    }

    FileOwnership::ItoManaged
}

fn write_one(
    target: &Path,
    rendered_bytes: &[u8],
    mode: InstallMode,
    opts: &InitOptions,
    ownership: FileOwnership,
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
            if mode == InstallMode::Init && opts.force {
                ito_common::io::write_std(target, rendered_bytes)
                    .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
                return Ok(());
            }

            if mode == InstallMode::Init && !opts.force && !opts.update {
                // If the file exists but doesn't contain Ito markers, mimic TS init behavior:
                // refuse to overwrite without --force or --update.
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

    if target.exists() {
        match mode {
            InstallMode::Init => {
                if opts.force {
                    // --force always overwrites on init.
                } else if opts.update {
                    if ownership == FileOwnership::UserOwned {
                        return Ok(());
                    }
                } else {
                    return Err(CoreError::Validation(format!(
                        "Refusing to overwrite existing file without markers: {} (re-run with --force)",
                        target.display()
                    )));
                }
            }
            InstallMode::Update => {
                if ownership == FileOwnership::UserOwned {
                    return Ok(());
                }
            }
        }
    }

    ito_common::io::write_std(target, rendered_bytes)
        .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
    Ok(())
}

fn write_claude_settings(
    target: &Path,
    rendered_bytes: &[u8],
    mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    if let Some(parent) = target.parent() {
        ito_common::io::create_dir_all_std(parent)
            .map_err(|e| CoreError::io(format!("creating directory {}", parent.display()), e))?;
    }

    if mode == InstallMode::Init && target.exists() && !opts.force && !opts.update {
        return Err(CoreError::Validation(format!(
            "Refusing to overwrite existing file without markers: {} (re-run with --force)",
            target.display()
        )));
    }

    let template_value: Value = serde_json::from_slice(rendered_bytes).map_err(|e| {
        CoreError::Validation(format!(
            "Failed to parse Claude settings template {}: {}",
            target.display(),
            e
        ))
    })?;

    if !target.exists() || (mode == InstallMode::Init && opts.force) {
        let mut bytes = serde_json::to_vec_pretty(&template_value).map_err(|e| {
            CoreError::Validation(format!(
                "Failed to render Claude settings template {}: {}",
                target.display(),
                e
            ))
        })?;
        bytes.push(b'\n');
        ito_common::io::write_std(target, bytes)
            .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
        return Ok(());
    }

    let existing_raw = ito_common::io::read_to_string_std(target)
        .map_err(|e| CoreError::io(format!("reading {}", target.display()), e))?;
    let Ok(mut existing_value) = serde_json::from_str::<Value>(&existing_raw) else {
        // Preserve user-owned files that are not valid JSON during update flows.
        return Ok(());
    };

    merge_json_objects(&mut existing_value, &template_value);
    let mut merged = serde_json::to_vec_pretty(&existing_value).map_err(|e| {
        CoreError::Validation(format!(
            "Failed to render merged Claude settings {}: {}",
            target.display(),
            e
        ))
    })?;
    merged.push(b'\n');
    ito_common::io::write_std(target, merged)
        .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
    Ok(())
}

fn merge_json_objects(existing: &mut Value, template: &Value) {
    let Value::Object(template_map) = template else {
        *existing = template.clone();
        return;
    };
    if !existing.is_object() {
        *existing = Value::Object(Map::new());
    }

    let Some(existing_map) = existing.as_object_mut() else {
        return;
    };

    for (key, template_value) in template_map {
        if let Some(existing_value) = existing_map.get_mut(key) {
            merge_json_values(existing_value, template_value);
        } else {
            existing_map.insert(key.clone(), template_value.clone());
        }
    }
}

fn merge_json_values(existing: &mut Value, template: &Value) {
    match (existing, template) {
        (Value::Object(existing_map), Value::Object(template_map)) => {
            for (key, template_value) in template_map {
                if let Some(existing_value) = existing_map.get_mut(key) {
                    merge_json_values(existing_value, template_value);
                } else {
                    existing_map.insert(key.clone(), template_value.clone());
                }
            }
        }
        (Value::Array(existing_items), Value::Array(template_items)) => {
            for template_item in template_items {
                if !existing_items.contains(template_item) {
                    existing_items.push(template_item.clone());
                }
            }
        }
        (existing_value, template_value) => *existing_value = template_value.clone(),
    }
}

fn install_adapter_files(
    project_root: &Path,
    _mode: InstallMode,
    opts: &InitOptions,
    worktree_ctx: Option<&WorktreeTemplateContext>,
) -> CoreResult<()> {
    for tool in &opts.tools {
        match tool.as_str() {
            TOOL_OPENCODE => {
                let config_dir = project_root.join(".opencode");
                let manifests = crate::distribution::opencode_manifests(&config_dir);
                crate::distribution::install_manifests(&manifests, worktree_ctx)?;
            }
            TOOL_CLAUDE => {
                let manifests = crate::distribution::claude_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx)?;
            }
            TOOL_CODEX => {
                let manifests = crate::distribution::codex_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx)?;
            }
            TOOL_GITHUB_COPILOT => {
                let manifests = crate::distribution::github_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx)?;
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
                    if target.exists() {
                        if opts.update {
                            // --update: only update model field in existing agent files
                            if let Some(cfg) = config {
                                update_agent_model_field(&target, &cfg.model)?;
                            }
                            continue;
                        }
                        if !opts.force {
                            // Default init: skip existing files
                            continue;
                        }
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
    fn gitignore_audit_session_added() {
        let td = tempfile::tempdir().unwrap();
        ensure_repo_gitignore_ignores_audit_session(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert!(s.contains(".ito/.state/audit/.session"));
    }

    #[test]
    fn gitignore_both_session_entries() {
        let td = tempfile::tempdir().unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        ensure_repo_gitignore_ignores_audit_session(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert!(s.contains(".ito/session.json"));
        assert!(s.contains(".ito/.state/audit/.session"));
    }

    #[test]
    fn gitignore_preserves_existing_content_and_adds_newline_if_missing() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(td.path().join(".gitignore"), "node_modules").unwrap();
        ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert_eq!(s, "node_modules\n.ito/session.json\n");
    }

    #[test]
    fn gitignore_audit_events_unignored() {
        let td = tempfile::tempdir().unwrap();
        ensure_repo_gitignore_unignores_audit_events(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert!(s.contains("!.ito/.state/audit/"));
    }

    #[test]
    fn gitignore_full_audit_setup() {
        let td = tempfile::tempdir().unwrap();
        // Simulate a broad .state/ ignore
        std::fs::write(td.path().join(".gitignore"), ".ito/.state/\n").unwrap();
        ensure_repo_gitignore_ignores_audit_session(td.path(), ".ito").unwrap();
        ensure_repo_gitignore_unignores_audit_events(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert!(s.contains(".ito/.state/audit/.session"));
        assert!(s.contains("!.ito/.state/audit/"));
    }

    #[test]
    fn gitignore_ignores_local_configs() {
        let td = tempfile::tempdir().unwrap();
        ensure_repo_gitignore_ignores_local_configs(td.path(), ".ito").unwrap();
        let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
        assert!(s.contains(".ito/config.local.json"));
        assert!(s.contains(".local/ito/config.json"));
    }

    #[test]
    fn gitignore_exact_line_matching_trims_whitespace() {
        assert!(gitignore_has_exact_line("  foo  \nbar\n", "foo"));
        assert!(!gitignore_has_exact_line("foo\n", "bar"));
    }

    #[test]
    fn should_install_project_rel_filters_by_tool_id() {
        let mut tools = BTreeSet::new();
        tools.insert(TOOL_OPENCODE.to_string());

        assert!(should_install_project_rel("AGENTS.md", &tools));
        assert!(should_install_project_rel(".ito/config.json", &tools));
        assert!(should_install_project_rel(".opencode/config.json", &tools));
        assert!(!should_install_project_rel(".claude/settings.json", &tools));
        assert!(!should_install_project_rel(".codex/config.json", &tools));
        assert!(!should_install_project_rel(
            ".github/workflows/x.yml",
            &tools
        ));
    }

    #[test]
    fn release_tag_is_prefixed_with_v() {
        let tag = release_tag();
        assert!(tag.starts_with('v'));
    }

    #[test]
    fn update_model_in_yaml_replaces_or_inserts() {
        let yaml = "name: test\nmodel: \"old\"\n";
        let updated = update_model_in_yaml(yaml, "new");
        assert!(updated.contains("model: \"new\""));

        let yaml = "name: test\n";
        let updated = update_model_in_yaml(yaml, "new");
        assert!(updated.contains("model: \"new\""));
    }

    #[test]
    fn update_agent_model_field_updates_frontmatter_when_present() {
        let td = tempfile::tempdir().unwrap();
        let path = td.path().join("agent.md");
        std::fs::write(&path, "---\nname: test\nmodel: \"old\"\n---\nbody\n").unwrap();
        update_agent_model_field(&path, "new").unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert!(s.contains("model: \"new\""));

        let path = td.path().join("no-frontmatter.md");
        std::fs::write(&path, "no frontmatter\n").unwrap();
        update_agent_model_field(&path, "newer").unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert_eq!(s, "no frontmatter\n");
    }

    #[test]
    fn write_one_non_marker_files_skip_on_init_update_mode() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join("plain.txt");
        std::fs::write(&target, "existing").unwrap();

        let opts = InitOptions::new(BTreeSet::new(), false, true);
        write_one(
            &target,
            b"new",
            InstallMode::Init,
            &opts,
            FileOwnership::UserOwned,
        )
        .unwrap();
        let s = std::fs::read_to_string(&target).unwrap();
        assert_eq!(s, "existing");
    }

    #[test]
    fn write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join("plain.txt");
        std::fs::write(&target, "existing").unwrap();

        let opts = InitOptions::new(BTreeSet::new(), false, true);
        write_one(
            &target,
            b"new",
            InstallMode::Init,
            &opts,
            FileOwnership::ItoManaged,
        )
        .unwrap();
        let s = std::fs::read_to_string(&target).unwrap();
        assert_eq!(s, "new");
    }

    #[test]
    fn write_one_non_marker_user_owned_files_preserve_on_update_mode() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join("plain.txt");
        std::fs::write(&target, "existing").unwrap();

        let opts = InitOptions::new(BTreeSet::new(), false, true);
        write_one(
            &target,
            b"new",
            InstallMode::Update,
            &opts,
            FileOwnership::UserOwned,
        )
        .unwrap();
        let s = std::fs::read_to_string(&target).unwrap();
        assert_eq!(s, "existing");
    }

    #[test]
    fn write_one_marker_managed_files_refuse_overwrite_without_markers() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join("managed.md");
        std::fs::write(&target, "existing without markers\n").unwrap();

        let template = format!(
            "before\n{}\nmanaged\n{}\nafter\n",
            ito_templates::ITO_START_MARKER,
            ito_templates::ITO_END_MARKER
        );
        let opts = InitOptions::new(BTreeSet::new(), false, false);
        let err = write_one(
            &target,
            template.as_bytes(),
            InstallMode::Init,
            &opts,
            FileOwnership::ItoManaged,
        )
        .unwrap_err();
        assert!(err.to_string().contains("Refusing to overwrite"));
    }

    #[test]
    fn write_one_marker_managed_files_update_existing_markers() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join("managed.md");
        let existing = format!(
            "before\n{}\nold\n{}\nafter\n",
            ito_templates::ITO_START_MARKER,
            ito_templates::ITO_END_MARKER
        );
        std::fs::write(&target, existing).unwrap();

        let template = format!(
            "before\n{}\nnew\n{}\nafter\n",
            ito_templates::ITO_START_MARKER,
            ito_templates::ITO_END_MARKER
        );
        let opts = InitOptions::new(BTreeSet::new(), false, false);
        write_one(
            &target,
            template.as_bytes(),
            InstallMode::Init,
            &opts,
            FileOwnership::ItoManaged,
        )
        .unwrap();
        let s = std::fs::read_to_string(&target).unwrap();
        assert!(s.contains("new"));
        assert!(!s.contains("old"));
    }

    #[test]
    fn write_one_marker_managed_files_error_when_markers_missing_in_update_mode() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join("managed.md");
        // One marker present, one missing -> update should error.
        std::fs::write(
            &target,
            format!(
                "{}\nexisting without end marker\n",
                ito_templates::ITO_START_MARKER
            ),
        )
        .unwrap();

        let template = format!(
            "before\n{}\nmanaged\n{}\nafter\n",
            ito_templates::ITO_START_MARKER,
            ito_templates::ITO_END_MARKER
        );
        let opts = InitOptions::new(BTreeSet::new(), false, true);
        let err = write_one(
            &target,
            template.as_bytes(),
            InstallMode::Init,
            &opts,
            FileOwnership::ItoManaged,
        )
        .unwrap_err();
        assert!(err.to_string().contains("Failed to update markers"));
    }

    #[test]
    fn merge_json_objects_keeps_existing_and_adds_template_keys() {
        let mut existing = serde_json::json!({
            "permissions": {
                "allow": ["Bash(ls)"]
            },
            "hooks": {
                "SessionStart": [
                    {
                        "matcher": "*"
                    }
                ]
            }
        });
        let template = serde_json::json!({
            "hooks": {
                "PreToolUse": [
                    {
                        "matcher": "Bash|Edit|Write",
                        "hooks": [
                            {
                                "type": "command",
                                "command": "bash .claude/hooks/ito-audit.sh"
                            }
                        ]
                    }
                ]
            }
        });

        merge_json_objects(&mut existing, &template);

        assert_eq!(
            existing
                .pointer("/permissions/allow/0")
                .and_then(Value::as_str),
            Some("Bash(ls)")
        );
        assert!(existing.pointer("/hooks/SessionStart/0/matcher").is_some());
        assert!(
            existing
                .pointer("/hooks/PreToolUse/0/hooks/0/command")
                .is_some()
        );
    }

    #[test]
    fn classify_project_file_ownership_handles_user_owned_paths() {
        let ito_dir = ".ito";

        assert_eq!(
            classify_project_file_ownership(".ito/project.md", ito_dir),
            FileOwnership::UserOwned
        );
        assert_eq!(
            classify_project_file_ownership(".ito/config.json", ito_dir),
            FileOwnership::UserOwned
        );
        assert_eq!(
            classify_project_file_ownership(".ito/user-guidance.md", ito_dir),
            FileOwnership::UserOwned
        );
        assert_eq!(
            classify_project_file_ownership(".ito/user-prompts/tasks.md", ito_dir),
            FileOwnership::UserOwned
        );
        assert_eq!(
            classify_project_file_ownership(".ito/commands/review-edge.md", ito_dir),
            FileOwnership::ItoManaged
        );
    }

    #[test]
    fn write_claude_settings_merges_existing_file_on_update() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join(".claude/settings.json");
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(
            &target,
            "{\n  \"permissions\": {\n    \"allow\": [\"Bash(ls)\"]\n  }\n}\n",
        )
        .unwrap();

        let template = br#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash|Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "bash .claude/hooks/ito-audit.sh"
          }
        ]
      }
    ]
  }
}
"#;

        let opts = InitOptions::new(BTreeSet::new(), false, true);
        write_claude_settings(&target, template, InstallMode::Update, &opts).unwrap();

        let updated = std::fs::read_to_string(&target).unwrap();
        let value: Value = serde_json::from_str(&updated).unwrap();
        assert!(value.pointer("/permissions/allow").is_some());
        assert!(
            value
                .pointer("/hooks/PreToolUse/0/hooks/0/command")
                .is_some()
        );
    }

    #[test]
    fn merge_json_objects_appends_and_deduplicates_array_entries() {
        let mut existing = serde_json::json!({
            "permissions": {
                "allow": ["Bash(ls)"]
            },
            "hooks": {
                "PreToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [{"type": "command", "command": "echo existing"}]
                    }
                ]
            }
        });
        let template = serde_json::json!({
            "permissions": {
                "allow": ["Bash(ls)", "Bash(git status)"]
            },
            "hooks": {
                "PreToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [{"type": "command", "command": "echo existing"}]
                    },
                    {
                        "matcher": "Edit|Write",
                        "hooks": [{"type": "command", "command": "echo template"}]
                    }
                ]
            }
        });

        merge_json_objects(&mut existing, &template);

        let permissions = existing
            .pointer("/permissions/allow")
            .and_then(Value::as_array)
            .expect("permissions allow should remain an array");
        assert_eq!(permissions.len(), 2);
        assert_eq!(permissions[0].as_str(), Some("Bash(ls)"));
        assert_eq!(permissions[1].as_str(), Some("Bash(git status)"));

        let hooks = existing
            .pointer("/hooks/PreToolUse")
            .and_then(Value::as_array)
            .expect("PreToolUse should remain an array");
        assert_eq!(hooks.len(), 2);
        assert_eq!(
            hooks[0].pointer("/hooks/0/command").and_then(Value::as_str),
            Some("echo existing")
        );
        assert_eq!(
            hooks[1].pointer("/hooks/0/command").and_then(Value::as_str),
            Some("echo template")
        );
    }

    #[test]
    fn write_claude_settings_preserves_invalid_json_on_update() {
        let td = tempfile::tempdir().unwrap();
        let target = td.path().join(".claude/settings.json");
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(&target, "not-json\n").unwrap();

        let template = br#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash|Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "bash .claude/hooks/ito-audit.sh"
          }
        ]
      }
    ]
  }
}
"#;

        let opts = InitOptions::new(BTreeSet::new(), false, true);
        write_claude_settings(&target, template, InstallMode::Update, &opts).unwrap();

        let updated = std::fs::read_to_string(&target).unwrap();
        assert_eq!(updated, "not-json\n");
    }
}
