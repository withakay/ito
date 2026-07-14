use std::path::{Path, PathBuf};

use ito_config::types::{WorktreeInitConfig, WorktreeStrategy};

/// Worktree configuration serialized for instruction templates.
#[derive(Debug, Clone, serde::Serialize)]
pub(super) struct WorktreeConfig {
    pub(super) enabled: bool,
    pub(super) strategy: WorktreeStrategy,
    pub(super) layout_base_dir: Option<String>,
    pub(super) layout_dir_name: String,
    pub(super) apply_enabled: bool,
    pub(super) integration_mode: String,
    pub(super) copy_from_main: Vec<String>,
    pub(super) setup_commands: Vec<String>,
    pub(super) default_branch: String,
    /// Glob patterns from `worktrees.init.include` for worktree initialization.
    pub(super) init_include: Vec<String>,
    /// Setup commands from `worktrees.init.setup` for worktree initialization.
    pub(super) init_setup: Vec<String>,
    /// Absolute path to the current working worktree root.
    pub(super) worktree_root: Option<String>,
    /// Absolute path to the `.ito/` directory for this invocation.
    pub(super) ito_root: Option<String>,
    /// Absolute path to the project/repo root directory.
    pub(super) project_root: Option<String>,
}

/// Resolve the bare repo root for `bare_control_siblings` layouts.
pub(super) fn resolve_bare_repo_root(project_root: &Path) -> Option<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(project_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let common_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if common_dir.is_empty() {
        return None;
    }
    Path::new(&common_dir).parent().map(Path::to_path_buf)
}

/// Build template-facing worktree configuration from merged Ito config.
pub(super) fn worktree_config_from_merged(
    merged: &serde_json::Value,
    project_root: Option<&Path>,
) -> WorktreeConfig {
    let mut out = WorktreeConfig {
        enabled: false,
        strategy: WorktreeStrategy::CheckoutSubdir,
        layout_base_dir: None,
        layout_dir_name: "ito-worktrees".to_string(),
        apply_enabled: true,
        integration_mode: "commit_pr".to_string(),
        copy_from_main: vec![
            ".env".to_string(),
            ".envrc".to_string(),
            ".mise.local.toml".to_string(),
        ],
        setup_commands: Vec::new(),
        default_branch: "main".to_string(),
        init_include: Vec::new(),
        init_setup: Vec::new(),
        worktree_root: None,
        ito_root: None,
        project_root: None,
    };

    if let Some(wt) = merged.get("worktrees") {
        if let Some(v) = wt.get("enabled").and_then(|v| v.as_bool()) {
            out.enabled = v;
        }
        if let Some(v) = wt.get("strategy").and_then(|v| v.as_str())
            && let Some(parsed) = WorktreeStrategy::parse_value(v)
        {
            out.strategy = parsed;
        }
        if let Some(v) = wt.get("default_branch").and_then(|v| v.as_str())
            && !v.is_empty()
        {
            out.default_branch = v.to_string();
        }

        if let Some(layout) = wt.get("layout") {
            if let Some(v) = layout.get("base_dir").and_then(|v| v.as_str())
                && !v.is_empty()
            {
                out.layout_base_dir = Some(v.to_string());
            }
            if let Some(v) = layout.get("dir_name").and_then(|v| v.as_str())
                && !v.is_empty()
            {
                out.layout_dir_name = v.to_string();
            }
        }

        if let Some(apply) = wt.get("apply") {
            if let Some(v) = apply.get("enabled").and_then(|v| v.as_bool()) {
                out.apply_enabled = v;
            }
            if let Some(v) = apply.get("integration_mode").and_then(|v| v.as_str())
                && !v.is_empty()
            {
                out.integration_mode = v.to_string();
            }
            if let Some(arr) = apply.get("copy_from_main").and_then(|v| v.as_array()) {
                out.copy_from_main = arr
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect();
            }
            if let Some(arr) = apply.get("setup_commands").and_then(|v| v.as_array()) {
                out.setup_commands = arr
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect();
            }
        }

        if let Some(init) = wt.get("init")
            && let Ok(init_cfg) = serde_json::from_value::<WorktreeInitConfig>(init.clone())
        {
            out.init_include = init_cfg.include;
            if let Some(setup) = init_cfg.setup {
                out.init_setup = setup.as_commands().iter().map(|s| s.to_string()).collect();
            }
        }
    }

    if let Some(root) = project_root {
        out.project_root = match out.strategy {
            WorktreeStrategy::BareControlSiblings => {
                resolve_bare_repo_root(root).map(|p| p.to_string_lossy().to_string())
            }
            WorktreeStrategy::CheckoutSubdir | WorktreeStrategy::CheckoutSiblings => {
                Some(root.to_string_lossy().to_string())
            }
        };
    }

    out
}

pub(super) fn worktree_config_from_merged_with_paths(
    merged: &serde_json::Value,
    project_root: &Path,
    ito_path: &Path,
) -> WorktreeConfig {
    let mut out = worktree_config_from_merged(merged, Some(project_root));
    out.worktree_root = Some(project_root.to_string_lossy().to_string());
    out.ito_root = Some(ito_path.to_string_lossy().to_string());
    out
}

/// Alias used when the caller already holds a resolved config.
pub(super) fn worktree_config_from_resolved(
    merged: &serde_json::Value,
    project_root: &Path,
    ito_path: &Path,
) -> WorktreeConfig {
    worktree_config_from_merged_with_paths(merged, project_root, ito_path)
}
