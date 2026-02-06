# Spec: cli-audit

> CLI commands for inspecting, reconciling, and validating the audit log.

## ADDED

### Requirement: Audit log display

The CLI SHALL provide a command to view audit log events with filtering.

#### Scenario: Show recent events

WHEN `ito audit log` is invoked
THEN it SHALL display the most recent events (default: last 50)
AND each event SHALL be displayed with timestamp, entity, entity_id, operation, actor, and state transition
AND `--limit <N>` SHALL control how many events are shown

#### Scenario: Filter by change

WHEN `ito audit log --change <id>` is invoked
THEN only events with `scope` matching the change_id SHALL be displayed
AND change_id SHALL support fuzzy resolution (prefix matching, consistent with other `ito` commands)

#### Scenario: Filter by entity type

WHEN `ito audit log --entity <type>` is invoked
THEN only events with the specified entity type SHALL be displayed

#### Scenario: JSON output

WHEN `--json` is passed
THEN events SHALL be output as a JSON array

### Requirement: Audit reconciliation command

The CLI SHALL provide a command to detect and fix state drift between the audit log and file-on-disk state.

#### Scenario: Reconcile a specific change

WHEN `ito audit reconcile --change <id>` is invoked
THEN drift detection SHALL run for the specified change
AND drift items SHALL be displayed with expected vs actual state
AND the user SHALL be prompted to apply compensating events (unless `--yes` is passed)

#### Scenario: Reconcile all changes

WHEN `ito audit reconcile` is invoked without `--change`
THEN drift detection SHALL run for all active (non-archived) changes
AND drift items SHALL be grouped by change

#### Scenario: Dry-run reconciliation

WHEN `ito audit reconcile --dry-run` is invoked
THEN drift SHALL be detected and displayed
AND no events SHALL be written to the log

#### Scenario: Exit codes

WHEN reconciliation completes
THEN exit code 0 indicates no drift detected
AND exit code 1 indicates drift was detected (and optionally fixed)
AND exit code 2 indicates an error prevented reconciliation

### Requirement: Audit validation command

The CLI SHALL provide a command to validate the audit log's structural and semantic integrity.

#### Scenario: Basic validation

WHEN `ito audit validate` is invoked
THEN structural and semantic validation SHALL run
AND results SHALL be displayed as a summary with error/warning counts

#### Scenario: Strict validation

WHEN `ito audit validate --strict` is invoked
THEN warnings SHALL be treated as errors
AND exit code SHALL be non-zero if any issues are found

#### Scenario: State consistency check

WHEN `ito audit validate --check-state` is invoked
THEN state materialization SHALL run in addition to structural validation
AND file-on-disk state SHALL be compared against materialized state
AND mismatches SHALL be reported

### Requirement: Audit stats command

The CLI SHALL provide a command to display aggregate statistics from the audit log.

#### Scenario: Show audit stats

WHEN `ito audit stats [--change <id>]` is invoked
THEN it SHALL display: total events, events by entity type, events by operation, events by actor
AND optionally scoped to a specific change
AND `--json` SHALL produce structured JSON output
