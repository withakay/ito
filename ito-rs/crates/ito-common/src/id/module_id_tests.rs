use super::*;

#[test]
fn parse_module_id_pads_and_lowercases_name() {
    let parsed = parse_module_id("1_Foo-Bar").unwrap();
    assert_eq!(parsed.module_id.as_str(), "001");
    assert_eq!(parsed.module_name.as_deref(), Some("foo-bar"));
}

#[test]
fn parse_module_id_rejects_overflow() {
    let err = parse_module_id("1000").unwrap_err();
    assert_eq!(err.error, "Module ID 1000 exceeds maximum (999)");
}

#[test]
fn parse_module_id_rejects_overlong_input() {
    let input = format!("001_{}", "a".repeat(300));
    let err = parse_module_id(&input).expect_err("overlong module id should fail");
    assert!(err.error.contains("too long"));
}
