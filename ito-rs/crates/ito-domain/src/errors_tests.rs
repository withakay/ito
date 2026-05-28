use super::*;

#[test]
fn io_constructor_preserves_context_and_source() {
    let source = io::Error::new(io::ErrorKind::PermissionDenied, "no access");
    let error = DomainError::io("reading tasks", source);

    match error {
        DomainError::Io { context, source } => {
            assert_eq!(context, "reading tasks");
            assert_eq!(source.kind(), io::ErrorKind::PermissionDenied);
            assert_eq!(source.to_string(), "no access");
        }
        other => panic!("expected io variant, got {other:?}"),
    }
}

#[test]
fn not_found_constructor_formats_display_message() {
    let error = DomainError::not_found("module", "123_core");

    match &error {
        DomainError::NotFound { entity, id } => {
            assert_eq!(*entity, "module");
            assert_eq!(id, "123_core");
        }
        other => panic!("expected not found variant, got {other:?}"),
    }

    assert_eq!(error.to_string(), "module not found: 123_core");
}

#[test]
fn ambiguous_target_joins_candidates_in_display_message() {
    let matches = vec!["001-01_alpha".to_string(), "001-02_alpha-fix".to_string()];
    let error = DomainError::ambiguous_target("change", "alpha", &matches);

    match &error {
        DomainError::AmbiguousTarget {
            entity,
            input,
            matches,
        } => {
            assert_eq!(*entity, "change");
            assert_eq!(input, "alpha");
            assert_eq!(matches, "001-01_alpha, 001-02_alpha-fix");
        }
        other => panic!("expected ambiguous target variant, got {other:?}"),
    }

    assert_eq!(
        error.to_string(),
        "Ambiguous change target 'alpha'. Matches: 001-01_alpha, 001-02_alpha-fix"
    );
}
