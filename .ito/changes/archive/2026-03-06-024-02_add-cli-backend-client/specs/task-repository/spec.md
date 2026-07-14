## ADDED Requirements

### Requirement: TaskRepository supports backend-backed task access

`TaskRepository` SHALL support backend-backed task access when backend mode is enabled.

#### Scenario: Task counts resolve from backend tasks artifact in backend mode

- **GIVEN** backend mode is enabled and backend provides tasks markdown for a change
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** Ito computes counts from backend-sourced task content

#### Scenario: Missing backend tasks artifact returns zero counts

- **GIVEN** backend mode is enabled and no tasks artifact exists for a change
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** it returns `(0, 0)`

#### Scenario: Filesystem path is used when backend mode is disabled

- **GIVEN** backend mode is disabled
- **WHEN** calling `task_repo.get_task_counts(change_id)`
- **THEN** Ito uses existing filesystem-backed behavior
