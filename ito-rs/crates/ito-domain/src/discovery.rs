//! Filesystem discovery helpers.
//!
//! These helpers list change/module/spec directories under an Ito project.
//! They are used by higher-level repositories and CLI commands.

use std::collections::BTreeSet;
use std::path::Path;

use miette::{IntoDiagnostic, Result};

use ito_common::fs::FileSystem;
use ito_common::paths;

fn list_child_dirs<F: FileSystem>(fs: &F, dir: &Path) -> Result<Vec<String>> {
    if !fs.exists(dir) {
        return Ok(Vec::new());
    }

    let entries = fs.read_dir(dir).into_diagnostic()?;
    let mut out: Vec<String> = Vec::new();
    for path in entries {
        if !fs.is_dir(&path) {
            continue;
        }

        let Some(name) = path.file_name() else {
            continue;
        };
        let name = name.to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        out.push(name);
    }

    out.sort();
    Ok(out)
}

/// List child directory names under `dir`.
///
/// Returned names are sorted. Non-directory entries are ignored.
pub fn list_dir_names<F: FileSystem>(fs: &F, dir: &Path) -> Result<Vec<String>> {
    list_child_dirs(fs, dir)
}

/// List change directory names under `{ito_path}/changes`, excluding `archive`.
pub fn list_change_dir_names<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<Vec<String>> {
    let mut out = list_child_dirs(fs, paths::changes_dir(ito_path).as_path())?;
    out.retain(|n| n != "archive");
    Ok(out)
}

/// List module directory names under `{ito_path}/modules`.
pub fn list_module_dir_names<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<Vec<String>> {
    list_child_dirs(fs, paths::modules_dir(ito_path).as_path())
}

/// Extract module ids (3-digit prefixes) from the module directory names.
pub fn list_module_ids<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<BTreeSet<String>> {
    let mut ids: BTreeSet<String> = BTreeSet::new();
    for name in list_module_dir_names(fs, ito_path)? {
        let Some((id_part, _)) = name.split_once('_') else {
            continue;
        };
        if id_part.len() == 3 && id_part.chars().all(|c| c.is_ascii_digit()) {
            ids.insert(id_part.to_string());
        }
    }
    Ok(ids)
}

/// List spec directory names under `{ito_path}/specs`.
pub fn list_spec_dir_names<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<Vec<String>> {
    list_child_dirs(fs, paths::specs_dir(ito_path).as_path())
}

// Spec-facing API.
/// List changes (spec-facing API).
pub fn list_changes<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<Vec<String>> {
    list_change_dir_names(fs, ito_path)
}

/// List modules (spec-facing API).
pub fn list_modules<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<Vec<String>> {
    list_module_dir_names(fs, ito_path)
}

/// List specs (spec-facing API).
pub fn list_specs<F: FileSystem>(fs: &F, ito_path: &Path) -> Result<Vec<String>> {
    list_spec_dir_names(fs, ito_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    use ito_common::fs::StdFs;

    #[test]
    fn list_changes_skips_archive_dir() {
        let td = tempfile::tempdir().unwrap();
        let ito_path = td.path().join(".ito");
        std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
        std::fs::create_dir_all(ito_path.join("changes/001-01_test")).unwrap();

        let fs = StdFs;
        let changes = list_changes(&fs, &ito_path).unwrap();
        assert_eq!(changes, vec!["001-01_test".to_string()]);
    }

    #[test]
    fn list_modules_only_returns_directories() {
        let td = tempfile::tempdir().unwrap();
        let ito_path = td.path().join(".ito");
        std::fs::create_dir_all(ito_path.join("modules/001_project-setup")).unwrap();
        std::fs::create_dir_all(ito_path.join("modules/.hidden")).unwrap();
        std::fs::create_dir_all(ito_path.join("modules/not-a-module")).unwrap();
        std::fs::write(ito_path.join("modules/file.txt"), "x").unwrap();

        let fs = StdFs;
        let modules = list_modules(&fs, &ito_path).unwrap();
        assert_eq!(
            modules,
            vec!["001_project-setup".to_string(), "not-a-module".to_string()]
        );
    }

    #[test]
    fn list_module_ids_extracts_numeric_prefixes() {
        let td = tempfile::tempdir().unwrap();
        let ito_path = td.path().join(".ito");
        std::fs::create_dir_all(ito_path.join("modules/001_project-setup")).unwrap();
        std::fs::create_dir_all(ito_path.join("modules/002_tools")).unwrap();
        std::fs::create_dir_all(ito_path.join("modules/not-a-module")).unwrap();

        let fs = StdFs;
        let ids = list_module_ids(&fs, &ito_path).unwrap();
        assert_eq!(
            ids.into_iter().collect::<Vec<_>>(),
            vec!["001".to_string(), "002".to_string()]
        );
    }
}
