use super::{
    PeerReviewContext, ReviewAffectedSpecInfo, ReviewArtifactInfo, ReviewCoveredRequirement,
    ReviewTaskSummaryInfo, ReviewTestingPolicy, ReviewTraceabilityInfo, ReviewUnresolvedReference,
    ReviewValidationIssueInfo, TemplatesError, artifact_done, default_schema_name,
    load_composed_user_guidance, read_change_schema, resolve_schema, validate_change_name_input,
};
use crate::show::{parse_change_show_json, read_change_delta_spec_files};
use crate::validate::validate_change;
use chrono::{SecondsFormat, Utc};
use ito_common::paths;
use ito_config::{ConfigContext, load_cascading_project_config};
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use std::collections::BTreeSet;
use std::path::Path;

/// Builds the PeerReviewContext used to render the agent review template.
///
/// The returned context aggregates change metadata, resolved schema artifacts,
/// validation issues, task progress summary, affected spec deltas and — when
/// requirement IDs are present — computed traceability information. Returns an
/// error when the change name is invalid, the change cannot be found, schema
/// resolution fails, or other template-preparation steps fail.
///
/// # Returns
///
/// `Ok(PeerReviewContext)` with the assembled review context on success, or
/// `Err(TemplatesError)` describing why the context could not be produced.
///
/// # Examples
///
/// ```no_run
/// // Prepare repositories, path and config context (omitted)
/// // let change_repo = ...;
/// // let module_repo = ...;
/// // let ito_path = std::path::Path::new("/path/to/ito");
/// // let ctx = ConfigContext::default();
/// let result = compute_review_context(&change_repo, &module_repo, ito_path, "changename", None, &ctx);
/// match result {
///     Ok(ctx) => println!("Generated review context for {}", ctx.change_name),
///     Err(err) => eprintln!("Failed to build review context: {:?}", err),
/// }
/// ```
pub fn compute_review_context(
    change_repo: &(impl ChangeRepository + ?Sized),
    module_repo: &(impl ModuleRepository + ?Sized),
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

    let (validation_passed, validation_issues) =
        match validate_change(change_repo, ito_path, change, false) {
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
    let traceability = if let Ok(delta_files) = read_change_delta_spec_files(change_repo, change) {
        let show = parse_change_show_json(change, &delta_files);

        for delta in &show.deltas {
            if delta.operation != "MODIFIED" {
                continue;
            }
            let description = if delta.description.trim().is_empty() {
                None
            } else {
                Some(delta.description.clone())
            };
            affected_specs.push(ReviewAffectedSpecInfo {
                spec_id: delta.spec.clone(),
                operation: delta.operation.clone(),
                description,
            });
        }

        // Collect (title, id) pairs from all delta requirements.
        let mut delta_requirements: Vec<(String, Option<String>)> = Vec::new();
        for d in &show.deltas {
            for req in &d.requirements {
                delta_requirements.push((req.text.clone(), req.requirement_id.clone()));
            }
        }

        let has_any_id = delta_requirements.iter().any(|(_, id)| id.is_some());
        if has_any_id {
            let trace_result = ito_domain::traceability::compute_traceability(
                &delta_requirements,
                &change_data.tasks,
            );

            let (status, reason) = match &trace_result.status {
                ito_domain::traceability::TraceStatus::Ready => ("ready".to_string(), None),
                ito_domain::traceability::TraceStatus::Invalid { missing_ids } => {
                    let reason = format!("Requirements missing IDs: {}", missing_ids.join(", "));
                    ("invalid".to_string(), Some(reason))
                }
                ito_domain::traceability::TraceStatus::Unavailable { reason } => {
                    ("unavailable".to_string(), Some(reason.clone()))
                }
            };

            let mut covered_requirements = Vec::new();
            for covered in &trace_result.covered_requirements {
                covered_requirements.push(ReviewCoveredRequirement {
                    requirement_id: covered.requirement_id.clone(),
                    covering_tasks: covered.covering_tasks.clone(),
                });
            }

            let mut unresolved_references = Vec::new();
            for unresolved in &trace_result.unresolved_references {
                unresolved_references.push(ReviewUnresolvedReference {
                    task_id: unresolved.task_id.clone(),
                    requirement_id: unresolved.requirement_id.clone(),
                });
            }

            Some(ReviewTraceabilityInfo {
                status,
                reason,
                declared_requirements: trace_result.declared_requirements.clone(),
                covered_requirements,
                uncovered_requirements: trace_result.uncovered_requirements.clone(),
                unresolved_references,
                diagnostics: trace_result.diagnostics.clone(),
            })
        } else {
            None
        }
    } else {
        None
    };

    let module_id = change_data.module_id.clone();
    let module_name = if let Some(id) = module_id.as_deref() {
        module_repo.get(id).ok().map(|module| module.name)
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
        traceability,
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
