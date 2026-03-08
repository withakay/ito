//! Backend-backed module repository adapter.
//!
//! Delegates module reads to a [`BackendModuleReader`] when backend mode is
//! enabled. The filesystem repository remains the fallback when backend mode
//! is disabled.

use ito_domain::backend::BackendModuleReader;
use ito_domain::changes::parse_module_id;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository as DomainModuleRepository, ModuleSummary};

/// Backend-backed module repository.
///
/// Wraps a [`BackendModuleReader`] implementation and delegates module reads
/// to the backend API.
pub struct BackendModuleRepository<R: BackendModuleReader> {
    reader: R,
}

impl<R: BackendModuleReader> BackendModuleRepository<R> {
    /// Create a backend-backed module repository.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BackendModuleReader> DomainModuleRepository for BackendModuleRepository<R> {
    fn exists(&self, id: &str) -> bool {
        let module_id = parse_module_id(id);
        match self.reader.get_module(&module_id) {
            Ok(_) => true,
            Err(DomainError::NotFound { .. }) => false,
            Err(err) => {
                tracing::warn!("backend module exists check failed: {err}");
                true
            }
        }
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        let module_id = parse_module_id(id_or_name);
        match self.reader.get_module(&module_id) {
            Ok(module) => Ok(module),
            Err(DomainError::NotFound { .. }) => Err(DomainError::not_found("module", id_or_name)),
            Err(err) => Err(err),
        }
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        self.reader.list_modules()
    }
}
