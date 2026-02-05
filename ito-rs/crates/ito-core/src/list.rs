use std::path::{Path, PathBuf};

use chrono::{DateTime, SecondsFormat, Timelike, Utc};
use miette::{IntoDiagnostic, Result};

use ito_common::fs::StdFs;
use ito_common::paths;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ModuleListItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "changeCount")]
    pub change_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ChangeListItem {
    pub name: String,
    #[serde(rename = "completedTasks")]
    pub completed_tasks: u32,
    #[serde(rename = "shelvedTasks")]
    pub shelved_tasks: u32,
    #[serde(rename = "inProgressTasks")]
    pub in_progress_tasks: u32,
    #[serde(rename = "pendingTasks")]
    pub pending_tasks: u32,
    #[serde(rename = "totalTasks")]
    pub total_tasks: u32,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    /// Legacy status field for backward compatibility
    pub status: String,
    /// Work status: draft, ready, in-progress, paused, complete
    #[serde(rename = "workStatus")]
    pub work_status: String,
    /// True when no remaining work (complete or paused)
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SpecListItem {
    pub id: String,
    #[serde(rename = "requirementCount")]
    pub requirement_count: u32,
}

pub fn list_modules(ito_path: &Path) -> Result<Vec<ModuleListItem>> {
    let changes_dir = paths::changes_dir(ito_path);

    let mut modules: Vec<ModuleListItem> = Vec::new();
    let modules_dir = paths::modules_dir(ito_path);

    let fs = StdFs;
    for full_name in ito_domain::discovery::list_module_dir_names(&fs, ito_path)? {
        let Some((id, name)) = parse_module_folder_name(&full_name) else {
            continue;
        };
        if std::fs::metadata(modules_dir.join(&full_name).join("module.md")).is_err() {
            continue;
        }
        let change_count = count_changes_for_module(&changes_dir, &id)?;
        modules.push(ModuleListItem {
            id,
            name,
            full_name,
            change_count,
        });
    }

    modules.sort_by(|a, b| a.full_name.cmp(&b.full_name));
    Ok(modules)
}

pub fn list_change_dirs(ito_path: &Path) -> Result<Vec<PathBuf>> {
    let fs = StdFs;
    Ok(ito_domain::discovery::list_change_dir_names(&fs, ito_path)?
        .into_iter()
        .map(|name| paths::change_dir(ito_path, &name))
        .collect())
}

pub fn last_modified_recursive(path: &Path) -> Result<DateTime<Utc>> {
    use std::collections::VecDeque;

    let mut max = std::fs::metadata(path)
        .into_diagnostic()?
        .modified()
        .into_diagnostic()?;

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

pub fn to_iso_millis(dt: DateTime<Utc>) -> String {
    // JS Date.toISOString() is millisecond-precision. Truncate to millis to avoid
    // platform-specific sub-ms differences.
    let nanos = dt.timestamp_subsec_nanos();
    let truncated = dt
        .with_nanosecond((nanos / 1_000_000) * 1_000_000)
        .unwrap_or(dt);
    truncated.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn list_specs(ito_path: &Path) -> Result<Vec<SpecListItem>> {
    let mut specs: Vec<SpecListItem> = Vec::new();
    let specs_dir = paths::specs_dir(ito_path);
    let fs = StdFs;
    for id in ito_domain::discovery::list_spec_dir_names(&fs, ito_path)? {
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

fn parse_module_folder_name(folder: &str) -> Option<(String, String)> {
    // TS regex: /^(\d{3})_([a-z][a-z0-9-]*)$/
    let bytes = folder.as_bytes();
    if bytes.len() < 5 {
        return None;
    }
    if !bytes.first()?.is_ascii_digit()
        || !bytes.get(1)?.is_ascii_digit()
        || !bytes.get(2)?.is_ascii_digit()
    {
        return None;
    }
    if *bytes.get(3)? != b'_' {
        return None;
    }
    let name = &folder[4..];
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
    Some((folder[0..3].to_string(), name.to_string()))
}

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

fn count_changes_for_module(changes_dir: &Path, module_id: &str) -> Result<usize> {
    if !changes_dir.exists() {
        return Ok(0);
    }
    let mut count = 0usize;
    let fs = StdFs;
    for name in ito_domain::discovery::list_dir_names(&fs, changes_dir)? {
        if name == "archive" {
            continue;
        }
        if std::fs::metadata(changes_dir.join(&name).join("proposal.md")).is_err() {
            continue;
        }
        if let Some(mid) = parse_modular_change_module_id(&name)
            && mid == module_id
        {
            count += 1;
        }
    }
    Ok(count)
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
}
