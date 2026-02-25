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

use super::{
    format_specs::FormatSpecRef, ValidationIssue, ValidationLevel, LEVEL_ERROR, LEVEL_INFO,
    LEVEL_WARNING,
};

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

/// Attach a stable validator id and spec path reference.
pub(crate) fn with_format_spec(mut i: ValidationIssue, spec: FormatSpecRef) -> ValidationIssue {
    let mut obj = match i.metadata.take() {
        Some(serde_json::Value::Object(map)) => map,
        Some(other) => {
            let mut map = serde_json::Map::new();
            map.insert("original_metadata".to_string(), other);
            map
        }
        None => serde_json::Map::new(),
    };
    obj.insert(
        "validator_id".to_string(),
        serde_json::Value::String(spec.validator_id.to_string()),
    );
    obj.insert(
        "spec_path".to_string(),
        serde_json::Value::String(spec.spec_path.to_string()),
    );
    i.metadata = Some(serde_json::Value::Object(obj));

    if !i.message.contains(spec.validator_id) {
        i.message = format!(
            "{} (validator: {})",
            i.message.trim_end(),
            spec.validator_id
        );
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_set_expected_fields() {
        let err = error("spec.md", "missing requirement");
        let warn = warning("spec.md", "brief purpose");
        let info_issue = info("spec.md", "looks good");

        assert_eq!(err.level, LEVEL_ERROR);
        assert_eq!(err.path, "spec.md");
        assert_eq!(err.message, "missing requirement");
        assert_eq!(err.line, None);
        assert_eq!(err.column, None);
        assert_eq!(err.metadata, None);

        assert_eq!(warn.level, LEVEL_WARNING);
        assert_eq!(info_issue.level, LEVEL_INFO);
    }

    #[test]
    fn location_helpers_set_line_and_column() {
        let base = issue(LEVEL_WARNING, "tasks.md", "task warning");

        let with_line_only = with_line(base.clone(), 8);
        assert_eq!(with_line_only.line, Some(8));
        assert_eq!(with_line_only.column, None);

        let with_both = with_loc(base, 11, 3);
        assert_eq!(with_both.line, Some(11));
        assert_eq!(with_both.column, Some(3));
    }

    #[test]
    fn metadata_helper_attaches_json_context() {
        let base = issue(LEVEL_ERROR, "config.json", "invalid value");
        let metadata = serde_json::json!({ "expected": "string", "actual": 42 });

        let enriched = with_metadata(base, metadata.clone());

        assert_eq!(enriched.metadata, Some(metadata));
    }

    #[test]
    fn format_spec_preserves_non_object_metadata() {
        let base = with_metadata(
            error("tasks.md", "bad"),
            serde_json::Value::String("preexisting".to_string()),
        );
        let out = with_format_spec(base, super::super::format_specs::TASKS_TRACKING_V1);

        let Some(meta) = out.metadata.as_ref().and_then(|m| m.as_object()) else {
            panic!("expected metadata object");
        };
        assert_eq!(
            meta.get("original_metadata").and_then(|v| v.as_str()),
            Some("preexisting")
        );
        assert_eq!(
            meta.get("validator_id").and_then(|v| v.as_str()),
            Some("ito.tasks-tracking.v1")
        );
    }

    #[test]
    fn format_spec_is_idempotent_for_message_suffix() {
        let base = error("specs", "no deltas");
        let out1 = with_format_spec(base, super::super::format_specs::DELTA_SPECS_V1);
        let out2 = with_format_spec(out1.clone(), super::super::format_specs::DELTA_SPECS_V1);
        assert_eq!(out1.message, out2.message);
        assert!(out2.message.contains("ito.delta-specs.v1"));
    }
}
