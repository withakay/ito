//! Module repository port definitions.

use super::{Module, ModuleSummary, SubModule, SubModuleSummary};
use crate::errors::{DomainError, DomainResult};

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

    /// List all sub-modules belonging to a parent module.
    ///
    /// `parent_id` is the parent module identifier (e.g., `"024"`).
    ///
    /// The default implementation returns a not-found error so that existing
    /// implementors do not need to be updated immediately.
    fn list_sub_modules(&self, parent_id: &str) -> DomainResult<Vec<SubModuleSummary>> {
        Err(DomainError::not_found("sub-module", parent_id))
    }

    /// Get a sub-module by its composite identifier (e.g., `"024.01"`).
    ///
    /// The default implementation returns a not-found error so that existing
    /// implementors do not need to be updated immediately.
    fn get_sub_module(&self, composite_id: &str) -> DomainResult<SubModule> {
        Err(DomainError::not_found("sub-module", composite_id))
    }
}
