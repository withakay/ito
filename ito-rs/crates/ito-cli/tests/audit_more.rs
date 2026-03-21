#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

fn run_git(repo: &std::path::Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run")
}

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
    let mut lines = Vec::new();
    for line in out.stdout.lines() {
        if !line.trim().is_empty() {
            lines.push(line);
        }
    }
    assert!(!lines.is_empty());
    for line in lines {
        let _: serde_json::Value = serde_json::from_str(line).expect("json line");
    }
}

#[test]
fn audit_more_local_audit_writes_use_internal_branch_without_worktree_log_churn() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
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

    assert!(
        !repo.path().join(".ito/.state/audit/events.jsonl").exists(),
        "local mode should not write tracked audit history into the worktree"
    );

    let show = run_git(
        repo.path(),
        &[
            "show",
            "refs/heads/ito/internal/audit:.ito/.state/audit/events.jsonl",
        ],
    );
    assert!(
        show.status.success(),
        "stderr={} stdout={}",
        String::from_utf8_lossy(&show.stderr),
        String::from_utf8_lossy(&show.stdout)
    );
    let branch_log = String::from_utf8_lossy(&show.stdout);
    assert!(branch_log.contains("\"entity_id\":\"1.1\""));
}

#[test]
fn audit_more_local_audit_writes_warn_and_fallback_without_worktree_log_when_branch_storage_is_unavailable()
 {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
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
    assert!(out.stderr.contains("fallback"), "stderr={}", out.stderr);
    assert!(
        !repo.path().join(".ito/.state/audit/events.jsonl").exists(),
        "fallback should not recreate the tracked worktree log"
    );

    let out = run_rust_candidate(
        rust_path,
        &["audit", "log", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let events: serde_json::Value = serde_json::from_str(&out.stdout).expect("audit log json");
    let events = events.as_array().expect("audit log array");
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(events[0]["entity_id"], "1.1");
}

#[test]
fn audit_commands_migrate_legacy_worktree_log_into_routed_storage() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
    fixtures::write(
        repo.path().join(".ito/changes/test-change/tasks.md"),
        "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [ ] pending\n",
    );
    let legacy_event = serde_json::json!({
        "v": 1,
        "ts": "2026-03-16T12:00:00.000Z",
        "entity": "task",
        "entity_id": "1.1",
        "scope": "test-change",
        "op": "create",
        "from": serde_json::Value::Null,
        "to": "pending",
        "actor": "cli",
        "by": "@test",
        "meta": serde_json::Value::Null,
        "ctx": {
            "session_id": "test-session",
            "harness_session_id": serde_json::Value::Null,
            "branch": serde_json::Value::Null,
            "worktree": serde_json::Value::Null,
            "commit": serde_json::Value::Null
        }
    });
    fixtures::write(
        repo.path().join(".ito/.state/audit/events.jsonl"),
        &(serde_json::to_string(&legacy_event).expect("legacy event") + "\n"),
    );

    let log = run_rust_candidate(
        rust_path,
        &["audit", "log", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(log.code, 0, "stderr={}", log.stderr);
    let events: serde_json::Value = serde_json::from_str(&log.stdout).expect("audit log json");
    let events = events.as_array().expect("audit log array");
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(events[0]["entity_id"], "1.1");

    let validate = run_rust_candidate(
        rust_path,
        &["audit", "validate", "--json", "--change", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(validate.code, 0, "stderr={}", validate.stderr);
    let validate_report: serde_json::Value =
        serde_json::from_str(&validate.stdout).expect("audit validate json");
    assert_eq!(validate_report["event_count"], 1);
    assert_eq!(validate_report["valid"], true);

    let reconcile = run_rust_candidate(
        rust_path,
        &["audit", "reconcile", "--json", "--change", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(reconcile.code, 0, "stderr={}", reconcile.stderr);
    let reconcile_report: serde_json::Value =
        serde_json::from_str(&reconcile.stdout).expect("reconcile json");
    assert_eq!(reconcile_report["drift_count"], 0);

    let stream = run_rust_candidate(
        rust_path,
        &["audit", "stream", "--json", "--last", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(stream.code, 0, "stderr={}", stream.stderr);
    let stream_line = stream
        .stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap();
    let stream_event: serde_json::Value =
        serde_json::from_str(stream_line).expect("stream json line");
    assert_eq!(stream_event["entity_id"], "1.1");

    let migrated = run_git(
        repo.path(),
        &[
            "show",
            "refs/heads/ito/internal/audit:.ito/.state/audit/events.jsonl",
        ],
    );
    assert!(
        migrated.status.success(),
        "stderr={} stdout={}",
        String::from_utf8_lossy(&migrated.stderr),
        String::from_utf8_lossy(&migrated.stdout)
    );
    assert!(String::from_utf8_lossy(&migrated.stdout).contains("\"entity_id\":\"1.1\""));
}

#[test]
fn audit_stream_all_worktrees_dedupes_shared_routed_storage() {
    let repo = fixtures::make_empty_repo();
    let home = tempfile::tempdir().expect("home");
    let worktree_root = tempfile::tempdir().expect("worktree root");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::git_init_with_initial_commit(repo.path());
    std::fs::create_dir_all(repo.path().join(".ito/changes/test-change")).unwrap();

    let sibling = worktree_root.path().join("repo-wt");
    let sibling_path = sibling.to_string_lossy().to_string();
    let add_worktree = run_git(
        repo.path(),
        &[
            "worktree",
            "add",
            sibling_path.as_str(),
            "-b",
            "feature-audit-stream",
        ],
    );
    assert_eq!(
        add_worktree.status.code(),
        Some(0),
        "stderr={} stdout={}",
        String::from_utf8_lossy(&add_worktree.stderr),
        String::from_utf8_lossy(&add_worktree.stdout)
    );
    std::fs::create_dir_all(sibling.join(".ito")).unwrap();

    let init = run_rust_candidate(
        rust_path,
        &["tasks", "init", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(init.code, 0, "stderr={}", init.stderr);

    let start = run_rust_candidate(
        rust_path,
        &["tasks", "start", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(start.code, 0, "stderr={}", start.stderr);

    let stream = run_rust_candidate(
        rust_path,
        &[
            "audit",
            "stream",
            "--json",
            "--last",
            "10",
            "--all-worktrees",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(stream.code, 0, "stderr={}", stream.stderr);
    let lines: Vec<_> = stream
        .stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(lines.len(), 1, "stdout={}", stream.stdout);
    let event: serde_json::Value = serde_json::from_str(lines[0]).expect("stream json line");
    assert_eq!(event["entity_id"], "1.1");
}
