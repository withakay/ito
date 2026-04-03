# Spec: audit-validate

> Structural and semantic validation of the audit event log.

## ADDED

### Requirement: Structural validation

The validator SHALL check that the audit log file is well-formed JSONL with valid event structure.

#### Scenario: Valid JSON lines

WHEN the audit log is validated
THEN each line SHALL be parsed as valid JSON
AND lines that fail JSON parsing SHALL be reported as errors with line number

#### Scenario: Required fields present

WHEN each event is validated
THEN it SHALL contain all required fields: `v`, `ts`, `entity`, `entity_id`, `op`, `actor`, `by`
AND missing required fields SHALL be reported as errors with line number and field name

#### Scenario: Valid timestamp format

WHEN the `ts` field is validated
THEN it SHALL be a valid RFC 3339 timestamp
AND invalid timestamps SHALL be reported as errors

#### Scenario: Valid entity types

WHEN the `entity` field is validated
THEN it SHALL be one of the known entity types: `task`, `change`, `module`, `wave`, `planning`, `config`
AND unknown entity types SHALL be reported as warnings (forward compatibility)

#### Scenario: Monotonic timestamps

WHEN timestamps are checked across the log
THEN events SHALL be in non-decreasing chronological order
AND out-of-order events SHALL be reported as warnings (may occur after git merge)

### Requirement: Semantic validation

The validator SHALL check logical consistency of the event history.

#### Scenario: No duplicate create events

WHEN events are validated
THEN there SHALL be at most one `create` event per `(entity, entity_id, scope)` tuple
AND duplicate creates SHALL be reported as errors

#### Scenario: Events reference existing entities

WHEN a non-create event references an entity
THEN a prior `create` event for that entity SHOULD exist in the log
AND events without a prior create SHALL be reported as warnings (not errors, since the log may be incomplete)

#### Scenario: Valid status transitions

WHEN a `status_change` event for a task is validated
THEN the `from` -> `to` transition SHALL be checked against the task status state machine (pending->in-progress, pending->shelved, in-progress->complete, in-progress->shelved, shelved->pending)
AND invalid transitions SHALL be reported as errors

### Requirement: State consistency validation

The validator SHALL optionally check that the audit log's materialized state matches file-on-disk state.

#### Scenario: State match check

WHEN `ito audit validate --check-state [--change <id>]` is invoked
THEN the validator SHALL materialize state from the audit log
AND compare it against file-on-disk state
AND report mismatches as warnings

#### Scenario: Strict mode

WHEN `ito audit validate --strict` is invoked
THEN warnings SHALL be treated as errors
AND the command SHALL exit with non-zero status if any warnings or errors are found
AND this mode is suitable for CI integration

### Requirement: Integrated validation

Audit event validation SHALL be embedded into existing validation flows so agents get audit checks automatically.

#### Scenario: `ito validate --changes` integration

WHEN `ito validate --changes` validates a change
THEN it SHALL also validate audit events scoped to that change
AND audit issues SHALL appear in the same `ValidationReport` alongside task and spec issues
AND audit issues SHALL use the same `ValidationIssue` severity levels (error, warning, info)

#### Scenario: Ralph completion validation

WHEN the Ralph automation loop validates completion of a change
THEN it SHALL check audit event consistency for that change
AND audit drift (materialized state != file state) SHALL be reported as a validation failure
AND the failure SHALL be injected into the next iteration prompt (same as existing task/project validation failures)

#### Scenario: Archive pre-check

WHEN `ito archive` runs its pre-archive validation
THEN it SHALL check audit event consistency for the change being archived
AND if drift is detected, it SHALL warn the user and prompt for confirmation (unless `--no-validate` is passed)
AND the user MAY run `ito audit reconcile --change <id> --fix` to resolve drift before retrying

#### Scenario: Standalone validation remains available

WHEN `ito audit validate` is invoked directly
THEN it SHALL run full structural + semantic + state consistency checks on the entire log
AND this mode SHALL be suitable for CI pipelines and deep diagnostics beyond change-scoped flows

### Requirement: Append-only immutability

The audit log SHALL be strictly append-only. Validation and reconciliation SHALL never modify or delete existing events.

#### Scenario: Reconciliation appends only

WHEN `ito audit reconcile --fix` detects drift
THEN it SHALL append compensating `Reconciled` events to the log
AND it SHALL NOT modify or delete any existing events
AND it SHALL NOT rewrite the JSONL file

#### Scenario: No deletion commands

WHEN a user or agent interacts with the audit system
THEN there SHALL be no command to delete, edit, or truncate audit events
AND the only way to correct an incorrect event is to append a compensating event

### Requirement: Validation output

The validator SHALL provide clear, actionable output.

#### Scenario: Summary output

WHEN validation completes
THEN a summary SHALL be displayed: total events checked, errors found, warnings found
AND each diagnostic SHALL include: line number, severity, message

#### Scenario: JSON output

WHEN `--json` is passed
THEN the validation result SHALL be output as structured JSON with `errors`, `warnings`, and `summary` fields
