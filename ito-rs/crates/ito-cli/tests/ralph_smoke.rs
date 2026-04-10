use std::path::Path;

use ito_test_support::pty::{run_pty_interactive, run_pty_interactive_with_env};
use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

fn make_base_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");

    // Minimal module.
    write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for Ralph tests. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Minimal spec.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Minimal change.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    td
}

fn write_complete_change(repo: &Path, change_id: &str) {
    write(
        repo.join(".ito/changes")
            .join(change_id)
            .join("proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );
    write(
        repo.join(".ito/changes").join(change_id).join("tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Done\n",
    );
    write(
        repo.join(".ito/changes")
            .join(change_id)
            .join("specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Delta\nThe system SHALL be testable.\n\n#### Scenario: Ok\n- **WHEN** run\n- **THEN** ok\n",
    );
}

fn reset_repo(dst: &Path, src: &Path) {
    ito_test_support::reset_dir(dst, src).unwrap();
}

#[cfg(unix)]
fn write_executable(path: impl AsRef<Path>, contents: &str) {
    use std::os::unix::fs::PermissionsExt;

    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
    let mut perms = std::fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).unwrap();
}

#[cfg(unix)]
fn make_fake_opencode(bin_dir: &Path, exit_code: i32) {
    let script = format!(
        "#!/bin/sh\n\
echo OPENCODE_ARGS:$*\n\
echo '<promise>COMPLETE</promise>'\n\
exit {exit_code}\n"
    );
    write_executable(bin_dir.join("opencode"), &script);
}

#[cfg(unix)]
fn make_fake_opencode_write_file(bin_dir: &Path, file_name: &str, exit_code: i32) {
    let script = format!(
        "#!/bin/sh\n\
printf 'generated\\n' > \"{}\"\n\
echo '<promise>COMPLETE</promise>'\n\
exit {}\n",
        file_name, exit_code
    );
    write_executable(bin_dir.join("opencode"), &script);
}

#[cfg(unix)]
fn make_fake_osascript(bin_dir: &Path, log_path: &Path) {
    let script = format!(
        "#!/bin/sh\n\
printf '%s\\n' \"$@\" >> \"{}\"\n\
exit 0\n",
        log_path.display()
    );
    write_executable(bin_dir.join("osascript"), &script);
}

#[cfg(unix)]
fn make_fake_gh(bin_dir: &Path, log_path: &Path) {
    let script = format!(
        "#!/bin/sh\n\
if [ \"$1\" = \"issue\" ] && [ \"$2\" = \"list\" ]; then\n\
  printf '[{{\"number\":7,\"title\":\"demo issue\"}}]'\n\
  exit 0\n\
fi\n\
if [ \"$1\" = \"issue\" ] && [ \"$2\" = \"close\" ]; then\n\
  printf '%s\\n' \"$@\" >> \"{}\"\n\
  exit 0\n\
fi\n\
if [ \"$1\" = \"issue\" ] && [ \"$2\" = \"edit\" ]; then\n\
  printf '%s\\n' \"$@\" >> \"{}\"\n\
  exit 0\n\
fi\n\
if [ \"$1\" = \"pr\" ] && [ \"$2\" = \"create\" ]; then\n\
  printf '%s\\n' \"$@\" >> \"{}\"\n\
  exit 0\n\
fi\n\
echo unexpected gh invocation: \"$@\" >&2\n\
exit 1\n",
        log_path.display(),
        log_path.display(),
        log_path.display()
    );
    write_executable(bin_dir.join("gh"), &script);
}

#[test]
#[cfg(unix)]
fn ralph_interactive_options_wizard_prompts_for_missing_values_and_applies_them() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    make_fake_opencode(bin.path(), 0);

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    // Prompts (in order): Harness (enter default), Model, Min, Max, no-commit (y), allow-all (y), exit-on-error (enter default)
    //
    // Note: dialoguer Confirm accepts a single keypress (y/n) without requiring Enter.
    // If we include a trailing newline after 'y', that newline can be consumed by the
    // next prompt and shift the input sequence.
    let input = "\nexample-model\n2\n2\nyy\n";

    let out = run_pty_interactive_with_env(
        rust_path,
        &["ralph", "--skip-validation", "do-work"],
        repo.path(),
        home.path(),
        input,
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stdout.contains("--- Ralph Options ---"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("(harness: opencode)"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("-m example-model"),
        "stdout={}",
        out.stdout
    );
    assert!(out.stdout.contains("--yolo"), "stdout={}", out.stdout);
    assert!(
        out.stdout.contains("=== Ralph Loop Iteration 2 ==="),
        "stdout={}",
        out.stdout
    );
}

#[test]
#[cfg(unix)]
fn ralph_interactive_options_wizard_exit_on_error_stops_on_nonzero_harness_exit() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    make_fake_opencode(bin.path(), 42);

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    // Prompts (in order): Harness (enter default), Model (blank), Min (default), Max (blank), no-commit (y), allow-all (default), exit-on-error (y)
    let input = "\n\n\n\ny\ny";

    let out = run_pty_interactive_with_env(
        rust_path,
        &["ralph", "--skip-validation", "do-work"],
        repo.path(),
        home.path(),
        input,
        &[("PATH", new_path.as_str())],
    );

    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stdout.contains("exited with code 42"),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn ralph_interactive_prompts_and_runs_selected_changes_sequentially() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Add a second change so interactive selection has multiple items.
    write_complete_change(repo.path(), "000-02_other");

    // MultiSelect: space toggles selection, arrows move, enter confirms.
    // Then the interactive options wizard prompts for any missing values.
    //
    // Select first + second change, then accept defaults for:
    // - model (blank)
    // - allow-all (false)
    // - exit-on-error (false)
    let input = " \x1b[B \n\n\n\n";
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
        input,
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(
        out.stdout.contains("=== Ralph Selection 1/2"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Starting Ralph for 000-01_test-change"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Starting Ralph for 000-02_other"),
        "stdout={}",
        out.stdout
    );

    assert!(
        repo.path()
            .join(".ito/.state/ralph/000-01_test-change/state.json")
            .exists()
    );
    assert!(
        repo.path()
            .join(".ito/.state/ralph/000-02_other/state.json")
            .exists()
    );
}

#[test]
fn ralph_interactive_status_prompts_for_exactly_one_change() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write_complete_change(repo.path(), "000-02_other");

    // Select default (first) change.
    let out = run_pty_interactive(
        rust_path,
        &["ralph", "--status"],
        repo.path(),
        home.path(),
        "\n",
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(
        out.stdout.contains("Ralph Status for 000-01_test-change"),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn ralph_no_interactive_without_target_returns_clear_error() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["ralph", "--no-interactive"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(out.stderr.contains("--change"), "stderr={}", out.stderr);
}

#[test]
fn ralph_stub_harness_writes_state_and_status_works() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Status before first run.
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Run one iteration using stub harness (default step returns <promise>COMPLETE</promise>).
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "stub",
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
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let state_path = repo
        .path()
        .join(".ito/.state/ralph/000-01_test-change/state.json");
    assert!(state_path.exists());

    // Status after run should mention iteration and history count.
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Iteration:"));
    assert!(out.stdout.contains("History entries:"));
    assert!(out.stdout.contains("Last outcome: validated-complete"));
    assert!(out.stdout.contains("Task progress:"));
}

#[test]
fn ralph_change_flag_supports_shorthand_resolution() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["ralph", "--change", "0-1", "--status", "--no-interactive"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Ralph Status for 000-01_test-change"));
}

#[test]
fn ralph_change_flag_supports_slug_query_resolution() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(
        repo.path()
            .join(".ito/changes/001-12_setup-wizard/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "setup wizard",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Ralph Status for 001-12_setup-wizard"));
}

#[test]
fn ralph_file_flag_requires_readable_file() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--file",
            "missing-prompt.txt",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr
            .contains("Failed to read prompt file missing-prompt.txt")
    );
}

#[test]
fn ralph_file_flag_allowed_without_change_or_module() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--file",
            "missing-prompt.txt",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr
            .contains("Failed to read prompt file missing-prompt.txt")
    );
}

#[test]
fn ralph_markdown_prd_source_marks_first_pending_task_complete() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(
        repo.path().join("PRD.md"),
        "- [ ] first task\n- [ ] second task\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--prd",
            "PRD.md",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    let prd = std::fs::read_to_string(repo.path().join("PRD.md")).unwrap();
    assert!(prd.contains("- [x] first task"), "prd={prd}");
}

#[test]
#[cfg(unix)]
fn ralph_sync_issue_updates_prd_back_to_github_issue() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let gh_log = repo.path().join("gh.log");

    reset_repo(repo.path(), base.path());
    write(repo.path().join("PRD.md"), "- [ ] first task\n");
    make_fake_gh(bin.path(), &gh_log);

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    let out = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--prd",
            "PRD.md",
            "--sync-issue",
            "42",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    let log = std::fs::read_to_string(&gh_log).unwrap();
    assert!(log.contains("issue"));
    assert!(log.contains("edit"));
    assert!(log.contains("42"));
    assert!(log.contains("--body-file"));
}

#[test]
fn ralph_yaml_source_marks_first_pending_task_complete() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(
        repo.path().join("tasks.yaml"),
        "tasks:\n  - title: first yaml task\n    completed: false\n  - title: second yaml task\n    completed: false\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--yaml",
            "tasks.yaml",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    let yaml = std::fs::read_to_string(repo.path().join("tasks.yaml")).unwrap();
    assert!(yaml.contains("completed: true"), "yaml={yaml}");
}

#[test]
#[cfg(unix)]
fn ralph_github_source_closes_issue_on_success() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let gh_log = repo.path().join("gh.log");

    reset_repo(repo.path(), base.path());
    make_fake_gh(bin.path(), &gh_log);

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    let out = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--github",
            "owner/repo",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    let log = std::fs::read_to_string(&gh_log).unwrap();
    assert!(log.contains("issue"));
    assert!(log.contains("close"));
    assert!(log.contains("7"));
}

#[test]
fn ralph_branch_per_task_creates_task_branch_for_prd_source() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(repo.path().join("PRD.md"), "- [ ] branch task\n");

    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["-c", "commit.gpgsign=false", "commit", "-m", "init"])
        .current_dir(repo.path())
        .status()
        .unwrap();

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--prd",
            "PRD.md",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--branch-per-task",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    let branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    let branch = String::from_utf8_lossy(&branch.stdout);
    assert!(branch.trim().starts_with("ralph/"), "branch={branch}");
}

#[test]
fn ralph_branch_per_task_requires_clean_worktree() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(repo.path().join("PRD.md"), "- [ ] branch task\n");

    std::process::Command::new("git")
        .args(["init", "-q", "-b", "main"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["-c", "commit.gpgsign=false", "commit", "-m", "init"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    write(repo.path().join("dirty.txt"), "dirty\n");

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--prd",
            "PRD.md",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--branch-per-task",
        ],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    assert!(
        out.stderr.contains("clean working tree"),
        "stderr={}",
        out.stderr
    );
}

#[test]
#[cfg(unix)]
fn ralph_create_pr_uses_base_branch_and_fake_gh() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let remote = tempfile::tempdir().expect("remote");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let gh_log = repo.path().join("gh.log");

    reset_repo(repo.path(), base.path());
    write(repo.path().join("PRD.md"), "- [ ] branch task\n");
    make_fake_gh(bin.path(), &gh_log);

    std::process::Command::new("git")
        .args(["init", "-q", "--bare"])
        .current_dir(remote.path())
        .status()
        .unwrap();

    std::process::Command::new("git")
        .args(["init", "-q", "-b", "main"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["remote", "add", "origin", remote.path().to_str().unwrap()])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["-c", "commit.gpgsign=false", "commit", "-m", "init"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .current_dir(repo.path())
        .status()
        .unwrap();

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    let out = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--prd",
            "PRD.md",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--branch-per-task",
            "--create-pr",
            "--draft-pr",
            "--base-branch",
            "main",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    let log = std::fs::read_to_string(&gh_log).unwrap();
    assert!(log.contains("pr"));
    assert!(log.contains("create"));
    assert!(log.contains("--base"));
    assert!(log.contains("main"));
}

#[test]
fn ralph_parallel_yaml_source_completes_grouped_tasks() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(
        repo.path().join("tasks.yaml"),
        "tasks:\n  - title: first grouped task\n    completed: false\n    parallel_group: 1\n  - title: second grouped task\n    completed: false\n    parallel_group: 1\n  - title: final grouped task\n    completed: false\n    parallel_group: 2\n",
    );

    std::process::Command::new("git")
        .args(["init", "-q", "-b", "main"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["-c", "commit.gpgsign=false", "commit", "-m", "init"])
        .current_dir(repo.path())
        .status()
        .unwrap();

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--yaml",
            "tasks.yaml",
            "--parallel",
            "--max-parallel",
            "2",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stdout={} stderr={}", out.stdout, out.stderr);
    let yaml = std::fs::read_to_string(repo.path().join("tasks.yaml")).unwrap();
    let complete_count = yaml.matches("completed: true").count();
    assert_eq!(complete_count, 3, "yaml={yaml}");
}

#[test]
#[cfg(unix)]
fn ralph_parallel_preserves_worker_code_changes() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(
        repo.path().join("tasks.yaml"),
        "tasks:\n  - title: write file\n    completed: false\n    parallel_group: 1\n",
    );
    std::process::Command::new("git")
        .args(["init", "-q", "-b", "main"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["-c", "commit.gpgsign=false", "commit", "-m", "init"])
        .current_dir(repo.path())
        .status()
        .unwrap();

    make_fake_opencode_write_file(bin.path(), "parallel-output.txt", 0);
    write_executable(
        bin.path().join("git"),
        "#!/bin/sh\nexec /usr/bin/git \"$@\"\n",
    );

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    let out = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--yaml",
            "tasks.yaml",
            "--parallel",
            "--max-parallel",
            "1",
            "--harness",
            "opencode",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    assert!(repo.path().join("parallel-output.txt").exists());
}

#[test]
#[cfg(unix)]
fn ralph_browser_flag_injects_agent_browser_guidance_for_opencode() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    make_fake_opencode(bin.path(), 0);
    write_executable(bin.path().join("agent-browser"), "#!/bin/sh\nexit 0\n");

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    let out = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "opencode",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--browser",
            "do",
            "work",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stdout.contains("agent-browser"),
        "stdout={}",
        out.stdout
    );
}

#[test]
#[cfg(unix)]
fn ralph_notify_emits_operator_notification_on_success() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let bin = tempfile::tempdir().expect("bin");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let notify_log = repo.path().join("notify.log");

    reset_repo(repo.path(), base.path());
    make_fake_osascript(bin.path(), &notify_log);

    let old_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{old_path}", bin.path().display());

    let out = run_pty_interactive_with_env(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--notify",
            "do",
            "work",
        ],
        repo.path(),
        home.path(),
        "",
        &[("PATH", new_path.as_str())],
    );

    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    let log = std::fs::read_to_string(&notify_log).unwrap();
    assert!(log.contains("display notification"), "log={log}");
}

#[test]
fn ralph_continue_ready_exits_successfully_when_all_changes_complete() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Ensure the base change is "complete" for work-status purposes.
    write(
        repo.path()
            .join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Delta\nThe system SHALL be testable.\n\n#### Scenario: Ok\n- **WHEN** run\n- **THEN** ok\n",
    );
    // Add a second complete change.
    write_complete_change(repo.path(), "000-02_other");

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--continue-ready",
            "--harness",
            "stub",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("All changes are complete."));
}

#[test]
fn ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Draft change blocks completion: tasks exist, but proposal/specs are missing.
    write(
        repo.path().join(".ito/changes/000-03_draft/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Todo\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--continue-ready",
            "--harness",
            "stub",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr.contains("no eligible changes"),
        "stderr={}",
        out.stderr
    );
    assert!(out.stderr.contains("000-03_draft"), "stderr={}", out.stderr);
}

/// Verifies Ralph can run using `--file` for an unscoped prompt (no change or module).
///
/// Confirms the command exits successfully, prints a message indicating an unscoped run,
/// and writes state to `.ito/.state/ralph/unscoped/state.json`.
///
/// # Examples
///
/// ```
/// // This test sets up a temporary repo and prompt file, then runs the `ito ralph` flow
/// // with the stub harness and `--file prompt.md`. It asserts success and the presence
/// // of the unscoped state file.
/// fn example_unscoped_file_run() {
///     let base = make_base_repo();
///     let repo = tempfile::tempdir().unwrap();
///     let home = tempfile::tempdir().unwrap();
///     let rust_path = assert_cmd::cargo::cargo_bin!("ito");
///
///     reset_repo(repo.path(), base.path());
///     write(repo.path().join("prompt.md"), "do work\n");
///
///     let out = run_rust_candidate(
///         rust_path,
///         &[
///             "ralph",
///             "--harness",
///             "stub",
///             "--no-commit",
///             "--no-interactive",
///             "--skip-validation",
///             "--min-iterations",
///             "1",
///             "--max-iterations",
///             "1",
///             "--file",
///             "prompt.md",
///         ],
///         repo.path(),
///         home.path(),
///     );
///     assert_eq!(out.code, 0);
///     assert!(out.stdout.contains("Starting Ralph for unscoped"));
///
///     let state_path = repo.path().join(".ito/.state/ralph/unscoped/state.json");
///     assert!(state_path.exists());
/// }
/// ```
#[test]
fn ralph_file_flag_runs_without_change_or_module() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(repo.path().join("prompt.md"), "do work\n");

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--file",
            "prompt.md",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Starting Ralph for unscoped"));

    let state_path = repo.path().join(".ito/.state/ralph/unscoped/state.json");
    assert!(state_path.exists());
}

/// Verifies the status flow accepts multiple harness names without failing.
///
/// Runs the `ito ralph --status` command for each harness in ["claude", "codex", "github-copilot", "copilot"]
/// against a prepared test repository and asserts each invocation exits with code `0`.
///
/// # Examples
///
/// ```
/// // The test sets up temporary repositories and invokes the `ito` binary for each harness:
/// ralph_accepts_new_harness_names_for_status_flow();
/// ```
#[test]
fn ralph_accepts_new_harness_names_for_status_flow() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    for harness in ["claude", "codex", "github-copilot", "copilot"] {
        let out = run_rust_candidate(
            rust_path,
            &[
                "ralph",
                "--change",
                "000-01_test-change",
                "--harness",
                harness,
                "--status",
                "--no-interactive",
            ],
            repo.path(),
            home.path(),
        );
        assert_eq!(out.code, 0, "harness={} stderr={}", harness, out.stderr);
    }
}

#[test]
fn ralph_unknown_harness_returns_clear_error() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "does-not-exist",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr.contains("invalid value")
            && out.stderr.contains("does-not-exist")
            && out.stderr.contains("--harness"),
        "stderr={}",
        out.stderr
    );
}
