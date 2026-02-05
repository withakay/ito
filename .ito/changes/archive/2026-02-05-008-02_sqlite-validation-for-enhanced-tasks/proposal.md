# SQLite Validation for Enhanced tasks.md

## Why

Ito's `tasks.md` files are intentionally human-editable and git-friendly, but the current enhanced format has a few pain points when used as a long-lived task backend:

- **Grep/diff/merge vs correctness**: free-form markdown is easy to edit, but subtle mistakes (typos, missing fields, dangling dependencies) can silently break readiness logic.
- **Corruption resistance**: we want validation that is strict, fast, and produces precise, actionable diagnostics (path + line) so broken task files are hard to keep broken.
- **Wave-level planning**: we want waves to depend on other waves without forcing tasks to reference tasks in other waves (tasks should stay locally-scoped and stable).
- **Reality changes**: tasks can become obsolete. We need a reversible way to remove a task from the critical path without deleting history or rewriting the plan.
- **Repository integrity**: change directories are currently identified by a full name (`008-01_slug`). We want the numeric prefix (`008-01`) to be the canonical identity, so duplicates like `008-01_foo` and `008-01_bar` are detected as an integrity error.

This change introduces a relational validation layer (in-memory SQLite) that preserves the human-editable markdown source-of-truth while enabling strong constraints, fast queries, and difficult-to-corrupt workflows.

## What Changes

### Enhanced tasks.md semantics (still markdown)

- Add a new reversible terminal status: `shelved`.
  - `shelved` tasks are treated as intentionally not-to-be-done (for now), but can be reverted back to `pending`.
  - `shelved` tasks do not block progress (they count as "done" for wave completion).
- Add an explicit per-task timestamp field: `**Updated At**: YYYY-MM-DD`.
  - The CLI updates this field on every status transition (start/complete/shelve/unshelve).
- Introduce **explicit wave dependencies**.
  - Waves may depend on other waves.
  - Tasks MUST NOT depend on tasks in other waves; cross-wave gating is expressed only at the wave level.
- Tighten dependency rules.
  - Task dependencies are many-to-many (within a wave).
  - Dependencies are validated (exist, same wave, not self, no cycles).
  - Non-shelved tasks MUST NOT depend on shelved tasks.

### Repository integrity validation

- Model `.ito/` workflow data as a small ERD:
  - `module -> change -> wave -> task`
  - plus dependency edges (`wave_dep`, `task_dep`) and artifacts (`proposal/design/tasks/specs`).
- Add a validator that loads this ERD into **in-memory SQLite** and enforces invariants via constraints + validation queries.
  - SQLite is not a storage backend; it is used only for validation and readiness queries.
- Enforce canonical change identity:
  - Directory format remains `NNN-NN_<slug>` (slug required).
  - **Identity** is `NNN-NN` (numeric only). Two directories with the same numeric prefix are an error.
  - Module/change numeric IDs are derived from prefixes (e.g. `008 -> 8`), but diagnostics preserve canonical padded forms.

### CLI integration (Rust-first)

- Primary implementation targets the Rust CLI (`ito-rs`) and Rust workflow library.
- `ito tasks` commands use the validator to:
  - refuse to operate when `tasks.md` is invalid (blocking errors)
  - compute readiness based on wave dependencies + within-wave task dependencies
  - support `shelve` and `unshelve` actions (reversible)
- `ito validate` surfaces repo integrity issues (including duplicate numeric change IDs) with actionable fixes (as errors).

## Capabilities

### New

- `repo-integrity-validation`

### Modified

- `cli-tasks`
- `cli-validate`

## Impact

- **tasks.md compatibility**: existing `tasks.md` files that express cross-wave dependencies at the task level may begin failing validation. The remediation is to move those dependencies to the wave header and keep task dependencies within a wave.
- **New status value**: tools that parse tasks.md must accept `shelved`.
- **New timestamp field**: enhanced tasks.md adds `**Updated At**: YYYY-MM-DD`; status transitions update it.
- **Stricter validation**: `ito tasks` and `ito validate` may begin reporting new errors in repos that previously worked by accident.
- **No storage migration**: tasks remain stored as markdown in the repo; SQLite is in-memory only.
