//! Core-layer error types.
//!
//! [`CoreError`] is the canonical error type for `ito-core`. All public
//! functions in this crate return [`CoreResult<T>`] rather than adapter-level
//! error types. Adapter layers (CLI, web) convert `CoreError` into their own
//! presentation types (e.g., miette `Report` for rich terminal output).

use std::io;

use ito_domain::errors::DomainError;
use thiserror::Error;

/// Result alias for core-layer operations.
pub type CoreResult<T> = Result<T, CoreError>;

/// Canonical error type for the core orchestration layer.
///
/// Variants cover the major failure categories encountered by core use-cases.
/// None of the variants carry presentation logic — that belongs in the adapter.
#[derive(Debug, Error)]
pub enum CoreError {
    /// An error propagated from the domain layer.
    #[error(transparent)]
    Domain(#[from] DomainError),

    /// Filesystem or other I/O failure.
    #[error("{context}: {source}")]
    Io {
        /// Short description of the operation that failed.
        context: String,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Input validation failure (bad arguments, constraint violations).
    #[error("{0}")]
    Validation(String),

    /// Parse failure (duration strings, JSON, YAML, etc.).
    #[error("{0}")]
    Parse(String),

    /// Process execution failure (git, shell commands).
    #[error("{0}")]
    Process(String),

    /// SQLite operation failure.
    #[error("sqlite error: {0}")]
    Sqlite(String),

    /// An expected asset or resource was not found.
    #[error("{0}")]
    NotFound(String),

    /// Serialization or deserialization failure.
    #[error("{context}: {message}")]
    Serde {
        /// Short description of the operation.
        context: String,
        /// Error detail.
        message: String,
    },
}

impl CoreError {
    /// Build an I/O error with context.
    pub fn io(context: impl Into<String>, source: io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }

    /// Build a validation error.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Build a parse error.
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }

    /// Build a process error.
    pub fn process(msg: impl Into<String>) -> Self {
        Self::Process(msg.into())
    }

    /// Build a not-found error.
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a `CoreError::Serde` containing a context and a message describing a
    /// serialization or deserialization failure.
    ///
    /// # Examples
    ///
    /// ```
    /// let err = CoreError::serde("load config", "missing field `name`");
    /// match err {
    ///     CoreError::Serde { context, message } => {
    ///         assert_eq!(context, "load config");
    ///         assert_eq!(message, "missing field `name`");
    ///     }
    ///     _ => panic!("expected Serde variant"),
    /// }
    /// ```
    pub fn serde(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Serde {
            context: context.into(),
            message: message.into(),
        }
    }

    /// Wraps a human-readable SQLite error message into a `CoreError::Sqlite`.
    ///
    /// Returns a `CoreError::Sqlite` containing the provided message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::errors::CoreError;
    ///
    /// let err = CoreError::sqlite("database locked");
    /// let CoreError::Sqlite(msg) = err else {
    ///     panic!("expected Sqlite variant");
    /// };
    /// assert_eq!(msg, "database locked");
    /// ```
    pub fn sqlite(msg: impl Into<String>) -> Self {
        Self::Sqlite(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that each `CoreError` helper constructor produces the expected enum variant and contains the correct data.
    ///
    /// Constructs every public `CoreError` variant via its respective helper (e.g., `io`, `validation`, `parse`, `process`,
    /// `not_found`, `serde`, `sqlite`) and asserts both the variant and the values carried by that variant.
    ///
    /// # Examples
    ///
    ///
    #[test]
    fn core_error_helpers_construct_expected_variants() {
        let io_err = CoreError::io("read config", io::Error::other("boom"));
        let CoreError::Io { context, source } = io_err else {
            panic!("expected io variant");
        };
        assert_eq!(context, "read config");
        assert_eq!(source.to_string(), "boom");

        let validation_err = CoreError::validation("bad");
        let CoreError::Validation(validation_msg) = validation_err else {
            panic!("expected validation variant");
        };
        assert_eq!(validation_msg, "bad");

        let parse_err = CoreError::parse("bad");
        let CoreError::Parse(parse_msg) = parse_err else {
            panic!("expected parse variant");
        };
        assert_eq!(parse_msg, "bad");

        let process_err = CoreError::process("bad");
        let CoreError::Process(process_msg) = process_err else {
            panic!("expected process variant");
        };
        assert_eq!(process_msg, "bad");

        let not_found_err = CoreError::not_found("bad");
        let CoreError::NotFound(not_found_msg) = not_found_err else {
            panic!("expected not-found variant");
        };
        assert_eq!(not_found_msg, "bad");

        let serde_err = CoreError::serde("load", "bad");
        let CoreError::Serde { context, message } = serde_err else {
            panic!("expected serde variant");
        };
        assert_eq!(context, "load");
        assert_eq!(message, "bad");

        let sqlite_err = CoreError::sqlite("bad");
        let CoreError::Sqlite(sqlite_msg) = sqlite_err else {
            panic!("expected sqlite variant");
        };
        assert_eq!(sqlite_msg, "bad");
    }
}
