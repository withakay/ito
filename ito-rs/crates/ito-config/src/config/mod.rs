use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use ito_common::fs::{FileSystem, StdFs};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod defaults;
pub mod schema;
pub mod types;

const REPO_CONFIG_FILE_NAME: &str = "ito.json";
const REPO_DOT_CONFIG_FILE_NAME: &str = ".ito.json";
const ITO_DIR_CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigContext {
    pub xdg_config_home: Option<PathBuf>,
    pub home_dir: Option<PathBuf>,
    pub project_dir: Option<PathBuf>,
}

impl ConfigContext {
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

pub fn load_project_config(project_root: &Path) -> Option<ProjectConfig> {
    load_project_config_fs(&StdFs, project_root)
}

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
pub struct CascadingProjectConfig {
    pub merged: Value,
    pub loaded_from: Vec<PathBuf>,
}

pub type ResolvedConfig = CascadingProjectConfig;

pub fn project_config_paths(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();

    out.push(project_root.join(REPO_CONFIG_FILE_NAME));
    out.push(project_root.join(REPO_DOT_CONFIG_FILE_NAME));
    out.push(ito_path.join(ITO_DIR_CONFIG_FILE_NAME));
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
/// 3) `<itoDir>/config.json`
/// 4) `$PROJECT_DIR/config.json` (when set)
pub fn load_cascading_project_config(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ConfigContext,
) -> CascadingProjectConfig {
    load_cascading_project_config_fs(&StdFs, project_root, ito_path, ctx)
}

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
        let Some(v) = load_json_object_fs(fs, &path) else {
            continue;
        };
        merge_json(&mut merged, v);
        loaded_from.push(path);
    }

    CascadingProjectConfig {
        merged,
        loaded_from,
    }
}

pub fn global_config_path(ctx: &ConfigContext) -> Option<PathBuf> {
    ito_config_dir(ctx).map(|d| d.join("config.json"))
}

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

pub fn load_global_config(ctx: &ConfigContext) -> GlobalConfig {
    load_global_config_fs(&StdFs, ctx)
}

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

    // ito_dir tests live in crate::ito_dir.
}
