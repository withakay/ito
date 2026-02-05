use chrono::TimeZone;
use ito_domain::tasks;

#[test]
fn enhanced_template_parses_and_has_checkpoint_warning() {
    let now = chrono::Local
        .with_ymd_and_hms(2026, 1, 28, 0, 0, 0)
        .unwrap();
    let md = tasks::enhanced_tasks_template("test-change", now);
    let parsed = tasks::parse_tasks_tracking_file(&md);
    assert_eq!(parsed.format, tasks::TasksFormat::Enhanced);
    assert_eq!(parsed.tasks.len(), 2);
    let (ready, blocked) = tasks::compute_ready_and_blocked(&parsed);
    assert!(ready.iter().any(|t| t.id == "1.1"));
    assert!(blocked.iter().any(|(t, _)| t.id == "Checkpoint"));
}

#[test]
fn update_enhanced_task_status_inserts_or_replaces_status_line() {
    let md = "## Wave 1\n\n### Task 1.1: Do it\n- **Dependencies**: None\n\n## Wave 2\n";
    let now = chrono::Local
        .with_ymd_and_hms(2026, 1, 28, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);
    assert!(out.contains("- **Status**: [x] complete"));

    let out2 = tasks::update_enhanced_task_status(&out, "1.1", tasks::TaskStatus::InProgress, now);
    assert!(out2.contains("- **Status**: [ ] in-progress"));
}
