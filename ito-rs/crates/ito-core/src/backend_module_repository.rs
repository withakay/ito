//! Backend-backed module repository adapter.
//!
//! Delegates module reads to a [`BackendModuleReader`] when backend mode is
//! enabled. The filesystem repository remains the fallback when backend mode
//! is disabled.

use ito_common::id::parse_module_id as parse_common_module_id;
use ito_domain::backend::BackendModuleReader;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{
    Module, ModuleRepository as DomainModuleRepository, ModuleSummary, SubModule, SubModuleSummary,
};

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

    fn list_sub_modules(&self, parent_id: &str) -> DomainResult<Vec<SubModuleSummary>> {
        let module_id = resolve_backend_module_key(parent_id);
        let module = match self.reader.get_module(&module_id) {
            Ok(m) => m,
            Err(DomainError::NotFound { .. }) => {
                return Err(DomainError::not_found("module", parent_id));
            }
            Err(err) => return Err(err),
        };
        let mut sub_modules = Vec::with_capacity(module.sub_modules.len());
        for s in module.sub_modules {
            sub_modules.push(SubModuleSummary {
                id: s.id,
                name: s.name,
                change_count: s.change_count,
            });
        }
        sub_modules.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(sub_modules)
    }

    fn get_sub_module(&self, composite_id: &str) -> DomainResult<SubModule> {
        // Extract the parent module ID from the composite sub-module ID (e.g., "005.01" -> "005").
        let parent_id = composite_id.split('.').next().unwrap_or(composite_id);
        let module_id = resolve_backend_module_key(parent_id);
        let module = match self.reader.get_module(&module_id) {
            Ok(m) => m,
            Err(DomainError::NotFound { .. }) => {
                return Err(DomainError::not_found("sub-module", composite_id));
            }
            Err(err) => return Err(err),
        };
        for sub in module.sub_modules {
            if sub.id == composite_id {
                return Ok(sub);
            }
        }
        Err(DomainError::not_found("sub-module", composite_id))
    }
}

fn resolve_backend_module_key(id_or_name: &str) -> String {
    if let Ok(parsed) = parse_common_module_id(id_or_name) {
        return parsed.module_id.as_str().to_string();
    }

    id_or_name.to_string()
}
