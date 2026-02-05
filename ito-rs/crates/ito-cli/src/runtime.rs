use ito_config::ConfigContext;
use ito_config::ito_dir::get_ito_path;
use ito_core::repo_index::RepoIndex;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub(crate) struct Runtime {
    ctx: ConfigContext,
    cwd: PathBuf,
    ito_path: OnceLock<PathBuf>,
    repo_index: OnceLock<RepoIndex>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            ctx: ConfigContext::from_process_env(),
            cwd: PathBuf::from("."),
            ito_path: OnceLock::new(),
            repo_index: OnceLock::new(),
        }
    }

    pub(crate) fn ctx(&self) -> &ConfigContext {
        &self.ctx
    }

    pub(crate) fn ito_path(&self) -> &Path {
        self.ito_path
            .get_or_init(|| get_ito_path(&self.cwd, &self.ctx))
            .as_path()
    }

    pub(crate) fn repo_index(&self) -> &RepoIndex {
        self.repo_index
            .get_or_init(|| RepoIndex::load(self.ito_path()).unwrap_or_default())
    }
}
