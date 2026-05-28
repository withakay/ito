use super::*;

#[test]
fn parse_change_id_pads_both_parts() {
    let parsed = parse_change_id("1-2_Bar").unwrap();
    assert_eq!(parsed.canonical.as_str(), "001-02_bar");
    assert_eq!(parsed.module_id.as_str(), "001");
    assert_eq!(parsed.change_num, "02");
    assert_eq!(parsed.name, "bar");
    assert_eq!(parsed.sub_module_id, None);
}

#[test]
fn parse_change_id_supports_extra_leading_zeros_for_change_num() {
    let parsed = parse_change_id("1-00003_bar").unwrap();
    assert_eq!(parsed.canonical.as_str(), "001-03_bar");
    assert_eq!(parsed.sub_module_id, None);
}

#[test]
fn parse_change_id_allows_three_digit_change_numbers() {
    let parsed = parse_change_id("1-100_Bar").unwrap();
    assert_eq!(parsed.canonical.as_str(), "001-100_bar");
    assert_eq!(parsed.change_num, "100");
}

#[test]
fn parse_change_id_normalizes_excessive_padding_for_large_change_numbers() {
    let parsed = parse_change_id("1-000100_bar").unwrap();
    assert_eq!(parsed.canonical.as_str(), "001-100_bar");
    assert_eq!(parsed.change_num, "100");
}

#[test]
fn parse_change_id_allows_large_change_numbers() {
    let parsed = parse_change_id("1-1234_example").unwrap();
    assert_eq!(parsed.canonical.as_str(), "001-1234_example");
    assert_eq!(parsed.change_num, "1234");
}

#[test]
fn parse_change_id_missing_name_has_specific_error() {
    let err = parse_change_id("1-2").unwrap_err();
    assert_eq!(err.error, "Change ID missing name: \"1-2\"");
}

#[test]
fn parse_change_id_uses_specific_hint_for_wrong_separator() {
    let err = parse_change_id("001_02_name").unwrap_err();
    assert_eq!(err.error, "Invalid change ID format: \"001_02_name\"");
    assert_eq!(
        err.hint.as_deref(),
        Some(
            "Change IDs use \"-\" between module and change number (e.g., \"001-02_name\" not \"001_02_name\")"
        )
    );
}

#[test]
fn parse_change_id_rejects_overlong_input() {
    let input = format!("001-01_{}", "a".repeat(300));
    let err = parse_change_id(&input).expect_err("overlong change id should fail");
    assert!(err.error.contains("too long"));
}

#[test]
fn parse_change_id_sub_module_format_canonical() {
    let parsed = parse_change_id("005.01-03_my-change").unwrap();
    assert_eq!(parsed.canonical.as_str(), "005.01-03_my-change");
    assert_eq!(parsed.module_id.as_str(), "005");
    assert_eq!(parsed.change_num, "03");
    assert_eq!(parsed.name, "my-change");
    let sub_id = parsed.sub_module_id.as_ref().unwrap();
    assert_eq!(sub_id.as_str(), "005.01");
}

#[test]
fn parse_change_id_sub_module_format_pads_all_parts() {
    let parsed = parse_change_id("5.1-3_foo").unwrap();
    assert_eq!(parsed.canonical.as_str(), "005.01-03_foo");
    assert_eq!(parsed.module_id.as_str(), "005");
    let sub_id = parsed.sub_module_id.as_ref().unwrap();
    assert_eq!(sub_id.as_str(), "005.01");
    assert_eq!(parsed.change_num, "03");
}

#[test]
fn parse_change_id_sub_module_format_lowercases_name() {
    let parsed = parse_change_id("005.01-03_My-Change").unwrap();
    assert_eq!(parsed.name, "my-change");
    assert_eq!(parsed.canonical.as_str(), "005.01-03_my-change");
}

#[test]
fn parse_change_id_sub_module_rejects_sub_overflow() {
    let err = parse_change_id("005.100-01_foo").unwrap_err();
    assert!(err.error.contains("exceeds maximum (99)"));
}

#[test]
fn parse_change_id_sub_module_rejects_module_overflow() {
    let err = parse_change_id("1000.01-01_foo").unwrap_err();
    assert!(err.error.contains("exceeds maximum (999)"));
}

#[test]
fn parse_change_id_sub_module_missing_name_is_error() {
    let err = parse_change_id("005.01-03").unwrap_err();
    assert!(err.error.contains("Invalid change ID format") || err.error.contains("missing name"));
}
