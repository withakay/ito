# Spec: audit-reconcile

> State drift detection and compensating event generation for the audit log.

## ADDED

### Requirement: State materialization from audit log

The reconciliation engine SHALL be able to materialize expected state from the audit event history.

#### Scenario: Materialize task states for a change

WHEN `materialize_task_states(events, change_id)` is called
THEN it SHALL replay all events where `entity == "task"` and `scope == change_id` in chronological order
AND return a map of `task_id -> expected_status` reflecting the last known state per task

#### Scenario: Materialize module state

WHEN `materialize_module_state(events, module_id)` is called
THEN it SHALL replay all events where `entity == "module"` and `entity_id == module_id`
AND return the set of changes added and their completion status

#### Scenario: Empty event history

WHEN the audit log is empty or missing
THEN materialization SHALL return empty state maps
AND reconciliation SHALL treat all current file state as "untracked" (no prior record)

### Requirement: Drift detection

The reconciliation engine SHALL compare materialized state against file-on-disk state and report divergences.

#### Scenario: Detect task status drift

WHEN drift detection runs for a change
THEN it SHALL parse `tasks.md` to get current task statuses
AND compare each task's file status against the materialized expected status
AND report any mismatches as drift items with: `task_id`, `expected` (from log), `actual` (from file), `severity` (warning or error)

#### Scenario: Detect untracked tasks

WHEN a task exists in `tasks.md` but has no events in the audit log
THEN it SHALL be reported as an "untracked" drift item (severity: warning)
AND the reconciliation report SHALL suggest creating a compensating `create` event

#### Scenario: Detect orphaned events

WHEN an event references a task_id that does not exist in `tasks.md`
THEN it SHALL be reported as an "orphaned" drift item (severity: warning)

### Requirement: Compensating event generation

The reconciliation engine SHALL generate events that bring the audit log in sync with the file-on-disk state.

#### Scenario: Generate compensating events

WHEN drift is detected and compensation is requested
THEN for each drift item, a new event SHALL be generated with `actor: "reconcile"`
AND the event's `from` field SHALL reflect the log's expected state
AND the event's `to` field SHALL reflect the file's actual state
AND the events SHALL be appended to the audit log

#### Scenario: Dry-run mode

WHEN reconciliation runs with `--dry-run`
THEN drift SHALL be detected and reported
AND compensating events SHALL be displayed but NOT written to the log

#### Scenario: Scoped reconciliation

WHEN `ito audit reconcile --change <id>` is invoked
THEN only tasks/waves within the specified change SHALL be reconciled
AND when no `--change` flag is provided, all active changes SHALL be reconciled
