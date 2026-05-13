//! Project planning path helpers.
//!
//! Ito's planning area lives under `{ito_path}/planning`.
//! This module provides pure path helpers. Filesystem I/O lives in `ito-core`.

use std::path::{Path, PathBuf};

// Filesystem I/O (init_planning_structure) lives in `ito-core`.

/// Path to the planning directory (`{ito_path}/planning`).
#[must_use]
pub fn planning_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("planning")
}

/// Path to the companion research directory (`{ito_path}/research`).
#[must_use]
pub fn research_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("research")
}
