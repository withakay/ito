# Spec: ito-core (MODIFIED)

> Additions to the core layer for audit log filesystem writer and reconciliation engine.

## ADDED

### Requirement: Filesystem audit writer

The `ito-core` crate SHALL provide a filesystem-backed implementation of the `AuditWriter` trait.

#### Scenario: FsAuditWriter implementation

WHEN `FsAuditWriter::new(ito_path)` is called
THEN it SHALL resolve the audit log path as `{ito_path}/.state/audit/events.jsonl`
AND create the parent directory if it does not exist

#### Scenario: Append event to file

WHEN `FsAuditWriter::append(event)` is called
THEN the event SHALL be serialized as a single-line JSON string
AND appended to the JSONL file followed by a newline
AND the file SHALL be opened in append mode (not truncating)

#### Scenario: Best-effort error handling

WHEN a file I/O error occurs during append
THEN the error SHALL be logged at `warn` level via `tracing`
AND `Ok(())` SHALL be returned (best-effort, never fails the caller)

#### Scenario: Testable via FileSystem trait

WHEN testing the audit writer
THEN `FsAuditWriter` SHALL accept a generic filesystem implementation matching the `FileSystem` trait pattern
AND tests SHALL use an in-memory filesystem to avoid real I/O

### Requirement: Audit log reader

The `ito-core` crate SHALL provide functions to read and parse the audit event log.

#### Scenario: Read all events

WHEN `read_audit_events(ito_path)` is called
THEN it SHALL read the JSONL file line by line
AND parse each line as an `AuditEvent`
AND return a `Vec<AuditEvent>` in chronological order
AND skip lines that fail JSON parsing (with a warning logged)

#### Scenario: Read events with filter

WHEN `read_audit_events_filtered(ito_path, filter)` is called
THEN it SHALL accept filter criteria: `entity`, `scope`, `op`, `after` (timestamp), `before` (timestamp)
AND return only events matching all provided filter criteria

### Requirement: Reconciliation engine

The `ito-core` crate SHALL provide a reconciliation engine that detects drift between the audit log and file-on-disk state.

#### Scenario: Reconcile tasks for a change

WHEN `reconcile_change(ito_path, change_id, dry_run)` is called
THEN it SHALL materialize expected task states from the audit log
AND parse current task states from `tasks.md`
AND compare each task's status
AND return a `ReconciliationReport` with drift items

#### Scenario: ReconciliationReport structure

WHEN a reconciliation report is generated
THEN it SHALL contain: `change_id`, `drift_items: Vec<DriftItem>`, `compensating_events: Vec<AuditEvent>`
AND `DriftItem` SHALL contain: `entity`, `entity_id`, `expected`, `actual`, `severity`

#### Scenario: Apply compensating events

WHEN `reconcile_change(ito_path, change_id, dry_run: false)` finds drift
THEN compensating events SHALL be generated with `actor: "reconcile"`
AND events SHALL be appended to the audit log via `FsAuditWriter`

### Requirement: Audit validation engine

The `ito-core` crate SHALL provide validation logic for the audit event log.

#### Scenario: Structural validation

WHEN `validate_audit_log(ito_path, strict)` is called
THEN it SHALL check: valid JSON per line, required fields present, valid timestamps, known entity types, monotonic ordering

#### Scenario: Semantic validation

WHEN semantic validation runs
THEN it SHALL check: no duplicate creates, events reference prior creates, valid status transitions

#### Scenario: Validation result

WHEN validation completes
THEN it SHALL return a `ValidationResult` with `errors: Vec<Diagnostic>`, `warnings: Vec<Diagnostic>`, where each `Diagnostic` has `line`, `severity`, `message`
