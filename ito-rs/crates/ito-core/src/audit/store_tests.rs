use super::*;
use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};

fn run_git(repo: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git command failed: git {}\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_git_repo(repo: &Path) {
    run_git(repo, &["init"]);
    run_git(repo, &["config", "user.email", "test@example.com"]);
    run_git(repo, &["config", "user.name", "Test User"]);
    run_git(repo, &["config", "commit.gpgsign", "false"]);
    std::fs::write(repo.join("README.md"), "hi\n").expect("write readme");
    run_git(repo, &["add", "README.md"]);
    run_git(repo, &["commit", "-m", "initial"]);
}

fn test_event(entity_id: &str) -> AuditEvent {
    AuditEvent {
        v: SCHEMA_VERSION,
        ts: "2026-02-08T14:30:00.000Z".to_string(),
        entity: "task".to_string(),
        entity_id: entity_id.to_string(),
        scope: Some("test-change".to_string()),
        op: "create".to_string(),
        from: None,
        to: Some("pending".to_string()),
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        count: 1,
        ctx: EventContext {
            session_id: "test-sid".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    }
}

#[test]
fn internal_branch_location_keys_include_branch_identity() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let first = LocalAuditStore::new(
        &ito_path,
        "ito/internal/audit-one".to_string(),
        tmp.path().join("one.jsonl"),
    );
    let second = LocalAuditStore::new(
        &ito_path,
        "ito/internal/audit-two".to_string(),
        tmp.path().join("two.jsonl"),
    );

    assert_ne!(
        audit_storage_location_key(&first.location()),
        audit_storage_location_key(&second.location())
    );
}

#[test]
fn read_all_merges_and_replays_fallback_events_when_branch_recovers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    init_git_repo(tmp.path());
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("create ito dir");

    let fallback_path = tmp.path().join("fallback-events.jsonl");
    let store = LocalAuditStore::new(
        &ito_path,
        "ito/internal/audit".to_string(),
        fallback_path.clone(),
    );

    let branch_event = test_event("1.1");
    store
        .append_to_branch(&branch_event)
        .expect("append branch event");

    let fallback_event = test_event("1.2");
    append_event_to_file(&fallback_path, &fallback_event).expect("append fallback event");

    let events = store.read_all();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].entity_id, "1.1");
    assert_eq!(events[1].entity_id, "1.2");

    let replayed = store.read_from_branch().expect("read branch after replay");
    let InternalBranchRead::Events(replayed) = replayed else {
        panic!("expected branch events after replay");
    };
    assert_eq!(replayed.len(), 2);
    assert!(!fallback_path.exists());
}

#[test]
fn legacy_worktree_log_is_removed_after_successful_migration() {
    let tmp = tempfile::tempdir().expect("tempdir");
    init_git_repo(tmp.path());
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("create ito dir");

    let fallback_path = tmp.path().join("fallback-events.jsonl");
    let store = LocalAuditStore::new(&ito_path, "ito/internal/audit".to_string(), fallback_path);
    let legacy_path = audit_log_path(&ito_path);
    append_event_to_file(&legacy_path, &test_event("1.1")).expect("write legacy event");

    let events = store.read_all();
    assert_eq!(events.len(), 1);
    assert!(!legacy_path.exists());

    let branch_events = store
        .read_from_branch()
        .expect("read migrated branch events");
    let InternalBranchRead::Events(branch_events) = branch_events else {
        panic!("expected migrated branch events");
    };
    assert_eq!(branch_events.len(), 1);
}
