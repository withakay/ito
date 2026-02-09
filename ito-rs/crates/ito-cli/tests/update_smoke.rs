use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

fn write_local_ito_skills(root: &Path) {
    // `ito update` installs adapter files for all tool ids, which in turn
    // installs ito-skills assets. In tests we avoid network fetches by
    // providing a local `ito-skills/` directory.
    let base = root.join("ito-skills");

    // Minimal adapter files.
    write(
        base.join("adapters/opencode/ito-skills.js"),
        "// test plugin\n",
    );
    write(
        base.join("adapters/claude/session-start.sh"),
        "#!/usr/bin/env sh\necho test\n",
    );
    write(base.join(".codex/ito-skills-bootstrap.md"), "# Bootstrap\n");

    // Must match ito-core `distribution.rs` ITO_SKILLS list.
    let skills = [
        "brainstorming",
        "dispatching-parallel-agents",
        "finishing-a-development-branch",
        "receiving-code-review",
        "requesting-code-review",
        "research",
        "subagent-driven-development",
        "systematic-debugging",
        "test-driven-development",
        "using-git-worktrees",
        "using-ito-skills",
        "verification-before-completion",
        "writing-skills",
    ];
    for skill in skills {
        write(
            base.join(format!("skills/{skill}/SKILL.md")),
            &format!("# {skill}\n"),
        );
    }
}

#[test]
fn update_installs_adapter_files_from_local_ito_skills() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    write(repo.path().join("README.md"), "# temp\n");
    write_local_ito_skills(repo.path());

    // Update should succeed without network when local ito-skills is present.
    let out = run_rust_candidate(rust_path, &["update", "."], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Spot-check adapter outputs.
    assert!(repo.path().join(".opencode/plugins/ito-skills.js").exists());
    assert!(repo.path().join(".claude/session-start.sh").exists());
    assert!(
        repo.path()
            .join(".codex/instructions/ito-skills-bootstrap.md")
            .exists()
    );
    assert!(
        repo.path()
            .join(".opencode/skills/ito-brainstorming/SKILL.md")
            .exists()
    );
}

#[test]
fn update_renders_agents_md_without_jinja2_syntax() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    write(repo.path().join("README.md"), "# temp\n");
    write_local_ito_skills(repo.path());

    let out = run_rust_candidate(rust_path, &["update", "."], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // AGENTS.md should be rendered (Jinja2 resolved), not raw template.
    let agents_md = repo.path().join("AGENTS.md");
    assert!(agents_md.exists(), "AGENTS.md should be installed");

    let content = std::fs::read_to_string(&agents_md).unwrap();
    assert!(
        !content.contains("{%"),
        "AGENTS.md should not contain raw Jinja2 block syntax after rendering"
    );
    assert!(
        content.contains("ito agent instruction worktrees"),
        "AGENTS.md should delegate worktree guidance to the CLI"
    );
}

#[test]
fn update_preserves_project_config_and_project_md() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    write(repo.path().join("README.md"), "# temp\n");
    write_local_ito_skills(repo.path());

    // Seed user edits that must survive `ito update`.
    write(
        repo.path().join(".ito/project.md"),
        "# My Project\n\nuser-edited project.md\n",
    );
    write(
        repo.path().join(".ito/config.json"),
        "{\n  \"custom\": true\n}\n",
    );

    let out = run_rust_candidate(rust_path, &["update", "."], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let project_md = std::fs::read_to_string(repo.path().join(".ito/project.md")).unwrap();
    assert!(project_md.contains("user-edited project.md"));

    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    assert!(config.contains("\"custom\": true"));
}
