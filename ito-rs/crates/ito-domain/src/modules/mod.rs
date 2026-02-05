//! Module domain models and repository.
//!
//! This module provides domain models for Ito modules and a repository
//! for loading and querying module data.

mod repository;

pub use repository::ModuleRepository;

use std::path::PathBuf;

/// Full module with metadata loaded.
#[derive(Debug, Clone)]
pub struct Module {
    /// Module identifier (e.g., "005")
    pub id: String,
    /// Module name (e.g., "dev-tooling")
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Path to the module directory
    pub path: PathBuf,
}

/// Lightweight module summary for listings.
#[derive(Debug, Clone)]
pub struct ModuleSummary {
    /// Module identifier
    pub id: String,
    /// Module name
    pub name: String,
    /// Number of changes in this module
    pub change_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = Module {
            id: "005".to_string(),
            name: "dev-tooling".to_string(),
            description: Some("Development tooling".to_string()),
            path: PathBuf::from("/test"),
        };

        assert_eq!(module.id, "005");
        assert_eq!(module.name, "dev-tooling");
    }

    #[test]
    fn test_module_summary() {
        let summary = ModuleSummary {
            id: "005".to_string(),
            name: "dev-tooling".to_string(),
            change_count: 3,
        };

        assert_eq!(summary.change_count, 3);
    }
}
