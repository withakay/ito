//! Spec repository port definitions.

use super::{SpecDocument, SpecSummary};
use crate::errors::DomainResult;

/// Port for accessing promoted spec data.
pub trait SpecRepository {
    /// List all promoted specs.
    fn list(&self) -> DomainResult<Vec<SpecSummary>>;

    /// Get a promoted spec by ID.
    fn get(&self, id: &str) -> DomainResult<SpecDocument>;
}
