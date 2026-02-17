//! Tests for relational validation of task and wave dependencies.

use super::validate_relational;
use crate::tasks::{DiagnosticLevel, TaskItem, TaskKind, TaskStatus, WaveInfo};

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

fn wave(wave_num: u32, depends_on: &[u32], header_line_index: usize) -> WaveInfo {
    WaveInfo {
        wave: wave_num,
        depends_on: depends_on.to_vec(),
        header_line_index,
        depends_on_line_index: Some(header_line_index + 1),
    }
}

#[test]
fn validate_relational_accepts_valid_dependency_graph() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Complete, &[], 0),
        task("1.2", Some(1), TaskStatus::Pending, &["1.1"], 1),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(diagnostics.is_empty());
}

#[test]
fn validate_relational_detects_duplicate_task_ids() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Complete, &[], 0),
        task("1.1", Some(1), TaskStatus::Pending, &[], 5),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Duplicate task id"))
    );
}

#[test]
fn validate_relational_detects_missing_task_dependencies() {
    let tasks = vec![task("1.1", Some(1), TaskStatus::Pending, &["missing"], 0)];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Missing dependency: missing"))
    );
}

#[test]
fn validate_relational_detects_self_referencing_task() {
    let tasks = vec![task("1.1", Some(1), TaskStatus::Pending, &["1.1"], 0)];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("cannot depend on itself"))
    );
}

#[test]
fn validate_relational_detects_cross_wave_task_dependencies() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Complete, &[], 0),
        task("2.1", Some(2), TaskStatus::Pending, &["1.1"], 1),
    ];
    let waves = vec![wave(1, &[], 0), wave(2, &[], 1)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Cross-wave dependency"))
    );
}

#[test]
fn validate_relational_detects_dependency_on_shelved_task() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Shelved, &[], 0),
        task("1.2", Some(1), TaskStatus::Pending, &["1.1"], 1),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Dependency is shelved"))
    );
}

#[test]
fn validate_relational_allows_shelved_task_depending_on_shelved_task() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Shelved, &[], 0),
        task("1.2", Some(1), TaskStatus::Shelved, &["1.1"], 1),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    // Should not have "Dependency is shelved" error since both are shelved
    assert!(
        !diagnostics
            .iter()
            .any(|d| d.message.contains("Dependency is shelved"))
    );
}

#[test]
fn validate_relational_detects_task_dependency_cycle() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Pending, &["1.2"], 0),
        task("1.2", Some(1), TaskStatus::Pending, &["1.1"], 1),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Dependency cycle detected"))
    );
}

#[test]
fn validate_relational_detects_three_node_task_cycle() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Pending, &["1.2"], 0),
        task("1.2", Some(1), TaskStatus::Pending, &["1.3"], 1),
        task("1.3", Some(1), TaskStatus::Pending, &["1.1"], 2),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Dependency cycle detected"))
    );
    let cycle_diag = diagnostics
        .iter()
        .find(|d| d.message.contains("Dependency cycle"))
        .unwrap();
    assert!(cycle_diag.message.contains("1.1"));
    assert!(cycle_diag.message.contains("1.2"));
    assert!(cycle_diag.message.contains("1.3"));
}

#[test]
fn validate_relational_detects_wave_dependency_cycle() {
    let tasks = vec![];
    let waves = vec![wave(1, &[2], 0), wave(2, &[1], 1)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Wave dependency cycle detected"))
    );
}

#[test]
fn validate_relational_ignores_empty_and_checkpoint_dependencies() {
    let tasks = vec![task(
        "1.1",
        Some(1),
        TaskStatus::Pending,
        &["", "Checkpoint"],
        0,
    )];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    // Should not have errors about missing "" or "Checkpoint" dependencies
    assert!(diagnostics.is_empty());
}

#[test]
fn validate_relational_handles_tasks_without_wave() {
    // Checkpoint-style tasks have wave = None
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Complete, &[], 0),
        task("checkpoint", None, TaskStatus::Pending, &[], 1),
    ];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(diagnostics.is_empty());
}

#[test]
fn validate_relational_multiple_errors_for_same_task() {
    // Task with both missing dependency and self-reference
    let tasks = vec![task(
        "1.1",
        Some(1),
        TaskStatus::Pending,
        &["missing", "1.1"],
        0,
    )];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(diagnostics.len() >= 2);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("Missing dependency"))
    );
    assert!(
        diagnostics
            .iter()
            .any(|d| d.message.contains("cannot depend on itself"))
    );
}

#[test]
fn validate_relational_with_complex_valid_graph() {
    let tasks = vec![
        task("1.1", Some(1), TaskStatus::Complete, &[], 0),
        task("1.2", Some(1), TaskStatus::Complete, &["1.1"], 1),
        task("1.3", Some(1), TaskStatus::Pending, &["1.1", "1.2"], 2),
        task("2.1", Some(2), TaskStatus::Pending, &[], 3),
        task("2.2", Some(2), TaskStatus::Pending, &["2.1"], 4),
    ];
    let waves = vec![wave(1, &[], 0), wave(2, &[1], 2)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(diagnostics.is_empty());
}

#[test]
fn validate_relational_reports_line_numbers() {
    let tasks = vec![task("1.1", Some(1), TaskStatus::Pending, &["missing"], 42)];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    let missing_dep_diag = diagnostics
        .iter()
        .find(|d| d.message.contains("Missing dependency"))
        .unwrap();
    assert_eq!(missing_dep_diag.line, Some(43)); // 0-indexed + 1
    assert_eq!(missing_dep_diag.task_id, Some("1.1".to_string()));
}

#[test]
fn validate_relational_marks_errors_as_error_level() {
    let tasks = vec![task("1.1", Some(1), TaskStatus::Pending, &["1.1"], 0)];
    let waves = vec![wave(1, &[], 0)];

    let diagnostics = validate_relational(&tasks, &waves);
    assert!(
        diagnostics
            .iter()
            .all(|d| d.level == DiagnosticLevel::Error)
    );
}
