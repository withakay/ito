<!-- ITO:START -->
## Context

Ito schemas include `apply.tracks` (for example `tasks.md` in `schemas/spec-driven/schema.yaml`), but several parts of Ito still assume the tracking filename is always `tasks.md`. In schema-driven validation workflows, this can result in validating the wrong file or failing to validate the intended tracking file.

This change treats the selected schema's `apply.tracks` as the canonical tracking file path for a change, while keeping existing behavior working for schemas that do not set `apply.tracks`.

## Goals / Non-Goals

**Goals:**

- Resolve a change's tracking file path from `schema.yaml` `apply.tracks`, with a `tasks.md` fallback.
- Ensure `ito validate` and `ito tasks` act on the same resolved tracking file.
- Keep path handling defensive (no traversal, no separators) so schemas cannot escape the change directory.
- Provide a clear, actionable error when `ito tasks` is used with a schema that tracks progress in a non-`ito.tasks-tracking.v1` format.

**Non-Goals:**

- Supporting nested tracking paths (for example `progress/todo.md`) or absolute paths.
- Automatically migrating existing changes between tracking formats.
- Updating every existing feature that reads `tasks.md` (for example archive or list status) as part of this change.

## Decisions

- Tracking file resolution:
  - If the resolved schema declares `apply.tracks`, use it.
  - Otherwise, default to `tasks.md`.
  - The resolved path is always interpreted relative to the change directory.

- Path safety:
  - Reject tracking paths containing path separators (`/` or `\\`), traversal (`..`), or absolute-path prefixes.
  - Treat the configured value as a filename (basename) only.

- Tasks CLI format gate:
  - `ito tasks ...` is only supported when the schema's tracking file is validated/declared as `ito.tasks-tracking.v1`.
  - If the schema uses a different tracking validator/format, `ito tasks` exits non-zero and explains that the schema uses a different tracking format, plus points at the schema/workflow documentation.

## Risks / Trade-offs

- [Risk] Some commands may still implicitly assume `tasks.md` and show inconsistent status for schemas tracking a different file.
  -> Mitigation: constrain scope to validation + tasks CLI for now; track follow-on changes to update other consumers.

- [Risk] Overly-permissive paths could enable reading/writing outside the change directory.
  -> Mitigation: treat `apply.tracks` as a basename; reject separators/traversal/absolute prefixes.
<!-- ITO:END -->
