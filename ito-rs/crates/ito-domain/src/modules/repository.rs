//! Module repository port definitions.

use super::{Module, ModuleSummary};
use crate::errors::DomainResult;

/// Port for accessing module data.
///
/// Domain and adapters should depend on this interface rather than concrete
/// storage details.
pub trait ModuleRepository {
    /// Check if a module exists.
    fn exists(&self, id: &str) -> bool;

    /// Get a module by ID or full name.
    fn get(&self, id_or_name: &str) -> DomainResult<Module>;

    /// List all modules.
    fn list(&self) -> DomainResult<Vec<ModuleSummary>>;
}
