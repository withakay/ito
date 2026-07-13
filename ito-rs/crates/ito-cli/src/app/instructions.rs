use crate::cli::{AgentArgs, AgentCommand, AgentInstructionArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::commands::sync::best_effort_sync_coordination;
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_config::types::ItoConfig;

use super::cleanup_instructions::generate_cleanup_instruction;
use super::memory_instructions::{MemoryTemplateConfig, memory_template_config_from_merged};
use ito_common::harness::detect_harness_name;
use ito_config::resolve_coordination_branch_settings;
use ito_core::harness_context;
use ito_core::templates as core_templates;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub(super) use super::worktree_instruction_config::{
    WorktreeConfig, worktree_config_from_merged_with_paths, worktree_config_from_resolved,
};
#[cfg(test)]
use super::worktree_instruction_config::{resolve_bare_repo_root, worktree_config_from_merged};
#[cfg(test)]
use ito_config::types::WorktreeStrategy;

#[derive(Debug, Clone, serde::Serialize)]
struct ContextFileEntry {
    id: String,
    path: String,
}

pub(crate) fn handle_agent(rt: &Runtime, args: &[String]) -> CliResult<()> {
    // Check for subcommand first - subcommand handlers have their own help checks
    match args.first().map(|s| s.as_str()) {
        Some("instruction") => handle_agent_instruction(rt, &args[1..]),
        // Show parent help only if no valid subcommand or explicit help request
        _ if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") => {
            println!(
                "{}",
                super::common::render_command_long_help(&["agent"], "ito agent")
            );
            Ok(())
        }
        _ => {
            // Unknown subcommand — log it as an invalid command.
            let mut raw_args = vec!["agent".to_string()];
            raw_args.extend(args.iter().cloned());
            let error_message = format!(
                "Unknown agent subcommand '{}'",
                args.first().map(|s| s.as_str()).unwrap_or("")
            );
            crate::util::maybe_log_invalid_command(rt, &raw_args, &error_message);
            println!(
                "{}",
                super::common::render_command_long_help(&["agent"], "ito agent")
            );
            Ok(())
        }
    }
}

/// Generate and print instructions for a requested agent artifact.
///
/// Parses the provided CLI arguments to determine the artifact and options, loads configuration and testing policy as needed, validates required flags (for example, `--tool` for `bootstrap` where allowed values are `opencode`, `claude`, `codex`, or `github-copilot`, and `--change` for change-scoped artifacts), and emits either plain instruction text or a JSON-wrapped response when `--json` is present. Prints the long help text when no arguments or a help flag is supplied and surfaces user-facing error messages for common failure cases.
pub(crate) fn handle_agent_instruction(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(
                &["agent", "instruction"],
                "ito agent instruction",
            )
        );
        return Ok(());
    }
    let want_json = args.iter().any(|a| a == "--json");
    let artifact = args.first().map(|s| s.as_str()).unwrap_or("");
    if artifact.is_empty() || artifact.starts_with('-') {
        return fail("Missing required argument <artifact>");
    }

    if artifact == "bootstrap" {
        let tool = parse_string_flag(args, "--tool");
        if tool.as_deref().unwrap_or("").is_empty() {
            return fail("Missing required option --tool for bootstrap artifact");
        }
        let tool = tool.expect("checked above");
        let valid_tools = ["opencode", "claude", "codex", "github-copilot", "pi"];
        if !valid_tools.contains(&tool.as_str()) {
            return fail(format!(
                "Invalid tool '{}'. Valid tools: {}",
                tool,
                valid_tools.join(", ")
            ));
        }

        let instruction = generate_bootstrap_instruction(&tool)?;
        return emit_instruction(want_json, "bootstrap", instruction);
    }

    if artifact == "project-setup" {
        let instruction = generate_project_setup_instruction()?;
        return emit_instruction(want_json, "project-setup", instruction);
    }

    if artifact == "backend" {
        let instruction = generate_backend_instruction()?;
        return emit_instruction(want_json, "backend", instruction);
    }
    if artifact == "repo-sweep" {
        eprintln!("- Generating repo-sweep instructions...");
        let instruction = generate_repo_sweep_instruction()?;
        return emit_instruction(want_json, "repo-sweep", instruction);
    }
    if artifact == "cleanup" {
        let instruction = generate_cleanup_instruction(rt)?;
        return emit_instruction(want_json, "cleanup", instruction);
    }
    if artifact == "orchestrate" {
        let ito_path = rt.ito_path();
        let prompt = match ito_core::orchestrate::load_orchestrate_user_prompt(ito_path) {
            Ok(prompt) => prompt,
            Err(ito_core::errors::CoreError::NotFound(_)) => {
                return fail(
                    "Missing required project file: .ito/user-prompts/orchestrate.md\n\n\
The orchestrator workflow is configured via orchestrate.md and a project workflow skill.\n\
Fix: run the orchestrator setup flow (agent-driven) by loading the ito-orchestrate-setup skill,\n\
then re-run:\n\n\
  ito agent instruction orchestrate\n",
                );
            }
            Err(e) => return Err(to_cli_error(e)),
        };

        let preset_name = prompt
            .front_matter
            .preset
            .clone()
            .unwrap_or_else(|| "generic".to_string());
        let preset = match ito_core::orchestrate::load_orchestrate_preset(&preset_name) {
            Ok(preset) => preset,
            Err(ito_core::errors::CoreError::NotFound(_)) => {
                let available = ito_core::orchestrate::list_orchestrate_presets();
                return fail(format!(
                    "Unknown orchestrate preset '{preset_name}'.\n\nAvailable presets:\n  {available}\n",
                    available = available.join("\n  ")
                ));
            }
            Err(e) => return Err(to_cli_error(e)),
        };

        let orchestrate_md_path = prompt.path.to_string_lossy().to_string();
        let orchestrate_md = prompt.raw;

        #[derive(serde::Serialize)]
        struct Ctx<'a> {
            orchestrate_md_path: &'a str,
            orchestrate_md: &'a str,
            workflow_skill_name: &'a str,
            preset_name: &'a str,
            gate_order: &'a [String],
            recommended_skills: &'a [String],
            coordinator_agent_name: &'a str,
            harness_name: &'a str,
            agent_roles_md: &'a str,
        }

        const ROLE_ORDER: &[&str] = &[
            "plan-worker",
            "research-worker",
            "apply-worker",
            "review-worker",
            "security-worker",
        ];

        let mut agent_roles = Vec::new();
        for role in ROLE_ORDER {
            let Some(agent) = preset.agent_roles.get(*role) else {
                continue;
            };
            if agent.is_empty() {
                continue;
            }
            agent_roles.push(format!("  - `{role}`: `{agent}`"));
        }
        for (role, agent) in &preset.agent_roles {
            if ROLE_ORDER.contains(&role.as_str()) || agent.is_empty() {
                continue;
            }
            agent_roles.push(format!("  - `{role}`: `{agent}`"));
        }
        let agent_roles_md = agent_roles.join("\n");
        let harness_name = detect_harness_name();

        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/orchestrate.md.j2",
            &Ctx {
                orchestrate_md_path: &orchestrate_md_path,
                orchestrate_md: &orchestrate_md,
                workflow_skill_name: "ito-orchestrator-workflow",
                preset_name: &preset.name,
                gate_order: &preset.gate_order,
                recommended_skills: &preset.recommended_skills,
                coordinator_agent_name: "ito-orchestrator",
                harness_name,
                agent_roles_md: &agent_roles_md,
            },
        )
        .map_err(|e| to_cli_error(format!("failed to render orchestrate instruction: {e}")))?;

        return emit_instruction(want_json, "orchestrate", instruction);
    }
    if artifact == "migrate-to-main" {
        let (typed, config_error) =
            match serde_json::from_value::<ItoConfig>(rt.resolved_config().merged.clone()) {
                Ok(typed) => (typed, None),
                Err(error) => (
                    ItoConfig::default(),
                    Some(format!("resolved Ito configuration is invalid: {error}")),
                ),
            };
        let ito_root = rt.ito_path();
        let project_root = ito_root.parent().unwrap_or(ito_root);
        let expected_coordination_ito_root =
            ito_core::legacy_coordination::expected_coordination_ito_root(
                project_root,
                ito_root,
                &typed.changes.coordination_branch,
                &typed.backend,
            );
        let observed_evidence_json = if let Some(error) = config_error {
            serde_json::to_string_pretty(&serde_json::json!({
                "classification": { "kind": "inspection_error" },
                "error": error,
            }))
            .map_err(to_cli_error)?
        } else {
            match ito_core::legacy_coordination::inspect_legacy_coordination(
                project_root,
                ito_root,
                &typed.changes.coordination_branch,
                expected_coordination_ito_root.as_deref(),
            ) {
                Ok(report) => serde_json::to_string_pretty(&report).map_err(to_cli_error)?,
                Err(error) => serde_json::to_string_pretty(&serde_json::json!({
                    "classification": { "kind": "inspection_error" },
                    "error": error.to_string(),
                }))
                .map_err(to_cli_error)?,
            }
        };
        let expected_managed_paths = ito_core::legacy_coordination::MANAGED_STATE_DIRS
            .iter()
            .map(|name| ito_root.join(name).to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        let instruction = ito_templates::instructions::render_instruction_template(
            ito_templates::instructions::MIGRATE_TO_MAIN_TEMPLATE_PATH,
            &serde_json::json!({
                "project_root": project_root.to_string_lossy(),
                "ito_root": ito_root.to_string_lossy(),
                "coordination_branch_name": typed.changes.coordination_branch.name,
                "coordination_enabled": typed.changes.coordination_branch.enabled.0,
                "coordination_storage": typed.changes.coordination_branch.storage.as_str(),
                "expected_coordination_ito_root": expected_coordination_ito_root
                    .as_deref()
                    .map(|path| path.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "unresolved".to_string()),
                "expected_managed_paths": expected_managed_paths,
                "observed_evidence_json": observed_evidence_json,
                "main_integration_mode": typed.changes.archive.main_integration_mode.as_str(),
            }),
        )
        .map_err(|error| to_cli_error(format!("rendering migrate-to-main instruction: {error}")))?;
        return emit_instruction(want_json, "migrate-to-main", instruction);
    }
    if artifact == "migrate-to-coordination-worktree" {
        let (_coord_enabled, coord_branch) =
            resolve_coordination_branch_settings(&rt.resolved_config().merged);
        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/migrate-to-coordination-worktree.md.j2",
            &serde_json::json!({
                "coordination_branch_name": coord_branch,
            }),
        )
        .map_err(|e| {
            to_cli_error(format!(
                "Failed to render the 'migrate-to-coordination-worktree' instruction template.\n\
             \n\
             Why: The Jinja2 template could not be rendered — this usually means a required \
             template variable is missing or the template asset is corrupt.\n\
             \n\
             How to fix: Reinstall the template assets with `ito init --force`, then retry. \
             If the problem persists, report it with the error below.\n\
             \n\
             Underlying error: {e}"
            ))
        })?;
        return emit_instruction(want_json, "migrate-to-coordination-worktree", instruction);
    }
    if artifact == "context" {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let inferred = harness_context::infer_context_from_cwd(&cwd).map_err(to_cli_error)?;

        if want_json {
            let rendered = serde_json::to_string_pretty(&inferred)
                .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
            println!("{rendered}");
            return Ok(());
        }

        if let Some(target) = &inferred.target {
            let kind = match target.kind {
                harness_context::InferredItoTargetKind::Change => "change",
                harness_context::InferredItoTargetKind::Module => "module",
            };
            print!(
                "[Ito Target] {kind} {id}\n[Ito Continuation] {nudge}\n",
                kind = kind,
                id = target.id.as_str(),
                nudge = inferred.nudge.as_str()
            );
        } else {
            print!(
                "[Ito Target] none\n[Ito Continuation] {nudge}\n",
                nudge = inferred.nudge.as_str()
            );
        }
        return Ok(());
    }

    if artifact == "schemas" {
        let ctx = rt.ctx();
        let response = core_templates::list_schemas_detail(ctx);

        if want_json {
            let rendered = serde_json::to_string_pretty(&response)
                .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
            println!("{rendered}");
            return Ok(());
        }

        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/schemas.md.j2",
            &response,
        )
        .map_err(|e| to_cli_error(format!("failed to render schemas instruction: {e}")))?;

        print!("{instruction}");
        return Ok(());
    }

    if artifact == "worktrees" {
        let ito_path = rt.ito_path();
        let project_root = ito_path.parent().unwrap_or(ito_path);
        let cfg = rt.resolved_config();
        let worktree = worktree_config_from_merged_with_paths(&cfg.merged, project_root, ito_path);
        let ito_dir_name = ito_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(".ito")
            .to_string();
        let loaded_from: Vec<String> = cfg
            .loaded_from
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        #[derive(serde::Serialize)]
        struct Ctx {
            ito_dir_name: String,
            worktree: WorktreeConfig,
            loaded_from: Vec<String>,
        }

        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/worktrees.md.j2",
            &Ctx {
                ito_dir_name,
                worktree,
                loaded_from,
            },
        )
        .map_err(|e| to_cli_error(format!("failed to render worktrees instruction: {e}")))?;

        return emit_instruction(want_json, artifact, instruction);
    }

    if artifact == "worktree-init" {
        let ito_path = rt.ito_path();
        let project_root = ito_path.parent().unwrap_or(ito_path);
        let cfg = rt.resolved_config();
        let worktree = worktree_config_from_merged_with_paths(&cfg.merged, project_root, ito_path);
        let change = parse_string_flag(args, "--change")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        #[derive(serde::Serialize)]
        struct Ctx {
            worktree: WorktreeConfig,
            change: Option<String>,
        }

        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/worktree-init.md.j2",
            &Ctx { worktree, change },
        )
        .map_err(|e| to_cli_error(format!("failed to render worktree-init instruction: {e}")))?;

        return emit_instruction(want_json, artifact, instruction);
    }

    if artifact == "manifesto" {
        return super::manifesto_instructions::handle_manifesto_instruction(rt, args, want_json);
    }

    if artifact == "finish" {
        best_effort_sync_coordination(rt, "before finish instructions");

        let ito_path = rt.ito_path();
        let project_root = ito_path.parent().unwrap_or(ito_path);
        let cfg = rt.resolved_config();
        let worktree = worktree_config_from_merged_with_paths(&cfg.merged, project_root, ito_path);
        let archive = archive_instruction_config_from_merged(&cfg.merged)?;
        let memory = memory_template_config_from_merged(&cfg.merged);
        let change = parse_string_flag(args, "--change")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        #[derive(serde::Serialize)]
        struct Ctx {
            worktree: WorktreeConfig,
            archive: ArchiveInstructionConfig,
            memory: MemoryTemplateConfig,
            change: Option<String>,
            /// Always `true` today: the existing archive prompt always renders.
            ///
            /// Reserved for future logic that suppresses the prompt when the
            /// change is already archived; templates SHOULD use this flag to
            /// decide whether the wrap-up reminder also lists the archive
            /// step (`{% if not archive_prompt_rendered %}`).
            archive_prompt_rendered: bool,
        }

        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/finish.md.j2",
            &Ctx {
                worktree,
                archive,
                memory,
                change,
                archive_prompt_rendered: true,
            },
        )
        .map_err(|e| to_cli_error(format!("failed to render finish instruction: {e}")))?;

        return emit_instruction(want_json, artifact, instruction);
    }

    if artifact == "archive" {
        best_effort_sync_coordination(rt, "before archive instructions");

        let runtime = rt.repository_runtime().map_err(to_cli_error)?;
        let cfg = rt.resolved_config();
        let archive = archive_instruction_config_from_merged(&cfg.merged)?;
        let change = parse_string_flag(args, "--change")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let all_changes: Vec<String> = runtime
            .repositories()
            .changes
            .list()
            .unwrap_or_default()
            .into_iter()
            .map(|c| c.id)
            .collect();

        let resolved_change = if let Some(ref raw) = change {
            let change_repo = runtime.repositories().changes.as_ref();
            match super::common::resolve_change_target(change_repo, raw) {
                Ok(resolved) => Some(resolved),
                Err(msg) => return fail(msg),
            }
        } else {
            None
        };

        let available_changes: Vec<String> = all_changes
            .into_iter()
            .filter(|id| resolved_change.as_deref() != Some(id.as_str()))
            .collect();

        #[derive(serde::Serialize)]
        struct Ctx {
            archive: ArchiveInstructionConfig,
            change: Option<String>,
            available_changes: Vec<String>,
        }

        let instruction = ito_templates::instructions::render_instruction_template(
            "agent/archive.md.j2",
            &Ctx {
                archive,
                change: resolved_change,
                available_changes,
            },
        )
        .map_err(|e| to_cli_error(format!("failed to render archive instruction: {e}")))?;

        return emit_instruction(want_json, "archive", instruction);
    }

    if super::memory_instructions::try_handle(rt, artifact, args, want_json)? {
        return Ok(());
    }

    let change = parse_string_flag(args, "--change");
    if change.as_deref().unwrap_or("").is_empty() {
        // Special case: proposal without --change outputs creation guide
        if artifact == "proposal" {
            return handle_new_proposal_guide(rt, want_json);
        }
        if artifact == "review" {
            return fail("review instruction requires --change <id>");
        }

        let runtime = rt.repository_runtime().map_err(to_cli_error)?;
        let changes = runtime.repositories().changes.list().unwrap_or_default();
        let mut msg = "Missing required option --change".to_string();
        if !changes.is_empty() {
            msg.push_str("\n\nAvailable changes:\n");
            for c in changes {
                msg.push_str(&format!("  {}\n", c.id));
            }
        }
        return fail(msg);
    }
    let ctx = rt.ctx();
    let ito_path = rt.ito_path();
    let want_sync = args.iter().any(|a| a == "--sync");
    if sync_before_change_resolution(artifact, want_sync) {
        best_effort_sync_coordination(rt, &format!("before {artifact} instructions"));
    }
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let change_repo = runtime.repositories().changes.as_ref();
    let change = change.expect("checked above");
    let change = match super::common::resolve_change_target(change_repo, &change) {
        Ok(resolved) => resolved,
        Err(msg) => return fail(msg),
    };
    let schema = parse_string_flag(args, "--schema");

    let project_root = ito_path.parent().unwrap_or(ito_path);
    let resolved = rt.resolved_config();
    let testing_policy = testing_policy_from_merged(&resolved.merged);

    let user_guidance = match core_templates::load_composed_user_guidance(ito_path, artifact) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Warning: failed to read user guidance files (.ito/user-prompts/<artifact>.md, .ito/user-prompts/guidance.md, .ito/user-guidance.md): {e}"
            );
            None
        }
    };

    // Match TS/ora: spinner output is written to stderr.
    eprintln!("- Generating instructions...");

    if artifact == "apply" {
        // Match TS/ora: spinner output is written to stderr.
        eprintln!("- Generating apply instructions...");

        let apply = match core_templates::compute_apply_instructions(
            ito_path,
            &change,
            schema.as_deref(),
            ctx,
        ) {
            Ok(r) => r,
            Err(core_templates::TemplatesError::InvalidChangeName) => {
                return fail("Invalid change name");
            }
            Err(core_templates::TemplatesError::ChangeNotFound(name)) => {
                return fail(format!("Change '{name}' not found"));
            }
            Err(core_templates::TemplatesError::SchemaNotFound(name)) => {
                return fail(super::common::schema_not_found_message(ctx, &name));
            }
            Err(e) => return Err(to_cli_error(e)),
        };

        if want_json {
            let rendered = serde_json::to_string_pretty(&apply)
                .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
            println!("{rendered}");
            return Ok(());
        }

        let worktree_config =
            worktree_config_from_resolved(&resolved.merged, project_root, ito_path);
        let memory_template = memory_template_config_from_merged(&resolved.merged);
        let out = render_apply_instructions_text(
            &apply,
            &testing_policy,
            user_guidance.as_deref(),
            &worktree_config,
            memory_template,
        );
        print!("{out}");
        return Ok(());
    }

    if artifact == "review" {
        let review = match core_templates::compute_review_context(
            change_repo,
            runtime.repositories().modules.as_ref(),
            ito_path,
            &change,
            schema.as_deref(),
            ctx,
        ) {
            Ok(r) => r,
            Err(core_templates::TemplatesError::InvalidChangeName) => {
                return fail("Invalid change name");
            }
            Err(core_templates::TemplatesError::ChangeNotFound(name)) => {
                return fail(format!("Change '{name}' not found"));
            }
            Err(core_templates::TemplatesError::SchemaNotFound(name)) => {
                return fail(super::common::schema_not_found_message(ctx, &name));
            }
            Err(e) => return Err(to_cli_error(e)),
        };

        let instruction =
            ito_templates::instructions::render_instruction_template("agent/review.md.j2", &review)
                .map_err(to_cli_error)?;

        return emit_instruction(want_json, artifact, instruction);
    }

    let resolved_instr = match core_templates::resolve_instructions(
        ito_path,
        &change,
        schema.as_deref(),
        artifact,
        ctx,
    ) {
        Ok(r) => r,
        Err(core_templates::TemplatesError::InvalidChangeName) => {
            return fail("Invalid change name");
        }
        Err(core_templates::TemplatesError::ChangeNotFound(name)) => {
            return fail(format!("Change '{name}' not found"));
        }
        Err(core_templates::TemplatesError::SchemaNotFound(name)) => {
            return fail(super::common::schema_not_found_message(ctx, &name));
        }
        Err(e) => return Err(to_cli_error(e)),
    };

    if want_json {
        let rendered = serde_json::to_string_pretty(&resolved_instr)
            .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
        println!("{rendered}");
        return Ok(());
    }
    let out = render_artifact_instructions_text(
        &resolved_instr,
        user_guidance.as_deref(),
        &testing_policy,
    )?;
    print!("{out}");

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
pub(super) struct TestingPolicy {
    tdd_workflow: String,
    coverage_target_percent: u64,
}

pub(super) fn testing_policy_from_merged(merged: &serde_json::Value) -> TestingPolicy {
    let mut out = TestingPolicy {
        tdd_workflow: "red-green-refactor".to_string(),
        coverage_target_percent: 80,
    };

    if let Some(v) = json_get(merged, &["defaults", "testing", "tdd", "workflow"])
        && let Some(s) = v.as_str()
    {
        let s = s.trim();
        if !s.is_empty() {
            out.tdd_workflow = s.to_string();
        }
    }

    if let Some(v) = json_get(
        merged,
        &["defaults", "testing", "coverage", "target_percent"],
    ) {
        if let Some(n) = v.as_u64() {
            out.coverage_target_percent = n;
        } else if let Some(n) = v.as_f64()
            && n.is_finite()
            && n >= 0.0
        {
            out.coverage_target_percent = n.round() as u64;
        }
    }

    out
}

/// Whether to sync the coordination branch before resolving a change artifact.
///
/// `proposal` and `review` always sync (they need up-to-date remote state).
/// `apply` only syncs when the caller explicitly passes `--sync` so that
/// instruction generation does not block on network I/O by default.
/// `archive` and `finish` do not reach this helper because their dedicated
/// instruction handlers sync before rendering.
fn sync_before_change_resolution(artifact: &str, want_sync: bool) -> bool {
    match artifact {
        "apply" => want_sync,
        "proposal" | "review" => true,
        _ => false,
    }
}

fn json_get<'a>(root: &'a serde_json::Value, keys: &[&str]) -> Option<&'a serde_json::Value> {
    let mut cur = root;
    for k in keys {
        let serde_json::Value::Object(map) = cur else {
            return None;
        };
        cur = map.get(*k)?;
    }
    Some(cur)
}

pub(crate) fn handle_agent_clap(rt: &Runtime, args: &AgentArgs) -> CliResult<()> {
    let result = match &args.command {
        Some(AgentCommand::Instruction(instr)) => handle_agent_instruction_clap(rt, instr),
        Some(AgentCommand::External(v)) => handle_agent(rt, v),
        None => handle_agent(rt, &[]),
    };

    // If the command failed, log it as an invalid command (best-effort).
    if let Err(ref e) = result
        && !e.is_silent()
    {
        let raw_args = reconstruct_agent_args(args);
        crate::util::maybe_log_invalid_command(rt, &raw_args, &e.to_string());
    }

    result
}

fn reconstruct_agent_args(args: &AgentArgs) -> Vec<String> {
    let mut raw = vec!["agent".to_string()];
    match &args.command {
        Some(AgentCommand::Instruction(instr)) => {
            raw.push("instruction".to_string());
            raw.extend(instr.to_argv());
        }
        Some(AgentCommand::External(v)) => {
            raw.extend(v.iter().cloned());
        }
        None => {}
    }
    raw
}

fn handle_agent_instruction_clap(rt: &Runtime, args: &AgentInstructionArgs) -> CliResult<()> {
    handle_agent_instruction(rt, &args.to_argv())
}

fn emit_instruction(want_json: bool, artifact_id: &str, instruction: String) -> CliResult<()> {
    if want_json {
        let response = core_templates::AgentInstructionResponse {
            artifact_id: artifact_id.to_string(),
            instruction,
        };
        let rendered = serde_json::to_string_pretty(&response)
            .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
        println!("{rendered}");
    } else {
        print!("{instruction}");
    }
    Ok(())
}

fn generate_bootstrap_instruction(tool: &str) -> CliResult<String> {
    #[derive(serde::Serialize)]
    struct Ctx<'a> {
        tool: &'a str,
    }
    ito_templates::instructions::render_instruction_template("agent/bootstrap.md.j2", &Ctx { tool })
        .map_err(|e| to_cli_error(format!("rendering bootstrap instruction: {e}")))
}

fn generate_project_setup_instruction() -> CliResult<String> {
    #[derive(serde::Serialize)]
    struct Ctx {}
    ito_templates::instructions::render_instruction_template("agent/project-setup.md.j2", &Ctx {})
        .map_err(|e| to_cli_error(format!("rendering project-setup instruction: {e}")))
}

fn generate_backend_instruction() -> CliResult<String> {
    #[derive(serde::Serialize)]
    struct Ctx {}
    ito_templates::instructions::render_instruction_template("agent/backend.md.j2", &Ctx {})
        .map_err(|e| to_cli_error(format!("rendering backend instruction: {e}")))
}

fn generate_repo_sweep_instruction() -> CliResult<String> {
    #[derive(serde::Serialize)]
    struct Ctx {}
    ito_templates::instructions::render_instruction_template("agent/repo-sweep.md.j2", &Ctx {})
        .map_err(|e| to_cli_error(format!("rendering repo-sweep instruction: {e}")))
}

#[derive(Debug, Clone, serde::Serialize)]
pub(super) struct ArchiveInstructionConfig {
    coordination_storage: String,
    coordination_active: bool,
    main_integration_mode: String,
}

pub(super) fn archive_instruction_config_from_merged(
    merged: &serde_json::Value,
) -> CliResult<ArchiveInstructionConfig> {
    let typed: ItoConfig = serde::Deserialize::deserialize(merged).map_err(|e| {
        to_cli_error(format!(
            "Failed to parse merged Ito config.\n\
             \n\
             Why: The merged config contains an invalid value or type, so instruction rendering cannot safely choose a main integration mode.\n\
             \n\
             How to fix: Run `ito config check` (or inspect your config files) and correct the invalid field, then retry.\n\
             \n\
             Underlying error: {e}"
        ))
    })?;

    Ok(ArchiveInstructionConfig {
        coordination_storage: typed
            .changes
            .coordination_branch
            .storage
            .as_str()
            .to_string(),
        coordination_active: cfg!(feature = "coordination-branch")
            && typed.changes.coordination_branch.enabled.0,
        main_integration_mode: typed
            .changes
            .archive
            .main_integration_mode
            .as_str()
            .to_string(),
    })
}

fn handle_new_proposal_guide(rt: &Runtime, want_json: bool) -> CliResult<()> {
    best_effort_sync_coordination(rt, "before proposal instructions");

    #[derive(serde::Serialize)]
    struct ModuleEntry {
        id: String,
        name: String,
    }
    #[derive(serde::Serialize)]
    struct Ctx {
        modules: Vec<ModuleEntry>,
    }
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let modules = runtime.repositories().modules.list().unwrap_or_default();
    let modules: Vec<ModuleEntry> = modules
        .into_iter()
        .map(|m| ModuleEntry {
            id: m.id,
            name: m.name,
        })
        .collect();
    let ctx = Ctx { modules };
    let instruction =
        ito_templates::instructions::render_instruction_template("agent/new-proposal.md.j2", &ctx)
            .map_err(|e| to_cli_error(format!("rendering new-proposal instruction: {e}")))?;

    emit_instruction(want_json, "new-proposal", instruction)
}

pub(super) fn render_artifact_instructions_text(
    instructions: &core_templates::InstructionsResponse,
    user_guidance: Option<&str>,
    testing_policy: &TestingPolicy,
) -> CliResult<String> {
    #[derive(Debug, Clone, serde::Serialize)]
    struct TemplateDependency {
        id: String,
        status: String,
        path: String,
        description: String,
    }

    #[derive(serde::Serialize)]
    struct TemplateInstructions {
        #[serde(rename = "changeName")]
        change_name: String,
        #[serde(rename = "artifactId")]
        artifact_id: String,
        #[serde(rename = "schemaName")]
        schema_name: String,
        description: String,
        instruction: String,
        template: String,
        unlocks: Vec<String>,
    }

    #[derive(serde::Serialize)]
    struct Ctx {
        instructions: TemplateInstructions,
        missing: Vec<String>,
        dependencies: Vec<TemplateDependency>,
        out_path: String,
        testing_policy: TestingPolicy,
        user_guidance: Option<String>,
    }

    let missing = collect_missing_dependencies(instructions);

    let mut dependencies = Vec::new();
    for dep in &instructions.dependencies {
        let p = Path::new(&instructions.change_dir).join(&dep.path);
        dependencies.push(TemplateDependency {
            id: dep.id.clone(),
            status: if dep.done {
                "done".to_string()
            } else {
                "missing".to_string()
            },
            path: p.to_string_lossy().to_string(),
            description: dep.description.clone(),
        });
    }

    let out_path = Path::new(&instructions.change_dir).join(&instructions.output_path);

    let user_guidance = user_guidance
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let ctx = Ctx {
        instructions: TemplateInstructions {
            change_name: instructions.change_name.clone(),
            artifact_id: instructions.artifact_id.clone(),
            schema_name: instructions.schema_name.clone(),
            description: instructions.description.clone(),
            instruction: instructions.instruction.clone().unwrap_or_default(),
            template: instructions.template.clone(),
            unlocks: instructions.unlocks.clone(),
        },
        missing,
        dependencies,
        out_path: out_path.to_string_lossy().to_string(),
        testing_policy: testing_policy.clone(),
        user_guidance,
    };

    ito_templates::instructions::render_instruction_template("agent/artifact.md.j2", &ctx)
        .map_err(|e| to_cli_error(format!("rendering artifact instruction: {e}")))
}

/// Render the apply instructions using the agent apply template and print the result to stdout.
///
/// The function renders the `agent/apply.md.j2` template with a context constructed from
/// `instructions`, `testing_policy`, optional `user_guidance`, and `worktree_config`, then writes
/// the rendered text to standard output. The provided `user_guidance` is trimmed and ignored if
/// empty.
pub(super) fn render_apply_instructions_text(
    instructions: &core_templates::ApplyInstructionsResponse,
    testing_policy: &TestingPolicy,
    user_guidance: Option<&str>,
    worktree_config: &WorktreeConfig,
    memory: MemoryTemplateConfig,
) -> String {
    #[derive(serde::Serialize)]
    struct Ctx {
        instructions: core_templates::ApplyInstructionsResponse,
        testing_policy: TestingPolicy,
        context_files: Vec<ContextFileEntry>,
        tracking_errors: Option<usize>,
        tracking_warnings: Option<usize>,
        user_guidance: Option<String>,
        worktree: WorktreeConfig,
        memory: MemoryTemplateConfig,
    }

    let context_files = collect_context_files(&instructions.context_files);
    let (tracking_errors, tracking_warnings) =
        collect_tracking_diagnostic_counts(instructions.tracks_diagnostics.as_deref());

    let user_guidance = user_guidance
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let ctx = Ctx {
        instructions: instructions.clone(),
        testing_policy: testing_policy.clone(),
        context_files,
        tracking_errors,
        tracking_warnings,
        user_guidance,
        worktree: worktree_config.clone(),
        memory,
    };

    ito_templates::instructions::render_instruction_template("agent/apply.md.j2", &ctx)
        .expect("apply instruction template should render")
}

fn collect_missing_dependencies(
    instructions: &core_templates::InstructionsResponse,
) -> Vec<String> {
    let mut out = Vec::new();
    for dep in &instructions.dependencies {
        if dep.done {
            continue;
        }
        out.push(dep.id.clone());
    }
    out
}

fn collect_context_files(map: &BTreeMap<String, String>) -> Vec<ContextFileEntry> {
    let mut out = Vec::new();
    for (id, path) in map {
        out.push(ContextFileEntry {
            id: id.clone(),
            path: path.clone(),
        });
    }
    out
}

fn collect_tracking_diagnostic_counts(
    diagnostics: Option<&[core_templates::TaskDiagnostic]>,
) -> (Option<usize>, Option<usize>) {
    let Some(diagnostics) = diagnostics else {
        return (None, None);
    };

    let mut errors = 0;
    let mut warnings = 0;
    for d in diagnostics {
        match d.level.as_str() {
            "error" => errors += 1,
            "warning" => warnings += 1,
            _ => {}
        }
    }

    let errors = if errors > 0 { Some(errors) } else { None };
    let warnings = if warnings > 0 { Some(warnings) } else { None };
    (errors, warnings)
}

#[cfg(test)]
#[path = "instructions_tests.rs"]
mod tests;
