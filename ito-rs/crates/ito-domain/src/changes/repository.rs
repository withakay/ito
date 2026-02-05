//! Change Repository - Clean abstraction over change storage.

use chrono::{DateTime, TimeZone, Utc};
use miette::{IntoDiagnostic, Result, miette};
use std::fs;
use std::path::Path;

use super::{Change, ChangeStatus, ChangeSummary, Spec, extract_module_id, parse_change_id};
use crate::tasks::TaskRepository;

/// Repository for accessing change data.
///
/// This abstraction hides the file system storage format from consumers.
/// All change queries should go through this interface rather than
/// directly reading files.
pub struct ChangeRepository<'a> {
    ito_path: &'a Path,
    task_repo: TaskRepository<'a>,
}

impl<'a> ChangeRepository<'a> {
    /// Create a new change repository for the given ito directory.
    pub fn new(ito_path: &'a Path) -> Self {
        Self {
            ito_path,
            task_repo: TaskRepository::new(ito_path),
        }
    }

    /// Get the path to the changes directory.
    fn changes_dir(&self) -> std::path::PathBuf {
        self.ito_path.join("changes")
    }

    /// Find a change directory by flexible ID matching.
    ///
    /// Accepts various formats:
    /// - Exact directory name: `005-01_my-change`
    /// - Shortened: `5-1_my-change`, `5-1`
    /// - Numeric only: `005-01`, `5-1`
    fn find_change_dir(&self, input: &str) -> Option<(std::path::PathBuf, String)> {
        let changes_dir = self.changes_dir();
        if !changes_dir.is_dir() {
            return None;
        }

        // First try exact match
        let exact_path = changes_dir.join(input);
        if exact_path.is_dir() {
            return Some((exact_path, input.to_string()));
        }

        // Parse the input to get normalized module and change numbers
        let (module_id, change_num) = parse_change_id(input)?;

        // Look for a directory starting with "NNN-NN_"
        let prefix = format!("{}-{}_", module_id, change_num);

        fs::read_dir(&changes_dir)
            .ok()?
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name()
                    .to_str()
                    .map(|n| n.starts_with(&prefix))
                    .unwrap_or(false)
            })
            .map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                (e.path(), name)
            })
    }

    /// Check if a change exists.
    ///
    /// Accepts flexible ID formats (see `find_change_dir`).
    pub fn exists(&self, id: &str) -> bool {
        self.find_change_dir(id).is_some()
    }

    /// Get a full change with all artifacts loaded.
    ///
    /// Accepts flexible ID formats (see `find_change_dir`).
    pub fn get(&self, id: &str) -> Result<Change> {
        let (path, actual_id) = self
            .find_change_dir(id)
            .ok_or_else(|| miette!("Change not found: {}", id))?;

        let proposal = self.read_optional_file(&path.join("proposal.md"))?;
        let design = self.read_optional_file(&path.join("design.md"))?;
        let specs = self.load_specs(&path)?;
        let tasks = self.task_repo.load_tasks(&actual_id)?;
        let last_modified = self.get_last_modified(&path)?;

        Ok(Change {
            id: actual_id.clone(),
            module_id: extract_module_id(&actual_id),
            path,
            proposal,
            design,
            specs,
            tasks,
            last_modified,
        })
    }

    /// List all changes as summaries (lightweight).
    pub fn list(&self) -> Result<Vec<ChangeSummary>> {
        let changes_dir = self.changes_dir();
        if !changes_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut summaries = Vec::new();
        for entry in fs::read_dir(&changes_dir).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            let summary = self.get_summary(name)?;
            summaries.push(summary);
        }

        // Sort by ID for consistent ordering
        summaries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(summaries)
    }

    /// List changes belonging to a specific module.
    ///
    /// Accepts flexible module ID formats (e.g., "5", "005", "005_dev-tooling").
    pub fn list_by_module(&self, module_id: &str) -> Result<Vec<ChangeSummary>> {
        let normalized_id = super::parse_module_id(module_id);
        let all = self.list()?;
        Ok(all
            .into_iter()
            .filter(|c| c.module_id.as_deref() == Some(&normalized_id))
            .collect())
    }

    /// List changes with incomplete tasks.
    pub fn list_incomplete(&self) -> Result<Vec<ChangeSummary>> {
        let all = self.list()?;
        Ok(all
            .into_iter()
            .filter(|c| c.status() == ChangeStatus::InProgress)
            .collect())
    }

    /// List changes with all tasks complete.
    pub fn list_complete(&self) -> Result<Vec<ChangeSummary>> {
        let all = self.list()?;
        Ok(all
            .into_iter()
            .filter(|c| c.status() == ChangeStatus::Complete)
            .collect())
    }

    /// Get a summary for a specific change (lightweight).
    ///
    /// Accepts flexible ID formats (see `find_change_dir`).
    pub fn get_summary(&self, id: &str) -> Result<ChangeSummary> {
        let (path, actual_id) = self
            .find_change_dir(id)
            .ok_or_else(|| miette!("Change not found: {}", id))?;

        let progress = self.task_repo.get_progress(&actual_id)?;
        let completed_tasks = progress.complete as u32;
        let shelved_tasks = progress.shelved as u32;
        let in_progress_tasks = progress.in_progress as u32;
        let pending_tasks = progress.pending as u32;
        let total_tasks = progress.total as u32;
        let last_modified = self.get_last_modified(&path)?;

        let has_proposal = path.join("proposal.md").is_file();
        let has_design = path.join("design.md").is_file();
        let has_specs = self.has_specs(&path);
        let has_tasks = total_tasks > 0;

        Ok(ChangeSummary {
            id: actual_id,
            module_id: extract_module_id(id),
            completed_tasks,
            shelved_tasks,
            in_progress_tasks,
            pending_tasks,
            total_tasks,
            last_modified,
            has_proposal,
            has_design,
            has_specs,
            has_tasks,
        })
    }

    /// Read an optional file, returning None if it doesn't exist.
    fn read_optional_file(&self, path: &Path) -> Result<Option<String>> {
        if path.is_file() {
            let content = fs::read_to_string(path).into_diagnostic()?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    /// Load specs from the specs/ directory.
    fn load_specs(&self, change_path: &Path) -> Result<Vec<Spec>> {
        let specs_dir = change_path.join("specs");
        if !specs_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut specs = Vec::new();
        for entry in fs::read_dir(&specs_dir).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            let spec_file = path.join("spec.md");
            if spec_file.is_file() {
                let content = fs::read_to_string(&spec_file).into_diagnostic()?;
                specs.push(Spec {
                    name: name.to_string(),
                    content,
                });
            }
        }

        specs.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(specs)
    }

    /// Check if the specs/ directory has any specs.
    fn has_specs(&self, change_path: &Path) -> bool {
        let specs_dir = change_path.join("specs");
        if !specs_dir.is_dir() {
            return false;
        }

        fs::read_dir(&specs_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().join("spec.md").is_file())
            })
            .unwrap_or(false)
    }

    /// Get the last modified time of a change (most recent file modification).
    fn get_last_modified(&self, change_path: &Path) -> Result<DateTime<Utc>> {
        let mut latest = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        for entry in walkdir::WalkDir::new(change_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Ok(metadata) = entry.metadata()
                && let Ok(modified) = metadata.modified()
            {
                let dt: DateTime<Utc> = modified.into();
                if dt > latest {
                    latest = dt;
                }
            }
        }

        Ok(latest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_ito(tmp: &TempDir) -> std::path::PathBuf {
        let ito_path = tmp.path().join(".ito");
        fs::create_dir_all(ito_path.join("changes")).unwrap();
        ito_path
    }

    fn create_change(ito_path: &Path, id: &str, with_tasks: bool) {
        let change_dir = ito_path.join("changes").join(id);
        fs::create_dir_all(&change_dir).unwrap();
        fs::write(change_dir.join("proposal.md"), "# Proposal\n").unwrap();
        fs::write(change_dir.join("design.md"), "# Design\n").unwrap();

        let specs_dir = change_dir.join("specs").join("test-spec");
        fs::create_dir_all(&specs_dir).unwrap();
        fs::write(specs_dir.join("spec.md"), "## Requirements\n").unwrap();

        if with_tasks {
            fs::write(
                change_dir.join("tasks.md"),
                "# Tasks\n- [x] Task 1\n- [ ] Task 2\n",
            )
            .unwrap();
        }
    }

    #[test]
    fn test_exists() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_test", false);

        let repo = ChangeRepository::new(&ito_path);
        assert!(repo.exists("005-01_test"));
        assert!(!repo.exists("999-99_nonexistent"));
    }

    #[test]
    fn test_get() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_test", true);

        let repo = ChangeRepository::new(&ito_path);
        let change = repo.get("005-01_test").unwrap();

        assert_eq!(change.id, "005-01_test");
        assert_eq!(change.module_id, Some("005".to_string()));
        assert!(change.proposal.is_some());
        assert!(change.design.is_some());
        assert_eq!(change.specs.len(), 1);
        assert_eq!(change.task_progress(), (1, 2));
    }

    #[test]
    fn test_get_not_found() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);

        let repo = ChangeRepository::new(&ito_path);
        let result = repo.get("999-99_nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_first", true);
        create_change(&ito_path, "005-02_second", false);
        create_change(&ito_path, "003-01_other", true);

        let repo = ChangeRepository::new(&ito_path);
        let changes = repo.list().unwrap();

        assert_eq!(changes.len(), 3);
        // Should be sorted by ID
        assert_eq!(changes[0].id, "003-01_other");
        assert_eq!(changes[1].id, "005-01_first");
        assert_eq!(changes[2].id, "005-02_second");
    }

    #[test]
    fn test_list_by_module() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_first", true);
        create_change(&ito_path, "005-02_second", false);
        create_change(&ito_path, "003-01_other", true);

        let repo = ChangeRepository::new(&ito_path);
        let changes = repo.list_by_module("005").unwrap();

        assert_eq!(changes.len(), 2);
        assert!(
            changes
                .iter()
                .all(|c| c.module_id == Some("005".to_string()))
        );
    }

    #[test]
    fn test_list_incomplete() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_incomplete", true); // 1/2 tasks
        create_change(&ito_path, "005-02_no_tasks", false);

        // Create a complete change
        let complete_dir = ito_path.join("changes").join("005-03_complete");
        fs::create_dir_all(&complete_dir).unwrap();
        fs::write(
            complete_dir.join("tasks.md"),
            "# Tasks\n- [x] Done\n- [x] Also done\n",
        )
        .unwrap();

        let repo = ChangeRepository::new(&ito_path);
        let incomplete = repo.list_incomplete().unwrap();

        assert_eq!(incomplete.len(), 1);
        assert_eq!(incomplete[0].id, "005-01_incomplete");
    }

    #[test]
    fn test_change_status() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_test", true);

        let repo = ChangeRepository::new(&ito_path);
        let change = repo.get("005-01_test").unwrap();

        assert_eq!(change.status(), ChangeStatus::InProgress);
    }

    #[test]
    fn test_flexible_id_exists() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_my-change", false);

        let repo = ChangeRepository::new(&ito_path);

        // Exact match
        assert!(repo.exists("005-01_my-change"));

        // Shortened formats
        assert!(repo.exists("5-1_my-change"));
        assert!(repo.exists("005-01"));
        assert!(repo.exists("5-1"));

        // Non-existent
        assert!(!repo.exists("005-02"));
        assert!(!repo.exists("5-2"));
    }

    #[test]
    fn test_flexible_id_get() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_my-change", true);

        let repo = ChangeRepository::new(&ito_path);

        // All these should return the same change
        let change1 = repo.get("005-01_my-change").unwrap();
        let change2 = repo.get("5-1_my-change").unwrap();
        let change3 = repo.get("005-01").unwrap();
        let change4 = repo.get("5-1").unwrap();

        assert_eq!(change1.id, "005-01_my-change");
        assert_eq!(change2.id, "005-01_my-change");
        assert_eq!(change3.id, "005-01_my-change");
        assert_eq!(change4.id, "005-01_my-change");
    }

    #[test]
    fn test_flexible_module_id_list_by_module() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_first", true);
        create_change(&ito_path, "005-02_second", false);
        create_change(&ito_path, "003-01_other", true);

        let repo = ChangeRepository::new(&ito_path);

        // All these should return the same 2 changes
        let changes1 = repo.list_by_module("005").unwrap();
        let changes2 = repo.list_by_module("5").unwrap();
        let changes3 = repo.list_by_module("005_dev-tooling").unwrap();

        assert_eq!(changes1.len(), 2);
        assert_eq!(changes2.len(), 2);
        assert_eq!(changes3.len(), 2);
    }
}
