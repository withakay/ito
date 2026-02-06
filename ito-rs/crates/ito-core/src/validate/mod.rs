//! Validate Ito repository artifacts.
//!
//! This module provides lightweight validation helpers for specs, changes, and
//! modules.
//!
//! The primary consumer is the CLI and any APIs that need a structured report
//! (`ValidationReport`) rather than a single error.

use std::path::{Path, PathBuf};

use miette::Result;
use serde::Serialize;

use ito_common::fs::StdFs;
use ito_common::paths;

use crate::change_repository::FsChangeRepository;
use crate::show::{parse_change_show_json, parse_spec_show_json, read_change_delta_spec_files};

mod issue;
mod repo_integrity;
mod report;

pub use issue::{error, info, issue, warning, with_line, with_loc, with_metadata};
pub use repo_integrity::validate_change_dirs_repo_integrity;
pub use report::{ReportBuilder, report};

/// Severity level for a [`ValidationIssue`].
pub type ValidationLevel = &'static str;

/// Validation issue is an error (always fails validation).
pub const LEVEL_ERROR: ValidationLevel = "ERROR";
/// Validation issue is a warning (fails validation in strict mode).
pub const LEVEL_WARNING: ValidationLevel = "WARNING";
/// Validation issue is informational (never fails validation).
pub const LEVEL_INFO: ValidationLevel = "INFO";

// Thresholds: match TS defaults.
const MIN_PURPOSE_LENGTH: usize = 50;
const MIN_MODULE_PURPOSE_LENGTH: usize = 20;
const MAX_DELTAS_PER_CHANGE: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// One validation finding.
pub struct ValidationIssue {
    /// Issue severity.
    pub level: String,
    /// Logical path within the validated artifact (or a filename).
    pub path: String,
    /// Human-readable message.
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional 1-based line number.
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional 1-based column number.
    pub column: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional structured metadata for tooling.
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// A validation report with a computed summary.
pub struct ValidationReport {
    /// Whether validation passed for the selected strictness.
    pub valid: bool,

    /// All issues found (errors + warnings + info).
    pub issues: Vec<ValidationIssue>,

    /// Counts grouped by severity.
    pub summary: ValidationSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Aggregated counts for a validation run.
pub struct ValidationSummary {
    /// Number of `ERROR` issues.
    pub errors: u32,
    /// Number of `WARNING` issues.
    pub warnings: u32,
    /// Number of `INFO` issues.
    pub info: u32,
}

impl ValidationReport {
    /// Construct a report and compute summary + `valid`.
    ///
    /// When `strict` is `true`, warnings are treated as failures.
    pub fn new(issues: Vec<ValidationIssue>, strict: bool) -> Self {
        let mut errors = 0u32;
        let mut warnings = 0u32;
        let mut info = 0u32;
        for i in &issues {
            match i.level.as_str() {
                LEVEL_ERROR => errors += 1,
                LEVEL_WARNING => warnings += 1,
                LEVEL_INFO => info += 1,
                _ => {}
            }
        }
        let valid = if strict {
            errors == 0 && warnings == 0
        } else {
            errors == 0
        };
        Self {
            valid,
            issues,
            summary: ValidationSummary {
                errors,
                warnings,
                info,
            },
        }
    }
}

/// Validate a spec markdown string and return a structured report.
pub fn validate_spec_markdown(markdown: &str, strict: bool) -> ValidationReport {
    let json = parse_spec_show_json("<spec>", markdown);

    let mut r = report(strict);

    if json.overview.trim().is_empty() {
        r.push(error("purpose", "Purpose section cannot be empty"));
    } else if json.overview.len() < MIN_PURPOSE_LENGTH {
        r.push(warning(
            "purpose",
            "Purpose section is too brief (less than 50 characters)",
        ));
    }

    if json.requirements.is_empty() {
        r.push(error(
            "requirements",
            "Spec must have at least one requirement",
        ));
    }

    for (idx, req) in json.requirements.iter().enumerate() {
        let path = format!("requirements[{idx}]");
        if req.text.trim().is_empty() {
            r.push(error(&path, "Requirement text cannot be empty"));
        }
        if req.scenarios.is_empty() {
            r.push(error(&path, "Requirement must have at least one scenario"));
        }
        for (sidx, sc) in req.scenarios.iter().enumerate() {
            let sp = format!("{path}.scenarios[{sidx}]");
            if sc.raw_text.trim().is_empty() {
                r.push(error(&sp, "Scenario text cannot be empty"));
            }
        }
    }

    r.finish()
}

/// Validate a spec by id from `.ito/specs/<id>/spec.md`.
pub fn validate_spec(ito_path: &Path, spec_id: &str, strict: bool) -> Result<ValidationReport> {
    let path = paths::spec_markdown_path(ito_path, spec_id);
    let markdown = ito_common::io::read_to_string(&path)?;
    Ok(validate_spec_markdown(&markdown, strict))
}

/// Validate a change's delta specs by change id.
pub fn validate_change(ito_path: &Path, change_id: &str, strict: bool) -> Result<ValidationReport> {
    let change_repo = FsChangeRepository::new(ito_path);
    let files = read_change_delta_spec_files(&change_repo, change_id)?;
    if files.is_empty() {
        let mut r = report(strict);
        r.push(error("specs", "Change must have at least one delta"));
        return Ok(r.finish());
    }

    let show = parse_change_show_json(change_id, &files);
    let mut rep = report(strict);
    if show.deltas.is_empty() {
        rep.push(error("specs", "Change must have at least one delta"));
        return Ok(rep.finish());
    }

    if show.deltas.len() > MAX_DELTAS_PER_CHANGE {
        rep.push(info(
            "deltas",
            "Consider splitting changes with more than 10 deltas",
        ));
    }

    for (idx, d) in show.deltas.iter().enumerate() {
        let base = format!("deltas[{idx}]");
        if d.description.trim().is_empty() {
            rep.push(error(&base, "Delta description cannot be empty"));
        } else if d.description.trim().len() < 20 {
            rep.push(warning(&base, "Delta description is too brief"));
        }

        if d.requirements.is_empty() {
            rep.push(warning(&base, "Delta should include requirements"));
        }

        for (ridx, req) in d.requirements.iter().enumerate() {
            let rp = format!("{base}.requirements[{ridx}]");
            if req.text.trim().is_empty() {
                rep.push(error(&rp, "Requirement text cannot be empty"));
            }
            let up = req.text.to_ascii_uppercase();
            if !up.contains("SHALL") && !up.contains("MUST") {
                rep.push(error(&rp, "Requirement must contain SHALL or MUST keyword"));
            }
            if req.scenarios.is_empty() {
                rep.push(error(&rp, "Requirement must have at least one scenario"));
            }
        }
    }

    Ok(rep.finish())
}

#[derive(Debug, Clone)]
/// A resolved module reference (directory + key paths).
pub struct ResolvedModule {
    /// 3-digit module id.
    pub id: String,
    /// Directory name under `.ito/modules/`.
    pub full_name: String,
    /// Full path to the module directory.
    pub module_dir: PathBuf,
    /// Full path to `module.md`.
    pub module_md: PathBuf,
}

/// Resolve a module directory name from user input.
///
/// Input can be a full directory name (`NNN_slug`) or the numeric module id
/// (`NNN`). Empty input returns `Ok(None)`.
pub fn resolve_module(ito_path: &Path, input: &str) -> Result<Option<ResolvedModule>> {
    let modules_dir = paths::modules_dir(ito_path);
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let mut wanted_id: Option<String> = None;
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        let num: u32 = trimmed.parse().unwrap_or(0);
        wanted_id = Some(format!("{num:03}"));
    }

    let fs = StdFs;
    for full_name in ito_domain::discovery::list_module_dir_names(&fs, ito_path)? {
        // folder format: NNN_name
        let Some((id_part, _)) = full_name.split_once('_') else {
            continue;
        };
        if id_part.len() != 3 || !id_part.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        if full_name == trimmed
            || wanted_id.as_deref().is_some_and(|w| w == id_part)
            || trimmed == id_part
        {
            let module_dir = modules_dir.join(&full_name);
            let module_md = module_dir.join("module.md");
            return Ok(Some(ResolvedModule {
                id: id_part.to_string(),
                full_name,
                module_dir,
                module_md,
            }));
        }
    }

    Ok(None)
}

/// Validate a module's `module.md` for minimal required sections.
///
/// Returns the resolved module directory name along with the report.
pub fn validate_module(
    ito_path: &Path,
    module_input: &str,
    strict: bool,
) -> Result<(String, ValidationReport)> {
    let resolved = resolve_module(ito_path, module_input)?;
    let Some(r) = resolved else {
        let mut rep = report(strict);
        rep.push(error("module", "Module not found"));
        return Ok((module_input.to_string(), rep.finish()));
    };

    let mut rep = report(strict);
    let md = match ito_common::io::read_to_string_std(&r.module_md) {
        Ok(c) => c,
        Err(_) => {
            rep.push(error("file", "Module must have a Purpose section"));
            return Ok((r.full_name, rep.finish()));
        }
    };

    let purpose = extract_section(&md, "Purpose");
    if purpose.trim().is_empty() {
        rep.push(error("purpose", "Module must have a Purpose section"));
    } else if purpose.trim().len() < MIN_MODULE_PURPOSE_LENGTH {
        rep.push(error(
            "purpose",
            "Module purpose must be at least 20 characters",
        ));
    }

    let scope = extract_section(&md, "Scope");
    if scope.trim().is_empty() {
        rep.push(error(
            "scope",
            "Module must have a Scope section with at least one capability (use \"*\" for unrestricted)",
        ));
    }

    Ok((r.full_name, rep.finish()))
}

fn extract_section(markdown: &str, header: &str) -> String {
    let mut in_section = false;
    let mut out = String::new();
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
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}
