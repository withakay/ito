use std::path::Path;
use std::sync::Arc;

use chrono::Utc;
use tempfile::TempDir;

use ito_core::backend_client::BackendRuntime;
use ito_core::change_repository::FsChangeRepository;
use ito_core::list::{list_changes, ChangeProgressFilter, ChangeSortOrder, ListChangesInput};
use ito_core::repository_runtime::{
    PersistenceMode, RemoteRepositoryFactory, RepositoryRuntimeBuilder, RepositorySet,
};
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions, Spec,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{ModuleRepository, ModuleSummary};
use ito_domain::tasks::{
    TaskInitResult, TaskMutationError, TaskMutationResult, TaskMutationService,
    TaskMutationServiceResult, TaskRepository, TasksParseResult,
};

fn write_change_dir(ito_path: &Path, id: &str) {
    let change_dir = ito_path.join("changes").join(id);
    std::fs::create_dir_all(&change_dir).expect("change dir");
    std::fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("proposal");
    std::fs::write(change_dir.join("tasks.md"), "- [ ] task\n").expect("tasks");
    std::fs::create_dir_all(change_dir.join("specs/alpha")).expect("specs");
    std::fs::write(change_dir.join("specs/alpha/spec.md"), "## ADDED\n").expect("spec");
}

fn write_archived_change_dir(ito_path: &Path, id: &str) {
    let archive_dir = ito_path.join("changes").join("archive").join(id);
    std::fs::create_dir_all(&archive_dir).expect("archive dir");
    std::fs::write(archive_dir.join("proposal.md"), "# Archived\n").expect("archived proposal");
}

#[test]
fn filesystem_change_repository_filters_archived() {
    let tmp = TempDir::new().expect("temp dir");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes")).expect("changes dir");

    write_change_dir(&ito_path, "001-01_active");
    write_archived_change_dir(&ito_path, "2026-03-01-001-02_archived");

    let repo = FsChangeRepository::new(&ito_path);

    let active_ids: Vec<String> = repo
        .list_with_filter(ChangeLifecycleFilter::Active)
        .expect("active list")
        .into_iter()
        .map(|c| c.id)
        .collect();
    assert_eq!(active_ids, vec!["001-01_active".to_string()]);

    let archived_ids: Vec<String> = repo
        .list_with_filter(ChangeLifecycleFilter::Archived)
        .expect("archived list")
        .into_iter()
        .map(|c| c.id)
        .collect();
    assert_eq!(archived_ids, vec!["001-02_archived".to_string()]);

    let all_ids: Vec<String> = repo
        .list_with_filter(ChangeLifecycleFilter::All)
        .expect("all list")
        .into_iter()
        .map(|c| c.id)
        .collect();
    assert_eq!(
        all_ids,
        vec!["001-01_active".to_string(), "001-02_archived".to_string()]
    );
}

struct FakeRemoteChangeRepo {
    summaries: Vec<ChangeSummary>,
}

impl FakeRemoteChangeRepo {
    fn new(summary: ChangeSummary) -> Self {
        Self {
            summaries: vec![summary],
        }
    }
}

impl ChangeRepository for FakeRemoteChangeRepo {
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

    fn exists_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> bool {
        if !filter.includes_active() {
            return false;
        }
        self.summaries.iter().any(|s| s.id == id)
    }

    fn get_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        if !filter.includes_active() {
            return Err(DomainError::not_found("change", id));
        }
        Err(DomainError::not_found("change", id))
    }

    fn list_with_filter(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self.summaries.clone())
    }

    fn list_by_module_with_filter(
        &self,
        _module_id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self.summaries.clone())
    }

    fn list_incomplete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(self.summaries.clone())
    }

    fn list_complete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        if !filter.includes_active() {
            return Ok(Vec::new());
        }
        Ok(Vec::new())
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ChangeLifecycleFilter,
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

struct FakeModuleRepo;

impl ModuleRepository for FakeModuleRepo {
    fn exists(&self, _id: &str) -> bool {
        false
    }

    fn get(&self, id_or_name: &str) -> DomainResult<ito_domain::modules::Module> {
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

struct FakeRemoteFactory {
    repos: RepositorySet,
}

impl RemoteRepositoryFactory for FakeRemoteFactory {
    fn build(&self, _runtime: &BackendRuntime) -> ito_core::errors::CoreResult<RepositorySet> {
        Ok(self.repos.clone())
    }
}

#[test]
fn remote_runtime_ignores_local_change_dirs() {
    let tmp = TempDir::new().expect("temp repo");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/000-01_local")).expect("local change");

    let summary = ChangeSummary {
        id: "090-01_remote".to_string(),
        module_id: Some("090".to_string()),
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 1,
        total_tasks: 1,
        last_modified: Utc::now(),
        has_proposal: true,
        has_design: false,
        has_specs: true,
        has_tasks: true,
    };
    let repos = RepositorySet {
        changes: Arc::new(FakeRemoteChangeRepo::new(summary)),
        modules: Arc::new(FakeModuleRepo),
        tasks: Arc::new(FakeTaskRepo),
        task_mutations: Arc::new(FakeTaskMutations),
    };
    let backend_runtime = BackendRuntime {
        base_url: "http://127.0.0.1:9010".to_string(),
        token: "test".to_string(),
        timeout: std::time::Duration::from_secs(1),
        max_retries: 1,
        backup_dir: tmp.path().join("backups"),
        org: "acme".to_string(),
        repo: "widgets".to_string(),
    };

    let runtime = RepositoryRuntimeBuilder::new(&ito_path)
        .mode(PersistenceMode::Remote)
        .backend_runtime(backend_runtime)
        .remote_factory(Arc::new(FakeRemoteFactory { repos }))
        .build()
        .expect("remote runtime");

    let summaries = list_changes(
        runtime.repositories().changes.as_ref(),
        ListChangesInput {
            progress_filter: ChangeProgressFilter::All,
            sort: ChangeSortOrder::Name,
        },
    )
    .expect("list remote changes");

    let ids: Vec<String> = summaries.into_iter().map(|s| s.name).collect();
    assert_eq!(ids, vec!["090-01_remote".to_string()]);
}
