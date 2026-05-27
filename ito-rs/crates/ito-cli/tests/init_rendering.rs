#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn init_renders_agents_md_without_raw_jinja2_syntax() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let args = fixtures::init_minimal_args(repo.path());
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);

    // AGENTS.md must exist and be rendered (no raw Jinja2 syntax).
    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    assert!(
        !agents.contains("{%"),
        "AGENTS.md should not contain raw Jinja2 block syntax: {{% ...\nGot:\n{agents}"
    );
    assert!(
        !agents.contains("{{"),
        "AGENTS.md should not contain raw Jinja2 variable syntax: {{{{ ...\nGot:\n{agents}"
    );

    // Worktree guidance should render explicit disabled-state guidance by default.
    assert!(
        agents.contains("Worktrees are not configured for this project."),
        "AGENTS.md should render explicit disabled-state worktree guidance\nGot:\n{agents}"
    );
}

#[test]
fn init_renders_skill_files_without_raw_jinja2_syntax() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write_local_ito_skills(repo.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "claude",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);

    // The using-git-worktrees skill should be installed and rendered.
    let skill_path = repo
        .path()
        .join(".claude/skills/ito-using-git-worktrees/SKILL.md");
    assert!(skill_path.exists(), "worktree skill should be installed");

    let skill = std::fs::read_to_string(&skill_path).unwrap();
    assert!(
        !skill.contains("{%"),
        "Skill file should not contain raw Jinja2 block syntax\nGot:\n{skill}"
    );
    assert!(
        !skill.contains("{{"),
        "Skill file should not contain raw Jinja2 variable syntax\nGot:\n{skill}"
    );

    // Skill should render explicit disabled-state guidance by default.
    assert!(
        skill.contains("Worktrees are not configured for this project."),
        "Skill file should render disabled-state worktree guidance\nGot:\n{skill}"
    );
}

#[test]
fn init_update_renders_agents_md_without_raw_jinja2() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // First, do a normal init.
    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "initial init failed: {}", out.stderr);

    // Run init --update.
    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--update",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "init --update failed: {}", out.stderr);

    // AGENTS.md must be rendered (no raw Jinja2).
    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    assert!(
        !agents.contains("{%"),
        "AGENTS.md should not contain raw Jinja2 after --update\nGot:\n{agents}"
    );
    assert!(
        !agents.contains("{{"),
        "AGENTS.md should not contain raw Jinja2 after --update\nGot:\n{agents}"
    );
    assert!(
        agents.contains("Worktrees are not configured for this project."),
        "AGENTS.md should render disabled-state guidance after --update\nGot:\n{agents}"
    );
}

// ─── --upgrade flag tests ────────────────────────────────────────────────────

/// Verifies that running `ito init --upgrade` refreshes the ITO-managed block in AGENTS.md and preserves user-authored content outside the Ito markers.
///
/// The test performs an initial `ito init` to create AGENTS.md with managed markers, appends user content outside the managed block, runs `ito init --upgrade`, and asserts that the ITO markers are present and the user content remains.
///
/// # Examples
///
/// ```
/// // Integration test; execute with `cargo test` (this function is a test case).
/// ```
#[test]
fn init_upgrade_refreshes_marker_managed_block_and_preserves_user_content() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // First, do a normal init to install AGENTS.md with markers.
    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "initial init failed: {}", out.stderr);

    // Append user-authored content outside the managed block.
    let agents_content = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    let updated_agents =
        format!("{agents_content}\n## My Team Section\nUser authored content here.\n");
    fixtures::write(repo.path().join("AGENTS.md"), &updated_agents);

    // Run init --upgrade — should succeed and preserve user content outside markers.
    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--upgrade",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "init --upgrade failed: {}", out.stderr);

    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    assert!(
        agents.contains("My Team Section"),
        "user content outside markers should be preserved after --upgrade"
    );
    assert!(
        agents.contains("<!-- ITO:START -->"),
        "managed markers should be present after --upgrade"
    );
}
