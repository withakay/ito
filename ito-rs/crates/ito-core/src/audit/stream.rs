//! Poll-based audit event streaming with multi-worktree support.
//!
//! Provides a simple polling mechanism that checks routed audit storage for
//! new events at a configurable interval. Supports monitoring events across
//! multiple git worktrees without assuming a tracked worktree JSONL file.

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use ito_domain::audit::event::AuditEvent;

use super::store::{AuditEventStore, audit_storage_location_key, default_audit_store};
use super::worktree::discover_worktrees;

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
pub struct StreamSource {
    /// Label for this source (e.g., "main" or worktree branch name).
    pub label: String,
    /// Routed audit store retained across polls.
    store: Box<dyn AuditEventStore>,
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
    let mut seen_locations = HashSet::new();

    // Main project source
    let main_store = default_audit_store(ito_path);
    let main_key = audit_storage_location_key(&main_store.location());
    let main_events = main_store.read_all();
    let start = main_events.len().saturating_sub(config.last);
    for event in &main_events[start..] {
        events.push(StreamEvent {
            event: event.clone(),
            source: "main".to_string(),
        });
    }
    sources.push(StreamSource {
        label: "main".to_string(),
        store: main_store,
        offset: main_events.len(),
    });
    seen_locations.insert(main_key);

    // Worktree sources
    if config.all_worktrees {
        let worktrees = discover_worktrees(ito_path);
        for wt in &worktrees {
            if wt.is_main {
                continue; // Already handled above
            }
            let wt_ito_path = wt.path.join(".ito");
            if !wt_ito_path.exists() {
                continue;
            }

            let wt_store = default_audit_store(&wt_ito_path);
            let wt_key = audit_storage_location_key(&wt_store.location());
            if !seen_locations.insert(wt_key) {
                continue;
            }

            let wt_events = wt_store.read_all();
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
                store: wt_store,
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
        let current_events = source.store.read_all();
        if current_events.len() <= source.offset {
            continue;
        }

        for event in &current_events[source.offset..] {
            new_events.push(StreamEvent {
                event: event.clone(),
                source: source.label.clone(),
            });
        }

        source.offset = current_events.len();
    }

    new_events
}

#[cfg(test)]
mod tests {
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
}
