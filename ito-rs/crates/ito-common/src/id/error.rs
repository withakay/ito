//! Identifier parse error type.

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error returned when parsing an Ito identifier fails.
pub struct IdParseError {
    /// Human-readable error message.
    pub error: String,

    /// Optional hint describing a common fix.
    pub hint: Option<String>,
}

impl IdParseError {
    /// Build a parse error with an optional remediation hint.
    pub(crate) fn new(error: impl Into<String>, hint: Option<impl Into<String>>) -> Self {
        Self {
            error: error.into(),
            hint: hint.map(|h| h.into()),
        }
    }
}
