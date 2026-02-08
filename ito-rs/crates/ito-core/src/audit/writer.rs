//! Filesystem-backed audit log writer.
//!
//! Appends events as single-line JSON to `.ito/.state/audit/events.jsonl`.
//! All writes are best-effort: failures are logged but never block the caller.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use ito_domain::audit::event::AuditEvent;
use ito_domain::audit::writer::AuditWriter;

/// Filesystem-backed implementation of `AuditWriter`.
///
/// Appends events to `{ito_path}/.state/audit/events.jsonl` in JSONL format.
pub struct FsAuditWriter {
    log_path: PathBuf,
}

impl FsAuditWriter {
    /// Create a new writer for the given Ito project path.
    pub fn new(ito_path: &Path) -> Self {
        let log_path = audit_log_path(ito_path);
        Self { log_path }
    }

    /// Return the path to the audit log file.
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }
}

impl AuditWriter for FsAuditWriter {
    fn append(&self, event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Best-effort: serialize, create dirs, append, flush.
        // On any failure, log a warning and return Ok.
        if let Err(e) = append_event_to_file(&self.log_path, event) {
            tracing::warn!("audit log write failed: {e}");
        }
        Ok(())
    }
}

/// Append a single event to the JSONL file at `path`.
fn append_event_to_file(path: &Path, event: &AuditEvent) -> std::io::Result<()> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string(event)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "{json}")?;
    file.flush()?;

    Ok(())
}

/// Returns the canonical path for the audit log file.
pub fn audit_log_path(ito_path: &Path) -> PathBuf {
    ito_path.join(".state").join("audit").join("events.jsonl")
}

#[cfg(test)]
mod tests {
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
}
