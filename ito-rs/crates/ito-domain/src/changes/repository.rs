//! Change Repository - Clean abstraction over change storage.

use chrono::{DateTime, TimeZone, Utc};
use ito_common::match_::nearest_matches;
use miette::{IntoDiagnostic, Result, miette};
use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use super::{
    Change, ChangeStatus, ChangeSummary, Spec, extract_module_id, parse_change_id, parse_module_id,
};
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

/// Deterministic resolution result for a change target input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeTargetResolution {
    /// Exactly one canonical change id matched.
    Unique(String),
    /// Multiple canonical change ids matched the target.
    Ambiguous(Vec<String>),
    /// No changes matched the target.
    NotFound,
}

/// Options for resolving a change target.
#[derive(Debug, Clone, Copy, Default)]
pub struct ResolveTargetOptions {
    /// Include archived changes under `.ito/changes/archive/` as resolver candidates.
    pub include_archived: bool,
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

    /// List change directory names in sorted order.
    fn list_change_dir_names(&self, include_archived: bool) -> Vec<String> {
        let changes_dir = self.changes_dir();
        if !changes_dir.is_dir() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let Ok(entries) = fs::read_dir(&changes_dir) else {
            return out;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name == "archive" {
                continue;
            }
            out.push(name.to_string());
        }

        if include_archived {
            let archive_dir = changes_dir.join("archive");
            let Ok(entries) = fs::read_dir(archive_dir) else {
                out.sort();
                out.dedup();
                return out;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };
                out.push(name.to_string());
            }
        }

        out.sort();
        out.dedup();
        out
    }

    fn split_canonical_change_id<'b>(&self, name: &'b str) -> Option<(String, String, &'b str)> {
        let (module_id, change_num) = parse_change_id(name)?;
        let slug = name.split_once('_').map(|(_id, s)| s).unwrap_or("");
        Some((module_id, change_num, slug))
    }

    fn tokenize_query(&self, input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn normalized_slug_text(&self, slug: &str) -> String {
        let mut out = String::new();
        for ch in slug.chars() {
            if ch.is_ascii_alphanumeric() {
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(' ');
            }
        }
        out
    }

    fn slug_matches_tokens(&self, slug: &str, tokens: &[String]) -> bool {
        if tokens.is_empty() {
            return false;
        }
        let text = self.normalized_slug_text(slug);
        for token in tokens {
            if !text.contains(token) {
                return false;
            }
        }
        true
    }

    fn is_numeric_module_selector(&self, input: &str) -> bool {
        let trimmed = input.trim();
        !trimmed.is_empty() && trimmed.chars().all(|ch| ch.is_ascii_digit())
    }

    fn extract_two_numbers_as_change_id(&self, input: &str) -> Option<(String, String)> {
        let re = Regex::new(r"\d+").ok()?;
        let mut parts: Vec<&str> = Vec::new();
        for m in re.find_iter(input) {
            parts.push(m.as_str());
            if parts.len() > 2 {
                return None;
            }
        }
        if parts.len() != 2 {
            return None;
        }
        let parsed = format!("{}-{}", parts[0], parts[1]);
        parse_change_id(&parsed)
    }

    /// Resolve an input change target into a canonical change id.
    ///
    /// Matching strategy is deterministic:
    /// - exact canonical directory name match first
    /// - canonical numeric-prefix match (`NNN-NN`) after parsing shorthand (e.g. `1-12`)
    /// - free-form two-integer extraction (e.g. `module 1 change 12`)
    /// - generic prefix match on canonical change ids
    pub fn resolve_target(&self, input: &str) -> ChangeTargetResolution {
        self.resolve_target_with_options(input, ResolveTargetOptions::default())
    }

    /// Resolve an input change target into a canonical change id using options.
    pub fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        let names = self.list_change_dir_names(options.include_archived);
        if names.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let input = input.trim();
        if input.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        if names.iter().any(|name| name == input) {
            return ChangeTargetResolution::Unique(input.to_string());
        }

        // 1) Numeric change selector (e.g. 1-12 or 001-12)
        let mut numeric_matches: BTreeSet<String> = BTreeSet::new();
        let numeric_selector =
            parse_change_id(input).or_else(|| self.extract_two_numbers_as_change_id(input));
        if let Some((module_id, change_num)) = numeric_selector {
            let numeric_prefix = format!("{module_id}-{change_num}");
            let with_separator = format!("{numeric_prefix}_");
            for name in &names {
                if name == &numeric_prefix || name.starts_with(&with_separator) {
                    numeric_matches.insert(name.clone());
                }
            }
            if !numeric_matches.is_empty() {
                let numeric_matches: Vec<String> = numeric_matches.into_iter().collect();
                if numeric_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(numeric_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(numeric_matches);
            }
        }

        // 2) Module-scoped slug query (e.g. 1:setup)
        if let Some((module, query)) = input.split_once(':') {
            let query = query.trim();
            if !query.is_empty() {
                let module_id = parse_module_id(module);
                let tokens = self.tokenize_query(query);
                let mut scoped_matches: BTreeSet<String> = BTreeSet::new();
                for name in &names {
                    let Some((name_module, _name_change, slug)) =
                        self.split_canonical_change_id(name)
                    else {
                        continue;
                    };
                    if name_module != module_id {
                        continue;
                    }
                    if self.slug_matches_tokens(slug, &tokens) {
                        scoped_matches.insert(name.clone());
                    }
                }

                if scoped_matches.is_empty() {
                    return ChangeTargetResolution::NotFound;
                }
                let scoped_matches: Vec<String> = scoped_matches.into_iter().collect();
                if scoped_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(scoped_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(scoped_matches);
            }
        }

        // 3) Module-only selector (e.g. 1 or 001)
        if self.is_numeric_module_selector(input) {
            let module_id = parse_module_id(input);
            let mut module_matches: BTreeSet<String> = BTreeSet::new();
            for name in &names {
                let Some((name_module, _name_change, _slug)) = self.split_canonical_change_id(name)
                else {
                    continue;
                };
                if name_module == module_id {
                    module_matches.insert(name.clone());
                }
            }

            if !module_matches.is_empty() {
                let module_matches: Vec<String> = module_matches.into_iter().collect();
                if module_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(module_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(module_matches);
            }
        }

        // 4) Canonical prefix match
        let mut matches: BTreeSet<String> = BTreeSet::new();
        for name in &names {
            if name.starts_with(input) {
                matches.insert(name.clone());
            }
        }

        // 5) Slug tokenized contains match
        if matches.is_empty() {
            let tokens = self.tokenize_query(input);
            for name in &names {
                let Some((_module, _change, slug)) = self.split_canonical_change_id(name) else {
                    continue;
                };
                if self.slug_matches_tokens(slug, &tokens) {
                    matches.insert(name.clone());
                }
            }
        }

        if matches.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let matches: Vec<String> = matches.into_iter().collect();
        if matches.len() == 1 {
            return ChangeTargetResolution::Unique(matches[0].clone());
        }

        ChangeTargetResolution::Ambiguous(matches)
    }

    /// Return best-effort suggestions for a change target.
    pub fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        let input = input.trim().to_lowercase();
        if input.is_empty() || max == 0 {
            return Vec::new();
        }

        let names = self.list_change_dir_names(false);
        let canonical_names: Vec<String> = names
            .iter()
            .filter_map(|name| {
                self.split_canonical_change_id(name)
                    .map(|(_module, _change, _slug)| name.clone())
            })
            .collect();
        let mut scored: Vec<(usize, String)> = Vec::new();
        let tokens = self.tokenize_query(&input);

        for name in &canonical_names {
            let lower = name.to_lowercase();
            let mut score = 0;

            if lower.starts_with(&input) {
                score = score.max(100);
            }
            if lower.contains(&input) {
                score = score.max(80);
            }

            let Some((_module, _change, slug)) = self.split_canonical_change_id(name) else {
                continue;
            };
            if !tokens.is_empty() && self.slug_matches_tokens(slug, &tokens) {
                score = score.max(70);
            }

            if let Some((module_id, change_num)) = parse_change_id(&input) {
                let numeric_prefix = format!("{module_id}-{change_num}");
                if name.starts_with(&numeric_prefix) {
                    score = score.max(95);
                }
            }

            if score > 0 {
                scored.push((score, name.clone()));
            }
        }

        scored.sort_by(|(a_score, a_name), (b_score, b_name)| {
            b_score.cmp(a_score).then_with(|| a_name.cmp(b_name))
        });

        let mut out: Vec<String> = scored
            .into_iter()
            .map(|(_score, name)| name)
            .take(max)
            .collect();

        if out.len() < max {
            let nearest = nearest_matches(&input, &canonical_names, max * 2);
            for candidate in nearest {
                if out.iter().any(|existing| existing == &candidate) {
                    continue;
                }
                out.push(candidate);
                if out.len() == max {
                    break;
                }
            }
        }

        out
    }

    fn resolve_unique_change_id(&self, input: &str) -> Result<String> {
        match self.resolve_target(input) {
            ChangeTargetResolution::Unique(id) => Ok(id),
            ChangeTargetResolution::Ambiguous(matches) => Err(miette!(
                "Ambiguous change target '{input}'. Matches: {}",
                matches.join(", ")
            )),
            ChangeTargetResolution::NotFound => Err(miette!("Change not found: {input}")),
        }
    }

    /// Check if a change exists.
    ///
    /// Accepts flexible ID formats resolved by [`Self::resolve_target`].
    pub fn exists(&self, id: &str) -> bool {
        matches!(self.resolve_target(id), ChangeTargetResolution::Unique(_))
    }

    /// Get a full change with all artifacts loaded.
    ///
    /// Accepts flexible ID formats resolved by [`Self::resolve_target`].
    pub fn get(&self, id: &str) -> Result<Change> {
        let actual_id = self.resolve_unique_change_id(id)?;
        let path = self.changes_dir().join(&actual_id);

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
            if name == "archive" {
                continue;
            }

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
    /// Accepts flexible ID formats resolved by [`Self::resolve_target`].
    pub fn get_summary(&self, id: &str) -> Result<ChangeSummary> {
        let actual_id = self.resolve_unique_change_id(id)?;
        let path = self.changes_dir().join(&actual_id);

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
        let module_id = extract_module_id(&actual_id);

        Ok(ChangeSummary {
            id: actual_id,
            module_id,
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

    fn create_archived_change(ito_path: &Path, id: &str) {
        let archive_dir = ito_path.join("changes").join("archive").join(id);
        fs::create_dir_all(&archive_dir).unwrap();
        fs::write(archive_dir.join("proposal.md"), "# Archived\n").unwrap();
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
    fn test_list_skips_archive_dir() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_first", true);
        create_archived_change(&ito_path, "005-99_old");

        let repo = ChangeRepository::new(&ito_path);
        let changes = repo.list().unwrap();

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].id, "005-01_first");
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
    fn test_resolve_target_unique_partial_and_leading_zero_shorthand() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_first-change", false);

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("1-12"),
            ChangeTargetResolution::Unique("001-12_first-change".to_string())
        );
        assert_eq!(
            repo.resolve_target("001-12_f"),
            ChangeTargetResolution::Unique("001-12_first-change".to_string())
        );
        assert_eq!(
            repo.resolve_target("module 1 change 12"),
            ChangeTargetResolution::Unique("001-12_first-change".to_string())
        );
        assert_eq!(
            repo.resolve_target("change 1.12"),
            ChangeTargetResolution::Unique("001-12_first-change".to_string())
        );
    }

    #[test]
    fn test_resolve_target_ignores_inputs_with_more_than_two_numbers() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_first-change", false);

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("module 1 change 12 revision 3"),
            ChangeTargetResolution::NotFound
        );
    }

    #[test]
    fn test_resolve_target_reports_ambiguity() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_first-change", false);
        create_change(&ito_path, "001-12_follow-up", false);

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("1-12"),
            ChangeTargetResolution::Ambiguous(vec![
                "001-12_first-change".to_string(),
                "001-12_follow-up".to_string(),
            ])
        );
        assert!(!repo.exists("1-12"));
        assert!(repo.get("1-12").is_err());
    }

    #[test]
    fn test_resolve_target_matches_slug_query_tokens() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_setup-wizard", false);
        create_change(&ito_path, "001-13_database-migration", false);

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("setup wizard"),
            ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
        );
        assert_eq!(
            repo.resolve_target("wizard"),
            ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
        );
    }

    #[test]
    fn test_resolve_target_module_scoped_query() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_setup-wizard", false);
        create_change(&ito_path, "002-12_setup-wizard", false);

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("1:setup"),
            ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
        );
        assert_eq!(
            repo.resolve_target("2:setup"),
            ChangeTargetResolution::Unique("002-12_setup-wizard".to_string())
        );
    }

    #[test]
    fn test_resolve_target_module_only_selector() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_setup-wizard", false);
        create_change(&ito_path, "002-12_setup-wizard", false);
        create_change(&ito_path, "002-13_follow-up", false);

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("1"),
            ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
        );
        assert_eq!(
            repo.resolve_target("002"),
            ChangeTargetResolution::Ambiguous(vec![
                "002-12_setup-wizard".to_string(),
                "002-13_follow-up".to_string(),
            ])
        );
    }

    #[test]
    fn test_resolve_target_excludes_archive_by_default() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_archived_change(&ito_path, "001-12_setup-wizard");

        let repo = ChangeRepository::new(&ito_path);

        assert_eq!(
            repo.resolve_target("1-12"),
            ChangeTargetResolution::NotFound
        );

        assert_eq!(
            repo.resolve_target_with_options(
                "1-12",
                ResolveTargetOptions {
                    include_archived: true,
                }
            ),
            ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
        );
    }

    #[test]
    fn test_suggest_targets_returns_nearest_matches() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_setup-wizard", false);
        create_change(&ito_path, "001-13_setup-service", false);
        create_change(&ito_path, "002-01_other-work", false);

        let repo = ChangeRepository::new(&ito_path);
        let suggestions = repo.suggest_targets("setup", 2);
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0], "001-12_setup-wizard");
        assert_eq!(suggestions[1], "001-13_setup-service");
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
