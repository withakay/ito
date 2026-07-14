#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;
use std::collections::BTreeSet;

#[test]
fn init_update_installs_exact_lifecycle_skill_inventory() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let repo_path = repo.path().to_string_lossy();
    let argv = ["init", repo_path.as_ref(), "--tools", "all", "--update"];
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let expected = [
        "ito",
        "ito-proposal",
        "ito-research",
        "ito-apply",
        "ito-review",
        "ito-archive",
        "ito-loop",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();

    for root in [
        ".claude/skills",
        ".codex/skills",
        ".github/skills",
        ".opencode/skills",
        ".pi/skills",
    ] {
        let actual = installed_skill_names(&repo.path().join(root));
        assert_eq!(actual, expected, "unexpected installed skills under {root}");
        assert!(
            actual.contains("ito-loop"),
            "ito-loop must remain installed"
        );
    }

    assert!(
        installed_skill_names(&repo.path().join(".agents/skills")).is_empty(),
        "Codex role definitions must not be installed as discoverable skills"
    );

    let expected_commands = expected;
    for (root, suffix) in [
        (".claude/commands", ".md"),
        (".codex/prompts", ".md"),
        (".github/prompts", ".prompt.md"),
        (".opencode/commands", ".md"),
        (".pi/commands", ".md"),
    ] {
        let actual = installed_command_names(&repo.path().join(root), suffix);
        assert_eq!(
            actual, expected_commands,
            "unexpected commands under {root}"
        );
    }
    assert!(
        !repo
            .path()
            .join(".codex/commands/ito-project-setup.md")
            .exists(),
        "retired project-setup wrapper must not be installed"
    );
}

fn installed_skill_names(root: &std::path::Path) -> BTreeSet<String> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return BTreeSet::new();
    };

    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("SKILL.md").is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}

fn installed_command_names(root: &std::path::Path, suffix: &str) -> BTreeSet<String> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return BTreeSet::new();
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter_map(|name| name.strip_suffix(suffix).map(str::to_owned))
        .collect()
}

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
fn init_update_routes_planning_through_proposal_for_all_harnesses() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let repo_path = repo.path().to_string_lossy();
    let argv = ["init", repo_path.as_ref(), "--tools", "all", "--update"];
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    for rel in ito_plan_surface_paths() {
        assert!(
            !repo.path().join(&rel).exists(),
            "retired planning surface should not be installed: {rel}"
        );
    }

    for rel in ito_proposal_skill_paths() {
        let contents = std::fs::read_to_string(repo.path().join(&rel)).expect("read proposal");
        assert!(contents.contains("name: ito-proposal"));
        assert!(contents.contains("pre-proposal planning"));
        assert!(contents.contains("ito plan init"));
    }

    for rel in generic_ito_skill_paths() {
        let contents = std::fs::read_to_string(repo.path().join(&rel)).expect("read ito skill");
        assert!(
            contents.contains("`ito plan init|status`"),
            "expected {rel} to keep planning workspace commands on the CLI"
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

fn ito_plan_surface_paths() -> Vec<String> {
    let mut paths = vec![
        ".claude/commands/ito-plan.md".to_string(),
        ".codex/prompts/ito-plan.md".to_string(),
        ".github/prompts/ito-plan.prompt.md".to_string(),
        ".opencode/commands/ito-plan.md".to_string(),
        ".pi/commands/ito-plan.md".to_string(),
    ];
    paths.extend([
        ".claude/skills/ito-plan/SKILL.md".to_string(),
        ".codex/skills/ito-plan/SKILL.md".to_string(),
        ".github/skills/ito-plan/SKILL.md".to_string(),
        ".opencode/skills/ito-plan/SKILL.md".to_string(),
        ".pi/skills/ito-plan/SKILL.md".to_string(),
    ]);
    paths
}

fn ito_proposal_skill_paths() -> Vec<String> {
    vec![
        ".claude/skills/ito-proposal/SKILL.md".to_string(),
        ".codex/skills/ito-proposal/SKILL.md".to_string(),
        ".github/skills/ito-proposal/SKILL.md".to_string(),
        ".opencode/skills/ito-proposal/SKILL.md".to_string(),
        ".pi/skills/ito-proposal/SKILL.md".to_string(),
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
