use std::collections::BTreeSet;
use std::path::Path;

use chrono::Utc;
use serde_json::{Map, Value};

use crate::errors::{CoreError, CoreResult};
use agent_frontmatter::{
    remove_agent_mode_field_for_direct_activation, update_agent_activation_field_from_rendered,
    update_agent_model_field,
};
use agents_cleanup::remove_obsolete_specialist_agents;

use markers::update_file_with_markers;

mod agent_frontmatter;
mod agents_cleanup;
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
/// Tool id for Pi.
pub const TOOL_PI: &str = "pi";

const CONFIG_SCHEMA_RELEASE_TAG_PLACEHOLDER: &str = "__ITO_RELEASE_TAG__";

/// Return the set of supported tool ids.
pub fn available_tool_ids() -> &'static [&'static str] {
    &[
        TOOL_CLAUDE,
        TOOL_CODEX,
        TOOL_GITHUB_COPILOT,
        TOOL_OPENCODE,
        TOOL_PI,
    ]
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
    /// When `true`, perform a marker-scoped upgrade of prompt/template assets.
    ///
    /// Only content between `<!-- ITO:START -->` and `<!-- ITO:END -->` markers
    /// is replaced from the embedded templates. All content outside the managed
    /// block is preserved exactly. When a marker-managed file is found to be
    /// missing valid Ito markers the file is left unchanged and actionable
    /// guidance is emitted rather than returning an error.
    ///
    /// `upgrade` implies `update` semantics (user-owned files are preserved).
    pub upgrade: bool,
}

impl InitOptions {
    /// Constructs an `InitOptions` configured for a standard (non-upgrade) installation.
    ///
    /// The returned value has `upgrade` set to `false`. The `force` flag controls whether
    /// existing files may be overwritten, and `update` enables update semantics that merge
    /// managed marker blocks instead of unconditional replacement.
    ///
    pub fn new(tools: BTreeSet<String>, force: bool, update: bool) -> Self {
        Self {
            tools,
            force,
            update,
            upgrade: false,
        }
    }

    /// Constructs `InitOptions` configured for upgrade mode.
    ///
    /// In upgrade mode the options enable update semantics and preserve user-owned
    /// files. The `force` flag is disabled and `update` and `upgrade` are enabled,
    /// so marker-managed files missing Ito markers are left unchanged with guidance
    /// rather than causing an error.
    ///
    pub fn new_upgrade(tools: BTreeSet<String>) -> Self {
        Self {
            tools,
            force: false,
            update: true,
            upgrade: true,
        }
    }

    /// Enable upgrade mode and its update semantics on this `InitOptions`.
    ///
    /// When upgrade is enabled it implies `update = true` and `force = false`; this
    /// method sets all three fields so that `force` cannot override the non-destructive
    /// marker-scoped upgrade behavior.
    ///
    pub fn with_upgrade(mut self) -> Self {
        self.upgrade = true;
        self.update = true;
        self.force = false;
        self
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

    // The removed tmux skill occupied an Ito-owned skill directory in every
    // harness. Update-style installs prune only those exact legacy paths;
    // unrelated tmux configuration remains user-owned and untouched.
    if mode == InstallMode::Update || opts.update || opts.force {
        remove_obsolete_tmux_skills(project_root)?;
    }

    // Repository-local ignore rules for per-worktree state.
    // This is not a templated file: we update `.gitignore` directly to preserve existing content.
    if mode == InstallMode::Init {
        ensure_repo_gitignore_ignores_session_json(project_root, &ito_dir)?;
        ensure_repo_gitignore_ignores_audit_session(project_root, &ito_dir)?;
        remove_repo_gitignore_unignores_audit_events(project_root, &ito_dir)?;
    }

    // Local (per-developer) config overlays should never be committed.
    ensure_repo_gitignore_ignores_local_configs(project_root, &ito_dir)?;

    install_adapter_files(project_root, mode, opts, worktree_ctx)?;
    install_agent_templates(project_root, mode, opts)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A known legacy Ito-managed path found in a project.
pub struct LegacyPathHit {
    /// Path relative to the project root.
    pub relative_path: String,
    /// Human-readable reason this path is legacy.
    pub description: &'static str,
    /// Replacement path, when a current artifact supersedes the legacy path.
    pub replacement: Option<&'static str>,
}

/// Detect known legacy Ito-managed paths under the project root.
pub fn detect_legacy_paths(project_root: &Path) -> Vec<LegacyPathHit> {
    let skill_roots = [
        ".claude/skills",
        ".opencode/skills",
        ".codex/skills",
        ".github/skills",
        ".pi/skills",
    ];
    let mut hits = Vec::new();
    for entry in ito_templates::legacy::LEGACY_ENTRIES {
        if entry.old_path.starts_with('.') {
            push_legacy_hit_if_exists(project_root, entry.old_path, entry, &mut hits);
        } else {
            for root in skill_roots {
                let rel = format!("{root}/{}", entry.old_path);
                push_legacy_hit_if_exists(project_root, &rel, entry, &mut hits);
            }
        }
    }
    hits.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    hits
}

/// Remove previously detected legacy Ito-managed paths from the project root.
pub fn remove_legacy_paths(project_root: &Path, hits: &[LegacyPathHit]) -> CoreResult<()> {
    for hit in hits {
        let path = project_root.join(&hit.relative_path);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => {
                return Err(CoreError::io(format!("reading {}", path.display()), err));
            }
        };
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| CoreError::io(format!("removing {}", path.display()), e))?;
        } else {
            std::fs::remove_file(&path)
                .map_err(|e| CoreError::io(format!("removing {}", path.display()), e))?;
        }
    }
    Ok(())
}

fn push_legacy_hit_if_exists(
    project_root: &Path,
    rel: &str,
    entry: &ito_templates::legacy::LegacyEntry,
    hits: &mut Vec<LegacyPathHit>,
) {
    let normalized = rel.trim_end_matches('/');
    if std::fs::symlink_metadata(project_root.join(normalized)).is_ok() {
        hits.push(LegacyPathHit {
            relative_path: normalized.to_string(),
            description: entry.description,
            replacement: entry.new_path,
        });
    }
}

fn remove_obsolete_tmux_skills(project_root: &Path) -> CoreResult<()> {
    let hits: Vec<_> = detect_legacy_paths(project_root)
        .into_iter()
        .filter(|hit| hit.relative_path.ends_with("/ito-tmux"))
        .collect();
    remove_legacy_paths(project_root, &hits)
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

/// Remove the legacy audit events unignore so worktree audit logs stay untracked.
fn remove_repo_gitignore_unignores_audit_events(
    project_root: &Path,
    ito_dir: &str,
) -> CoreResult<()> {
    let entry = format!("!{ito_dir}/.state/audit/");
    remove_gitignore_exact_line(project_root, &entry)
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

fn remove_gitignore_exact_line(project_root: &Path, entry: &str) -> CoreResult<()> {
    let path = project_root.join(".gitignore");
    let existing = match ito_common::io::read_to_string_std(&path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(CoreError::io(format!("reading {}", path.display()), e)),
    };

    let mut filtered = Vec::new();
    let mut removed = false;
    for line in existing.lines() {
        if line.trim() == entry {
            removed = true;
            continue;
        }
        filtered.push(line);
    }
    if !removed {
        return Ok(());
    }

    let mut updated = filtered.join("\n");
    if !updated.is_empty() {
        updated.push('\n');
    }

    ito_common::io::write_std(&path, updated)
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
    let semver = option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
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

        // Stamp every managed-block markdown file with the current CLI version.
        if rel.ends_with(".md")
            && !rel.ends_with(".md.j2")
            && let Ok(text) = std::str::from_utf8(&bytes)
            && text.contains(ito_templates::ITO_START_MARKER)
        {
            bytes = ito_templates::stamp_version(text, semver).into_bytes();
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
    if rel.starts_with(".pi/") {
        return tools.contains(TOOL_PI);
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

    let wiki_prefix = format!("{ito_dir}/wiki/");
    if rel.starts_with(&wiki_prefix) {
        return FileOwnership::UserOwned;
    }

    FileOwnership::ItoManaged
}

/// Returns `true` when the rendered template's managed block spans the entire
/// file — i.e. nothing meaningful sits outside the `<!-- ITO:START -->` /
/// `<!-- ITO:END -->` markers. Such files have no user-editable region, so on
/// update they can be rewritten wholesale to drop stale content from previous
/// versions.
///
/// The rendered template bytes come from embedded assets that always place
/// markers on their own lines, so a raw `find` is sufficient here.
fn template_is_entirely_managed(text: &str) -> bool {
    let Some(start) = text.find(ito_templates::ITO_START_MARKER) else {
        return false;
    };
    let Some(end) = text.find(ito_templates::ITO_END_MARKER) else {
        return false;
    };
    let before = text[..start].trim();
    let after = text[end + ito_templates::ITO_END_MARKER.len()..].trim();
    before.is_empty() && after.is_empty()
}

/// Returns `true` when the rendered template has non-trivial content sitting
/// **before** its managed block (typically YAML frontmatter). Used by the
/// first-marker migration path: when an existing on-disk file has no markers
/// and the template has frontmatter + markers, the marker-prepend strategy
/// would re-order the existing frontmatter and corrupt the file. In that case
/// the safer migration is to write the rendered template wholesale.
fn template_has_prefix_outside_markers(text: &str) -> bool {
    let Some(start) = text.find(ito_templates::ITO_START_MARKER) else {
        return false;
    };
    !text[..start].trim().is_empty()
}

/// Render an agent template and stamp the managed block (if any) with the
/// current CLI version. Used by both `Init` and `Update` paths so newly
/// written agent files always carry an `ITO:VERSION` line consistent with the
/// installer.
fn render_and_stamp_agent(
    contents: &[u8],
    config: Option<&ito_templates::agents::AgentConfig>,
    target: &Path,
) -> Vec<u8> {
    let semver = option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let rendered = match (std::str::from_utf8(contents), config) {
        (Ok(template_str), Some(cfg)) => {
            ito_templates::agents::render_agent_template(template_str, cfg).into_bytes()
        }
        _ => contents.to_vec(),
    };
    let path_str = target.to_string_lossy();
    if !path_str.ends_with(".md") || path_str.ends_with(".md.j2") {
        return rendered;
    }
    let Ok(text) = std::str::from_utf8(&rendered) else {
        return rendered;
    };
    if !text.contains(ito_templates::ITO_START_MARKER) {
        return rendered;
    }
    ito_templates::stamp_version(text, semver).into_bytes()
}

/// Write a rendered managed-block markdown file with marker-scoped update
/// semantics, suitable for installer paths that do not need the full
/// `write_one` ownership/upgrade machinery (e.g. the harness manifest
/// installer in `distribution.rs`).
///
/// Behaviour:
///
/// - **No target on disk** → write `rendered_bytes` verbatim.
/// - **`mode == Init` && `opts.force`** → wholesale overwrite (matches the
///   `--force` semantics in `write_one`).
/// - **Template has no managed block** → wholesale overwrite (caller wanted
///   plain replacement).
/// - **Existing target has no markers** → wholesale overwrite. Treats the
///   file as legacy from before managed markers were retrofitted; no user
///   content is at risk because there was no marker boundary to honour.
/// - **Existing target has markers** → marker-scoped update via
///   `update_file_with_markers`, preserving everything outside the managed
///   block byte-for-byte.
///
/// Returns `Ok(())` on success. Errors mirror `write_one` (IO + marker
/// validation diagnostics).
pub(crate) fn write_marker_aware_markdown(
    target: &Path,
    rendered_bytes: &[u8],
    mode: InstallMode,
    opts: &InitOptions,
) -> CoreResult<()> {
    if let Some(parent) = target.parent() {
        ito_common::io::create_dir_all_std(parent)
            .map_err(|e| CoreError::io(format!("creating directory {}", parent.display()), e))?;
    }

    let wholesale = |target: &Path| -> CoreResult<()> {
        ito_common::io::write_std(target, rendered_bytes)
            .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))
    };

    if !target.exists() {
        return wholesale(target);
    }

    if mode == InstallMode::Init && opts.force {
        return wholesale(target);
    }

    let Ok(text) = std::str::from_utf8(rendered_bytes) else {
        return wholesale(target);
    };
    let Some(block) = ito_templates::extract_managed_block(text) else {
        return wholesale(target);
    };

    let existing = ito_common::io::read_to_string_std(target)
        .map_err(|e| CoreError::io(format!("reading {}", target.display()), e))?;
    let has_start = existing.contains(ito_templates::ITO_START_MARKER);
    let has_end = existing.contains(ito_templates::ITO_END_MARKER);
    match (has_start, has_end) {
        (false, false) => return wholesale(target),
        (true, true) => {}
        (true, false) | (false, true) => {
            // Partial marker pair indicates the user (or some other tool)
            // damaged the managed region. Refusing to write here mirrors
            // `update_file_with_markers`' error path and prevents silently
            // clobbering user content. The user must restore the markers
            // (or pass `--force`) before update can proceed.
            return Err(CoreError::Validation(format!(
                "Refusing to update {}: file has a partial Ito marker pair (start={has_start}, end={has_end}). \
Restore both markers manually, or rerun with `--force` to overwrite the file wholesale.",
                target.display()
            )));
        }
    }

    let _ = update_file_with_markers(
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
    Ok(())
}

/// Writes a rendered template to `target`, handling Ito-managed marker blocks, overwrite/update semantics,
/// and ownership rules.
///
/// When the rendered template contains Ito start/end markers, this function treats the file as
/// marker-managed: it will update only the managed block when the target exists (honoring `--force`,
/// `--update`, and `--upgrade` semantics), or write the template verbatim when the target does not
/// exist. For non-marker files, behavior depends on `mode`, `opts.force`, `opts.update`, and `ownership`:
/// - On Init: `--force` overwrites; `--update` skips user-owned files; otherwise the function refuses
///   to overwrite existing files without markers.
/// - On Update: skips user-owned files; otherwise writes/overwrites the target.
///
/// Errors are returned for IO failures and for invalid marker states when an update is attempted
/// (except when `opts.upgrade` is true, in which case a missing marker in an expected marker-managed
/// file produces a warning and the existing file is preserved).
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
            // --force always overwrites the file wholesale on init.
            if mode == InstallMode::Init && opts.force {
                ito_common::io::write_std(target, rendered_bytes)
                    .map_err(|e| CoreError::io(format!("writing {}", target.display()), e))?;
                return Ok(());
            }

            // User-owned files keep their content untouched on update / non-forced
            // init even when the template now ships managed markers. Updating the
            // managed block here would clobber user edits to files Ito only seeds
            // (e.g. .ito/project.md, .ito/user-prompts/*.md).
            if ownership == FileOwnership::UserOwned {
                let updating = mode == InstallMode::Update
                    || (mode == InstallMode::Init && (opts.update || opts.upgrade));
                if updating {
                    return Ok(());
                }
            }

            // Read the existing file once and check for both Ito markers.
            let existing = ito_common::io::read_to_string_std(target)
                .map_err(|e| CoreError::io(format!("reading {}", target.display()), e))?;
            let has_markers = existing.contains(ito_templates::ITO_START_MARKER)
                && existing.contains(ito_templates::ITO_END_MARKER);

            if !has_markers {
                if opts.upgrade {
                    // Upgrade fail-safe: when a file is expected to be marker-managed but no
                    // longer contains valid Ito markers, preserve the file unchanged and emit
                    // actionable guidance rather than returning an error.
                    eprintln!(
                        "warning: skipping upgrade of {} — Ito markers not found.\n\
                        To restore managed upgrade support, re-add the markers manually:\n\
                        \n\
                        {start}\n\
                        <ito-managed content>\n\
                        {end}\n\
                        \n\
                        Then re-run `ito init --upgrade`.",
                        target.display(),
                        start = ito_templates::ITO_START_MARKER,
                        end = ito_templates::ITO_END_MARKER,
                    );
                    return Ok(());
                }

                if mode == InstallMode::Init && !opts.update {
                    // Plain init: refuse to overwrite without --force or --update.
                    return Err(CoreError::Validation(format!(
                        "Refusing to overwrite existing file without markers: {} (re-run with --force)",
                        target.display()
                    )));
                }

                // update / `init --update` against an Ito-managed file that
                // predates marker rollout. Only safe to wholesale-rewrite when
                // the existing file is genuinely marker-free (a partial
                // marker pair indicates the user has manually edited the
                // managed region and we should error rather than overwrite).
                let existing_has_no_markers = !existing.contains(ito_templates::ITO_START_MARKER)
                    && !existing.contains(ito_templates::ITO_END_MARKER);
                if existing_has_no_markers {
                    // Decide between wholesale rewrite and marker-prepend
                    // based on the template's shape:
                    //
                    // - If the template's managed block spans the entire
                    //   file, there is no user-editable region; rewrite
                    //   wholesale to drop stale content from the previous
                    //   version.
                    //
                    // - If the template has a non-empty prefix (typically
                    //   YAML frontmatter) above the managed block,
                    //   marker-prepend would re-order the existing
                    //   frontmatter and corrupt the file. Rewrite wholesale
                    //   instead.
                    //
                    // - Otherwise (template has only markers, no surrounding
                    //   content), fall through to `update_file_with_markers`
                    //   which prepends the managed block to the existing
                    //   file while preserving user content.
                    if template_is_entirely_managed(text)
                        || template_has_prefix_outside_markers(text)
                    {
                        ito_common::io::write_std(target, rendered_bytes).map_err(|e| {
                            CoreError::io(format!("writing {}", target.display()), e)
                        })?;
                        return Ok(());
                    }
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
    mode: InstallMode,
    opts: &InitOptions,
    worktree_ctx: Option<&WorktreeTemplateContext>,
) -> CoreResult<()> {
    for tool in &opts.tools {
        match tool.as_str() {
            TOOL_OPENCODE => {
                let config_dir = project_root.join(".opencode");
                let manifests = crate::distribution::opencode_manifests(&config_dir);
                crate::distribution::install_manifests(&manifests, worktree_ctx, mode, opts)?;
            }
            TOOL_CLAUDE => {
                let manifests = crate::distribution::claude_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx, mode, opts)?;
            }
            TOOL_CODEX => {
                let manifests = crate::distribution::codex_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx, mode, opts)?;
            }
            TOOL_GITHUB_COPILOT => {
                let manifests = crate::distribution::github_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx, mode, opts)?;
            }
            TOOL_PI => {
                let manifests = crate::distribution::pi_manifests(project_root);
                crate::distribution::install_manifests(&manifests, worktree_ctx, mode, opts)?;
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
    use ito_templates::agents::{AgentTier, Harness, default_agent_configs, get_agent_files};

    let configs = default_agent_configs();

    // Map tool names to harnesses
    let tool_harness_map = [
        (TOOL_OPENCODE, Harness::OpenCode),
        (TOOL_CLAUDE, Harness::ClaudeCode),
        (TOOL_CODEX, Harness::Codex),
        (TOOL_GITHUB_COPILOT, Harness::GitHubCopilot),
        (TOOL_PI, Harness::Pi),
    ];

    for (tool_id, harness) in tool_harness_map {
        if !opts.tools.contains(tool_id) {
            continue;
        }

        let agent_dir = project_root.join(harness.project_agent_path());
        // Update-style installs and forceful re-inits should both clear the
        // legacy `ito-orchestrator-*` specialist assets before writing the new
        // `ito-*` names. Plain init keeps untouched user files in place.
        let should_remove_obsolete_specialists =
            mode == InstallMode::Update || opts.update || opts.force;
        if should_remove_obsolete_specialists {
            remove_obsolete_specialist_agents(&agent_dir)?;
        }

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

            // Get config for this tier.
            //
            // Some harness agent templates are not tiered but still include
            // `{{model}}` placeholders (e.g., coordinator agents). In that case,
            // render using the harness's General tier defaults.
            let mut config = tier.and_then(|t| configs.get(&(harness, t)));
            if config.is_none()
                && let Ok(s) = std::str::from_utf8(contents)
                && s.contains("{{model}}")
            {
                config = configs.get(&(harness, AgentTier::General));
            }

            match mode {
                InstallMode::Init => {
                    if target.exists() && !opts.force {
                        if opts.update {
                            let rendered = render_and_stamp_agent(contents, config, &target);
                            update_existing_agent_template(&target, &rendered, mode, opts, config)?;
                        }
                        continue;
                    }

                    let rendered = render_and_stamp_agent(contents, config, &target);
                    write_marker_aware_markdown(&target, &rendered, mode, opts)?;
                }
                InstallMode::Update => {
                    let rendered = render_and_stamp_agent(contents, config, &target);
                    if target.exists() {
                        update_existing_agent_template(&target, &rendered, mode, opts, config)?;
                    } else {
                        write_marker_aware_markdown(&target, &rendered, mode, opts)?;
                    }
                }
            }
        }
    }

    Ok(())
}
/// Updates an existing agent template without clobbering user-owned bodies.
///
/// Files with complete Ito markers get their managed block refreshed. Legacy
/// markerless files keep their body and only receive frontmatter model updates.
/// Partial marker pairs are treated like damaged managed regions: preserve the
/// body for compatibility, but warn so the user can repair the file.
fn update_existing_agent_template(
    target: &Path,
    rendered: &[u8],
    mode: InstallMode,
    opts: &InitOptions,
    config: Option<&ito_templates::agents::AgentConfig>,
) -> CoreResult<()> {
    let existing = ito_common::io::read_to_string_std(target)
        .map_err(|e| CoreError::io(format!("reading {}", target.display()), e))?;
    let has_start = existing.contains(ito_templates::ITO_START_MARKER);
    let has_end = existing.contains(ito_templates::ITO_END_MARKER);

    match (has_start, has_end) {
        (true, true) => write_marker_aware_markdown(target, rendered, mode, opts)?,
        (false, false) => {}
        (true, false) | (false, true) => {
            eprintln!(
                "warning: skipping marker update for {}: file has a partial Ito marker pair \
                (start={has_start}, end={has_end}). Restore both markers manually, or rerun \
                with `--force` to overwrite the file wholesale.",
                target.display()
            );
        }
    }

    if let Some(cfg) = config {
        update_agent_model_field(target, &cfg.model)?;
    }
    update_agent_activation_field_from_rendered(target, rendered)?;
    remove_agent_mode_field_for_direct_activation(target, rendered)?;

    Ok(())
}

#[cfg(test)]
mod json_tests;

#[cfg(test)]
mod installers_tests;
