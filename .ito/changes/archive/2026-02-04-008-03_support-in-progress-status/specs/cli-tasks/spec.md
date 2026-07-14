## MODIFIED Requirements

### Requirement: Task execution management

The CLI SHALL provide commands to start, complete, and move to the next task, with automatic dependency validation. **The CLI SHALL support both enhanced format and checkbox-only format for the `start` command.**

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

#### Scenario: Start a task in checkbox format

- **WHEN** executing `ito tasks start <change-id> <task-id>`
- **AND** the tasks.md file uses checkbox-only format
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID (1-indexed line number in checkbox format)
- **AND** verify that no other task is currently in-progress (`- [~]`)
- **AND** update the task marker from `- [ ]` to `- [~]`
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been started
- **AND** print an error if the task is already complete (`- [x]`)
- **AND** print an error if another task is already in-progress

#### Scenario: Complete a task

- **WHEN** executing `ito tasks complete <change-id> <task-id>`
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID
- **AND** update the task status to "complete"
- **AND** set the task's `**Updated At**` field to today (`YYYY-MM-DD`)
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been completed
- **AND** print an error if the task ID is not found

#### Scenario: Complete a task in checkbox format

- **WHEN** executing `ito tasks complete <change-id> <task-id>`
- **AND** the tasks.md file uses checkbox-only format
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** find the task with the specified ID (1-indexed line number in checkbox format)
- **AND** update the task marker from `- [ ]` or `- [~]` to `- [x]`
- **AND** write the updated tasks.md file
- **AND** display a confirmation that the task has been completed

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

#### Scenario: Move to next task in checkbox format

- **WHEN** executing `ito tasks next <change-id>`
- **AND** the tasks.md file uses checkbox-only format
- **THEN** read `.ito/changes/<change-id>/tasks.md`
- **AND** identify the first task with status "pending" (`- [ ]`)
- **AND** verify no task is currently in-progress (`- [~]`)
- **AND** display the next pending task
- **AND** print guidance to start it with `ito tasks start <change-id> <task-id>`
- **AND** if a task is already in-progress, display it as the current task
- **AND** if no pending tasks remain, display a completion message
