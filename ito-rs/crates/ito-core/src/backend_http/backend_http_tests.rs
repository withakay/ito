use super::{
    is_not_found_error, optional_task_text_body, parse_timestamp, request_retries_enabled,
    retries_enabled_by_default,
};
use ito_domain::errors::DomainError;

#[test]
fn get_requests_are_retried_by_default() {
    assert!(retries_enabled_by_default("GET"));
}

#[test]
fn post_requests_are_not_retried_by_default() {
    assert!(!retries_enabled_by_default("POST"));
    assert!(!retries_enabled_by_default("PUT"));
    assert!(!retries_enabled_by_default("PATCH"));
    assert!(!retries_enabled_by_default("DELETE"));
    assert!(!retries_enabled_by_default("HEAD"));
    assert!(!retries_enabled_by_default("OPTIONS"));
    assert!(!retries_enabled_by_default("TRACE"));
}

#[test]
fn audit_ingest_posts_can_opt_into_retries() {
    assert!(request_retries_enabled("POST", true));
    assert!(!request_retries_enabled("POST", false));
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

#[test]
fn parse_timestamp_returns_error_for_invalid_rfc3339() {
    assert!(parse_timestamp("not-a-timestamp").is_err());
}

#[test]
fn archived_task_fallback_only_treats_not_found_as_missing() {
    assert!(is_not_found_error(&DomainError::not_found("task", "1.1")));
    assert!(!is_not_found_error(&DomainError::io(
        "reading tasks",
        std::io::Error::other("boom"),
    )));
}
