<!-- ITO:START -->
## Context

Schemas can declare a tracking file via `apply.tracks`, but Ito currently assumes `tasks.md` for:

- tasks validation in `ito validate`
- `ito tasks` operations

To make schemas genuinely schema-driven, Ito must resolve and operate on the same tracking file that the schema declares.

## Decisions

### Tracking file resolution

- Resolve the tracking file path for a change as:
  1) `schema.yaml` `apply.tracks` if present
  2) otherwise `tasks.md`

### Path safety

- Treat `apply.tracks` as a file name relative to the change directory.
- Reject any configured tracking value that includes path separators or traversal (`/`, `\\`, `..`).

### Validation integration

- When schema validation is configured (via `validation.yaml`), validate only the resolved tracking file.
- Do not validate `tasks.md` if the schema tracks a different file.

### Tasks CLI integration

- `ito tasks ...` reads and updates the resolved tracking file.
- If the resolved tracking file is not an Ito tasks-tracking file (schema chooses a different format), `ito tasks` fails with a clear, actionable error.

### Empty tasks-tracking severity

- When validating a file as `ito.tasks-tracking.v1`, if it contains zero recognizable tasks, emit:
  - a warning in non-strict mode
  - an error in strict mode

## Notes

- Prefer a single path-resolution helper used by validate/tasks/audit so behavior stays consistent.
<!-- ITO:END -->
