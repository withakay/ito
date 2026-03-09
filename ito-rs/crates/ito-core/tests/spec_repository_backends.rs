use std::path::PathBuf;
use std::sync::Arc;

use chrono::Utc;
use tempfile::TempDir;

use ito_core::backend_client::BackendRuntime;
use ito_core::repository_runtime::{
    PersistenceMode, RemoteRepositoryFactory, RepositoryRuntimeBuilder, RepositorySet,
};
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
use ito_domain::specs::{SpecDocument, SpecRepository, SpecSummary};
use ito_domain::tasks::{
    TaskInitResult, TaskMutationError, TaskMutationResult, TaskMutationService,
    TaskMutationServiceResult, TaskRepository, TasksParseResult,
};

fn write(path: impl Into<PathBuf>, contents: &str) {
    let path = path.into();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dirs");
    }
    std::fs::write(path, contents).expect("write file");
}

#[test]
fn filesystem_runtime_exposes_promoted_specs() {
    let tmp = TempDir::new().expect("temp repo");
    let ito_path = tmp.path().join(".ito");
    write(
        ito_path.join("specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nAlpha purpose.\n",
    );
    write(
        ito_path.join("specs/beta/spec.md"),
        "# Beta\n\n## Purpose\nBeta purpose.\n",
    );

    let runtime = RepositoryRuntimeBuilder::new(&ito_path)
        .build()
        .expect("filesystem runtime");

    let summaries = runtime.repositories().specs.list().expect("list specs");
    assert_eq!(
        summaries
            .into_iter()
            .map(|spec| spec.id)
            .collect::<Vec<_>>(),
        vec!["alpha".to_string(), "beta".to_string()]
    );

    let spec = runtime
        .repositories()
        .specs
        .get("alpha")
        .expect("get alpha spec");
    assert_eq!(spec.id, "alpha");
    assert!(spec.markdown.contains("# Alpha"));
    assert!(spec.path.ends_with(".ito/specs/alpha/spec.md"));
}

#[test]
fn remote_runtime_exposes_spec_repository_without_local_specs() {
    let tmp = TempDir::new().expect("temp repo");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("ito dir");

    let repos = RepositorySet {
        changes: Arc::new(FakeChangeRepo),
        modules: Arc::new(FakeModuleRepo),
        tasks: Arc::new(FakeTaskRepo),
        task_mutations: Arc::new(FakeTaskMutations),
        specs: Arc::new(FakeSpecRepo::new()),
    };

    let runtime = RepositoryRuntimeBuilder::new(&ito_path)
        .mode(PersistenceMode::Remote)
        .backend_runtime(BackendRuntime {
            base_url: "http://127.0.0.1:9010".to_string(),
            token: "test-token".to_string(),
            timeout: std::time::Duration::from_secs(1),
            max_retries: 1,
            backup_dir: tmp.path().join("backups"),
            org: "acme".to_string(),
            repo: "widgets".to_string(),
        })
        .remote_factory(Arc::new(FakeRemoteFactory { repos }))
        .build()
        .expect("remote runtime");

    let summaries = runtime.repositories().specs.list().expect("list specs");
    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].id, "alpha");
    assert_eq!(summaries[1].id, "beta");

    let spec = runtime
        .repositories()
        .specs
        .get("beta")
        .expect("get beta spec");
    assert_eq!(spec.id, "beta");
    assert!(spec.markdown.contains("# Beta"));
}

#[derive(Clone)]
struct FakeRemoteFactory {
    repos: RepositorySet,
}

impl RemoteRepositoryFactory for FakeRemoteFactory {
    fn build(&self, _runtime: &BackendRuntime) -> ito_core::errors::CoreResult<RepositorySet> {
        Ok(self.repos.clone())
    }
}

struct FakeChangeRepo;

impl ChangeRepository for FakeChangeRepo {
    fn resolve_target_with_options(
        &self,
        _input: &str,
        _options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        ChangeTargetResolution::NotFound
    }

    fn suggest_targets(&self, _input: &str, _max: usize) -> Vec<String> {
        Vec::new()
    }

    fn exists(&self, _id: &str) -> bool {
        false
    }

    fn exists_with_filter(&self, _id: &str, _filter: ChangeLifecycleFilter) -> bool {
        false
    }

    fn get_with_filter(&self, id: &str, _filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        Err(DomainError::not_found("change", id))
    }

    fn list_with_filter(&self, _filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        Ok(Vec::new())
    }

    fn list_by_module_with_filter(
        &self,
        _module_id: &str,
        _filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        Ok(Vec::new())
    }

    fn list_incomplete_with_filter(
        &self,
        _filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        Ok(Vec::new())
    }

    fn list_complete_with_filter(
        &self,
        _filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        Ok(Vec::new())
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        _filter: ChangeLifecycleFilter,
    ) -> DomainResult<ChangeSummary> {
        Err(DomainError::not_found("change", id))
    }
}

struct FakeModuleRepo;

impl ModuleRepository for FakeModuleRepo {
    fn exists(&self, _id: &str) -> bool {
        false
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        Err(DomainError::not_found("module", id_or_name))
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        Ok(Vec::new())
    }
}

struct FakeTaskRepo;

impl TaskRepository for FakeTaskRepo {
    fn load_tasks(&self, _change_id: &str) -> DomainResult<TasksParseResult> {
        Ok(TasksParseResult::empty())
    }
}

struct FakeTaskMutations;

impl TaskMutationService for FakeTaskMutations {
    fn load_tasks_markdown(&self, _change_id: &str) -> TaskMutationServiceResult<Option<String>> {
        Ok(None)
    }

    fn init_tasks(&self, change_id: &str) -> TaskMutationServiceResult<TaskInitResult> {
        Ok(TaskInitResult {
            change_id: change_id.to_string(),
            path: None,
            existed: false,
            revision: None,
        })
    }

    fn start_task(
        &self,
        _change_id: &str,
        _task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        Err(TaskMutationError::validation("unused in test"))
    }

    fn complete_task(
        &self,
        _change_id: &str,
        _task_id: &str,
        _note: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        Err(TaskMutationError::validation("unused in test"))
    }

    fn shelve_task(
        &self,
        _change_id: &str,
        _task_id: &str,
        _reason: Option<String>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        Err(TaskMutationError::validation("unused in test"))
    }

    fn unshelve_task(
        &self,
        _change_id: &str,
        _task_id: &str,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        Err(TaskMutationError::validation("unused in test"))
    }

    fn add_task(
        &self,
        _change_id: &str,
        _title: &str,
        _wave: Option<u32>,
    ) -> TaskMutationServiceResult<TaskMutationResult> {
        Err(TaskMutationError::validation("unused in test"))
    }
}

struct FakeSpecRepo {
    specs: Vec<SpecDocument>,
}

impl FakeSpecRepo {
    fn new() -> Self {
        Self {
            specs: vec![
                SpecDocument {
                    id: "beta".to_string(),
                    path: PathBuf::from("remote/specs/beta/spec.md"),
                    markdown: "# Beta\n".to_string(),
                    last_modified: Utc::now(),
                },
                SpecDocument {
                    id: "alpha".to_string(),
                    path: PathBuf::from("remote/specs/alpha/spec.md"),
                    markdown: "# Alpha\n".to_string(),
                    last_modified: Utc::now(),
                },
            ],
        }
    }
}

impl SpecRepository for FakeSpecRepo {
    fn list(&self) -> DomainResult<Vec<SpecSummary>> {
        let mut specs: Vec<SpecSummary> = self
            .specs
            .iter()
            .map(|spec| SpecSummary {
                id: spec.id.clone(),
                path: spec.path.clone(),
                last_modified: spec.last_modified,
            })
            .collect();
        specs.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(specs)
    }

    fn get(&self, id: &str) -> DomainResult<SpecDocument> {
        self.specs
            .iter()
            .find(|spec| spec.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("spec", id))
    }
}
