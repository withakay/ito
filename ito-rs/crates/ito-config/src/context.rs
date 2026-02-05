use std::path::{Path, PathBuf};

use ito_common::fs::FileSystem;

use crate::{ConfigContext, ResolvedConfig, ito_config_dir, load_cascading_project_config_fs};

#[derive(Debug, Clone)]
pub struct ItoContext {
    pub config_dir: Option<PathBuf>,
    pub project_root: PathBuf,
    pub ito_path: Option<PathBuf>,
    pub config: ResolvedConfig,
}

impl ItoContext {
    pub fn resolve<F: FileSystem>(fs: &F, project_root: &Path) -> Self {
        let ctx = ConfigContext::from_process_env();
        Self::resolve_with_ctx(fs, project_root, ctx)
    }

    pub fn resolve_with_ctx<F: FileSystem>(
        fs: &F,
        project_root: &Path,
        ctx: ConfigContext,
    ) -> Self {
        let project_root = project_root.to_path_buf();
        let ito_path = crate::ito_dir::get_ito_path_fs(fs, &project_root, &ctx);
        let config_dir = ito_config_dir(&ctx);

        let config = load_cascading_project_config_fs(fs, &project_root, &ito_path, &ctx);

        let ito_path = fs.is_dir(&ito_path).then_some(ito_path);

        Self {
            config_dir,
            project_root,
            ito_path,
            config,
        }
    }
}
