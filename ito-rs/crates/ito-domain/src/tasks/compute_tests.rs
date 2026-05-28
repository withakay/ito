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

/// Constructs a TaskItem with the given id, wave, status, dependencies, and header index, using sensible defaults for other fields.
///
/// The function is intended as a test helper to create TaskItem instances quickly.
///
/// # Returns
///
/// A `TaskItem` populated with the provided values; unspecified fields are set to empty or `None` defaults.
///
/// # Examples
///
/// ```
/// let t = task("1.1", Some(1), TaskStatus::Pending, &["1.0"], 10);
/// assert_eq!(t.id, "1.1");
/// assert_eq!(t.wave, Some(1));
/// assert_eq!(t.dependencies, vec!["1.0"]);
/// assert_eq!(t.header_line_index, 10);
/// ```
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
        requirements: Vec::new(),
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

#[test]
fn enhanced_ready_and_blocked_lists_are_sorted_by_task_id() {
    let parsed = TasksParseResult {
        format: TasksFormat::Enhanced,
        tasks: vec![
            task("1.2", Some(1), TaskStatus::Pending, &[], 80),
            task("1.1", Some(1), TaskStatus::Pending, &["missing"], 120),
            task("1.3", Some(1), TaskStatus::Pending, &[], 40),
        ],
        waves: vec![WaveInfo {
            wave: 1,
            depends_on: Vec::new(),
            header_line_index: 200,
            depends_on_line_index: None,
        }],
        diagnostics: Vec::new(),
        progress: progress_zero(),
    };

    let (ready, blocked) = compute_ready_and_blocked(&parsed);
    assert_eq!(
        ready.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["1.2", "1.3"]
    );
    assert_eq!(
        blocked
            .iter()
            .map(|(t, _)| t.id.as_str())
            .collect::<Vec<_>>(),
        vec!["1.1"]
    );
}
