## MODIFIED Requirements

### Requirement: TaskRepository provides centralized task access

A `TaskRepository` interface SHALL exist in `ito-domain` that provides methods for loading and querying task data without exposing markdown parsing details.

`ito-core` SHALL provide a filesystem-backed implementation of this interface for production use.

When operating on a change, the `TaskRepository` MUST load task data from the change's resolved tracking file path (derived from the selected schema's `apply.tracks`, defaulting to `tasks.md`).

#### Scenario: Get task counts for a change

- **GIVEN** a change with tasks in either checkbox or enhanced format
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns a `(completed, total)` tuple with accurate counts
- **AND** both formats are correctly parsed

#### Scenario: Get task counts for missing tracking file

- **GIVEN** a change with no tracking file at its resolved tracking path
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns `(0, 0)`
