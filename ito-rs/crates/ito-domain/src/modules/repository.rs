//! Module Repository - Clean abstraction over module storage.

use miette::{IntoDiagnostic, Result, miette};
use std::fs;
use std::path::Path;

use super::{Module, ModuleSummary};
use crate::changes::{extract_module_id, parse_module_id};

/// Repository for accessing module data.
///
/// This abstraction hides the file system storage format from consumers.
/// All module queries should go through this interface rather than
/// directly reading files.
pub struct ModuleRepository<'a> {
    ito_path: &'a Path,
}

impl<'a> ModuleRepository<'a> {
    /// Create a new module repository for the given ito directory.
    pub fn new(ito_path: &'a Path) -> Self {
        Self { ito_path }
    }

    /// Get the path to the modules directory.
    fn modules_dir(&self) -> std::path::PathBuf {
        self.ito_path.join("modules")
    }

    /// Find the full module directory name for a given ID or full name.
    ///
    /// Accepts flexible formats:
    /// - Just the ID: "000", "0", "5"
    /// - Full directory name: "000_ungrouped", "5_dev-tooling"
    fn find_module_dir(&self, id_or_name: &str) -> Option<std::path::PathBuf> {
        let modules_dir = self.modules_dir();
        if !modules_dir.is_dir() {
            return None;
        }

        // Normalize the ID using the shared parsing function
        let normalized_id = parse_module_id(id_or_name);
        let prefix = format!("{}_", normalized_id);

        fs::read_dir(&modules_dir)
            .ok()?
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name()
                    .to_str()
                    .map(|n| n.starts_with(&prefix))
                    .unwrap_or(false)
            })
            .map(|e| e.path())
    }

    /// Check if a module exists.
    pub fn exists(&self, id: &str) -> bool {
        self.find_module_dir(id).is_some()
    }

    /// Get a module by ID or full name.
    ///
    /// Accepts flexible formats:
    /// - Just the ID: "000", "0", "5"
    /// - Full directory name: "000_ungrouped", "5_dev-tooling"
    pub fn get(&self, id_or_name: &str) -> Result<Module> {
        let path = self
            .find_module_dir(id_or_name)
            .ok_or_else(|| miette!("Module not found: {}", id_or_name))?;

        let id = parse_module_id(id_or_name);
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_prefix(&format!("{}_", id)))
            .unwrap_or("unknown")
            .to_string();

        // Try to load module.yaml for description
        let description = self.load_module_description(&path)?;

        Ok(Module {
            id: id.to_string(), // Use the extracted ID, not the full name
            name,
            description,
            path,
        })
    }

    /// List all modules.
    pub fn list(&self) -> Result<Vec<ModuleSummary>> {
        let modules_dir = self.modules_dir();
        if !modules_dir.is_dir() {
            return Ok(Vec::new());
        }

        let changes_dir = self.ito_path.join("changes");
        let change_counts = self.count_changes_by_module(&changes_dir)?;

        let mut summaries = Vec::new();
        for entry in fs::read_dir(&modules_dir).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            // Parse "NNN_name" format
            let Some((id, name)) = dir_name.split_once('_') else {
                continue;
            };

            let change_count = change_counts.get(id).copied().unwrap_or(0);

            summaries.push(ModuleSummary {
                id: id.to_string(),
                name: name.to_string(),
                change_count,
            });
        }

        // Sort by ID for consistent ordering
        summaries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(summaries)
    }

    /// Load module description from module.yaml if it exists.
    fn load_module_description(&self, module_path: &Path) -> Result<Option<String>> {
        let yaml_path = module_path.join("module.yaml");
        if !yaml_path.is_file() {
            return Ok(None);
        }

        let content = fs::read_to_string(&yaml_path).into_diagnostic()?;

        // Simple YAML parsing for description field
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

    /// Count changes per module.
    fn count_changes_by_module(
        &self,
        changes_dir: &Path,
    ) -> Result<std::collections::HashMap<String, u32>> {
        let mut counts = std::collections::HashMap::new();

        if !changes_dir.is_dir() {
            return Ok(counts);
        }

        for entry in fs::read_dir(changes_dir).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let path = entry.path();
            if !path.is_dir() {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

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
        // Should be sorted by ID
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
}
