use super::{optional_task_text_body, retries_enabled_by_default};

#[test]
fn get_requests_are_retried_by_default() {
    assert!(retries_enabled_by_default("GET"));
}

#[test]
fn post_requests_are_not_retried_by_default() {
    assert!(!retries_enabled_by_default("POST"));
}

#[test]
fn optional_task_text_body_serializes_payload_when_present() {
    assert_eq!(
        optional_task_text_body("note", Some("done".to_string())),
        r#"{"note":"done"}"#
    );
    assert_eq!(
        optional_task_text_body("reason", Some("blocked".to_string())),
        r#"{"reason":"blocked"}"#
    );
}

#[test]
fn optional_task_text_body_uses_empty_object_when_absent() {
    assert_eq!(optional_task_text_body("note", None), "{}");
}
