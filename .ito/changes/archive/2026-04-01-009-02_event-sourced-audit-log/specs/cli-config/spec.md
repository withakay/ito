# Spec: cli-config (MODIFIED)

> Modifications to emit audit events on config mutations.

## MODIFIED

### Requirement: Configuration Management

The `ito config` command SHALL provide set, get, unset, list, path, and schema operations for managing Ito configuration.

#### Scenario: Set config emits audit event

WHEN `ito config set <key> <value>` is invoked and the write succeeds
THEN an audit event SHALL be emitted with `entity: "config"`, `entity_id: <key>`, `op: "set"`, `to: <value>`, `actor: "cli"`
AND the previous value (if any) SHALL be captured in the `from` field

#### Scenario: Unset config emits audit event

WHEN `ito config unset <key>` is invoked and the key existed
THEN an audit event SHALL be emitted with `entity: "config"`, `entity_id: <key>`, `op: "unset"`, `from: <old_value>`, `to: null`, `actor: "cli"`

#### Scenario: Audit event failure does not block config operation

WHEN the config operation succeeds but the audit event write fails
THEN the config change SHALL be preserved
AND the audit failure SHALL be logged at `warn` level
AND the command SHALL exit successfully
