#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

// PTY-based interactive tests are skipped on Windows due to platform differences
// in terminal handling that can cause hangs.
#[cfg(unix)]
use ito_test_support::pty::run_pty_interactive;

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

// PTY-based interactive tests are skipped on Windows due to platform differences
// in terminal handling that cause hangs. The underlying init logic is cross-platform.
#[test]
#[cfg(unix)]
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
    let out = run_pty_interactive(
        rust_path,
        &["init", repo.path().to_string_lossy().as_ref()],
        repo.path(),
        home.path(),
        "\n\n\n",
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
