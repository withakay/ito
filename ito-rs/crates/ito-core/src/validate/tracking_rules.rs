use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use super::rules_engine::rule_issue;
use super::{
    ArtifactValidatorContext, CoreResult, DomainChangeRepository, LEVEL_ERROR, LEVEL_WARNING,
    ValidationIssue, ValidationLevelYaml, ValidatorId, parse_change_show_json,
    read_change_delta_spec_files,
};

static IMPLEMENTATION_FILE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\.(rs|ts|tsx|js|py|go|toml|yaml|yml|json|sh)$")
        .expect("valid implementation file regex")
});

pub(super) fn rules() -> &'static [&'static str] {
    &["task_quality"]
}

pub(super) fn run_rule(
    rep: &mut super::ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    path: &std::path::Path,
    report_path: &str,
    rule_name: &str,
    level: ValidationLevelYaml,
) -> CoreResult<()> {
    if rule_name == "task_quality" {
        rep.extend(validate_task_quality_rule(
            change_repo,
            ctx.change_id,
            path,
            report_path,
            level,
        )?);
    }
    Ok(())
}

fn validate_task_quality_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    path: &std::path::Path,
    report_path: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    use ito_domain::tasks::parse_tasks_tracking_file;

    let contents = ito_common::io::read_to_string(path);
    let Ok(contents) = contents else {
        let error = contents.expect_err("read_to_string should fail when contents are unavailable");
        return Ok(vec![rule_issue(
            ValidatorId::TasksTrackingV1,
            "task_quality",
            LEVEL_ERROR,
            report_path,
            format!("Failed to read {report_path}: {error}"),
        )]);
    };
    let parsed = parse_tasks_tracking_file(&contents);
    let show = parse_change_show_json(
        change_id,
        &read_change_delta_spec_files(change_repo, change_id)?,
    );
    let known_requirement_ids: BTreeSet<String> = show
        .deltas
        .iter()
        .flat_map(|delta| delta.requirements.iter())
        .filter_map(|requirement| requirement.requirement_id.clone())
        .collect();
    let missing_status_tasks: BTreeSet<String> = parsed
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.message.contains("Invalid or missing status"))
        .filter_map(|diagnostic| diagnostic.task_id.clone())
        .collect();

    let mut issues = Vec::new();
    for task in parsed.tasks {
        if missing_status_tasks.contains(&task.id) {
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, LEVEL_ERROR),
                report_path,
                format!("Missing Status for task '{}'", task.id),
            ));
        }
        if task
            .done_when
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, LEVEL_ERROR),
                report_path,
                format!("Missing Done When for task '{}'", task.id),
            ));
        }
        if task.files.is_empty() {
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, LEVEL_WARNING),
                report_path,
                format!("Missing Files for task '{}'", task.id),
            ));
        }
        if task.action.trim().is_empty() {
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, LEVEL_WARNING),
                report_path,
                format!("Missing Action for task '{}'", task.id),
            ));
        }

        let implementation_task = task
            .files
            .iter()
            .any(|file| IMPLEMENTATION_FILE_RE.is_match(file));
        let verify = task.verify.as_deref().map(str::trim).unwrap_or("");
        if verify.is_empty() {
            let missing_verify_level = if implementation_task && task_is_active(task.status) {
                LEVEL_ERROR
            } else {
                LEVEL_WARNING
            };
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, missing_verify_level),
                report_path,
                format!("Missing Verify for task '{}'", task.id),
            ));
        } else if is_vague_verify(verify) {
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, LEVEL_WARNING),
                report_path,
                format!("Task '{}' has a Vague Verify value '{}'", task.id, verify),
            ));
        }

        for requirement_id in task.requirements {
            if known_requirement_ids.contains(&requirement_id) {
                continue;
            }
            issues.push(rule_issue(
                ValidatorId::TasksTrackingV1,
                "task_quality",
                task_quality_issue_level(level, LEVEL_ERROR),
                report_path,
                format!(
                    "Task '{}' references unknown requirement ID '{}'",
                    task.id, requirement_id
                ),
            ));
        }
    }

    Ok(issues)
}

fn task_quality_issue_level(
    configured_level: ValidationLevelYaml,
    table_level: &'static str,
) -> &'static str {
    // Opt-in rule severity is a floor: users can turn rule errors down to warnings,
    // but setting the rule to `error` does not promote advisory rows into errors.
    if configured_level == ValidationLevelYaml::Warning && table_level == LEVEL_ERROR {
        return LEVEL_WARNING;
    }
    table_level
}

fn task_is_active(status: ito_domain::tasks::TaskStatus) -> bool {
    match status {
        ito_domain::tasks::TaskStatus::Pending | ito_domain::tasks::TaskStatus::InProgress => true,
        ito_domain::tasks::TaskStatus::Complete | ito_domain::tasks::TaskStatus::Shelved => false,
    }
}

fn is_vague_verify(verify: &str) -> bool {
    [
        "run tests",
        "run the tests",
        "run all tests",
        "test it",
        "verify manually",
        "check it works",
    ]
    .iter()
    .any(|candidate| verify.trim().eq_ignore_ascii_case(candidate))
}
