# Design: Change and Module Repository Pattern

## Overview

Implement repository pattern for Changes and Modules, building on the `TaskRepository` established in 005-11. This creates a clean data access layer that hides storage details from the rest of the application.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        ito-cli                            │
│  (list.rs, status.rs, validate.rs, archive.rs, show.rs)    │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     ito-workflow                          │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │ChangeRepository │  │ModuleRepository │  │TaskRepository│ │
│  └────────┬────────┘  └────────┬────────┘  └──────┬──────┘ │
│           │                    │                   │        │
│           ▼                    ▼                   ▼        │
│  ┌─────────────────────────────────────────────────────────┐│
│  │              Domain Models                              ││
│  │  Change, ChangeSummary, Module, ModuleSummary, Task     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     File System                             │
│  .ito/changes/*/proposal.md, design.md, tasks.md, specs/ │
│  .ito/modules/*/module.yaml                              │
└─────────────────────────────────────────────────────────────┘
```

## Domain Models

### Change

```rust
pub struct Change {
    pub id: String,
    pub module_id: Option<String>,
    pub path: PathBuf,
    pub proposal: Option<String>,      // Raw markdown content
    pub design: Option<String>,        // Raw markdown content
    pub specs: Vec<Spec>,
    pub tasks: TasksParseResult,
    pub last_modified: DateTime<Utc>,
}

impl Change {
    pub fn status(&self) -> ChangeStatus;
    pub fn artifacts_complete(&self) -> bool;
    pub fn task_progress(&self) -> (u32, u32);  // (completed, total)
}

pub enum ChangeStatus {
    NoTasks,
    InProgress,
    Complete,
}
```

### ChangeSummary

```rust
pub struct ChangeSummary {
    pub id: String,
    pub module_id: Option<String>,
    pub completed_tasks: u32,
    pub total_tasks: u32,
    pub last_modified: DateTime<Utc>,
    pub has_proposal: bool,
    pub has_design: bool,
    pub has_specs: bool,
    pub has_tasks: bool,
}
```

### Module

```rust
pub struct Module {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: PathBuf,
}

pub struct ModuleSummary {
    pub id: String,
    pub name: String,
    pub change_count: u32,
}
```

## Repository APIs

### ChangeRepository

```rust
pub struct ChangeRepository<'a> {
    ito_path: &'a Path,
    task_repo: TaskRepository<'a>,
}

impl<'a> ChangeRepository<'a> {
    pub fn new(ito_path: &'a Path) -> Self;

    /// Get full change with all artifacts loaded
    pub fn get(&self, id: &str) -> Result<Change>;

    /// List all changes as summaries (lightweight)
    pub fn list(&self) -> Result<Vec<ChangeSummary>>;

    /// List changes belonging to a module
    pub fn list_by_module(&self, module_id: &str) -> Result<Vec<ChangeSummary>>;

    /// List changes with incomplete tasks
    pub fn list_incomplete(&self) -> Result<Vec<ChangeSummary>>;

    /// List changes with complete tasks
    pub fn list_complete(&self) -> Result<Vec<ChangeSummary>>;

    /// Check if a change exists
    pub fn exists(&self, id: &str) -> bool;
}
```

### ModuleRepository

```rust
pub struct ModuleRepository<'a> {
    ito_path: &'a Path,
}

impl<'a> ModuleRepository<'a> {
    pub fn new(ito_path: &'a Path) -> Self;

    /// Get a module by ID
    pub fn get(&self, id: &str) -> Result<Module>;

    /// List all modules
    pub fn list(&self) -> Result<Vec<ModuleSummary>>;

    /// Get module with its changes
    pub fn get_with_changes(&self, id: &str) -> Result<(Module, Vec<ChangeSummary>)>;

    /// Check if a module exists
    pub fn exists(&self, id: &str) -> bool;
}
```

## Migration Strategy

### Phase 1: Core Implementation
1. Create domain models in `ito-workflow/src/changes/mod.rs`
2. Create `ChangeRepository` in `ito-workflow/src/changes/repository.rs`
3. Create `ModuleRepository` in `ito-workflow/src/modules/repository.rs`
4. Add comprehensive unit tests

### Phase 2: CLI Migration
1. Migrate `list.rs` to use `ChangeRepository::list()` and `ModuleRepository::list()`
2. Migrate `status.rs` to use `ChangeRepository::get()`
3. Migrate `validate.rs` to use `ChangeRepository::list()`
4. Migrate remaining commands

### Phase 3: Cleanup
1. Remove direct path construction from CLI commands
2. Remove duplicated loading logic
3. Deprecate/remove low-level path helpers if no longer needed

## Integration with TaskRepository

`ChangeRepository` will internally use `TaskRepository` to load task data:

```rust
impl<'a> ChangeRepository<'a> {
    pub fn new(ito_path: &'a Path) -> Self {
        Self {
            ito_path,
            task_repo: TaskRepository::new(ito_path),
        }
    }

    fn load_tasks(&self, change_id: &str) -> TasksParseResult {
        self.task_repo.load_tasks(change_id).unwrap_or_else(|_| TasksParseResult::empty())
    }
}
```

## File Organization

```
ito-workflow/src/
  changes/
    mod.rs           # Domain models: Change, ChangeSummary, ChangeStatus
    repository.rs    # ChangeRepository implementation
  modules/
    mod.rs           # Domain models: Module, ModuleSummary
    repository.rs    # ModuleRepository implementation
  tasks/
    mod.rs           # (existing)
    repository.rs    # TaskRepository (existing from 005-11)
    parse.rs         # (existing)
```

## Testing

- Unit tests for each repository method
- Integration tests loading real change data
- Tests for edge cases (missing files, malformed content)
- Tests for computed properties (status, completeness)
