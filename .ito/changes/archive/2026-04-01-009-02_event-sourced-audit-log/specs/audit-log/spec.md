# Spec: audit-log

> Core audit log infrastructure: event schema, JSONL writer, append-only file management.

## ADDED

### Requirement: Audit event schema

The audit log SHALL define a versioned event schema (v1) that captures domain state transitions with sufficient context for replay, reconciliation, and observability.

#### Scenario: Event structure

WHEN an audit event is created
THEN it SHALL contain the following fields:
- `v` (u32): Schema version, currently `1`
- `ts` (String): UTC timestamp in RFC 3339 format with millisecond precision
- `entity` (String): Entity type — one of `task`, `change`, `module`, `wave`, `planning`, `config`
- `entity_id` (String): Entity identifier (task id, change id, module id, config key, etc.)
- `scope` (Option<String>): Scoping context — the change_id for task/wave events, `None` for global entities
- `op` (String): Operation type (e.g., `status_change`, `create`, `archive`, `shelve`, `set`)
- `from` (Option<String>): Previous state value (`None` for create operations)
- `to` (Option<String>): New state value (`None` for delete/archive operations where target state is implicit)
- `actor` (String): Mutation source — one of `cli`, `reconcile`, `ralph`
- `by` (String): User/agent identity, derived from git config `user.name` or `$USER` env var, formatted as `@lowercase-hyphenated`
- `meta` (Option<serde_json::Value>): Optional operation-specific metadata (e.g., file paths for task add, resolution for archive)
- `ctx` (EventContext): Session and git context for traceability

#### Scenario: EventContext structure

WHEN an audit event is created
THEN it SHALL include a `ctx` field containing:
- `session_id` (String): Ito-generated UUID v4 per CLI process group, persisted to `.state/audit/.session`
- `harness_session_id` (Option<String>): Captured from `$ITO_HARNESS_SESSION_ID` or `$CLAUDE_SESSION_ID` env vars, in that order of precedence
- `branch` (Option<String>): Current git branch name from `git symbolic-ref --short HEAD` (None if detached HEAD)
- `worktree` (Option<String>): Worktree name/path if not the main worktree (None if main)
- `commit` (Option<String>): Short HEAD commit hash (8 chars) from `git rev-parse --short HEAD`
AND all fields except `session_id` SHALL be optional (None when not available)
AND the `ctx` SHALL serialize as a nested JSON object

#### Scenario: Session ID lifecycle

WHEN a CLI command emits an audit event
THEN it SHALL read the session ID from `{ito_path}/.state/audit/.session` if it exists
AND if the file does not exist, it SHALL generate a new UUID v4, write it to `.session`, and use it
AND the `.session` file SHALL be gitignored (it is process-local, not project history)
AND all events within a single CLI invocation SHALL share the same session ID

#### Scenario: Git context resolution

WHEN `EventContext` is resolved
THEN git context fields (branch, worktree, commit) SHALL be captured once per CLI invocation and cached
AND resolution failures (e.g., no git repository, detached HEAD) SHALL result in None values, not errors
AND git context resolution SHALL NOT block or slow down the primary operation

#### Scenario: Event serialization

WHEN an audit event is serialized
THEN it SHALL produce a single-line JSON object (no pretty-printing)
AND the JSON SHALL be deterministic (keys in declaration order via `serde` derive)

#### Scenario: Schema versioning

WHEN an event is deserialized
THEN the `v` field SHALL be checked
AND events with unknown schema versions SHALL be preserved but flagged as warnings during validation

### Requirement: Audit event types

The audit log SHALL define typed operation constants for each entity to ensure consistency.

#### Scenario: Task operations

WHEN a task entity event is recorded
THEN the `op` field SHALL be one of: `create`, `status_change`, `add`

#### Scenario: Change operations

WHEN a change entity event is recorded
THEN the `op` field SHALL be one of: `create`, `archive`

#### Scenario: Module operations

WHEN a module entity event is recorded
THEN the `op` field SHALL be one of: `create`, `change_added`, `change_completed`

#### Scenario: Wave operations

WHEN a wave entity event is recorded
THEN the `op` field SHALL be one of: `unlock`

#### Scenario: Planning operations

WHEN a planning entity event is recorded
THEN the `op` field SHALL be one of: `decision`, `blocker`, `question`, `note`, `focus_change`

#### Scenario: Config operations

WHEN a config entity event is recorded
THEN the `op` field SHALL be one of: `set`, `unset`

### Requirement: Audit log file storage

The audit log SHALL be stored as a single append-only JSONL file per project.

#### Scenario: File location

WHEN the audit log is written
THEN events SHALL be appended to `.ito/.state/audit/events.jsonl`
AND the directory SHALL be created on first write if it does not exist

#### Scenario: Append-only semantics

WHEN an event is written to the log
THEN it SHALL be appended as a new line at the end of the file
AND existing lines SHALL NOT be modified or removed
AND each line SHALL be a complete, self-contained JSON object terminated by a newline

#### Scenario: Empty or missing log

WHEN the audit log file does not exist
THEN read operations SHALL treat this as an empty event history (zero events)
AND write operations SHALL create the file and parent directories

### Requirement: Audit log writer

The audit log SHALL provide an injectable writer trait for testability.

#### Scenario: Writer trait definition

WHEN the audit writer is defined
THEN it SHALL expose a method `append_event(event: &AuditEvent) -> Result<()>`
AND the trait SHALL be generic over the filesystem implementation (matching the `FileSystem` trait pattern used by `FsTaskRepository`)

#### Scenario: Best-effort semantics

WHEN an audit event write fails (I/O error, permission denied, etc.)
THEN the failure SHALL be logged via the `tracing` crate at `warn` level
AND the calling operation SHALL NOT be blocked or aborted
AND the function SHALL return `Ok(())` after logging the failure (best-effort)

#### Scenario: Filesystem writer implementation

WHEN the filesystem-backed writer appends an event
THEN it SHALL open the file in append mode
AND write a single JSON line followed by a newline character
AND flush the write buffer

### Requirement: Git integration

The audit log SHALL be version-controlled alongside the project data it describes.

#### Scenario: File is tracked by git

WHEN `ito init` creates the `.ito/.state/audit/` directory
THEN `events.jsonl` SHALL NOT be listed in `.gitignore`
AND the file SHALL be committed with other project artifacts

#### Scenario: Merge-friendly format

WHEN two branches with divergent audit logs are merged
THEN git's default line-based merge SHALL produce a valid JSONL file (both sides' appended lines are kept)
AND `ito audit validate` can detect and report duplicate events if any
