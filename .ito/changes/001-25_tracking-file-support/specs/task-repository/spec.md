## MODIFIED Requirements

### Requirement: TaskRepository provides centralized task access

A `TaskRepository` interface SHALL exist in `ito-domain` that provides methods for loading and querying task data without exposing markdown parsing details.

`ito-core` SHALL provide a filesystem-backed implementation of this interface for production use.

The filesystem-backed implementation MUST load tasks from the resolved tracking file path for the change (schema `apply.tracks` if present, otherwise `tasks.md`).

#### Scenario: Get task counts for a change

- **GIVEN** a change with tasks in either checkbox or enhanced format
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns a `(completed, total)` tuple with accurate counts
- **AND** both formats are correctly parsed

#### Scenario: Get task counts uses schema-selected tracking file

- **GIVEN** a change whose schema declares `apply.tracks: todo.md`
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it reads tasks from `todo.md`
- **AND** it does not require `tasks.md` to exist

#### Scenario: Get task counts for missing tracking file

- **GIVEN** a change with no tracking file present at the resolved path
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns `(0, 0)`
