//! Tests for checkbox task helpers.

use super::checkbox::{is_checkbox_task_id_token, split_checkbox_task_label};

#[test]
fn is_checkbox_task_id_token_accepts_valid_numeric_ids() {
    assert!(is_checkbox_task_id_token("1"));
    assert!(is_checkbox_task_id_token("42"));
    assert!(is_checkbox_task_id_token("1.1"));
    assert!(is_checkbox_task_id_token("1.2.3"));
    assert!(is_checkbox_task_id_token("10.20.30"));
}

#[test]
fn is_checkbox_task_id_token_rejects_invalid_formats() {
    // Empty string
    assert!(!is_checkbox_task_id_token(""));

    // Starts or ends with dot
    assert!(!is_checkbox_task_id_token(".1"));
    assert!(!is_checkbox_task_id_token("1."));

    // Double dots
    assert!(!is_checkbox_task_id_token("1..2"));

    // Non-numeric characters
    assert!(!is_checkbox_task_id_token("1.a"));
    assert!(!is_checkbox_task_id_token("a.1"));
    assert!(!is_checkbox_task_id_token("1-2"));
    assert!(!is_checkbox_task_id_token("Task 1.1"));

    // Contains spaces
    assert!(!is_checkbox_task_id_token("1 1"));
}

#[test]
fn split_checkbox_task_label_extracts_id_from_prefixed_text() {
    assert_eq!(
        split_checkbox_task_label("1.1 First task"),
        Some(("1.1", "First task"))
    );

    assert_eq!(
        split_checkbox_task_label("42 Do something"),
        Some(("42", "Do something"))
    );

    assert_eq!(
        split_checkbox_task_label("1.2.3 Complex task"),
        Some(("1.2.3", "Complex task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_colon_and_dot_suffixes() {
    assert_eq!(
        split_checkbox_task_label("1.1: First task"),
        Some(("1.1", "First task"))
    );

    assert_eq!(
        split_checkbox_task_label("42. Do something"),
        Some(("42", "Do something"))
    );

    assert_eq!(
        split_checkbox_task_label("1.2.3.: Complex task"),
        Some(("1.2.3", "Complex task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_tab_separation() {
    assert_eq!(
        split_checkbox_task_label("1.1\tFirst task"),
        Some(("1.1", "First task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_extra_whitespace() {
    assert_eq!(
        split_checkbox_task_label("  1.1  First task  "),
        Some(("1.1", "First task"))
    );

    assert_eq!(
        split_checkbox_task_label("1.1   Multiple spaces"),
        Some(("1.1", "Multiple spaces"))
    );
}

#[test]
fn split_checkbox_task_label_returns_none_for_invalid_input() {
    // No whitespace separator
    assert_eq!(split_checkbox_task_label("1.1First"), None);

    // Empty string
    assert_eq!(split_checkbox_task_label(""), None);

    // Non-numeric prefix
    assert_eq!(split_checkbox_task_label("Task 1.1"), None);
    assert_eq!(split_checkbox_task_label("abc def"), None);

    // Just whitespace
    assert_eq!(split_checkbox_task_label("   "), None);
}

#[test]
fn split_checkbox_task_label_handles_empty_task_name() {
    // ID with trailing whitespace only
    assert_eq!(
        split_checkbox_task_label("1.1 "),
        Some(("1.1", ""))
    );

    assert_eq!(
        split_checkbox_task_label("42:   "),
        Some(("42", ""))
    );
}

#[test]
fn is_checkbox_task_id_token_boundary_cases() {
    // Very long numeric ID
    assert!(is_checkbox_task_id_token("999999999"));
    assert!(is_checkbox_task_id_token("1.2.3.4.5.6.7.8.9.10"));

    // Single digit
    assert!(is_checkbox_task_id_token("0"));
    assert!(is_checkbox_task_id_token("9"));
}

#[test]
fn split_checkbox_task_label_preserves_internal_structure() {
    // Task name with numbers
    assert_eq!(
        split_checkbox_task_label("1 Task 2.3 update"),
        Some(("1", "Task 2.3 update"))
    );

    // Task name with special characters
    assert_eq!(
        split_checkbox_task_label("1.1 Update foo/bar.rs"),
        Some(("1.1", "Update foo/bar.rs"))
    );

    // Task name with colon
    assert_eq!(
        split_checkbox_task_label("1 Task: implement feature"),
        Some(("1", "Task: implement feature"))
    );
}