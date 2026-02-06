# Spec: change-creation (MODIFIED)

> Modifications to emit audit events when changes and modules are created.

## MODIFIED

### Requirement: Change Creation

The system SHALL provide a function to create new change directories programmatically.

#### Scenario: Create change

WHEN `createChange(projectRoot, name)` is called
THEN the change directory SHALL be created at `{ito_path}/changes/{change_id}/`
AND `.ito.yaml` metadata SHALL be written
AND the module's `module.md` SHALL be updated with the new change entry
AND an audit event SHALL be emitted with `entity: "change"`, `entity_id: <change_id>`, `op: "create"`, `actor: "cli"`, `meta: { "module": "<module_id>" }`
AND an audit event SHALL be emitted with `entity: "module"`, `entity_id: <module_id>`, `op: "change_added"`, `actor: "cli"`, `meta: { "change": "<change_id>" }`

#### Scenario: Audit event failure does not block creation

WHEN change directory creation succeeds but audit event write fails
THEN the change SHALL be preserved (already created on disk)
AND the audit failure SHALL be logged at `warn` level
AND the command SHALL exit successfully
