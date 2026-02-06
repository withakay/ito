# Spec: cli-tasks (MODIFIED)

> Modifications to emit audit events on task state mutations.

## MODIFIED

### Requirement: Task execution management

The CLI SHALL provide commands to start, complete, and move to the next task, with automatic dependency validation.

#### Scenario: Start a task

WHEN `ito tasks start <change_id> <task_id>` is invoked
AND the task is in the ready set (wave unlocked, dependencies met)
THEN the task status SHALL be updated to "in-progress" in tasks.md
AND the `Updated At` field SHALL be set to today's date
AND the updated tasks.md SHALL be written to disk
AND an audit event SHALL be emitted with `entity: "task"`, `entity_id: <task_id>`, `scope: <change_id>`, `op: "status_change"`, `from: "pending"`, `to: "in-progress"`, `actor: "cli"`

#### Scenario: Complete a task

WHEN `ito tasks complete <change_id> <task_id>` is invoked
THEN the task status SHALL be updated to "complete" in tasks.md
AND the `Updated At` field SHALL be set to today's date
AND the updated tasks.md SHALL be written to disk
AND an audit event SHALL be emitted with `entity: "task"`, `entity_id: <task_id>`, `scope: <change_id>`, `op: "status_change"`, `from: <previous_status>`, `to: "complete"`, `actor: "cli"`

#### Scenario: Audit event failure does not block task update

WHEN a task status update succeeds but the audit event write fails
THEN the task update SHALL be preserved (already written to disk)
AND the audit failure SHALL be logged at `warn` level
AND the command SHALL exit successfully (exit code 0)

### Requirement: Task shelving

The CLI SHALL support shelving and unshelving tasks to reflect changes in plan without deleting tasks.

#### Scenario: Shelve a task

WHEN `ito tasks shelve <change_id> <task_id>` is invoked
AND the task has status "pending" or "in-progress"
THEN the task status SHALL be updated to "shelved" in tasks.md
AND the `Updated At` field SHALL be set to today's date
AND the updated tasks.md SHALL be written to disk
AND an audit event SHALL be emitted with `entity: "task"`, `op: "status_change"`, `from: <previous_status>`, `to: "shelved"`, `actor: "cli"`

#### Scenario: Unshelve a task

WHEN `ito tasks unshelve <change_id> <task_id>` is invoked
AND the task has status "shelved"
THEN the task status SHALL be updated to "pending" in tasks.md
AND the `Updated At` field SHALL be set to today's date
AND the updated tasks.md SHALL be written to disk
AND an audit event SHALL be emitted with `entity: "task"`, `op: "status_change"`, `from: "shelved"`, `to: "pending"`, `actor: "cli"`

### Requirement: Task addition

The CLI SHALL support adding new tasks to an enhanced tasks.md.

#### Scenario: Add a task

WHEN `ito tasks add <change_id> <task_name> [--wave N]` is invoked
AND the tasks file is in enhanced format
THEN a new task block SHALL be inserted into the specified wave
AND the task SHALL be assigned the next available ID within the wave
AND an audit event SHALL be emitted with `entity: "task"`, `entity_id: <new_task_id>`, `scope: <change_id>`, `op: "create"`, `from: null`, `to: "pending"`, `actor: "cli"`, `meta: { "name": "<task_name>", "wave": N }`
