//! Task scheduling helpers.
//!
//! This module contains the logic for determining which tasks are actionable.
//! It handles:
//! - **Wave Gating**: Tasks in Wave N are blocked until all tasks in Wave N-1 (or explicit dependencies) are complete.
//! - **Explicit Dependencies**: Tasks can depend on specific other tasks by ID.
//! - **Back-compat**: Legacy behavior for files without explicit wave definitions.

use std::collections::BTreeMap;

use super::{TaskItem, TaskStatus, TasksFormat, TasksParseResult};

fn parse_numeric_task_id(id: &str) -> Option<(u32, u32)> {
    let (wave, task) = id.split_once('.')?;
    let wave = wave.parse::<u32>().ok()?;
    let task = task.parse::<u32>().ok()?;
    Some((wave, task))
}

fn compare_task_ids(a: &str, b: &str) -> std::cmp::Ordering {
    match (parse_numeric_task_id(a), parse_numeric_task_id(b)) {
        (Some(aa), Some(bb)) => aa.cmp(&bb).then(a.cmp(b)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

/// Compute ready tasks and blocked tasks (with reasons) from a parsed tasks file.
pub fn compute_ready_and_blocked(
    parsed: &TasksParseResult,
) -> (Vec<TaskItem>, Vec<(TaskItem, Vec<String>)>) {
    let tasks = &parsed.tasks;

    if parsed.format == TasksFormat::Checkbox {
        let has_in_progress = tasks.iter().any(|t| t.status == TaskStatus::InProgress);
        if has_in_progress {
            return (Vec::new(), Vec::new());
        }
        let mut ready: Vec<TaskItem> = tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .cloned()
            .collect();
        ready.sort_by_key(|task| task.header_line_index);
        return (ready, Vec::new());
    }

    let mut by_id: std::collections::BTreeMap<&str, &TaskItem> = std::collections::BTreeMap::new();
    for t in tasks {
        by_id.insert(t.id.as_str(), t);
    }

    let mut wave_complete: BTreeMap<u32, bool> = BTreeMap::new();
    for w in &parsed.waves {
        let done = tasks
            .iter()
            .filter(|t| t.wave == Some(w.wave))
            .all(|t| t.status.is_done());
        wave_complete.insert(w.wave, done);
    }

    let mut wave_unlocked: BTreeMap<u32, bool> = BTreeMap::new();
    for w in &parsed.waves {
        let unlocked = w
            .depends_on
            .iter()
            .all(|dep| wave_complete.get(dep).copied().unwrap_or(false));
        wave_unlocked.insert(w.wave, unlocked);
    }

    // Back-compat gating when no WaveInfo entries exist.
    let mut first_incomplete_wave: Option<u32> = None;
    if parsed.waves.is_empty() {
        let mut waves: Vec<u32> = tasks.iter().filter_map(|t| t.wave).collect();
        waves.sort();
        waves.dedup();
        for w in waves {
            let all_done = tasks
                .iter()
                .filter(|t| t.wave == Some(w))
                .all(|t| t.status.is_done());
            if !all_done {
                first_incomplete_wave = Some(w);
                break;
            }
        }
    }

    let all_waves_complete = if parsed.waves.is_empty() {
        first_incomplete_wave.is_none()
    } else {
        wave_complete.values().all(|v| *v)
    };

    let mut ready: Vec<TaskItem> = Vec::new();
    let mut blocked: Vec<(TaskItem, Vec<String>)> = Vec::new();

    for t in tasks {
        if t.status != TaskStatus::Pending {
            continue;
        }
        let mut blockers: Vec<String> = Vec::new();

        if parsed.waves.is_empty() {
            if let Some(first) = first_incomplete_wave {
                let is_later_wave = t.wave.is_some_and(|w| w > first);
                let is_checkpoint_like = t.wave.is_none();
                if is_later_wave || is_checkpoint_like {
                    blockers.push(format!("Blocked until Wave {first} is complete"));
                }
            }
        } else {
            match t.wave {
                Some(w) => {
                    if !wave_unlocked.get(&w).copied().unwrap_or(true) {
                        if let Some(wave) = parsed.waves.iter().find(|wi| wi.wave == w) {
                            for dep in &wave.depends_on {
                                if !wave_complete.get(dep).copied().unwrap_or(false) {
                                    blockers.push(format!("Blocked by Wave {dep}"));
                                }
                            }
                        } else {
                            blockers.push(format!("Blocked: Wave {w} is locked"));
                        }
                    }
                }
                None => {
                    if !all_waves_complete {
                        blockers.push("Blocked until all waves are complete".to_string());
                    }
                }
            }
        }

        for dep in &t.dependencies {
            if dep.is_empty() || dep == "Checkpoint" {
                continue;
            }
            let Some(dep_task) = by_id.get(dep.as_str()).copied() else {
                blockers.push(format!("Missing dependency: {dep}"));
                continue;
            };
            if t.wave != dep_task.wave {
                blockers.push(format!("Cross-wave dependency: {dep}"));
            }
            if dep_task.status != TaskStatus::Complete {
                blockers.push(format!("Dependency not complete: {dep}"));
            }
        }

        if blockers.is_empty() {
            ready.push(t.clone());
        } else {
            blocked.push((t.clone(), blockers));
        }
    }

    ready.sort_by(|a, b| compare_task_ids(&a.id, &b.id));
    blocked.sort_by(|(a, _), (b, _)| compare_task_ids(&a.id, &b.id));

    (ready, blocked)
}

#[cfg(test)]
#[path = "compute_tests.rs"]
mod compute_tests;
