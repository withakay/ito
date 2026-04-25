//! Validate Ito repository artifacts.
//!
//! This module provides lightweight validation helpers for specs, changes, and
//! modules.
//!
//! The primary consumer is the CLI and any APIs that need a structured report
//! (`ValidationReport`) rather than a single error.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use regex::Regex;
use serde::Serialize;

use ito_common::fs::StdFs;
use ito_common::paths;

use crate::show::{
    parse_change_show_json, parse_spec_show_json, read_change_delta_spec_files,
    read_change_proposal_markdown,
};
use crate::templates::{
    ResolvedSchema, ValidationLevelYaml, ValidationYaml, ValidatorId, artifact_done,
    load_schema_validation, read_change_schema, resolve_schema,
};
use ito_config::ConfigContext;
use ito_domain::changes::ChangeRepository as DomainChangeRepository;
use ito_domain::modules::ModuleRepository as DomainModuleRepository;

mod format_specs;
mod issue;
mod repo_integrity;
mod report;

pub(crate) use issue::with_format_spec;
pub use issue::{error, info, issue, warning, with_line, with_loc, with_metadata, with_rule_id};
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
const MAX_SCENARIO_STEPS: usize = 8;
const DELTA_SPECS_ARTIFACT_RULES: &[&str] = &["contract_refs", "scenario_grammar", "ui_mechanics"];
const DELTA_SPECS_PROPOSAL_RULES: &[&str] = &["capabilities_consistency"];
const TASKS_TRACKING_RULES: &[&str] = &["task_quality"];

static UI_MECHANICS_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"(?i)\bclick\s+(?:on\s+|the\s+)?\w+",
        r"(?i)\bwait\s+\d+\s*(?:ms|millisecond|second|s)\b",
        r"(?i)\bsleep\s+\d+\b",
        r"(?i)\bselector\s*[:=]",
        r"(?i)\bcss\s+selector\b",
    ]
    .into_iter()
    .map(|pattern| Regex::new(pattern).expect("valid UI mechanics regex"))
    .collect()
});

static INLINE_CODE_TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`([^`]+)`").expect("valid inline code regex"));

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
    /// Optional rule id when the issue came from an opt-in rule.
    pub rule_id: Option<String>,
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

/// Validate a change and produce a ValidationReport describing any issues found.
///
/// The function resolves the change's schema (when available) and runs schema-driven
/// validation if the schema provides a validation.yaml. For legacy delta-driven
/// schemas or when schema resolution/validation is unavailable it falls back to
/// delta-specs and tasks-tracking validations. The `strict` flag influences severity
/// handling and the report's final `valid` value.
///
/// # Returns
///
/// A `ValidationReport` summarizing all discovered issues and an aggregate summary,
/// wrapped in `CoreResult`. The error variant is used for IO or repository access
/// failures that prevent performing the validations.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// // let change_repo = ...; // impl DomainChangeRepository
/// // let report = validate_change(&change_repo, Path::new("/path/to/ito"), "change-123", true).unwrap();
/// // println!("Valid: {}", report.valid);
/// ```
pub fn validate_change(
    change_repo: &(impl DomainChangeRepository + ?Sized),
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
            validate_change_delta_specs(&mut rep, change_repo, change_id, strict)?;

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

    validate_change_delta_specs(&mut rep, change_repo, change_id, strict)?;
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
    change_repo: &(impl DomainChangeRepository + ?Sized),
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
                run_configured_rules(
                    rep,
                    change_repo,
                    ctx,
                    validator_id,
                    ValidationRuleTarget::Artifact { id: artifact_id },
                    cfg.rules.as_ref(),
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
        run_configured_rules(
            rep,
            change_repo,
            ctx,
            validator_id,
            ValidationRuleTarget::Artifact { id: artifact_id },
            cfg.rules.as_ref(),
        )?;
    }

    if let Some(proposal) = validation.proposal.as_ref() {
        let report_path = format!("changes/{change_id}/proposal.md");
        let abs_path = change_dir.join("proposal.md");
        let present = abs_path.exists();

        if proposal.required && !present {
            rep.push(issue(
                missing_level,
                "proposal",
                format!("Missing required proposal artifact: {report_path}"),
            ));
        }

        if present {
            if let Some(validator_id) = proposal.validate_as {
                let ctx = ArtifactValidatorContext {
                    ito_path,
                    change_id,
                    strict,
                };
                match validator_id {
                    ValidatorId::DeltaSpecsV1 => {
                        run_configured_rules(
                            rep,
                            change_repo,
                            ctx,
                            validator_id,
                            ValidationRuleTarget::Proposal,
                            proposal.rules.as_ref(),
                        )?;
                    }
                    ValidatorId::TasksTrackingV1 => {
                        rep.push(error(
                            "schema.validation",
                            "Validator 'ito.tasks-tracking.v1' is not valid for proposal artifacts",
                        ));
                    }
                }
            }
        }
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
                        let ctx = ArtifactValidatorContext {
                            ito_path,
                            change_id,
                            strict,
                        };
                        run_configured_rules(
                            rep,
                            change_repo,
                            ctx,
                            ValidatorId::TasksTrackingV1,
                            ValidationRuleTarget::Tracking {
                                path: &abs_path,
                                report_path: &report_path,
                            },
                            tracking.rules.as_ref(),
                        )?;
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

#[derive(Debug, Clone, Copy)]
enum ValidationRuleTarget<'a> {
    Artifact { id: &'a str },
    Proposal,
    Tracking { path: &'a Path, report_path: &'a str },
}

impl ValidationRuleTarget<'_> {
    fn config_path(self, rule_name: &str) -> String {
        match self {
            ValidationRuleTarget::Artifact { id } => {
                format!("schema.validation.artifacts.{id}.rules.{rule_name}")
            }
            ValidationRuleTarget::Proposal => {
                format!("schema.validation.proposal.rules.{rule_name}")
            }
            ValidationRuleTarget::Tracking { .. } => {
                format!("schema.validation.tracking.rules.{rule_name}")
            }
        }
    }

    fn label(self) -> String {
        match self {
            ValidationRuleTarget::Artifact { id } => format!("artifact '{id}'"),
            ValidationRuleTarget::Proposal => "proposal artifact".to_string(),
            ValidationRuleTarget::Tracking { report_path, .. } => {
                format!("tracking file '{report_path}'")
            }
        }
    }
}

fn format_spec_for_validator(
    validator_id: ValidatorId,
) -> crate::validate::format_specs::FormatSpecRef {
    match validator_id {
        ValidatorId::DeltaSpecsV1 => format_specs::DELTA_SPECS_V1,
        ValidatorId::TasksTrackingV1 => format_specs::TASKS_TRACKING_V1,
    }
}

fn rule_issue(
    validator_id: ValidatorId,
    rule_name: &str,
    level: ValidationLevel,
    path: impl AsRef<str>,
    message: impl Into<String>,
) -> ValidationIssue {
    with_format_spec(
        with_rule_id(issue(level, path, message), rule_name),
        format_spec_for_validator(validator_id),
    )
}

fn run_configured_rules(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    validator_id: ValidatorId,
    target: ValidationRuleTarget<'_>,
    rules: Option<&BTreeMap<String, ValidationLevelYaml>>,
) -> CoreResult<()> {
    let Some(rules) = rules else {
        return Ok(());
    };

    for (rule_name, level) in rules {
        let supported_rules = supported_rules_for_target(validator_id, target);
        if !supported_rules.iter().any(|supported| supported == rule_name) {
            let supported = if supported_rules.is_empty() {
                "none".to_string()
            } else {
                supported_rules.join(", ")
            };
            rep.push(with_format_spec(
                warning(
                    target.config_path(rule_name),
                    format!(
                        "Unknown validation rule '{rule_name}' for {} (validator: {}). Supported rules: {supported}",
                        target.label(),
                        validator_id.as_str()
                    ),
                ),
                format_spec_for_validator(validator_id),
            ));
            continue;
        }

        match (validator_id, target) {
            (ValidatorId::DeltaSpecsV1, ValidationRuleTarget::Artifact { id: "specs" }) => {
                run_delta_specs_artifact_rule(rep, change_repo, ctx, rule_name, *level)?;
            }
            (ValidatorId::DeltaSpecsV1, ValidationRuleTarget::Proposal) => {
                run_delta_specs_proposal_rule(rep, change_repo, ctx, rule_name, *level)?;
            }
            (
                ValidatorId::TasksTrackingV1,
                ValidationRuleTarget::Tracking { path, report_path },
            ) => {
                run_tasks_tracking_rule(rep, change_repo, ctx, path, report_path, rule_name, *level)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn supported_rules_for_target(
    validator_id: ValidatorId,
    target: ValidationRuleTarget<'_>,
) -> &'static [&'static str] {
    match (validator_id, target) {
        (ValidatorId::DeltaSpecsV1, ValidationRuleTarget::Artifact { id: "specs" }) => {
            DELTA_SPECS_ARTIFACT_RULES
        }
        (ValidatorId::DeltaSpecsV1, ValidationRuleTarget::Proposal) => {
            DELTA_SPECS_PROPOSAL_RULES
        }
        (ValidatorId::TasksTrackingV1, ValidationRuleTarget::Tracking { .. }) => {
            TASKS_TRACKING_RULES
        }
        _ => &[],
    }
}

fn run_delta_specs_artifact_rule(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    rule_name: &str,
    level: ValidationLevelYaml,
) -> CoreResult<()> {
    match rule_name {
        "scenario_grammar" => rep.extend(validate_scenario_grammar_rule(
            change_repo,
            ctx.change_id,
            level,
        )?),
        "ui_mechanics" => rep.extend(validate_ui_mechanics_rule(change_repo, ctx.change_id)?),
        "contract_refs" => {}
        _ => {}
    }
    Ok(())
}

fn run_delta_specs_proposal_rule(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    rule_name: &str,
    level: ValidationLevelYaml,
) -> CoreResult<()> {
    match rule_name {
        "capabilities_consistency" => rep.extend(validate_capabilities_consistency_rule(
            change_repo,
            ctx.ito_path,
            ctx.change_id,
            level,
        )?),
        _ => {}
    }
    Ok(())
}

fn run_tasks_tracking_rule(
    _rep: &mut ReportBuilder,
    _change_repo: &(impl DomainChangeRepository + ?Sized),
    _ctx: ArtifactValidatorContext<'_>,
    _path: &Path,
    _report_path: &str,
    rule_name: &str,
    _level: ValidationLevelYaml,
) -> CoreResult<()> {
    match rule_name {
        "task_quality" => Ok(()),
        _ => Ok(()),
    }
}

fn validate_scenario_grammar_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let show = parse_change_show_json(change_id, &read_change_delta_spec_files(change_repo, change_id)?);
    let mut issues = Vec::new();

    for (delta_idx, delta) in show.deltas.iter().enumerate() {
        for (requirement_idx, requirement) in delta.requirements.iter().enumerate() {
            for (scenario_idx, scenario) in requirement.scenarios.iter().enumerate() {
                if scenario.raw_text.trim().is_empty() {
                    continue;
                }

                let steps = extract_scenario_steps(&scenario.raw_text);
                let has_given = steps.iter().any(|step| step.keyword == "GIVEN");
                let has_when = steps.iter().any(|step| step.keyword == "WHEN");
                let has_then = steps.iter().any(|step| step.keyword == "THEN");
                let path = format!(
                    "deltas[{delta_idx}].requirements[{requirement_idx}].scenarios[{scenario_idx}]"
                );

                if !has_when {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        level.as_level_str(),
                        &path,
                        "Scenario is missing WHEN step",
                    ));
                }
                if !has_then {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        level.as_level_str(),
                        &path,
                        "Scenario is missing THEN step",
                    ));
                }
                if !has_given {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        LEVEL_WARNING,
                        &path,
                        "Scenario is missing GIVEN step",
                    ));
                }
                if steps.len() > MAX_SCENARIO_STEPS {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        LEVEL_WARNING,
                        &path,
                        format!(
                            "Scenario has more than {MAX_SCENARIO_STEPS} steps; consider splitting it"
                        ),
                    ));
                }
            }
        }
    }

    Ok(issues)
}

fn validate_ui_mechanics_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
) -> CoreResult<Vec<ValidationIssue>> {
    let show = parse_change_show_json(change_id, &read_change_delta_spec_files(change_repo, change_id)?);
    let mut issues = Vec::new();

    for (delta_idx, delta) in show.deltas.iter().enumerate() {
        for (requirement_idx, requirement) in delta.requirements.iter().enumerate() {
            if requirement.tags.iter().any(|tag| tag == "ui") {
                continue;
            }

            for (scenario_idx, scenario) in requirement.scenarios.iter().enumerate() {
                if scenario.raw_text.trim().is_empty() {
                    continue;
                }

                let Some(pattern) = UI_MECHANICS_PATTERNS
                    .iter()
                    .find(|pattern| pattern.is_match(&scenario.raw_text))
                else {
                    continue;
                };

                issues.push(rule_issue(
                    ValidatorId::DeltaSpecsV1,
                    "ui_mechanics",
                    LEVEL_WARNING,
                    format!(
                        "deltas[{delta_idx}].requirements[{requirement_idx}].scenarios[{scenario_idx}]"
                    ),
                    format!(
                        "Scenario may be describing UI mechanics rather than behavior (matched pattern: {})",
                        pattern.as_str()
                    ),
                ));
            }
        }
    }

    Ok(issues)
}

#[derive(Debug, Clone)]
struct ScenarioStep {
    keyword: &'static str,
}

fn extract_scenario_steps(raw_text: &str) -> Vec<ScenarioStep> {
    raw_text
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let upper = line.to_ascii_uppercase();
            if upper.starts_with("- **GIVEN**") {
                return Some(ScenarioStep { keyword: "GIVEN" });
            }
            if upper.starts_with("- **WHEN**") {
                return Some(ScenarioStep { keyword: "WHEN" });
            }
            if upper.starts_with("- **THEN**") {
                return Some(ScenarioStep { keyword: "THEN" });
            }
            if upper.starts_with("- **AND**") {
                return Some(ScenarioStep { keyword: "AND" });
            }
            None
        })
        .collect()
}

fn validate_capabilities_consistency_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ito_path: &Path,
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let Some(proposal) = read_change_proposal_markdown(change_repo, change_id)? else {
        return Ok(Vec::new());
    };

    let parsed = parse_proposal_capabilities(&proposal);
    let delta_specs = read_change_delta_spec_files(change_repo, change_id)?;
    let delta_names: BTreeSet<String> = delta_specs.into_iter().map(|spec| spec.spec).collect();
    let baseline_names: BTreeSet<String> =
        ito_domain::discovery::list_spec_dir_names(&StdFs, ito_path).into_core()?.into_iter().collect();

    let mut issues = Vec::new();
    for warning_message in parsed.warnings {
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "capabilities_consistency",
            LEVEL_WARNING,
            "proposal.capabilities",
            warning_message,
        ));
    }

    for capability in &parsed.new_capabilities {
        if !delta_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed in the proposal but no delta spec exists at specs/{capability}/spec.md"
                ),
            ));
        }
        if baseline_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed as new but already exists in .ito/specs/{capability}/"
                ),
            ));
        }
    }

    for capability in &parsed.modified_capabilities {
        if !delta_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed in the proposal but no delta spec exists at specs/{capability}/spec.md"
                ),
            ));
        }
        if !baseline_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed as modified but no baseline spec exists in .ito/specs/{capability}/"
                ),
            ));
        }
    }

    let declared: BTreeSet<String> = parsed
        .new_capabilities
        .iter()
        .chain(parsed.modified_capabilities.iter())
        .cloned()
        .collect();
    for capability in delta_names {
        if declared.contains(&capability) {
            continue;
        }
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "capabilities_consistency",
            level.as_level_str(),
            "proposal.capabilities",
            format!("Delta capability '{capability}' is not listed in the proposal"),
        ));
    }

    Ok(issues)
}

#[derive(Debug, Default)]
struct ParsedProposalCapabilities {
    new_capabilities: Vec<String>,
    modified_capabilities: Vec<String>,
    warnings: Vec<String>,
}

fn parse_proposal_capabilities(markdown: &str) -> ParsedProposalCapabilities {
    let mut parsed = ParsedProposalCapabilities::default();
    let mut in_capabilities = false;

    enum CapabilitySection {
        None,
        New,
        Modified,
    }

    let mut section = CapabilitySection::None;
    for line in markdown.lines() {
        let line = line.trim_end();
        let trimmed = line.trim();

        if let Some(title) = trimmed.strip_prefix("## ") {
            in_capabilities = title.trim().eq_ignore_ascii_case("Capabilities");
            if !in_capabilities {
                section = CapabilitySection::None;
            }
            continue;
        }
        if !in_capabilities {
            continue;
        }

        if let Some(title) = trimmed.strip_prefix("### ") {
            section = if title.trim().eq_ignore_ascii_case("New Capabilities") {
                CapabilitySection::New
            } else if title.trim().eq_ignore_ascii_case("Modified Capabilities") {
                CapabilitySection::Modified
            } else {
                CapabilitySection::None
            };
            continue;
        }

        let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
            .map(str::trim)
        else {
            continue;
        };
        if rest.is_empty() || rest.starts_with("<!--") {
            continue;
        }

        let Some(capability) = extract_first_inline_code_token(rest) else {
            parsed.warnings.push(format!(
                "Capability bullet is missing an inline-code token: {rest}"
            ));
            continue;
        };
        if capability.starts_with('<') && capability.ends_with('>') {
            continue;
        }

        match section {
            CapabilitySection::New => parsed.new_capabilities.push(capability),
            CapabilitySection::Modified => parsed.modified_capabilities.push(capability),
            CapabilitySection::None => {}
        }
    }

    parsed
}

fn extract_first_inline_code_token(line: &str) -> Option<String> {
    let captures = INLINE_CODE_TOKEN_RE.captures(line)?;
    let token = captures.get(1)?.as_str().trim();
    if token.is_empty() {
        return None;
    }
    Some(token.to_string())
}

/// Dispatches and runs the appropriate artifact validator, extending `rep` with any issues found.
///
/// This function selects a validator by `validator_id` and runs it for the artifact identified by
/// `artifact_id` and its declared `generates` outputs. Validation results are appended to
/// `rep`. Returns `Ok(())` on successful dispatch and execution of the chosen validator; validation
/// failures are reported via `rep`.
///
/// # Parameters
///
/// - `rep`: report builder to receive produced validation issues.
/// - `change_repo`: repository used by validators that need change data.
/// - `ctx`: validation context carrying `ito_path`, `change_id`, and `strict`.
/// - `artifact_id`: identifier of the artifact being validated (used in reported issue paths).
/// - `generates`: artifact's declared output pattern or path.
/// - `validator_id`: selects which validator to execute.
///
/// # Returns
///
/// `Ok(())` if the validator was dispatched and executed (or a non-fatal condition was reported
/// into `rep`); underlying repository or validation errors are propagated as `CoreResult` errors.
///
/// # Examples
///
/// ```no_run
/// // Illustrative usage (types omitted for brevity)
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # use std::path::Path;
/// // let mut rep = ReportBuilder::new(false);
/// // let change_repo = ...; // implements DomainChangeRepository
/// // let ctx = ArtifactValidatorContext { ito_path: Path::new("."), change_id: "C001", strict: true };
/// // run_validator_for_artifact(&mut rep, &change_repo, ctx, "artifactA", "outputs/tasks.md", ValidatorId::TasksTrackingV1)?;
/// # Ok(()) }
/// ```
fn run_validator_for_artifact(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    artifact_id: &str,
    generates: &str,
    validator_id: ValidatorId,
) -> CoreResult<()> {
    match validator_id {
        ValidatorId::DeltaSpecsV1 => {
            validate_change_delta_specs(rep, change_repo, ctx.change_id, ctx.strict)?;
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
    use ito_domain::tasks::{DiagnosticLevel, parse_tasks_tracking_file};

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
                rule_id: None,
                metadata: None,
            },
            TASKS_TRACKING_V1,
        ));
    }
    issues
}

/// Validate delta-spec files for a change and append any findings to the provided report.
///
/// This function reads the change's delta spec files, performs structural and content checks
/// (descriptions, requirements, scenarios, and size limits), and runs traceability analysis
/// against the change's tasks when at least one requirement exposes an ID. Validation issues
/// are pushed into `rep`. Returns an error only for underlying repository or IO failures.
///
/// # Parameters
///
/// - `rep`: report builder to receive validation issues.
/// - `change_repo`: repository used to read change data (delta spec files and change tasks).
/// - `change_id`: identifier of the change to validate.
/// - `strict`: when `true`, uncovered requirements from traceability are reported as errors;
///   when `false`, they are reported as warnings.
///
/// # Examples
///
/// ```
/// // Setup placeholders appropriate for your test harness:
/// // let mut rep = ReportBuilder::new(false);
/// // let change_repo = MyChangeRepo::new(...);
/// // let change_id = "CHG-001";
/// // assert!(validate_change_delta_specs(&mut rep, &change_repo, change_id, true).is_ok());
/// ```
fn validate_change_delta_specs(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    strict: bool,
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

    // --- Traceability validation ---
    // Collect (title, id) pairs from all delta requirements.
    let mut delta_requirements: Vec<(String, Option<String>)> = Vec::new();
    for d in &show.deltas {
        for req in &d.requirements {
            delta_requirements.push((req.text.clone(), req.requirement_id.clone()));
        }
    }

    // Only run traceability if at least one requirement has an ID.
    let has_any_id = delta_requirements.iter().any(|(_, id)| id.is_some());
    if has_any_id {
        let change_data = change_repo.get(change_id).into_core()?;
        let trace_result =
            ito_domain::traceability::compute_traceability(&delta_requirements, &change_data.tasks);

        match &trace_result.status {
            ito_domain::traceability::TraceStatus::Invalid { missing_ids } => {
                for title in missing_ids {
                    rep.push(with_format_spec(
                        error(
                            "traceability",
                            format!(
                                "Requirement '{}' has no Requirement ID; all requirements must have IDs for traceability",
                                title
                            ),
                        ),
                        DELTA_SPECS_V1,
                    ));
                }
            }
            ito_domain::traceability::TraceStatus::Unavailable { reason } => {
                rep.push(with_format_spec(
                    info(
                        "traceability",
                        format!("Traceability unavailable: {reason}"),
                    ),
                    DELTA_SPECS_V1,
                ));
            }
            ito_domain::traceability::TraceStatus::Ready => {
                for diag in &trace_result.diagnostics {
                    rep.push(with_format_spec(
                        error("traceability", diag.clone()),
                        DELTA_SPECS_V1,
                    ));
                }
                for unresolved in &trace_result.unresolved_references {
                    rep.push(with_format_spec(
                        error(
                            "traceability",
                            format!(
                                "Task '{}' references unknown requirement ID '{}'",
                                unresolved.task_id, unresolved.requirement_id
                            ),
                        ),
                        DELTA_SPECS_V1,
                    ));
                }
                for uncovered in &trace_result.uncovered_requirements {
                    let i = if strict {
                        error(
                            "traceability",
                            format!(
                                "Requirement '{}' is not covered by any active task",
                                uncovered
                            ),
                        )
                    } else {
                        warning(
                            "traceability",
                            format!(
                                "Requirement '{}' is not covered by any active task",
                                uncovered
                            ),
                        )
                    };
                    rep.push(with_format_spec(i, DELTA_SPECS_V1));
                }
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
    module_repo: &(impl DomainModuleRepository + ?Sized),
    ito_path: &Path,
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
            let module_dir = if m.path.as_os_str().is_empty() {
                let fallback = paths::modules_dir(ito_path).join(&full_name);
                if !fallback.exists() {
                    return Ok(None);
                }
                fallback
            } else {
                m.path
            };
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
/// Also validates all sub-modules under the module: each `sub/SS_name/`
/// directory must have a valid `module.md` with a Purpose section.
///
/// Returns the resolved module directory name along with the report.
pub fn validate_module(
    module_repo: &(impl DomainModuleRepository + ?Sized),
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

    // Validate sub-modules.
    validate_sub_modules_under_module(&mut rep, module_repo, &r.module_dir, &r.id, strict);

    Ok((r.full_name, rep.finish()))
}

/// Validate all sub-modules belonging to a parent module.
///
/// Uses the repository to iterate recognized sub-modules and validates their
/// `module.md` (presence and Purpose section). Additionally scans `sub/`
/// for any directories that the repository did not recognize — those have
/// invalid naming and are reported as errors.
fn validate_sub_modules_under_module(
    rep: &mut ReportBuilder,
    module_repo: &(impl DomainModuleRepository + ?Sized),
    module_dir: &Path,
    parent_id: &str,
    strict: bool,
) {
    let sub_dir = module_dir.join("sub");
    if !sub_dir.exists() {
        return;
    }

    // Retrieve sub-modules through the repository to avoid re-discovering
    // the same filesystem layout the repository already parsed.
    let module = match module_repo.get(parent_id) {
        Ok(m) => m,
        Err(_) => return, // Parent module not found; outer validation already handles this.
    };

    // Track which directory names the repository recognized as valid so we
    // can later flag any unrecognized entries.
    let mut recognized_dirs: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(module.sub_modules.len());

    for sm in &module.sub_modules {
        let dir_name = sm
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&sm.name)
            .to_string();
        recognized_dirs.insert(dir_name.clone());

        // Validate naming convention: sub_id must be exactly two ASCII digits.
        if sm.sub_id.len() != 2 || !sm.sub_id.bytes().all(|b| b.is_ascii_digit()) {
            rep.push(error(
                format!("sub-modules/{dir_name}"),
                format!("Sub-module directory '{dir_name}' does not follow the SS_name convention"),
            ));
            continue;
        }

        // Validate module.md presence.
        let module_md = sm.path.join("module.md");
        if !module_md.exists() {
            let level = if strict { LEVEL_ERROR } else { LEVEL_WARNING };
            rep.push(issue(
                level,
                format!("sub-modules/{dir_name}"),
                format!("Sub-module '{dir_name}' is missing module.md"),
            ));
            continue;
        }

        // Validate module.md content.
        let content = match ito_common::io::read_to_string_std(&module_md) {
            Ok(c) => c,
            Err(err) => {
                rep.push(error(
                    format!("sub-modules/{dir_name}/module.md"),
                    format!("Failed to read module.md: {err}"),
                ));
                continue;
            }
        };

        let purpose = extract_section(&content, "Purpose");
        if purpose.trim().is_empty() {
            rep.push(error(
                format!("sub-modules/{dir_name}/purpose"),
                format!("Sub-module '{dir_name}' module.md must have a Purpose section"),
            ));
        } else if purpose.trim().len() < MIN_MODULE_PURPOSE_LENGTH {
            rep.push(warning(
                format!("sub-modules/{dir_name}/purpose"),
                format!(
                    "Sub-module '{dir_name}' purpose is too brief (less than {MIN_MODULE_PURPOSE_LENGTH} characters)"
                ),
            ));
        }
    }

    // Report any sub/ entries that the repository silently skipped because
    // they do not follow the required naming convention.
    if let Ok(entries) = std::fs::read_dir(&sub_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !recognized_dirs.contains(dir_name) {
                rep.push(error(
                    format!("sub-modules/{dir_name}"),
                    format!(
                        "Sub-module directory '{dir_name}' does not follow the SS_name convention"
                    ),
                ));
            }
        }
    }
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
