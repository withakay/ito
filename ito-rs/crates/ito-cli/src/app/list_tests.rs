use super::{format_relative_time, format_task_status, handle_list, parse_sort_order};
use crate::runtime::Runtime;
use chrono::{Duration, Utc};

#[test]
fn parse_sort_order_supports_separate_and_equals_forms() {
    let args = vec!["--sort".to_string(), "name".to_string()];
    assert_eq!(parse_sort_order(&args), Some("name"));

    let args = vec!["--sort=recent".to_string()];
    assert_eq!(parse_sort_order(&args), Some("recent"));
}

#[test]
fn format_task_status_handles_various_states() {
    let make_summary = |completed, shelved, in_progress, pending, total, work_status: &str| {
        ito_core::list::ChangeListSummary {
            name: "test".to_string(),
            status: "in-progress".to_string(),
            work_status: work_status.to_string(),
            completed: false,
            completed_tasks: completed,
            shelved_tasks: shelved,
            in_progress_tasks: in_progress,
            pending_tasks: pending,
            total_tasks: total,
            last_modified: Utc::now(),
        }
    };

    // No tasks
    let s = make_summary(0, 0, 0, 0, 0, "ready");
    assert_eq!(format_task_status(&s), "No tasks");

    // All complete
    let s = make_summary(3, 0, 0, 0, 3, "complete");
    assert!(format_task_status(&s).contains("Complete"));
    assert!(format_task_status(&s).contains("3c"));

    // Paused (complete + shelved = total, shelved > 0)
    let s = make_summary(2, 1, 0, 0, 3, "paused");
    assert!(format_task_status(&s).contains("Paused"));
    assert!(format_task_status(&s).contains("2c"));
    assert!(format_task_status(&s).contains("1s"));

    // In progress
    let s = make_summary(1, 0, 1, 1, 3, "in-progress");
    assert!(format_task_status(&s).contains("Active"));
    assert!(format_task_status(&s).contains("1i"));

    // Ready (pending work, nothing in progress)
    let s = make_summary(1, 0, 0, 2, 3, "ready");
    let status = format_task_status(&s);
    assert!(status.contains("1c"));
    assert!(status.contains("2p"));
}

#[test]
fn progress_filter_flags_are_mutually_exclusive() {
    let rt = Runtime::new();
    let args = vec!["--pending".to_string(), "--partial".to_string()];
    let err = handle_list(&rt, &args).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Flags --completed, --partial, and --pending are mutually exclusive."
    );
}

#[test]
fn format_relative_time_covers_major_buckets() {
    assert_eq!(
        format_relative_time(Utc::now() + Duration::seconds(1)),
        "just now"
    );

    let then = Utc::now() - Duration::minutes(2);
    assert_eq!(format_relative_time(then), "2m ago");

    let then = Utc::now() - Duration::hours(2);
    assert_eq!(format_relative_time(then), "2h ago");

    let then = Utc::now() - Duration::days(2);
    assert_eq!(format_relative_time(then), "2d ago");

    let then = Utc::now() - Duration::days(40);
    assert_eq!(
        format_relative_time(then),
        then.format("%-m/%-d/%Y").to_string()
    );
}
