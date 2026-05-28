#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn init_upgrade_reports_legacy_cleanup_candidates() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let repo_path = repo.path().to_string_lossy();
    let init = run_rust_candidate(
        rust_path,
        &["init", repo_path.as_ref(), "--tools", "codex"],
        repo.path(),
        home.path(),
    );
    assert_eq!(init.code, 0, "initial init failed: {}", init.stderr);

    fixtures::write(
        repo.path()
            .join(".codex/skills/ito-writing-skills/SKILL.md"),
        "legacy skill\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["init", repo_path.as_ref(), "--tools", "codex", "--upgrade"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "upgrade failed: {}", out.stderr);
    assert!(out.stderr.contains("legacy Ito-managed cleanup candidates"));
    assert!(
        out.stderr
            .contains(".codex/skills/ito-writing-skills/SKILL.md")
    );
    assert!(
        repo.path()
            .join(".codex/skills/ito-writing-skills/SKILL.md")
            .exists(),
        "plain upgrade should not remove legacy file"
    );
}

#[test]
fn init_upgrade_cleanup_force_removes_legacy_candidates() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let repo_path = repo.path().to_string_lossy();
    let init = run_rust_candidate(
        rust_path,
        &["init", repo_path.as_ref(), "--tools", "codex"],
        repo.path(),
        home.path(),
    );
    assert_eq!(init.code, 0, "initial init failed: {}", init.stderr);

    let legacy = repo
        .path()
        .join(".codex/skills/ito-writing-skills/SKILL.md");
    fixtures::write(&legacy, "legacy skill\n");

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo_path.as_ref(),
            "--tools",
            "codex",
            "--upgrade",
            "--cleanup",
            "--force",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "cleanup failed: {}", out.stderr);
    assert!(!legacy.exists(), "legacy file should be removed");
    assert!(out.stderr.contains("Removed 1 legacy Ito-managed"));
}
