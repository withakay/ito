# Task Repository Specification

## Purpose

Define the `task-repository` capability: a single, authoritative way to load and query tasks from a change's `tasks.md`, supporting both checkbox and enhanced task formats.

## Requirements

### Requirement: TaskRepository provides centralized task access

A TaskRepository component SHALL exist that provides methods for loading and querying task data without exposing markdown parsing details.

#### Scenario: Get task counts for a change

- **GIVEN** a change with tasks in either checkbox or enhanced format
- **WHEN** calling `TaskRepository::get_task_counts(change_id)`
- **THEN** it returns a `(completed, total)` tuple with accurate counts
- **AND** both formats are correctly parsed

#### Scenario: Get task counts for missing tasks file

- **GIVEN** a change with no tasks.md file
- **WHEN** calling `TaskRepository::get_task_counts(change_id)`
- **THEN** it returns `(0, 0)`

### Requirement: TaskRepository supports checkbox and enhanced formats

TaskRepository SHALL support both checkbox tasks (`- [ ]`, `- [x]`) and enhanced task status fields.

#### Scenario: Checkbox format progress is computed

- **GIVEN** a tasks.md containing checkbox tasks
- **WHEN** TaskRepository parses the file
- **THEN** progress totals and completion counts match the checkboxes

#### Scenario: Enhanced format progress is computed

- **GIVEN** a tasks.md containing enhanced tasks with `- **Status**: [x] complete` and `- **Status**: [ ] pending`
- **WHEN** TaskRepository parses the file
- **THEN** progress totals and completion counts match the enhanced statuses

### Requirement: CLI uses TaskRepository for task counting

CLI commands that display task counts (e.g., `ito list`) SHALL use TaskRepository rather than ad-hoc markdown parsing.

#### Scenario: List shows enhanced format task counts

- **GIVEN** a change using enhanced task format with 3 complete and 1 pending task
- **WHEN** running `ito list`
- **THEN** the output shows "3/4 tasks" for that change

#### Scenario: No legacy task counter is required

- **GIVEN** task counting is performed via TaskRepository
- **WHEN** building the workspace or running `ito list`
- **THEN** task counting does not depend on any legacy markdown counter helper
