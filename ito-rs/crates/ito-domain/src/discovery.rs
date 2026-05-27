//! Filesystem discovery helpers.
//!
//! These helpers list change/module/spec directories under an Ito project.
//! They are used by higher-level repositories and CLI commands.

use std::collections::BTreeSet;
use std::path::Path;

use crate::errors::{DomainError, DomainResult};
use ito_common::fs::FileSystem;
use ito_common::paths;

fn list_child_dirs<F: FileSystem>(fs: &F, dir: &Path) -> DomainResult<Vec<String>> {
    if !fs.exists(dir) {
        return Ok(Vec::new());
    }

    let entries = fs
        .read_dir(dir)
        .map_err(|source| DomainError::io("listing directory entries", source))?;
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
pub fn list_dir_names<F: FileSystem>(fs: &F, dir: &Path) -> DomainResult<Vec<String>> {
    list_child_dirs(fs, dir)
}

/// List change directory names under `{ito_path}/changes`, excluding `archive`.
pub fn list_change_dir_names<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<Vec<String>> {
    let mut out = list_child_dirs(fs, paths::changes_dir(ito_path).as_path())?;
    out.retain(|n| n != "archive");
    Ok(out)
}

/// List module directory names under `{ito_path}/modules`.
pub fn list_module_dir_names<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<Vec<String>> {
    list_child_dirs(fs, paths::modules_dir(ito_path).as_path())
}

/// Extract module ids (3-digit prefixes) from the module directory names.
pub fn list_module_ids<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<BTreeSet<String>> {
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
pub fn list_spec_dir_names<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<Vec<String>> {
    list_child_dirs(fs, paths::specs_dir(ito_path).as_path())
}

// Spec-facing API.
/// List changes (spec-facing API).
pub fn list_changes<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<Vec<String>> {
    list_change_dir_names(fs, ito_path)
}

/// List modules (spec-facing API).
pub fn list_modules<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<Vec<String>> {
    list_module_dir_names(fs, ito_path)
}

/// List specs (spec-facing API).
pub fn list_specs<F: FileSystem>(fs: &F, ito_path: &Path) -> DomainResult<Vec<String>> {
    list_spec_dir_names(fs, ito_path)
}

#[cfg(test)]
#[path = "discovery_tests.rs"]
mod discovery_tests;
