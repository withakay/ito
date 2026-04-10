#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

/// Verifies that `--no-coordination-worktree` skips worktree creation and writes
/// `storage: "embedded"` to the project config.
#[test]
fn init_no_coordination_worktree_writes_embedded_storage() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::git_init_with_initial_commit(repo.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--no-coordination-worktree",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={} stdout={}", out.stderr, out.stdout);

    // Parse config JSON and assert storage == "embedded".
    let config_str =
        std::fs::read_to_string(repo.path().join(".ito/config.json")).expect("config.json");
    let config: serde_json::Value =
        serde_json::from_str(&config_str).expect("config.json should be valid JSON");
    let storage = config["changes"]["coordination_branch"]["storage"].as_str();
    assert_eq!(
        storage,
        Some("embedded"),
        "config.json should contain storage: \"embedded\"; got:\n{config_str}"
    );
}

/// Verifies that `ito init --upgrade` does NOT touch the coordination storage
/// mode or create a coordination worktree.
#[test]
fn init_upgrade_does_not_touch_coordination_storage() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::git_init_with_initial_commit(repo.path());

    // First, do a normal init to create the config.
    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--no-coordination-worktree",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "initial init failed: {}", out.stderr);

    // Confirm embedded was written.
    let config_before =
        std::fs::read_to_string(repo.path().join(".ito/config.json")).expect("config.json");
    let config_before: serde_json::Value =
        serde_json::from_str(&config_before).expect("config.json should be valid JSON");
    assert_eq!(
        config_before["changes"]["coordination_branch"]["storage"].as_str(),
        Some("embedded"),
        "expected embedded storage after --no-coordination-worktree"
    );

    // Now run --upgrade — it must NOT change the storage field.
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

    // Storage field must be unchanged and still set to embedded.
    let config_after =
        std::fs::read_to_string(repo.path().join(".ito/config.json")).expect("config.json");
    let config_after_json: serde_json::Value =
        serde_json::from_str(&config_after).expect("config.json should be valid JSON");
    assert_eq!(
        config_after_json["changes"]["coordination_branch"]["storage"].as_str(),
        Some("embedded"),
        "--upgrade must preserve embedded coordination storage; got:\n{config_after}"
    );
}

/// Verifies that a fresh `ito init` without a git remote gracefully falls back
/// to embedded mode (warns but does not fail).
#[test]
fn init_without_git_remote_falls_back_gracefully() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    // Initialize git but do NOT add a remote — org/repo resolution will fail.
    fixtures::git_init_with_initial_commit(repo.path());

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
    // Init must succeed even without a remote.
    assert_eq!(
        out.code, 0,
        "init should succeed without a remote; stderr={}",
        out.stderr
    );
    // A warning should be emitted to stderr.
    assert!(
        out.stderr.to_lowercase().contains("warning")
            || out.stderr.to_lowercase().contains("skipping"),
        "expected a warning about missing remote; stderr={}",
        out.stderr
    );
    // Core .ito/ structure must still be created.
    assert!(repo.path().join(".ito").is_dir());
    assert!(repo.path().join(".ito/config.json").exists());
}

/// Verifies that a fresh `ito init` with a git remote and a reachable origin
/// creates the coordination worktree and wires symlinks.
#[test]
#[cfg(unix)]
fn init_with_git_remote_creates_coordination_worktree() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::git_init_with_initial_commit(repo.path());

    // Add a fake origin remote with a recognisable org/repo URL so that
    // resolve_org_repo_from_config_or_remote can parse it.  We use a local
    // bare repo as the remote so git operations succeed without network access.
    let remote = fixtures::make_bare_remote();
    fixtures::add_origin(repo.path(), remote.path());

    // Push to origin so the remote is reachable.
    let push = std::process::Command::new("git")
        .args(["push", "origin", "HEAD:main"])
        .current_dir(repo.path())
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git push should run");
    assert!(
        push.status.success(),
        "git push failed: {}",
        String::from_utf8_lossy(&push.stderr)
    );

    // Write org/repo into the project config so coordination_worktree_path
    // can resolve a deterministic path without needing a real GitHub URL.
    fixtures::write(
        repo.path().join(".ito/config.json"),
        "{\n  \"backend\": { \"project\": { \"org\": \"testorg\", \"repo\": \"testrepo\" } }\n}\n",
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
    assert_eq!(
        out.code, 0,
        "init should succeed with a remote; stderr={} stdout={}",
        out.stderr, out.stdout
    );

    // The coordination worktree should have been created under
    // ~/.local/share/ito/testorg/testrepo (or XDG_DATA_HOME equivalent).
    // We verify indirectly: .ito/changes should be a symlink.
    let changes_path = repo.path().join(".ito/changes");
    assert!(
        std::fs::read_link(&changes_path).is_ok(),
        ".ito/changes should be a symlink after coordination worktree setup"
    );

    // The project config should record storage: "worktree".
    let config =
        std::fs::read_to_string(repo.path().join(".ito/config.json")).expect("config.json");
    assert!(
        config.contains("\"worktree\""),
        "config.json should contain worktree storage mode; got:\n{config}"
    );
}
