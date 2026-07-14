#[path = "support/mod.rs"]
mod fixtures;

#[cfg(unix)]
use ito_test_support::run_rust_candidate;

#[test]
#[cfg(unix)]
fn apply_instruction_does_not_fetch_by_default_in_worktree_mode() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let _remote = setup_worktree_backed_apply_repo(repo.path(), home.path(), rust_path);

    let fake_git = tempfile::tempdir().expect("fake git");
    let fetch_log = fake_git.path().join("fetch.log");
    let real_git = real_git_path();
    write_fetch_logging_git(fake_git.path(), &real_git);

    let out = run_candidate_with_fetch_logging(
        rust_path,
        &[
            "agent",
            "instruction",
            "apply",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
        fake_git.path(),
        &fetch_log,
        &real_git,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("## Apply: 000-01_test-change"));
    assert!(
        out.stdout
            .contains("does **not** sync coordination state by default"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout
            .contains("These paths are relative to the execute-ready implementation checkout root"),
        "stdout={}",
        out.stdout
    );
    assert_fetch_log_empty(&fetch_log);
}

#[test]
#[cfg(unix)]
fn apply_instruction_sync_flag_fetches_coordination_branch_in_worktree_mode() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let _remote = setup_worktree_backed_apply_repo(repo.path(), home.path(), rust_path);

    let fake_git = tempfile::tempdir().expect("fake git");
    let fetch_log = fake_git.path().join("fetch.log");
    let real_git = real_git_path();
    write_fetch_logging_git(fake_git.path(), &real_git);

    let out = run_candidate_with_fetch_logging(
        rust_path,
        &[
            "agent",
            "instruction",
            "apply",
            "--change",
            "000-01_test-change",
            "--sync",
        ],
        repo.path(),
        home.path(),
        fake_git.path(),
        &fetch_log,
        &real_git,
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("## Apply: 000-01_test-change"));
    let log = std::fs::read_to_string(&fetch_log).unwrap_or_default();
    assert!(
        log.contains("fetch") && log.contains("ito/internal/changes"),
        "expected --sync to fetch coordination branch; log={log:?}"
    );
}

#[cfg(unix)]
fn setup_worktree_backed_apply_repo(
    repo: &std::path::Path,
    home: &std::path::Path,
    rust_path: &std::path::Path,
) -> tempfile::TempDir {
    fixtures::git_init_with_initial_commit(repo);
    let remote = fixtures::make_bare_remote();
    fixtures::add_origin(repo, remote.path());

    let push = std::process::Command::new("git")
        .args(["push", "origin", "HEAD:main"])
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git push should run");
    assert!(
        push.status.success(),
        "git push failed: {}",
        String::from_utf8_lossy(&push.stderr)
    );

    fixtures::write(
        repo.join(".ito/config.json"),
        "{\n  \"backend\": { \"project\": { \"org\": \"testorg\", \"repo\": \"testrepo\" } }\n}\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--update",
        ],
        repo,
        home,
    );
    assert_eq!(
        out.code, 0,
        "init should enable coordination worktree; stderr={} stdout={}",
        out.stderr, out.stdout
    );

    let config_path = repo.join(".ito/config.json");
    let mut config: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
    config["changes"]["proposal"]["integration_mode"] =
        serde_json::Value::String("direct_merge".to_string());
    assert_eq!(
        config["changes"]["coordination_branch"]["storage"], "worktree",
        "fixture must retain worktree-backed coordination"
    );
    fixtures::write(
        &config_path,
        &format!("{}\n", serde_json::to_string_pretty(&config).unwrap()),
    );

    // Proposal packages are authoritative Git contents under the main-first
    // workflow, not coordination-worktree links.
    let changes_path = repo.join(".ito/changes");
    if std::fs::symlink_metadata(&changes_path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        std::fs::remove_file(&changes_path).unwrap();
    }
    std::fs::create_dir_all(&changes_path).unwrap();

    fixtures::write(
        repo.join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for apply instruction sync tests. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );
    fixtures::write(
        repo.join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );
    fixtures::write(
        repo.join(".ito/changes/000-01_test-change/.ito.yaml"),
        "schema: spec-driven\n",
    );
    fixtures::write(
        repo.join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    fixtures::write(
        repo.join(".ito/changes/000-01_test-change/design.md"),
        "# Design\n\nKeep apply instruction tests on authoritative main history.\n",
    );
    fixtures::write(
        repo.join(".ito/changes/000-01_test-change/tasks.md"),
        "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Do a thing\n- **Dependencies**: None\n- **Updated At**: 2026-07-13\n- **Status**: [ ] pending\n",
    );
    fixtures::write(
        repo.join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha Delta\nThe system SHALL include alpha delta behavior in strict validation.\n\n#### Scenario: Delta ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    let add = std::process::Command::new("git")
        .args(["add", "-f", ".ito/config.json", ".ito/changes"])
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git add should run");
    assert!(
        add.status.success(),
        "git add failed: {}",
        String::from_utf8_lossy(&add.stderr)
    );
    let commit = std::process::Command::new("git")
        .args([
            "-c",
            "commit.gpgSign=false",
            "commit",
            "-m",
            "integrate reviewed proposal",
        ])
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git commit should run");
    assert!(
        commit.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
    let _ = std::fs::remove_file(repo.join(".git/ito-sync-state.json"));

    remote
}

#[cfg(unix)]
fn real_git_path() -> std::path::PathBuf {
    let Some(path) = std::env::var_os("PATH") else {
        panic!("PATH is not set");
    };
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join("git");
        if candidate.is_file() {
            return candidate;
        }
    }
    panic!("git executable not found on PATH");
}

#[cfg(unix)]
fn write_fetch_logging_git(fake_git_dir: &std::path::Path, real_git: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let script = fake_git_dir.join("git");
    fixtures::write(
        &script,
        "#!/bin/sh\nif [ \"$1\" = \"fetch\" ]; then\n  printf '%s\\n' \"$*\" >> \"$GIT_FETCH_LOG\"\nfi\nexec \"$REAL_GIT\" \"$@\"\n",
    );
    let mut permissions = std::fs::metadata(&script)
        .expect("fake git metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&script, permissions).expect("chmod fake git");

    assert!(
        real_git.is_file(),
        "real git missing: {}",
        real_git.display()
    );
}

#[cfg(unix)]
fn run_candidate_with_fetch_logging(
    program: &std::path::Path,
    args: &[&str],
    cwd: &std::path::Path,
    home: &std::path::Path,
    fake_git_dir: &std::path::Path,
    fetch_log: &std::path::Path,
    real_git: &std::path::Path,
) -> ito_test_support::CmdOutput {
    let path = std::env::var_os("PATH").unwrap_or_default();
    let path = std::env::join_paths(
        std::iter::once(fake_git_dir.to_path_buf()).chain(std::env::split_paths(&path)),
    )
    .expect("join PATH");

    let mut cmd = ito_test_support::rust_candidate_command(program);
    cmd.args(args)
        .current_dir(cwd)
        .env("CI", "1")
        .env("NO_COLOR", "1")
        .env("ITO_INTERACTIVE", "0")
        .env("TERM", "dumb")
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_DATA_HOME", home)
        .env("PATH", path)
        .env("REAL_GIT", real_git)
        .env("GIT_FETCH_LOG", fetch_log)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .env_remove("GIT_QUARANTINE_PATH")
        .env_remove("GIT_PREFIX");

    let output = cmd
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {:?}: {e}", cmd));
    ito_test_support::CmdOutput {
        code: output.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}

#[cfg(unix)]
fn assert_fetch_log_empty(fetch_log: &std::path::Path) {
    let log = std::fs::read_to_string(fetch_log).unwrap_or_default();
    assert!(
        log.trim().is_empty(),
        "default apply instruction generation must not fetch; log={log:?}"
    );
}
