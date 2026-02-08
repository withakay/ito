## MODIFIED Requirements

### Requirement: ChangeRepository provides centralized change access

A `ChangeRepository` interface SHALL exist in `ito-domain` that provides methods for loading and querying change data.

`ito-core` SHALL provide a filesystem-backed implementation of this interface for production use.

#### Scenario: Get a change by ID

- **GIVEN** a change with ID "005-01_my-change" exists
- **WHEN** calling `change_repo.get("005-01_my-change")`
- **THEN** it returns a `Change` object with all artifacts loaded
- **AND** the `Change` includes proposal, design, specs, and tasks

#### Scenario: Get a non-existent change

- **GIVEN** no change with ID "999-99_nonexistent" exists
- **WHEN** calling `change_repo.get("999-99_nonexistent")`
- **THEN** it returns an error indicating the change was not found

#### Scenario: List all changes

- **WHEN** calling `change_repo.list()`
- **THEN** it returns a `Vec<ChangeSummary>` with all changes
- **AND** each summary includes id, module_id, task counts, and last modified time

#### Scenario: List changes by module

- **GIVEN** module "005" has 3 changes and module "003" has 2 changes
- **WHEN** calling `change_repo.list_by_module("005")`
- **THEN** it returns only the 3 changes belonging to module "005"

#### Scenario: List incomplete changes

- **GIVEN** some changes have incomplete tasks
- **WHEN** calling `change_repo.list_incomplete()`
- **THEN** it returns only changes where completed_tasks < total_tasks
