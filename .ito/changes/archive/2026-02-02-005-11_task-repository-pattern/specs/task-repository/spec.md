## ADDED Requirements

### Requirement: TaskRepository provides centralized task access

A `TaskRepository` struct SHALL exist in `ito-domain` that provides methods for loading and querying task data without exposing markdown parsing details.

#### Scenario: Get task counts for a change

- **GIVEN** a change with tasks in either checkbox or enhanced format
- **WHEN** calling `TaskRepository::get_task_counts(change_id)`
- **THEN** it returns `(completed, total)` tuple with accurate counts
- **AND** both formats are correctly parsed

#### Scenario: Get task counts for missing tasks file

- **GIVEN** a change with no tasks.md file
- **WHEN** calling `TaskRepository::get_task_counts(change_id)`
- **THEN** it returns `(0, 0)`

### Requirement: List command uses TaskRepository

The `ito list` command SHALL use `TaskRepository` for task counting instead of direct markdown parsing.

#### Scenario: List shows enhanced format task counts

- **GIVEN** a change using enhanced task format with 3 complete and 1 pending task
- **WHEN** running `ito list`
- **THEN** the output shows "3/4 tasks" for that change

## REMOVED Requirements

### Requirement: Duplicate task counting in ito-core
The `count_tasks_markdown()` function SHALL be removed from `ito-core/src/list.rs` as it duplicates functionality in `ito-domain` and only supports checkbox format.

#### Scenario: Duplicate counter removed

- **GIVEN** the task repository is used for task counting
- **WHEN** building the workspace or running `ito list`
- **THEN** there is no dependency on `count_tasks_markdown()`
