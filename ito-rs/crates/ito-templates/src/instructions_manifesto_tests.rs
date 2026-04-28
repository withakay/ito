use super::render_instruction_template;

use serde::Serialize;

#[derive(Serialize)]
struct ChangeCtx {
    present: bool,
    id: &'static str,
    dir: &'static str,
    schema: &'static str,
    module_id: &'static str,
    module_name: &'static str,
    available_artifacts: Vec<&'static str>,
    missing_artifacts: Vec<&'static str>,
}

#[derive(Serialize)]
struct WorktreeCtx {
    enabled: bool,
    strategy: &'static str,
    layout_base_dir: Option<&'static str>,
    layout_dir_name: &'static str,
    apply_enabled: bool,
    apply_integration_mode: &'static str,
    default_branch: &'static str,
}

#[derive(Serialize)]
struct CoordinationCtx {
    enabled: bool,
    name: &'static str,
    storage: &'static str,
    worktree_path: &'static str,
}

#[derive(Serialize)]
struct MemoryCtx {
    capture_configured: bool,
    search_configured: bool,
    query_configured: bool,
    capture_instruction: Option<&'static str>,
    search_instruction: Option<&'static str>,
    query_instruction: Option<&'static str>,
}

#[derive(Serialize)]
struct RenderedInstructionCtx {
    id: &'static str,
    body: &'static str,
}

#[derive(Serialize)]
struct Ctx {
    variant: &'static str,
    mode: &'static str,
    capability_profile: &'static str,
    operation: Option<&'static str>,
    project_path: &'static str,
    generated_at: &'static str,
    state_capsule_json: &'static str,
    change: ChangeCtx,
    worktree: WorktreeCtx,
    coordination: CoordinationCtx,
    config_capsule_json: &'static str,
    memory: MemoryCtx,
    user_guidance: &'static str,
    rendered_instructions: Vec<RenderedInstructionCtx>,
}

fn base_ctx() -> Ctx {
    Ctx {
        variant: "light",
        mode: "manifesto",
        capability_profile: "full",
        operation: None,
        project_path: ".ito",
        generated_at: "2026-04-27T10:00:00Z",
        state_capsule_json: "{\n  \"state\": \"no-change-selected\"\n}",
        change: ChangeCtx {
            present: false,
            id: "",
            dir: "",
            schema: "",
            module_id: "",
            module_name: "",
            available_artifacts: Vec::new(),
            missing_artifacts: Vec::new(),
        },
        worktree: WorktreeCtx {
            enabled: true,
            strategy: "bare_control_siblings",
            layout_base_dir: None,
            layout_dir_name: "ito-worktrees",
            apply_enabled: true,
            apply_integration_mode: "commit_pr",
            default_branch: "main",
        },
        coordination: CoordinationCtx {
            enabled: true,
            name: "ito/internal/changes",
            storage: "worktree",
            worktree_path: "<redacted-path>",
        },
        config_capsule_json: "{\n  \"defaults\": {\n    \"variant\": \"light\"\n  }\n}",
        memory: MemoryCtx {
            capture_configured: false,
            search_configured: false,
            query_configured: false,
            capture_instruction: None,
            search_instruction: None,
            query_instruction: None,
        },
        user_guidance: "Use care.",
        rendered_instructions: Vec::new(),
    }
}

#[test]
fn manifesto_template_renders_minimal_context() {
    let out = render_instruction_template("agent/manifesto.md.j2", &base_ctx()).unwrap();

    assert!(out.contains("Ito Manifesto: Execution Contract"));
    assert!(out.contains("Manifesto variant: `light`"));
    assert!(out.contains("Coordination branch mode is enabled."));
    assert!(out.contains("No change is selected."));
    assert!(out.contains("<user_guidance>"));
}

#[test]
fn manifesto_template_renders_embedded_instruction_entries() {
    let mut ctx = base_ctx();
    ctx.variant = "full";
    ctx.operation = Some("proposal");
    ctx.state_capsule_json = "{\n  \"state\": \"proposal-drafting\"\n}";
    ctx.change = ChangeCtx {
        present: true,
        id: "019-10_manifesto-instruction",
        dir: ".ito/changes/019-10_manifesto-instruction",
        schema: "spec-driven",
        module_id: "019",
        module_name: "templates",
        available_artifacts: vec!["proposal"],
        missing_artifacts: vec!["design", "specs", "tasks"],
    };
    ctx.rendered_instructions = vec![RenderedInstructionCtx {
        id: "proposal",
        body: "Create or refine the proposal.",
    }];

    let out = render_instruction_template("agent/manifesto.md.j2", &ctx).unwrap();

    assert!(out.contains("Manifesto variant: `full`"));
    assert!(out.contains("### `proposal`"));
    assert!(out.contains("Create or refine the proposal."));
}
