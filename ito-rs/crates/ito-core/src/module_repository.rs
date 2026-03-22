//! Filesystem-backed module repository implementation.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ito_common::fs::{FileSystem, StdFs};
use ito_domain::changes::{extract_module_id, parse_module_id};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{
    Module, ModuleRepository as DomainModuleRepository, ModuleSummary, SubModule, SubModuleSummary,
};

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

        let entries = self.fs.read_dir(&modules_dir).ok()?;
        for entry in entries {
            let matches = entry
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&prefix));
            if matches {
                return Some(entry);
            }
        }
        None
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
        use ito_common::id::{ItoIdKind, classify_id};

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

            // Only count direct module changes (NNN-NN_name), not sub-module
            // changes (NNN.SS-NN_name). Sub-module changes are counted separately
            // in `count_changes_by_sub_module`.
            if classify_id(name) == ItoIdKind::SubModuleChangeId {
                continue;
            }

            if let Some(module_id) = extract_module_id(name) {
                *counts.entry(module_id).or_insert(0) += 1;
            }
        }

        Ok(counts)
    }

    /// Count changes per sub-module key (e.g., `"024.01"`) by scanning the
    /// changes directory for entries whose IDs contain a dot before the hyphen.
    fn count_changes_by_sub_module(&self) -> DomainResult<HashMap<String, u32>> {
        use ito_common::id::{ItoIdKind, classify_id};

        let mut counts: HashMap<String, u32> = HashMap::new();
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

            if classify_id(name) == ItoIdKind::SubModuleChangeId {
                // Extract the NNN.SS prefix (everything before the first `-`).
                if let Some(sub_module_key) = name.split('-').next() {
                    // Normalize: parse the NNN.SS part.
                    if let Ok(parsed) =
                        ito_common::id::parse_sub_module_id(sub_module_key)
                    {
                        *counts
                            .entry(parsed.sub_module_id.as_str().to_string())
                            .or_insert(0) += 1;
                    }
                }
            }
        }

        Ok(counts)
    }

    /// Scan a module directory for sub-module directories under `sub/` and
    /// return full [`SubModule`] values.
    ///
    /// Sub-module directories follow the `SS_name` naming convention where
    /// `SS` is a zero-padded two-digit number.
    fn scan_sub_modules(
        &self,
        module_dir: &Path,
        parent_module_id: &str,
    ) -> DomainResult<Vec<SubModule>> {
        let sub_dir = module_dir.join("sub");
        if !self.fs.is_dir(&sub_dir) {
            return Ok(Vec::new());
        }

        let change_counts = self.count_changes_by_sub_module()?;
        let mut sub_modules = Vec::new();

        for path in self
            .fs
            .read_dir(&sub_dir)
            .map_err(|source| DomainError::io("listing sub-module directories", source))?
        {
            if !self.fs.is_dir(&path) {
                continue;
            }

            let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            let Some((sub_num_str, name)) = dir_name.split_once('_') else {
                continue;
            };

            // Validate that sub_num_str is numeric.
            if sub_num_str.is_empty() || !sub_num_str.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            let sub_num: u32 = match sub_num_str.parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            let sub_id_str = format!("{parent_module_id}.{sub_num:02}");
            let description = self.load_sub_module_description(&path)?;
            let change_count = change_counts.get(&sub_id_str).copied().unwrap_or(0);

            sub_modules.push(SubModule {
                id: sub_id_str,
                parent_module_id: parent_module_id.to_string(),
                sub_id: format!("{sub_num:02}"),
                name: name.to_string(),
                description,
                change_count,
                path,
            });
        }

        sub_modules.sort_by(|a, b| a.sub_id.cmp(&b.sub_id));
        Ok(sub_modules)
    }

    /// Scan a module directory for sub-module directories under `sub/` and
    /// return lightweight [`SubModuleSummary`] values.
    fn scan_sub_module_summaries(
        &self,
        module_dir: &Path,
        parent_module_id: &str,
    ) -> DomainResult<Vec<SubModuleSummary>> {
        let sub_modules = self.scan_sub_modules(module_dir, parent_module_id)?;
        let mut summaries = Vec::with_capacity(sub_modules.len());
        for sm in sub_modules {
            summaries.push(SubModuleSummary {
                id: sm.id,
                name: sm.name,
                change_count: sm.change_count,
            });
        }
        Ok(summaries)
    }

    /// Load an optional description from a sub-module's `module.md`.
    ///
    /// Looks for the first non-empty paragraph after the `## Purpose` heading.
    fn load_sub_module_description(&self, sub_module_path: &Path) -> DomainResult<Option<String>> {
        let md_path = sub_module_path.join("module.md");
        if !self.fs.is_file(&md_path) {
            return Ok(None);
        }

        let content = self
            .fs
            .read_to_string(&md_path)
            .map_err(|source| DomainError::io("reading sub-module module.md", source))?;

        // Extract the first non-empty line after `## Purpose`.
        let mut in_purpose = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.eq_ignore_ascii_case("## purpose") {
                in_purpose = true;
                continue;
            }
            if in_purpose {
                if trimmed.starts_with('#') {
                    // Reached the next heading — stop.
                    break;
                }
                if !trimmed.is_empty() {
                    return Ok(Some(trimmed.to_string()));
                }
            }
        }

        Ok(None)
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
        let sub_modules = self.scan_sub_modules(&path, &id)?;

        Ok(Module {
            id,
            name,
            description,
            path,
            sub_modules,
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

            let sub_modules = self.scan_sub_module_summaries(&path, id)?;

            summaries.push(ModuleSummary {
                id: id.to_string(),
                name: name.to_string(),
                change_count: change_counts.get(id).copied().unwrap_or(0),
                sub_modules,
            });
        }

        summaries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(summaries)
    }

    fn list_sub_modules(&self, parent_id: &str) -> DomainResult<Vec<SubModuleSummary>> {
        let path = self
            .find_module_dir(parent_id)
            .ok_or_else(|| DomainError::not_found("module", parent_id))?;

        let normalized_id = parse_module_id(parent_id);
        self.scan_sub_module_summaries(&path, &normalized_id)
    }

    fn get_sub_module(&self, composite_id: &str) -> DomainResult<SubModule> {
        // Parse the composite id (e.g., "024.01" or "024.01_auth").
        let parsed = ito_common::id::parse_sub_module_id(composite_id).map_err(|e| {
            DomainError::io(
                "parsing sub-module id",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, e.error),
            )
        })?;

        let parent_id = parsed.parent_module_id.as_str();
        let sub_num = &parsed.sub_num;

        let module_path = self
            .find_module_dir(parent_id)
            .ok_or_else(|| DomainError::not_found("module", parent_id))?;

        let sub_dir = module_path.join("sub");
        if !self.fs.is_dir(&sub_dir) {
            return Err(DomainError::not_found("sub-module", composite_id));
        }

        // Find the sub-module directory matching the sub number prefix.
        let prefix = format!("{sub_num}_");
        let entries = self
            .fs
            .read_dir(&sub_dir)
            .map_err(|source| DomainError::io("listing sub-module directories", source))?;
        let mut sub_module_path = None;
        for entry in entries {
            let matches = entry
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&prefix));
            if matches {
                sub_module_path = Some(entry);
                break;
            }
        }
        let Some(sub_module_path) = sub_module_path else {
            return Err(DomainError::not_found("sub-module", composite_id));
        };

        let Some(dir_name) = sub_module_path.file_name().and_then(|n| n.to_str()) else {
            return Err(DomainError::not_found("sub-module", composite_id));
        };

        let name = dir_name
            .strip_prefix(&prefix)
            .unwrap_or(dir_name)
            .to_string();

        let change_counts = self.count_changes_by_sub_module()?;
        let sub_id_str = parsed.sub_module_id.as_str().to_string();
        let change_count = change_counts.get(&sub_id_str).copied().unwrap_or(0);
        let description = self.load_sub_module_description(&sub_module_path)?;

        Ok(SubModule {
            id: sub_id_str,
            parent_module_id: parent_id.to_string(),
            sub_id: sub_num.clone(),
            name,
            description,
            change_count,
            path: sub_module_path,
        })
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

    // ── Task 2.7: Regression tests for sub-module support ─────────────────

    /// Set up a sub-module directory under a parent module.
    fn create_sub_module(ito_path: &Path, parent_id: &str, parent_name: &str, sub_num: &str, sub_name: &str) {
        let module_dir = ito_path.join("modules").join(format!("{parent_id}_{parent_name}"));
        let sub_dir = module_dir.join("sub").join(format!("{sub_num}_{sub_name}"));
        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(
            sub_dir.join("module.md"),
            format!("# {sub_name}\n\n## Purpose\n{sub_name} sub-module\n"),
        )
        .unwrap();
    }

    #[test]
    fn regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes() {
        // Regression test: a parent module can have direct changes (024-07_*)
        // while a sub-module (024.01) owns its own changes (024.01-01_*).
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);

        // Set up module 024 with sub-module 01_auth.
        create_module(&ito_path, "024", "ito-backend");
        create_sub_module(&ito_path, "024", "ito-backend", "01", "auth");

        // Direct change on the parent module.
        create_change(&ito_path, "024-07_health-check");
        // Sub-module change.
        create_change(&ito_path, "024.01-01_add-jwt");

        let repo = ModuleRepository::new(&ito_path);

        // --- Module listing ---
        let modules = repo.list().unwrap();
        assert_eq!(modules.len(), 1);
        let m = &modules[0];
        assert_eq!(m.id, "024");
        // The parent module's change_count should include only the direct change
        // (024-07_health-check). The sub-module change (024.01-01_add-jwt) is
        // attributed to the sub-module, not the parent.
        assert_eq!(m.change_count, 1, "parent module should count only direct changes");
        // Sub-module summary should be populated.
        assert_eq!(m.sub_modules.len(), 1);
        assert_eq!(m.sub_modules[0].id, "024.01");
        assert_eq!(m.sub_modules[0].name, "auth");
        assert_eq!(m.sub_modules[0].change_count, 1, "sub-module should count its own change");

        // --- Module get ---
        let module = repo.get("024").unwrap();
        assert_eq!(module.id, "024");
        assert_eq!(module.sub_modules.len(), 1);
        assert_eq!(module.sub_modules[0].id, "024.01");
        assert_eq!(module.sub_modules[0].parent_module_id, "024");
        assert_eq!(module.sub_modules[0].sub_id, "01");
        assert_eq!(module.sub_modules[0].name, "auth");
        assert_eq!(module.sub_modules[0].change_count, 1);

        // --- list_sub_modules ---
        use ito_domain::modules::ModuleRepository as DomainModuleRepository;
        let sub_summaries = DomainModuleRepository::list_sub_modules(&repo, "024").unwrap();
        assert_eq!(sub_summaries.len(), 1);
        assert_eq!(sub_summaries[0].id, "024.01");

        // --- get_sub_module ---
        let sub = DomainModuleRepository::get_sub_module(&repo, "024.01").unwrap();
        assert_eq!(sub.id, "024.01");
        assert_eq!(sub.parent_module_id, "024");
        assert_eq!(sub.sub_id, "01");
        assert_eq!(sub.name, "auth");
        assert_eq!(sub.change_count, 1);
    }

    #[test]
    fn regression_change_repository_populates_sub_module_id() {
        use crate::change_repository::FsChangeRepository;

        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);

        // Set up module 024 with sub-module 01_auth.
        create_module(&ito_path, "024", "ito-backend");
        create_sub_module(&ito_path, "024", "ito-backend", "01", "auth");

        // Direct change on the parent module.
        create_change(&ito_path, "024-07_health-check");
        // Sub-module change.
        create_change(&ito_path, "024.01-01_add-jwt");

        let change_repo = FsChangeRepository::new(&ito_path);
        let summaries = change_repo.list().unwrap();

        let direct = summaries.iter().find(|s| s.id == "024-07_health-check").unwrap();
        assert_eq!(direct.module_id.as_deref(), Some("024"));
        assert_eq!(direct.sub_module_id, None, "direct change should have no sub_module_id");

        let sub_change = summaries.iter().find(|s| s.id == "024.01-01_add-jwt").unwrap();
        assert_eq!(sub_change.module_id.as_deref(), Some("024"));
        assert_eq!(
            sub_change.sub_module_id.as_deref(),
            Some("024.01"),
            "sub-module change should have sub_module_id set"
        );
    }
}
