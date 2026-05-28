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
#[path = "writer_tests.rs"]
mod writer_tests;
