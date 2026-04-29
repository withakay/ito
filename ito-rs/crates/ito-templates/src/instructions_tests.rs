use super::*;

use serde::Serialize;

const READ_ONLY_MAIN_RULE: &str = "Treat the main/control checkout";
const BEFORE_WRITE_WORKTREE_RULE: &str = "Before any write operation, create a dedicated change worktree or move into the existing worktree for that change";
const BEFORE_WRITE_THIS_CHANGE_RULE: &str =
    "Before any write operation, create the dedicated worktree for this change or move into it";
const NO_MAIN_WRITE_RULE: &str = "Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work";

fn assert_main_worktree_guardrails(text: &str) {
    assert!(text.contains(READ_ONLY_MAIN_RULE));
    assert!(text.contains(BEFORE_WRITE_WORKTREE_RULE));
    assert!(text.contains(NO_MAIN_WRITE_RULE));
}

fn assert_change_worktree_guardrails(text: &str) {
    assert!(text.contains(READ_ONLY_MAIN_RULE));
    assert!(text.contains(BEFORE_WRITE_THIS_CHANGE_RULE));
    assert!(text.contains(NO_MAIN_WRITE_RULE));
}

fn assert_contains_all(text: &str, snippets: &[&str]) {
    for snippet in snippets {
        assert!(
            text.contains(snippet),
            "missing expected snippet: {snippet}"
        );
    }
}

#[test]
fn render_template_str_renders_from_serialize_ctx() {
    #[derive(Serialize)]
    struct Ctx {
        name: &'static str,
    }

    let out = render_template_str("hello {{ name }}", &Ctx { name: "world" }).unwrap();
    assert_eq!(out, "hello world");
}

#[test]
fn render_template_str_is_strict_on_undefined() {
    #[derive(Serialize)]
    struct Ctx {}

    let err = render_template_str("hello {{ missing }}", &Ctx {}).unwrap_err();
    assert_eq!(err.kind(), minijinja::ErrorKind::UndefinedError);
}

#[test]
fn render_instruction_template_str_trims_block_whitespace() {
    #[derive(Serialize)]
    struct Ctx {
        tasks: Vec<&'static str>,
    }

    let template = "## Tasks\n{% if tasks %}\n### Pending\n{% for task in tasks %}\n- {{ task }}\n{% endfor %}\n{% endif %}\n## Done\n";
    let out = render_instruction_template_str(
        template,
        &Ctx {
            tasks: vec!["Write test", "Fix renderer"],
        },
    )
    .unwrap();

    assert_eq!(
        out,
        "## Tasks\n### Pending\n- Write test\n- Fix renderer\n## Done\n"
    );
}

#[test]
fn render_template_str_preserves_trailing_newline() {
    #[derive(Serialize)]
    struct Ctx {
        name: &'static str,
    }

    let out = render_template_str("hello {{ name }}\n", &Ctx { name: "world" }).unwrap();
    assert_eq!(out, "hello world\n");
}

#[test]
fn list_instruction_templates_is_sorted_and_non_empty() {
    let templates = list_instruction_templates();
    assert!(!templates.is_empty());

    let mut sorted = templates.clone();
    sorted.sort_unstable();
    assert_eq!(templates, sorted);
}

#[test]
fn template_fetchers_work_for_known_and_unknown_paths() {
    let templates = list_instruction_templates();
    let known = *templates
        .first()
        .expect("expected at least one embedded instruction template");

    let bytes = get_instruction_template_bytes(known);
    assert!(bytes.is_some());

    let text = get_instruction_template(known);
    assert!(text.is_some());

    assert_eq!(get_instruction_template_bytes("missing/template.md"), None);
    assert_eq!(get_instruction_template("missing/template.md"), None);
}

#[test]
fn render_instruction_template_returns_not_found_for_missing_template() {
    #[derive(Serialize)]
    struct Ctx {}

    let err = render_instruction_template("missing/template.md", &Ctx {}).unwrap_err();
    assert_eq!(err.kind(), minijinja::ErrorKind::TemplateNotFound);
}

#[test]
fn artifact_template_renders_when_instruction_is_empty() {
    #[derive(Serialize)]
    struct Instructions {
        #[serde(rename = "artifactId")]
        artifact_id: &'static str,
        #[serde(rename = "changeName")]
        change_name: &'static str,
        #[serde(rename = "schemaName")]
        schema_name: &'static str,
        #[serde(rename = "changeDir")]
        change_dir: &'static str,
        #[serde(rename = "outputPath")]
        output_path: &'static str,
        description: &'static str,
        instruction: &'static str,
        template: &'static str,
        dependencies: Vec<&'static str>,
        unlocks: Vec<&'static str>,
    }
    #[derive(Serialize)]
    struct TestingPolicy {
        tdd_workflow: &'static str,
        coverage_target_percent: u64,
    }
    #[derive(Serialize)]
    struct Ctx {
        instructions: Instructions,
        missing: Vec<&'static str>,
        dependencies: Vec<&'static str>,
        out_path: &'static str,
        testing_policy: TestingPolicy,
        user_guidance: Option<&'static str>,
    }

    let ctx = Ctx {
        instructions: Instructions {
            artifact_id: "tasks",
            change_name: "016-18_add-archived-list-filter",
            schema_name: "minimalist",
            change_dir: "/repo/.ito/changes/016-18_add-archived-list-filter",
            output_path: "tasks.md",
            description: "Write the implementation task list.",
            instruction: "",
            template: "## 1. Implementation\n- [ ] 1.1 Add tests\n",
            dependencies: vec![],
            unlocks: vec![],
        },
        missing: vec![],
        dependencies: vec![],
        out_path: "/repo/.ito/changes/016-18_add-archived-list-filter/tasks.md",
        testing_policy: TestingPolicy {
            tdd_workflow: "red-green-refactor",
            coverage_target_percent: 80,
        },
        user_guidance: None,
    };

    let out = render_instruction_template("agent/artifact.md.j2", &ctx).unwrap();

    assert!(out.contains("Create the tasks artifact"));
    assert!(out.contains("Write to: /repo/.ito/changes/016-18_add-archived-list-filter/tasks.md"));
    assert!(!out.contains("<instruction>"));
}

#[test]
fn orchestrate_template_renders_authoritative_policy() {
    #[derive(Serialize)]
    struct Ctx {
        orchestrate_md_path: &'static str,
        orchestrate_md: &'static str,
        workflow_skill_name: &'static str,
        preset_name: &'static str,
        gate_order: Vec<&'static str>,
        recommended_skills: Vec<&'static str>,
        coordinator_agent_name: &'static str,
        harness_name: &'static str,
        agent_roles_md: &'static str,
    }

    let rendered = render_instruction_template(
        "agent/orchestrate.md.j2",
        &Ctx {
            orchestrate_md_path: "/repo/.ito/user-prompts/orchestrate.md",
            orchestrate_md: "---\npreset: generic\n---\n\n## MUST\n- Run tests\n",
            workflow_skill_name: "ito-orchestrator-workflow",
            preset_name: "generic",
            gate_order: vec!["apply-complete", "tests"],
            recommended_skills: vec![],
            coordinator_agent_name: "ito-orchestrator",
            harness_name: "opencode",
            agent_roles_md: "  - `plan-worker`: `ito-planner`\n  - `apply-worker`: `ito-worker`\n  - `review-worker`: `ito-reviewer`",
        },
    )
    .unwrap();

    assert_contains_all(
        &rendered,
        &[
            "Orchestrate: Change Apply Coordination",
            "Source-of-Truth Precedence",
            "`ito agent instruction orchestrate` is the authoritative source of truth",
            "Project `orchestrate.md` guidance is additive local policy",
            "Direct Coordinator Activation",
            "Activate `ito-orchestrator` directly as the coordinator",
            "`ito-general` and `ito-thinking` are also direct entrypoints",
            "Do not dispatch `ito-orchestrator`, `ito-general`, or `ito-thinking` as ordinary worker sub-agents",
            "Delegated Role Agents",
            "`ito-planner`",
            "`ito-researcher`",
            "`ito-worker`",
            "`ito-reviewer`",
            "`ito-test-runner`",
            "Gate Planning",
            "`depends_on`",
            "`preferred_gates`",
            "Run State",
            "`.ito/.state/orchestrate/runs/<run-id>/`",
            "`run.json`",
            "`plan.json`",
            "`events.jsonl`",
            "`changes/<change-id>.json`",
            "Failure and Remediation",
            "`change_id`",
            "`failed_gate`",
            "`rerun_gates`",
            "Resume Behavior",
            "remaining gates",
            "orchestrate.md (Current)",
            "ito-orchestrator-workflow",
            "Preset",
            "Detected harness",
            "`opencode`",
        ],
    );
}

#[test]
fn review_template_renders_conditional_sections() {
    #[derive(Serialize)]
    struct Artifact {
        id: &'static str,
        path: &'static str,
        present: bool,
    }

    #[derive(Serialize)]
    struct ValidationIssue {
        level: &'static str,
        path: &'static str,
        message: &'static str,
        line: Option<u32>,
        column: Option<u32>,
    }

    #[derive(Serialize)]
    struct TaskSummary {
        total: usize,
        complete: usize,
        in_progress: usize,
        pending: usize,
        shelved: usize,
        wave_count: usize,
    }

    #[derive(Serialize)]
    struct AffectedSpec {
        spec_id: &'static str,
        operation: &'static str,
        description: &'static str,
    }

    #[derive(Serialize)]
    struct TestingPolicy {
        tdd_workflow: &'static str,
        coverage_target_percent: u64,
    }

    #[derive(Serialize)]
    struct Ctx {
        change_name: &'static str,
        change_dir: &'static str,
        schema_name: &'static str,
        module_id: &'static str,
        module_name: &'static str,
        artifacts: Vec<Artifact>,
        validation_issues: Vec<ValidationIssue>,
        validation_passed: bool,
        task_summary: TaskSummary,
        affected_specs: Vec<AffectedSpec>,
        user_guidance: &'static str,
        testing_policy: TestingPolicy,
        generated_at: &'static str,
    }

    let ctx = Ctx {
        change_name: "000-01_test-change",
        change_dir: "/tmp/.ito/changes/000-01_test-change",
        schema_name: "spec-driven",
        module_id: "000_ungrouped",
        module_name: "Ungrouped",
        artifacts: vec![
            Artifact {
                id: "proposal",
                path: "/tmp/.ito/changes/000-01_test-change/proposal.md",
                present: true,
            },
            Artifact {
                id: "design",
                path: "/tmp/.ito/changes/000-01_test-change/design.md",
                present: false,
            },
            Artifact {
                id: "tasks",
                path: "/tmp/.ito/changes/000-01_test-change/tasks.md",
                present: true,
            },
            Artifact {
                id: "specs",
                path: "/tmp/.ito/changes/000-01_test-change/specs",
                present: true,
            },
        ],
        validation_issues: vec![ValidationIssue {
            level: "warning",
            path: ".ito/changes/000-01_test-change/tasks.md",
            message: "sample warning",
            line: Some(3),
            column: Some(1),
        }],
        validation_passed: false,
        task_summary: TaskSummary {
            total: 4,
            complete: 1,
            in_progress: 1,
            pending: 2,
            shelved: 0,
            wave_count: 2,
        },
        affected_specs: vec![AffectedSpec {
            spec_id: "agent-instructions",
            operation: "MODIFIED",
            description: "Review routing",
        }],
        user_guidance: "Follow strict review format.",
        testing_policy: TestingPolicy {
            tdd_workflow: "red-green-refactor",
            coverage_target_percent: 80,
        },
        generated_at: "2026-02-19T00:00:00Z",
    };

    let out = render_instruction_template("agent/review.md.j2", &ctx).unwrap();
    assert!(out.contains("Peer Review"));
    assert!(out.contains("## Proposal Review"));
    assert!(out.contains("## Spec Review"));
    assert!(out.contains("## Task Review"));
    assert!(!out.contains("## Design Review"));
    assert!(out.contains("## Testing Policy"));
    assert!(out.contains("<user_guidance>"));
    assert!(out.contains("## Output Format"));
    assert!(out.contains("Verdict: needs-discussion"));
}

#[test]
fn worktrees_template_bare_control_siblings_branches_from_default_branch() {
    #[derive(Serialize)]
    struct WorktreeCtx {
        enabled: bool,
        strategy: &'static str,
        layout_dir_name: &'static str,
        default_branch: &'static str,
        integration_mode: &'static str,
        ito_root: &'static str,
        project_root: &'static str,
        worktree_root: &'static str,
    }

    #[derive(Serialize)]
    struct Ctx {
        worktree: WorktreeCtx,
        loaded_from: Vec<&'static str>,
        ito_dir_name: &'static str,
    }

    let ctx = Ctx {
        worktree: WorktreeCtx {
            enabled: true,
            strategy: "bare_control_siblings",
            layout_dir_name: "ito-worktrees",
            default_branch: "develop",
            integration_mode: "commit_pr",
            ito_root: "/repo/main/.ito",
            project_root: "/repo",
            worktree_root: "/repo/main",
        },
        loaded_from: Vec::new(),
        ito_dir_name: ".ito",
    };

    let out = render_instruction_template("agent/worktrees.md.j2", &ctx).unwrap();
    assert_main_worktree_guardrails(&out);
    assert!(
        out.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(out.contains("Do not reuse one worktree for two changes"));
    assert!(out.contains(
        "git -C \"$PROJECT_ROOT\" worktree add \"$WORKTREES_ROOT/${BRANCH_NAME}\" -b \"${BRANCH_NAME}\" \"develop\""
    ));
}

#[test]
fn schemas_template_includes_fix_and_platform_guidance() {
    #[derive(Serialize)]
    struct Schema<'a> {
        name: &'a str,
        description: &'a str,
        artifacts: Vec<&'a str>,
        source: &'a str,
    }

    #[derive(Serialize)]
    struct Ctx<'a> {
        schemas: Vec<Schema<'a>>,
        recommended_default: &'a str,
    }

    let ctx = Ctx {
        schemas: vec![
            Schema {
                name: "minimalist",
                description: "Lightweight workflow for small changes",
                artifacts: vec!["specs", "tasks"],
                source: "embedded",
            },
            Schema {
                name: "spec-driven",
                description: "Proposal-driven workflow",
                artifacts: vec!["proposal", "specs", "design", "tasks"],
                source: "embedded",
            },
            Schema {
                name: "tdd",
                description: "Test-first workflow",
                artifacts: vec!["spec", "tests", "implementation", "docs"],
                source: "embedded",
            },
        ],
        recommended_default: "spec-driven",
    };

    let out = render_instruction_template("agent/schemas.md.j2", &ctx).unwrap();
    assert!(out.contains("bounded bug fixes or regression-oriented corrections"));
    assert!(out.contains("supporting platform or infrastructure"));
    assert!(out.contains("When in doubt, start from `ito-proposal`"));
}

#[test]
fn apply_template_bare_control_siblings_branches_from_default_branch() {
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
    struct ProgressCtx {
        total: usize,
        complete: usize,
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

    let ctx = Ctx {
        instructions: InstructionsCtx {
            change_name: "000-01_test-change",
            schema_name: "spec-driven",
            state: "ready",
            missing_artifacts: Vec::new(),
            instruction: "Implement the change.",
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
            default_branch: "develop",
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
        memory: MemoryCtx::default(),
    };

    let out = render_instruction_template("agent/apply.md.j2", &ctx).unwrap();
    assert_change_worktree_guardrails(&out);
    assert!(out.contains("Use the full change ID as the branch and primary worktree directory name: `000-01_test-change`"));
    assert!(out.contains("Do not reuse one worktree for two changes"));
    assert!(out.contains(
        "Additional worktrees for this same change must start with `000-01_test-change`"
    ));
    assert!(out.contains(
        "git -C \"$PROJECT_ROOT\" worktree add \"$CHANGE_DIR\" -b \"$CHANGE_NAME\" \"develop\""
    ));
    assert!(out.contains("refreshes coordination state before rendering"));
    assert!(out.contains("ito sync"));
    assert!(out.contains("ito patch change 000-01_test-change proposal"));
    assert!(out.contains("ito write change 000-01_test-change design"));
    let sync_pos = out.find("ito sync").expect("sync instruction");
    let details_pos = out.find("<details>").expect("manual details");
    assert!(
        sync_pos < details_pos,
        "sync should be in the recommended setup path"
    );
    assert_eq!(out[..details_pos].matches("\nito sync\n").count(), 2);
}

#[test]
fn apply_template_checkout_subdir_branches_from_default_branch() {
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
    struct ProgressCtx {
        total: usize,
        complete: usize,
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

    let ctx = Ctx {
        instructions: InstructionsCtx {
            change_name: "000-01_test-change",
            schema_name: "spec-driven",
            state: "ready",
            missing_artifacts: Vec::new(),
            instruction: "Implement the change.",
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
            strategy: "checkout_subdir",
            default_branch: "develop",
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
        memory: MemoryCtx::default(),
    };

    let out = render_instruction_template("agent/apply.md.j2", &ctx).unwrap();
    assert_change_worktree_guardrails(&out);
    assert!(out.contains("Default branch: `develop`"));
    assert!(out.contains(
        "git -C \"$WORKTREE_ROOT\" worktree add \"$CHANGE_DIR\" -b \"$CHANGE_NAME\" \"develop\""
    ));
}

#[test]
fn apply_template_requires_change_worktree_when_apply_setup_disabled() {
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
    struct ProgressCtx {
        total: usize,
        complete: usize,
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

    let out = render_instruction_template(
        "agent/apply.md.j2",
        &Ctx {
            instructions: InstructionsCtx {
                change_name: "000-01_test-change",
                schema_name: "spec-driven",
                state: "ready",
                missing_artifacts: Vec::new(),
                instruction: "Implement the change.",
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
                apply_enabled: false,
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
            memory: MemoryCtx::default(),
        },
    )
    .unwrap();

    assert!(out.contains("Do not write from the main/control checkout"));
    assert!(out.contains("Move into the dedicated change worktree before any write operation"));
    assert!(!out.contains("Work in your current directory"));
}

#[test]
fn new_proposal_template_moves_to_worktree_after_create() {
    #[derive(Serialize)]
    struct ModuleCtx {
        id: &'static str,
        name: &'static str,
    }

    #[derive(Serialize)]
    struct WorktreeCtx {
        enabled: bool,
        layout_dir_name: &'static str,
        default_branch: &'static str,
    }

    #[derive(Serialize)]
    struct Ctx {
        modules: Vec<ModuleCtx>,
        worktree: WorktreeCtx,
    }

    let out = render_instruction_template(
        "agent/new-proposal.md.j2",
        &Ctx {
            modules: vec![ModuleCtx {
                id: "012",
                name: "git-worktrees",
            }],
            worktree: WorktreeCtx {
                enabled: true,
                layout_dir_name: "ito-worktrees",
                default_branch: "main",
            },
        },
    )
    .unwrap();

    assert!(out.contains("Before running any command that writes proposal artifacts"));
    assert!(out.contains(READ_ONLY_MAIN_RULE));
    assert!(out.contains("Do not run `ito create change` from the main/control checkout"));
    assert!(out.contains(NO_MAIN_WRITE_RULE));
    assert!(out.contains("proposal-<short-name>"));
    assert!(out.contains("CHANGE_DIR=$(ito worktree ensure --change \"<change-id>\")"));
    assert!(out.contains("cd \"$CHANGE_DIR\""));
    assert!(out.contains("Run all subsequent file operations from `$CHANGE_DIR`"));
}

#[test]
fn worktree_init_template_includes_fresh_worktree_rules() {
    #[derive(Serialize)]
    struct WorktreeCtx {
        enabled: bool,
        init_include: Vec<&'static str>,
        init_setup: Vec<&'static str>,
    }

    #[derive(Serialize)]
    struct Ctx {
        worktree: WorktreeCtx,
        change: &'static str,
    }

    let out = render_instruction_template(
        "agent/worktree-init.md.j2",
        &Ctx {
            worktree: WorktreeCtx {
                enabled: true,
                init_include: Vec::new(),
                init_setup: Vec::new(),
            },
            change: "012-06_example-change",
        },
    )
    .unwrap();

    assert!(out.contains("Worktree rules:"));
    assert_main_worktree_guardrails(&out);
    assert!(
        out.contains("Use the full change ID as the branch and primary worktree directory name")
    );
    assert!(out.contains("Do not reuse one worktree for two changes"));
    assert!(out.contains("WORKTREE_PATH=$(ito worktree ensure --change '012-06_example-change')"));
}

#[test]
fn repo_sweep_template_renders() {
    #[derive(Serialize)]
    struct Ctx {}

    let rendered = render_instruction_template("agent/repo-sweep.md.j2", &Ctx {}).unwrap();
    assert!(rendered.contains("Sub-Module"));
    assert!(rendered.contains("NNN.SS-NN_name"));
}

#[test]
fn archive_template_renders_generic_guidance_without_change() {
    #[derive(Serialize)]
    struct ArchiveCfg {
        coordination_storage: String,
        main_integration_mode: String,
    }

    #[derive(Serialize)]
    struct Ctx {
        archive: ArchiveCfg,
        change: Option<String>,
        available_changes: Vec<String>,
    }

    let out = render_instruction_template(
        "agent/archive.md.j2",
        &Ctx {
            archive: ArchiveCfg {
                coordination_storage: "worktree".to_string(),
                main_integration_mode: "pull_request".to_string(),
            },
            change: None,
            available_changes: vec![],
        },
    )
    .unwrap();

    assert!(out.contains("ito archive"));
    assert!(out.contains("ito sync"));
    assert_eq!(out.matches("\nito sync\n").count(), 1);
    assert!(out.contains("ito audit reconcile"));
    assert!(out.contains("Default archive main integration mode: `pull_request`"));
    // generic mode must NOT look like a targeted command
    assert!(!out.contains("Archive:"));
}

#[test]
fn archive_template_renders_targeted_instruction_with_change() {
    #[derive(Serialize)]
    struct ArchiveCfg {
        coordination_storage: String,
        main_integration_mode: String,
    }

    #[derive(Serialize)]
    struct Ctx {
        archive: ArchiveCfg,
        change: Option<String>,
        available_changes: Vec<String>,
    }

    let out = render_instruction_template(
        "agent/archive.md.j2",
        &Ctx {
            archive: ArchiveCfg {
                coordination_storage: "worktree".to_string(),
                main_integration_mode: "pull_request_auto_merge".to_string(),
            },
            change: Some("009-02_event-sourced-audit-log".to_string()),
            available_changes: vec![],
        },
    )
    .unwrap();

    assert!(out.contains("009-02_event-sourced-audit-log"));
    assert!(out.contains("ito archive 009-02_event-sourced-audit-log --yes"));
    assert!(out.contains("ito sync"));
    assert_eq!(out.matches("\nito sync\n").count(), 1);
    assert!(out.contains("ito audit reconcile --change 009-02_event-sourced-audit-log"));
    assert!(out.contains("Configured mode: `pull_request_auto_merge`"));
    assert!(out.contains("request auto-merge"));
}

#[test]
fn archive_template_lists_available_changes_in_generic_mode() {
    #[derive(Serialize)]
    struct ArchiveCfg {
        coordination_storage: String,
        main_integration_mode: String,
    }

    #[derive(Serialize)]
    struct Ctx {
        archive: ArchiveCfg,
        change: Option<String>,
        available_changes: Vec<String>,
    }

    let out = render_instruction_template(
        "agent/archive.md.j2",
        &Ctx {
            archive: ArchiveCfg {
                coordination_storage: "embedded".to_string(),
                main_integration_mode: "pull_request".to_string(),
            },
            change: None,
            available_changes: vec!["001-01_init".to_string(), "002-03_cleanup".to_string()],
        },
    )
    .unwrap();

    assert!(out.contains("001-01_init"));
    assert!(out.contains("002-03_cleanup"));
}

#[test]
fn finish_template_prompts_for_archive() {
    #[derive(Serialize)]
    struct Worktree {
        enabled: bool,
        strategy: &'static str,
        layout_dir_name: &'static str,
        default_branch: &'static str,
    }

    #[derive(Serialize)]
    struct ArchiveCfg {
        main_integration_mode: &'static str,
        coordination_storage: &'static str,
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
        worktree: Worktree,
        archive: ArchiveCfg,
        memory: MemoryCtx,
        change: Option<String>,
        archive_prompt_rendered: bool,
    }

    let out = render_instruction_template(
        "agent/finish.md.j2",
        &Ctx {
            worktree: Worktree {
                enabled: true,
                strategy: "bare_control_siblings",
                layout_dir_name: "ito-worktrees",
                default_branch: "main",
            },
            archive: ArchiveCfg {
                main_integration_mode: "pull_request",
                coordination_storage: "worktree",
            },
            memory: MemoryCtx::default(),
            change: Some("025-09_add-worktree-sync-command".to_string()),
            archive_prompt_rendered: true,
        },
    )
    .unwrap();

    assert!(out.contains("Do you want to archive this change now?"));
    assert!(out.contains("ito sync"));
    assert_eq!(out.matches("\nito sync\n").count(), 1);
    assert!(
        out.contains("ito agent instruction archive --change '025-09_add-worktree-sync-command'")
    );
    assert!(out.contains("`changes.archive.main_integration_mode`: `pull_request`"));
    // Wrap-up reminder always renders specs/docs checks; archive is suppressed
    // because the existing archive prompt covers it.
    assert!(out.contains("### Refresh archive and specs"));
    assert!(out.contains("**Specs**:"));
    assert!(out.contains("**Docs**:"));
    assert!(
        !out.contains("**Archive**:"),
        "archive item must be suppressed when prompt is rendered"
    );
    // Memory capture reminder is keyed on memory.capture.configured.
    assert!(
        !out.contains("### Capture memories"),
        "capture reminder must be absent when memory.capture is unconfigured"
    );
}

#[test]
fn finish_template_includes_capture_reminder_when_memory_capture_configured() {
    #[derive(Serialize)]
    struct Worktree {
        enabled: bool,
        strategy: &'static str,
        layout_dir_name: &'static str,
        default_branch: &'static str,
    }
    #[derive(Serialize)]
    struct ArchiveCfg {
        main_integration_mode: &'static str,
        coordination_storage: &'static str,
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
        worktree: Worktree,
        archive: ArchiveCfg,
        memory: MemoryCtx,
        change: Option<String>,
        archive_prompt_rendered: bool,
    }

    let out = render_instruction_template(
        "agent/finish.md.j2",
        &Ctx {
            worktree: Worktree {
                enabled: true,
                strategy: "bare_control_siblings",
                layout_dir_name: "ito-worktrees",
                default_branch: "main",
            },
            archive: ArchiveCfg {
                main_integration_mode: "pull_request",
                coordination_storage: "worktree",
            },
            memory: MemoryCtx {
                capture: MemoryOpState { configured: true },
                ..Default::default()
            },
            change: Some("000-01_test-change".to_string()),
            archive_prompt_rendered: true,
        },
    )
    .unwrap();

    assert!(out.contains("### Capture memories"));
    assert!(out.contains("ito agent instruction memory-capture"));
    assert!(out.contains("### Refresh archive and specs"));
}

#[test]
fn finish_template_includes_archive_check_when_prompt_suppressed() {
    #[derive(Serialize)]
    struct Worktree {
        enabled: bool,
        strategy: &'static str,
        layout_dir_name: &'static str,
        default_branch: &'static str,
    }
    #[derive(Serialize)]
    struct ArchiveCfg {
        main_integration_mode: &'static str,
        coordination_storage: &'static str,
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
        worktree: Worktree,
        archive: ArchiveCfg,
        memory: MemoryCtx,
        change: Option<String>,
        archive_prompt_rendered: bool,
    }

    let out = render_instruction_template(
        "agent/finish.md.j2",
        &Ctx {
            worktree: Worktree {
                enabled: true,
                strategy: "bare_control_siblings",
                layout_dir_name: "ito-worktrees",
                default_branch: "main",
            },
            archive: ArchiveCfg {
                main_integration_mode: "pull_request",
                coordination_storage: "worktree",
            },
            memory: MemoryCtx::default(),
            change: Some("000-01_test-change".to_string()),
            archive_prompt_rendered: false,
        },
    )
    .unwrap();

    assert!(out.contains("**Archive**:"));
    assert!(out.contains("**Specs**:"));
    assert!(out.contains("**Docs**:"));
}
