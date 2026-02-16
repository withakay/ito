#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

fn normalize_path_for_assert(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("/private").unwrap_or(s);
    s.to_string()
}

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
