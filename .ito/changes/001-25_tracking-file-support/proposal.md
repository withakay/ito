<!-- ITO:START -->
## Why

Workflow schemas can declare `apply.tracks`, but Ito currently hard-codes `tasks.md` for validation and `ito tasks` operations. This causes schema authors and users to track progress in one file while Ito reads and writes a different one.

## What Changes

- Resolve a change's tracking file path from the selected schema's `apply.tracks` (when present), otherwise default to `tasks.md`.
- Update `ito validate` tasks tracking validation to validate the resolved tracking file and to avoid validating `tasks.md` when a schema tracks a different file.
- Update `ito tasks` to read and update the resolved tracking file.
- Add defensive path handling for `apply.tracks` to prevent traversal and separators.
- Improve empty tasks-tracking handling: if a file is declared as `ito.tasks-tracking.v1` but has zero recognizable tasks, emit a warning (or an error in strict mode).

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `cli-tasks`: operate on a schema-selected tracking file instead of always `tasks.md`.
- `cli-validate`: validate the schema-selected tracking file instead of always `tasks.md`.
- `task-repository`: load task counts and task data from the resolved tracking file.
- `tasks-tracking`: clarify format applicability to schema-selected tracking files and empty-file severity.

## Impact

- Rust CLI behavior changes for `ito validate` and `ito tasks` when `apply.tracks` is configured.
- Validation diagnostics may change (file path, severity) for empty tracking files and for schemas that track a non-`tasks.md` file.
- Implementation touches schema parsing, validation pipeline, task repository, and tasks CLI.
<!-- ITO:END -->
