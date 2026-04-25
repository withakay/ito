//! Integration coverage for change `023-09_marker-aware-manifest-installs`.
//!
//! These tests prove that user-edited content sitting outside the
//! `<!-- ITO:START -->` / `<!-- ITO:END -->` managed block of an installed
//! harness skill or command survives `ito init --update`, while the managed
//! block itself is refreshed to match the current bundle.
//!
//! Companion to the project-template test
//! `update_preserves_user_guidance_and_user_prompt_files` in `update_smoke.rs`,
//! which covers the project-template installer path. This file covers the
//! harness manifest installer path that previously did wholesale overwrites.

#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

const USER_NOTE: &str = "<!-- user-appended after ito-end --> my project notes\n";

fn init_opencode(repo: &std::path::Path, home: &std::path::Path) {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let repo_path = repo.to_string_lossy();
    let argv = ["init", repo_path.as_ref(), "--tools", "opencode"];
    let out = run_rust_candidate(rust_path, &argv, repo, home);
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);
}

fn run_update(repo: &std::path::Path, home: &std::path::Path) {
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let repo_path = repo.to_string_lossy();
    let argv = [
        "init",
        repo_path.as_ref(),
        "--tools",
        "opencode",
        "--update",
    ];
    let out = run_rust_candidate(rust_path, &argv, repo, home);
    assert_eq!(out.code, 0, "init --update failed: {}", out.stderr);
}

fn append_after_end_marker(path: &std::path::Path, note: &str) {
    let existing = std::fs::read_to_string(path).expect("read file");
    let appended = format!("{existing}\n{note}");
    std::fs::write(path, appended).expect("write file");
}

#[test]
fn update_preserves_user_edits_after_end_marker_in_harness_skill() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    fixtures::reset_repo(repo.path(), base.path());

    init_opencode(repo.path(), home.path());

    let skill_path = repo.path().join(".opencode/skills/ito-feature/SKILL.md");
    assert!(
        skill_path.exists(),
        "ito-feature skill should be installed by init"
    );

    // Sanity: the installed skill carries a managed block.
    let pre = std::fs::read_to_string(&skill_path).expect("read skill");
    assert!(pre.contains("<!-- ITO:START -->"), "missing ITO:START");
    assert!(pre.contains("<!-- ITO:END -->"), "missing ITO:END");

    // User edits content AFTER the managed block.
    append_after_end_marker(&skill_path, USER_NOTE);

    // Run update. Marker-aware manifest installer should refresh the managed
    // block but leave the user note byte-for-byte intact.
    run_update(repo.path(), home.path());

    let after = std::fs::read_to_string(&skill_path).expect("read skill");
    assert!(
        after.contains(USER_NOTE.trim()),
        "user-appended note must survive update; got:\n{after}"
    );
    assert!(
        after.contains("<!--ITO:VERSION:"),
        "managed block must be stamped with the CLI version after update"
    );
}

#[test]
fn update_preserves_user_edits_after_end_marker_in_harness_command() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    fixtures::reset_repo(repo.path(), base.path());

    init_opencode(repo.path(), home.path());

    let cmd_path = repo.path().join(".opencode/commands/ito-loop.md");
    assert!(
        cmd_path.exists(),
        "ito-loop command should be installed by init"
    );

    let pre = std::fs::read_to_string(&cmd_path).expect("read command");
    assert!(pre.contains("<!-- ITO:START -->"));
    assert!(pre.contains("<!-- ITO:END -->"));

    append_after_end_marker(&cmd_path, USER_NOTE);

    run_update(repo.path(), home.path());

    let after = std::fs::read_to_string(&cmd_path).expect("read command");
    assert!(
        after.contains(USER_NOTE.trim()),
        "user-appended note must survive update on harness commands"
    );
}

#[test]
fn second_update_is_a_noop_for_harness_skills() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    fixtures::reset_repo(repo.path(), base.path());

    init_opencode(repo.path(), home.path());

    let skill_path = repo.path().join(".opencode/skills/ito-feature/SKILL.md");
    let cmd_path = repo.path().join(".opencode/commands/ito-loop.md");

    // First update establishes the canonical state.
    run_update(repo.path(), home.path());

    let skill_after_first = std::fs::read_to_string(&skill_path).expect("read skill");
    let cmd_after_first = std::fs::read_to_string(&cmd_path).expect("read command");

    // Second update against an unchanged tree must be byte-identical.
    run_update(repo.path(), home.path());

    let skill_after_second = std::fs::read_to_string(&skill_path).expect("read skill 2");
    let cmd_after_second = std::fs::read_to_string(&cmd_path).expect("read command 2");

    assert_eq!(
        skill_after_first, skill_after_second,
        "harness skill must be byte-identical across consecutive updates"
    );
    assert_eq!(
        cmd_after_first, cmd_after_second,
        "harness command must be byte-identical across consecutive updates"
    );
}

#[test]
fn update_refuses_to_overwrite_partial_marker_pair() {
    // If a user (or some other tool) has damaged the managed region by
    // leaving only one marker, the writer must refuse rather than silently
    // wholesale-overwrite. Mirrors `write_one`'s safety contract for
    // project templates.
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    fixtures::reset_repo(repo.path(), base.path());

    init_opencode(repo.path(), home.path());

    let skill_path = repo.path().join(".opencode/skills/ito-feature/SKILL.md");
    let pristine = std::fs::read_to_string(&skill_path).expect("read skill");

    // Strip the END marker but keep START → corrupt half-pair.
    let damaged = pristine.replace("<!-- ITO:END -->", "");
    std::fs::write(&skill_path, damaged).expect("write damaged skill");

    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let repo_path = repo.path().to_string_lossy();
    let argv = [
        "init",
        repo_path.as_ref(),
        "--tools",
        "opencode",
        "--update",
    ];
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_ne!(
        out.code, 0,
        "expected init --update to fail on partial marker pair"
    );
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("partial Ito marker pair") || combined.contains("Refusing to update"),
        "expected diagnostic about partial marker pair; got:\nstdout={}\nstderr={}",
        out.stdout,
        out.stderr,
    );
}

#[test]
fn update_still_refreshes_non_markdown_manifest_assets() {
    // Non-markdown manifest entries (helper scripts, adapter glue) are
    // intentionally outside the marker-scoped contract; they continue to be
    // refreshed wholesale by the manifest installer.
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    fixtures::reset_repo(repo.path(), base.path());

    init_opencode(repo.path(), home.path());

    let plugin_path = repo.path().join(".opencode/plugins/ito-skills.js");
    assert!(plugin_path.exists(), "opencode plugin must be installed");

    // Tamper with the plugin file (simulating a stale older copy).
    std::fs::write(&plugin_path, "// stale plugin\n").expect("tamper with plugin");

    run_update(repo.path(), home.path());

    let refreshed = std::fs::read_to_string(&plugin_path).expect("read plugin");
    assert!(
        !refreshed.contains("// stale plugin"),
        "non-markdown manifest assets should still be refreshed by update; got:\n{refreshed}"
    );
}
