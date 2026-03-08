use std::path::Path;
use std::sync::Arc;

use chrono::Utc;
use tempfile::TempDir;

use ito_core::backend_client::BackendRuntime;
use ito_core::errors::CoreResult;
use ito_core::list::{
    ChangeProgressFilter, ChangeSortOrder, ListChangesInput, list_changes, list_modules,
};
use ito_core::repository_runtime::{
    PersistenceMode, RemoteRepositoryFactory, RepositoryRuntimeBuilder, RepositorySet,
};
use ito_core::task_mutations::{TaskInitResult, TaskMutationResult, TaskMutationService};
use ito_domain::changes::{
    Change, ChangeRepository, ChangeSummary, ChangeTargetResolution, ResolveTargetOptions, Spec,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
use ito_domain::tasks::{TaskRepository, TasksParseResult};

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dirs should exist");
    }
    std::fs::write(path, contents).expect("test fixture should write");
}

fn make_change(root: &Path, id: &str) {
    write(
        root.join(".ito/changes").join(id).join("proposal.md"),
        "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
    );
    write(
        root.join(".ito/changes").join(id).join("tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 todo\n",
    );
    write(
        root.join(".ito/changes")
            .join(id)
            .join("specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
    );
}

fn make_module(root: &Path, id: &str, name: &str) {
    let module_dir = root.join(".ito/modules").join(format!("{id}_{name}"));
    std::fs::create_dir_all(&module_dir).expect("module dir");
}

#[test]
fn filesystem_runtime_builds_repository_set() {
    let repo = TempDir::new().expect("temp repo");
    let ito_path = repo.path().join(".ito");
    make_change(repo.path(), "000-01_alpha");
    make_module(repo.path(), "001", "demo");

    let runtime = RepositoryRuntimeBuilder::new(&ito_path)
        .build()
        .expect("filesystem runtime");
    assert_eq!(runtime.mode(), PersistenceMode::Filesystem);

    let repos = runtime.repositories();
    let changes = list_changes(
        repos.changes.as_ref(),
        ListChangesInput {
            progress_filter: ChangeProgressFilter::All,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("list changes");
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].name, "000-01_alpha");

    let modules = list_modules(repos.modules.as_ref()).expect("list modules");
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].id, "001");
}

struct FakeChangeRepo {
    summaries: Vec<ChangeSummary>,
    full: Vec<Change>,
}

impl FakeChangeRepo {
    fn new(summary: ChangeSummary, change: Change) -> Self {
        Self {
            summaries: vec![summary],
            full: vec![change],
        }
    }
}

impl ChangeRepository for FakeChangeRepo {
    fn resolve_target_with_options(
        &self,
        input: &str,
        _options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        if self.summaries.iter().any(|s| s.id == input) {
            return ChangeTargetResolution::Unique(input.to_string());
        }
        ChangeTargetResolution::NotFound
    }

    fn suggest_targets(&self, _input: &str, _max: usize) -> Vec<String> {
        Vec::new()
    }

    fn exists(&self, id: &str) -> bool {
        self.summaries.iter().any(|s| s.id == id)
    }

    fn exists_with_filter(
        &self,
        id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> bool {
        if !filter.includes_active() {
            return false;
        }
        self.summaries.iter().any(|s| s.id == id)
    }

    fn get_with_filter(
        &self,
        id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Change> {
        if !filter.includes_active() {
            return Err(DomainError::not_found("change", id));
        }
        self.full
            .iter()
            .find(|c| c.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("change", id))
    }

    fn list_with_filter(
        &self,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self.summaries.clone())
    }

    fn list_by_module_with_filter(
        &self,
        module_id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self
            .summaries
            .iter()
            .filter(|s| s.module_id.as_deref() == Some(module_id))
            .cloned()
            .collect())
    }

    fn list_incomplete_with_filter(
        &self,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self
            .summaries
            .iter()
            .filter(|s| s.total_tasks == 0 || s.completed_tasks < s.total_tasks)
            .cloned()
            .collect())
    }

    fn list_complete_with_filter(
        &self,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self
            .summaries
            .iter()
            .filter(|s| s.total_tasks > 0 && s.completed_tasks == s.total_tasks)
            .cloned()
            .collect())
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<ChangeSummary> {
        if !filter.includes_active() {
            return Err(DomainError::not_found("change", id));
        }
        self.summaries
            .iter()
            .find(|s| s.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("change", id))
    }
}

struct FakeModuleRepo {
    modules: Vec<ModuleSummary>,
    full: Vec<Module>,
}

impl FakeModuleRepo {
    fn new(summary: ModuleSummary, module: Module) -> Self {
        Self {
            modules: vec![summary],
            full: vec![module],
        }
    }
}

impl ModuleRepository for FakeModuleRepo {
    fn exists(&self, id: &str) -> bool {
        self.modules.iter().any(|m| m.id == id)
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        self.full
            .iter()
            .find(|m| m.id == id_or_name || m.name == id_or_name)
            .cloned()
            .ok_or_else(|| DomainError::not_found("module", id_or_name))
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        Ok(self.modules.clone())
    }
}

struct FakeTaskRepo;

impl TaskRepository for FakeTaskRepo {
    fn load_tasks(&self, _change_id: &str) -> DomainResult<TasksParseResult> {
        Ok(TasksParseResult::empty())
    }
}

struct FakeTaskMutations;

impl FakeTaskMutations {
    fn unsupported<T>() -> CoreResult<T> {
        Err(ito_core::errors::CoreError::process(
            "task mutations not configured",
        ))
    }
}

impl TaskMutationService for FakeTaskMutations {
    fn load_tasks_markdown(&self, _change_id: &str) -> CoreResult<Option<String>> {
        Ok(None)
    }

    fn init_tasks(&self, _change_id: &str) -> CoreResult<TaskInitResult> {
        Self::unsupported()
    }

    fn start_task(&self, _change_id: &str, _task_id: &str) -> CoreResult<TaskMutationResult> {
        Self::unsupported()
    }

    fn complete_task(
        &self,
        _change_id: &str,
        _task_id: &str,
        _note: Option<String>,
    ) -> CoreResult<TaskMutationResult> {
        Self::unsupported()
    }

    fn shelve_task(
        &self,
        _change_id: &str,
        _task_id: &str,
        _reason: Option<String>,
    ) -> CoreResult<TaskMutationResult> {
        Self::unsupported()
    }

    fn unshelve_task(&self, _change_id: &str, _task_id: &str) -> CoreResult<TaskMutationResult> {
        Self::unsupported()
    }

    fn add_task(
        &self,
        _change_id: &str,
        _title: &str,
        _wave: Option<u32>,
    ) -> CoreResult<TaskMutationResult> {
        Self::unsupported()
    }
}

struct FakeRemoteFactory {
    repos: RepositorySet,
}

impl RemoteRepositoryFactory for FakeRemoteFactory {
    fn build(&self, _runtime: &BackendRuntime) -> CoreResult<RepositorySet> {
        Ok(self.repos.clone())
    }
}

#[test]
fn remote_runtime_uses_remote_factory() {
    let repo = TempDir::new().expect("temp repo");
    let ito_path = repo.path().join(".ito");

    let summary = ChangeSummary {
        id: "025-04_demo".to_string(),
        module_id: Some("025".to_string()),
        completed_tasks: 1,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 0,
        total_tasks: 1,
        last_modified: Utc::now(),
        has_proposal: true,
        has_design: false,
        has_specs: true,
        has_tasks: true,
    };
    let change = Change {
        id: "025-04_demo".to_string(),
        module_id: Some("025".to_string()),
        path: std::path::PathBuf::new(),
        proposal: Some("# Proposal".to_string()),
        design: None,
        specs: vec![Spec {
            name: "repository-runtime-selection".to_string(),
            content: "## ADDED".to_string(),
        }],
        tasks: TasksParseResult::empty(),
        last_modified: Utc::now(),
    };
    let module_summary = ModuleSummary {
        id: "025".to_string(),
        name: "repository-backends".to_string(),
        change_count: 1,
    };
    let module = Module {
        id: "025".to_string(),
        name: "repository-backends".to_string(),
        description: Some("Demo".to_string()),
        path: std::path::PathBuf::new(),
    };

    let repos = RepositorySet {
        changes: Arc::new(FakeChangeRepo::new(summary, change)),
        modules: Arc::new(FakeModuleRepo::new(module_summary, module)),
        tasks: Arc::new(FakeTaskRepo),
        task_mutations: Arc::new(FakeTaskMutations),
    };
    let backend_runtime = BackendRuntime {
        base_url: "http://127.0.0.1:9010".to_string(),
        token: "test".to_string(),
        timeout: std::time::Duration::from_secs(1),
        max_retries: 1,
        backup_dir: repo.path().join("backups"),
        org: "acme".to_string(),
        repo: "widgets".to_string(),
    };
    let factory = FakeRemoteFactory { repos };

    let runtime = RepositoryRuntimeBuilder::new(&ito_path)
        .mode(PersistenceMode::Remote)
        .backend_runtime(backend_runtime)
        .remote_factory(Arc::new(factory))
        .build()
        .expect("remote runtime");

    assert_eq!(runtime.mode(), PersistenceMode::Remote);
    let repos = runtime.repositories();
    let summaries = list_changes(
        repos.changes.as_ref(),
        ListChangesInput {
            progress_filter: ChangeProgressFilter::All,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("list remote changes");
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].name, "025-04_demo");
    let modules = list_modules(repos.modules.as_ref()).expect("list remote modules");
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].id, "025");
}
