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

### Requirement: Validation output

The validator SHALL provide clear, actionable output.

#### Scenario: Summary output

WHEN validation completes
THEN a summary SHALL be displayed: total events checked, errors found, warnings found
AND each diagnostic SHALL include: line number, severity, message

#### Scenario: JSON output

WHEN `--json` is passed
THEN the validation result SHALL be output as structured JSON with `errors`, `warnings`, and `summary` fields
