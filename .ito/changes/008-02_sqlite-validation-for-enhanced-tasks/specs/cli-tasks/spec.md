## MODIFIED Requirements

### Requirement: Tasks initialization

The CLI SHALL initialize an enhanced tasks.md file in a change directory with structured format for waves, verification, and status tracking.

#### Scenario: Initialize tasks for a change

- **WHEN** executing `ito tasks init <change-id>`
- **THEN** create `.ito/changes/<change-id>/tasks.md` if it does not exist
- **AND** generate the enhanced tasks format with:
  - Header section with change ID, tool compatibility notes, and execution mode
  - Wave sections that include an explicit `Depends On` line
  - Example tasks that demonstrate within-wave dependencies only
  - Instructions for verification commands, done-when criteria, and status tracking
- **AND** display a success message with the path to the tasks file
- **AND** print guidance explaining:
  - waves may depend on other waves
  - tasks MUST NOT depend on tasks in other waves
  - shelved tasks are supported and reversible
- **AND** display an error if the change directory does not exist

### Requirement: Tasks status display

The CLI SHALL display the current status of all tasks in a change, including wave progress and completion counts.

#### Scenario: Show tasks status

- **WHEN** executing `ito tasks status <change-id>`
- **THEN** parse `.ito/changes/<change-id>/tasks.md`
- **AND** extract all tasks with their wave, status, dependencies, and done-when criteria
- **AND** display a summary showing:
  - Total number of tasks
  - Number of tasks by status (pending, in-progress, complete, shelved)
  - Current wave and wave progress
  - Next task(s) ready to execute (wave dependencies satisfied and within-wave task dependencies complete)
- **AND** display a table with tasks grouped by wave showing status, files affected, and dependencies
- **AND** print an error if the tasks file does not exist

### Requirement: Task execution management

The CLI SHALL provide commands to start, complete, and move to the next task, with automatic dependency validation.

#### Scenario: Start a task

- **WHEN** executing `ito tasks start <change-id> <task-id>`
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID
- **AND** verify that the task's wave is unlocked (all wave dependencies are complete)
- **AND** verify that all within-wave dependencies for the task have status "complete"
- **AND** update the task status to "in-progress"
- **AND** set the task's `**Updated At**` field to today (`YYYY-MM-DD`)
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been started
- **AND** print an error if the task ID is not found
- **AND** print an error if the task is shelved
- **AND** print an error if any dependencies are not complete

#### Scenario: Complete a task

- **WHEN** executing `ito tasks complete <change-id> <task-id>`
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID
- **AND** update the task status to "complete"
- **AND** set the task's `**Updated At**` field to today (`YYYY-MM-DD`)
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been completed
- **AND** print an error if the task ID is not found

#### Scenario: Move to next task

- **WHEN** executing `ito tasks next <change-id>`
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** identify all tasks with status "pending" that:
  - are in an unlocked wave
  - have all within-wave dependencies marked "complete"
- **AND** exclude tasks with status "shelved" from readiness
- **AND** display the list of ready tasks with their IDs, descriptions, and affected files
- **AND** if exactly one ready task exists, automatically start it and display confirmation
- **AND** if multiple ready tasks exist, display them and ask user which to start
- **AND** if no ready tasks exist, display a message indicating all complete/shelved or blockers remain

### Requirement: Task structure validation

The CLI SHALL validate that tasks.md follows the enhanced format and provide guidance on corrections.

#### Scenario: Validate tasks file

- **WHEN** the tasks file is loaded or modified
- **THEN** check that the file includes required sections: header, waves, tasks
- **AND** verify that each wave declares dependencies via a `Depends On` line (or explicitly `None`)
- **AND** verify that each task has: ID, description, files, dependencies (or "None"), action, verify, done-when, status, updated-at
- **AND** check that status values are valid: pending, in-progress, complete, shelved
- **AND** validate that task dependencies refer only to tasks in the same wave
- **AND** validate that non-shelved tasks do not depend on shelved tasks
- **AND** display errors for any structural issues found
- **AND** suggest corrections and provide examples

#### Scenario: Blocking validation errors

- **GIVEN** `.ito/changes/<change-id>/tasks.md` contains any structural validation errors
- **WHEN** executing any `ito tasks` subcommand that reads or modifies tasks
- **THEN** the command fails
- **AND** the command prints the validation errors with file path and line numbers
- **AND** the command does not modify tasks.md

### Requirement: Wave management

The CLI SHALL support organizing tasks into waves that enable parallel execution and checkpointing.

#### Scenario: Add a wave

- **WHEN** editing tasks.md to add a new wave
- **THEN** ensure the wave has a clear number and optional description
- **AND** specify wave dependencies using an explicit `Depends On` line (for example `Wave 1, Wave 3` or `None`)
- **AND** include tasks under the wave with proper hierarchy
- **AND** support checkpoint tasks that require human approval

#### Scenario: Wave dependency validation

- **WHEN** executing `ito tasks next <change-id>` or `ito tasks start <change-id> <task-id>`
- **THEN** check that all waves listed in the current wave's `Depends On` line are complete
- **AND** treat a wave as complete when all tasks in that wave are either "complete" or "shelved"
- **AND** display an error if a required wave has incomplete tasks
- **AND** list which tasks must be completed (or shelved) to unlock the wave

### Requirement: Status tracking

The CLI SHALL maintain accurate status tracking for all tasks and support status transitions.

#### Scenario: Validate status transitions

- **WHEN** a task status is updated
- **THEN** verify that the transition is valid:
  - pending -> in-progress
  - pending -> shelved
  - in-progress -> complete
  - in-progress -> shelved
  - shelved -> pending
  - complete (no transitions allowed)
- **AND** display an error for invalid status transitions
- **AND** maintain status history if specified in the task format

#### Scenario: Display task progress

- **WHEN** executing `ito tasks status <change-id>`
- **THEN** calculate and display overall progress percentage based on tasks that are complete or shelved
- **AND** show wave-specific progress percentages
- **AND** display counts for complete vs shelved
- **AND** indicate estimated time remaining if duration information is available

## ADDED Requirements

### Requirement: Task shelving

The CLI SHALL support shelving and unshelving tasks to reflect changes in plan without deleting tasks.

#### Scenario: Shelve a task

- **WHEN** executing `ito tasks shelve <change-id> <task-id>`
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID
- **AND** update the task status to "shelved"
- **AND** set the task's `**Updated At**` field to today (`YYYY-MM-DD`)
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been shelved
- **AND** print an error if the task ID is not found
- **AND** print an error if the task is already complete

#### Scenario: Unshelve a task

- **WHEN** executing `ito tasks unshelve <change-id> <task-id>`
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID
- **AND** update the task status to "pending"
- **AND** set the task's `**Updated At**` field to today (`YYYY-MM-DD`)
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been unshelved
- **AND** print an error if the task ID is not found
- **AND** print an error if the task is not currently shelved
