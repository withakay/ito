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

/// Serde models for `config.json`.
pub mod types;

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
}

/// Alias used by consumers who only care about the resolved config output.
pub type ResolvedConfig = CascadingProjectConfig;

/// Return the ordered list of configuration file paths consulted for a project.
pub fn project_config_paths(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = vec![
        project_root.join(REPO_CONFIG_FILE_NAME),
        project_root.join(REPO_DOT_CONFIG_FILE_NAME),
        ito_path.join(ITO_DIR_CONFIG_FILE_NAME),
        ito_path.join(ITO_DIR_LOCAL_CONFIG_FILE_NAME),
        project_root.join(PROJECT_LOCAL_CONFIG_PATH),
    ];
    if let Some(p) = &ctx.project_dir {
        out.push(p.join(ITO_DIR_CONFIG_FILE_NAME));
    }

    out
}

/// Load and merge project configuration sources in precedence order.
///
/// Precedence (low -> high):
/// 1) `<repo-root>/ito.json`
/// 2) `<repo-root>/.ito.json`
/// 3) `<itoDir>/config.json` (team/project defaults, typically committed)
/// 4) `<itoDir>/config.local.json` (per-developer overrides, gitignored)
/// 5) `<repo-root>/.local/ito/config.json` (optional per-developer overrides, gitignored)
/// 6) `$PROJECT_DIR/config.json` (when set)
pub fn load_cascading_project_config(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> CascadingProjectConfig {
    load_cascading_project_config_fs(&StdFs, project_root, ito_path, ctx)
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

    let paths = project_config_paths(project_root, ito_path, ctx);
    for path in paths {
        let Some(mut v) = load_json_object_fs(fs, &path) else {
            continue;
        };
        // Migrate legacy camelCase worktree keys before merging so that
        // the new key names participate in the normal merge process and
        // override defaults correctly.
        migrate_legacy_worktree_keys(&mut v);
        merge_json(&mut merged, v);
        loaded_from.push(path);
    }

    CascadingProjectConfig {
        merged,
        loaded_from,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cascading_project_config_merges_sources_in_order_with_scalar_override() {
        let repo = tempfile::tempdir().unwrap();

        std::fs::write(
            repo.path().join("ito.json"),
            "{\"obj\":{\"a\":1},\"arr\":[1],\"x\":\"repo\"}",
        )
        .unwrap();
        std::fs::write(
            repo.path().join(".ito.json"),
            "{\"obj\":{\"b\":2},\"arr\":[2],\"y\":\"dot\"}",
        )
        .unwrap();

        let project_dir = tempfile::tempdir().unwrap();
        std::fs::write(
            project_dir.path().join("config.json"),
            "{\"obj\":{\"c\":3},\"x\":\"project_dir\"}",
        )
        .unwrap();

        let ctx = ConfigContext {
            xdg_config_home: None,
            home_dir: None,
            project_dir: Some(project_dir.path().to_path_buf()),
        };
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);
        std::fs::create_dir_all(&ito_path).unwrap();
        std::fs::write(
            ito_path.join("config.json"),
            "{\"obj\":{\"a\":9},\"z\":\"ito_dir\"}",
        )
        .unwrap();

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);

        assert_eq!(
            r.merged.get("obj").unwrap(),
            &serde_json::json!({"a": 9, "b": 2, "c": 3})
        );
        assert_eq!(r.merged.get("arr").unwrap(), &serde_json::json!([2]));
        assert_eq!(
            r.merged.get("x").unwrap(),
            &serde_json::json!("project_dir")
        );
        assert_eq!(r.merged.get("y").unwrap(), &serde_json::json!("dot"));
        assert_eq!(r.merged.get("z").unwrap(), &serde_json::json!("ito_dir"));

        // Defaults are present.
        assert!(r.merged.get("cache").is_some());
        assert!(r.merged.get("harnesses").is_some());

        assert_eq!(
            r.loaded_from,
            vec![
                repo.path().join("ito.json"),
                repo.path().join(".ito.json"),
                ito_path.join("config.json"),
                project_dir.path().join("config.json"),
            ]
        );
    }

    #[test]
    fn cascading_project_config_ignores_invalid_json_sources() {
        let repo = tempfile::tempdir().unwrap();

        std::fs::write(repo.path().join("ito.json"), "{\"a\":1}").unwrap();
        std::fs::write(repo.path().join(".ito.json"), "not-json").unwrap();

        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        assert_eq!(r.merged.get("a").unwrap(), &serde_json::json!(1));
        assert!(r.merged.get("cache").is_some());

        assert_eq!(r.loaded_from, vec![repo.path().join("ito.json")]);
    }

    #[test]
    fn cascading_project_config_ignores_schema_ref_key() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::write(
            repo.path().join("ito.json"),
            "{\"$schema\":\"./config.schema.json\",\"a\":1}",
        )
        .unwrap();

        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        assert_eq!(r.merged.get("a").unwrap(), &serde_json::json!(1));
        assert!(r.merged.get("$schema").is_none());
    }

    #[test]
    fn global_config_path_prefers_xdg() {
        let ctx = ConfigContext {
            xdg_config_home: Some(PathBuf::from("/tmp/xdg")),
            home_dir: Some(PathBuf::from("/tmp/home")),
            project_dir: None,
        };
        #[cfg(not(windows))]
        assert_eq!(
            global_config_path(&ctx).unwrap(),
            PathBuf::from("/tmp/xdg/ito/config.json")
        );
    }

    #[test]
    fn ito_config_dir_prefers_xdg() {
        let ctx = ConfigContext {
            xdg_config_home: Some(PathBuf::from("/tmp/xdg")),
            home_dir: Some(PathBuf::from("/tmp/home")),
            project_dir: None,
        };
        #[cfg(not(windows))]
        assert_eq!(ito_config_dir(&ctx).unwrap(), PathBuf::from("/tmp/xdg/ito"));
    }

    #[test]
    fn worktrees_config_has_defaults_in_cascading_config() {
        let repo = tempfile::tempdir().unwrap();
        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        let wt = r
            .merged
            .get("worktrees")
            .expect("worktrees key should exist");

        assert_eq!(wt.get("enabled").and_then(|v| v.as_bool()), Some(false));
        assert_eq!(
            wt.get("strategy").and_then(|v| v.as_str()),
            Some("checkout_subdir")
        );
        assert_eq!(
            wt.get("default_branch").and_then(|v| v.as_str()),
            Some("main")
        );

        let layout = wt.get("layout").unwrap();
        assert_eq!(
            layout.get("dir_name").and_then(|v| v.as_str()),
            Some("ito-worktrees")
        );

        let apply = wt.get("apply").unwrap();
        assert_eq!(apply.get("enabled").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            apply.get("integration_mode").and_then(|v| v.as_str()),
            Some("commit_pr")
        );

        let copy = apply
            .get("copy_from_main")
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(copy.len(), 3);
    }

    #[test]
    fn legacy_worktree_default_branch_key_migrates() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::write(
            repo.path().join("ito.json"),
            r#"{"worktrees":{"defaultBranch":"develop"}}"#,
        )
        .unwrap();

        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        let wt = r.merged.get("worktrees").unwrap();

        assert_eq!(
            wt.get("default_branch").and_then(|v| v.as_str()),
            Some("develop")
        );
        assert!(wt.get("defaultBranch").is_none());
    }

    #[test]
    fn legacy_worktree_local_files_key_migrates() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::write(
            repo.path().join("ito.json"),
            r#"{"worktrees":{"localFiles":[".env",".secrets"]}}"#,
        )
        .unwrap();

        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        let wt = r.merged.get("worktrees").unwrap();
        let apply = wt.get("apply").unwrap();
        let copy = apply
            .get("copy_from_main")
            .and_then(|v| v.as_array())
            .unwrap();

        assert_eq!(copy.len(), 2);
        assert_eq!(copy[0].as_str(), Some(".env"));
        assert_eq!(copy[1].as_str(), Some(".secrets"));
        assert!(wt.get("localFiles").is_none());
    }

    #[test]
    fn new_worktree_keys_take_precedence_over_legacy() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::write(
            repo.path().join("ito.json"),
            r#"{"worktrees":{"defaultBranch":"legacy","default_branch":"new-main","localFiles":[".old"],"apply":{"copy_from_main":[".new"]}}}"#,
        )
        .unwrap();

        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        let wt = r.merged.get("worktrees").unwrap();

        assert_eq!(
            wt.get("default_branch").and_then(|v| v.as_str()),
            Some("new-main")
        );

        let apply = wt.get("apply").unwrap();
        let copy = apply
            .get("copy_from_main")
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0].as_str(), Some(".new"));
    }

    #[test]
    fn coordination_branch_defaults_exist_in_cascading_config() {
        let repo = tempfile::tempdir().unwrap();
        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        let changes = r.merged.get("changes").expect("changes key should exist");
        let coordination = changes
            .get("coordination_branch")
            .expect("coordination_branch key should exist");

        assert_eq!(
            coordination.get("enabled").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            coordination.get("name").and_then(|v| v.as_str()),
            Some("ito/internal/changes")
        );
    }

    #[test]
    fn coordination_branch_defaults_can_be_overridden() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::write(
            repo.path().join("ito.json"),
            r#"{"changes":{"coordination_branch":{"enabled":false,"name":"team/internal/coord"}}}"#,
        )
        .unwrap();

        let ctx = ConfigContext::default();
        let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

        let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
        let changes = r.merged.get("changes").expect("changes key should exist");
        let coordination = changes
            .get("coordination_branch")
            .expect("coordination_branch key should exist");

        assert_eq!(
            coordination.get("enabled").and_then(|v| v.as_bool()),
            Some(false)
        );
        assert_eq!(
            coordination.get("name").and_then(|v| v.as_str()),
            Some("team/internal/coord")
        );
    }

    // ito_dir tests live in crate::ito_dir.
}
