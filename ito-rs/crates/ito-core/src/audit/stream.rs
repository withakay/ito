//! Poll-based audit event streaming with multi-worktree support.
//!
//! Provides a simple file-watching mechanism that polls the JSONL audit log
//! for new events at a configurable interval. Supports monitoring events
//! across multiple git worktrees.

use std::path::{Path, PathBuf};
use std::time::Duration;

use ito_domain::audit::event::AuditEvent;

use super::reader::read_audit_events;
use super::worktree::{discover_worktrees, worktree_audit_log_path};
use super::writer::audit_log_path;

/// Configuration for the event stream.
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Poll interval (default: 500ms).
    pub poll_interval: Duration,
    /// Monitor all worktrees, not just the current one.
    pub all_worktrees: bool,
    /// Number of initial events to emit on startup.
    pub last: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(500),
            all_worktrees: false,
            last: 10,
        }
    }
}

/// A single stream source: either the main project or a worktree.
#[derive(Debug)]
pub struct StreamSource {
    /// Label for this source (e.g., "main" or worktree branch name).
    pub label: String,
    /// Path to the audit log file.
    log_path: PathBuf,
    /// Number of lines previously seen.
    offset: usize,
}

/// A streamed event with its source label.
#[derive(Debug)]
pub struct StreamEvent {
    /// The audit event.
    pub event: AuditEvent,
    /// Label of the source (e.g., "main" or branch name).
    pub source: String,
}

/// Read the initial batch of events for streaming (the last N events).
///
/// Returns events from the main project log and, if `all_worktrees` is true,
/// from all discovered worktrees.
pub fn read_initial_events(
    ito_path: &Path,
    config: &StreamConfig,
) -> (Vec<StreamEvent>, Vec<StreamSource>) {
    let mut sources = Vec::new();
    let mut events = Vec::new();

    // Main project source
    let main_events = read_audit_events(ito_path);
    let main_log = audit_log_path(ito_path);
    let start = main_events.len().saturating_sub(config.last);
    for event in &main_events[start..] {
        events.push(StreamEvent {
            event: event.clone(),
            source: "main".to_string(),
        });
    }
    sources.push(StreamSource {
        label: "main".to_string(),
        log_path: main_log,
        offset: main_events.len(),
    });

    // Worktree sources
    if config.all_worktrees {
        let worktrees = discover_worktrees(ito_path);
        for wt in &worktrees {
            if wt.is_main {
                continue; // Already handled above
            }
            let wt_ito_path = wt.path.join(".ito");
            let wt_log = worktree_audit_log_path(wt);
            let wt_events = read_audit_events(&wt_ito_path);
            let label = wt
                .branch
                .clone()
                .unwrap_or_else(|| wt.path.display().to_string());
            let start = wt_events.len().saturating_sub(config.last);
            for event in &wt_events[start..] {
                events.push(StreamEvent {
                    event: event.clone(),
                    source: label.clone(),
                });
            }
            sources.push(StreamSource {
                label,
                log_path: wt_log,
                offset: wt_events.len(),
            });
        }
    }

    (events, sources)
}

/// Poll all sources for new events since the last check.
///
/// Updates the offsets in each source so subsequent polls only return new events.
pub fn poll_new_events(sources: &mut [StreamSource]) -> Vec<StreamEvent> {
    let mut new_events = Vec::new();

    for source in sources.iter_mut() {
        let Ok(contents) = std::fs::read_to_string(&source.log_path) else {
            continue;
        };

        let lines: Vec<&str> = contents.lines().collect();
        if lines.len() <= source.offset {
            continue;
        }

        for line in &lines[source.offset..] {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(event) = serde_json::from_str::<AuditEvent>(line) {
                new_events.push(StreamEvent {
                    event,
                    source: source.label.clone(),
                });
            }
        }

        source.offset = lines.len();
    }

    new_events
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};
    use ito_domain::audit::writer::AuditWriter;

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
    fn read_initial_events_returns_last_n() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = crate::audit::writer::FsAuditWriter::new(&ito_path);
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

        let writer = crate::audit::writer::FsAuditWriter::new(&ito_path);
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

        let writer = crate::audit::writer::FsAuditWriter::new(&ito_path);
        writer.append(&test_event("1.1")).expect("append");

        let config = StreamConfig::default();
        let (_initial, mut sources) = read_initial_events(&ito_path, &config);

        let new = poll_new_events(&mut sources);
        assert!(new.is_empty());
    }

    #[test]
    fn default_config_has_sensible_values() {
        let config = StreamConfig::default();
        assert_eq!(config.poll_interval, Duration::from_millis(500));
        assert!(!config.all_worktrees);
        assert_eq!(config.last, 10);
    }
}
