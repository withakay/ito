# Spec: ito-domain (MODIFIED)

> Additions to the domain layer for audit event types.

## ADDED

### Requirement: Audit event domain types

The `ito-domain` crate SHALL provide the audit event data model in a new `audit` module.

#### Scenario: AuditEvent struct

WHEN the `audit` module is defined
THEN it SHALL export an `AuditEvent` struct with fields matching the audit-log spec schema (v, ts, entity, entity_id, scope, op, from, to, actor, by, meta)
AND the struct SHALL derive `Serialize`, `Deserialize`, `Debug`, `Clone`

#### Scenario: EntityType enum

WHEN the `audit` module is defined
THEN it SHALL export an `EntityType` enum with variants: `Task`, `Change`, `Module`, `Wave`, `Planning`, `Config`
AND it SHALL serialize to lowercase strings (`task`, `change`, `module`, `wave`, `planning`, `config`)

#### Scenario: Actor enum

WHEN the `audit` module is defined
THEN it SHALL export an `Actor` enum with variants: `Cli`, `Reconcile`, `Ralph`
AND it SHALL serialize to lowercase strings (`cli`, `reconcile`, `ralph`)

#### Scenario: EventContext struct

WHEN the `audit` module is defined
THEN it SHALL export an `EventContext` struct with fields:
  - `session_id` (String) -- Ito-generated UUID v4 per CLI process group
  - `harness_session_id` (Option<String>) -- Captured from `$ITO_HARNESS_SESSION_ID` or `$CLAUDE_SESSION_ID` env vars
  - `branch` (Option<String>) -- Current git branch from `git symbolic-ref --short HEAD`
  - `worktree` (Option<String>) -- Worktree name if not the main worktree
  - `commit` (Option<String>) -- Short HEAD commit hash (8 chars) from `git rev-parse --short HEAD`
AND the struct SHALL derive `Serialize`, `Deserialize`, `Debug`, `Clone`
AND all fields except `session_id` SHALL be `Option<String>` for graceful degradation

#### Scenario: EventContext on AuditEvent

WHEN an `AuditEvent` is constructed
THEN it SHALL include a `ctx` field of type `EventContext`
AND the `ctx` field SHALL serialize as a nested JSON object

#### Scenario: EventContext resolution

WHEN `EventContext` is resolved
THEN git context fields SHALL be captured once per CLI invocation and cached for reuse
AND the session ID SHALL be read from `{ito_path}/.state/audit/.session` if it exists, or generated and written if not
AND the harness session ID SHALL be read from environment variables (`$ITO_HARNESS_SESSION_ID`, `$CLAUDE_SESSION_ID`) in order of precedence

#### Scenario: AuditEventBuilder

WHEN constructing audit events
THEN a builder pattern SHALL be provided to construct `AuditEvent` instances
AND the builder SHALL auto-populate `v` (current schema version), `ts` (UTC now), `by` (from git/env), and `ctx` (from resolved EventContext)
AND required fields (`entity`, `entity_id`, `op`) SHALL be enforced at compile time or via builder validation

#### Scenario: Crate dependency constraint

WHEN the `audit` module is added to `ito-domain`
THEN it SHALL NOT introduce any new crate dependencies beyond what `ito-domain` already depends on (`serde`, `serde_json`, `chrono`)
AND it SHALL NOT depend on `ito-core` or `ito-cli`

### Requirement: Audit writer trait

The `ito-domain` crate SHALL define a trait for audit log writing to enable dependency inversion.

#### Scenario: AuditWriter trait

WHEN the audit writer trait is defined
THEN it SHALL be named `AuditWriter`
AND it SHALL expose: `fn append(&self, event: &AuditEvent) -> Result<()>`
AND it SHALL be object-safe for dynamic dispatch

#### Scenario: NoopAuditWriter

WHEN audit logging is not configured or not desired
THEN a `NoopAuditWriter` implementation SHALL be provided that discards all events
AND this SHALL be the default when no audit log path is available

### Requirement: Append-only contract

The audit event model SHALL enforce the append-only invariant at the type level.

#### Scenario: No mutation API

WHEN the audit module is defined
THEN it SHALL NOT provide any method to modify or delete events
AND `AuditEvent` SHALL be immutable after construction (all fields set at creation time)
AND the `AuditWriter` trait SHALL expose only `append` (no `update`, `delete`, or `truncate`)

### Requirement: Worktree identification

The `ito-domain` crate SHALL provide types for identifying audit event sources across worktrees.

#### Scenario: WorktreeInfo type

WHEN worktree discovery is needed
THEN the domain SHALL export a `WorktreeInfo` struct with fields: `path` (PathBuf), `branch` (Option<String>), `is_main` (bool)
AND it SHALL derive `Debug`, `Clone`

#### Scenario: Tagged audit event

WHEN events are observed from multiple worktrees (e.g., during streaming)
THEN a `TaggedAuditEvent` struct SHALL be provided with fields: `event` (AuditEvent), `source` (WorktreeInfo)
AND this type is used by the stream command to interleave and tag events
