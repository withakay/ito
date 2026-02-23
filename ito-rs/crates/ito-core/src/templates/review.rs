use super::{
    PeerReviewContext, ReviewAffectedSpecInfo, ReviewArtifactInfo, ReviewTaskSummaryInfo,
    ReviewTestingPolicy, ReviewValidationIssueInfo, TemplatesError, artifact_done,
    default_schema_name, load_composed_user_guidance, read_change_schema, resolve_schema,
    validate_change_name_input,
};
use crate::change_repository::FsChangeRepository;
use crate::module_repository::FsModuleRepository;
use crate::show::{parse_change_show_json, read_change_delta_spec_files};
use crate::validate::validate_change;
use chrono::{SecondsFormat, Utc};
use ito_common::paths;
use ito_config::{ConfigContext, load_cascading_project_config};
use std::collections::BTreeSet;
use std::path::Path;

/// Build the context payload used by `agent/review.md.j2`.
pub fn compute_review_context(
    ito_path: &Path,
    change: &str,
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<PeerReviewContext, TemplatesError> {
    if !validate_change_name_input(change) {
        return Err(TemplatesError::InvalidChangeName);
    }

    let chosen_schema = match schema_name {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => read_change_schema(ito_path, change),
    };

    let resolved = resolve_schema(Some(&chosen_schema), ctx)?;
    let change_dir = paths::change_dir(ito_path, change);
    if !change_dir.exists() {
        return Err(TemplatesError::ChangeNotFound(change.to_string()));
    }

    let change_repo = FsChangeRepository::new(ito_path);
    let change_data = change_repo
        .get(change)
        .map_err(|_| TemplatesError::ChangeNotFound(change.to_string()))?;

    let mut artifacts = Vec::new();
    for artifact in &resolved.schema.artifacts {
        artifacts.push(ReviewArtifactInfo {
            id: artifact.id.clone(),
            path: change_dir
                .join(&artifact.generates)
                .to_string_lossy()
                .to_string(),
            present: artifact_done(&change_dir, &artifact.generates),
        });
    }

    let (validation_passed, validation_issues) = match validate_change(&change_repo, change, false)
    {
        Ok(report) => {
            let mut issues = Vec::new();
            for issue in report.issues {
                issues.push(ReviewValidationIssueInfo {
                    level: issue.level,
                    path: issue.path,
                    message: issue.message,
                    line: issue.line,
                    column: issue.column,
                });
            }
            (report.valid, issues)
        }
        Err(err) => (
            false,
            vec![ReviewValidationIssueInfo {
                level: "error".to_string(),
                path: paths::change_dir(ito_path, change)
                    .to_string_lossy()
                    .to_string(),
                message: err.to_string(),
                line: None,
                column: None,
            }],
        ),
    };

    let wave_count = {
        let distinct_waves: BTreeSet<u32> = change_data
            .tasks
            .tasks
            .iter()
            .filter_map(|task| task.wave)
            .collect();
        if distinct_waves.is_empty() {
            change_data.tasks.waves.len()
        } else {
            distinct_waves.len()
        }
    };

    let task_summary = if change_data.tasks.progress.total == 0 {
        None
    } else {
        Some(ReviewTaskSummaryInfo {
            total: change_data.tasks.progress.total,
            complete: change_data.tasks.progress.complete,
            in_progress: change_data.tasks.progress.in_progress,
            pending: change_data.tasks.progress.pending,
            shelved: change_data.tasks.progress.shelved,
            wave_count,
        })
    };

    let mut affected_specs = Vec::new();
    if let Ok(delta_files) = read_change_delta_spec_files(&change_repo, change) {
        let show = parse_change_show_json(change, &delta_files);
        for delta in show.deltas {
            if delta.operation != "MODIFIED" {
                continue;
            }

            let description = if delta.description.trim().is_empty() {
                None
            } else {
                Some(delta.description)
            };

            affected_specs.push(ReviewAffectedSpecInfo {
                spec_id: delta.spec,
                operation: delta.operation,
                description,
            });
        }
    }

    let module_id = change_data.module_id.clone();
    let module_name = if let Some(id) = module_id.as_deref() {
        FsModuleRepository::new(ito_path)
            .get(id)
            .ok()
            .map(|module| module.name)
    } else {
        None
    };

    let user_guidance = load_composed_user_guidance(ito_path, "review").unwrap_or(None);
    let testing_policy = read_testing_policy(ito_path, ctx);

    Ok(PeerReviewContext {
        change_name: change.to_string(),
        change_dir: change_dir.to_string_lossy().to_string(),
        schema_name: if resolved.schema.name.trim().is_empty() {
            default_schema_name().to_string()
        } else {
            resolved.schema.name
        },
        module_id,
        module_name,
        artifacts,
        validation_issues,
        validation_passed,
        task_summary,
        affected_specs,
        user_guidance,
        testing_policy,
        generated_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    })
}

fn read_testing_policy(ito_path: &Path, ctx: &ConfigContext) -> ReviewTestingPolicy {
    let project_root = ctx
        .project_dir
        .as_deref()
        .unwrap_or_else(|| ito_path.parent().unwrap_or(ito_path));
    let merged_config = load_cascading_project_config(project_root, ito_path, ctx).merged;
    let tdd_workflow = merged_config
        .get("defaults")
        .and_then(|v| v.get("testing"))
        .and_then(|v| v.get("tdd"))
        .and_then(|v| v.get("workflow"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("red-green-refactor")
        .to_string();
    let coverage_target_percent = merged_config
        .get("defaults")
        .and_then(|v| v.get("testing"))
        .and_then(|v| v.get("coverage"))
        .and_then(|v| v.get("target_percent"))
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(80);

    ReviewTestingPolicy {
        tdd_workflow,
        coverage_target_percent,
    }
}
