# cli-validate (MODIFIED)

## Purpose

Extend the existing `ito validate` command to include audit event validation as an integrated part of change validation, rather than requiring a separate `ito audit validate` invocation.

## Requirements

### REQ-CLIVAL-1: Audit validation integrated into `ito validate --changes`

When `ito validate` runs change validation (via `--changes`, `--all`, or a specific change id), the validator SHALL also run audit event validation for that change and merge the results into the same `ValidationReport`.

- **Scenario: Change with consistent audit trail**
  - Given a change `005-01_foo` with tasks in various states
  - And an `events.jsonl` containing matching state-change events
  - When `ito validate --changes` is run
  - Then the change passes validation (no audit-related issues)

- **Scenario: Change with missing audit events**
  - Given a change `005-01_foo` with task 1.1 marked complete in `tasks.md`
  - And `events.jsonl` has no `TaskStatusChanged` event for task 1.1
  - When `ito validate --changes` is run
  - Then a warning issue is produced: "Task 1.1 status 'complete' has no corresponding audit event"

- **Scenario: Change with divergent audit state**
  - Given task 1.2 has status `in-progress` in `tasks.md`
  - And the last audit event for task 1.2 shows status `pending`
  - When `ito validate --changes` is run
  - Then a warning issue is produced: "Task 1.2 file status 'in-progress' diverges from audit state 'pending'"

- **Scenario: No events.jsonl exists**
  - Given no `.ito/.state/audit/events.jsonl` file exists
  - When `ito validate --changes` is run
  - Then an info-level issue is produced: "No audit log found; audit validation skipped"
  - And validation otherwise proceeds normally

### REQ-CLIVAL-2: Audit-specific flags

The `ito validate` command SHALL accept an `--audit` flag to control audit validation behavior.

- **Scenario: Explicit audit-only validation**
  - Given `ito validate --audit` is run
  - Then only audit event validation is performed (JSONL integrity + drift detection)
  - And results use the standard `ValidationReport` format

- **Scenario: Skip audit validation**
  - Given `ito validate --changes --no-audit` is run
  - Then change validation runs without audit event checking

### REQ-CLIVAL-3: Audit issues use standard severity levels

Audit validation issues SHALL use the existing `ValidationIssue` infrastructure with appropriate severity:

- **Error**: Malformed JSONL lines, unparseable events, unknown schema version
- **Warning**: State drift (file vs audit mismatch), missing events for observed state changes, non-monotonic timestamps
- **Info**: No audit log present, empty audit log

### REQ-CLIVAL-4: Strict mode applies to audit issues

When `ito validate --strict` is used, audit warnings SHALL be promoted to errors, consistent with existing strict mode behavior for spec and task validation.

- **Scenario: Strict mode with audit drift**
  - Given a change with audit drift warnings
  - When `ito validate --strict` is run
  - Then the drift warnings become errors and validation fails
