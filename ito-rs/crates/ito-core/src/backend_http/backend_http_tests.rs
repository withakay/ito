use super::retries_enabled_by_default;

#[test]
fn get_requests_are_retried_by_default() {
    assert!(retries_enabled_by_default("GET"));
}

#[test]
fn post_requests_are_not_retried_by_default() {
    assert!(!retries_enabled_by_default("POST"));
}
