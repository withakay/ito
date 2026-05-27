use super::*;

#[test]
fn parse_spec_id_preserves_value() {
    let parsed = parse_spec_id("cli-init").unwrap();
    assert_eq!(parsed.spec_id.as_str(), "cli-init");
}

#[test]
fn parse_spec_id_rejects_path_traversal_sequences() {
    let err = parse_spec_id("../secrets").expect_err("path traversal should fail");
    assert!(err.error.contains("Invalid spec ID format"));
}
