#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn init_update_with_tools_all_preserves_agent_activation_contract() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let repo_path = repo.path().to_string_lossy();
    let argv = ["init", repo_path.as_ref(), "--tools", "all", "--update"];
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    for rel in direct_agent_paths() {
        let contents = std::fs::read_to_string(repo.path().join(&rel)).expect("read direct agent");
        assert!(
            contents.contains("activation: direct"),
            "expected {rel} to declare direct activation"
        );
        assert!(
            !contents.contains("mode: subagent"),
            "expected direct agent {rel} to avoid subagent mode"
        );
    }

    for rel in delegated_agent_paths() {
        let contents =
            std::fs::read_to_string(repo.path().join(&rel)).expect("read delegated agent");
        assert!(
            contents.contains("activation: delegated"),
            "expected {rel} to declare delegated activation"
        );
    }

    for rel in opencode_delegated_agent_paths() {
        let contents =
            std::fs::read_to_string(repo.path().join(&rel)).expect("read opencode delegated agent");
        assert!(
            contents.contains("mode: subagent"),
            "expected OpenCode delegated agent {rel} to remain a subagent"
        );
    }
}

#[test]
fn init_update_adds_activation_to_existing_agent_frontmatter() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".opencode/agents/ito-general.md"),
        "---\ndescription: existing direct\nmodel: old\n---\n\n<!-- ITO:START -->\nstale\n<!-- ITO:END -->\n",
    );
    fixtures::write(
        repo.path().join(".opencode/agents/ito-worker.md"),
        "---\ndescription: existing delegated\nmode: subagent\nmodel: old\n---\n\n<!-- ITO:START -->\nstale\n<!-- ITO:END -->\n",
    );

    let repo_path = repo.path().to_string_lossy();
    let argv = [
        "init",
        repo_path.as_ref(),
        "--tools",
        "opencode",
        "--update",
    ];
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let direct = std::fs::read_to_string(repo.path().join(".opencode/agents/ito-general.md"))
        .expect("read direct");
    assert!(direct.contains("activation: direct"));
    assert!(!direct.contains("mode: subagent"));

    let delegated = std::fs::read_to_string(repo.path().join(".opencode/agents/ito-worker.md"))
        .expect("read delegated");
    assert!(delegated.contains("activation: delegated"));
    assert!(delegated.contains("mode: subagent"));
}

#[test]
fn init_update_installs_ito_plan_command_and_skill_for_all_harnesses() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let repo_path = repo.path().to_string_lossy();
    let argv = ["init", repo_path.as_ref(), "--tools", "all", "--update"];
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    for rel in ito_plan_command_paths() {
        let contents = std::fs::read_to_string(repo.path().join(&rel)).expect("read command");
        assert!(
            contents.contains("Load and follow the `ito-plan` skill"),
            "expected {rel} to load the ito-plan skill"
        );
        assert!(
            contents.contains("$ARGUMENTS"),
            "expected {rel} to pass through user arguments"
        );
    }

    for rel in ito_plan_skill_paths() {
        let contents = std::fs::read_to_string(repo.path().join(&rel)).expect("read skill");
        assert!(
            contents.contains("name: ito-plan") && contents.contains("# Plan Before Proposal"),
            "expected {rel} to contain planning workflow guidance"
        );
        assert!(
            contents.contains("Proposal Handoff Format"),
            "expected {rel} to describe proposal handoff guidance"
        );
    }

    for rel in generic_ito_skill_paths() {
        let contents = std::fs::read_to_string(repo.path().join(&rel)).expect("read ito skill");
        assert!(
            contents.contains("`ito plan init/status` are CLI workspace commands"),
            "expected {rel} to keep ito plan routed to the CLI"
        );
    }
}

fn direct_agent_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for root in [
        ".opencode/agents",
        ".claude/agents",
        ".github/agents",
        ".pi/agents",
    ] {
        for name in ["ito-general", "ito-thinking", "ito-orchestrator"] {
            paths.push(format!("{root}/{name}.md"));
        }
    }
    for name in ["ito-general", "ito-thinking", "ito-orchestrator"] {
        paths.push(format!(".agents/skills/{name}/SKILL.md"));
    }
    paths
}

fn delegated_agent_paths() -> Vec<String> {
    let mut paths = fixtures::installed_specialist_asset_paths();
    for root in [
        ".opencode/agents",
        ".claude/agents",
        ".github/agents",
        ".pi/agents",
    ] {
        paths.push(format!("{root}/ito-quick.md"));
    }
    paths.push(".opencode/agents/ito-test-runner.md".to_string());
    paths.push(".agents/skills/ito-quick/SKILL.md".to_string());
    paths
}

fn opencode_delegated_agent_paths() -> Vec<String> {
    vec![
        ".opencode/agents/ito-quick.md".to_string(),
        ".opencode/agents/ito-planner.md".to_string(),
        ".opencode/agents/ito-researcher.md".to_string(),
        ".opencode/agents/ito-worker.md".to_string(),
        ".opencode/agents/ito-reviewer.md".to_string(),
        ".opencode/agents/ito-test-runner.md".to_string(),
    ]
}

fn ito_plan_command_paths() -> Vec<String> {
    vec![
        ".claude/commands/ito-plan.md".to_string(),
        ".codex/prompts/ito-plan.md".to_string(),
        ".github/prompts/ito-plan.prompt.md".to_string(),
        ".opencode/commands/ito-plan.md".to_string(),
        ".pi/commands/ito-plan.md".to_string(),
    ]
}

fn ito_plan_skill_paths() -> Vec<String> {
    vec![
        ".claude/skills/ito-plan/SKILL.md".to_string(),
        ".codex/skills/ito-plan/SKILL.md".to_string(),
        ".github/skills/ito-plan/SKILL.md".to_string(),
        ".opencode/skills/ito-plan/SKILL.md".to_string(),
        ".pi/skills/ito-plan/SKILL.md".to_string(),
    ]
}

fn generic_ito_skill_paths() -> Vec<String> {
    vec![
        ".claude/skills/ito/SKILL.md".to_string(),
        ".codex/skills/ito/SKILL.md".to_string(),
        ".github/skills/ito/SKILL.md".to_string(),
        ".opencode/skills/ito/SKILL.md".to_string(),
        ".pi/skills/ito/SKILL.md".to_string(),
    ]
}
