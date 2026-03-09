//! Convert Ito markdown artifacts into JSON-friendly structures.
//!
//! This module is used by "show"-style commands and APIs. It reads spec and
//! change markdown files from disk and produces lightweight structs that can be
//! serialized to JSON.

use std::path::Path;

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use crate::spec_repository::FsSpecRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::specs::SpecRepository;
use serde::Serialize;

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
/// JSON-serializable view of all main specs bundled together.
pub struct SpecsBundleJson {
    #[serde(rename = "specCount")]
    /// Total number of bundled specs.
    pub spec_count: u32,

    /// Bundled specs, ordered by ascending spec id.
    pub specs: Vec<BundledSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// One bundled spec entry (id + source path + raw markdown).
pub struct BundledSpec {
    /// Spec id (folder name under `.ito/specs/`).
    pub id: String,

    /// Absolute path to the source `.ito/specs/<id>/spec.md` file.
    pub path: String,

    /// Raw markdown contents of the spec.
    pub markdown: String,
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
pub fn read_spec_markdown(ito_path: &Path, id: &str) -> CoreResult<String> {
    let repo = FsSpecRepository::new(ito_path);
    read_spec_markdown_from_repository(&repo, id)
}

/// Read the markdown for a spec id from a repository.
pub fn read_spec_markdown_from_repository(
    repo: &(impl SpecRepository + ?Sized),
    id: &str,
) -> CoreResult<String> {
    let spec = repo.get(id).into_core()?;
    Ok(spec.markdown)
}

/// Read the proposal markdown for a change id.
pub fn read_change_proposal_markdown(
    repo: &(impl ChangeRepository + ?Sized),
    change_id: &str,
) -> CoreResult<Option<String>> {
    let change = repo.get(change_id).into_core()?;
    Ok(change.proposal)
}

/// Read the raw markdown for a module's `module.md` file.
pub fn read_module_markdown(
    module_repo: &(impl ModuleRepository + ?Sized),
    module_id: &str,
) -> CoreResult<String> {
    let module = module_repo.get(module_id).into_core()?;
    let module_md_path = module.path.join("module.md");
    if module_md_path.is_file() {
        let md = ito_common::io::read_to_string_or_default(&module_md_path);
        return Ok(md);
    }

    if module.path.as_os_str().is_empty() {
        return Ok(render_module_markdown_fallback(&module));
    }

    Ok(String::new())
}

fn render_module_markdown_fallback(module: &ito_domain::modules::Module) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n", module.name));
    if let Some(description) = module.description.as_deref()
        && !description.trim().is_empty()
    {
        out.push_str("\n## Purpose\n");
        out.push_str(description.trim());
        out.push('\n');
    }
    out
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

/// Bundle all main specs under `.ito/specs/*/spec.md` into a JSON-friendly structure.
pub fn bundle_main_specs_show_json(ito_path: &Path) -> CoreResult<SpecsBundleJson> {
    use ito_common::fs::StdFs;

    let fs = StdFs;
    let mut ids = ito_domain::discovery::list_spec_dir_names(&fs, ito_path).into_core()?;
    ids.sort();

    if ids.is_empty() {
        return Err(CoreError::not_found(
            "No specs found under .ito/specs (expected .ito/specs/<id>/spec.md)".to_string(),
        ));
    }

    let mut specs = Vec::with_capacity(ids.len());
    for id in ids {
        let path = ito_common::paths::spec_markdown_path(ito_path, &id);
        let markdown = ito_common::io::read_to_string(&path)
            .map_err(|e| CoreError::io(format!("reading spec {}", id), std::io::Error::other(e)))?;
        specs.push(BundledSpec {
            id,
            path: path.to_string_lossy().to_string(),
            markdown,
        });
    }

    Ok(SpecsBundleJson {
        spec_count: specs.len() as u32,
        specs,
    })
}

/// Bundle all promoted specs from a repository into a JSON-friendly structure.
pub fn bundle_specs_show_json_from_repository(
    repo: &(impl SpecRepository + ?Sized),
) -> CoreResult<SpecsBundleJson> {
    let mut summaries = repo.list().into_core()?;
    summaries.sort_by(|left, right| left.id.cmp(&right.id));
    if summaries.is_empty() {
        return Err(CoreError::not_found(
            "No specs found under .ito/specs (expected .ito/specs/<id>/spec.md)".to_string(),
        ));
    }

    let mut specs = Vec::with_capacity(summaries.len());
    for summary in summaries {
        let spec = repo.get(&summary.id).into_core()?;
        specs.push(BundledSpec {
            id: spec.id,
            path: spec.path.to_string_lossy().to_string(),
            markdown: spec.markdown,
        });
    }

    Ok(SpecsBundleJson {
        spec_count: specs.len() as u32,
        specs,
    })
}

/// Bundle all main specs under `.ito/specs/*/spec.md` into a single markdown stream.
///
/// Each spec is preceded by a metadata comment line:
/// `<!-- spec-id: <id>; source: <absolute-path-to-spec.md> -->`.
pub fn bundle_main_specs_markdown(ito_path: &Path) -> CoreResult<String> {
    let repo = FsSpecRepository::new(ito_path);
    bundle_specs_markdown_from_repository(&repo)
}

/// Bundle all promoted specs from a repository into a single markdown stream.
pub fn bundle_specs_markdown_from_repository(
    repo: &(impl SpecRepository + ?Sized),
) -> CoreResult<String> {
    let bundle = bundle_specs_show_json_from_repository(repo)?;
    let mut out = String::new();
    for (i, spec) in bundle.specs.iter().enumerate() {
        if i != 0 {
            out.push_str("\n\n");
        }
        out.push_str(&format!(
            "<!-- spec-id: {}; source: {} -->\n",
            spec.id, spec.path
        ));
        out.push_str(&spec.markdown);
    }
    Ok(out)
}

/// Return all delta spec files for a change from the repository.
pub fn read_change_delta_spec_files(
    repo: &(impl ChangeRepository + ?Sized),
    change_id: &str,
) -> CoreResult<Vec<DeltaSpecFile>> {
    let change = repo.get(change_id).into_core()?;
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
pub fn load_delta_spec_file(path: &Path) -> CoreResult<DeltaSpecFile> {
    let markdown = ito_common::io::read_to_string(path).map_err(|e| {
        CoreError::io(
            format!("reading delta spec {}", path.display()),
            std::io::Error::other(e),
        )
    })?;
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
    if op == "ADDED" || op == "MODIFIED" || op == "REMOVED" || op == "RENAMED" {
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
