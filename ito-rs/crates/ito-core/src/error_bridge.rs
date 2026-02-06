//! Bridges domain errors into core diagnostics.

use ito_domain::errors::DomainError;
use miette::{Report, miette};

/// Convert a domain error into a core miette report.
pub fn domain_to_report(error: DomainError) -> Report {
    miette!(error.to_string())
}

/// Convert domain results into core-level miette results.
pub trait IntoCoreMiette<T> {
    /// Convert `Result<T, DomainError>` into `miette::Result<T>`.
    fn into_core_miette(self) -> miette::Result<T>;
}

impl<T> IntoCoreMiette<T> for Result<T, DomainError> {
    fn into_core_miette(self) -> miette::Result<T> {
        self.map_err(domain_to_report)
    }
}
