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
    /// Number of events previously seen.
    ///
    /// This offset assumes the store is append-only between polls. If the
    /// underlying log is truncated or rotated, a shorter read is treated as a
    /// reset and the next append-only growth resumes from the new length.
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
///
/// Offsets are based on event counts, so a store that shrinks between polls is
/// treated as having reset; the shorter snapshot advances no offset until new
/// events extend the store again.
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
#[path = "stream_tests.rs"]
mod stream_tests;
