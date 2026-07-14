use super::*;

fn write_unintegrated_change(repo: &Path, change_id: &str) -> std::path::PathBuf {
    let change = repo.join(".ito/changes").join(change_id);
    for (path, contents) in [
        (".ito.yaml", "schema: spec-driven\n"),
        ("proposal.md", "# Proposal\n\nNot integrated into main.\n"),
        ("design.md", "# Design\n\nReject before dispatch.\n"),
        (
            "tasks.md",
            "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Work\n- **Dependencies**: None\n- **Updated At**: 2026-07-13\n- **Status**: [ ] pending\n",
        ),
        (
            "specs/ralph-readiness/spec.md",
            "## ADDED Requirements\n\n### Requirement: Ralph readiness\nIto SHALL gate Ralph before dispatch.\n\n#### Scenario: Local-only proposal\n- **WHEN** Ralph starts\n- **THEN** no state is written\n",
        ),
    ] {
        write(change.join(path), contents);
    }
    change
}

fn git_stdout(repo: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git command");
    assert!(output.status.success());
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn git_status_without_session(repo: &Path) -> String {
    git_stdout(repo, &["status", "--porcelain"])
        .lines()
        .filter(|line| !line.ends_with(".ito/session.json"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn ralph_interactive_runs_selected_changes_in_verified_worktrees() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    reset_repo(repo.path(), base.path());
    git(repo.path(), &["switch", "main"]);
    write_complete_change(repo.path(), "000-02_other");
    git(repo.path(), &["add", "-A"]);
    git(
        repo.path(),
        &[
            "commit",
            "--no-gpg-sign",
            "--no-verify",
            "-m",
            "integrate second proposal",
        ],
    );

    let worktrees = tempfile::tempdir().expect("Ralph worktrees");
    let first = worktrees.path().join("000-01_test-change");
    let second = worktrees.path().join("000-02_other");
    git(
        repo.path(),
        &[
            "worktree",
            "add",
            first.to_str().unwrap(),
            "000-01_test-change",
        ],
    );
    git(
        repo.path(),
        &[
            "worktree",
            "add",
            "-b",
            "000-02_other",
            second.to_str().unwrap(),
            "main",
        ],
    );

    let out = run_pty_interactive(
        rust_path,
        &[
            "ralph",
            "--harness",
            "stub",
            "--no-commit",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
        " \x1b[B \n\n\n\n",
    );
    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(out.stdout.contains("Starting Ralph for 000-01_test-change"));
    assert!(out.stdout.contains("Starting Ralph for 000-02_other"));
    assert!(
        first
            .join(".ito/.state/ralph/000-01_test-change/state.json")
            .exists()
    );
    assert!(
        second
            .join(".ito/.state/ralph/000-02_other/state.json")
            .exists()
    );
}

#[test]
#[cfg(unix)]
fn ralph_and_loop_reject_before_harness_or_state_mutation() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "000-02_local-only";
    reset_repo(repo.path(), base.path());
    make_fake_opencode_write_file(bin.path(), "ralph-launched.txt", 0);
    let change = write_unintegrated_change(repo.path(), change_id);
    let tasks_before = std::fs::read(change.join("tasks.md")).unwrap();
    let status_before = git_status_without_session(repo.path());
    let head_before = git_stdout(repo.path(), &["rev-parse", "HEAD"]);
    let new_path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );

    for command in ["ralph", "loop"] {
        let output = run_pty_interactive_with_env(
            rust_path,
            &[
                command,
                "--change",
                change_id,
                "--harness",
                "opencode",
                "--no-commit",
                "--no-interactive",
                "--min-iterations",
                "1",
                "--max-iterations",
                "1",
                "do",
                "work",
            ],
            repo.path(),
            home.path(),
            "",
            &[("PATH", new_path.as_str())],
        );
        assert_ne!(output.code, 0, "stdout={}", output.stdout);
        assert!(output.stderr.contains("not ready") || output.stdout.contains("not ready"));
        assert!(!repo.path().join("ralph-launched.txt").exists());
        assert!(
            !repo
                .path()
                .join(".ito/.state/ralph")
                .join(change_id)
                .exists()
        );
        assert_eq!(
            std::fs::read(change.join("tasks.md")).unwrap(),
            tasks_before
        );
        assert_eq!(git_status_without_session(repo.path()), status_before);
        assert_eq!(git_stdout(repo.path(), &["rev-parse", "HEAD"]), head_before);
    }
}

#[test]
#[cfg(unix)]
fn ralph_continue_ready_gates_dynamic_dispatch() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "000-02_dynamic-local-only";
    reset_repo(repo.path(), base.path());
    make_fake_opencode_write_file(bin.path(), "dynamic-ralph-launched.txt", 0);
    write_unintegrated_change(repo.path(), change_id);
    let new_path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let output = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--continue-ready",
            "--harness",
            "opencode",
            "--no-commit",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );
    assert_ne!(output.code, 0, "stdout={}", output.stdout);
    assert!(
        output.stderr.contains("readiness failed") || output.stdout.contains("readiness failed")
    );
    assert!(!repo.path().join("dynamic-ralph-launched.txt").exists());
    assert!(
        !repo
            .path()
            .join(".ito/.state/ralph")
            .join(change_id)
            .exists()
    );
}
