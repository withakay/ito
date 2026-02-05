#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdParseError {
    pub error: String,
    pub hint: Option<String>,
}

impl IdParseError {
    pub(crate) fn new(error: impl Into<String>, hint: Option<impl Into<String>>) -> Self {
        Self {
            error: error.into(),
            hint: hint.map(|h| h.into()),
        }
    }
}
