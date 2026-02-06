//! Precomputed index of an Ito repository.
//!
//! `RepoIndex` gathers a few commonly-used directory listings and ids from the
//! `.ito/` working directory.

use std::collections::BTreeSet;
use std::path::Path;

use crate::error_bridge::IntoCoreMiette;
use miette::Result;

use ito_common::fs::StdFs;

#[derive(Debug, Default, Clone)]
/// Directory listings and ids derived from an Ito repo.
pub struct RepoIndex {
    /// Full module directory names under `.ito/modules/` (e.g. `014_rust-documentation`).
    pub module_dir_names: Vec<String>,

    /// Canonical 3-digit module ids extracted from module directory names.
    pub module_ids: BTreeSet<String>,

    /// Change directory names under `.ito/changes/`.
    pub change_dir_names: Vec<String>,

    /// Spec directory names under `.ito/specs/`.
    pub spec_dir_names: Vec<String>,
}

impl RepoIndex {
    /// Load a fresh index from `ito_path`.
    pub fn load(ito_path: &Path) -> Result<Self> {
        let fs = StdFs;
        let module_dir_names =
            ito_domain::discovery::list_module_dir_names(&fs, ito_path).into_core_miette()?;
        let module_ids =
            ito_domain::discovery::list_module_ids(&fs, ito_path).into_core_miette()?;
        let change_dir_names =
            ito_domain::discovery::list_change_dir_names(&fs, ito_path).into_core_miette()?;
        let spec_dir_names =
            ito_domain::discovery::list_spec_dir_names(&fs, ito_path).into_core_miette()?;

        Ok(Self {
            module_dir_names,
            module_ids,
            change_dir_names,
            spec_dir_names,
        })
    }
}
