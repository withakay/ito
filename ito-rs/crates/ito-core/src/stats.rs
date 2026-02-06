//! Statistics collection and computation for Ito command usage.
//!
//! This module provides functions to parse execution logs and compute
//! command usage statistics from `.jsonl` log files.

use crate::errors::CoreResult;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};

/// Statistics about command usage, keyed by command ID.
#[derive(Debug, Clone)]
pub struct CommandStats {
    /// Map from command ID to execution count.
    pub counts: BTreeMap<String, u64>,
}

/// Recursively collect all `.jsonl` files in a directory tree.
///
/// Returns a vector of paths to `.jsonl` files found under `dir`.
/// Silently skips directories or files that cannot be read.
pub fn collect_jsonl_files(dir: &Path) -> CoreResult<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect_jsonl_files_recursive(dir, &mut out);
    Ok(out)
}

fn collect_jsonl_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries {
        let Ok(e) = e else {
            continue;
        };
        let path = e.path();
        if path.is_dir() {
            collect_jsonl_files_recursive(&path, out);
            continue;
        }
        let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
            continue;
        };
        if ext == "jsonl" {
            out.push(path);
        }
    }
}

/// Compute command usage statistics from execution logs.
///
/// Parses all `.jsonl` files under `log_dir`, looking for `command_end` events,
/// and counts how many times each known command ID has been executed.
///
/// Returns a [`CommandStats`] struct with counts for all known commands.
pub fn compute_command_stats(log_dir: &Path) -> CoreResult<CommandStats> {
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    for id in known_command_ids() {
        counts.insert(id.to_string(), 0);
    }

    let files = collect_jsonl_files(log_dir)?;

    for path in files {
        let Ok(f) = std::fs::File::open(&path) else {
            continue;
        };
        let reader = std::io::BufReader::new(f);
        for line in reader.lines() {
            let Ok(line) = line else {
                continue;
            };
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            #[derive(serde::Deserialize)]
            struct Event {
                event_type: Option<String>,
                command_id: Option<String>,
            }

            let Ok(ev) = serde_json::from_str::<Event>(line) else {
                continue;
            };
            let Some(event_type) = ev.event_type else {
                continue;
            };
            if event_type != "command_end" {
                continue;
            }
            let Some(command_id) = ev.command_id else {
                continue;
            };

            let entry = counts.entry(command_id).or_insert(0);
            *entry = entry.saturating_add(1);
        }
    }

    Ok(CommandStats { counts })
}

/// Return the static list of known Ito command IDs.
///
/// This list is used to initialize the stats map with zero counts for all
/// known commands, even if they haven't been executed yet.
pub fn known_command_ids() -> &'static [&'static str] {
    &[
        "ito.init",
        "ito.update",
        "ito.list",
        "ito.config.path",
        "ito.config.list",
        "ito.config.get",
        "ito.config.set",
        "ito.config.unset",
        "ito.agent_config.init",
        "ito.agent_config.summary",
        "ito.agent_config.get",
        "ito.agent_config.set",
        "ito.create.module",
        "ito.create.change",
        "ito.new.change",
        "ito.plan.init",
        "ito.plan.status",
        "ito.state.show",
        "ito.state.decision",
        "ito.state.blocker",
        "ito.state.note",
        "ito.state.focus",
        "ito.state.question",
        "ito.tasks.init",
        "ito.tasks.status",
        "ito.tasks.next",
        "ito.tasks.start",
        "ito.tasks.complete",
        "ito.tasks.shelve",
        "ito.tasks.unshelve",
        "ito.tasks.add",
        "ito.tasks.show",
        "ito.workflow.init",
        "ito.workflow.list",
        "ito.workflow.show",
        "ito.status",
        "ito.stats",
        "ito.templates",
        "ito.instructions",
        "ito.x_instructions",
        "ito.agent.instruction",
        "ito.show",
        "ito.validate",
        "ito.ralph",
        "ito.loop",
    ]
}
