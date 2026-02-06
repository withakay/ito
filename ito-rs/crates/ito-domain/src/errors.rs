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
