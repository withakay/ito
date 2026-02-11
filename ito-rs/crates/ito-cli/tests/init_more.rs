#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

// PTY-based interactive tests are skipped on Windows due to platform differences
// in terminal handling that can cause hangs.
#[cfg(unix)]
use ito_test_support::pty::run_pty_interactive;

fn expected_release_tag() -> String {
    let version = option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    if version.starts_with('v') {
        return version.to_string();
    }

    format!("v{version}")
}

#[test]
fn init_requires_tools_when_non_interactive() {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::write(repo.path().join("README.md"), "# temp\n");

    let out = run_rust_candidate(rust_path, &["init"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("requires --tools"));
}

#[test]
fn init_with_tools_none_installs_ito_skeleton() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let args = fixtures::init_minimal_args(repo.path());
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0);

    assert!(repo.path().join(".ito").is_dir());
    assert!(
        repo.path().join(".ito/user-guidance.md").exists()
            || repo.path().join(".ito/specs").exists()
    );
    assert!(repo.path().join(".ito/user-prompts/guidance.md").exists());
    assert!(repo.path().join(".ito/user-prompts/proposal.md").exists());
    assert!(repo.path().join(".ito/user-prompts/apply.md").exists());
    assert!(repo.path().join(".ito/user-prompts/tasks.md").exists());
}

#[test]
fn init_writes_config_with_release_tag_schema_reference() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let args = fixtures::init_minimal_args(repo.path());
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);

    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    let expected_release_tag = expected_release_tag();
    let expected = format!(
        "\"$schema\": \"https://raw.githubusercontent.com/withakay/ito/{expected_release_tag}/schemas/ito-config.schema.json\""
    );
    assert!(
        config.contains(&expected),
        "expected generated .ito/config.json to include a release-tag schema reference\nexpected fragment: {expected}\nGot:\n{config}"
    );
}

#[test]
fn init_help_prints_usage() {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::write(repo.path().join("README.md"), "# temp\n");

    let out = run_rust_candidate(rust_path, &["init", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Usage: ito init"));
}

#[test]
fn init_with_tools_csv_installs_selected_adapters() {
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
            "claude,codex",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    assert!(repo.path().join(".claude/session-start.sh").exists());
    assert!(
        repo.path()
            .join(".codex/instructions/ito-skills-bootstrap.md")
            .exists()
    );
    assert!(!repo.path().join(".opencode").exists());
}

#[test]
fn init_tools_csv_ignores_empty_segments() {
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
            ",claude,,opencode,",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    assert!(repo.path().join(".claude/session-start.sh").exists());
    assert!(repo.path().join(".opencode/plugins/ito-skills.js").exists());
}

#[test]
fn init_refuses_to_overwrite_existing_file_without_markers_when_not_forced() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // AGENTS.md is installed by default; create a conflicting file without markers.
    fixtures::write(repo.path().join("AGENTS.md"), "custom agents\n");

    let args = fixtures::init_minimal_args(repo.path());
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(
        out.stderr
            .contains("Refusing to overwrite existing file without markers")
    );
}

// PTY-based interactive tests are skipped on Windows and in CI due to platform
// and timing differences in terminal handling that cause hangs. The dialoguer
// multi-select widget uses raw-mode input which races with pre-buffered PTY
// input in headless environments. The underlying init logic is cross-platform
// and covered by the non-interactive tests above.
#[test]
#[cfg(unix)]
#[ignore = "PTY interactive test hangs in CI; run locally with --include-ignored"]
fn init_interactive_detects_tools_and_installs_adapter_files() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Ensure adapter installs succeed without network.
    fixtures::write_local_ito_skills(repo.path());

    // Seed tool detection without creating conflicting files that init would refuse to overwrite.
    std::fs::create_dir_all(repo.path().join(".claude")).unwrap();
    std::fs::create_dir_all(repo.path().join(".opencode")).unwrap();

    // Drive the interactive prompt:
    // - step 1: Enter
    // - tool multi-select: Enter to accept defaults
    // - step 3: Enter
    // - worktree wizard: Enter (default: disable)
    let out = run_pty_interactive(
        rust_path,
        &["init", repo.path().to_string_lossy().as_ref()],
        repo.path(),
        home.path(),
        "\n\n\n\n",
    );
    assert_eq!(out.code, 0, "stdout={}", out.stdout);

    // Spot-check adapter outputs from both Claude + OpenCode.
    assert!(repo.path().join(".claude/session-start.sh").exists());
    assert!(repo.path().join(".opencode/plugins/ito-skills.js").exists());
    assert!(
        repo.path()
            .join(".opencode/skills/ito-brainstorming/SKILL.md")
            .exists()
    );

    // Worktree config should be persisted to the per-dev overlay `.ito/config.local.json`.
    let config_path = repo.path().join(".ito/config.local.json");
    let config = std::fs::read_to_string(config_path).unwrap();
    assert!(
        config.contains("\"worktrees\""),
        "expected worktree config to be written to project local config"
    );

    // The global config file should not be created/modified by init.
    let global = home.path().join(".config/ito/config.json");
    assert!(
        !global.exists(),
        "expected init to avoid writing global config.json"
    );
}

#[test]
fn init_tools_parser_covers_all_and_invalid_id() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write_local_ito_skills(repo.path());

    let repo_path = repo.path().to_string_lossy().to_string();
    let args: Vec<String> = vec![
        "init".to_string(),
        repo_path.clone(),
        "--tools".to_string(),
        "all".to_string(),
        "--force".to_string(),
    ];
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={} stdout={}", out.stderr, out.stdout);

    let args: Vec<String> = vec![
        "init".to_string(),
        repo_path,
        "--tools".to_string(),
        "not-a-tool".to_string(),
    ];
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Unknown tool id"));
}

#[test]
fn init_update_preserves_user_files_and_creates_missing() {
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

    // Modify user-owned files to simulate user edits.
    fixtures::write(
        repo.path().join(".ito/project.md"),
        "# My Custom Project\n\nThis is user-authored content.\n",
    );
    fixtures::write(repo.path().join(".ito/config.json"), r#"{"custom": true}"#);

    // Add custom content outside the managed block in AGENTS.md.
    let agents_content = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    let updated_agents = format!("{agents_content}\n## My Custom Section\nUser content here.\n");
    fixtures::write(repo.path().join("AGENTS.md"), &updated_agents);

    // Delete a managed file to verify it gets recreated.
    std::fs::remove_file(repo.path().join(".ito/planning/STATE.md")).unwrap();

    // Run init --update — should succeed without --force.
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

    // User-owned file should be preserved.
    let project_md = std::fs::read_to_string(repo.path().join(".ito/project.md")).unwrap();
    assert!(
        project_md.contains("My Custom Project"),
        "project.md should be preserved"
    );
    assert!(
        project_md.contains("user-authored content"),
        "project.md user content should be intact"
    );

    // config.json should be preserved.
    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    assert!(
        config.contains(r#""custom": true"#),
        "config.json should be preserved"
    );

    // AGENTS.md managed block should be updated, but user content outside markers should be kept.
    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    assert!(
        agents.contains("My Custom Section"),
        "AGENTS.md user content should be preserved"
    );
    assert!(
        agents.contains("<!-- ITO:START -->"),
        "AGENTS.md should still have markers"
    );

    // Deleted file should be recreated.
    assert!(
        repo.path().join(".ito/planning/STATE.md").exists(),
        "STATE.md should be recreated"
    );
}

#[test]
fn init_update_without_prior_init_creates_all_files() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Run init --update on a fresh repo (no prior init).
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
    assert_eq!(
        out.code, 0,
        "init --update on fresh repo failed: {}",
        out.stderr
    );

    // Core files should be created.
    assert!(repo.path().join(".ito").is_dir());
    assert!(repo.path().join(".ito/project.md").exists());
    assert!(repo.path().join(".ito/user-guidance.md").exists());
    assert!(repo.path().join(".ito/user-prompts/guidance.md").exists());
    assert!(repo.path().join(".ito/user-prompts/proposal.md").exists());
    assert!(repo.path().join(".ito/user-prompts/apply.md").exists());
    assert!(repo.path().join(".ito/user-prompts/tasks.md").exists());
    assert!(repo.path().join(".ito/config.json").exists());
    assert!(repo.path().join("AGENTS.md").exists());
}

#[test]
fn init_update_does_not_overwrite_existing_user_prompt_stubs() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

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

    fixtures::write(
        repo.path().join(".ito/user-prompts/tasks.md"),
        "Custom tasks guidance\n",
    );

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

    let tasks_prompt =
        std::fs::read_to_string(repo.path().join(".ito/user-prompts/tasks.md")).unwrap();
    assert!(tasks_prompt.contains("Custom tasks guidance"));
}

#[test]
fn init_force_overwrites_existing_user_prompt_stubs() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

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

    fixtures::write(
        repo.path().join(".ito/user-prompts/tasks.md"),
        "Custom tasks guidance\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--force",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "init --force failed: {}", out.stderr);

    let tasks_prompt =
        std::fs::read_to_string(repo.path().join(".ito/user-prompts/tasks.md")).unwrap();
    assert!(!tasks_prompt.contains("Custom tasks guidance"));
    assert!(tasks_prompt.contains("Tasks Guidance"));
}

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

#[test]
fn init_update_does_not_error_on_existing_agents_md_without_markers() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Create an AGENTS.md without markers (would normally cause init to fail).
    fixtures::write(repo.path().join("AGENTS.md"), "custom agents\n");

    // Without --update: should fail.
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
    assert_ne!(out.code, 0, "plain init should fail");

    // With --update: should succeed by updating the managed block.
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
    assert_eq!(out.code, 0, "init --update should succeed: {}", out.stderr);

    // AGENTS.md should now have the managed block and preserve existing content.
    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    assert!(
        agents.contains("<!-- ITO:START -->"),
        "AGENTS.md should have managed block after --update"
    );
    assert!(
        agents.contains("custom agents"),
        "AGENTS.md should preserve existing content"
    );
}
