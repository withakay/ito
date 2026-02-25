<!-- ITO:START -->
## Why

Schemas can already declare `apply.tracks`, but Ito tooling is still effectively hard-coded to `tasks.md` for task operations and task validation. This makes schema-specific workflows unreliable and can cause Ito to validate or update the wrong tracking file.

## What Changes

- Resolve a change's tracking file path from the selected schema's `apply.tracks`, defaulting to `tasks.md` for backwards compatibility.
- Update `ito validate <change-id>` (in schema-driven validation mode) to validate the resolved tracking file and to avoid validating `tasks.md` when the schema tracks a different file.
- Update `ito tasks ...` commands to read and update the resolved tracking file.
- When the schema declares a tracking format other than `ito.tasks-tracking.v1`, `ito tasks` MUST exit with a helpful error explaining the mismatch and how to proceed.
- Add defensive path handling for `apply.tracks` values (reject traversal and path separators).
- If a file is declared as `ito.tasks-tracking.v1` but contains zero recognizable tasks, emit a warning (or fail validation in strict mode).

## Capabilities

### New Capabilities

- `schema-tracking-file`: Resolve and sanitize schema-provided tracking file paths for changes.

### Modified Capabilities

- `cli-tasks`: Operate on schema-resolved tracking files and reject incompatible tracking formats.
- `cli-validate`: Validate the schema-resolved tracking file (not a hard-coded `tasks.md`) and warn/error on empty `ito.tasks-tracking.v1` tracking files.
- `task-repository`: Load task counts and task data from a resolved tracking file path rather than assuming `tasks.md`.

## Impact

- Tasks and validation plumbing in `ito-core` / `ito-domain` will need to thread a resolved tracking file path through task loading, validation, and audit reconcile behaviors.
- Schema/validation configuration becomes part of day-to-day workflows (agents/users running `ito tasks` and `ito validate`).
<!-- ITO:END -->
