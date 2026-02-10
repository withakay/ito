//! Listing helpers for modules, changes, and specs.
//!
//! These functions are used by the CLI to produce stable, JSON-friendly
//! summaries of on-disk Ito state.

use std::path::{Path, PathBuf};

use chrono::{DateTime, SecondsFormat, Timelike, Utc};

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use ito_common::fs::StdFs;
use ito_common::paths;
use ito_domain::changes::{
    ChangeRepository as DomainChangeRepository, ChangeStatus, ChangeSummary,
};
use ito_domain::modules::ModuleRepository as DomainModuleRepository;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
/// Module entry returned by `ito list modules`.
pub struct ModuleListItem {
    /// 3-digit module id.
    pub id: String,
    /// Module name (slug).
    pub name: String,
    #[serde(rename = "fullName")]
    /// Folder name (`NNN_name`).
    pub full_name: String,
    #[serde(rename = "changeCount")]
    /// Number of changes currently associated with the module.
    pub change_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
/// Change entry returned by `ito list changes`.
pub struct ChangeListItem {
    /// Change folder name.
    pub name: String,
    #[serde(rename = "completedTasks")]
    /// Number of completed tasks.
    pub completed_tasks: u32,
    #[serde(rename = "shelvedTasks")]
    /// Number of shelved tasks.
    pub shelved_tasks: u32,
    #[serde(rename = "inProgressTasks")]
    /// Number of in-progress tasks.
    pub in_progress_tasks: u32,
    #[serde(rename = "pendingTasks")]
    /// Number of pending tasks.
    pub pending_tasks: u32,
    #[serde(rename = "totalTasks")]
    /// Total number of tasks.
    pub total_tasks: u32,
    #[serde(rename = "lastModified")]
    /// Last modified time for the change directory.
    pub last_modified: String,
    /// Legacy status field for backward compatibility
    pub status: String,
    /// Work status: draft, ready, in-progress, paused, complete
    #[serde(rename = "workStatus")]
    pub work_status: String,
    /// True when no remaining work (complete or paused)
    /// True when no remaining work (complete or paused)
    pub completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Progress filter for the `ito list` default changes path.
pub enum ChangeProgressFilter {
    /// Return all changes.
    All,
    /// Return only ready changes.
    Ready,
    /// Return only completed (including paused) changes.
    Completed,
    /// Return only partially complete changes.
    Partial,
    /// Return only pending changes.
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Sort order for the `ito list` default changes path.
pub enum ChangeSortOrder {
    /// Sort by most-recent first.
    Recent,
    /// Sort by change name.
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Input arguments for the default `ito list` changes use-case.
pub struct ListChangesInput {
    /// Progress filter to apply before sorting.
    pub progress_filter: ChangeProgressFilter,
    /// Sort order applied to filtered changes.
    pub sort: ChangeSortOrder,
}

impl Default for ListChangesInput {
    fn default() -> Self {
        Self {
            progress_filter: ChangeProgressFilter::All,
            sort: ChangeSortOrder::Recent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Stable typed summary returned to adapters for `ito list` changes.
pub struct ChangeListSummary {
    /// Change folder name.
    pub name: String,
    /// Number of completed tasks.
    pub completed_tasks: u32,
    /// Number of shelved tasks.
    pub shelved_tasks: u32,
    /// Number of in-progress tasks.
    pub in_progress_tasks: u32,
    /// Number of pending tasks.
    pub pending_tasks: u32,
    /// Total number of tasks.
    pub total_tasks: u32,
    /// Last modified time for the change directory.
    pub last_modified: DateTime<Utc>,
    /// Legacy status field for backward compatibility.
    pub status: String,
    /// Work status: draft, ready, in-progress, paused, complete.
    pub work_status: String,
    /// True when no remaining work (complete or paused).
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
/// Spec entry returned by `ito list specs`.
pub struct SpecListItem {
    /// Spec id.
    pub id: String,
    #[serde(rename = "requirementCount")]
    /// Count of requirements in `spec.md`.
    pub requirement_count: u32,
}

/// List modules under `{ito_path}/modules`.
pub fn list_modules(module_repo: &impl DomainModuleRepository) -> CoreResult<Vec<ModuleListItem>> {
    let mut modules: Vec<ModuleListItem> = Vec::new();

    for module in module_repo.list().into_core()? {
        let full_name = format!("{}_{}", module.id, module.name);
        modules.push(ModuleListItem {
            id: module.id,
            name: module.name,
            full_name,
            change_count: module.change_count as usize,
        });
    }

    modules.sort_by(|a, b| a.full_name.cmp(&b.full_name));
    Ok(modules)
}

/// List change directories under `{ito_path}/changes`.
pub fn list_change_dirs(ito_path: &Path) -> CoreResult<Vec<PathBuf>> {
    let fs = StdFs;
    Ok(ito_domain::discovery::list_change_dir_names(&fs, ito_path)
        .into_core()?
        .into_iter()
        .map(|name| paths::change_dir(ito_path, &name))
        .collect())
}

/// List active changes using typed summaries for adapter rendering.
pub fn list_changes(
    change_repo: &impl DomainChangeRepository,
    input: ListChangesInput,
) -> CoreResult<Vec<ChangeListSummary>> {
    let mut summaries: Vec<ChangeSummary> = change_repo.list().into_core()?;

    match input.progress_filter {
        ChangeProgressFilter::All => {}
        ChangeProgressFilter::Ready => summaries.retain(|s| s.is_ready()),
        ChangeProgressFilter::Completed => summaries.retain(is_completed),
        ChangeProgressFilter::Partial => summaries.retain(is_partial),
        ChangeProgressFilter::Pending => summaries.retain(is_pending),
    }

    match input.sort {
        ChangeSortOrder::Name => summaries.sort_by(|a, b| a.id.cmp(&b.id)),
        ChangeSortOrder::Recent => summaries.sort_by(|a, b| b.last_modified.cmp(&a.last_modified)),
    }

    Ok(summaries
        .into_iter()
        .map(|s| {
            let status = match s.status() {
                ChangeStatus::NoTasks => "no-tasks",
                ChangeStatus::InProgress => "in-progress",
                ChangeStatus::Complete => "complete",
            };
            ChangeListSummary {
                name: s.id.clone(),
                completed_tasks: s.completed_tasks,
                shelved_tasks: s.shelved_tasks,
                in_progress_tasks: s.in_progress_tasks,
                pending_tasks: s.pending_tasks,
                total_tasks: s.total_tasks,
                last_modified: s.last_modified,
                status: status.to_string(),
                work_status: s.work_status().to_string(),
                completed: is_completed(&s),
            }
        })
        .collect())
}

/// Compute the most-recent modification time under `path`.
pub fn last_modified_recursive(path: &Path) -> CoreResult<DateTime<Utc>> {
    use std::collections::VecDeque;

    let mut max = std::fs::metadata(path)
        .map_err(|e| CoreError::io("reading metadata", e))?
        .modified()
        .map_err(|e| CoreError::io("getting modification time", std::io::Error::other(e)))?;

    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(path.to_path_buf());

    while let Some(p) = queue.pop_front() {
        let meta = match std::fs::symlink_metadata(&p) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if let Ok(m) = meta.modified()
            && m > max
        {
            max = m;
        }
        if meta.is_dir() {
            let iter = match std::fs::read_dir(&p) {
                Ok(i) => i,
                Err(_) => continue,
            };
            for entry in iter {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                queue.push_back(entry.path());
            }
        }
    }

    let dt: DateTime<Utc> = max.into();
    Ok(dt)
}

/// Render a UTC timestamp as ISO-8601 with millisecond precision.
pub fn to_iso_millis(dt: DateTime<Utc>) -> String {
    // JS Date.toISOString() is millisecond-precision. Truncate to millis to avoid
    // platform-specific sub-ms differences.
    let nanos = dt.timestamp_subsec_nanos();
    let truncated = dt
        .with_nanosecond((nanos / 1_000_000) * 1_000_000)
        .unwrap_or(dt);
    truncated.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// List specs under `{ito_path}/specs`.
pub fn list_specs(ito_path: &Path) -> CoreResult<Vec<SpecListItem>> {
    let mut specs: Vec<SpecListItem> = Vec::new();
    let specs_dir = paths::specs_dir(ito_path);
    let fs = StdFs;
    for id in ito_domain::discovery::list_spec_dir_names(&fs, ito_path).into_core()? {
        let spec_md = specs_dir.join(&id).join("spec.md");
        let content = ito_common::io::read_to_string_or_default(&spec_md);
        let requirement_count = if content.is_empty() {
            0
        } else {
            count_requirements_in_spec_markdown(&content)
        };
        specs.push(SpecListItem {
            id,
            requirement_count,
        });
    }

    specs.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(specs)
}

#[cfg(test)]
fn parse_modular_change_module_id(folder: &str) -> Option<&str> {
    // Accept canonical folder names like:
    // - "NNN-NN_name" (2+ digit change number)
    // - "NNN-100_name" (overflow change number)
    // NOTE: This is a fast path for listing; full canonicalization lives in `parse_change_id`.
    let bytes = folder.as_bytes();
    if bytes.len() < 8 {
        return None;
    }
    if !bytes.first()?.is_ascii_digit()
        || !bytes.get(1)?.is_ascii_digit()
        || !bytes.get(2)?.is_ascii_digit()
    {
        return None;
    }
    if *bytes.get(3)? != b'-' {
        return None;
    }

    // Scan digits until '_'
    let mut i = 4usize;
    let mut digit_count = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'_' {
            break;
        }
        if !b.is_ascii_digit() {
            return None;
        }
        digit_count += 1;
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'_' {
        return None;
    }
    // Canonical change numbers are at least 2 digits ("01"), but be permissive.
    if digit_count == 0 {
        return None;
    }

    let name = &folder[(i + 1)..];
    let mut chars = name.chars();
    let first = chars.next()?;
    if !first.is_ascii_lowercase() {
        return None;
    }
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return None;
        }
    }

    Some(&folder[0..3])
}

#[derive(Debug, Clone)]
struct Section {
    level: usize,
    title: String,
    children: Vec<Section>,
}

fn count_requirements_in_spec_markdown(content: &str) -> u32 {
    let sections = parse_sections(content);
    // Match TS MarkdownParser.parseSpec: requires Purpose and Requirements.
    let purpose = find_section(&sections, "Purpose");
    let req = find_section(&sections, "Requirements");
    if purpose.is_none() || req.is_none() {
        return 0;
    }
    req.map(|s| s.children.len() as u32).unwrap_or(0)
}

fn is_completed(s: &ChangeSummary) -> bool {
    use ito_domain::changes::ChangeWorkStatus;
    matches!(
        s.work_status(),
        ChangeWorkStatus::Complete | ChangeWorkStatus::Paused
    )
}

fn is_partial(s: &ChangeSummary) -> bool {
    use ito_domain::changes::ChangeWorkStatus;
    matches!(
        s.work_status(),
        ChangeWorkStatus::Ready | ChangeWorkStatus::InProgress
    ) && s.total_tasks > 0
        && s.completed_tasks > 0
        && s.completed_tasks < s.total_tasks
}

fn is_pending(s: &ChangeSummary) -> bool {
    use ito_domain::changes::ChangeWorkStatus;
    matches!(
        s.work_status(),
        ChangeWorkStatus::Ready | ChangeWorkStatus::InProgress
    ) && s.total_tasks > 0
        && s.completed_tasks == 0
}

fn parse_sections(content: &str) -> Vec<Section> {
    let normalized = content.replace(['\r'], "");
    let lines: Vec<&str> = normalized.split('\n').collect();
    let mut sections: Vec<Section> = Vec::new();
    let mut stack: Vec<Section> = Vec::new();

    for line in lines {
        let trimmed = line.trim_end();
        if let Some((level, title)) = parse_header(trimmed) {
            let section = Section {
                level,
                title: title.to_string(),
                children: Vec::new(),
            };

            while stack.last().is_some_and(|s| s.level >= level) {
                let completed = stack.pop().expect("checked");
                attach_section(&mut sections, &mut stack, completed);
            }

            stack.push(section);
        }
    }

    while let Some(completed) = stack.pop() {
        attach_section(&mut sections, &mut stack, completed);
    }

    sections
}

fn attach_section(sections: &mut Vec<Section>, stack: &mut [Section], section: Section) {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(section);
    } else {
        sections.push(section);
    }
}

fn parse_header(line: &str) -> Option<(usize, &str)> {
    let bytes = line.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut i = 0usize;
    while i < bytes.len() && bytes[i] == b'#' {
        i += 1;
    }
    if i == 0 || i > 6 {
        return None;
    }
    if i >= bytes.len() || !bytes[i].is_ascii_whitespace() {
        return None;
    }
    let title = line[i..].trim();
    if title.is_empty() {
        return None;
    }
    Some((i, title))
}

fn find_section<'a>(sections: &'a [Section], title: &str) -> Option<&'a Section> {
    for s in sections {
        if s.title.eq_ignore_ascii_case(title) {
            return Some(s);
        }
        if let Some(child) = find_section(&s.children, title) {
            return Some(child);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: impl AsRef<Path>, contents: &str) {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("parent dirs should exist");
        }
        std::fs::write(path, contents).expect("test fixture should write");
    }

    fn make_change(root: &Path, id: &str, tasks: &str) {
        write(
            root.join(".ito/changes").join(id).join("proposal.md"),
            "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
        );
        write(root.join(".ito/changes").join(id).join("tasks.md"), tasks);
        write(
            root.join(".ito/changes")
                .join(id)
                .join("specs")
                .join("alpha")
                .join("spec.md"),
            "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
        );
    }

    #[test]
    fn counts_requirements_from_headings() {
        let md = r#"
# Title

## Purpose
blah

## Requirements

### Requirement: One
foo

### Requirement: Two
bar
"#;
        assert_eq!(count_requirements_in_spec_markdown(md), 2);
    }

    #[test]
    fn iso_millis_matches_expected_shape() {
        let dt = DateTime::parse_from_rfc3339("2026-01-26T00:00:00.123Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(to_iso_millis(dt), "2026-01-26T00:00:00.123Z");
    }

    #[test]
    fn parse_modular_change_module_id_allows_overflow_change_numbers() {
        assert_eq!(parse_modular_change_module_id("001-02_foo"), Some("001"));
        assert_eq!(parse_modular_change_module_id("001-100_foo"), Some("001"));
        assert_eq!(parse_modular_change_module_id("001-1234_foo"), Some("001"));
    }

    #[test]
    fn list_changes_filters_by_progress_status() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");
        make_change(
            repo.path(),
            "000-01_pending",
            "## 1. Implementation\n- [ ] 1.1 todo\n",
        );
        make_change(
            repo.path(),
            "000-02_partial",
            "## 1. Implementation\n- [x] 1.1 done\n- [ ] 1.2 todo\n",
        );
        make_change(
            repo.path(),
            "000-03_completed",
            "## 1. Implementation\n- [x] 1.1 done\n",
        );

        let change_repo = crate::change_repository::FsChangeRepository::new(&ito_path);

        let ready = list_changes(
            &change_repo,
            ListChangesInput {
                progress_filter: ChangeProgressFilter::Ready,
                sort: ChangeSortOrder::Name,
            },
        )
        .expect("ready list should succeed");
        assert_eq!(ready.len(), 2);
        assert_eq!(ready[0].name, "000-01_pending");
        assert_eq!(ready[1].name, "000-02_partial");

        let pending = list_changes(
            &change_repo,
            ListChangesInput {
                progress_filter: ChangeProgressFilter::Pending,
                sort: ChangeSortOrder::Name,
            },
        )
        .expect("pending list should succeed");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].name, "000-01_pending");

        let partial = list_changes(
            &change_repo,
            ListChangesInput {
                progress_filter: ChangeProgressFilter::Partial,
                sort: ChangeSortOrder::Name,
            },
        )
        .expect("partial list should succeed");
        assert_eq!(partial.len(), 1);
        assert_eq!(partial[0].name, "000-02_partial");

        let completed = list_changes(
            &change_repo,
            ListChangesInput {
                progress_filter: ChangeProgressFilter::Completed,
                sort: ChangeSortOrder::Name,
            },
        )
        .expect("completed list should succeed");
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].name, "000-03_completed");
        assert!(completed[0].completed);
    }

    #[test]
    fn list_changes_sorts_by_name_and_recent() {
        let repo = tempfile::tempdir().expect("repo tempdir");
        let ito_path = repo.path().join(".ito");
        make_change(
            repo.path(),
            "000-01_alpha",
            "## 1. Implementation\n- [ ] 1.1 todo\n",
        );
        make_change(
            repo.path(),
            "000-02_beta",
            "## 1. Implementation\n- [ ] 1.1 todo\n",
        );
        // Set explicit mtimes so sort-by-recent is deterministic without sleeping.
        let alpha_dir = repo.path().join(".ito/changes/000-01_alpha");
        let beta_dir = repo.path().join(".ito/changes/000-02_beta");
        let earlier = filetime::FileTime::from_unix_time(1_000_000, 0);
        let later = filetime::FileTime::from_unix_time(2_000_000, 0);
        filetime::set_file_mtime(&alpha_dir, earlier).expect("set alpha mtime");
        filetime::set_file_mtime(&beta_dir, later).expect("set beta mtime");

        let change_repo = crate::change_repository::FsChangeRepository::new(&ito_path);

        let by_name = list_changes(
            &change_repo,
            ListChangesInput {
                progress_filter: ChangeProgressFilter::All,
                sort: ChangeSortOrder::Name,
            },
        )
        .expect("name sort should succeed");
        assert_eq!(by_name[0].name, "000-01_alpha");
        assert_eq!(by_name[1].name, "000-02_beta");

        let by_recent = list_changes(
            &change_repo,
            ListChangesInput {
                progress_filter: ChangeProgressFilter::All,
                sort: ChangeSortOrder::Recent,
            },
        )
        .expect("recent sort should succeed");
        assert_eq!(by_recent[0].name, "000-02_beta");
        assert_eq!(by_recent[1].name, "000-01_alpha");
    }
}
