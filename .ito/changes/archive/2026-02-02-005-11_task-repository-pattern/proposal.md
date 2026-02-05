# Change: Task Repository Pattern

## Why

The `ito list` command was showing "No tasks" for changes using the enhanced task format (e.g., `013-18_cleanup-ito-skills-repo`). This happened because task counting was duplicated across the codebase with inconsistent implementations:

- `ito-core/src/list.rs` had `count_tasks_markdown()` that only recognized checkbox format (`- [ ]`, `- [x]`)
- `ito-workflow/src/tasks/parse.rs` had the authoritative parser supporting both formats

This violated DRY and led to bugs when different parts of the codebase used different parsing logic.

## What Changes

- Implement a `TaskRepository` pattern in `ito-workflow` that centralizes all task loading
- Migrate `ito list` to use `TaskRepository` instead of direct markdown parsing
- Remove the duplicate `count_tasks_markdown()` function from `ito-core`
- Hide markdown storage format as an implementation detail

## Capabilities

### New Capabilities

- `task-repository`: Clean abstraction (`TaskRepository`) over task storage that provides `get_task_counts()`, `get_progress()`, `has_tasks()`, and `get_tasks()` methods, hiding markdown parsing from consumers

### Modified Capabilities

- `list-command`: Now uses `TaskRepository` to correctly count tasks in both checkbox and enhanced formats

## Impact

- **Bug Fix**: `ito list` now correctly shows task counts for enhanced format (e.g., "3/4 tasks" instead of "No tasks")
- **Architecture**: Establishes repository pattern for task data access
- **Code Quality**: Removes duplicate parsing code, single source of truth for task parsing
