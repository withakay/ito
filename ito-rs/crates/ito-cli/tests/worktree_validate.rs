#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

fn normalize_path_for_assert(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("/private").unwrap_or(s);
    s.to_string()
}

fn run_git(repo: &std::path::Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git {} failed\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_repo_with_worktrees_enabled(repo: &std::path::Path) {
    fixtures::write(
        repo.join(".ito/config.json"),
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "checkout_subdir",
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    );
    fixtures::git_init_with_initial_commit(repo);
}

fn write_worktree_config(worktree_path: &std::path::Path) {
    fixtures::write(
        worktree_path.join(".ito/config.json"),
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "checkout_subdir",
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    );
}

#[test]
fn worktree_validate_fails_on_main_checkout_and_emits_json() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    init_repo_with_worktrees_enabled(repo.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "worktree",
            "validate",
            "--change",
            "012-07_guard-opencode-worktree-path",
            "--json",
        ],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0, "main checkout should fail validation");
    let json: serde_json::Value = serde_json::from_str(&out.stdout).expect("json output");
    assert_eq!(json["status"], "main_checkout");
    assert_eq!(json["changeId"], "012-07_guard-opencode-worktree-path");
    assert!(
        json["message"]
            .as_str()
            .unwrap_or_default()
            .contains("main/control worktree")
    );
    let expected = normalize_path_for_assert(json["expectedPath"].as_str().expect("expected path"));
    assert_eq!(
        expected,
        normalize_path_for_assert(
            &repo
                .path()
                .join(".ito-worktrees/012-07_guard-opencode-worktree-path")
                .to_string_lossy()
        )
    );
}

#[test]
fn worktree_validate_accepts_same_change_suffix_worktree() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    init_repo_with_worktrees_enabled(repo.path());

    let worktree_path = repo
        .path()
        .join(".ito-worktrees/012-07_guard-opencode-worktree-path-review");
    run_git(
        repo.path(),
        &[
            "worktree",
            "add",
            "-b",
            "012-07_guard-opencode-worktree-path-review",
            worktree_path.to_string_lossy().as_ref(),
        ],
    );
    write_worktree_config(&worktree_path);

    let out = run_rust_candidate(
        rust_path,
        &[
            "worktree",
            "validate",
            "--change",
            "012-07_guard-opencode-worktree-path",
            "--json",
        ],
        &worktree_path,
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let json: serde_json::Value = serde_json::from_str(&out.stdout).expect("json output");
    assert_eq!(json["status"], "ok");
}

#[test]
fn worktree_validate_reports_mismatch_without_failing() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    init_repo_with_worktrees_enabled(repo.path());

    let worktree_path = repo.path().join(".ito-worktrees/other-change-review");
    run_git(
        repo.path(),
        &[
            "worktree",
            "add",
            "-b",
            "other-change-review",
            worktree_path.to_string_lossy().as_ref(),
        ],
    );
    write_worktree_config(&worktree_path);

    let out = run_rust_candidate(
        rust_path,
        &[
            "worktree",
            "validate",
            "--change",
            "012-07_guard-opencode-worktree-path",
            "--json",
        ],
        &worktree_path,
        home.path(),
    );

    assert_eq!(out.code, 0, "mismatch should be advisory");
    let json: serde_json::Value = serde_json::from_str(&out.stdout).expect("json output");
    assert_eq!(json["status"], "mismatch");
    assert!(
        json["message"]
            .as_str()
            .unwrap_or_default()
            .contains("full change ID")
    );
}

#[test]
fn worktree_validate_succeeds_when_disabled() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/config.json"),
        r#"{ "worktrees": { "enabled": false } }"#,
    );
    fixtures::git_init_with_initial_commit(repo.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "worktree",
            "validate",
            "--change",
            "012-07_guard-opencode-worktree-path",
            "--json",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let json: serde_json::Value = serde_json::from_str(&out.stdout).expect("json output");
    assert_eq!(json["status"], "disabled");
}
