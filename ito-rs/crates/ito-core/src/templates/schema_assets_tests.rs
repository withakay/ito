use super::{is_safe_relative_path, is_safe_schema_name};

#[test]
fn safe_relative_path_validation_blocks_traversal_and_absolute_paths() {
    assert!(is_safe_relative_path("proposal.md"));
    assert!(is_safe_relative_path("nested/template.md"));

    assert!(!is_safe_relative_path(""));
    assert!(!is_safe_relative_path("../escape.md"));
    assert!(!is_safe_relative_path("./relative.md"));
    assert!(!is_safe_relative_path("/abs/path.md"));
    assert!(!is_safe_relative_path("nested\\windows.md"));
}

#[test]
fn safe_schema_name_rejects_dot_segments_and_periods() {
    assert!(is_safe_schema_name("spec-driven"));

    assert!(!is_safe_schema_name("../spec-driven"));
    assert!(!is_safe_schema_name("spec.driven"));
    assert!(!is_safe_schema_name(""));
}
