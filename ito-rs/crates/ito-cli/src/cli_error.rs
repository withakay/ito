use std::fmt;

#[derive(Debug, Clone)]
pub struct CliError {
    message: String,
    silent: bool,
    exit_code: i32,
}

impl CliError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            silent: false,
            exit_code: 1,
        }
    }

    pub fn silent() -> Self {
        Self {
            message: String::new(),
            silent: true,
            exit_code: 1,
        }
    }

    /// Construct a `CliError` with a specific exit code.
    ///
    /// Used by `ito validate repo` to honour the documented exit codes:
    /// `1` for validation failures and `2` for usage errors / unloadable
    /// configuration.
    pub fn with_code(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            silent: false,
            exit_code,
        }
    }

    /// Construct a silent `CliError` with a specific exit code.
    pub fn silent_with_code(exit_code: i32) -> Self {
        Self {
            message: String::new(),
            silent: true,
            exit_code,
        }
    }

    pub fn is_silent(&self) -> bool {
        self.silent
    }

    /// Process exit code to use when this error escapes to the entrypoint.
    pub fn exit_code(&self) -> i32 {
        self.exit_code
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
