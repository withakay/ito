//! Module repository port definitions.

use miette::Result;

use super::{Module, ModuleSummary};

/// Port for accessing module data.
///
/// Domain and adapters should depend on this interface rather than concrete
/// storage details.
pub trait ModuleRepository {
    /// Check if a module exists.
    fn exists(&self, id: &str) -> bool;

    /// Get a module by ID or full name.
    fn get(&self, id_or_name: &str) -> Result<Module>;

    /// List all modules.
    fn list(&self) -> Result<Vec<ModuleSummary>>;
}
