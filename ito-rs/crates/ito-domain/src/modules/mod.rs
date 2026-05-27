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
    /// Sub-modules belonging to this module; empty when none are defined.
    pub sub_modules: Vec<SubModule>,
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
    /// Sub-module summaries; empty when none are defined.
    pub sub_modules: Vec<SubModuleSummary>,
}

/// A sub-module that groups changes within a parent module.
///
/// Sub-modules allow a module to be divided into named sections, each with
/// their own change sequence. The canonical identifier is `NNN.SS` where
/// `NNN` is the parent module number and `SS` is the sub-module number.
#[derive(Debug, Clone)]
pub struct SubModule {
    /// Canonical sub-module identifier (e.g., "005.01")
    pub id: String,
    /// Parent module identifier (e.g., "005")
    pub parent_module_id: String,
    /// Sub-module number, zero-padded to 2 digits (e.g., "01")
    pub sub_id: String,
    /// Sub-module name (e.g., "core-api")
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Number of changes in this sub-module
    pub change_count: u32,
    /// Path to the sub-module directory
    pub path: PathBuf,
}

/// Lightweight sub-module summary for listings.
///
/// Included in [`ModuleSummary`] when sub-modules are present.
#[derive(Debug, Clone)]
pub struct SubModuleSummary {
    /// Canonical sub-module identifier (e.g., "005.01")
    pub id: String,
    /// Sub-module name
    pub name: String,
    /// Number of changes in this sub-module
    pub change_count: u32,
}

#[cfg(test)]
mod modules_tests;
