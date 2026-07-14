# Spec: Ready Work Filter

Filter and display work items (changes and tasks) that are ready for implementation.

## ADDED Requirements

### Requirement: List ready changes

The system SHALL provide a `--ready` flag on the `ito list` command that filters to show only changes that are ready for implementation.

A change is considered "ready" when ALL of the following are true:
- The change has a proposal (`has_proposal` is true)
- The change has specs (`has_specs` is true)
- The change has tasks (`has_tasks` is true)
- The change status is `InProgress` (has pending tasks remaining)

#### Scenario: List with ready flag shows only ready changes

- **WHEN** user runs `ito list --ready`
- **THEN** the output SHALL include only changes where proposal, specs, and tasks exist AND status is InProgress
- **AND** changes without proposals, specs, or tasks SHALL be excluded
- **AND** completed changes (all tasks done) SHALL be excluded

#### Scenario: List with ready flag and no ready changes

- **WHEN** user runs `ito list --ready`
- **AND** no changes meet the ready criteria
- **THEN** the output SHALL display an empty list or appropriate message

#### Scenario: Ready flag combines with other list options

- **WHEN** user runs `ito list --ready --json`
- **THEN** the output SHALL be JSON formatted
- **AND** SHALL contain only ready changes

### Requirement: Show ready tasks for a specific change

The system SHALL provide a `ito tasks ready <CHANGE>` subcommand that displays tasks ready to be worked on for a specific change.

A task is considered "ready" when:
- The task status is pending (not started, not completed, not shelved)
- The task is in the earliest incomplete wave

#### Scenario: Show ready tasks for a change with pending tasks

- **WHEN** user runs `ito tasks ready 000-01_my-change`
- **AND** the change has pending tasks in wave 1
- **THEN** the output SHALL list all pending tasks from wave 1
- **AND** tasks from later waves SHALL NOT be shown

#### Scenario: Show ready tasks when current wave is complete

- **WHEN** user runs `ito tasks ready 000-01_my-change`
- **AND** wave 1 is fully complete but wave 2 has pending tasks
- **THEN** the output SHALL list pending tasks from wave 2

#### Scenario: Show ready tasks for a change with no pending tasks

- **WHEN** user runs `ito tasks ready 000-01_my-change`
- **AND** all tasks are complete
- **THEN** the output SHALL indicate no ready tasks

#### Scenario: Show ready tasks for non-existent change

- **WHEN** user runs `ito tasks ready non-existent-change`
- **THEN** the command SHALL exit with an error
- **AND** display an appropriate error message

### Requirement: Show ready tasks across all changes

The system SHALL allow `ito tasks ready` without a change argument to display ready tasks across all changes.

#### Scenario: Show all ready tasks without change argument

- **WHEN** user runs `ito tasks ready`
- **THEN** the output SHALL list ready tasks from all changes that have pending work
- **AND** tasks SHALL be grouped by change

#### Scenario: Show all ready tasks with no pending work

- **WHEN** user runs `ito tasks ready`
- **AND** no changes have pending tasks
- **THEN** the output SHALL indicate no ready tasks across any changes

### Requirement: Ready tasks JSON output

The system SHALL support `--json` flag for `ito tasks ready` to output machine-readable format.

#### Scenario: Ready tasks JSON output for single change

- **WHEN** user runs `ito tasks ready 000-01_my-change --json`
- **THEN** the output SHALL be valid JSON
- **AND** SHALL include task IDs, descriptions, and wave numbers

#### Scenario: Ready tasks JSON output for all changes

- **WHEN** user runs `ito tasks ready --json`
- **THEN** the output SHALL be valid JSON
- **AND** SHALL include change IDs with their respective ready tasks
