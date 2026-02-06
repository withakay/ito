//! Convert Ito markdown artifacts into JSON-friendly structures.
//!
//! This module is used by "show"-style commands and APIs. It reads spec and
//! change markdown files from disk and produces lightweight structs that can be
//! serialized to JSON.

use std::path::Path;

use crate::error_bridge::IntoCoreMiette;
use miette::Result;
use serde::Serialize;

use ito_common::paths;
use ito_domain::changes::ChangeRepository;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// One raw scenario block from a spec or delta.
pub struct Scenario {
    #[serde(rename = "rawText")]
    /// The original scenario text (preserves newlines).
    pub raw_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// A single requirement statement and its scenarios.
pub struct Requirement {
    /// The normalized requirement statement.
    pub text: String,

    /// Scenario blocks associated with the requirement.
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// JSON-serializable view of a spec markdown file.
pub struct SpecShowJson {
    /// Spec id (folder name under `.ito/specs/`).
    pub id: String,
    /// Human-readable title (currently same as `id`).
    pub title: String,
    /// Extracted `## Purpose` section.
    pub overview: String,
    #[serde(rename = "requirementCount")]
    /// Total number of requirements.
    pub requirement_count: u32,

    /// Requirements parsed from the markdown.
    pub requirements: Vec<Requirement>,

    /// Metadata describing the output format.
    pub metadata: SpecMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Additional info included in serialized spec output.
pub struct SpecMetadata {
    /// Output schema version.
    pub version: String,

    /// Output format identifier.
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// JSON-serializable view of a change (proposal + deltas).
pub struct ChangeShowJson {
    /// Change id (folder name under `.ito/changes/`).
    pub id: String,
    /// Human-readable title (currently same as `id`).
    pub title: String,
    #[serde(rename = "deltaCount")]
    /// Total number of deltas.
    pub delta_count: u32,

    /// Parsed deltas from delta spec files.
    pub deltas: Vec<ChangeDelta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// One delta entry extracted from a change delta spec.
pub struct ChangeDelta {
    /// Spec id the delta belongs to.
    pub spec: String,

    /// Delta operation (e.g. `ADDED`, `MODIFIED`).
    pub operation: String,

    /// Human-readable description for display.
    pub description: String,

    /// Primary requirement extracted for the delta (legacy shape).
    pub requirement: Requirement,

    /// All requirements extracted for the delta.
    pub requirements: Vec<Requirement>,
}

/// Read the markdown for a spec id from `.ito/specs/<id>/spec.md`.
pub fn read_spec_markdown(ito_path: &Path, id: &str) -> Result<String> {
    let path = paths::spec_markdown_path(ito_path, id);
    ito_common::io::read_to_string(&path)
}

/// Read the proposal markdown for a change id.
pub fn read_change_proposal_markdown(
    repo: &impl ChangeRepository,
    change_id: &str,
) -> Result<Option<String>> {
    let change = repo.get(change_id).into_core_miette()?;
    Ok(change.proposal)
}

/// Parse spec markdown into a serializable structure.
pub fn parse_spec_show_json(id: &str, markdown: &str) -> SpecShowJson {
    let overview = extract_section_text(markdown, "Purpose");
    let requirements = parse_spec_requirements(markdown);
    SpecShowJson {
        id: id.to_string(),
        title: id.to_string(),
        overview,
        requirement_count: requirements.len() as u32,
        requirements,
        metadata: SpecMetadata {
            version: "1.0.0".to_string(),
            format: "ito".to_string(),
        },
    }
}

/// Return all delta spec files for a change from the repository.
pub fn read_change_delta_spec_files(
    repo: &impl ChangeRepository,
    change_id: &str,
) -> Result<Vec<DeltaSpecFile>> {
    let change = repo.get(change_id).into_core_miette()?;
    let mut out: Vec<DeltaSpecFile> = change
        .specs
        .into_iter()
        .map(|spec| DeltaSpecFile {
            spec: spec.name,
            markdown: spec.content,
        })
        .collect();
    out.sort_by(|a, b| a.spec.cmp(&b.spec));
    Ok(out)
}

/// Parse a change id plus its delta spec files into a JSON-friendly structure.
pub fn parse_change_show_json(change_id: &str, delta_specs: &[DeltaSpecFile]) -> ChangeShowJson {
    let mut deltas: Vec<ChangeDelta> = Vec::new();
    for file in delta_specs {
        deltas.extend(parse_delta_spec_file(file));
    }

    ChangeShowJson {
        id: change_id.to_string(),
        title: change_id.to_string(),
        delta_count: deltas.len() as u32,
        deltas,
    }
}

#[derive(Debug, Clone)]
/// One loaded delta spec file.
pub struct DeltaSpecFile {
    /// Spec id this delta spec belongs to.
    pub spec: String,

    /// Full markdown contents of the delta `spec.md`.
    pub markdown: String,
}

/// Load a delta `spec.md` and infer the spec id from its parent directory.
pub fn load_delta_spec_file(path: &Path) -> Result<DeltaSpecFile> {
    let markdown = ito_common::io::read_to_string(path)?;
    let spec = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    Ok(DeltaSpecFile { spec, markdown })
}

fn parse_delta_spec_file(file: &DeltaSpecFile) -> Vec<ChangeDelta> {
    let mut out: Vec<ChangeDelta> = Vec::new();

    let mut current_op: Option<String> = None;
    let mut i = 0usize;
    let normalized = file.markdown.replace('\r', "");
    let lines: Vec<&str> = normalized.split('\n').collect();
    while i < lines.len() {
        let line = lines[i].trim_end();
        if let Some(op) = parse_delta_op_header(line) {
            current_op = Some(op);
            i += 1;
            continue;
        }

        if let Some(title) = line.strip_prefix("### Requirement:") {
            let op = current_op.clone().unwrap_or_else(|| "ADDED".to_string());
            let (_req_title, requirement, next) = parse_requirement_block(&lines, i);
            i = next;

            let description = match op.as_str() {
                "ADDED" => format!("Add requirement: {}", requirement.text),
                "MODIFIED" => format!("Modify requirement: {}", requirement.text),
                "REMOVED" => format!("Remove requirement: {}", requirement.text),
                "RENAMED" => format!("Rename requirement: {}", requirement.text),
                _ => format!("Add requirement: {}", requirement.text),
            };
            out.push(ChangeDelta {
                spec: file.spec.clone(),
                operation: op,
                description,
                requirement: requirement.clone(),
                requirements: vec![requirement],
            });
            // Title is currently unused but parsed for parity with TS structure.
            let _ = title;
            continue;
        }

        i += 1;
    }

    out
}

fn parse_delta_op_header(line: &str) -> Option<String> {
    // Example: "## ADDED Requirements"
    let t = line.trim();
    let rest = t.strip_prefix("## ")?;
    let rest = rest.trim();
    let op = rest.strip_suffix(" Requirements").unwrap_or(rest).trim();
    if matches!(op, "ADDED" | "MODIFIED" | "REMOVED" | "RENAMED") {
        return Some(op.to_string());
    }
    None
}

fn parse_spec_requirements(markdown: &str) -> Vec<Requirement> {
    let req_section = extract_section_lines(markdown, "Requirements");
    parse_requirements_from_lines(&req_section)
}

fn parse_requirements_from_lines(lines: &[String]) -> Vec<Requirement> {
    let mut out: Vec<Requirement> = Vec::new();
    let mut i = 0usize;
    let raw: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    while i < raw.len() {
        let line = raw[i].trim_end();
        if line.starts_with("### Requirement:") {
            let (_title, req, next) = parse_requirement_block(&raw, i);
            out.push(req);
            i = next;
            continue;
        }
        i += 1;
    }
    out
}

fn parse_requirement_block(lines: &[&str], start: usize) -> (String, Requirement, usize) {
    let header = lines[start].trim_end();
    let title = header
        .strip_prefix("### Requirement:")
        .unwrap_or("")
        .trim()
        .to_string();

    let mut i = start + 1;

    // Requirement statement: consume non-empty lines until we hit a scenario header or next requirement.
    let mut statement_lines: Vec<String> = Vec::new();
    while i < lines.len() {
        let t = lines[i].trim_end();
        if t.starts_with("#### Scenario:")
            || t.starts_with("### Requirement:")
            || t.starts_with("## ")
        {
            break;
        }
        if !t.trim().is_empty() {
            statement_lines.push(t.trim().to_string());
        }
        i += 1;
    }
    let text = collapse_whitespace(&statement_lines.join(" "));

    // Scenarios
    let mut scenarios: Vec<Scenario> = Vec::new();
    while i < lines.len() {
        let t = lines[i].trim_end();
        if t.starts_with("### Requirement:") || t.starts_with("## ") {
            break;
        }
        if let Some(_name) = t.strip_prefix("#### Scenario:") {
            i += 1;
            let mut raw_lines: Vec<String> = Vec::new();
            while i < lines.len() {
                let l = lines[i].trim_end();
                if l.starts_with("#### Scenario:")
                    || l.starts_with("### Requirement:")
                    || l.starts_with("## ")
                {
                    break;
                }
                raw_lines.push(l.to_string());
                i += 1;
            }
            let raw_text = trim_trailing_blank_lines(&raw_lines).join("\n");
            scenarios.push(Scenario { raw_text });
            continue;
        }
        i += 1;
    }

    (title, Requirement { text, scenarios }, i)
}

fn extract_section_text(markdown: &str, header: &str) -> String {
    let lines = extract_section_lines(markdown, header);
    let joined = lines.join(" ");
    collapse_whitespace(joined.trim())
}

fn extract_section_lines(markdown: &str, header: &str) -> Vec<String> {
    let mut in_section = false;
    let mut out: Vec<String> = Vec::new();
    let normalized = markdown.replace('\r', "");
    for raw in normalized.split('\n') {
        let line = raw.trim_end();
        if let Some(h) = line.strip_prefix("## ") {
            let title = h.trim();
            if title.eq_ignore_ascii_case(header) {
                in_section = true;
                continue;
            }
            if in_section {
                break;
            }
        }
        if in_section {
            out.push(line.to_string());
        }
    }
    out
}

fn collapse_whitespace(input: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in input.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

fn trim_trailing_blank_lines(lines: &[String]) -> Vec<String> {
    let mut start = 0usize;
    while start < lines.len() {
        if lines[start].trim().is_empty() {
            start += 1;
        } else {
            break;
        }
    }

    let mut end = lines.len();
    while end > start {
        if lines[end - 1].trim().is_empty() {
            end -= 1;
        } else {
            break;
        }
    }

    lines[start..end].to_vec()
}
