use std::path::{Path, PathBuf};

use miette::Result;
use serde::Serialize;

use ito_common::fs::StdFs;
use ito_common::paths;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Scenario {
    #[serde(rename = "rawText")]
    pub raw_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Requirement {
    pub text: String,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SpecShowJson {
    pub id: String,
    pub title: String,
    pub overview: String,
    #[serde(rename = "requirementCount")]
    pub requirement_count: u32,
    pub requirements: Vec<Requirement>,
    pub metadata: SpecMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SpecMetadata {
    pub version: String,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeShowJson {
    pub id: String,
    pub title: String,
    #[serde(rename = "deltaCount")]
    pub delta_count: u32,
    pub deltas: Vec<ChangeDelta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeDelta {
    pub spec: String,
    pub operation: String,
    pub description: String,
    pub requirement: Requirement,
    pub requirements: Vec<Requirement>,
}

pub fn read_spec_markdown(ito_path: &Path, id: &str) -> Result<String> {
    let path = paths::spec_markdown_path(ito_path, id);
    ito_common::io::read_to_string(&path)
}

pub fn read_change_proposal_markdown(ito_path: &Path, change_id: &str) -> Result<String> {
    let path = paths::change_dir(ito_path, change_id).join("proposal.md");
    ito_common::io::read_to_string(&path)
}

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

pub fn read_change_delta_spec_paths(ito_path: &Path, change_id: &str) -> Result<Vec<PathBuf>> {
    let specs_dir = paths::change_specs_dir(ito_path, change_id);
    if !specs_dir.exists() {
        return Ok(vec![]);
    }

    let mut out: Vec<PathBuf> = Vec::new();
    let fs = StdFs;
    for name in ito_domain::discovery::list_dir_names(&fs, &specs_dir)? {
        let spec_md = specs_dir.join(name).join("spec.md");
        if spec_md.exists() {
            out.push(spec_md);
        }
    }
    out.sort();
    Ok(out)
}

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
pub struct DeltaSpecFile {
    pub spec: String,
    pub markdown: String,
}

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
