//! Domain-layer error types.

use std::io;

use thiserror::Error;

/// Result alias for domain-layer operations.
pub type DomainResult<T> = Result<T, DomainError>;

/// Error type used by domain ports and domain utilities.
#[derive(Debug, Error)]
pub enum DomainError {
    /// Filesystem or other IO failure.
    #[error("I/O failure while {context}: {source}")]
    Io {
        /// Short operation context.
        context: &'static str,
        /// Source error.
        #[source]
        source: io::Error,
    },

    /// Requested entity was not found.
    #[error("{entity} not found: {id}")]
    NotFound {
        /// Entity kind.
        entity: &'static str,
        /// Requested identifier.
        id: String,
    },

    /// Target was ambiguous and matched multiple entities.
    #[error("Ambiguous {entity} target '{input}'. Matches: {matches}")]
    AmbiguousTarget {
        /// Entity kind.
        entity: &'static str,
        /// User-provided target.
        input: String,
        /// Comma-separated matching candidates.
        matches: String,
    },
}

impl DomainError {
    /// Build an IO-flavored domain error with a static context string.
    pub fn io(context: &'static str, source: io::Error) -> Self {
        Self::Io { context, source }
    }

    /// Build a not-found error for an entity.
    pub fn not_found(entity: &'static str, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity,
            id: id.into(),
        }
    }

    /// Build an ambiguity error for an entity target.
    pub fn ambiguous_target(entity: &'static str, input: &str, matches: &[String]) -> Self {
        Self::AmbiguousTarget {
            entity,
            input: input.to_string(),
            matches: matches.join(", "),
        }
    }
}

#[cfg(test)]
mod tests {
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
}
