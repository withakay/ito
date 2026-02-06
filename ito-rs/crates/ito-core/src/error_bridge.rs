//! Bridges domain errors into core errors.

use crate::errors::CoreError;
use ito_domain::errors::DomainError;

/// Convert domain results into core-level results.
pub trait IntoCoreResult<T> {
    /// Convert `Result<T, DomainError>` into `Result<T, CoreError>`.
    fn into_core(self) -> Result<T, CoreError>;
}

impl<T> IntoCoreResult<T> for Result<T, DomainError> {
    fn into_core(self) -> Result<T, CoreError> {
        self.map_err(CoreError::from)
    }
}
