//! Repository runtime selection and composition.
//!
//! Centralizes selection of repository implementations for the active
//! persistence mode, so adapters do not instantiate concrete repositories
//! directly.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use ito_config::ito_dir::{absolutize_and_normalize, lexical_normalize};
use ito_config::types::{ItoConfig, RepositoryPersistenceMode};
use ito_config::{ConfigContext, load_cascading_project_config};

use crate::backend_change_repository::BackendChangeRepository;
use crate::backend_client::{BackendRuntime, resolve_backend_runtime};
use crate::backend_http::BackendHttpClient;
use crate::backend_module_repository::BackendModuleRepository;
use crate::backend_spec_repository::BackendSpecRepository;
use crate::change_repository::FsChangeRepository;
use crate::errors::{CoreError, CoreResult};
use crate::module_repository::FsModuleRepository;
use crate::remote_task_repository::RemoteTaskRepository;
use crate::spec_repository::FsSpecRepository;
use crate::sqlite_project_store::SqliteBackendProjectStore;
use crate::task_mutations::{FsTaskMutationService, boxed_fs_task_mutation_service};
use crate::task_repository::FsTaskRepository;
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::specs::SpecRepository;
use ito_domain::tasks::{TaskMutationService, TaskRepository};

/// Client persistence mode used for repository selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceMode {
    /// Local filesystem-backed repositories.
    Filesystem,
    /// Local SQLite-backed repositories.
    Sqlite,
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
    /// Task mutation service implementation.
    pub task_mutations: Arc<dyn TaskMutationService + Send + Sync>,
    /// Promoted spec repository implementation.
    pub specs: Arc<dyn SpecRepository + Send + Sync>,
}

/// Resolved SQLite runtime settings for local persistence.
#[derive(Debug, Clone)]
pub struct SqliteRuntime {
    /// Path to the SQLite database file.
    pub db_path: PathBuf,
    /// Organization namespace for the local project (currently derived as `local`).
    pub org: String,
    /// Repository namespace for the local project (derived from the project root directory name).
    pub repo: String,
}

/// Resolved repository runtime for the current configuration.
pub struct RepositoryRuntime {
    mode: PersistenceMode,
    ito_path: PathBuf,
    backend_runtime: Option<BackendRuntime>,
    sqlite_runtime: Option<SqliteRuntime>,
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

    /// Resolved SQLite runtime, if SQLite mode is active.
    pub fn sqlite_runtime(&self) -> Option<&SqliteRuntime> {
        self.sqlite_runtime.as_ref()
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
            tasks: Arc::new(RemoteTaskRepository::new(client.clone())),
            task_mutations: Arc::new(client.clone()),
            specs: Arc::new(BackendSpecRepository::new(client.clone())),
        })
    }
}

/// Builder for repository runtime selection.
pub struct RepositoryRuntimeBuilder {
    ito_path: PathBuf,
    mode: PersistenceMode,
    backend_runtime: Option<BackendRuntime>,
    sqlite_runtime: Option<SqliteRuntime>,
    remote_factory: Arc<dyn RemoteRepositoryFactory>,
}

impl RepositoryRuntimeBuilder {
    /// Create a builder targeting the provided `.ito/` path.
    pub fn new(ito_path: impl Into<PathBuf>) -> Self {
        Self {
            ito_path: ito_path.into(),
            mode: PersistenceMode::Filesystem,
            backend_runtime: None,
            sqlite_runtime: None,
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

    /// Set the SQLite runtime for SQLite mode.
    pub fn sqlite_runtime(mut self, runtime: SqliteRuntime) -> Self {
        self.sqlite_runtime = Some(runtime);
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
                    sqlite_runtime: None,
                    repositories,
                })
            }
            PersistenceMode::Sqlite => {
                let runtime = self.sqlite_runtime.ok_or_else(|| {
                    CoreError::validation("sqlite mode requires sqlite runtime".to_string())
                })?;
                let repositories = sqlite_repository_set(&runtime)?;
                Ok(RepositoryRuntime {
                    mode: PersistenceMode::Sqlite,
                    ito_path: self.ito_path,
                    backend_runtime: None,
                    sqlite_runtime: Some(runtime),
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
                    sqlite_runtime: None,
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
    let raw_mode = merged
        .pointer("/repository/mode")
        .and_then(|v| v.as_str())
        .unwrap_or("filesystem");

    // Fail fast on unrecognized repository.mode values before attempting full
    // config deserialization. This prevents silent fallback to filesystem mode
    // when the user has set an invalid mode string.
    if RepositoryPersistenceMode::parse_value(raw_mode).is_none() {
        let valid = RepositoryPersistenceMode::ALL.join(", ");
        return Err(CoreError::validation(format!(
            "Invalid repository.mode '{raw_mode}': must be one of {valid}"
        )));
    }

    let sqlite_enabled = raw_mode == "sqlite";

    let config = match serde_json::from_value::<ItoConfig>(merged) {
        Ok(config) => config,
        Err(err) => {
            if backend_enabled || sqlite_enabled {
                let mode = if backend_enabled {
                    "backend mode is enabled"
                } else {
                    "sqlite persistence mode is enabled"
                };
                return Err(CoreError::validation(format!(
                    "Failed to parse Ito config while {mode}: {err}"
                )));
            }
            return RepositoryRuntimeBuilder::new(ito_path).build();
        }
    };

    if !config.backend.enabled {
        return match config.repository.mode {
            RepositoryPersistenceMode::Filesystem => {
                RepositoryRuntimeBuilder::new(ito_path).build()
            }
            RepositoryPersistenceMode::Sqlite => {
                let runtime = resolve_sqlite_runtime(&config, project_root)?;
                RepositoryRuntimeBuilder::new(ito_path)
                    .mode(PersistenceMode::Sqlite)
                    .sqlite_runtime(runtime)
                    .build()
            }
        };
    }

    let runtime = resolve_backend_runtime(&config.backend)?.ok_or_else(|| {
        CoreError::validation("Backend mode is enabled but runtime was not resolved".to_string())
    })?;

    RepositoryRuntimeBuilder::new(ito_path)
        .mode(PersistenceMode::Remote)
        .backend_runtime(runtime)
        .build()
}

fn resolve_sqlite_runtime(config: &ItoConfig, project_root: &Path) -> CoreResult<SqliteRuntime> {
    let Some(db_path) = config.repository.sqlite.db_path.as_deref() else {
        return Err(CoreError::validation(
            "SQLite persistence mode requires 'repository.sqlite.dbPath' to be set",
        ));
    };
    let db_path = db_path.trim();
    if db_path.is_empty() {
        return Err(CoreError::validation(
            "SQLite persistence mode requires 'repository.sqlite.dbPath' to be set",
        ));
    }

    let db_path = PathBuf::from(db_path);
    let db_path = if db_path.is_absolute() {
        db_path
    } else {
        project_root.join(db_path)
    };
    let db_path =
        absolutize_and_normalize(&db_path).unwrap_or_else(|_| lexical_normalize(&db_path));

    let repo = match project_root.file_name().and_then(|s| s.to_str()) {
        Some(name) if !name.trim().is_empty() => name.to_string(),
        _ => "project".to_string(),
    };

    Ok(SqliteRuntime {
        db_path,
        org: "local".to_string(),
        repo,
    })
}

fn filesystem_repository_set(ito_path: &Path) -> RepositorySet {
    let ito_path = ito_path.to_path_buf();
    RepositorySet {
        changes: Arc::new(OwnedFsChangeRepository::new(ito_path.clone())),
        modules: Arc::new(OwnedFsModuleRepository::new(ito_path.clone())),
        tasks: Arc::new(OwnedFsTaskRepository::new(ito_path.clone())),
        task_mutations: Arc::new(FsTaskMutationService::new(ito_path.clone())),
        specs: Arc::new(OwnedFsSpecRepository::new(ito_path)),
    }
}

fn sqlite_repository_set(runtime: &SqliteRuntime) -> CoreResult<RepositorySet> {
    let store = SqliteBackendProjectStore::open(&runtime.db_path)?;
    store.repository_set(&runtime.org, &runtime.repo)
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

pub(crate) fn boxed_fs_task_mutation_port(
    ito_path: PathBuf,
) -> Box<dyn TaskMutationService + Send> {
    boxed_fs_task_mutation_service(ito_path)
}

pub(crate) fn boxed_fs_spec_repository(ito_path: PathBuf) -> Box<dyn SpecRepository + Send> {
    Box::new(OwnedFsSpecRepository::new(ito_path))
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

#[derive(Debug, Clone)]
struct OwnedFsSpecRepository {
    ito_path: PathBuf,
}

impl OwnedFsSpecRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsSpecRepository<'_> {
        FsSpecRepository::new(&self.ito_path)
    }
}

impl SpecRepository for OwnedFsSpecRepository {
    fn list(&self) -> ito_domain::errors::DomainResult<Vec<ito_domain::specs::SpecSummary>> {
        self.inner().list()
    }

    fn get(&self, id: &str) -> ito_domain::errors::DomainResult<ito_domain::specs::SpecDocument> {
        self.inner().get(id)
    }
}
