//! Helpers for constructing validation issues.
//!
//! Validation runs typically accumulate multiple issues. These helpers keep call
//! sites compact while producing consistent JSON-serializable issue shapes.

use super::{LEVEL_ERROR, LEVEL_INFO, LEVEL_WARNING, ValidationIssue, ValidationLevel};

/// Construct a [`ValidationIssue`] with a fixed `level`, `path`, and message.
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

/// Convenience constructor for an `ERROR` issue.
pub fn error(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_ERROR, path, message)
}

/// Convenience constructor for a `WARNING` issue.
pub fn warning(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_WARNING, path, message)
}

/// Convenience constructor for an `INFO` issue.
pub fn info(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_INFO, path, message)
}

/// Attach a 1-based line number to an existing issue.
pub fn with_line(mut i: ValidationIssue, line: u32) -> ValidationIssue {
    i.line = Some(line);
    i
}

/// Attach a 1-based line + column location to an existing issue.
pub fn with_loc(mut i: ValidationIssue, line: u32, column: u32) -> ValidationIssue {
    i.line = Some(line);
    i.column = Some(column);
    i
}

/// Attach structured metadata to an existing issue.
pub fn with_metadata(mut i: ValidationIssue, metadata: serde_json::Value) -> ValidationIssue {
    i.metadata = Some(metadata);
    i
}
