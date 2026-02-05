# Tasks: Task Repository Pattern

## Implementation

- [x] Create `TaskRepository` struct in `ito-workflow/src/tasks/repository.rs`
- [x] Add `TasksParseResult::empty()` method for missing files
- [x] Export `TaskRepository` from `ito-workflow/src/tasks/mod.rs`
- [x] Add `miette` dependency to `ito-workflow/Cargo.toml`

## Migration

- [x] Update `ito-cli/src/app/list.rs` to use `TaskRepository::get_task_counts()`
- [x] Remove `count_tasks_markdown()` from `ito-core/src/list.rs`
- [x] Remove associated test for `count_tasks_markdown()`

## Validation

- [x] Unit tests pass for TaskRepository (checkbox and enhanced formats)
- [x] `ito list` correctly shows "3/4 tasks" for `013-18_cleanup-ito-skills-repo`
- [x] All existing tests pass
