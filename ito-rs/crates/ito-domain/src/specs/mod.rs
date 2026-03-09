//! Promoted spec domain models and repository.

mod repository;

pub use repository::SpecRepository;

use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Full promoted spec document.
#[derive(Debug, Clone)]
pub struct SpecDocument {
    /// Spec identifier.
    pub id: String,
    /// Canonical source path for the spec.
    pub path: PathBuf,
    /// Raw markdown contents.
    pub markdown: String,
    /// Last modification timestamp.
    pub last_modified: DateTime<Utc>,
}

/// Lightweight promoted spec summary.
#[derive(Debug, Clone)]
pub struct SpecSummary {
    /// Spec identifier.
    pub id: String,
    /// Canonical source path for the spec.
    pub path: PathBuf,
    /// Last modification timestamp.
    pub last_modified: DateTime<Utc>,
}
