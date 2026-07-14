#![cfg(feature = "coordination-branch")]

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

/// Verifies that compiling coordination support does not activate it on init.
#[test]
fn init_without_explicit_setup_stays_disabled_and_embedded() {
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
    assert_eq!(
        out.code, 0,
        "init should succeed without a remote; stderr={}",
        out.stderr
    );
    assert!(repo.path().join(".ito").is_dir());
    let config = std::fs::read_to_string(repo.path().join(".ito/config.json"))
        .expect("config.json should exist");
    let config: ito_config::types::ItoConfig =
        serde_json::from_str(&config).expect("valid Ito config JSON");
    assert!(!config.changes.coordination_branch.enabled.0);
    assert_eq!(
        config.changes.coordination_branch.storage.as_str(),
        "embedded"
    );
    assert!(
        std::fs::read_link(repo.path().join(".ito/changes")).is_err(),
        "default init must not wire a coordination symlink"
    );
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
            "--setup-coordination-branch",
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
    // Authoritative proposal state remains real Git content, while only the
    // compatibility coordination directories are linked.
    let changes_path = repo.path().join(".ito/changes");
    assert!(
        changes_path.is_dir() && std::fs::read_link(&changes_path).is_err(),
        ".ito/changes should remain a real directory after coordination setup"
    );
    let modules_path = repo.path().join(".ito/modules");
    assert!(
        std::fs::read_link(&modules_path).is_ok(),
        ".ito/modules should be a coordination symlink"
    );

    // The project config should record storage: "worktree".
    let config =
        std::fs::read_to_string(repo.path().join(".ito/config.json")).expect("config.json");
    assert!(
        config.contains("\"worktree\""),
        "config.json should contain worktree storage mode; got:\n{config}"
    );
    assert!(
        config.contains("\"enabled\": true"),
        "config.json should enable explicitly requested coordination; got:\n{config}"
    );
}

/// Verifies that `ito init --update` repairs missing coordination links in an
/// existing worktree-backed repo.
#[test]
#[cfg(unix)]
fn init_update_repairs_missing_coordination_symlink() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::git_init_with_initial_commit(repo.path());

    let remote = fixtures::make_bare_remote();
    fixtures::add_origin(repo.path(), remote.path());

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
            "--setup-coordination-branch",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "initial init --update failed: {}", out.stderr);

    let modules_link = repo.path().join(".ito/modules");
    let metadata = std::fs::symlink_metadata(&modules_link).expect("modules symlink metadata");
    assert!(
        metadata.file_type().is_symlink(),
        ".ito/modules should exist as a symlink after initial init --update"
    );
    std::fs::remove_file(&modules_link).expect("remove modules symlink");
    assert!(!modules_link.exists(), "modules symlink should be removed");

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
    assert_eq!(out.code, 0, "repair init --update failed: {}", out.stderr);

    assert!(
        std::fs::read_link(&modules_link).is_ok(),
        ".ito/modules should be restored as a coordination symlink"
    );
}
