use super::{LEVEL_ERROR, LEVEL_INFO, LEVEL_WARNING, ValidationIssue, ValidationLevel};

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

pub fn error(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_ERROR, path, message)
}

pub fn warning(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_WARNING, path, message)
}

pub fn info(path: impl AsRef<str>, message: impl Into<String>) -> ValidationIssue {
    issue(LEVEL_INFO, path, message)
}

pub fn with_line(mut i: ValidationIssue, line: u32) -> ValidationIssue {
    i.line = Some(line);
    i
}

pub fn with_loc(mut i: ValidationIssue, line: u32, column: u32) -> ValidationIssue {
    i.line = Some(line);
    i.column = Some(column);
    i
}

pub fn with_metadata(mut i: ValidationIssue, metadata: serde_json::Value) -> ValidationIssue {
    i.metadata = Some(metadata);
    i
}
