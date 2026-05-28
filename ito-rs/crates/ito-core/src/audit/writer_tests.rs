use super::*;
use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};

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
fn creates_directory_and_file_on_first_write() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&test_event("1.1")).expect("append");

    assert!(writer.log_path().exists());
}

#[test]
fn appends_events_to_existing_file() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&test_event("1.1")).expect("first append");
    writer.append(&test_event("1.2")).expect("second append");

    let contents = std::fs::read_to_string(writer.log_path()).expect("read");
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn preserves_existing_content() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&test_event("1.1")).expect("first append");

    let first_line = std::fs::read_to_string(writer.log_path()).expect("read");

    writer.append(&test_event("1.2")).expect("second append");

    let contents = std::fs::read_to_string(writer.log_path()).expect("read");
    assert!(contents.starts_with(first_line.trim()));
}

#[test]
fn events_deserialize_back_correctly() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let event = test_event("1.1");
    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&event).expect("append");

    let contents = std::fs::read_to_string(writer.log_path()).expect("read");
    let parsed: AuditEvent =
        serde_json::from_str(contents.lines().next().expect("line")).expect("parse");
    assert_eq!(parsed, event);
}

#[test]
fn best_effort_returns_ok_even_on_failure() {
    // Write to an invalid path (nested under a file, not a directory)
    let tmp = tempfile::tempdir().expect("tempdir");
    let file_path = tmp.path().join("not_a_dir");
    std::fs::write(&file_path, "block").expect("write blocker");

    let writer = FsAuditWriter {
        log_path: file_path.join("subdir").join("events.jsonl"),
        ito_path: PathBuf::from("/project/.ito"),
        mirror_settings: OnceLock::new(),
    };
    // Should not panic and should return Ok
    let result = writer.append(&test_event("1.1"));
    assert!(result.is_ok());
}

#[test]
fn audit_log_path_resolves_correctly() {
    let path = audit_log_path(Path::new("/project/.ito"));
    assert_eq!(
        path,
        PathBuf::from("/project/.ito/.state/audit/events.jsonl")
    );
}

#[test]
fn each_line_is_valid_json() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = FsAuditWriter::new(&ito_path);
    for i in 0..5 {
        writer
            .append(&test_event(&format!("1.{i}")))
            .expect("append");
    }

    let contents = std::fs::read_to_string(writer.log_path()).expect("read");
    for line in contents.lines() {
        let _: AuditEvent = serde_json::from_str(line).expect("valid JSON");
    }
}
