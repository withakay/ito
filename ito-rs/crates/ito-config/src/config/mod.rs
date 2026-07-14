//! Configuration loading.
//!
//! Ito supports configuration at multiple layers:
//!
//! - Repo-local: `ito.json` and `.ito.json`
//! - Project/Ito dir: `<itoDir>/config.json` (and optionally `$PROJECT_DIR/config.json`)
//! - Global: `~/.config/ito/config.json` (or `$XDG_CONFIG_HOME/ito/config.json`)
//!
//! This module loads these sources, merges them with defaults, and records the
//! paths that contributed to the final configuration.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use ito_common::fs::{FileSystem, StdFs};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Default config values and JSON serialization helpers.
pub mod defaults;

/// JSON schema generation for Ito configuration.
pub mod schema;

/// Setup/update coverage classification for config fields.
pub mod setup_coverage;

/// Backend server configuration types (multi-tenant API).
pub mod backend_types;

/// Proposal integration configuration types.
pub mod proposal_types;

/// Serde models for `config.json`.
pub mod types;

/// Worktree initialization configuration types (split out to keep `types.rs` under the line limit).
pub mod worktree_init_types;

const REPO_CONFIG_FILE_NAME: &str = "ito.json";
const REPO_DOT_CONFIG_FILE_NAME: &str = ".ito.json";
const ITO_DIR_CONFIG_FILE_NAME: &str = "config.json";
const ITO_DIR_LOCAL_CONFIG_FILE_NAME: &str = "config.local.json";
const PROJECT_LOCAL_CONFIG_PATH: &str = ".local/ito/config.json";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
/// Global (user-level) configuration.
pub struct GlobalConfig {
    /// Preferred Ito working directory name.
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
/// Repo-local configuration.
pub struct ProjectConfig {
    /// Repo-local Ito working directory name override.
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
}

#[derive(Debug, Clone, Default)]
/// Process environment inputs for configuration resolution.
pub struct ConfigContext {
    /// Optional XDG config home path override.
    pub xdg_config_home: Option<PathBuf>,
    /// Home directory, used for non-XDG config lookup.
    pub home_dir: Option<PathBuf>,
    /// Optional project directory override (used by some harnesses).
    pub project_dir: Option<PathBuf>,
}

impl ConfigContext {
    /// Build a context from environment variables.
    pub fn from_process_env() -> Self {
        let xdg_config_home = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from);

        // Use HOME consistently across platforms for tests.
        let home_dir = std::env::var_os("HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from));

        let project_dir = std::env::var_os("PROJECT_DIR").map(PathBuf::from);
        let project_dir = project_dir.map(|p| {
            if p.is_absolute() {
                return p;
            }
            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            cwd.join(p)
        });

        Self {
            xdg_config_home,
            home_dir,
            project_dir,
        }
    }
}

fn read_to_string_optional_fs<F: FileSystem>(fs: &F, path: &Path) -> Option<String> {
    match fs.read_to_string(path) {
        Ok(s) => Some(s),
        Err(e) if e.kind() == ErrorKind::NotFound => None,
        Err(_) => None,
    }
}

/// Load `ito.json` from `project_root`.
pub fn load_project_config(project_root: &Path) -> Option<ProjectConfig> {
    load_project_config_fs(&StdFs, project_root)
}

/// Like [`load_project_config`], but uses an injected file-system.
pub fn load_project_config_fs<F: FileSystem>(fs: &F, project_root: &Path) -> Option<ProjectConfig> {
    let path = project_root.join(REPO_CONFIG_FILE_NAME);
    let contents = read_to_string_optional_fs(fs, &path)?;

    match serde_json::from_str(&contents) {
        Ok(v) => Some(v),
        Err(_) => {
            eprintln!(
                "Warning: Invalid JSON in {}, ignoring project config",
                path.display()
            );
            None
        }
    }
}

fn load_json_object_fs<F: FileSystem>(fs: &F, path: &Path) -> Option<Value> {
    let contents = read_to_string_optional_fs(fs, path)?;

    let v: Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Warning: Invalid JSON in {}, ignoring", path.display());
            return None;
        }
    };

    match v {
        Value::Object(mut obj) => {
            // Ignore JSON schema references in config.
            obj.remove("$schema");
            Some(Value::Object(obj))
        }
        _ => {
            eprintln!(
                "Warning: Expected JSON object in {}, ignoring",
                path.display()
            );
            None
        }
    }
}

fn merge_json(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (k, v) in overlay_map {
                let entry = base_map.get_mut(&k);
                if let Some(base_v) = entry {
                    merge_json(base_v, v);
                    continue;
                }
                base_map.insert(k, v);
            }
        }
        (base_v, overlay_v) => {
            *base_v = overlay_v;
        }
    }
}

/// Migrate legacy camelCase worktree keys to their new snake_case equivalents.
///
/// Legacy key mappings:
/// - `worktrees.defaultBranch` → `worktrees.default_branch`
/// - `worktrees.localFiles` → `worktrees.apply.copy_from_main`
///
/// New keys take precedence if both old and new are present.
/// Emits deprecation warnings to stderr when legacy keys are found.
fn migrate_legacy_worktree_keys(config: &mut Value) {
    let Value::Object(root) = config else {
        return;
    };

    let Some(Value::Object(wt)) = root.get_mut("worktrees") else {
        return;
    };

    // worktrees.defaultBranch → worktrees.default_branch
    if let Some(legacy_val) = wt.remove("defaultBranch") {
        eprintln!(
            "Warning: Config key 'worktrees.defaultBranch' is deprecated. \
             Use 'worktrees.default_branch' instead."
        );
        if !wt.contains_key("default_branch") {
            wt.insert("default_branch".to_string(), legacy_val);
        }
    }

    // worktrees.localFiles → worktrees.apply.copy_from_main
    if let Some(legacy_val) = wt.remove("localFiles") {
        eprintln!(
            "Warning: Config key 'worktrees.localFiles' is deprecated. \
             Use 'worktrees.apply.copy_from_main' instead."
        );
        let apply = wt
            .entry("apply")
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        if let Value::Object(apply_map) = apply
            && !apply_map.contains_key("copy_from_main")
        {
            apply_map.insert("copy_from_main".to_string(), legacy_val);
        }
    }
}

/// Remove the retired tmux preference from a resolved layer without rewriting
/// the user's source file.
fn ignore_removed_tmux_key(config: &mut Value) {
    let Value::Object(root) = config else {
        return;
    };
    let Some(Value::Object(tools)) = root.get_mut("tools") else {
        return;
    };
    let Some(Value::Object(tmux)) = tools.get_mut("tmux") else {
        return;
    };
    if tmux.remove("enabled").is_none() {
        return;
    }

    eprintln!(
        "Warning: Config key 'tools.tmux.enabled' was removed and has no effect. \
         Remove it from Ito configuration; external tmux use requires no Ito setting."
    );
    if tmux.is_empty() {
        tools.remove("tmux");
    }
    if tools.is_empty() {
        root.remove("tools");
    }
}

fn project_path_from_json(v: &Value) -> Option<String> {
    let Value::Object(map) = v else {
        return None;
    };
    let Some(Value::String(s)) = map.get("projectPath") else {
        return None;
    };
    if s.trim().is_empty() {
        return None;
    }
    Some(s.clone())
}

/// Returns a repo-local `projectPath` override (Ito working directory name).
///
/// Precedence (low -> high): `ito.json`, then `.ito.json`.
///
/// NOTE: This does *not* consult `<itoDir>/config.json` to avoid cycles.
pub fn load_repo_project_path_override(project_root: &Path) -> Option<String> {
    load_repo_project_path_override_fs(&StdFs, project_root)
}

/// Like [`load_repo_project_path_override`], but uses an injected file-system.
pub fn load_repo_project_path_override_fs<F: FileSystem>(
    fs: &F,
    project_root: &Path,
) -> Option<String> {
    let mut out = None;

    let repo = project_root.join(REPO_CONFIG_FILE_NAME);
    if let Some(v) = load_json_object_fs(fs, &repo)
        && let Some(p) = project_path_from_json(&v)
    {
        out = Some(p);
    }

    let repo = project_root.join(REPO_DOT_CONFIG_FILE_NAME);
    if let Some(v) = load_json_object_fs(fs, &repo)
        && let Some(p) = project_path_from_json(&v)
    {
        out = Some(p);
    }

    out
}

#[derive(Debug, Clone)]
/// Merged project configuration along with provenance.
pub struct CascadingProjectConfig {
    /// Fully merged config JSON.
    pub merged: Value,
    /// Paths that were successfully loaded and merged.
    pub loaded_from: Vec<PathBuf>,
    /// Per-layer breakdown, in precedence order (low → high).
    ///
    /// Each entry carries the source path and the parsed JSON value of
    /// that single layer (before merging). Validation rules that need to
    /// distinguish "value came from a committed file" from "value came
    /// from a gitignored override" inspect this list rather than the
    /// merged view.
    pub layers: Vec<ResolvedConfigLayer>,
}

#[derive(Debug, Clone)]
/// A single configuration layer that contributed to the merged view.
pub struct ResolvedConfigLayer {
    /// Absolute path of the layer's source file.
    pub path: PathBuf,
    /// Parsed JSON of this layer, before merging.
    pub value: Value,
}

/// Alias used by consumers who only care about the resolved config output.
pub type ResolvedConfig = CascadingProjectConfig;

/// Return the ordered list of configuration file paths consulted for a project.
pub fn project_config_paths(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();

    // Global config is the lowest-precedence layer.
    if let Some(path) = global_config_path(ctx) {
        out.push(path);
    }

    out.extend([
        project_root.join(REPO_CONFIG_FILE_NAME),
        project_root.join(REPO_DOT_CONFIG_FILE_NAME),
        ito_path.join(ITO_DIR_CONFIG_FILE_NAME),
        ito_path.join(ITO_DIR_LOCAL_CONFIG_FILE_NAME),
        project_root.join(PROJECT_LOCAL_CONFIG_PATH),
    ]);
    if let Some(p) = &ctx.project_dir {
        out.push(p.join(ITO_DIR_CONFIG_FILE_NAME));
    }

    out
}

/// Load and merge project configuration sources in precedence order.
///
/// Precedence (low -> high):
/// 1) `~/.config/ito/config.json` (global defaults; XDG-aware)
/// 2) `<repo-root>/ito.json`
/// 3) `<repo-root>/.ito.json`
/// 4) `<itoDir>/config.json` (team/project defaults, typically committed)
/// 5) `<itoDir>/config.local.json` (per-developer overrides, gitignored)
/// 6) `<repo-root>/.local/ito/config.json` (optional per-developer overrides, gitignored)
/// 7) `$PROJECT_DIR/config.json` (when set)
pub fn load_cascading_project_config(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> CascadingProjectConfig {
    load_cascading_project_config_fs(&StdFs, project_root, ito_path, ctx)
}

/// Resolve coordination branch settings from merged config JSON.
///
/// Falls back to documented defaults when the merged value cannot be
/// deserialized into [`types::ItoConfig`].
pub fn resolve_coordination_branch_settings(merged: &Value) -> (bool, String) {
    let Ok(cfg) = serde_json::from_value::<types::ItoConfig>(merged.clone()) else {
        let defaults = types::CoordinationBranchConfig::default();
        return (defaults.enabled.0, defaults.name);
    };

    (
        cfg.changes.coordination_branch.enabled.0,
        cfg.changes.coordination_branch.name,
    )
}

/// Resolve audit mirror settings from merged config JSON.
///
/// Falls back to documented defaults when the merged value cannot be
/// deserialized into [`types::ItoConfig`].
pub fn resolve_audit_mirror_settings(merged: &Value) -> (bool, String) {
    let Ok(cfg) = serde_json::from_value::<types::ItoConfig>(merged.clone()) else {
        let defaults = types::AuditMirrorConfig::default();
        return (defaults.enabled, defaults.branch);
    };

    (cfg.audit.mirror.enabled, cfg.audit.mirror.branch)
}

/// Like [`load_cascading_project_config`], but uses an injected file-system.
pub fn load_cascading_project_config_fs<F: FileSystem>(
    fs: &F,
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> CascadingProjectConfig {
    let mut merged = defaults::default_config_json();
    let mut loaded_from: Vec<PathBuf> = Vec::new();
    let mut layers: Vec<ResolvedConfigLayer> = Vec::new();

    let paths = project_config_paths(project_root, ito_path, ctx);
    for path in paths {
        let Some(mut v) = load_json_object_fs(fs, &path) else {
            continue;
        };
        // Migrate legacy camelCase worktree keys before merging so that
        // the new key names participate in the normal merge process and
        // override defaults correctly.
        migrate_legacy_worktree_keys(&mut v);
        ignore_removed_tmux_key(&mut v);
        layers.push(ResolvedConfigLayer {
            path: path.clone(),
            value: v.clone(),
        });
        merge_json(&mut merged, v);
        loaded_from.push(path);
    }

    CascadingProjectConfig {
        merged,
        loaded_from,
        layers,
    }
}

/// Return the global configuration file path, if it can be determined.
pub fn global_config_path(ctx: &ConfigContext) -> Option<PathBuf> {
    ito_config_dir(ctx).map(|d| d.join("config.json"))
}

/// Return the global configuration directory (`~/.config/ito` or XDG equivalent).
pub fn ito_config_dir(ctx: &ConfigContext) -> Option<PathBuf> {
    #[cfg(windows)]
    {
        // TS uses APPDATA on Windows. We accept HOME/USERPROFILE for tests but prefer APPDATA.
        let appdata = std::env::var_os("APPDATA").map(PathBuf::from);
        let base = appdata
            .or_else(|| ctx.xdg_config_home.clone())
            .or_else(|| ctx.home_dir.clone());
        return base.map(|b| b.join("ito"));
    }

    #[cfg(not(windows))]
    {
        let base = if let Some(xdg) = &ctx.xdg_config_home {
            xdg.clone()
        } else if let Some(home) = &ctx.home_dir {
            home.join(".config")
        } else {
            return None;
        };

        Some(base.join("ito"))
    }
}

/// Load the global config file.
pub fn load_global_config(ctx: &ConfigContext) -> GlobalConfig {
    load_global_config_fs(&StdFs, ctx)
}

/// Like [`load_global_config`], but uses an injected file-system.
pub fn load_global_config_fs<F: FileSystem>(fs: &F, ctx: &ConfigContext) -> GlobalConfig {
    let Some(path) = global_config_path(ctx) else {
        return GlobalConfig::default();
    };

    let Some(contents) = read_to_string_optional_fs(fs, &path) else {
        return GlobalConfig::default();
    };

    match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "Warning: Invalid JSON in {}, using defaults",
                path.display()
            );
            GlobalConfig::default()
        }
    }
}

/// Load the full [`types::ItoConfig`] from the global config file.
///
/// This reads `~/.config/ito/config.json` (or its XDG equivalent) and
/// deserializes it into the full configuration struct. Fields not present
/// in the file receive their defaults.
pub fn load_global_ito_config(ctx: &ConfigContext) -> types::ItoConfig {
    load_global_ito_config_fs(&StdFs, ctx)
}

/// Like [`load_global_ito_config`], but uses an injected file-system.
pub fn load_global_ito_config_fs<F: FileSystem>(fs: &F, ctx: &ConfigContext) -> types::ItoConfig {
    let Some(path) = global_config_path(ctx) else {
        return types::ItoConfig::default();
    };

    let Some(contents) = read_to_string_optional_fs(fs, &path) else {
        return types::ItoConfig::default();
    };

    match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "Warning: Invalid JSON in {}, using defaults",
                path.display()
            );
            types::ItoConfig::default()
        }
    }
}

#[cfg(test)]
mod config_tests;
