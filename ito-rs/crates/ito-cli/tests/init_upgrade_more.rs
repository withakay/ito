//! Integration tests for `ito init --upgrade` (marker-scoped refresh).
//!
//! These tests verify the fail-safe upgrade behavior: when `--upgrade` is
//! passed, files without Ito-managed markers are preserved unchanged (with a
//! warning), while files that do have markers get their managed block refreshed.

#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

/// Verifies that `ito init --upgrade` preserves files that have no Ito markers
/// and emits a warning rather than erroring out.
#[test]
fn init_upgrade_skips_and_warns_when_markers_missing() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // Write an AGENTS.md without markers.
    let original_content = "# Custom AGENTS\n\nThis file has no Ito markers.\n";
    fixtures::write(repo.path().join("AGENTS.md"), original_content);

    // init --upgrade should succeed (fail-safe, not error).
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
    assert_eq!(
        out.code, 0,
        "init --upgrade should succeed even when markers are missing: {}",
        out.stderr
    );

    // File should be preserved unchanged.
    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    assert_eq!(
        agents, original_content,
        "file should be preserved when markers are missing in upgrade mode"
    );

    // Warning should appear in stderr.
    assert!(
        out.stderr.contains("warning") || out.stderr.contains("skipping upgrade"),
        "upgrade should emit a warning when markers are missing; stderr was: {}",
        out.stderr
    );
}

/// Verifies that `ito init --upgrade` is accepted as a valid CLI flag and succeeds.
#[test]
fn init_upgrade_flag_is_accepted() {
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
            "--upgrade",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(
        out.code, 0,
        "init --upgrade should be accepted; stderr={}",
        out.stderr
    );

    // Core files should be created.
    assert!(repo.path().join(".ito").is_dir());
    assert!(repo.path().join("AGENTS.md").exists());
}

/// Verifies that `ito init --update` preserves user-owned files.
///
/// This is part of the non-destructive update semantics that both `--update` and
/// `--upgrade` share for user-owned content.
#[test]
fn init_update_preserves_user_owned_files() {
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

    // Modify user-owned files.
    fixtures::write(
        repo.path().join(".ito/project.md"),
        "# My Custom Project\n\nThis is user-authored content.\n",
    );

    // Run init --update (compatibility alias).
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

    let project_md = std::fs::read_to_string(repo.path().join(".ito/project.md")).unwrap();
    assert!(
        project_md.contains("My Custom Project"),
        "project.md should be preserved by --update"
    );
}

/// Verifies that `ito init --upgrade` replaces the managed block content while
/// preserving user-authored content outside the Ito markers.
///
/// Uses a sentinel string injected inside the managed block to prove the block
/// was actually refreshed (sentinel gone), not merely left intact.
#[test]
fn init_upgrade_refreshes_marker_managed_block_and_preserves_user_content() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    // First, do a normal init to install AGENTS.md with markers.
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

    // Read the installed AGENTS.md and splice a sentinel into the managed block,
    // then append user content after the closing marker.
    let agents_content = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();
    let agents_with_sentinel = agents_content.replace(
        "<!-- ITO:START -->",
        "<!-- ITO:START -->\nSENTINEL_MANAGED_BLOCK",
    );
    let updated_agents =
        format!("{agents_with_sentinel}\n## My Team Section\nUser authored content here.\n");
    fixtures::write(repo.path().join("AGENTS.md"), &updated_agents);

    // Run init --upgrade — should replace the managed block (removing sentinel)
    // and preserve user content outside the markers.
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

    let agents = std::fs::read_to_string(repo.path().join("AGENTS.md")).unwrap();

    assert!(
        !agents.contains("SENTINEL_MANAGED_BLOCK"),
        "managed block should be refreshed (sentinel removed) after --upgrade"
    );
    assert!(
        agents.contains("My Team Section"),
        "user content outside markers should be preserved after --upgrade"
    );
    assert!(
        agents.contains("<!-- ITO:START -->"),
        "managed markers should be present after --upgrade"
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
