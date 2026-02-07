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

### Requirement: Live event streaming

The CLI SHALL provide a command to tail audit events in real-time across all worktrees.

#### Scenario: Basic stream

WHEN `ito audit stream` is invoked
THEN it SHALL tail the local audit event file and display new events as they are appended
AND events SHALL be displayed with timestamp, entity, entity_id, operation, actor, and worktree/branch tag
AND the command SHALL run indefinitely until interrupted (Ctrl+C)

#### Scenario: Worktree-aware streaming

WHEN `ito audit stream` is invoked in a project that uses `git worktree`
THEN it SHALL discover all worktrees via `git worktree list --porcelain`
AND it SHALL monitor the audit event file in each worktree simultaneously
AND events from all worktrees SHALL be interleaved by timestamp
AND each event SHALL be tagged with the worktree name or branch to distinguish sources

#### Scenario: Worktree removal during streaming

WHEN a worktree is removed while `ito audit stream` is running
THEN the watcher for that worktree SHALL be dropped silently
AND streaming SHALL continue for remaining worktrees
AND new worktrees created during streaming SHALL NOT be auto-discovered (restart required)

#### Scenario: Stream filtering

WHEN `ito audit stream --entity <type>` or `--change <id>` or `--op <operation>` is passed
THEN only events matching the filter criteria SHALL be displayed
AND filtering SHALL apply across all monitored worktrees

#### Scenario: JSON stream output

WHEN `ito audit stream --json` is invoked
THEN each event SHALL be emitted as a single JSON line (JSONL format)
AND this mode is suitable for piping to other tools (e.g., `jq`, log aggregators)

#### Scenario: Stream is informative only

WHEN events from a worktree branch are observed via stream
AND that branch is later discarded without merging
THEN those events will have been displayed in the stream
AND they will NOT exist in the merged mainline audit log
AND this is expected behavior -- the stream is a live monitoring aid, not the source of truth
