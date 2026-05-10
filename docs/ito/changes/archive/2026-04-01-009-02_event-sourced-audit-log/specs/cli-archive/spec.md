# Spec: cli-archive (MODIFIED)

> Modifications to emit audit events when changes are archived.

## MODIFIED

### Requirement: Archive Process

The archive operation SHALL follow a structured process to safely move changes to the archive.

#### Scenario: Performing archive

WHEN `ito archive <change_id>` is invoked and confirmed
THEN delta specs SHALL be applied to main specs (unless `--skip-specs`)
AND the change directory SHALL be moved to `changes/archive/YYYY-MM-DD-<change-name>`
AND an audit event SHALL be emitted with `entity: "change"`, `entity_id: <change_id>`, `op: "archive"`, `actor: "cli"`, `meta: { "archive_path": "<relative_archive_path>" }`
AND an audit event SHALL be emitted with `entity: "module"`, `entity_id: <module_id>`, `op: "change_completed"`, `actor: "cli"`, `meta: { "change": "<change_id>" }`

#### Scenario: Audit events emitted before directory move

WHEN the archive process runs
THEN audit events SHALL be emitted BEFORE the change directory is moved
AND this ensures the events are written while the `.ito/.state/audit/events.jsonl` path is still accessible relative to the change

#### Scenario: Audit event failure does not block archive

WHEN the archive operation succeeds but audit event write fails
THEN the archive SHALL be preserved
AND the audit failure SHALL be logged at `warn` level
