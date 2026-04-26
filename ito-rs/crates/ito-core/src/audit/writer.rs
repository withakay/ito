//! Filesystem-backed audit log helpers.
//!
//! These helpers append events as single-line JSONL at a concrete filesystem path.
//! Routed audit storage chooses when filesystem paths are used (for example on an
//! internal audit branch or a local fallback store); callers should prefer the
//! routed store entrypoints over writing directly to a worktree path.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use ito_config::{ConfigContext, load_cascading_project_config, resolve_audit_mirror_settings};
use ito_domain::audit::event::AuditEvent;
use ito_domain::audit::writer::AuditWriter;

use super::store::{AuditEventStore, AuditStorageLocation};

/// Filesystem-backed implementation of `AuditWriter` for a specific log path.
///
/// Prefer `default_audit_store()` for normal CLI/runtime usage so audit writes
/// follow the routed storage policy. Construct `FsAuditWriter` directly only
/// when a caller intentionally needs a concrete filesystem log path.
pub struct FsAuditWriter {
    log_path: PathBuf,
    ito_path: PathBuf,
    mirror_settings: OnceLock<(bool, String)>,
}

impl FsAuditWriter {
    /// Create a new writer for the given Ito project path.
    pub fn new(ito_path: &Path) -> Self {
        let log_path = audit_log_path(ito_path);
        Self {
            log_path,
            ito_path: ito_path.to_path_buf(),
            mirror_settings: OnceLock::new(),
        }
    }

    /// Return the path to the audit log file.
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }

    fn resolve_mirror_settings(&self) -> (bool, String) {
        self.mirror_settings
            .get_or_init(|| {
                let Some(project_root) = self.ito_path.parent() else {
                    return (false, String::new());
                };
                let ctx = ConfigContext::from_process_env();
                let resolved = load_cascading_project_config(project_root, &self.ito_path, &ctx);
                resolve_audit_mirror_settings(&resolved.merged)
            })
            .clone()
    }
}

impl AuditWriter for FsAuditWriter {
    fn append(&self, event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Best-effort: serialize, create dirs, append, flush.
        // On any failure, log a warning and return Ok.
        if let Err(e) = append_event_to_file(&self.log_path, event) {
            tracing::warn!("audit log write failed: {e}");
            return Ok(());
        }

        let (enabled, branch) = self.resolve_mirror_settings();
        if enabled {
            let Some(repo_root) = self.ito_path.parent() else {
                return Ok(());
            };
            if let Err(err) = super::mirror::sync_audit_mirror(repo_root, &self.ito_path, &branch) {
                eprintln!(
                    "Warning: audit mirror sync failed (branch '{}'): {err}",
                    branch
                );
            }
        }
        Ok(())
    }
}

impl AuditEventStore for FsAuditWriter {
    fn read_all(&self) -> Vec<AuditEvent> {
        read_events_from_path(&self.log_path)
    }

    fn location(&self) -> AuditStorageLocation {
        AuditStorageLocation::Filesystem(self.log_path.clone())
    }
}

/// Append a single event to the JSONL file at `path`.
pub(crate) fn append_event_to_file(path: &Path, event: &AuditEvent) -> std::io::Result<()> {
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

pub(crate) fn read_events_from_path(path: &Path) -> Vec<AuditEvent> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    parse_events_from_jsonl(&contents)
}

pub(crate) fn parse_events_from_jsonl(contents: &str) -> Vec<AuditEvent> {
    let mut events = Vec::new();
    for (line_num, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<AuditEvent>(line) {
            Ok(event) => events.push(event),
            Err(e) => {
                tracing::warn!("audit log line {}: malformed event: {e}", line_num + 1);
            }
        }
    }

    events
}

/// Returns the legacy worktree-relative audit log path.
///
/// This helper remains available for migration and compatibility code. New
/// routed audit writes should go through `default_audit_store()` instead.
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
}
