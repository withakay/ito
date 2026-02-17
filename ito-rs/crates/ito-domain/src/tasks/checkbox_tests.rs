//! Tests for checkbox format helper functions.

use super::checkbox::{is_checkbox_task_id_token, split_checkbox_task_label};

#[test]
fn is_checkbox_task_id_token_accepts_valid_formats() {
    assert!(is_checkbox_task_id_token("1"));
    assert!(is_checkbox_task_id_token("42"));
    assert!(is_checkbox_task_id_token("1.1"));
    assert!(is_checkbox_task_id_token("2.3"));
    assert!(is_checkbox_task_id_token("1.2.3"));
    assert!(is_checkbox_task_id_token("10.20.30"));
    assert!(is_checkbox_task_id_token("999"));
}

#[test]
fn is_checkbox_task_id_token_rejects_invalid_formats() {
    // Empty string
    assert!(!is_checkbox_task_id_token(""));

    // Non-numeric characters
    assert!(!is_checkbox_task_id_token("a"));
    assert!(!is_checkbox_task_id_token("1a"));
    assert!(!is_checkbox_task_id_token("a1"));
    assert!(!is_checkbox_task_id_token("1.a"));
    assert!(!is_checkbox_task_id_token("1-1"));

    // Leading or trailing dots
    assert!(!is_checkbox_task_id_token(".1"));
    assert!(!is_checkbox_task_id_token("1."));
    assert!(!is_checkbox_task_id_token(".1."));

    // Consecutive dots
    assert!(!is_checkbox_task_id_token("1..2"));
    assert!(!is_checkbox_task_id_token("1...2"));

    // Whitespace
    assert!(!is_checkbox_task_id_token("1 1"));
    assert!(!is_checkbox_task_id_token(" 1"));
    assert!(!is_checkbox_task_id_token("1 "));

    // Special characters
    assert!(!is_checkbox_task_id_token("1,1"));
    assert!(!is_checkbox_task_id_token("1:1"));
    assert!(!is_checkbox_task_id_token("1;1"));
}

#[test]
fn split_checkbox_task_label_extracts_id_and_rest() {
    assert_eq!(
        split_checkbox_task_label("1.1 First task"),
        Some(("1.1", "First task"))
    );
    assert_eq!(
        split_checkbox_task_label("42 Do something"),
        Some(("42", "Do something"))
    );
    assert_eq!(
        split_checkbox_task_label("1.2.3 Complex ID"),
        Some(("1.2.3", "Complex ID"))
    );
}

#[test]
fn split_checkbox_task_label_handles_colon_suffix() {
    assert_eq!(
        split_checkbox_task_label("1.1: First task"),
        Some(("1.1", "First task"))
    );
    assert_eq!(
        split_checkbox_task_label("2: Second task"),
        Some(("2", "Second task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_dot_suffix() {
    assert_eq!(
        split_checkbox_task_label("1.1. First task"),
        Some(("1.1", "First task"))
    );
    assert_eq!(
        split_checkbox_task_label("2. Second task"),
        Some(("2", "Second task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_leading_whitespace() {
    assert_eq!(
        split_checkbox_task_label("  1.1 First task"),
        Some(("1.1", "First task"))
    );
    assert_eq!(
        split_checkbox_task_label("\t2 Second task"),
        Some(("2", "Second task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_tab_separator() {
    assert_eq!(
        split_checkbox_task_label("1.1\tFirst task"),
        Some(("1.1", "First task"))
    );
}

#[test]
fn split_checkbox_task_label_returns_none_for_invalid_inputs() {
    // Empty string
    assert_eq!(split_checkbox_task_label(""), None);

    // No whitespace after ID
    assert_eq!(split_checkbox_task_label("1.1"), None);

    // Invalid ID format
    assert_eq!(split_checkbox_task_label("abc First task"), None);
    assert_eq!(split_checkbox_task_label("1-1 Invalid"), None);

    // Only whitespace
    assert_eq!(split_checkbox_task_label("   "), None);
}

#[test]
fn split_checkbox_task_label_preserves_trailing_whitespace_in_rest() {
    assert_eq!(
        split_checkbox_task_label("1 Task with trailing  "),
        Some(("1", "Task with trailing"))
    );
}

#[test]
fn split_checkbox_task_label_handles_unicode_in_task_name() {
    assert_eq!(
        split_checkbox_task_label("1 测试任务"),
        Some(("1", "测试任务"))
    );
    assert_eq!(
        split_checkbox_task_label("1.1 Tâche française"),
        Some(("1.1", "Tâche française"))
    );
    assert_eq!(
        split_checkbox_task_label("2 🚀 Rocket task"),
        Some(("2", "🚀 Rocket task"))
    );
}

#[test]
fn split_checkbox_task_label_handles_multiple_spaces() {
    assert_eq!(
        split_checkbox_task_label("1.1    Multiple    spaces"),
        Some(("1.1", "Multiple    spaces"))
    );
}

#[test]
fn is_checkbox_task_id_token_handles_large_numbers() {
    assert!(is_checkbox_task_id_token("999999"));
    assert!(is_checkbox_task_id_token("100.200.300"));
}

#[test]
fn split_checkbox_task_label_edge_case_single_digit_with_many_dots() {
    // This should be valid according to the token rules
    assert_eq!(
        split_checkbox_task_label("1.2.3.4.5 Deep nesting"),
        Some(("1.2.3.4.5", "Deep nesting"))
    );
}
