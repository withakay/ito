//! Filesystem-backed implementation of the domain change repository port.

use chrono::{DateTime, TimeZone, Utc};
use ito_common::fs::{FileSystem, StdFs};
use ito_common::match_::nearest_matches;
use ito_common::paths;
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository as DomainChangeRepository, ChangeStatus,
    ChangeSummary, ChangeTargetResolution, ResolveTargetOptions, Spec, extract_module_id,
    extract_sub_module_id, parse_change_id, parse_module_id,
};
use ito_domain::discovery;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::TaskRepository as DomainTaskRepository;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::front_matter;
use crate::task_repository::FsTaskRepository;

/// Filesystem-backed change repository.
pub struct FsChangeRepository<'a, F: FileSystem = StdFs> {
    ito_path: &'a Path,
    task_repo: FsTaskRepository<'a>,
    fs: F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChangeLifecycle {
    Active,
    Archived,
}

#[derive(Debug, Clone)]
struct ChangeLocation {
    id: String,
    path: PathBuf,
    lifecycle: ChangeLifecycle,
}

impl<'a> FsChangeRepository<'a, StdFs> {
    /// Create a repository backed by the real filesystem.
    pub fn new(ito_path: &'a Path) -> Self {
        Self::with_fs(ito_path, StdFs)
    }
}

impl<'a, F: FileSystem> FsChangeRepository<'a, F> {
    /// Create a repository with an explicit filesystem implementation.
    pub fn with_fs(ito_path: &'a Path, fs: F) -> Self {
        Self::with_task_repo(ito_path, FsTaskRepository::new(ito_path), fs)
    }

    /// Create a repository with an injected task repository.
    ///
    /// Use this when you need to control the task repository instance
    /// (e.g., in tests or when sharing a repo across multiple consumers).
    pub fn with_task_repo(ito_path: &'a Path, task_repo: FsTaskRepository<'a>, fs: F) -> Self {
        Self {
            ito_path,
            task_repo,
            fs,
        }
    }

    /// Resolve an input change target into a canonical change id.
    pub fn resolve_target(&self, input: &str) -> ChangeTargetResolution {
        DomainChangeRepository::resolve_target(self, input)
    }

    /// Resolve an input change target into a canonical change id using options.
    pub fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        DomainChangeRepository::resolve_target_with_options(self, input, options)
    }

    /// Return best-effort suggestions for a change target.
    pub fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        DomainChangeRepository::suggest_targets(self, input, max)
    }

    /// Check if a change exists.
    pub fn exists(&self, id: &str) -> bool {
        DomainChangeRepository::exists(self, id)
    }

    /// Get a full change with all artifacts loaded.
    pub fn get(&self, id: &str) -> DomainResult<Change> {
        DomainChangeRepository::get(self, id)
    }

    /// List all changes as summaries (lightweight).
    pub fn list(&self) -> DomainResult<Vec<ChangeSummary>> {
        DomainChangeRepository::list(self)
    }

    /// List changes belonging to a specific module.
    pub fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>> {
        DomainChangeRepository::list_by_module(self, module_id)
    }

    /// List changes with incomplete tasks.
    pub fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>> {
        DomainChangeRepository::list_incomplete(self)
    }

    /// List changes with all tasks complete.
    pub fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>> {
        DomainChangeRepository::list_complete(self)
    }

    /// Get a summary for a specific change (lightweight).
    pub fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        DomainChangeRepository::get_summary(self, id)
    }

    fn changes_dir(&self) -> std::path::PathBuf {
        paths::changes_dir(self.ito_path)
    }

    fn list_change_locations(&self, filter: ChangeLifecycleFilter) -> Vec<ChangeLocation> {
        let mut active = Vec::new();
        if filter.includes_active() {
            active = self.list_active_locations();
        }

        let mut archived = Vec::new();
        if filter.includes_archived() {
            archived = self.list_archived_locations();
        }

        match filter {
            ChangeLifecycleFilter::Active => active,
            ChangeLifecycleFilter::Archived => archived,
            ChangeLifecycleFilter::All => {
                let mut merged: BTreeMap<String, ChangeLocation> = BTreeMap::new();
                for loc in active {
                    merged.insert(loc.id.clone(), loc);
                }
                for loc in archived {
                    merged.entry(loc.id.clone()).or_insert(loc);
                }
                merged.into_values().collect()
            }
        }
    }

    fn list_active_locations(&self) -> Vec<ChangeLocation> {
        let mut out = Vec::new();
        for name in discovery::list_change_dir_names(&self.fs, self.ito_path).unwrap_or_default() {
            let path = self.changes_dir().join(&name);
            out.push(ChangeLocation {
                id: name,
                path,
                lifecycle: ChangeLifecycle::Active,
            });
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }

    fn list_archived_locations(&self) -> Vec<ChangeLocation> {
        let archive_dir = self.changes_dir().join("archive");
        let mut by_id: BTreeMap<String, String> = BTreeMap::new();
        let archived = discovery::list_dir_names(&self.fs, &archive_dir).unwrap_or_default();

        for name in archived {
            let Some(change_id) = self.parse_archived_change_id(&name) else {
                continue;
            };
            let entry = by_id.entry(change_id).or_insert(name.clone());
            if name > *entry {
                *entry = name;
            }
        }

        let mut out = Vec::new();
        for (id, dir_name) in by_id {
            out.push(ChangeLocation {
                id,
                path: archive_dir.join(dir_name),
                lifecycle: ChangeLifecycle::Archived,
            });
        }
        out
    }

    fn parse_archived_change_id(&self, name: &str) -> Option<String> {
        if let Some(remainder) = self.strip_archive_date_prefix(name)
            && parse_change_id(remainder).is_some()
        {
            return Some(remainder.to_string());
        }

        if parse_change_id(name).is_some() {
            return Some(name.to_string());
        }

        None
    }

    fn strip_archive_date_prefix<'b>(&self, name: &'b str) -> Option<&'b str> {
        let mut parts = name.splitn(4, '-');
        let year = parts.next()?;
        let month = parts.next()?;
        let day = parts.next()?;
        let remainder = parts.next()?;

        if year.len() != 4 || month.len() != 2 || day.len() != 2 {
            return None;
        }
        if !year.chars().all(|c| c.is_ascii_digit())
            || !month.chars().all(|c| c.is_ascii_digit())
            || !day.chars().all(|c| c.is_ascii_digit())
        {
            return None;
        }
        if remainder.trim().is_empty() {
            return None;
        }
        Some(remainder)
    }

    fn list_change_ids(&self, filter: ChangeLifecycleFilter) -> Vec<String> {
        let mut out: Vec<String> = self
            .list_change_locations(filter)
            .into_iter()
            .map(|loc| loc.id)
            .collect();
        out.sort();
        out.dedup();
        out
    }

    fn find_change_location(
        &self,
        id: &str,
        filter: ChangeLifecycleFilter,
    ) -> Option<ChangeLocation> {
        self.list_change_locations(filter)
            .into_iter()
            .find(|loc| loc.id == id)
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

    fn resolve_unique_change_id(
        &self,
        input: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<String> {
        match self.resolve_target_with_options(input, ResolveTargetOptions { lifecycle: filter }) {
            ChangeTargetResolution::Unique(id) => Ok(id),
            ChangeTargetResolution::Ambiguous(matches) => {
                Err(DomainError::ambiguous_target("change", input, &matches))
            }
            ChangeTargetResolution::NotFound => Err(DomainError::not_found("change", input)),
        }
    }

    fn resolve_unique_change_location(
        &self,
        input: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<ChangeLocation> {
        let id = self.resolve_unique_change_id(input, filter)?;
        self.find_change_location(&id, filter)
            .ok_or_else(|| DomainError::not_found("change", input))
    }

    fn read_optional_file(&self, path: &Path) -> DomainResult<Option<String>> {
        if self.fs.is_file(path) {
            let content = self
                .fs
                .read_to_string(path)
                .map_err(|source| DomainError::io("reading optional file", source))?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    fn load_specs(&self, change_path: &Path) -> DomainResult<Vec<Spec>> {
        let specs_dir = change_path.join("specs");
        if !self.fs.is_dir(&specs_dir) {
            return Ok(Vec::new());
        }

        let mut specs = Vec::new();
        for name in discovery::list_dir_names(&self.fs, &specs_dir)? {
            let spec_file = specs_dir.join(&name).join("spec.md");
            if self.fs.is_file(&spec_file) {
                let content = self
                    .fs
                    .read_to_string(&spec_file)
                    .map_err(|source| DomainError::io("reading spec file", source))?;
                specs.push(Spec { name, content });
            }
        }

        specs.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(specs)
    }

    fn has_specs(&self, change_path: &Path) -> bool {
        let specs_dir = change_path.join("specs");
        if !self.fs.is_dir(&specs_dir) {
            return false;
        }

        discovery::list_dir_names(&self.fs, &specs_dir)
            .map(|entries| {
                entries
                    .into_iter()
                    .any(|name| self.fs.is_file(&specs_dir.join(name).join("spec.md")))
            })
            .unwrap_or(false)
    }

    /// Validate front matter identifiers in a change artifact, if present.
    ///
    /// Parses front matter from the content and checks that any declared
    /// `change_id` matches the expected change directory name. Integrity
    /// checksums are also validated when present. If the content has no
    /// front matter, this is a no-op.
    fn validate_artifact_front_matter(
        &self,
        content: &str,
        expected_change_id: &str,
    ) -> DomainResult<()> {
        let parsed = front_matter::parse(content).map_err(|e| {
            DomainError::io(
                "parsing front matter",
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()),
            )
        })?;

        let Some(fm) = &parsed.front_matter else {
            return Ok(());
        };

        // Validate change_id if declared in front matter
        front_matter::validate_id("change_id", fm.change_id.as_deref(), expected_change_id)
            .map_err(|e| {
                DomainError::io(
                    "front matter validation",
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()),
                )
            })?;

        // Validate body integrity checksum if present
        front_matter::validate_integrity(fm, &parsed.body).map_err(|e| {
            DomainError::io(
                "front matter integrity",
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()),
            )
        })?;

        Ok(())
    }

    fn get_last_modified(&self, change_path: &Path) -> DomainResult<DateTime<Utc>> {
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

    fn load_tasks_for_location(
        &self,
        location: &ChangeLocation,
    ) -> DomainResult<ito_domain::tasks::TasksParseResult> {
        match location.lifecycle {
            ChangeLifecycle::Active => self.task_repo.load_tasks(&location.id),
            ChangeLifecycle::Archived => self.task_repo.load_tasks_from_dir(&location.path),
        }
    }

    fn build_summary_for_location(&self, location: &ChangeLocation) -> DomainResult<ChangeSummary> {
        let tasks = self.load_tasks_for_location(location)?;
        let progress = tasks.progress;
        let completed_tasks = progress.complete as u32;
        let shelved_tasks = progress.shelved as u32;
        let in_progress_tasks = progress.in_progress as u32;
        let pending_tasks = progress.pending as u32;
        let total_tasks = progress.total as u32;
        let last_modified = self.get_last_modified(&location.path)?;

        let has_proposal = self.fs.is_file(&location.path.join("proposal.md"));
        let has_design = self.fs.is_file(&location.path.join("design.md"));
        let has_specs = self.has_specs(&location.path);
        let has_tasks = total_tasks > 0;
        let module_id = extract_module_id(&location.id);
        let sub_module_id = extract_sub_module_id(&location.id);
        let meta = crate::change_meta::read_change_meta_from_dir(&self.fs, &location.path);

        Ok(ChangeSummary {
            id: location.id.clone(),
            module_id,
            sub_module_id,
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
            orchestrate: meta.orchestrate,
        })
    }
}

impl<'a, F: FileSystem> DomainChangeRepository for FsChangeRepository<'a, F> {
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        let names = self.list_change_ids(options.lifecycle);
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

        let mut matches: BTreeSet<String> = BTreeSet::new();
        for name in &names {
            if name.starts_with(input) {
                matches.insert(name.clone());
            }
        }

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

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        let input = input.trim().to_lowercase();
        if input.is_empty() || max == 0 {
            return Vec::new();
        }

        let names = self.list_change_ids(ChangeLifecycleFilter::Active);
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

    fn exists(&self, id: &str) -> bool {
        self.exists_with_filter(id, ChangeLifecycleFilter::Active)
    }

    fn exists_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> bool {
        let resolution =
            self.resolve_target_with_options(id, ResolveTargetOptions { lifecycle: filter });
        match resolution {
            ChangeTargetResolution::Unique(_) => true,
            ChangeTargetResolution::Ambiguous(_) => false,
            ChangeTargetResolution::NotFound => false,
        }
    }

    fn get_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        let location = self.resolve_unique_change_location(id, filter)?;
        let actual_id = location.id.clone();

        let proposal = self.read_optional_file(&location.path.join("proposal.md"))?;
        let design = self.read_optional_file(&location.path.join("design.md"))?;

        // Validate front matter identifiers in artifacts (non-blocking for
        // files without front matter).
        if let Some(content) = &proposal {
            self.validate_artifact_front_matter(content, &actual_id)?;
        }
        if let Some(content) = &design {
            self.validate_artifact_front_matter(content, &actual_id)?;
        }

        let specs = self.load_specs(&location.path)?;
        let tasks = self.load_tasks_for_location(&location)?;
        let last_modified = self.get_last_modified(&location.path)?;
        let path = location.path;
        let meta = crate::change_meta::read_change_meta_from_dir(&self.fs, &path);

        let sub_module_id = extract_sub_module_id(&actual_id);

        Ok(Change {
            id: actual_id.clone(),
            module_id: extract_module_id(&actual_id),
            sub_module_id,
            path,
            proposal,
            design,
            specs,
            tasks,
            orchestrate: meta.orchestrate,
            last_modified,
        })
    }

    fn list_with_filter(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        let mut summaries = Vec::new();
        for location in self.list_change_locations(filter) {
            summaries.push(self.build_summary_for_location(&location)?);
        }

        summaries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(summaries)
    }

    fn list_by_module_with_filter(
        &self,
        module_id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let normalized_id = parse_module_id(module_id);
        let all = self.list_with_filter(filter)?;
        Ok(all
            .into_iter()
            .filter(|c| c.module_id.as_deref() == Some(&normalized_id))
            .collect())
    }

    fn list_incomplete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.list_with_filter(filter)?;
        Ok(all
            .into_iter()
            .filter(|c| c.status() == ChangeStatus::InProgress)
            .collect())
    }

    fn list_complete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.list_with_filter(filter)?;
        Ok(all
            .into_iter()
            .filter(|c| c.status() == ChangeStatus::Complete)
            .collect())
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<ChangeSummary> {
        let location = self.resolve_unique_change_location(id, filter)?;
        self.build_summary_for_location(&location)
    }
}

/// Backward-compatible alias for the default filesystem-backed repository.
pub type ChangeRepository<'a> = FsChangeRepository<'a, StdFs>;

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
    fn exists_and_get_work() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_test", true);

        let repo = FsChangeRepository::new(&ito_path);
        assert!(repo.exists("005-01_test"));
        assert!(!repo.exists("999-99_missing"));

        let change = repo.get("005-01_test").unwrap();
        assert_eq!(change.id, "005-01_test");
        assert_eq!(change.task_progress(), (1, 2));
        assert!(change.proposal.is_some());
        assert!(change.design.is_some());
        assert_eq!(change.specs.len(), 1);
    }

    #[test]
    fn list_skips_archive_dir() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "005-01_first", true);
        create_archived_change(&ito_path, "005-99_old");

        let repo = FsChangeRepository::new(&ito_path);
        let changes = repo.list().unwrap();

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].id, "005-01_first");
    }

    #[test]
    fn resolve_target_reports_ambiguity() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_first-change", false);
        create_change(&ito_path, "001-12_follow-up", false);

        let repo = FsChangeRepository::new(&ito_path);
        assert_eq!(
            repo.resolve_target("1-12"),
            ChangeTargetResolution::Ambiguous(vec![
                "001-12_first-change".to_string(),
                "001-12_follow-up".to_string(),
            ])
        );
    }

    #[test]
    fn resolve_target_module_scoped_query() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_setup-wizard", false);
        create_change(&ito_path, "002-12_setup-wizard", false);

        let repo = FsChangeRepository::new(&ito_path);
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
    fn resolve_target_includes_archive_when_requested() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_archived_change(&ito_path, "001-12_setup-wizard");

        let repo = FsChangeRepository::new(&ito_path);
        assert_eq!(
            repo.resolve_target("1-12"),
            ChangeTargetResolution::NotFound
        );

        assert_eq!(
            repo.resolve_target_with_options(
                "1-12",
                ResolveTargetOptions {
                    lifecycle: ChangeLifecycleFilter::All,
                }
            ),
            ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
        );
    }

    #[test]
    fn suggest_targets_prioritizes_slug_matches() {
        let tmp = TempDir::new().unwrap();
        let ito_path = setup_test_ito(&tmp);
        create_change(&ito_path, "001-12_setup-wizard", false);
        create_change(&ito_path, "001-13_setup-service", false);
        create_change(&ito_path, "002-01_other-work", false);

        let repo = FsChangeRepository::new(&ito_path);
        let suggestions = repo.suggest_targets("setup", 2);
        assert_eq!(
            suggestions,
            vec!["001-12_setup-wizard", "001-13_setup-service"]
        );
    }
}
