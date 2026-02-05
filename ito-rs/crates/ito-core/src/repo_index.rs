use std::collections::BTreeSet;
use std::path::Path;

use miette::Result;

use ito_common::fs::StdFs;

#[derive(Debug, Default, Clone)]
pub struct RepoIndex {
    pub module_dir_names: Vec<String>,
    pub module_ids: BTreeSet<String>,
    pub change_dir_names: Vec<String>,
    pub spec_dir_names: Vec<String>,
}

impl RepoIndex {
    pub fn load(ito_path: &Path) -> Result<Self> {
        let fs = StdFs;
        let module_dir_names = ito_domain::discovery::list_module_dir_names(&fs, ito_path)?;
        let module_ids = ito_domain::discovery::list_module_ids(&fs, ito_path)?;
        let change_dir_names = ito_domain::discovery::list_change_dir_names(&fs, ito_path)?;
        let spec_dir_names = ito_domain::discovery::list_spec_dir_names(&fs, ito_path)?;

        Ok(Self {
            module_dir_names,
            module_ids,
            change_dir_names,
            spec_dir_names,
        })
    }
}
