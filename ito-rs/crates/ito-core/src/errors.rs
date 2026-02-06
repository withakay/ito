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

    /// Build a serde error.
    pub fn serde(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Serde {
            context: context.into(),
            message: message.into(),
        }
    }

    /// Build a sqlite error.
    pub fn sqlite(msg: impl Into<String>) -> Self {
        Self::Sqlite(msg.into())
    }
}
