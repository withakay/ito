//! Filesystem-backed module repository implementation.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ito_common::fs::{FileSystem, StdFs};
use ito_domain::changes::{extract_module_id, parse_module_id};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository as DomainModuleRepository, ModuleSummary};

/// Filesystem-backed implementation of the domain `ModuleRepository` port.
pub struct FsModuleRepository<'a, F: FileSystem = StdFs> {
    ito_path: &'a Path,
    fs: F,
}

impl<'a> FsModuleRepository<'a, StdFs> {
    /// Create a filesystem-backed module repository using the standard filesystem.
    pub fn new(ito_path: &'a Path) -> Self {
        Self {
            ito_path,
            fs: StdFs,
        }
    }
}

impl<'a, F: FileSystem> FsModuleRepository<'a, F> {
    /// Create a filesystem-backed module repository with a custom filesystem.
    pub fn with_fs(ito_path: &'a Path, fs: F) -> Self {
        Self { ito_path, fs }
    }

    /// Get the path to the modules directory.
    fn modules_dir(&self) -> PathBuf {
        self.ito_path.join("modules")
    }

    /// Find the full module directory for a given module id or full name.
    fn find_module_dir(&self, id_or_name: &str) -> Option<PathBuf> {
        let modules_dir = self.modules_dir();
        if !self.fs.is_dir(&modules_dir) {
            return None;
        }

        let normalized_id = parse_module_id(id_or_name);
        let prefix = format!("{normalized_id}_");

        self.fs
            .read_dir(&modules_dir)
            .ok()?
            .into_iter()
            .find(|entry| {
                entry
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with(&prefix))
                    .unwrap_or(false)
            })
    }

    fn load_module_description(&self, module_path: &Path) -> DomainResult<Option<String>> {
        let yaml_path = module_path.join("module.yaml");
        if !self.fs.is_file(&yaml_path) {
            return Ok(None);
        }

        let content = self
            .fs
            .read_to_string(&yaml_path)
            .map_err(|source| DomainError::io("reading module.yaml", source))?;

        for line in content.lines() {
            let line = line.trim();
            if let Some(desc) = line.strip_prefix("description:") {
                let desc = desc.trim().trim_matches('"').trim_matches('\'');
                if !desc.is_empty() {
                    return Ok(Some(desc.to_string()));
                }
            }
        }

        Ok(None)
    }

    fn count_changes_by_module(&self) -> DomainResult<HashMap<String, u32>> {
        let mut counts = HashMap::new();
        let changes_dir = self.ito_path.join("changes");
        if !self.fs.is_dir(&changes_dir) {
            return Ok(counts);
        }

        for path in self
            .fs
            .read_dir(&changes_dir)
            .map_err(|source| DomainError::io("listing change directories", source))?
        {
            if !self.fs.is_dir(&path) {
                continue;
            }

            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            if let Some(module_id) = extract_module_id(name) {
                *counts.entry(module_id).or_insert(0) += 1;
            }
        }

        Ok(counts)
    }

    /// Check if a module exists.
    pub fn exists(&self, id: &str) -> bool {
        DomainModuleRepository::exists(self, id)
    }

    /// Get a module by ID or full name.
    pub fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        DomainModuleRepository::get(self, id_or_name)
    }

    /// List all modules.
    pub fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        DomainModuleRepository::list(self)
    }
}

impl<F: FileSystem> DomainModuleRepository for FsModuleRepository<'_, F> {
    fn exists(&self, id: &str) -> bool {
        self.find_module_dir(id).is_some()
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        let path = self
            .find_module_dir(id_or_name)
            .ok_or_else(|| DomainError::not_found("module", id_or_name))?;

        let id = parse_module_id(id_or_name);
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_prefix(&format!("{id}_")))
            .unwrap_or("unknown")
            .to_string();

        let description = self.load_module_description(&path)?;

        Ok(Module {
            id,
            name,
            description,
            path,
        })
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        let modules_dir = self.modules_dir();
        if !self.fs.is_dir(&modules_dir) {
            return Ok(Vec::new());
        }

        let change_counts = self.count_changes_by_module()?;

        let mut summaries = Vec::new();
        for path in self
            .fs
            .read_dir(&modules_dir)
            .map_err(|source| DomainError::io("listing module directories", source))?
        {
            if !self.fs.is_dir(&path) {
                continue;
            }

            let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let Some((id, name)) = dir_name.split_once('_') else {
                continue;
            };

            summaries.push(ModuleSummary {
                id: id.to_string(),
                name: name.to_string(),
                change_count: change_counts.get(id).copied().unwrap_or(0),
            });
        }

        summaries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(summaries)
    }
}

/// Backward-compatible alias for the default filesystem-backed module repository.
pub type ModuleRepository<'a> = FsModuleRepository<'a, StdFs>;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;

    use super::{FsModuleRepository, ModuleRepository};

    fn setup_test_ito(tmp: &TempDir) -> std::path::PathBuf {
        let ito_path = tmp.path().join(".ito");
        fs::create_dir_all(ito_path.join("modules")).unwrap();
        fs::create_dir_all(ito_path.join("changes")).unwrap();
        ito_path
    }

    fn create_module(ito_path: &Path, id: &str, name: &str) {
        let module_dir = ito_path.join("modules").join(format!("{}_{}", id, name));
        fs::create_dir_all(&module_dir).unwrap();
    }

    fn create_change(ito_path: &Path, id: &str) {
        let change_dir = ito_path.join("changes").join(id);
        fs::create_dir_all(&change_dir).unwrap();
    }

    #[test]
    fn test_exists() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_module(&ito_path, "005", "dev-tooling");

        let repo = ModuleRepository::new(&ito_path);
        assert!(repo.exists("005"));
        assert!(!repo.exists("999"));
    }

    #[test]
    fn test_get() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_module(&ito_path, "005", "dev-tooling");

        let repo = ModuleRepository::new(&ito_path);
        let module = repo.get("005").unwrap();

        assert_eq!(module.id, "005");
        assert_eq!(module.name, "dev-tooling");
    }

    #[test]
    fn test_get_not_found() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);

        let repo = ModuleRepository::new(&ito_path);
        let result = repo.get("999");
        assert!(result.is_err());
    }

    #[test]
    fn test_list() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_module(&ito_path, "005", "dev-tooling");
        create_module(&ito_path, "003", "qa-testing");
        create_module(&ito_path, "001", "workflow");

        let repo = ModuleRepository::new(&ito_path);
        let modules = repo.list().unwrap();

        assert_eq!(modules.len(), 3);
        assert_eq!(modules[0].id, "001");
        assert_eq!(modules[1].id, "003");
        assert_eq!(modules[2].id, "005");
    }

    #[test]
    fn test_list_with_change_counts() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_module(&ito_path, "005", "dev-tooling");
        create_module(&ito_path, "003", "qa-testing");

        create_change(&ito_path, "005-01_first");
        create_change(&ito_path, "005-02_second");
        create_change(&ito_path, "003-01_test");

        let repo = ModuleRepository::new(&ito_path);
        let modules = repo.list().unwrap();

        let module_005 = modules.iter().find(|m| m.id == "005").unwrap();
        let module_003 = modules.iter().find(|m| m.id == "003").unwrap();

        assert_eq!(module_005.change_count, 2);
        assert_eq!(module_003.change_count, 1);
    }

    #[test]
    fn test_get_uses_full_name_input() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_module(&ito_path, "005", "dev-tooling");

        let repo = FsModuleRepository::new(&ito_path);
        let module = repo.get("005_dev-tooling").unwrap();
        assert_eq!(module.id, "005");
        assert_eq!(module.name, "dev-tooling");
    }
}
