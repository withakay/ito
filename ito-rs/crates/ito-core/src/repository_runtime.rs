//! Repository runtime selection and composition.
//!
//! Centralizes selection of repository implementations for the active
//! persistence mode, so adapters do not instantiate concrete repositories
//! directly.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use ito_config::types::ItoConfig;
use ito_config::{load_cascading_project_config, ConfigContext};

use crate::backend_change_repository::BackendChangeRepository;
use crate::backend_client::{resolve_backend_runtime, BackendRuntime};
use crate::backend_http::BackendHttpClient;
use crate::backend_module_repository::BackendModuleRepository;
use crate::change_repository::FsChangeRepository;
use crate::errors::{CoreError, CoreResult};
use crate::module_repository::FsModuleRepository;
use crate::remote_task_repository::RemoteTaskRepository;
use crate::task_repository::FsTaskRepository;
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;

/// Client persistence mode used for repository selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceMode {
    /// Local filesystem-backed repositories.
    Filesystem,
    /// Remote repositories backed by the backend API.
    Remote,
}

/// Bundle of domain repositories selected for the active persistence mode.
#[derive(Clone)]
pub struct RepositorySet {
    /// Change repository implementation.
    pub changes: Arc<dyn ChangeRepository + Send + Sync>,
    /// Module repository implementation.
    pub modules: Arc<dyn ModuleRepository + Send + Sync>,
    /// Task repository implementation.
    pub tasks: Arc<dyn TaskRepository + Send + Sync>,
}

/// Resolved repository runtime for the current configuration.
pub struct RepositoryRuntime {
    mode: PersistenceMode,
    ito_path: PathBuf,
    backend_runtime: Option<BackendRuntime>,
    repositories: RepositorySet,
}

impl RepositoryRuntime {
    /// Active persistence mode.
    pub fn mode(&self) -> PersistenceMode {
        self.mode
    }

    /// Root `.ito/` path for filesystem-backed helpers.
    pub fn ito_path(&self) -> &Path {
        self.ito_path.as_path()
    }

    /// Resolved backend runtime, if remote mode is active.
    pub fn backend_runtime(&self) -> Option<&BackendRuntime> {
        self.backend_runtime.as_ref()
    }

    /// Selected repository bundle.
    pub fn repositories(&self) -> &RepositorySet {
        &self.repositories
    }
}

/// Factory interface for building remote repository bundles.
pub trait RemoteRepositoryFactory: Send + Sync {
    /// Build a repository bundle using the provided backend runtime.
    fn build(&self, runtime: &BackendRuntime) -> CoreResult<RepositorySet>;
}

/// Remote factory that uses HTTP-backed repositories.
pub struct HttpRemoteRepositoryFactory;

impl RemoteRepositoryFactory for HttpRemoteRepositoryFactory {
    fn build(&self, runtime: &BackendRuntime) -> CoreResult<RepositorySet> {
        let client = BackendHttpClient::new(runtime.clone());
        Ok(RepositorySet {
            changes: Arc::new(BackendChangeRepository::new(client.clone())),
            modules: Arc::new(BackendModuleRepository::new(client.clone())),
            tasks: Arc::new(RemoteTaskRepository::new(client)),
        })
    }
}

/// Builder for repository runtime selection.
pub struct RepositoryRuntimeBuilder {
    ito_path: PathBuf,
    mode: PersistenceMode,
    backend_runtime: Option<BackendRuntime>,
    remote_factory: Arc<dyn RemoteRepositoryFactory>,
}

impl RepositoryRuntimeBuilder {
    /// Create a builder targeting the provided `.ito/` path.
    pub fn new(ito_path: impl Into<PathBuf>) -> Self {
        Self {
            ito_path: ito_path.into(),
            mode: PersistenceMode::Filesystem,
            backend_runtime: None,
            remote_factory: Arc::new(HttpRemoteRepositoryFactory),
        }
    }

    /// Set the persistence mode.
    pub fn mode(mut self, mode: PersistenceMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the backend runtime for remote mode.
    pub fn backend_runtime(mut self, runtime: BackendRuntime) -> Self {
        self.backend_runtime = Some(runtime);
        self
    }

    /// Override the remote repository factory.
    pub fn remote_factory(mut self, factory: Arc<dyn RemoteRepositoryFactory>) -> Self {
        self.remote_factory = factory;
        self
    }

    /// Build the repository runtime.
    pub fn build(self) -> CoreResult<RepositoryRuntime> {
        match self.mode {
            PersistenceMode::Filesystem => {
                let repositories = filesystem_repository_set(&self.ito_path);
                Ok(RepositoryRuntime {
                    mode: PersistenceMode::Filesystem,
                    ito_path: self.ito_path,
                    backend_runtime: None,
                    repositories,
                })
            }
            PersistenceMode::Remote => {
                let runtime = self.backend_runtime.ok_or_else(|| {
                    CoreError::validation("remote mode requires backend runtime".to_string())
                })?;
                let repositories = self.remote_factory.build(&runtime)?;
                Ok(RepositoryRuntime {
                    mode: PersistenceMode::Remote,
                    ito_path: self.ito_path,
                    backend_runtime: Some(runtime),
                    repositories,
                })
            }
        }
    }
}

/// Resolve repository runtime for the current configuration.
pub fn resolve_repository_runtime(
    ito_path: &Path,
    ctx: &ConfigContext,
) -> CoreResult<RepositoryRuntime> {
    let project_root = ctx
        .project_dir
        .as_deref()
        .unwrap_or_else(|| ito_path.parent().unwrap_or(ito_path));
    let merged = load_cascading_project_config(project_root, ito_path, ctx).merged;
    let backend_enabled = merged
        .pointer("/backend/enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let config = match serde_json::from_value::<ItoConfig>(merged) {
        Ok(config) => config,
        Err(err) => {
            if backend_enabled {
                return Err(CoreError::validation(format!(
                    "Failed to parse Ito config while backend mode is enabled: {err}"
                )));
            }
            return RepositoryRuntimeBuilder::new(ito_path).build();
        }
    };

    if !config.backend.enabled {
        return RepositoryRuntimeBuilder::new(ito_path).build();
    }

    let runtime = resolve_backend_runtime(&config.backend)?.ok_or_else(|| {
        CoreError::validation("Backend mode is enabled but runtime was not resolved".to_string())
    })?;

    RepositoryRuntimeBuilder::new(ito_path)
        .mode(PersistenceMode::Remote)
        .backend_runtime(runtime)
        .build()
}

fn filesystem_repository_set(ito_path: &Path) -> RepositorySet {
    let ito_path = ito_path.to_path_buf();
    RepositorySet {
        changes: Arc::new(OwnedFsChangeRepository::new(ito_path.clone())),
        modules: Arc::new(OwnedFsModuleRepository::new(ito_path.clone())),
        tasks: Arc::new(OwnedFsTaskRepository::new(ito_path)),
    }
}

pub(crate) fn boxed_fs_change_repository(ito_path: PathBuf) -> Box<dyn ChangeRepository + Send> {
    Box::new(OwnedFsChangeRepository::new(ito_path))
}

pub(crate) fn boxed_fs_module_repository(ito_path: PathBuf) -> Box<dyn ModuleRepository + Send> {
    Box::new(OwnedFsModuleRepository::new(ito_path))
}

pub(crate) fn boxed_fs_task_repository(ito_path: PathBuf) -> Box<dyn TaskRepository + Send> {
    Box::new(OwnedFsTaskRepository::new(ito_path))
}

// ── Owned-path filesystem wrappers ─────────────────────────────────

#[derive(Debug, Clone)]
struct OwnedFsChangeRepository {
    ito_path: PathBuf,
}

impl OwnedFsChangeRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsChangeRepository<'_> {
        FsChangeRepository::new(&self.ito_path)
    }
}

impl ChangeRepository for OwnedFsChangeRepository {
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ito_domain::changes::ResolveTargetOptions,
    ) -> ito_domain::changes::ChangeTargetResolution {
        self.inner().resolve_target_with_options(input, options)
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        self.inner().suggest_targets(input, max)
    }

    fn exists(&self, id: &str) -> bool {
        self.inner().exists(id)
    }

    fn exists_with_filter(
        &self,
        id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> bool {
        self.inner().exists_with_filter(id, filter)
    }

    fn get_with_filter(
        &self,
        id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> ito_domain::errors::DomainResult<ito_domain::changes::Change> {
        self.inner().get_with_filter(id, filter)
    }

    fn list_with_filter(
        &self,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> ito_domain::errors::DomainResult<Vec<ito_domain::changes::ChangeSummary>> {
        self.inner().list_with_filter(filter)
    }

    fn list_by_module_with_filter(
        &self,
        module_id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> ito_domain::errors::DomainResult<Vec<ito_domain::changes::ChangeSummary>> {
        self.inner().list_by_module_with_filter(module_id, filter)
    }

    fn list_incomplete_with_filter(
        &self,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> ito_domain::errors::DomainResult<Vec<ito_domain::changes::ChangeSummary>> {
        self.inner().list_incomplete_with_filter(filter)
    }

    fn list_complete_with_filter(
        &self,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> ito_domain::errors::DomainResult<Vec<ito_domain::changes::ChangeSummary>> {
        self.inner().list_complete_with_filter(filter)
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> ito_domain::errors::DomainResult<ito_domain::changes::ChangeSummary> {
        self.inner().get_summary_with_filter(id, filter)
    }
}

#[derive(Debug, Clone)]
struct OwnedFsModuleRepository {
    ito_path: PathBuf,
}

impl OwnedFsModuleRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsModuleRepository<'_> {
        FsModuleRepository::new(&self.ito_path)
    }
}

impl ModuleRepository for OwnedFsModuleRepository {
    fn exists(&self, id: &str) -> bool {
        self.inner().exists(id)
    }

    fn get(
        &self,
        id_or_name: &str,
    ) -> ito_domain::errors::DomainResult<ito_domain::modules::Module> {
        self.inner().get(id_or_name)
    }

    fn list(&self) -> ito_domain::errors::DomainResult<Vec<ito_domain::modules::ModuleSummary>> {
        self.inner().list()
    }
}

#[derive(Debug, Clone)]
struct OwnedFsTaskRepository {
    ito_path: PathBuf,
}

impl OwnedFsTaskRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsTaskRepository<'_> {
        FsTaskRepository::new(&self.ito_path)
    }
}

impl TaskRepository for OwnedFsTaskRepository {
    fn load_tasks(
        &self,
        change_id: &str,
    ) -> ito_domain::errors::DomainResult<ito_domain::tasks::TasksParseResult> {
        self.inner().load_tasks(change_id)
    }
}
