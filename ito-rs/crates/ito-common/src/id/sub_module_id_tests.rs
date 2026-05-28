use super::*;

#[test]
fn parse_sub_module_id_canonical_form() {
    let parsed = parse_sub_module_id("005.01").unwrap();
    assert_eq!(parsed.sub_module_id.as_str(), "005.01");
    assert_eq!(parsed.parent_module_id.as_str(), "005");
    assert_eq!(parsed.sub_num, "01");
    assert_eq!(parsed.sub_name, None);
}

#[test]
fn parse_sub_module_id_pads_both_parts() {
    let parsed = parse_sub_module_id("5.1").unwrap();
    assert_eq!(parsed.sub_module_id.as_str(), "005.01");
    assert_eq!(parsed.parent_module_id.as_str(), "005");
    assert_eq!(parsed.sub_num, "01");
}

#[test]
fn parse_sub_module_id_with_name_suffix() {
    let parsed = parse_sub_module_id("005.01_core-api").unwrap();
    assert_eq!(parsed.sub_module_id.as_str(), "005.01");
    assert_eq!(parsed.sub_name.as_deref(), Some("core-api"));
}

#[test]
fn parse_sub_module_id_lowercases_name() {
    let parsed = parse_sub_module_id("005.01_Core-API").unwrap();
    assert_eq!(parsed.sub_name.as_deref(), Some("core-api"));
}

#[test]
fn parse_sub_module_id_strips_extra_leading_zeros() {
    let parsed = parse_sub_module_id("005.001").unwrap();
    assert_eq!(parsed.sub_module_id.as_str(), "005.01");
    assert_eq!(parsed.sub_num, "01");
}

#[test]
fn parse_sub_module_id_rejects_empty() {
    let err = parse_sub_module_id("").unwrap_err();
    assert_eq!(err.error, "Sub-module ID cannot be empty");
}

#[test]
fn parse_sub_module_id_rejects_missing_dot() {
    let err = parse_sub_module_id("005-01").unwrap_err();
    assert!(err.error.contains("Invalid sub-module ID format"));
}

#[test]
fn parse_sub_module_id_rejects_module_overflow() {
    let err = parse_sub_module_id("1000.01").unwrap_err();
    assert!(err.error.contains("exceeds maximum (999)"));
}

#[test]
fn parse_sub_module_id_rejects_sub_overflow() {
    let err = parse_sub_module_id("005.100").unwrap_err();
    assert!(err.error.contains("exceeds maximum (99)"));
}

#[test]
fn parse_sub_module_id_rejects_non_digit_module() {
    let err = parse_sub_module_id("abc.01").unwrap_err();
    assert!(err.error.contains("Invalid sub-module ID format"));
}

#[test]
fn parse_sub_module_id_rejects_overlong_input() {
    let input = format!("005.01_{}", "a".repeat(300));
    let err = parse_sub_module_id(&input).expect_err("overlong sub-module id should fail");
    assert!(err.error.contains("too long"));
}

#[test]
fn sub_module_id_display() {
    let id = SubModuleId::new("005.01".to_string());
    assert_eq!(id.to_string(), "005.01");
}
