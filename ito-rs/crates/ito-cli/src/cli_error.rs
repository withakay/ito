use std::fmt;

use ito_core::errors::CoreError;

#[derive(Debug, Clone)]
pub struct CliError {
    message: String,
    silent: bool,
    exit_code: i32,
    #[allow(dead_code)]
    feature_unavailable: Option<FeatureUnavailableDetails>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct FeatureUnavailableDetails {
    feature: String,
    requested_by: String,
    recovery: String,
}

impl CliError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            silent: false,
            exit_code: 1,
            feature_unavailable: None,
        }
    }

    pub fn silent() -> Self {
        Self {
            message: String::new(),
            silent: true,
            exit_code: 1,
            feature_unavailable: None,
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
            feature_unavailable: None,
        }
    }

    /// Construct a silent `CliError` with a specific exit code.
    pub fn silent_with_code(exit_code: i32) -> Self {
        Self {
            message: String::new(),
            silent: true,
            exit_code,
            feature_unavailable: None,
        }
    }

    /// Construct a typed unavailable-feature error for compatibility dispatch.
    pub fn feature_unavailable(
        feature: impl Into<String>,
        requested_by: impl Into<String>,
        recovery: impl Into<String>,
    ) -> Self {
        let details = FeatureUnavailableDetails {
            feature: feature.into(),
            requested_by: requested_by.into(),
            recovery: recovery.into(),
        };
        Self {
            message: format!(
                "feature '{}' is unavailable (requested by {}). Recovery: {}",
                details.feature, details.requested_by, details.recovery
            ),
            silent: false,
            exit_code: 1,
            feature_unavailable: Some(details),
        }
    }

    /// Machine-readable representation for typed unavailable-feature errors.
    pub fn feature_unavailable_json(&self) -> Option<serde_json::Value> {
        self.feature_unavailable.as_ref().map(|details| {
            serde_json::json!({
                "error": {
                    "kind": "feature_unavailable",
                    "feature": details.feature,
                    "requested_by": details.requested_by,
                    "recovery": details.recovery,
                }
            })
        })
    }

    /// Preserve typed core capability errors at the CLI presentation boundary.
    pub fn from_core(error: CoreError) -> Self {
        match error {
            CoreError::FeatureUnavailable {
                feature,
                requested_by,
                recovery,
            } => Self::feature_unavailable(feature.as_str(), requested_by, recovery),
            other => Self::msg(other.to_string()),
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

#[cfg(test)]
#[path = "cli_error_tests.rs"]
mod cli_error_tests;
