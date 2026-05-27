use super::*;
use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};
use std::path::Path;

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
fn read_initial_events_returns_last_n() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = crate::audit::default_audit_store(&ito_path);
    for i in 0..20 {
        writer
            .append(&test_event(&format!("1.{i}")))
            .expect("append");
    }

    let config = StreamConfig {
        last: 5,
        all_worktrees: false,
        ..Default::default()
    };

    let (events, sources) = read_initial_events(&ito_path, &config);
    assert_eq!(events.len(), 5);
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].offset, 20);
    assert_eq!(events[0].event.entity_id, "1.15");
}

#[test]
fn poll_detects_new_events() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = crate::audit::default_audit_store(&ito_path);
    writer.append(&test_event("1.1")).expect("append");

    let config = StreamConfig::default();
    let (_initial, mut sources) = read_initial_events(&ito_path, &config);

    // Write more events
    writer.append(&test_event("1.2")).expect("append");
    writer.append(&test_event("1.3")).expect("append");

    let new = poll_new_events(&mut sources);
    assert_eq!(new.len(), 2);
    assert_eq!(new[0].event.entity_id, "1.2");
    assert_eq!(new[1].event.entity_id, "1.3");
}

#[test]
fn poll_returns_empty_when_no_new_events() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = crate::audit::default_audit_store(&ito_path);
    writer.append(&test_event("1.1")).expect("append");

    let config = StreamConfig::default();
    let (_initial, mut sources) = read_initial_events(&ito_path, &config);

    let new = poll_new_events(&mut sources);
    assert!(new.is_empty());
}

#[test]
fn poll_detects_new_events_from_routed_store() {
    let tmp = tempfile::tempdir().expect("tempdir");
    init_git_repo(tmp.path());
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("create ito dir");

    let store = crate::audit::default_audit_store(&ito_path);
    store.append(&test_event("1.1")).expect("append");

    let config = StreamConfig::default();
    let (_initial, mut sources) = read_initial_events(&ito_path, &config);

    store.append(&test_event("1.2")).expect("append");
    store.append(&test_event("1.3")).expect("append");

    let new = poll_new_events(&mut sources);
    assert_eq!(new.len(), 2);
    assert_eq!(new[0].event.entity_id, "1.2");
    assert_eq!(new[1].event.entity_id, "1.3");
}

#[test]
fn default_config_has_sensible_values() {
    let config = StreamConfig::default();
    assert_eq!(config.poll_interval, Duration::from_millis(500));
    assert!(!config.all_worktrees);
    assert_eq!(config.last, 10);
}
