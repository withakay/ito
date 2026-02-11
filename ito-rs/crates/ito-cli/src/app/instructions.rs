use crate::cli::{AgentArgs, AgentCommand, AgentInstructionArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_config::load_cascading_project_config;
use ito_core::change_repository::FsChangeRepository;
use ito_core::module_repository::FsModuleRepository;
use ito_core::workflow as core_workflow;
use std::collections::BTreeMap;
use std::path::Path;

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
            println!(
                "{}",
                super::common::render_command_long_help(&["agent"], "ito agent")
            );
            Ok(())
        }
    }
}

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
        let valid_tools = ["opencode", "claude", "codex"];
        if !valid_tools.contains(&tool.as_str()) {
            return fail(format!(
                "Invalid tool '{}'. Valid tools: {}",
                tool,
                valid_tools.join(", ")
            ));
        }

        let instruction = generate_bootstrap_instruction(&tool);
        if want_json {
            let response = core_workflow::AgentInstructionResponse {
                artifact_id: "bootstrap".to_string(),
                instruction,
            };
            let rendered = serde_json::to_string_pretty(&response).expect("json should serialize");
            println!("{rendered}");
            return Ok(());
        }

        print!("{instruction}");
        return Ok(());
    }

    if artifact == "project-setup" {
        let instruction = generate_project_setup_instruction();
        if want_json {
            let response = core_workflow::AgentInstructionResponse {
                artifact_id: "project-setup".to_string(),
                instruction,
            };
            let rendered = serde_json::to_string_pretty(&response).expect("json should serialize");
            println!("{rendered}");
            return Ok(());
        }
        print!("{instruction}");
        return Ok(());
    }

    if artifact == "worktrees" || artifact == "workflow" {
        let ctx = rt.ctx();
        let ito_path = rt.ito_path();
        let project_root = ito_path.parent().unwrap_or(ito_path);
        let cfg = load_cascading_project_config(project_root, ito_path, ctx);
        let worktree = worktree_config_from_merged(&cfg.merged);
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

        if want_json {
            let response = core_workflow::AgentInstructionResponse {
                artifact_id: artifact.to_string(),
                instruction,
            };
            let rendered = serde_json::to_string_pretty(&response).expect("json should serialize");
            println!("{rendered}");
            return Ok(());
        }

        print!("{instruction}");
        return Ok(());
    }

    let change = parse_string_flag(args, "--change");
    if change.as_deref().unwrap_or("").is_empty() {
        // Special case: proposal without --change outputs creation guide
        if artifact == "proposal" {
            return handle_new_proposal_guide(rt, want_json);
        }

        let change_repo = FsChangeRepository::new(rt.ito_path());
        let changes = change_repo.list().unwrap_or_default();
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
    let change_repo = FsChangeRepository::new(ito_path);
    let change = change.expect("checked above");
    let change = match super::common::resolve_change_target(&change_repo, &change) {
        Ok(resolved) => resolved,
        Err(msg) => return fail(msg),
    };
    let schema = parse_string_flag(args, "--schema");

    let project_root = ito_path.parent().unwrap_or(ito_path);
    let testing_policy = load_testing_policy(project_root, ito_path, ctx);

    let user_guidance = match core_workflow::load_composed_user_guidance(ito_path, artifact) {
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

        let apply = match core_workflow::compute_apply_instructions(
            ito_path,
            &change,
            schema.as_deref(),
            ctx,
        ) {
            Ok(r) => r,
            Err(core_workflow::WorkflowError::InvalidChangeName) => {
                return fail("Invalid change name");
            }
            Err(core_workflow::WorkflowError::ChangeNotFound(name)) => {
                return fail(format!("Change '{name}' not found"));
            }
            Err(core_workflow::WorkflowError::SchemaNotFound(name)) => {
                return fail(super::common::schema_not_found_message(ctx, &name));
            }
            Err(e) => return Err(to_cli_error(e)),
        };

        if want_json {
            let rendered = serde_json::to_string_pretty(&apply).expect("json should serialize");
            println!("{rendered}");
            return Ok(());
        }

        let worktree_config = load_worktree_config(project_root, ito_path, ctx);
        print_apply_instructions_text(
            &apply,
            &testing_policy,
            user_guidance.as_deref(),
            &worktree_config,
        );
        return Ok(());
    }

    let resolved = match core_workflow::resolve_instructions(
        ito_path,
        &change,
        schema.as_deref(),
        artifact,
        ctx,
    ) {
        Ok(r) => r,
        Err(core_workflow::WorkflowError::InvalidChangeName) => {
            return fail("Invalid change name");
        }
        Err(core_workflow::WorkflowError::ChangeNotFound(name)) => {
            return fail(format!("Change '{name}' not found"));
        }
        Err(core_workflow::WorkflowError::SchemaNotFound(name)) => {
            return fail(super::common::schema_not_found_message(ctx, &name));
        }
        Err(e) => return Err(to_cli_error(e)),
    };

    if want_json {
        let rendered = serde_json::to_string_pretty(&resolved).expect("json should serialize");
        println!("{rendered}");
        return Ok(());
    }
    print_artifact_instructions_text(&resolved, user_guidance.as_deref(), &testing_policy);

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
struct TestingPolicy {
    tdd_workflow: String,
    coverage_target_percent: u64,
}

fn load_testing_policy(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ito_config::ConfigContext,
) -> TestingPolicy {
    let mut out = TestingPolicy {
        tdd_workflow: "red-green-refactor".to_string(),
        coverage_target_percent: 80,
    };

    let cfg = load_cascading_project_config(project_root, ito_path, ctx);
    let merged = cfg.merged;

    if let Some(v) = json_get(&merged, &["defaults", "testing", "tdd", "workflow"])
        && let Some(s) = v.as_str()
    {
        let s = s.trim();
        if !s.is_empty() {
            out.tdd_workflow = s.to_string();
        }
    }

    if let Some(v) = json_get(
        &merged,
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
    match &args.command {
        Some(AgentCommand::Instruction(instr)) => handle_agent_instruction_clap(rt, instr),
        Some(AgentCommand::External(v)) => handle_agent(rt, v),
        None => handle_agent(rt, &[]),
    }
}

fn handle_agent_instruction_clap(rt: &Runtime, args: &AgentInstructionArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();
    argv.push(args.artifact.clone());
    if let Some(change) = &args.change {
        argv.push("--change".to_string());
        argv.push(change.clone());
    }
    if let Some(tool) = &args.tool {
        argv.push("--tool".to_string());
        argv.push(tool.clone());
    }
    if let Some(schema) = &args.schema {
        argv.push("--schema".to_string());
        argv.push(schema.clone());
    }
    if args.json {
        argv.push("--json".to_string());
    }
    handle_agent_instruction(rt, &argv)
}

fn generate_bootstrap_instruction(tool: &str) -> String {
    #[derive(serde::Serialize)]
    struct Ctx<'a> {
        tool: &'a str,
    }

    ito_templates::instructions::render_instruction_template("agent/bootstrap.md.j2", &Ctx { tool })
        .expect("bootstrap instruction template should render")
}

fn generate_project_setup_instruction() -> String {
    #[derive(serde::Serialize)]
    struct Ctx {}

    ito_templates::instructions::render_instruction_template("agent/project-setup.md.j2", &Ctx {})
        .expect("project-setup instruction template should render")
}

fn handle_new_proposal_guide(rt: &Runtime, want_json: bool) -> CliResult<()> {
    #[derive(serde::Serialize)]
    struct ModuleEntry {
        id: String,
        name: String,
    }

    #[derive(serde::Serialize)]
    struct Ctx {
        modules: Vec<ModuleEntry>,
    }

    let module_repo = FsModuleRepository::new(rt.ito_path());
    let modules = module_repo.list().unwrap_or_default();
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
            .expect("new-proposal instruction template should render");

    if want_json {
        let response = core_workflow::AgentInstructionResponse {
            artifact_id: "new-proposal".to_string(),
            instruction,
        };
        let rendered = serde_json::to_string_pretty(&response).expect("json should serialize");
        println!("{rendered}");
        return Ok(());
    }

    print!("{instruction}");
    Ok(())
}

fn print_artifact_instructions_text(
    instructions: &core_workflow::InstructionsResponse,
    user_guidance: Option<&str>,
    testing_policy: &TestingPolicy,
) {
    #[derive(Debug, Clone, serde::Serialize)]
    struct TemplateDependency {
        id: String,
        status: String,
        path: String,
        description: String,
    }

    #[derive(serde::Serialize)]
    struct Ctx {
        instructions: core_workflow::InstructionsResponse,
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
        instructions: instructions.clone(),
        missing,
        dependencies,
        out_path: out_path.to_string_lossy().to_string(),
        testing_policy: testing_policy.clone(),
        user_guidance,
    };

    let out =
        ito_templates::instructions::render_instruction_template("agent/artifact.md.j2", &ctx)
            .expect("artifact instruction template should render");

    print!("{out}");
}

/// Worktree configuration serialized for the apply instruction template.
#[derive(Debug, Clone, serde::Serialize)]
struct WorktreeConfig {
    enabled: bool,
    strategy: String,
    layout_base_dir: Option<String>,
    layout_dir_name: String,
    apply_enabled: bool,
    integration_mode: String,
    copy_from_main: Vec<String>,
    setup_commands: Vec<String>,
    default_branch: String,
}

fn worktree_config_from_merged(merged: &serde_json::Value) -> WorktreeConfig {
    let mut out = WorktreeConfig {
        enabled: false,
        strategy: "checkout_subdir".to_string(),
        layout_base_dir: None,
        layout_dir_name: "ito-worktrees".to_string(),
        apply_enabled: true,
        integration_mode: "commit_pr".to_string(),
        copy_from_main: vec![
            ".env".to_string(),
            ".envrc".to_string(),
            ".mise.local.toml".to_string(),
        ],
        setup_commands: Vec::new(),
        default_branch: "main".to_string(),
    };

    if let Some(wt) = merged.get("worktrees") {
        if let Some(v) = wt.get("enabled").and_then(|v| v.as_bool()) {
            out.enabled = v;
        }
        if let Some(v) = wt.get("strategy").and_then(|v| v.as_str()) {
            out.strategy = v.to_string();
        }
        if let Some(v) = wt.get("default_branch").and_then(|v| v.as_str())
            && !v.is_empty()
        {
            out.default_branch = v.to_string();
        }

        if let Some(layout) = wt.get("layout") {
            if let Some(v) = layout.get("base_dir").and_then(|v| v.as_str())
                && !v.is_empty()
            {
                out.layout_base_dir = Some(v.to_string());
            }
            if let Some(v) = layout.get("dir_name").and_then(|v| v.as_str())
                && !v.is_empty()
            {
                out.layout_dir_name = v.to_string();
            }
        }

        if let Some(apply) = wt.get("apply") {
            if let Some(v) = apply.get("enabled").and_then(|v| v.as_bool()) {
                out.apply_enabled = v;
            }
            if let Some(v) = apply.get("integration_mode").and_then(|v| v.as_str())
                && !v.is_empty()
            {
                out.integration_mode = v.to_string();
            }
            if let Some(arr) = apply.get("copy_from_main").and_then(|v| v.as_array()) {
                let mut items = Vec::new();
                for item in arr {
                    if let Some(s) = item.as_str() {
                        items.push(s.to_string());
                    }
                }
                out.copy_from_main = items;
            }
            if let Some(arr) = apply.get("setup_commands").and_then(|v| v.as_array()) {
                let mut items = Vec::new();
                for item in arr {
                    if let Some(s) = item.as_str() {
                        items.push(s.to_string());
                    }
                }
                out.setup_commands = items;
            }
        }
    }

    out
}

fn load_worktree_config(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ito_config::ConfigContext,
) -> WorktreeConfig {
    let cfg = load_cascading_project_config(project_root, ito_path, ctx);
    worktree_config_from_merged(&cfg.merged)
}

fn print_apply_instructions_text(
    instructions: &core_workflow::ApplyInstructionsResponse,
    testing_policy: &TestingPolicy,
    user_guidance: Option<&str>,
    worktree_config: &WorktreeConfig,
) {
    #[derive(serde::Serialize)]
    struct Ctx {
        instructions: core_workflow::ApplyInstructionsResponse,
        testing_policy: TestingPolicy,
        context_files: Vec<ContextFileEntry>,
        tracking_errors: Option<usize>,
        tracking_warnings: Option<usize>,
        user_guidance: Option<String>,
        worktree: WorktreeConfig,
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
    };

    let out = ito_templates::instructions::render_instruction_template("agent/apply.md.j2", &ctx)
        .expect("apply instruction template should render");

    print!("{out}");
}

fn collect_missing_dependencies(instructions: &core_workflow::InstructionsResponse) -> Vec<String> {
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
    diagnostics: Option<&[core_workflow::TaskDiagnostic]>,
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
