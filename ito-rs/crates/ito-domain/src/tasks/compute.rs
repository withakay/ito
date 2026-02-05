use std::collections::BTreeMap;

use super::{TaskItem, TaskStatus, TasksFormat, TasksParseResult};

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
        ready.sort_by(|a, b| a.header_line_index.cmp(&b.header_line_index));
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

    ready.sort_by(|a, b| {
        let aw = a.wave.unwrap_or(u32::MAX);
        let bw = b.wave.unwrap_or(u32::MAX);
        aw.cmp(&bw)
            .then(a.header_line_index.cmp(&b.header_line_index))
    });
    blocked.sort_by(|(a, _), (b, _)| {
        let aw = a.wave.unwrap_or(u32::MAX);
        let bw = b.wave.unwrap_or(u32::MAX);
        aw.cmp(&bw)
            .then(a.header_line_index.cmp(&b.header_line_index))
    });

    (ready, blocked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{ProgressInfo, TaskKind, WaveInfo};

    fn progress_zero() -> ProgressInfo {
        ProgressInfo {
            total: 0,
            complete: 0,
            shelved: 0,
            in_progress: 0,
            pending: 0,
            remaining: 0,
        }
    }

    fn task(
        id: &str,
        wave: Option<u32>,
        status: TaskStatus,
        deps: &[&str],
        header_line_index: usize,
    ) -> TaskItem {
        TaskItem {
            id: id.to_string(),
            name: id.to_string(),
            wave,
            status,
            updated_at: None,
            dependencies: deps.iter().map(|s| (*s).to_string()).collect(),
            files: Vec::new(),
            action: String::new(),
            verify: None,
            done_when: None,
            kind: TaskKind::Normal,
            header_line_index,
        }
    }

    #[test]
    fn checkbox_mode_returns_pending_sorted_and_no_blocked() {
        let parsed = TasksParseResult {
            format: TasksFormat::Checkbox,
            tasks: vec![
                task("2", None, TaskStatus::Pending, &[], 2),
                task("1", None, TaskStatus::Complete, &[], 1),
                task("3", None, TaskStatus::Pending, &[], 0),
            ],
            waves: Vec::new(),
            diagnostics: Vec::new(),
            progress: progress_zero(),
        };

        let (ready, blocked) = compute_ready_and_blocked(&parsed);
        assert!(blocked.is_empty());
        assert_eq!(ready.len(), 2);
        assert_eq!(ready[0].id, "3");
        assert_eq!(ready[1].id, "2");
    }

    #[test]
    fn enhanced_backcompat_blocks_later_waves_and_checkpoints_until_first_incomplete_wave_done() {
        let parsed = TasksParseResult {
            format: TasksFormat::Enhanced,
            tasks: vec![
                task("1.1", Some(1), TaskStatus::Pending, &[], 0),
                task("1.2", Some(1), TaskStatus::Complete, &[], 1),
                task("2.1", Some(2), TaskStatus::Pending, &[], 2),
                task("checkpoint", None, TaskStatus::Pending, &[], 3),
            ],
            waves: Vec::new(),
            diagnostics: Vec::new(),
            progress: progress_zero(),
        };

        let (ready, blocked) = compute_ready_and_blocked(&parsed);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "1.1");

        let mut blocked_ids: Vec<&str> = blocked.iter().map(|(t, _)| t.id.as_str()).collect();
        blocked_ids.sort();
        assert_eq!(blocked_ids, vec!["2.1", "checkpoint"]);

        let reasons_for_2_1 = blocked
            .iter()
            .find(|(t, _)| t.id == "2.1")
            .unwrap()
            .1
            .join("\n");
        assert!(reasons_for_2_1.contains("Blocked until Wave 1 is complete"));
    }

    #[test]
    fn enhanced_wave_dependency_blocks_by_wave_and_unblocks_when_complete() {
        let parsed = TasksParseResult {
            format: TasksFormat::Enhanced,
            tasks: vec![
                task("1.1", Some(1), TaskStatus::Complete, &[], 0),
                task("2.1", Some(2), TaskStatus::Pending, &[], 1),
            ],
            waves: vec![
                WaveInfo {
                    wave: 1,
                    depends_on: Vec::new(),
                    header_line_index: 0,
                    depends_on_line_index: None,
                },
                WaveInfo {
                    wave: 2,
                    depends_on: vec![1],
                    header_line_index: 0,
                    depends_on_line_index: None,
                },
            ],
            diagnostics: Vec::new(),
            progress: progress_zero(),
        };

        let (ready, blocked) = compute_ready_and_blocked(&parsed);
        assert!(blocked.is_empty());
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "2.1");

        let mut parsed = parsed;
        parsed.tasks[0].status = TaskStatus::Pending;
        let (ready, blocked) = compute_ready_and_blocked(&parsed);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "1.1");
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].0.id, "2.1");
        assert!(blocked[0].1.iter().any(|m| m.contains("Blocked by Wave 1")));
    }

    #[test]
    fn enhanced_task_dependencies_produce_missing_crosswave_and_not_complete_blockers() {
        let parsed = TasksParseResult {
            format: TasksFormat::Enhanced,
            tasks: vec![
                task("1.1", Some(1), TaskStatus::Complete, &[], 0),
                task("1.2", Some(1), TaskStatus::Pending, &["missing"], 1),
                task("2.1", Some(2), TaskStatus::Pending, &["1.1"], 2),
                task("2.2", Some(2), TaskStatus::Pending, &["2.1"], 3),
            ],
            waves: vec![
                WaveInfo {
                    wave: 1,
                    depends_on: Vec::new(),
                    header_line_index: 0,
                    depends_on_line_index: None,
                },
                WaveInfo {
                    wave: 2,
                    depends_on: Vec::new(),
                    header_line_index: 0,
                    depends_on_line_index: None,
                },
            ],
            diagnostics: Vec::new(),
            progress: progress_zero(),
        };

        let (ready, blocked) = compute_ready_and_blocked(&parsed);
        assert!(ready.is_empty());

        let b_1_2 = blocked.iter().find(|(t, _)| t.id == "1.2").unwrap();
        assert!(
            b_1_2
                .1
                .iter()
                .any(|m| m.contains("Missing dependency: missing"))
        );

        let b_2_1 = blocked.iter().find(|(t, _)| t.id == "2.1").unwrap();
        assert!(
            b_2_1
                .1
                .iter()
                .any(|m| m.contains("Cross-wave dependency: 1.1"))
        );

        let b_2_2 = blocked.iter().find(|(t, _)| t.id == "2.2").unwrap();
        assert!(
            b_2_2
                .1
                .iter()
                .any(|m| m.contains("Dependency not complete: 2.1"))
        );
    }
}
