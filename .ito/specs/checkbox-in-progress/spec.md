
# Checkbox In-Progress Specification

## Purpose

Define the `checkbox-in-progress` capability: how checkbox-format `tasks.md` represents and transitions in-progress tasks.

## Requirements

### Requirement: Checkbox format supports in-progress marker

The system SHALL recognize `- [~]` as an in-progress task marker in checkbox-format tasks.md files. The system SHALL also recognize `- [>]` as an alias for in-progress.

#### Scenario: Parse in-progress checkbox marker

- **WHEN** a tasks.md file contains `- [~] Task description`
- **THEN** the parser SHALL identify this task as having status "in-progress"

#### Scenario: Parse right-arrow in-progress checkbox marker

- **WHEN** a tasks.md file contains `- [>] Task description`
- **THEN** the parser SHALL identify this task as having status "in-progress"

#### Scenario: Parse mixed checkbox statuses

- **WHEN** a tasks.md file contains:
  ```
  - [ ] Pending task
  - [~] In-progress task
  - [x] Completed task
  ```
- **THEN** the parser SHALL correctly identify each task's status as pending, in-progress, and complete respectively

### Requirement: Checkbox format allows status transitions to in-progress

The system SHALL allow transitioning a checkbox task from pending (`- [ ]`) to in-progress (`- [~]`).

#### Scenario: Transition pending to in-progress

- **WHEN** a checkbox task has status pending (`- [ ]`)
- **AND** user requests to start the task
- **THEN** the system SHALL update the marker to `- [~]`

#### Scenario: Transition in-progress to complete

- **WHEN** a checkbox task has status in-progress (`- [~]`)
- **AND** user requests to complete the task
- **THEN** the system SHALL update the marker to `- [x]`

### Requirement: Only one task can be in-progress at a time

The system SHALL enforce that at most one checkbox task can have in-progress status at any given time.

#### Scenario: Starting new task when another is in-progress

- **WHEN** task A has status in-progress (`- [~]`)
- **AND** user requests to start task B
- **THEN** the system SHALL return an error indicating task A is already in progress
- **AND** SHALL NOT modify task B's status

#### Scenario: Starting task when none are in-progress

- **WHEN** no tasks have status in-progress
- **AND** user requests to start a pending task
- **THEN** the system SHALL update that task's marker to `- [~]`
