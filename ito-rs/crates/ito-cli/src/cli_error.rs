use std::fmt;

#[derive(Debug, Clone)]
pub struct CliError {
    message: String,
    silent: bool,
}

impl CliError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            silent: false,
        }
    }

    pub fn silent() -> Self {
        Self {
            message: String::new(),
            silent: true,
        }
    }

    pub fn is_silent(&self) -> bool {
        self.silent
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}

pub type CliResult<T = ()> = Result<T, CliError>;

pub fn fail<T>(message: impl Into<String>) -> CliResult<T> {
    Err(CliError::msg(message))
}

pub fn silent_fail<T>() -> CliResult<T> {
    Err(CliError::silent())
}

pub fn to_cli_error<E: fmt::Display>(e: E) -> CliError {
    CliError::msg(e.to_string())
}
