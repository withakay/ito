# Design: Task Repository Pattern

## Overview

Implement a repository pattern for task data access, centralizing all task loading through a single abstraction that hides the markdown storage format.

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   ito-cli     │────▶│  TaskRepository  │────▶│  parse.rs       │
│   (list.rs)     │     │  (repository.rs) │     │  (authoritative │
└─────────────────┘     └──────────────────┘     │   parser)       │
                                                  └─────────────────┘
```

Before: `ito-cli` → `count_tasks_markdown()` (broken for enhanced format)
After: `ito-cli` → `TaskRepository` → `parse_tasks_tracking_file()` (handles both formats)

## Implementation

### TaskRepository API

```rust
pub struct TaskRepository<'a> {
    ito_path: &'a Path,
}

impl<'a> TaskRepository<'a> {
    pub fn new(ito_path: &'a Path) -> Self;

    /// Load full task parse result
    pub fn load_tasks(&self, change_id: &str) -> Result<TasksParseResult>;

    /// Get progress info (total, complete, in_progress, etc.)
    pub fn get_progress(&self, change_id: &str) -> Result<ProgressInfo>;

    /// Get (completed, total) counts
    pub fn get_task_counts(&self, change_id: &str) -> Result<(u32, u32)>;

    /// Check if change has any tasks
    pub fn has_tasks(&self, change_id: &str) -> Result<bool>;

    /// Get all task items
    pub fn get_tasks(&self, change_id: &str) -> Result<Vec<TaskItem>>;
}
```

### Integration with list command

```rust
// Before (ito-cli/src/app/list.rs)
let (total, completed) = ito_core::list::count_tasks_markdown(&contents);

// After
let task_repo = TaskRepository::new(ito_path);
let (completed, total) = task_repo.get_task_counts(name).unwrap_or((0, 0));
```

## Files Changed

| File | Change |
|------|--------|
| `ito-workflow/src/tasks/repository.rs` | New file - TaskRepository implementation |
| `ito-workflow/src/tasks/parse.rs` | Add `TasksParseResult::empty()` |
| `ito-workflow/src/tasks/mod.rs` | Export TaskRepository |
| `ito-workflow/Cargo.toml` | Add miette dependency |
| `ito-cli/src/app/list.rs` | Use TaskRepository |
| `ito-core/src/list.rs` | Remove count_tasks_markdown() |

## Testing

- Unit tests in `repository.rs` for both checkbox and enhanced formats
- Integration test via `ito list` showing correct counts for `013-18`
