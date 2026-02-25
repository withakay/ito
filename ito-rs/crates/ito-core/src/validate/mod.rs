//! Validate Ito repository artifacts.
//!
//! This module provides lightweight validation helpers for specs, changes, and
//! modules.
//!
//! The primary consumer is the CLI and any APIs that need a structured report
//! (`ValidationReport`) rather than a single error.

use std::path::{Path, PathBuf};

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use serde::Serialize;

use ito_common::paths;

use crate::show::{parse_change_show_json, parse_spec_show_json, read_change_delta_spec_files};
use crate::templates::{
    artifact_done, load_schema_validation, read_change_schema, resolve_schema, ResolvedSchema,
    ValidationLevelYaml, ValidationYaml, ValidatorId,
};
use ito_config::ConfigContext;
use ito_domain::changes::ChangeRepository as DomainChangeRepository;
use ito_domain::modules::ModuleRepository as DomainModuleRepository;

mod format_specs;
mod issue;
mod repo_integrity;
mod report;

pub(crate) use issue::with_format_spec;
pub use issue::{error, info, issue, warning, with_line, with_loc, with_metadata};
pub use repo_integrity::validate_change_dirs_repo_integrity;
pub use report::{report, ReportBuilder};

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
pub fn validate_spec(ito_path: &Path, spec_id: &str, strict: bool) -> CoreResult<ValidationReport> {
    let path = paths::spec_markdown_path(ito_path, spec_id);
    let markdown = ito_common::io::read_to_string_std(&path)
        .map_err(|e| CoreError::io(format!("reading spec {}", spec_id), e))?;
    Ok(validate_spec_markdown(&markdown, strict))
}

/// Validate a change's delta specs by change id.
pub fn validate_change(
    change_repo: &impl DomainChangeRepository,
    ito_path: &Path,
    change_id: &str,
    strict: bool,
) -> CoreResult<ValidationReport> {
    let mut rep = report(strict);

    let (ctx, schema_name) = resolve_validation_context(ito_path, change_id);

    let resolved = match resolve_schema(Some(&schema_name), &ctx) {
        Ok(s) => {
            rep.push(info(
                "schema",
                format!(
                    "Resolved schema '{}' from {}",
                    s.schema.name,
                    s.source.as_str()
                ),
            ));
            Some(s)
        }
        Err(e) => {
            rep.push(error(
                "schema",
                format!("Failed to resolve schema '{schema_name}': {e}"),
            ));
            None
        }
    };

    if let Some(resolved) = &resolved {
        match load_schema_validation(resolved) {
            Ok(Some(validation)) => {
                rep.push(info("schema.validation", "Using schema validation.yaml"));
                validate_change_against_schema_validation(
                    &mut rep,
                    change_repo,
                    ito_path,
                    change_id,
                    resolved,
                    &validation,
                    strict,
                )?;
                return Ok(rep.finish());
            }
            Ok(None) => {}
            Err(e) => {
                rep.push(error(
                    "schema.validation",
                    format!("Failed to load schema validation.yaml: {e}"),
                ));
                return Ok(rep.finish());
            }
        }

        if is_legacy_delta_schema(&resolved.schema.name) {
            validate_change_delta_specs(&mut rep, change_repo, change_id)?;

            let tracks_rel = resolved
                .schema
                .apply
                .as_ref()
                .and_then(|a| a.tracks.as_deref())
                .unwrap_or("tasks.md");

            if !ito_domain::tasks::is_safe_tracking_filename(tracks_rel) {
                rep.push(error(
                    "tracking",
                    format!("Invalid tracking file path in apply.tracks: '{tracks_rel}'"),
                ));
                return Ok(rep.finish());
            }

            let report_path = format!("changes/{change_id}/{tracks_rel}");
            let abs_path = paths::change_dir(ito_path, change_id).join(tracks_rel);
            rep.extend(validate_tasks_tracking_path(
                &abs_path,
                &report_path,
                strict,
            ));
            return Ok(rep.finish());
        }

        rep.push(info(
            "schema.validation",
            "Schema has no validation.yaml; manual validation required",
        ));
        validate_apply_required_artifacts(&mut rep, ito_path, change_id, resolved);
        return Ok(rep.finish());
    }

    validate_change_delta_specs(&mut rep, change_repo, change_id)?;
    Ok(rep.finish())
}

/// Returns true for built-in schemas that predate schema-driven `validation.yaml`.
fn is_legacy_delta_schema(schema_name: &str) -> bool {
    schema_name == "spec-driven" || schema_name == "tdd"
}

fn schema_artifact_ids(resolved: &ResolvedSchema) -> Vec<String> {
    let mut ids = Vec::new();
    for a in &resolved.schema.artifacts {
        ids.push(a.id.clone());
    }
    ids
}

fn validate_apply_required_artifacts(
    rep: &mut ReportBuilder,
    ito_path: &Path,
    change_id: &str,
    resolved: &ResolvedSchema,
) {
    let change_dir = paths::change_dir(ito_path, change_id);
    if !change_dir.exists() {
        rep.push(error(
            "change",
            format!("Change directory not found: changes/{change_id}"),
        ));
        return;
    }

    let required_ids: Vec<String> = match resolved.schema.apply.as_ref() {
        Some(apply) => apply
            .requires
            .clone()
            .unwrap_or_else(|| schema_artifact_ids(resolved)),
        None => schema_artifact_ids(resolved),
    };

    for id in required_ids {
        let Some(a) = resolved.schema.artifacts.iter().find(|a| a.id == id) else {
            rep.push(error(
                "schema.validation",
                format!("Schema apply.requires references unknown artifact id '{id}'"),
            ));
            continue;
        };
        if artifact_done(&change_dir, &a.generates) {
            continue;
        }
        rep.push(warning(
            format!("artifacts.{id}"),
            format!(
                "Apply-required artifact '{id}' is missing (expected output: {})",
                a.generates
            ),
        ));
    }
}

fn resolve_validation_context(ito_path: &Path, change_id: &str) -> (ConfigContext, String) {
    let schema_name = read_change_schema(ito_path, change_id);

    let mut ctx = ConfigContext::from_process_env();
    ctx.project_dir = ito_path.parent().map(|p| p.to_path_buf());

    (ctx, schema_name)
}

fn validate_change_against_schema_validation(
    rep: &mut ReportBuilder,
    change_repo: &impl DomainChangeRepository,
    ito_path: &Path,
    change_id: &str,
    resolved: &ResolvedSchema,
    validation: &ValidationYaml,
    strict: bool,
) -> CoreResult<()> {
    let change_dir = paths::change_dir(ito_path, change_id);

    let missing_level = validation
        .defaults
        .missing_required_artifact_level
        .unwrap_or(ValidationLevelYaml::Warning)
        .as_level_str();

    for (artifact_id, cfg) in &validation.artifacts {
        let Some(schema_artifact) = resolved
            .schema
            .artifacts
            .iter()
            .find(|a| a.id == *artifact_id)
        else {
            rep.push(error(
                "schema.validation",
                format!("validation.yaml references unknown artifact id '{artifact_id}'"),
            ));
            continue;
        };

        let present = artifact_done(&change_dir, &schema_artifact.generates);
        if cfg.required && !present {
            rep.push(issue(
                missing_level,
                format!("artifacts.{artifact_id}"),
                format!(
                    "Missing required artifact '{artifact_id}' (expected output: {})",
                    schema_artifact.generates
                ),
            ));
        }

        if !present {
            if let Some(validator_id @ ValidatorId::DeltaSpecsV1) = cfg.validate_as {
                // Only delta-spec validation runs without a generated artifact because it
                // validates change-wide state; tasks-tracking validation is file-backed.
                let ctx = ArtifactValidatorContext {
                    ito_path,
                    change_id,
                    strict,
                };
                run_validator_for_artifact(
                    rep,
                    change_repo,
                    ctx,
                    artifact_id,
                    &schema_artifact.generates,
                    validator_id,
                )?;
            }
            continue;
        }

        let Some(validator_id) = cfg.validate_as else {
            continue;
        };
        let ctx = ArtifactValidatorContext {
            ito_path,
            change_id,
            strict,
        };
        run_validator_for_artifact(
            rep,
            change_repo,
            ctx,
            artifact_id,
            &schema_artifact.generates,
            validator_id,
        )?;
    }

    if let Some(tracking) = validation.tracking.as_ref() {
        match tracking.source {
            crate::templates::ValidationTrackingSourceYaml::ApplyTracks => {
                let tracks_rel = resolved
                    .schema
                    .apply
                    .as_ref()
                    .and_then(|a| a.tracks.as_deref());

                let Some(tracks_rel) = tracks_rel else {
                    if tracking.required {
                        rep.push(error(
                            "tracking",
                            "Schema tracking is required but schema apply.tracks is not set",
                        ));
                    }
                    return Ok(());
                };

                if !ito_domain::tasks::is_safe_tracking_filename(tracks_rel) {
                    rep.push(error(
                        "tracking",
                        format!("Invalid tracking file path in apply.tracks: '{tracks_rel}'"),
                    ));
                    return Ok(());
                }

                let report_path = format!("changes/{change_id}/{tracks_rel}");
                let abs_path = paths::change_dir(ito_path, change_id).join(tracks_rel);

                let present = abs_path.exists();
                if tracking.required && !present {
                    rep.push(error(
                        "tracking",
                        format!("Missing required tracking file: {report_path}"),
                    ));
                }
                if !present {
                    return Ok(());
                }

                match tracking.validate_as {
                    ValidatorId::TasksTrackingV1 => {
                        rep.extend(validate_tasks_tracking_path(
                            &abs_path,
                            &report_path,
                            strict,
                        ));
                    }
                    ValidatorId::DeltaSpecsV1 => {
                        rep.push(error(
                            "schema.validation",
                            "Validator 'ito.delta-specs.v1' is not valid for tracking files",
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

fn run_validator_for_artifact(
    rep: &mut ReportBuilder,
    change_repo: &impl DomainChangeRepository,
    ctx: ArtifactValidatorContext<'_>,
    artifact_id: &str,
    generates: &str,
    validator_id: ValidatorId,
) -> CoreResult<()> {
    match validator_id {
        ValidatorId::DeltaSpecsV1 => {
            validate_change_delta_specs(rep, change_repo, ctx.change_id)?;
        }
        ValidatorId::TasksTrackingV1 => {
            use format_specs::TASKS_TRACKING_V1;

            if generates.contains('*') {
                rep.push(with_format_spec(
                    error(
                        format!("artifacts.{artifact_id}"),
                        format!(
                            "Validator '{}' requires a single file path; got pattern '{}'",
                            TASKS_TRACKING_V1.validator_id, generates
                        ),
                    ),
                    TASKS_TRACKING_V1,
                ));
                return Ok(());
            }

            let report_path = format!("changes/{}/{generates}", ctx.change_id);
            let abs_path = paths::change_dir(ctx.ito_path, ctx.change_id).join(generates);
            rep.extend(validate_tasks_tracking_path(
                &abs_path,
                &report_path,
                ctx.strict,
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct ArtifactValidatorContext<'a> {
    ito_path: &'a Path,
    change_id: &'a str,
    strict: bool,
}

fn validate_tasks_tracking_path(
    path: &Path,
    report_path: &str,
    strict: bool,
) -> Vec<ValidationIssue> {
    use format_specs::TASKS_TRACKING_V1;
    use ito_domain::tasks::{parse_tasks_tracking_file, DiagnosticLevel};

    let contents = match ito_common::io::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return vec![with_format_spec(
                error(report_path, format!("Failed to read {report_path}: {e}")),
                TASKS_TRACKING_V1,
            )];
        }
    };

    let parsed = parse_tasks_tracking_file(&contents);
    let mut issues = Vec::new();

    if parsed.tasks.is_empty() {
        let msg = "Tracking file contains no recognizable tasks";
        let i = if strict {
            error(report_path, msg)
        } else {
            warning(report_path, msg)
        };
        issues.push(with_format_spec(i, TASKS_TRACKING_V1));
    }
    for d in &parsed.diagnostics {
        let level = match d.level {
            DiagnosticLevel::Error => LEVEL_ERROR,
            DiagnosticLevel::Warning => LEVEL_WARNING,
        };
        issues.push(with_format_spec(
            ValidationIssue {
                path: report_path.to_string(),
                level: level.to_string(),
                message: d.message.clone(),
                line: d.line.map(|l| l as u32),
                column: None,
                metadata: None,
            },
            TASKS_TRACKING_V1,
        ));
    }
    issues
}

fn validate_change_delta_specs(
    rep: &mut ReportBuilder,
    change_repo: &impl DomainChangeRepository,
    change_id: &str,
) -> CoreResult<()> {
    use format_specs::DELTA_SPECS_V1;

    let files = read_change_delta_spec_files(change_repo, change_id)?;
    if files.is_empty() {
        rep.push(with_format_spec(
            error("specs", "Change must have at least one delta"),
            DELTA_SPECS_V1,
        ));
        return Ok(());
    }

    let show = parse_change_show_json(change_id, &files);
    if show.deltas.is_empty() {
        rep.push(with_format_spec(
            error("specs", "Change must have at least one delta"),
            DELTA_SPECS_V1,
        ));
        return Ok(());
    }

    if show.deltas.len() > MAX_DELTAS_PER_CHANGE {
        rep.push(with_format_spec(
            info(
                "deltas",
                "Consider splitting changes with more than 10 deltas",
            ),
            DELTA_SPECS_V1,
        ));
    }

    for (idx, d) in show.deltas.iter().enumerate() {
        let base = format!("deltas[{idx}]");
        if d.description.trim().is_empty() {
            rep.push(with_format_spec(
                error(&base, "Delta description cannot be empty"),
                DELTA_SPECS_V1,
            ));
        } else if d.description.trim().len() < 20 {
            rep.push(with_format_spec(
                warning(&base, "Delta description is too brief"),
                DELTA_SPECS_V1,
            ));
        }

        if d.requirements.is_empty() {
            rep.push(with_format_spec(
                warning(&base, "Delta should include requirements"),
                DELTA_SPECS_V1,
            ));
        }

        for (ridx, req) in d.requirements.iter().enumerate() {
            let rp = format!("{base}.requirements[{ridx}]");
            if req.text.trim().is_empty() {
                rep.push(with_format_spec(
                    error(&rp, "Requirement text cannot be empty"),
                    DELTA_SPECS_V1,
                ));
            }
            let up = req.text.to_ascii_uppercase();
            if !up.contains("SHALL") && !up.contains("MUST") {
                rep.push(with_format_spec(
                    error(&rp, "Requirement must contain SHALL or MUST keyword"),
                    DELTA_SPECS_V1,
                ));
            }
            if req.scenarios.is_empty() {
                rep.push(with_format_spec(
                    error(&rp, "Requirement must have at least one scenario"),
                    DELTA_SPECS_V1,
                ));
            }
        }
    }
    Ok(())
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
pub fn resolve_module(
    module_repo: &impl DomainModuleRepository,
    _ito_path: &Path,
    input: &str,
) -> CoreResult<Option<ResolvedModule>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let module = module_repo.get(trimmed).into_core();
    match module {
        Ok(m) => {
            let full_name = format!("{}_{}", m.id, m.name);
            let module_dir = m.path;
            let module_md = module_dir.join("module.md");
            Ok(Some(ResolvedModule {
                id: m.id,
                full_name,
                module_dir,
                module_md,
            }))
        }
        Err(_) => Ok(None),
    }
}

/// Validate a module's `module.md` for minimal required sections.
///
/// Returns the resolved module directory name along with the report.
pub fn validate_module(
    module_repo: &impl DomainModuleRepository,
    ito_path: &Path,
    module_input: &str,
    strict: bool,
) -> CoreResult<(String, ValidationReport)> {
    let resolved = resolve_module(module_repo, ito_path, module_input)?;
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

/// Validate a change's tasks.md file and return any issues found.
pub fn validate_tasks_file(
    ito_path: &Path,
    change_id: &str,
    strict: bool,
) -> CoreResult<Vec<ValidationIssue>> {
    use crate::templates::{load_schema_validation, read_change_schema, resolve_schema};
    use ito_domain::tasks::tasks_path_checked;

    // `read_change_schema` uses `change_id` as a path segment; reject traversal.
    if tasks_path_checked(ito_path, change_id).is_none() {
        return Ok(vec![error(
            "tracking",
            format!("invalid change id path segment: \"{change_id}\""),
        )]);
    }

    let schema_name = read_change_schema(ito_path, change_id);
    let mut ctx = ConfigContext::from_process_env();
    ctx.project_dir = ito_path.parent().map(|p| p.to_path_buf());

    let mut issues: Vec<ValidationIssue> = Vec::new();

    let mut tracking_file = "tasks.md".to_string();
    let resolved = match resolve_schema(Some(&schema_name), &ctx) {
        Ok(r) => Some(r),
        Err(e) => {
            issues.push(error(
                "schema",
                format!("Failed to resolve schema '{schema_name}': {e}"),
            ));
            None
        }
    };

    if let Some(resolved) = resolved.as_ref() {
        // If schema validation declares a non-tasks tracking validator, this file is not a
        // tasks-tracking file that `ito validate` can interpret.
        if let Ok(Some(validation)) = load_schema_validation(resolved)
            && let Some(tracking) = validation.tracking.as_ref()
            && tracking.validate_as != ValidatorId::TasksTrackingV1
        {
            issues.push(error(
                "tracking",
                format!(
                    "Schema tracking validator '{}' is not valid for tasks tracking files",
                    tracking.validate_as.as_str()
                ),
            ));
            return Ok(issues);
        }

        if let Some(tracks) = resolved
            .schema
            .apply
            .as_ref()
            .and_then(|a| a.tracks.as_deref())
        {
            tracking_file = tracks.to_string();
        }
    }

    if !ito_domain::tasks::is_safe_tracking_filename(&tracking_file) {
        issues.push(error(
            "tracking",
            format!("Invalid tracking file path in apply.tracks: '{tracking_file}'"),
        ));
        return Ok(issues);
    }

    let path = paths::change_dir(ito_path, change_id).join(&tracking_file);
    let report_path = format!("changes/{change_id}/{tracking_file}");
    issues.extend(validate_tasks_tracking_path(&path, &report_path, strict));
    Ok(issues)
}
