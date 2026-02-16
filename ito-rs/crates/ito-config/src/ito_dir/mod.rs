//! Ito working directory discovery.
//!
//! This module answers: "where is the `.ito/` directory for this project?".
//! It mirrors the precedence rules from the TypeScript implementation.

use std::path::{Path, PathBuf};

use ito_common::fs::{FileSystem, StdFs};

use crate::{ConfigContext, load_global_config_fs, load_repo_project_path_override_fs};

/// Determine the configured Ito working directory name.
///
/// This returns the directory name (not a full path). It consults repo-local
/// configuration first, then global config, then falls back to `.ito`.
pub fn get_ito_dir_name(project_root: &Path, ctx: &ConfigContext) -> String {
    get_ito_dir_name_fs(&StdFs, project_root, ctx)
}

/// Like [`get_ito_dir_name`], but uses an injected file-system.
pub fn get_ito_dir_name_fs<F: FileSystem>(
    fs: &F,
    project_root: &Path,
    ctx: &ConfigContext,
) -> String {
    // Priority order matches TS:
    // 1. Repo-level ito.json projectPath
    // 2. Repo-level .ito.json projectPath
    // 3. Global config (~/.config/ito/config.json) projectPath
    // 4. Default: '.ito'
    if let Some(project_path) = load_repo_project_path_override_fs(fs, project_root)
        && let Some(project_path) = sanitize_ito_dir_name(&project_path)
    {
        return project_path;
    }

    if let Some(project_path) = load_global_config_fs(fs, ctx).project_path
        && let Some(project_path) = sanitize_ito_dir_name(&project_path)
    {
        return project_path;
    }

    ".ito".to_string()
}

/// Resolve the `.ito/` path for `project_root`.
pub fn get_ito_path(project_root: &Path, ctx: &ConfigContext) -> PathBuf {
    get_ito_path_fs(&StdFs, project_root, ctx)
}

/// Resolve the Ito directory path for a project using an injected filesystem.
///
/// This absolutizes and lexically normalizes `project_root` (falling back to a lossy
/// normalization if the current working directory cannot be determined) and appends
/// the Ito directory name selected from repository overrides, global configuration, or the default.
///
/// # Examples
///
/// ```no_run
/// use ito_common::fs::StdFs;
/// use ito_config::ConfigContext;
/// use ito_config::ito_dir::get_ito_path_fs;
/// use std::path::Path;
///
/// let fs = StdFs;
/// let ctx = ConfigContext::default();
/// let ito_path = get_ito_path_fs(&fs, Path::new("some/project"), &ctx);
/// assert!(ito_path.ends_with(".ito"));
/// ```
pub fn get_ito_path_fs<F: FileSystem>(fs: &F, project_root: &Path, ctx: &ConfigContext) -> PathBuf {
    let root = absolutize_and_normalize_lossy(project_root);
    root.join(get_ito_dir_name_fs(fs, &root, ctx))
}

/// Resolves a possibly-relative path to an absolute, lexically normalized form.
///
/// If `input` is absolute it is normalized in place; otherwise it is joined
/// onto the current working directory before normalization. The result has no
/// `.` or `..` components and does not access the filesystem, so it is safe to
/// use for display even when the path does not exist.
///
/// # Errors
///
/// Returns an `io::Error` when the current directory cannot be determined and
/// `input` is relative.
///
/// # Examples
///
/// ```
/// use ito_config::ito_dir::absolutize_and_normalize;
/// use std::path::Path;
///
/// let abs = absolutize_and_normalize(Path::new(".")).unwrap();
/// assert!(abs.is_absolute());
/// ```
pub fn absolutize_and_normalize(input: &Path) -> std::io::Result<PathBuf> {
    let abs = if input.is_absolute() {
        input.to_path_buf()
    } else {
        std::env::current_dir()?.join(input)
    };

    Ok(lexical_normalize(&abs))
}

/// Produce an absolute, lexically normalized PathBuf, falling back to lexical normalization if the current working directory cannot be determined.
///
/// The result is a canonicalized form of `input` where `.` and `..` components are resolved lexically. If obtaining the current directory fails, this function returns the lexically normalized `input` without attempting to make it absolute.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let p = super::absolutize_and_normalize_lossy(Path::new("foo/./bar"));
/// assert!(p.ends_with(Path::new("foo/bar")));
/// ```
fn absolutize_and_normalize_lossy(input: &Path) -> PathBuf {
    absolutize_and_normalize(input).unwrap_or_else(|_| lexical_normalize(input))
}

/// Lexically normalizes a path by resolving `.` and `..` components without accessing the filesystem.
///
/// This performs purely lexical simplification: it removes `.` segments, collapses `..` where possible,
/// preserves rooted prefixes, and never queries the filesystem.
///
/// # Examples
///
/// ```
/// use ito_config::ito_dir::lexical_normalize;
/// use std::path::{Path, PathBuf};
///
/// let p = Path::new("a/./b/../c");
/// assert_eq!(lexical_normalize(p), PathBuf::from("a/c"));
///
/// let abs = Path::new("/a/b/../c");
/// assert_eq!(lexical_normalize(abs), PathBuf::from("/a/c"));
///
/// let up = Path::new("../a/../b");
/// assert_eq!(lexical_normalize(up), PathBuf::from("../b"));
/// ```
pub fn lexical_normalize(path: &Path) -> PathBuf {
    use std::path::Component;

    let mut out = PathBuf::new();
    let mut stack: Vec<std::ffi::OsString> = Vec::new();
    let mut rooted = false;

    for c in path.components() {
        match c {
            Component::Prefix(p) => {
                out.push(p.as_os_str());
            }
            Component::RootDir => {
                rooted = true;
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if let Some(last) = stack.last()
                    && last != ".."
                {
                    stack.pop();
                    continue;
                }
                if !rooted {
                    stack.push(std::ffi::OsString::from(".."));
                }
            }
            Component::Normal(seg) => {
                stack.push(seg.to_os_string());
            }
        }
    }

    if rooted {
        out.push(std::path::MAIN_SEPARATOR.to_string());
    }
    for seg in stack {
        out.push(seg);
    }

    out
}

fn sanitize_ito_dir_name(input: &str) -> Option<String> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    if input.len() > 128 {
        return None;
    }

    if input.contains('/') || input.contains('\\') || input.contains("..") {
        return None;
    }

    if Path::new(input).is_absolute() {
        return None;
    }

    Some(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_ito_dir_name_defaults_to_dot_ito() {
        let td = tempfile::tempdir().unwrap();
        let ctx = ConfigContext::default();
        assert_eq!(get_ito_dir_name(td.path(), &ctx), ".ito");
    }

    #[test]
    fn repo_config_overrides_global_config() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(
            td.path().join("ito.json"),
            "{\"projectPath\":\".repo-ito\"}",
        )
        .unwrap();

        let home = tempfile::tempdir().unwrap();
        let cfg_dir = home.path().join(".config/ito");
        std::fs::create_dir_all(&cfg_dir).unwrap();
        std::fs::write(
            cfg_dir.join("config.json"),
            "{\"projectPath\":\".global-ito\"}",
        )
        .unwrap();

        let ctx = ConfigContext {
            xdg_config_home: None,
            home_dir: Some(home.path().to_path_buf()),
            project_dir: None,
        };

        assert_eq!(get_ito_dir_name(td.path(), &ctx), ".repo-ito");
    }

    #[test]
    fn dot_repo_config_overrides_repo_config() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(
            td.path().join("ito.json"),
            "{\"projectPath\":\".repo-ito\"}",
        )
        .unwrap();
        std::fs::write(
            td.path().join(".ito.json"),
            "{\"projectPath\":\".dot-ito\"}",
        )
        .unwrap();

        let ctx = ConfigContext::default();
        assert_eq!(get_ito_dir_name(td.path(), &ctx), ".dot-ito");
    }

    #[test]
    fn get_ito_path_normalizes_dotdot_segments() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        std::fs::create_dir_all(repo.join("a")).unwrap();
        std::fs::create_dir_all(repo.join("b")).unwrap();

        let ctx = ConfigContext::default();
        let p = repo.join("a/../b");

        let ito_path = get_ito_path(&p, &ctx);
        assert!(ito_path.ends_with("b/.ito"));
    }

    #[test]
    fn invalid_repo_project_path_falls_back_to_default() {
        let td = tempfile::tempdir().unwrap();
        std::fs::write(
            td.path().join("ito.json"),
            "{\"projectPath\":\"../escape\"}",
        )
        .unwrap();

        let ctx = ConfigContext::default();
        assert_eq!(get_ito_dir_name(td.path(), &ctx), ".ito");
    }

    #[test]
    fn sanitize_rejects_path_separators_and_overlong_values() {
        assert_eq!(sanitize_ito_dir_name(".ito"), Some(".ito".to_string()));
        assert_eq!(sanitize_ito_dir_name("../x"), None);
        assert_eq!(sanitize_ito_dir_name("a/b"), None);
        assert_eq!(sanitize_ito_dir_name("a\\b"), None);
        assert_eq!(sanitize_ito_dir_name(&"a".repeat(129)), None);
    }
}
