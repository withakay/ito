use crate::cli::{AgentArgs, AgentCommand, AgentInstructionArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_config::types::WorktreeStrategy;
use ito_config::{load_cascading_project_config, resolve_coordination_branch_settings};
use ito_core::change_repository::FsChangeRepository;
use ito_core::git::{CoordinationGitErrorKind, fetch_coordination_branch};
use ito_core::module_repository::FsModuleRepository;
use ito_core::templates as core_templates;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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

/// Handle the "agent instruction" subcommand, generating and printing instructions for a requested artifact.
///
/// This function processes the provided CLI arguments for the `ito agent instruction` subcommand and
/// performs one of:
/// - print long help when no arguments or help flag is present,
/// - render and print bootstrap, project-setup, worktrees, proposal, apply, or other artifact instructions,
/// - validate required flags (e.g., `--tool` for `bootstrap`, `--change` for change-scoped artifacts),
/// - output either plain instruction text or a JSON-wrapped response when `--json` is provided.
///
/// It also loads configuration, testing policy, and optional user guidance as needed and surfaces
/// user-facing error messages for common failure cases.
///
/// # Examples
///
/// ```no_run
/// use ito_cli::app::instructions::handle_agent_instruction;
/// // `rt` should be a properly constructed `Runtime` from the hosting application.
/// // The following demonstrates calling the handler to print help (no arguments).
/// let rt = /* construct Runtime here */;
/// let args: Vec<String> = vec![];
/// let _ = handle_agent_instruction(&rt, &args);
/// ```
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
            let response = core_templates::AgentInstructionResponse {
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
            let response = core_templates::AgentInstructionResponse {
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

    if artifact == "worktrees" {
        let ctx = rt.ctx();
        let ito_path = rt.ito_path();
        let project_root = ito_path.parent().unwrap_or(ito_path);
        let cfg = load_cascading_project_config(project_root, ito_path, ctx);
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

        if want_json {
            let response = core_templates::AgentInstructionResponse {
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
        if artifact == "review" {
            return fail("review instruction requires --change <id>");
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

        let (coord_enabled, coord_branch) =
            load_coordination_branch_settings(project_root, ito_path, ctx);
        if coord_enabled
            && let Err(err) = fetch_coordination_branch(project_root, &coord_branch)
            && err.kind != CoordinationGitErrorKind::RemoteMissing
        {
            eprintln!(
                "Warning: failed to sync coordination branch '{}' before apply instructions: {}",
                coord_branch, err.message
            );
        }

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

    if artifact == "review" {
        let review =
            match core_templates::compute_review_context(ito_path, &change, schema.as_deref(), ctx, user_guidance.clone())
            {
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

        if want_json {
            let response = core_templates::AgentInstructionResponse {
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

    let resolved = match core_templates::resolve_instructions(
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

fn load_coordination_branch_settings(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ito_config::ConfigContext,
) -> (bool, String) {
    let merged = load_cascading_project_config(project_root, ito_path, ctx).merged;
    resolve_coordination_branch_settings(&merged)
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
        let response = core_templates::AgentInstructionResponse {
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
    instructions: &core_templates::InstructionsResponse,
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
        instructions: core_templates::InstructionsResponse,
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
    strategy: WorktreeStrategy,
    layout_base_dir: Option<String>,
    layout_dir_name: String,
    apply_enabled: bool,
    integration_mode: String,
    copy_from_main: Vec<String>,
    setup_commands: Vec<String>,
    default_branch: String,
    /// Absolute path to the current working worktree root.
    ///
    /// This is the directory that contains the `.ito/` folder for this invocation.
    worktree_root: Option<String>,
    /// Absolute path to the `.ito/` directory for this invocation.
    ito_root: Option<String>,
    /// Absolute path to the project/repo root directory.
    ///
    /// For `BareControlSiblings` this is the bare repo root (where `.bare/`
    /// and `.git` live), resolved via `git rev-parse --git-common-dir`.
    /// Templates use this to emit absolute paths so agents create worktrees
    /// in the correct location regardless of their cwd.
    project_root: Option<String>,
}

/// Resolve the bare repo root for `bare_control_siblings` layouts.
///
/// Runs `git rev-parse --path-format=absolute --git-common-dir` from
/// `project_root` and returns its parent directory. For a bare repo
/// where `.bare/` holds the git objects, this gives the directory
/// containing `.bare/`, `.git`, and the worktree directories.
fn resolve_bare_repo_root(project_root: &Path) -> Option<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(project_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let common_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if common_dir.is_empty() {
        return None;
    }
    Path::new(&common_dir).parent().map(Path::to_path_buf)
}

/// Build a WorktreeConfig from a merged project JSON configuration, using an optional
/// project_root to resolve absolute paths required by templates.
///
/// This reads the `worktrees` section (if present) and populates fields such as
/// `enabled`, `strategy`, `layout_base_dir`, `layout_dir_name`, `apply_enabled`,
/// `integration_mode`, `copy_from_main`, `setup_commands`, and `default_branch`.
/// Empty string values are ignored and leave defaults in place. If `project_root` is
/// provided the function sets `project_root` on the returned config; for the
/// `BareControlSiblings` strategy the root is resolved to the bare repository root
/// (parent of the Git common-dir), otherwise the provided `project_root` is used
/// as-is.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// use std::path::Path;
///
/// let merged = json!({
///     "worktrees": {
///         "enabled": true,
///         "strategy": "checkout_subdir",
///         "layout": { "dir_name": "wt-dir" },
///         "apply": {
///             "enabled": false,
///             "integration_mode": "commit_pr",
///             "copy_from_main": [".env"],
///             "setup_commands": ["echo hi"]
///         },
///         "default_branch": "develop"
///     }
/// });
///
/// let cfg = worktree_config_from_merged(&merged, Some(Path::new("/repo")));
/// assert!(cfg.enabled);
/// assert_eq!(cfg.layout_dir_name, "wt-dir");
/// assert_eq!(cfg.apply_enabled, false);
/// assert_eq!(cfg.default_branch, "develop");
/// ```
fn worktree_config_from_merged(
    merged: &serde_json::Value,
    project_root: Option<&Path>,
) -> WorktreeConfig {
    let mut out = WorktreeConfig {
        enabled: false,
        strategy: WorktreeStrategy::CheckoutSubdir,
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
        worktree_root: None,
        ito_root: None,
        project_root: None,
    };

    if let Some(wt) = merged.get("worktrees") {
        if let Some(v) = wt.get("enabled").and_then(|v| v.as_bool()) {
            out.enabled = v;
        }
        if let Some(v) = wt.get("strategy").and_then(|v| v.as_str())
            && let Some(parsed) = WorktreeStrategy::parse_value(v)
        {
            out.strategy = parsed;
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

    // Resolve the absolute project root for all strategies so templates
    // can emit absolute paths and agents create worktrees in the correct
    // location regardless of their cwd.
    //
    // For BareControlSiblings the root is the bare repo directory (parent
    // of `.bare/`), which may differ from the cwd when running from inside
    // a worktree.  For other strategies it is the checkout root.
    if let Some(root) = project_root {
        out.project_root = match out.strategy {
            WorktreeStrategy::BareControlSiblings => {
                resolve_bare_repo_root(root).map(|p| p.to_string_lossy().to_string())
            }
            WorktreeStrategy::CheckoutSubdir | WorktreeStrategy::CheckoutSiblings => {
                Some(root.to_string_lossy().to_string())
            }
        };
    }

    out
}

fn worktree_config_from_merged_with_paths(
    merged: &serde_json::Value,
    project_root: &Path,
    ito_path: &Path,
) -> WorktreeConfig {
    let mut out = worktree_config_from_merged(merged, Some(project_root));
    out.worktree_root = Some(project_root.to_string_lossy().to_string());
    out.ito_root = Some(ito_path.to_string_lossy().to_string());
    out
}

/// Builds a WorktreeConfig from the project's cascading configuration and records the project and `.ito` root paths.
///
/// The returned WorktreeConfig is populated from the merged cascading configuration for `project_root` and `ito_path`.
/// Its `worktree_root` and `ito_root` fields are set to the provided `project_root` and `ito_path` (converted to strings).
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// // `ctx` is a `ito_config::ConfigContext` previously constructed
/// let cfg = load_worktree_config(Path::new("/path/to/project"), Path::new("/path/to/project/.ito"), &ctx);
/// assert_eq!(cfg.worktree_root.as_deref(), Some("/path/to/project"));
/// assert_eq!(cfg.ito_root.as_deref(), Some("/path/to/project/.ito"));
/// ```
fn load_worktree_config(
    project_root: &Path,
    ito_path: &Path,
    ctx: &ito_config::ConfigContext,
) -> WorktreeConfig {
    let cfg = load_cascading_project_config(project_root, ito_path, ctx);
    worktree_config_from_merged_with_paths(&cfg.merged, project_root, ito_path)
}

/// Render and print the apply instructions using the agent apply template.
///
/// This builds a template context from `instructions`, `testing_policy`, optional
/// `user_guidance`, and `worktree_config`, collects context-file entries and
/// tracking diagnostic counts, renders the `agent/apply.md.j2` template, and
/// writes the resulting text to standard output.
///
/// The provided `user_guidance` string is trimmed and ignored if empty.
///
/// # Examples
///
/// ```no_run
/// // Prepare or obtain `instructions`, `testing_policy`, `worktree_config`,
/// // and optionally `user_guidance` from the surrounding application logic,
/// // then render and print the apply instructions:
/// // print_apply_instructions_text(&instructions, &testing_policy, Some("Extra notes"), &worktree_config);
/// ```
fn print_apply_instructions_text(
    instructions: &core_templates::ApplyInstructionsResponse,
    testing_policy: &TestingPolicy,
    user_guidance: Option<&str>,
    worktree_config: &WorktreeConfig,
) {
    #[derive(serde::Serialize)]
    struct Ctx {
        instructions: core_templates::ApplyInstructionsResponse,
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
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn worktree_config_defaults_when_no_worktrees_key() {
        let merged = json!({});
        let cfg = worktree_config_from_merged(&merged, None);

        assert!(!cfg.enabled);
        assert_eq!(cfg.strategy, WorktreeStrategy::CheckoutSubdir);
        assert!(cfg.layout_base_dir.is_none());
        assert_eq!(cfg.layout_dir_name, "ito-worktrees");
        assert!(cfg.apply_enabled);
        assert_eq!(cfg.integration_mode, "commit_pr");
        assert_eq!(
            cfg.copy_from_main,
            vec![".env", ".envrc", ".mise.local.toml"]
        );
        assert!(cfg.setup_commands.is_empty());
        assert_eq!(cfg.default_branch, "main");
        assert!(cfg.project_root.is_none());
    }

    #[test]
    fn worktree_config_parses_all_fields() {
        let merged = json!({
            "worktrees": {
                "enabled": true,
                "strategy": "checkout_siblings",
                "default_branch": "develop",
                "layout": {
                    "base_dir": "/tmp/worktrees",
                    "dir_name": "my-trees"
                },
                "apply": {
                    "enabled": false,
                    "integration_mode": "merge",
                    "copy_from_main": [".env.local"],
                    "setup_commands": ["npm install", "cargo build"]
                }
            }
        });
        let cfg = worktree_config_from_merged(&merged, None);

        assert!(cfg.enabled);
        assert_eq!(cfg.strategy, WorktreeStrategy::CheckoutSiblings);
        assert_eq!(cfg.default_branch, "develop");
        assert_eq!(cfg.layout_base_dir.as_deref(), Some("/tmp/worktrees"));
        assert_eq!(cfg.layout_dir_name, "my-trees");
        assert!(!cfg.apply_enabled);
        assert_eq!(cfg.integration_mode, "merge");
        assert_eq!(cfg.copy_from_main, vec![".env.local"]);
        assert_eq!(cfg.setup_commands, vec!["npm install", "cargo build"]);
    }

    #[test]
    fn worktree_config_ignores_empty_strings() {
        let merged = json!({
            "worktrees": {
                "strategy": "",
                "default_branch": "",
                "layout": {
                    "base_dir": "",
                    "dir_name": ""
                },
                "apply": {
                    "integration_mode": ""
                }
            }
        });
        let cfg = worktree_config_from_merged(&merged, None);

        // Should keep defaults when values are empty strings.
        assert_eq!(cfg.strategy, WorktreeStrategy::CheckoutSubdir);
        assert_eq!(cfg.default_branch, "main");
        assert!(cfg.layout_base_dir.is_none());
        assert_eq!(cfg.layout_dir_name, "ito-worktrees");
        assert_eq!(cfg.integration_mode, "commit_pr");
    }

    #[test]
    fn worktree_config_checkout_subdir_sets_project_root() {
        let merged = json!({
            "worktrees": {
                "strategy": "checkout_subdir"
            }
        });
        let root = Path::new("/fake/project");
        let cfg = worktree_config_from_merged(&merged, Some(root));

        assert_eq!(cfg.project_root.as_deref(), Some("/fake/project"));
    }

    #[test]
    fn worktree_config_checkout_siblings_sets_project_root() {
        let merged = json!({
            "worktrees": {
                "strategy": "checkout_siblings"
            }
        });
        let root = Path::new("/fake/project");
        let cfg = worktree_config_from_merged(&merged, Some(root));

        assert_eq!(cfg.project_root.as_deref(), Some("/fake/project"));
    }

    #[test]
    fn worktree_config_bare_control_siblings_calls_resolve() {
        let merged = json!({
            "worktrees": {
                "strategy": "bare_control_siblings"
            }
        });
        // With no project_root passed, project_root stays None.
        let cfg = worktree_config_from_merged(&merged, None);
        assert!(cfg.project_root.is_none());

        // With a project_root in a real git repo, resolve_bare_repo_root
        // will return Some (the enclosing repo's root).  We just verify
        // the function runs without panicking and produces a path.
        let cfg = worktree_config_from_merged(&merged, Some(Path::new(".")));
        // Inside the test workspace this will resolve; in a non-git
        // context it would be None.  Either outcome is acceptable.
        if let Some(ref root) = cfg.project_root {
            assert!(!root.is_empty());
        }
    }

    #[test]
    fn worktree_config_no_project_root_when_none_passed() {
        let merged = json!({
            "worktrees": {
                "strategy": "checkout_siblings"
            }
        });
        let cfg = worktree_config_from_merged(&merged, None);
        assert!(cfg.project_root.is_none());
    }

    #[test]
    fn json_get_traverses_nested_keys() {
        let root = json!({
            "a": {
                "b": {
                    "c": 42
                }
            }
        });

        let result = json_get(&root, &["a", "b", "c"]);
        assert_eq!(result, Some(&json!(42)));
    }

    #[test]
    fn json_get_returns_none_for_missing_key() {
        let root = json!({"a": {"b": 1}});
        assert!(json_get(&root, &["a", "x"]).is_none());
    }

    #[test]
    fn json_get_returns_none_for_non_object_intermediate() {
        let root = json!({"a": 42});
        assert!(json_get(&root, &["a", "b"]).is_none());
    }

    #[test]
    fn json_get_empty_keys_returns_root() {
        let root = json!({"a": 1});
        assert_eq!(json_get(&root, &[]), Some(&root));
    }

    #[test]
    fn collect_tracking_diagnostic_counts_none_input() {
        let (errors, warnings) = collect_tracking_diagnostic_counts(None);
        assert!(errors.is_none());
        assert!(warnings.is_none());
    }

    #[test]
    fn collect_tracking_diagnostic_counts_empty_slice() {
        let diags: Vec<core_templates::TaskDiagnostic> = vec![];
        let (errors, warnings) = collect_tracking_diagnostic_counts(Some(&diags));
        assert!(errors.is_none());
        assert!(warnings.is_none());
    }

    #[test]
    fn collect_tracking_diagnostic_counts_mixed_levels() {
        let diags = vec![
            core_templates::TaskDiagnostic {
                level: "error".to_string(),
                message: "e1".to_string(),
                task_id: None,
            },
            core_templates::TaskDiagnostic {
                level: "warning".to_string(),
                message: "w1".to_string(),
                task_id: None,
            },
            core_templates::TaskDiagnostic {
                level: "error".to_string(),
                message: "e2".to_string(),
                task_id: None,
            },
            core_templates::TaskDiagnostic {
                level: "info".to_string(),
                message: "i1".to_string(),
                task_id: None,
            },
        ];
        let (errors, warnings) = collect_tracking_diagnostic_counts(Some(&diags));
        assert_eq!(errors, Some(2));
        assert_eq!(warnings, Some(1));
    }

    #[test]
    fn worktree_config_parses_bare_control_siblings_strategy() {
        let merged = json!({
            "worktrees": {
                "strategy": "bare_control_siblings"
            }
        });
        let cfg = worktree_config_from_merged(&merged, None);
        assert_eq!(cfg.strategy, WorktreeStrategy::BareControlSiblings);
    }

    #[test]
    fn collect_context_files_preserves_order() {
        let mut map = BTreeMap::new();
        map.insert("alpha".to_string(), "path/a".to_string());
        map.insert("beta".to_string(), "path/b".to_string());

        let entries = collect_context_files(&map);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, "alpha");
        assert_eq!(entries[0].path, "path/a");
        assert_eq!(entries[1].id, "beta");
        assert_eq!(entries[1].path, "path/b");
    }
}
