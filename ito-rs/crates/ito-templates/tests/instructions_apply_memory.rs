use ito_templates::instructions::render_instruction_template;
use serde::Serialize;

#[derive(Serialize)]
struct ProgressCtx {
    total: u64,
    complete: u64,
}

#[derive(Serialize)]
struct InstructionsCtx {
    #[serde(rename = "changeName")]
    change_name: &'static str,
    #[serde(rename = "schemaName")]
    schema_name: &'static str,
    state: &'static str,
    #[serde(rename = "missingArtifacts")]
    missing_artifacts: Vec<&'static str>,
    instruction: &'static str,
    #[serde(rename = "tracksFile")]
    tracks_file: bool,
    #[serde(rename = "tracksPath")]
    tracks_path: &'static str,
    #[serde(rename = "tracksFormat")]
    tracks_format: &'static str,
    progress: ProgressCtx,
    tasks: Vec<&'static str>,
}

#[derive(Serialize)]
struct WorktreeCtx {
    enabled: bool,
    apply_enabled: bool,
    strategy: &'static str,
    default_branch: &'static str,
    layout_dir_name: &'static str,
    integration_mode: &'static str,
    copy_from_main: Vec<&'static str>,
    setup_commands: Vec<&'static str>,
}

#[derive(Serialize)]
struct TestingPolicyCtx {
    tdd_workflow: &'static str,
    coverage_target_percent: u64,
}

#[derive(Serialize, Default)]
struct MemoryOpState {
    configured: bool,
}

#[derive(Serialize, Default)]
struct MemoryCtx {
    capture: MemoryOpState,
    search: MemoryOpState,
    query: MemoryOpState,
}

#[derive(Serialize)]
struct Ctx {
    instructions: InstructionsCtx,
    context_files: Vec<&'static str>,
    worktree: WorktreeCtx,
    tracking_errors: Vec<&'static str>,
    tracking_warnings: Vec<&'static str>,
    testing_policy: TestingPolicyCtx,
    user_guidance: &'static str,
    memory: MemoryCtx,
}

#[test]
fn apply_template_renders_capture_reminder_when_configured() {
    let ctx = Ctx {
        instructions: InstructionsCtx {
            change_name: "029-02_agent-memory-abstraction",
            schema_name: "spec-driven",
            state: "ready",
            missing_artifacts: Vec::new(),
            instruction: "Implement.",
            tracks_file: false,
            tracks_path: "",
            tracks_format: "",
            progress: ProgressCtx {
                total: 0,
                complete: 0,
            },
            tasks: Vec::new(),
        },
        context_files: Vec::new(),
        worktree: WorktreeCtx {
            enabled: true,
            apply_enabled: true,
            strategy: "bare_control_siblings",
            default_branch: "main",
            layout_dir_name: "ito-worktrees",
            integration_mode: "commit_pr",
            copy_from_main: Vec::new(),
            setup_commands: Vec::new(),
        },
        tracking_errors: Vec::new(),
        tracking_warnings: Vec::new(),
        testing_policy: TestingPolicyCtx {
            tdd_workflow: "red-green-refactor",
            coverage_target_percent: 80,
        },
        user_guidance: "",
        memory: MemoryCtx {
            capture: MemoryOpState { configured: true },
            ..Default::default()
        },
    };

    let out = render_instruction_template("agent/apply.md.j2", &ctx).unwrap();
    assert!(out.contains("### Capture memories"));
    assert!(out.contains("ito agent instruction memory-capture"));
    assert!(out.contains("ito patch change 029-02_agent-memory-abstraction proposal"));
    assert!(out.contains("ito write change 029-02_agent-memory-abstraction design"));
}

#[test]
fn apply_template_omits_capture_reminder_when_search_only_configured() {
    let ctx = Ctx {
        instructions: InstructionsCtx {
            change_name: "test",
            schema_name: "spec-driven",
            state: "ready",
            missing_artifacts: Vec::new(),
            instruction: "Implement.",
            tracks_file: false,
            tracks_path: "",
            tracks_format: "",
            progress: ProgressCtx {
                total: 0,
                complete: 0,
            },
            tasks: Vec::new(),
        },
        context_files: Vec::new(),
        worktree: WorktreeCtx {
            enabled: true,
            apply_enabled: true,
            strategy: "bare_control_siblings",
            default_branch: "main",
            layout_dir_name: "ito-worktrees",
            integration_mode: "commit_pr",
            copy_from_main: Vec::new(),
            setup_commands: Vec::new(),
        },
        tracking_errors: Vec::new(),
        tracking_warnings: Vec::new(),
        testing_policy: TestingPolicyCtx {
            tdd_workflow: "red-green-refactor",
            coverage_target_percent: 80,
        },
        user_guidance: "",
        memory: MemoryCtx {
            search: MemoryOpState { configured: true },
            ..Default::default()
        },
    };

    let out = render_instruction_template("agent/apply.md.j2", &ctx).unwrap();
    assert!(
        !out.contains("### Capture memories"),
        "capture reminder must be absent when only search is configured"
    );
}
