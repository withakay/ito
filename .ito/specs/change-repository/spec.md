## ADDED Requirements

### Requirement: ChangeRepository provides centralized change access

A `ChangeRepository` struct SHALL exist in `ito-workflow` that provides methods for loading and querying change data.

#### Scenario: Get a change by ID

- **GIVEN** a change with ID "005-01_my-change" exists
- **WHEN** calling `ChangeRepository::get("005-01_my-change")`
- **THEN** it returns a `Change` object with all artifacts loaded
- **AND** the `Change` includes proposal, design, specs, and tasks

#### Scenario: Get a non-existent change

- **GIVEN** no change with ID "999-99_nonexistent" exists
- **WHEN** calling `ChangeRepository::get("999-99_nonexistent")`
- **THEN** it returns an error indicating the change was not found

#### Scenario: List all changes

- **WHEN** calling `ChangeRepository::list()`
- **THEN** it returns a `Vec<ChangeSummary>` with all changes
- **AND** each summary includes id, module_id, task counts, and last modified time

#### Scenario: List changes by module

- **GIVEN** module "005" has 3 changes and module "003" has 2 changes
- **WHEN** calling `ChangeRepository::list_by_module("005")`
- **THEN** it returns only the 3 changes belonging to module "005"

#### Scenario: List incomplete changes

- **GIVEN** some changes have incomplete tasks
- **WHEN** calling `ChangeRepository::list_incomplete()`
- **THEN** it returns only changes where completed_tasks < total_tasks

### Requirement: Change domain model encapsulates artifacts

A `Change` struct SHALL encapsulate all change artifacts and provide computed properties.

#### Scenario: Change includes all artifacts

- **GIVEN** a change with proposal.md, design.md, specs/, and tasks.md
- **WHEN** the change is loaded via `ChangeRepository::get()`
- **THEN** `change.proposal` contains the parsed proposal
- **AND** `change.design` contains the parsed design
- **AND** `change.specs` contains a list of parsed specs
- **AND** `change.tasks` contains the `TasksParseResult`

#### Scenario: Change computes status

- **GIVEN** a change with 5 total tasks and 5 completed tasks
- **WHEN** accessing `change.status()`
- **THEN** it returns `ChangeStatus::Complete`

#### Scenario: Change computes artifact completeness

- **GIVEN** a change with proposal.md but no tasks.md
- **WHEN** accessing `change.artifacts_complete()`
- **THEN** it returns `false`

### Requirement: ChangeSummary provides lightweight listing

A `ChangeSummary` struct SHALL provide essential change info without loading full artifacts.

#### Scenario: Summary includes counts without full parse

- **WHEN** calling `ChangeRepository::list()`
- **THEN** each `ChangeSummary` includes:
  - `id: String`
  - `module_id: Option<String>`
  - `completed_tasks: u32`
  - `total_tasks: u32`
  - `last_modified: DateTime<Utc>`
  - `has_proposal: bool`
  - `has_design: bool`
  - `has_specs: bool`
  - `has_tasks: bool`
