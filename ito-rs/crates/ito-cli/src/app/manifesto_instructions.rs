use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::commands::sync::best_effort_sync_coordination;
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use chrono::{SecondsFormat, Utc};
use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;
use ito_core::coordination_worktree::CoordinationSyncOutcome;
use ito_core::git_remote::resolve_org_repo_from_config_or_remote;
use ito_core::memory::{CaptureInputs, QueryInputs, SearchInputs};
use ito_core::repo_paths::coordination_worktree_path;
use ito_core::templates as core_templates;
use ito_core::validate::validate_change;
use ito_core::{
    Change, ChangeLifecycleFilter, ChangeRepository as DomainChangeRepository,
    ChangeTargetResolution,
};
use serde_json::{Value, json};
use std::path::Path;

use super::instructions::{
    archive_instruction_config_from_merged, load_coordination_branch_settings, load_testing_policy,
    load_worktree_config, render_apply_instructions_text, render_artifact_instructions_text,
    worktree_config_from_merged_with_paths,
};
use super::memory_instructions::{
    MemoryTemplateConfig, memory_template_config_from_merged, render_memory_instruction_text,
};

const MANIFESTO_VARIANTS: &[&str] = &["light", "full"];
const MANIFESTO_PROFILES: &[&str] = &[
    "planning",
    "proposal-only",
    "review-only",
    "apply",
    "archive",
    "full",
];
const MANIFESTO_OPERATIONS: &[&str] = &[
    "proposal", "specs", "design", "tasks", "apply", "review", "archive", "finish",
];

#[derive(Debug, Clone)]
struct ManifestoRequest {
    change: Option<String>,
    variant: String,
    profile: String,
    operation: Option<String>,
}

pub(super) fn handle_manifesto_instruction(
    rt: &Runtime,
    args: &[String],
    want_json: bool,
) -> CliResult<()> {
    let request = parse_manifesto_request(args)?;
    let coordination_sync_outcome =
        best_effort_sync_coordination(rt, "before manifesto instructions");
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let ctx = rt.ctx();
    let cfg = load_cascading_project_config(project_root, ito_path, ctx);
    let typed: ItoConfig = serde_json::from_value(cfg.merged.clone()).unwrap_or_default();

    let worktree = load_worktree_config(project_root, ito_path, ctx);
    let worktree_json = json!({
        "enabled": worktree.enabled,
        "strategy": worktree_strategy_name(worktree.strategy),
        "layout_base_dir": worktree
            .layout_base_dir
            .as_ref()
            .map(|p| redact_path_value(p, project_root)),
        "layout_dir_name": redact_path_value(&worktree.layout_dir_name, project_root),
        "apply_enabled": worktree.apply_enabled,
        "apply_integration_mode": worktree.integration_mode.clone(),
        "default_branch": worktree.default_branch.clone(),
    });

    let (coord_enabled_cfg, coord_name) =
        load_coordination_branch_settings(project_root, ito_path, ctx);
    let coord_enabled =
        coord_enabled_cfg || typed.changes.coordination_branch.storage.as_str() == "worktree";
    let (org, repo) = resolve_org_repo_from_config_or_remote(project_root, &typed.backend)
        .unwrap_or_else(|| (String::new(), String::new()));
    let coord_path =
        coordination_worktree_path(&typed.changes.coordination_branch, ito_path, &org, &repo);
    let coordination_json = json!({
        "enabled": coord_enabled,
        "name": coord_name,
        "storage": typed.changes.coordination_branch.storage.as_str(),
        "worktree_path": if coord_enabled {
            Value::String(redact_path_value(coord_path.to_string_lossy().as_ref(), project_root))
        } else {
            Value::Null
        },
    });

    let memory = memory_template_config_from_merged(&cfg.merged);
    let memory_instructions = build_manifesto_memory_instructions(typed.memory.as_ref());
    let resolved_change = if let Some(change) = request.change.as_deref() {
        Some(resolve_manifesto_change(
            runtime.repositories().changes.as_ref(),
            change,
        )?)
    } else {
        None
    };

    let mut validation_status = "unknown".to_string();
    let mut rendered_instructions: Vec<Value> = Vec::new();
    // TODO(wave-2): populate from authoritative host/backend review metadata when available.
    let review_status = "unknown".to_string();

    let change_json = if let Some(change_id) = resolved_change.as_deref() {
        let is_archived = runtime
            .repositories()
            .changes
            .exists_with_filter(change_id, ChangeLifecycleFilter::Archived);
        let summary = runtime
            .repositories()
            .changes
            .get_summary_with_filter(change_id, ChangeLifecycleFilter::All)
            .map_err(to_cli_error)?;
        let change = runtime
            .repositories()
            .changes
            .get_with_filter(change_id, ChangeLifecycleFilter::All)
            .map_err(to_cli_error)?;
        let change_opt = Some(change.clone());
        let module_name = summary
            .module_id
            .as_deref()
            .and_then(|module_id| runtime.repositories().modules.get(module_id).ok())
            .map(|module| module.name);

        let change_status = if is_archived {
            None
        } else {
            Some(
                core_templates::compute_change_status(ito_path, change_id, None, ctx)
                    .map_err(to_cli_error)?,
            )
        };

        let available_artifacts = if let Some(status) = &change_status {
            status
                .artifacts
                .iter()
                .filter(|artifact| artifact.status == "done")
                .map(|artifact| artifact.id.clone())
                .collect::<Vec<_>>()
        } else {
            let mut artifacts = Vec::new();
            if change.proposal.is_some() {
                artifacts.push("proposal".to_string());
            }
            if change.design.is_some() {
                artifacts.push("design".to_string());
            }
            if !change.specs.is_empty() {
                artifacts.push("specs".to_string());
            }
            if change.tasks.progress.total > 0 {
                artifacts.push("tasks".to_string());
            }
            artifacts
        };
        let missing_artifacts = if let Some(status) = &change_status {
            status
                .artifacts
                .iter()
                .filter(|artifact| artifact.status != "done")
                .map(|artifact| artifact.id.clone())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        if is_archived {
            validation_status = "unavailable".to_string();
        } else if change.tasks.progress.total > 0 {
            let report = validate_change(
                runtime.repositories().changes.as_ref(),
                ito_path,
                change_id,
                true,
            )
            .map_err(to_cli_error)?;
            validation_status = if report.valid {
                "passed".to_string()
            } else {
                "failed".to_string()
            };
        } else {
            validation_status = "unavailable".to_string();
        }

        let state = resolve_manifesto_state(
            is_archived,
            &change_opt,
            change_status.as_ref(),
            &validation_status,
            &review_status,
        );

        if request.variant == "full" {
            rendered_instructions = render_manifesto_instruction_bodies(
                rt,
                &request,
                change_id,
                &state,
                &review_status,
            )?;
        }

        json!({
            "present": true,
            "id": change_id,
            "dir": format!("{}/changes/{}", ito_path.file_name().and_then(|s| s.to_str()).unwrap_or(".ito"), change_id),
            "schema": if let Some(status) = &change_status {
                Value::String(status.schema_name.clone())
            } else {
                Value::String("unknown".to_string())
            },
            "module_id": summary.module_id,
            "module_name": module_name,
            "available_artifacts": available_artifacts,
            "missing_artifacts": missing_artifacts,
            "state": state,
        })
    } else {
        let state = resolve_manifesto_state(false, &None, None, "unavailable", &review_status);
        if request.variant == "full" {
            rendered_instructions =
                render_manifesto_instruction_bodies(rt, &request, "", &state, &review_status)?;
        }
        json!({
            "present": false,
            "id": Value::Null,
            "dir": Value::Null,
            "schema": Value::Null,
            "module_id": Value::Null,
            "module_name": Value::Null,
            "available_artifacts": Vec::<String>::new(),
            "missing_artifacts": Vec::<String>::new(),
            "state": state,
        })
    };

    let change_state = change_json
        .get("state")
        .and_then(Value::as_str)
        .unwrap_or("no-change-selected")
        .to_string();
    let state_capsule = json!({
        "mode": "manifesto",
        "variant": request.variant,
        "capability_profile": request.profile,
        "project_path": ito_path.file_name().and_then(|s| s.to_str()).unwrap_or(".ito"),
        "change_id": resolved_change,
        "schema": change_json.get("schema").cloned().unwrap_or(Value::Null),
        "operation": request.operation,
        "artifacts": {
            "proposal": artifact_capsule_state(&change_json, "proposal"),
            "specs": artifact_capsule_state(&change_json, "specs"),
            "design": artifact_capsule_state(&change_json, "design"),
            "tasks": artifact_capsule_state(&change_json, "tasks"),
        },
        "worktree": {
            "enabled": worktree.enabled,
            "current_checkout_role": current_checkout_role(project_root, &worktree.default_branch, worktree.enabled),
            "required_before_writes": worktree.enabled,
        },
        "coordination_branch": {
            "enabled": coord_enabled,
            "storage": typed.changes.coordination_branch.storage.as_str(),
            "synced_at_generation": if coord_enabled && coordination_sync_confirmed(&coordination_sync_outcome) {
                Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true))
            } else {
                Value::Null
            },
        },
        "validation": {
            "last_known_status": validation_status,
        },
        "review_status": review_status,
    });

    let config_capsule = build_manifesto_config_capsule(
        &typed,
        project_root,
        &worktree_json,
        &coordination_json,
        &memory,
    );

    let user_guidance = core_templates::load_composed_user_guidance(ito_path, "manifesto")
        .unwrap_or(None)
        .unwrap_or_default();

    let instruction = ito_templates::instructions::render_instruction_template(
        "agent/manifesto.md.j2",
        &json!({
            "variant": request.variant,
            "mode": "manifesto",
            "capability_profile": request.profile,
            "operation": request.operation,
            "project_path": ito_path.file_name().and_then(|s| s.to_str()).unwrap_or(".ito"),
            "generated_at": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            "state_capsule_json": serde_json::to_string_pretty(&state_capsule).map_err(|e| to_cli_error(format!("serializing state capsule: {e}")))?,
            "change": change_json,
            "worktree": worktree_json,
            "coordination": coordination_json,
            "config_capsule_json": serde_json::to_string_pretty(&config_capsule).map_err(|e| to_cli_error(format!("serializing config capsule: {e}")))?,
            "memory": {
                "capture_configured": memory.capture.configured,
                "search_configured": memory.search.configured,
                "query_configured": memory.query.configured,
                "capture_instruction": memory_instructions.capture,
                "search_instruction": memory_instructions.search,
                "query_instruction": memory_instructions.query,
            },
            "user_guidance": user_guidance,
            "rendered_instructions": rendered_instructions,
        }),
    )
    .map_err(|e| to_cli_error(format!("failed to render manifesto instruction: {e}")))?;

    if want_json {
        let response = json!({
            "artifact": "manifesto",
            "variant": request.variant,
            "profile": request.profile,
            "supported_variants": MANIFESTO_VARIANTS,
            "supported_profiles": MANIFESTO_PROFILES,
            "state": change_state,
            "instruction": instruction,
        });
        let rendered = serde_json::to_string_pretty(&response)
            .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
        println!("{rendered}");
        return Ok(());
    }

    print!("{instruction}");
    Ok(())
}

fn parse_manifesto_request(args: &[String]) -> CliResult<ManifestoRequest> {
    let variant = parse_string_flag(args, "--variant").unwrap_or_else(|| "light".to_string());
    if !MANIFESTO_VARIANTS.contains(&variant.as_str()) {
        return fail(format!(
            "Invalid value for --variant ('{variant}'). Valid values: {}",
            MANIFESTO_VARIANTS.join(", ")
        ));
    }

    let profile = parse_string_flag(args, "--profile").unwrap_or_else(|| "full".to_string());
    if !MANIFESTO_PROFILES.contains(&profile.as_str()) {
        return fail(format!(
            "Invalid value for --profile ('{profile}'). Valid values: {}",
            MANIFESTO_PROFILES.join(", ")
        ));
    }

    let operation = parse_string_flag(args, "--operation");
    if variant != "full" && operation.is_some() {
        return fail("The --operation selector is only supported with --variant full");
    }
    if let Some(ref operation) = operation
        && !MANIFESTO_OPERATIONS.contains(&operation.as_str())
    {
        return fail(format!(
            "Invalid value for --operation ('{operation}'). Valid values: {}",
            MANIFESTO_OPERATIONS.join(", ")
        ));
    }

    Ok(ManifestoRequest {
        change: parse_string_flag(args, "--change"),
        variant,
        profile,
        operation,
    })
}

fn resolve_manifesto_change(
    change_repo: &dyn DomainChangeRepository,
    input: &str,
) -> CliResult<String> {
    let input = input.trim();
    if change_repo.exists_with_filter(input, ChangeLifecycleFilter::All) {
        return Ok(input.to_string());
    }

    match change_repo.resolve_target(input) {
        ChangeTargetResolution::Unique(id) => Ok(id),
        ChangeTargetResolution::Ambiguous(ids) => fail(format!(
            "Change target '{input}' is ambiguous. Matches: {}",
            ids.join(", ")
        )),
        ChangeTargetResolution::NotFound => fail(format!("Change '{input}' not found")),
    }
}

fn resolve_manifesto_state(
    is_archived: bool,
    change: &Option<Change>,
    change_status: Option<&core_templates::ChangeStatus>,
    validation_status: &str,
    review_status: &str,
) -> String {
    let Some(change) = change else {
        return "no-change-selected".to_string();
    };
    if is_archived {
        return "finished".to_string();
    }
    if review_status == "pending-approval" || review_status == "changes-requested" {
        return "review-needed".to_string();
    }
    if change.work_status().to_string() == "draft"
        || (change_status
            .is_some_and(|status| !status.apply_requires.is_empty() && !status.is_complete)
            && !change.artifacts_complete())
    {
        return "proposal-drafting".to_string();
    }
    if change.progress().in_progress > 0
        || (change.progress().complete > 0 && change.progress().complete < change.progress().total)
    {
        return "applying".to_string();
    }
    if change.progress().total > 0 && change.progress().complete == change.progress().total {
        return if validation_status == "passed" {
            "archive-ready".to_string()
        } else {
            "reviewing-implementation".to_string()
        };
    }

    "apply-ready".to_string()
}

fn render_manifesto_instruction_bodies(
    rt: &Runtime,
    request: &ManifestoRequest,
    change_id: &str,
    state: &str,
    review_status: &str,
) -> CliResult<Vec<Value>> {
    if request.variant != "full" {
        return Ok(Vec::new());
    }
    if change_id.is_empty() {
        if request.operation.is_some() {
            return fail(
                "The --operation selector requires --change when rendering a full manifesto",
            );
        }
        return Ok(Vec::new());
    }
    let mut allowed = allowed_manifesto_artifacts(state, &request.profile, review_status);
    if let Some(operation) = request.operation.as_deref() {
        if !allowed.iter().any(|artifact| artifact == operation) {
            return fail(format!(
                "Requested operation '{operation}' is not allowed for state '{state}' and profile '{}'.",
                request.profile
            ));
        }
        allowed.retain(|artifact| artifact == operation);
    }

    let mut rendered = Vec::new();
    for artifact in allowed {
        let body = render_manifesto_instruction_body(rt, change_id, &artifact)?;
        if !body.trim().is_empty() {
            rendered.push(json!({ "id": artifact, "body": body }));
        }
    }
    Ok(rendered)
}

fn render_manifesto_instruction_body(
    rt: &Runtime,
    change_id: &str,
    artifact: &str,
) -> CliResult<String> {
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let ctx = rt.ctx();
    let testing_policy = load_testing_policy(project_root, ito_path, ctx);
    let user_guidance =
        core_templates::load_composed_user_guidance(ito_path, artifact).unwrap_or(None);

    match artifact {
        "proposal" | "specs" | "design" | "tasks" => {
            let resolved =
                core_templates::resolve_instructions(ito_path, change_id, None, artifact, ctx)
                    .map_err(to_cli_error)?;
            render_artifact_instructions_text(&resolved, user_guidance.as_deref(), &testing_policy)
        }
        "apply" => {
            let apply = core_templates::compute_apply_instructions(ito_path, change_id, None, ctx)
                .map_err(to_cli_error)?;
            let worktree_config = load_worktree_config(project_root, ito_path, ctx);
            let memory_template = memory_template_config_from_merged(
                &load_cascading_project_config(project_root, ito_path, ctx).merged,
            );
            Ok(render_apply_instructions_text(
                &apply,
                &testing_policy,
                user_guidance.as_deref(),
                &worktree_config,
                memory_template,
            ))
        }
        "review" => {
            let review = core_templates::compute_review_context(
                runtime.repositories().changes.as_ref(),
                runtime.repositories().modules.as_ref(),
                ito_path,
                change_id,
                None,
                ctx,
            )
            .map_err(to_cli_error)?;
            ito_templates::instructions::render_instruction_template("agent/review.md.j2", &review)
                .map_err(to_cli_error)
        }
        "archive" => {
            let cfg = load_cascading_project_config(project_root, ito_path, ctx);
            let archive = archive_instruction_config_from_merged(&cfg.merged)?;
            ito_templates::instructions::render_instruction_template(
                "agent/archive.md.j2",
                &json!({
                    "archive": archive,
                    "change": change_id,
                    "available_changes": Vec::<String>::new(),
                }),
            )
            .map_err(to_cli_error)
        }
        "finish" => {
            let cfg = load_cascading_project_config(project_root, ito_path, ctx);
            let worktree =
                worktree_config_from_merged_with_paths(&cfg.merged, project_root, ito_path);
            let archive = archive_instruction_config_from_merged(&cfg.merged)?;
            let memory = memory_template_config_from_merged(&cfg.merged);
            ito_templates::instructions::render_instruction_template(
                "agent/finish.md.j2",
                &json!({
                    "worktree": worktree,
                    "archive": archive,
                    "memory": memory,
                    "change": change_id,
                    "archive_prompt_rendered": true,
                }),
            )
            .map_err(to_cli_error)
        }
        _ => Ok(String::new()),
    }
}

fn allowed_manifesto_artifacts(state: &str, profile: &str, review_status: &str) -> Vec<String> {
    let state_ops: &[&str] = match state {
        "no-change-selected" => &[],
        "proposal-drafting" => &["proposal", "specs", "design", "tasks", "review"],
        "review-needed" => &["review"],
        "apply-ready" => &["apply"],
        "applying" => &["apply", "review"],
        "reviewing-implementation" => &["review"],
        "archive-ready" => &["archive"],
        "finished" => &["finish"],
        _ => &[],
    };
    let profile_ops: &[&str] = match profile {
        "planning" => &[],
        "proposal-only" => &["proposal", "specs", "design", "tasks", "review"],
        "review-only" => &["review"],
        "apply" => &["apply", "review"],
        "archive" => &["archive", "finish"],
        "full" => &[
            "proposal", "specs", "design", "tasks", "apply", "review", "archive", "finish",
        ],
        _ => &[],
    };
    let mut out = Vec::new();
    for artifact in MANIFESTO_OPERATIONS {
        if state_ops.contains(artifact) && profile_ops.contains(artifact) {
            out.push((*artifact).to_string());
        }
    }
    if review_status == "pending-approval" || review_status == "changes-requested" {
        out.retain(|artifact| artifact == "review");
    }
    out
}

fn current_checkout_role(
    project_root: &Path,
    default_branch: &str,
    worktree_enabled: bool,
) -> &'static str {
    let Some(branch) = git_current_branch(project_root) else {
        return "unknown";
    };
    if branch == default_branch || branch == "main" || branch == "master" {
        return "main/control";
    }
    if branch.is_empty() {
        return "unknown";
    }
    if worktree_enabled && is_registered_dedicated_worktree(project_root) {
        "change-worktree"
    } else {
        "non-default-branch"
    }
}

fn git_current_branch(project_root: &Path) -> Option<String> {
    let branch = std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(project_root)
        .output()
        .ok()?;
    if !branch.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&branch.stdout).trim().to_string())
}

fn is_registered_dedicated_worktree(project_root: &Path) -> bool {
    let Ok(current_root) = project_root.canonicalize() else {
        return false;
    };
    let Ok(output) = std::process::Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(project_root)
        .output()
    else {
        return false;
    };
    if !output.status.success() {
        return false;
    }
    let output = String::from_utf8_lossy(&output.stdout);
    let worktrees = output
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .filter_map(|path| Path::new(path).canonicalize().ok())
        .collect::<Vec<_>>();
    worktrees.len() > 1 && worktrees.iter().any(|path| path == &current_root)
}

fn coordination_sync_confirmed(outcome: &Option<CoordinationSyncOutcome>) -> bool {
    match outcome {
        Some(CoordinationSyncOutcome::Synchronized) => true,
        Some(CoordinationSyncOutcome::RateLimited) => false,
        Some(CoordinationSyncOutcome::Embedded) => false,
        None => false,
    }
}

#[derive(Debug, Clone)]
struct ManifestoMemoryInstructions {
    capture: Value,
    search: Value,
    query: Value,
}

fn build_manifesto_memory_instructions(
    memory: Option<&ito_config::types::MemoryConfig>,
) -> ManifestoMemoryInstructions {
    let capture = render_memory_instruction_text(
        &ito_core::memory::render_capture(
            memory,
            &CaptureInputs {
                context: Some("<context>".to_string()),
                files: Vec::new(),
                folders: Vec::new(),
            },
        ),
        "memory-capture",
    );
    let search = render_memory_instruction_text(
        &ito_core::memory::render_search(
            memory,
            &SearchInputs {
                query: "<query>".to_string(),
                limit: None,
                scope: None,
            },
        ),
        "memory-search",
    );
    let query = render_memory_instruction_text(
        &ito_core::memory::render_query(
            memory,
            &QueryInputs {
                query: "<query>".to_string(),
            },
        ),
        "memory-query",
    );

    ManifestoMemoryInstructions {
        capture: option_string_to_value(capture),
        search: option_string_to_value(search),
        query: option_string_to_value(query),
    }
}

fn option_string_to_value(value: Option<String>) -> Value {
    match value {
        Some(value) => Value::String(value),
        None => Value::Null,
    }
}

fn worktree_strategy_name(strategy: ito_config::types::WorktreeStrategy) -> &'static str {
    match strategy {
        ito_config::types::WorktreeStrategy::CheckoutSubdir => "checkout_subdir",
        ito_config::types::WorktreeStrategy::CheckoutSiblings => "checkout_siblings",
        ito_config::types::WorktreeStrategy::BareControlSiblings => "bare_control_siblings",
    }
}

fn redact_path_value(raw: &str, project_root: &Path) -> String {
    let path = Path::new(raw);
    if !path.is_absolute() {
        return raw.to_string();
    }
    if let Ok(stripped) = path.strip_prefix(project_root) {
        let rel = stripped.to_string_lossy();
        if rel.is_empty() {
            ".".to_string()
        } else {
            format!("./{rel}")
        }
    } else {
        "<redacted-path>".to_string()
    }
}

fn build_manifesto_config_capsule(
    typed: &ItoConfig,
    project_root: &Path,
    worktree: &Value,
    coordination: &Value,
    memory: &MemoryTemplateConfig,
) -> Value {
    json!({
        "defaults": {
            "variant": "light",
            "profile": "full",
        },
        "worktrees": worktree,
        "coordination_branch": coordination,
        "memory": {
            "capture_configured": memory.capture.configured,
            "search_configured": memory.search.configured,
            "query_configured": memory.query.configured,
        },
        "backend": {
            "enabled": typed.backend.enabled,
            "project": {
                "org": typed.backend.project.org.clone(),
                "repo": typed.backend.project.repo.clone(),
            },
            "url": if typed.backend.enabled {
                Value::String("<redacted-backend-url>".to_string())
            } else {
                Value::Null
            },
        },
        "project_root": redact_path_value(project_root.to_string_lossy().as_ref(), project_root),
    })
}

fn artifact_capsule_state(change: &Value, artifact: &str) -> &'static str {
    let available = change
        .get("available_artifacts")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .any(|item| item == artifact)
        });
    if available { "done" } else { "missing" }
}
