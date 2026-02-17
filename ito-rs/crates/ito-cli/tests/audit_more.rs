#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn audit_log_stats_and_validate_json_outputs_are_well_formed() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito/changes/test-change")).unwrap();

    // Generate some audit events by exercising task commands.
    let out = run_rust_candidate(
        rust_path,
        &["tasks", "init", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "start", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "complete", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["audit", "log", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("audit log json");
    let arr = v.as_array().expect("audit log array");
    assert!(!arr.is_empty(), "expected at least one audit event");

    let out = run_rust_candidate(
        rust_path,
        &["audit", "stats", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("audit stats json");
    assert!(v.get("total_events").and_then(|n| n.as_u64()).unwrap_or(0) > 0);
    assert!(v.get("by_entity").is_some());
    assert!(v.get("by_op").is_some());

    let out = run_rust_candidate(
        rust_path,
        &["audit", "validate", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("audit validate json");
    assert!(v.get("event_count").is_some());
    assert!(v.get("valid").is_some());
}

#[test]
fn audit_subcommands_cover_text_output_limit_reconcile_and_stream() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito/changes/test-change")).unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "init", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let out = run_rust_candidate(
        rust_path,
        &["tasks", "start", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Text log output.
    let out = run_rust_candidate(rust_path, &["audit", "log"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("events"));

    // Limit.
    let out = run_rust_candidate(
        rust_path,
        &["audit", "log", "--json", "--limit", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("audit log json");
    assert!(v.as_array().expect("array").len() <= 1);

    // Reconcile (json + text).
    let out = run_rust_candidate(
        rust_path,
        &["audit", "reconcile", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("reconcile json");
    assert!(v.get("drift_count").is_some());

    let out = run_rust_candidate(rust_path, &["audit", "reconcile"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Reconcile:"));

    // Validate + stats text output.
    let out = run_rust_candidate(rust_path, &["audit", "validate"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Audit Validate"));

    let out = run_rust_candidate(rust_path, &["audit", "stats"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Audit Stats"));

    // Stream is implemented as a one-shot tail.
    let out = run_rust_candidate(
        rust_path,
        &["audit", "stream", "--json", "--last", "2"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let lines: Vec<&str> = out
        .stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    assert!(!lines.is_empty());
    for line in lines {
        let _: serde_json::Value = serde_json::from_str(line).expect("json line");
    }
}
