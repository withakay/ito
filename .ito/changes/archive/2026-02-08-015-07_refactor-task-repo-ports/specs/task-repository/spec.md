## MODIFIED Requirements

### Requirement: TaskRepository provides centralized task access

A `TaskRepository` interface SHALL exist in `ito-domain` that provides methods for loading and querying task data without exposing markdown parsing details.

`ito-core` SHALL provide a filesystem-backed implementation of this interface for production use.

#### Scenario: Get task counts for a change

- **GIVEN** a change with tasks in either checkbox or enhanced format
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns a `(completed, total)` tuple with accurate counts
- **AND** both formats are correctly parsed

#### Scenario: Get task counts for missing tasks file

- **GIVEN** a change with no tasks.md file
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns `(0, 0)`
