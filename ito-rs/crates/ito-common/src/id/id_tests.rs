use super::*;

#[test]
fn looks_like_change_id_requires_digits_hyphen_and_underscore() {
    assert!(looks_like_change_id("001-02_hello"));
    assert!(!looks_like_change_id("-02_hello"));
    assert!(!looks_like_change_id("001_hello"));
    assert!(!looks_like_change_id("001-02hello"));
    assert!(!looks_like_change_id("abc-02_hello"));
}

#[test]
fn looks_like_change_id_recognizes_sub_module_format() {
    assert!(looks_like_change_id("005.01-03_my-change"));
    assert!(looks_like_change_id("5.1-2_foo"));
}

#[test]
fn looks_like_module_id_is_digit_prefixed() {
    assert!(looks_like_module_id("001"));
    assert!(looks_like_module_id("001_demo"));
    assert!(looks_like_module_id(" 001_demo "));
    assert!(!looks_like_module_id(""));
    assert!(!looks_like_module_id("demo"));
    assert!(!looks_like_module_id("_001_demo"));
}

#[test]
fn classify_id_module_change_id() {
    assert_eq!(classify_id("005-01_my-change"), ItoIdKind::ModuleChangeId);
    assert_eq!(classify_id("1-2_foo"), ItoIdKind::ModuleChangeId);
}

#[test]
fn classify_id_sub_module_change_id() {
    assert_eq!(
        classify_id("005.01-03_my-change"),
        ItoIdKind::SubModuleChangeId
    );
    assert_eq!(classify_id("5.1-2_foo"), ItoIdKind::SubModuleChangeId);
}

#[test]
fn classify_id_sub_module_id() {
    assert_eq!(classify_id("005.01"), ItoIdKind::SubModuleId);
    assert_eq!(classify_id("005.01_core-api"), ItoIdKind::SubModuleId);
}

#[test]
fn classify_id_module_id() {
    assert_eq!(classify_id("005"), ItoIdKind::ModuleId);
    assert_eq!(classify_id("005_dev-tooling"), ItoIdKind::ModuleId);
    assert_eq!(classify_id("1"), ItoIdKind::ModuleId);
}

#[test]
fn classify_id_hyphen_without_underscore_is_module_change_id() {
    // "005-01" has a hyphen in the prefix, so classify it structurally as a change id.
    // Full validation is left to parse_change_id.
    assert_eq!(classify_id("005-01"), ItoIdKind::ModuleChangeId);
}
