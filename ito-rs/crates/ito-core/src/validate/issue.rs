//! Helpers for constructing validation issues.
//!
//! This module provides a fluent API for creating and enriching `ValidationIssue` instances.
//! It is the primary way to generate issues during validation logic.
//!
//! # Usage
//!
//! ```no_run
//! use ito_core::validate::{error, warning, with_loc};
//!
//! let err = error("path/to/file", "Something went wrong");
//! let warn = with_loc(warning("path/to/file", "Check this"), 10, 5);
//! ```

use super::{LEVEL_ERROR, LEVEL_INFO, LEVEL_WARNING, ValidationIssue, ValidationLevel};

/// Construct a [`ValidationIssue`] with a fixed `level`, `path`, and message.
///
/// This is the low-level constructor. Prefer using [`error`], [`warning`], or [`info`]
/// for better readability.
pub fn issue(
    level: ValidationLevel,
    path: impl AsRef<str>,
    message: impl Into<String>,
) -> ValidationIssue {
    ValidationIssue {
        level: level.to_string(),
        path: path.as_ref().to_string(),
        message: message.into(),
        line: None,
        column: None,
        metadata: None,
    }
}

/// Creates an `ERROR` level issue.
///
/// Use this for validation failures that must prevent the operation from succeeding.
pub fn error(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_ERROR, path, message)
}

/// Creates a `WARNING` level issue.
///
/// Use this for potential problems that should be fixed but do not strictly prevent
/// the operation from succeeding (unless strict mode is enabled).
pub fn warning(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_WARNING, path, message)
}

/// Creates an `INFO` level issue.
///
/// Use this for informational messages, successful checks, or context that helps
/// the user understand the validation state.
pub fn info(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_INFO, path, message)
}

/// Attach a 1-based line number to an existing issue.
///
/// Use this when the issue can be pinpointed to a specific line.
pub fn with_line(mut i: ValidationIssue, line: u32) -> ValidationIssue {
    i.line = Some(line);
    i
}

/// Attach a 1-based line + column location to an existing issue.
///
/// Use this when precise location information is available.
pub fn with_loc(mut i: ValidationIssue, line: u32, column: u32) -> ValidationIssue {
    i.line = Some(line);
    i.column = Some(column);
    i
}

/// Attach structured metadata to an existing issue.
///
/// Use this to attach extra JSON-serializable context (e.g., "expected" vs "actual" values)
/// that can be used by machine-readable output formats.
pub fn with_metadata(mut i: ValidationIssue, metadata: serde_json::Value) -> ValidationIssue {
    i.metadata = Some(metadata);
    i
}
