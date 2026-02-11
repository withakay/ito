//! In-memory mock implementations of domain repository traits for unit testing.
//!
//! These mocks store data in `Vec`s and `HashMap`s, requiring no filesystem access.
//! They implement the domain traits from `ito-domain`, allowing tests to exercise
//! business logic without touching disk.

use std::collections::HashMap;

use chrono::Utc;
use ito_domain::changes::{
    Change, ChangeRepository, ChangeStatus, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
use ito_domain::tasks::{ProgressInfo, TaskRepository, TasksFormat, TasksParseResult};

// ---------------------------------------------------------------------------
// MockTaskRepository
// ---------------------------------------------------------------------------

/// In-memory task repository that returns pre-configured task parse results.
#[derive(Debug, Clone, Default)]
pub struct MockTaskRepository {
    tasks: HashMap<String, TasksParseResult>,
}

impl MockTaskRepository {
    /// Create a new empty mock task repository.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a pre-built `TasksParseResult` for a given change ID.
    pub fn with_tasks(mut self, change_id: &str, result: TasksParseResult) -> Self {
        self.tasks.insert(change_id.to_string(), result);
        self
    }
}

impl TaskRepository for MockTaskRepository {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        Ok(self
            .tasks
            .get(change_id)
            .cloned()
            .unwrap_or_else(TasksParseResult::empty))
    }
}

// ---------------------------------------------------------------------------
// MockChangeRepository
// ---------------------------------------------------------------------------

/// In-memory change repository backed by `Vec<Change>` and `Vec<ChangeSummary>`.
///
/// Configure via builder methods, then pass as `&impl ChangeRepository` to
/// business-logic functions under test.
#[derive(Debug, Clone, Default)]
pub struct MockChangeRepository {
    changes: HashMap<String, Change>,
    summaries: Vec<ChangeSummary>,
}

impl MockChangeRepository {
    /// Create a new empty mock change repository.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a full `Change` (also generates a `ChangeSummary` entry).
    pub fn with_change(mut self, change: Change) -> Self {
        let progress = &change.tasks.progress;
        let summary = ChangeSummary {
            id: change.id.clone(),
            module_id: change.module_id.clone(),
            completed_tasks: progress.complete as u32,
            shelved_tasks: progress.shelved as u32,
            in_progress_tasks: progress.in_progress as u32,
            pending_tasks: progress.pending as u32,
            total_tasks: progress.total as u32,
            last_modified: change.last_modified,
            has_proposal: change.proposal.is_some(),
            has_design: change.design.is_some(),
            has_specs: !change.specs.is_empty(),
            has_tasks: progress.total > 0,
        };
        self.summaries.push(summary);
        self.changes.insert(change.id.clone(), change);
        self
    }

    /// Register a standalone `ChangeSummary` without a full `Change`.
    pub fn with_summary(mut self, summary: ChangeSummary) -> Self {
        self.summaries.push(summary);
        self
    }
}

impl ChangeRepository for MockChangeRepository {
    fn resolve_target_with_options(
        &self,
        input: &str,
        _options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        let matches: Vec<&String> = self.changes.keys().filter(|k| k.contains(input)).collect();
        match matches.len() {
            0 => ChangeTargetResolution::NotFound,
            1 => ChangeTargetResolution::Unique(matches[0].clone()),
            _ => ChangeTargetResolution::Ambiguous(matches.into_iter().cloned().collect()),
        }
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        self.changes
            .keys()
            .filter(|k| k.contains(input))
            .take(max)
            .cloned()
            .collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.changes.contains_key(id)
    }

    fn get(&self, id: &str) -> DomainResult<Change> {
        self.changes
            .get(id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("change", id))
    }

    fn list(&self) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self.summaries.clone())
    }

    fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self
            .summaries
            .iter()
            .filter(|s| s.module_id.as_deref() == Some(module_id))
            .cloned()
            .collect())
    }

    fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self
            .summaries
            .iter()
            .filter(|s| {
                let status = s.status();
                match status {
                    ChangeStatus::Complete => false,
                    ChangeStatus::NoTasks => true,
                    ChangeStatus::InProgress => true,
                }
            })
            .cloned()
            .collect())
    }

    fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self
            .summaries
            .iter()
            .filter(|s| {
                let status = s.status();
                match status {
                    ChangeStatus::Complete => true,
                    ChangeStatus::NoTasks => false,
                    ChangeStatus::InProgress => false,
                }
            })
            .cloned()
            .collect())
    }

    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        self.summaries
            .iter()
            .find(|s| s.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("change", id))
    }
}

// ---------------------------------------------------------------------------
// MockModuleRepository
// ---------------------------------------------------------------------------

/// In-memory module repository.
#[derive(Debug, Clone, Default)]
pub struct MockModuleRepository {
    modules: HashMap<String, Module>,
    summaries: Vec<ModuleSummary>,
}

impl MockModuleRepository {
    /// Create a new empty mock module repository.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a `Module` (also generates a `ModuleSummary` with zero changes).
    pub fn with_module(mut self, module: Module) -> Self {
        let summary = ModuleSummary {
            id: module.id.clone(),
            name: module.name.clone(),
            change_count: 0,
        };
        self.summaries.push(summary);
        self.modules.insert(module.id.clone(), module);
        self
    }

    /// Register a `Module` with a specific change count.
    pub fn with_module_and_count(mut self, module: Module, change_count: u32) -> Self {
        let summary = ModuleSummary {
            id: module.id.clone(),
            name: module.name.clone(),
            change_count,
        };
        self.summaries.push(summary);
        self.modules.insert(module.id.clone(), module);
        self
    }
}

impl ModuleRepository for MockModuleRepository {
    fn exists(&self, id: &str) -> bool {
        self.modules.contains_key(id)
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        // Try by ID first, then by name
        if let Some(m) = self.modules.get(id_or_name) {
            return Ok(m.clone());
        }
        for m in self.modules.values() {
            if m.name == id_or_name {
                return Ok(m.clone());
            }
        }
        Err(DomainError::not_found("module", id_or_name))
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        Ok(self.summaries.clone())
    }
}

// ---------------------------------------------------------------------------
// Helpers for creating test domain objects
// ---------------------------------------------------------------------------

/// Create a minimal `Change` with sensible defaults for testing.
pub fn make_change(id: &str) -> Change {
    Change {
        id: id.to_string(),
        module_id: None,
        path: std::path::PathBuf::from(format!("/tmp/test/{id}")),
        proposal: None,
        design: None,
        specs: Vec::new(),
        tasks: TasksParseResult::empty(),
        last_modified: Utc::now(),
    }
}

/// Create a `Change` with a specific module and task progress.
pub fn make_change_with_progress(
    id: &str,
    module_id: Option<&str>,
    total: usize,
    complete: usize,
) -> Change {
    let mut change = make_change(id);
    change.module_id = module_id.map(String::from);
    change.tasks = TasksParseResult {
        format: TasksFormat::Checkbox,
        tasks: Vec::new(),
        waves: Vec::new(),
        diagnostics: Vec::new(),
        progress: ProgressInfo {
            total,
            complete,
            shelved: 0,
            in_progress: 0,
            pending: total.saturating_sub(complete),
            remaining: total.saturating_sub(complete),
        },
    };
    change
}

/// Create a minimal `ChangeSummary` for testing.
pub fn make_change_summary(id: &str) -> ChangeSummary {
    ChangeSummary {
        id: id.to_string(),
        module_id: None,
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 0,
        total_tasks: 0,
        last_modified: Utc::now(),
        has_proposal: false,
        has_design: false,
        has_specs: false,
        has_tasks: false,
    }
}

/// Create a minimal `Module` for testing.
pub fn make_module(id: &str, name: &str) -> Module {
    Module {
        id: id.to_string(),
        name: name.to_string(),
        description: None,
        path: std::path::PathBuf::from(format!("/tmp/test/modules/{id}")),
    }
}

/// Create a minimal `TasksParseResult` with given progress.
pub fn make_tasks_result(total: usize, complete: usize) -> TasksParseResult {
    TasksParseResult {
        format: TasksFormat::Checkbox,
        tasks: Vec::new(),
        waves: Vec::new(),
        diagnostics: Vec::new(),
        progress: ProgressInfo {
            total,
            complete,
            shelved: 0,
            in_progress: 0,
            pending: total.saturating_sub(complete),
            remaining: total.saturating_sub(complete),
        },
    }
}
