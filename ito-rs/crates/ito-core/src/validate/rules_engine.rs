use std::collections::BTreeMap;
use std::path::Path;

use super::{
    ArtifactValidatorContext, CoreResult, DomainChangeRepository, ReportBuilder, ValidationIssue,
    ValidationLevel, ValidationLevelYaml, ValidatorId, delta_rules, format_specs, issue,
    tracking_rules, warning, with_format_spec, with_rule_id,
};

pub(super) fn run_artifact_rules(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    validator_id: ValidatorId,
    artifact_id: &str,
    rules: Option<&BTreeMap<String, ValidationLevelYaml>>,
) -> CoreResult<()> {
    run_configured_rules(
        rep,
        change_repo,
        ctx,
        validator_id,
        ValidationRuleTarget::Artifact { id: artifact_id },
        rules,
    )
}

pub(super) fn run_proposal_rules(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    validator_id: ValidatorId,
    rules: Option<&BTreeMap<String, ValidationLevelYaml>>,
) -> CoreResult<()> {
    run_configured_rules(
        rep,
        change_repo,
        ctx,
        validator_id,
        ValidationRuleTarget::Proposal,
        rules,
    )
}

pub(super) fn run_tracking_rules(
    rep: &mut ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    validator_id: ValidatorId,
    path: &Path,
    report_path: &str,
    rules: Option<&BTreeMap<String, ValidationLevelYaml>>,
) -> CoreResult<()> {
    run_configured_rules(
        rep,
        change_repo,
        ctx,
        validator_id,
        ValidationRuleTarget::Tracking { path, report_path },
        rules,
    )
}

#[derive(Debug, Clone, Copy)]
enum ValidationRuleTarget<'a> {
    Artifact {
        id: &'a str,
    },
    Proposal,
    Tracking {
        path: &'a Path,
        report_path: &'a str,
    },
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
        if !supported_rules
            .iter()
            .any(|supported| supported == rule_name)
        {
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
                delta_rules::run_artifact_rule(rep, change_repo, ctx, rule_name, *level)?;
            }
            (ValidatorId::DeltaSpecsV1, ValidationRuleTarget::Proposal) => {
                delta_rules::run_proposal_rule(rep, change_repo, ctx, rule_name, *level)?;
            }
            (
                ValidatorId::TasksTrackingV1,
                ValidationRuleTarget::Tracking { path, report_path },
            ) => {
                tracking_rules::run_rule(
                    rep,
                    change_repo,
                    ctx,
                    path,
                    report_path,
                    rule_name,
                    *level,
                )?;
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
            delta_rules::artifact_rules()
        }
        (ValidatorId::DeltaSpecsV1, ValidationRuleTarget::Proposal) => {
            delta_rules::proposal_rules()
        }
        (ValidatorId::TasksTrackingV1, ValidationRuleTarget::Tracking { .. }) => {
            tracking_rules::rules()
        }
        _ => &[],
    }
}

fn format_spec_for_validator(validator_id: ValidatorId) -> format_specs::FormatSpecRef {
    match validator_id {
        ValidatorId::DeltaSpecsV1 => format_specs::DELTA_SPECS_V1,
        ValidatorId::TasksTrackingV1 => format_specs::TASKS_TRACKING_V1,
    }
}

pub(super) fn rule_issue(
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
