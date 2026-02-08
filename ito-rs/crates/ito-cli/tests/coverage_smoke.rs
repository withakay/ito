use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

#[test]
fn serve_errors_when_no_ito_dir_exists() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(rust_path, &["serve"], repo.path(), home.path());
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr.contains("No .ito directory"),
        "stderr={}",
        out.stderr
    );
}

#[test]
fn audit_validate_and_log_work_with_empty_event_log() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    write(repo.path().join(".ito/.state/audit/events.jsonl"), "");
    write(
        repo.path().join(".ito/changes/000-01_drift/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Todo\n",
    );

    let out = run_rust_candidate(rust_path, &["audit", "validate"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("Audit Validate:"),
        "stdout={}",
        out.stdout
    );

    let out = run_rust_candidate(rust_path, &["audit", "log"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("No audit events found."),
        "stdout={} stderr={}",
        out.stdout,
        out.stderr
    );

    let out = run_rust_candidate(
        rust_path,
        &["audit", "validate", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("\"valid\""));

    let out = run_rust_candidate(
        rust_path,
        &["audit", "log", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("["));

    let out = run_rust_candidate(rust_path, &["audit", "reconcile"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Reconcile:"), "stdout={}", out.stdout);
}

#[test]
fn completions_command_runs_for_all_shells() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    for shell in ["bash", "zsh", "fish", "powershell"] {
        let out = run_rust_candidate(rust_path, &["completions", shell], repo.path(), home.path());
        assert_eq!(out.code, 0, "shell={shell} stderr={}", out.stderr);
        assert!(!out.stdout.trim().is_empty());
    }
}
