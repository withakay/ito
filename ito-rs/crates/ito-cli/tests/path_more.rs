#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

/// Normalize a filesystem path string for test assertions.
///
/// This trims surrounding whitespace and removes a leading `/private` prefix if present,
/// returning the adjusted path as an owned `String`.
///
/// # Examples
///
/// ```
/// let p = normalize_path_for_assert(" /private/var/tmp/repo ");
/// assert_eq!(p, "/var/tmp/repo".to_string());
///
/// let p2 = normalize_path_for_assert("/home/user/project");
/// assert_eq!(p2, "/home/user/project".to_string());
/// ```
fn normalize_path_for_assert(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("/private").unwrap_or(s);
    s.to_string()
}

/// Verifies that `ito path` root subcommands resolve to the repository's absolute paths in an initialized repository.
///
/// This test asserts that `project-root`, `worktree-root`, and `ito-root` return the repository path (with `.ito` for `ito-root`), and that the JSON `--json` output contains the same absolute `path` value.
///
/// # Examples
///
/// ```no_run
/// // Prepared repository should make `ito path project-root` print the repo absolute path.
/// ```
#[test]
fn path_roots_are_absolute_in_initialized_repo() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["path", "project-root"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        normalize_path_for_assert(&out.stdout),
        normalize_path_for_assert(&repo.path().to_string_lossy())
    );

    let out = run_rust_candidate(
        rust_path,
        &["path", "worktree-root"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        normalize_path_for_assert(&out.stdout),
        normalize_path_for_assert(&repo.path().to_string_lossy())
    );

    let out = run_rust_candidate(rust_path, &["path", "ito-root"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        normalize_path_for_assert(&out.stdout),
        normalize_path_for_assert(&repo.path().join(".ito").to_string_lossy())
    );

    let out = run_rust_candidate(
        rust_path,
        &["path", "project-root", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("path json");
    let got = normalize_path_for_assert(v["path"].as_str().expect("path"));
    let want = normalize_path_for_assert(&repo.path().to_string_lossy());
    assert_eq!(got, want);
}

#[test]
fn path_worktrees_root_requires_worktrees_enabled() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["path", "worktrees-root"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Worktrees are not enabled"));
}

#[test]
fn path_worktrees_root_and_change_worktree_resolve_from_config() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();
    std::fs::write(
        repo.path().join(".ito/config.json"),
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "checkout_subdir",
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    )
    .unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["path", "worktrees-root"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        normalize_path_for_assert(&out.stdout),
        normalize_path_for_assert(&repo.path().join(".ito-worktrees").to_string_lossy())
    );

    let out = run_rust_candidate(
        rust_path,
        &["path", "worktree", "--change", "001-02_foo"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        normalize_path_for_assert(&out.stdout),
        normalize_path_for_assert(
            &repo
                .path()
                .join(".ito-worktrees/001-02_foo")
                .to_string_lossy()
        )
    );

    let out = run_rust_candidate(
        rust_path,
        &["path", "worktree", "--main"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        normalize_path_for_assert(&out.stdout),
        normalize_path_for_assert(&repo.path().to_string_lossy())
    );
}

#[test]
fn path_errors_in_bare_repo() {
    let bare = fixtures::make_bare_remote();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["path", "project-root"],
        bare.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Ito must be run from a git worktree"));
}

#[test]
fn path_roots_json_includes_worktree_fields_when_enabled() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();
    std::fs::write(
        repo.path().join(".ito/config.json"),
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "checkout_subdir",
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    )
    .unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["path", "roots", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("roots json");
    assert_eq!(v["enabled"], true);
    assert_eq!(v["strategy"], "checkout_subdir");

    let worktrees_root = normalize_path_for_assert(v["worktreesRoot"].as_str().expect("wt root"));
    assert_eq!(
        worktrees_root,
        normalize_path_for_assert(&repo.path().join(".ito-worktrees").to_string_lossy())
    );
    let main_root = normalize_path_for_assert(v["mainWorktreeRoot"].as_str().expect("main root"));
    assert_eq!(
        main_root,
        normalize_path_for_assert(&repo.path().to_string_lossy())
    );
}

#[test]
fn path_worktree_requires_a_selector_flag() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();
    std::fs::write(
        repo.path().join(".ito/config.json"),
        r#"{"worktrees": {"enabled": true, "strategy": "checkout_subdir", "layout": {"dir_name": "ito-worktrees"}}}"#,
    )
    .unwrap();

    let out = run_rust_candidate(rust_path, &["path", "worktree"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Missing selector"));
}

#[test]
fn path_missing_subcommand_errors() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    let out = run_rust_candidate(rust_path, &["path"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    let msg = format!("{}\n{}", out.stdout, out.stderr);
    assert!(
        msg.contains("Missing required subcommand")
            || msg.contains("Usage")
            || msg.contains("USAGE"),
        "stdout={} stderr={}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn path_roots_text_renders_worktree_fields_when_available() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();
    std::fs::write(
        repo.path().join(".ito/config.json"),
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "checkout_subdir",
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    )
    .unwrap();

    let out = run_rust_candidate(rust_path, &["path", "roots"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("project_root:"));
    assert!(out.stdout.contains("worktree_root:"));
    assert!(out.stdout.contains("ito_root:"));
    assert!(out.stdout.contains("worktrees_root:"));
    assert!(out.stdout.contains("main_worktree_root:"));
    assert!(out.stdout.contains("worktrees_enabled:"));
    assert!(out.stdout.contains("worktree_strategy:"));
}
