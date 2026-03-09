//! Backend-backed module repository adapter.
//!
//! Delegates module reads to a [`BackendModuleReader`] when backend mode is
//! enabled. The filesystem repository remains the fallback when backend mode
//! is disabled.

use ito_common::id::parse_module_id as parse_common_module_id;
use ito_domain::backend::BackendModuleReader;
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
        let module_id = resolve_backend_module_key(id);
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
        let module_id = resolve_backend_module_key(id_or_name);
        match self.reader.get_module(&module_id) {
            Ok(module) => Ok(module),
            Err(DomainError::NotFound { .. }) => Err(DomainError::not_found("module", id_or_name)),
            Err(err) => Err(err),
        }
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        let mut modules = self.reader.list_modules()?;
        modules.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(modules)
    }
}

fn resolve_backend_module_key(id_or_name: &str) -> String {
    if let Ok(parsed) = parse_common_module_id(id_or_name) {
        return parsed.module_id.as_str().to_string();
    }

    id_or_name.to_string()
}
