use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

fn make_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");

    // Minimal module.
    write(
        td.path().join(".ito/modules/024_test/module.md"),
        "# Backend\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Scope\n- *\n\n## Changes\n- [ ] 024-01_first-change\n- [ ] 024-02_second-change\n",
    );

    // Three changes across two modules.
    write(
        td.path()
            .join(".ito/changes/024-01_first-change/proposal.md"),
        "# Proposal\n\nNeedle: alpha\n",
    );
    write(
        td.path()
            .join(".ito/changes/024-02_second-change/proposal.md"),
        "# Proposal\n\nNeedle: beta\n",
    );
    write(
        td.path()
            .join(".ito/changes/025-01_third-change/proposal.md"),
        "# Proposal\n\nNeedle: gamma\n",
    );

    td
}

#[test]
fn grep_change_scope_prints_matches_with_locations() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["grep", "024-01_first-change", "Needle:"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr: {}", out.stderr);
    assert!(out.stdout.contains("proposal.md"), "stdout: {}", out.stdout);
    assert!(
        out.stdout.contains("Needle: alpha"),
        "stdout: {}",
        out.stdout
    );
    assert!(
        !out.stdout.contains("Needle: beta"),
        "stdout: {}",
        out.stdout
    );
}

#[test]
fn grep_module_scope_searches_all_changes_in_module() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["grep", "--module", "024", "Needle:"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr: {}", out.stderr);
    assert!(
        out.stdout.contains("Needle: alpha"),
        "stdout: {}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Needle: beta"),
        "stdout: {}",
        out.stdout
    );
    assert!(
        !out.stdout.contains("Needle: gamma"),
        "stdout: {}",
        out.stdout
    );
}

#[test]
fn grep_all_scope_searches_all_changes() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["grep", "--all", "Needle:"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr: {}", out.stderr);
    assert!(
        out.stdout.contains("Needle: alpha"),
        "stdout: {}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Needle: beta"),
        "stdout: {}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Needle: gamma"),
        "stdout: {}",
        out.stdout
    );
}

#[test]
fn grep_limit_caps_output_and_prints_warning() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["grep", "--all", "--limit", "1", "Needle:"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr: {}", out.stderr);

    let lines: Vec<&str> = out
        .stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    assert_eq!(lines.len(), 1, "stdout: {}", out.stdout);

    assert!(
        out.stderr.contains("output limited to 1"),
        "stderr: {}",
        out.stderr
    );
}

#[test]
fn grep_change_scope_rejects_too_many_positional_args() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["grep", "024-01_first-change", "Needle:", "extra"],
        repo.path(),
        home.path(),
    );

    assert_ne!(out.code, 0);
    assert!(
        out.stderr.contains("expected: ito grep"),
        "stderr: {}",
        out.stderr
    );
}
