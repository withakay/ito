# Design: SQLite Validation for Enhanced tasks.md

## Context

Ito's enhanced `tasks.md` format is intentionally optimized for:

- storing task plans directly in the repo
- low-friction editing and review in PRs
- grep/diff/merge friendliness

As usage grows, correctness issues increasingly come from *cross-record invariants*:

- duplicate identities
- dangling references
- dependency cycles
- inconsistent wave gating
- 'dead' tasks that were abandoned but still block progress

These invariants are easier to express and validate as relational constraints and queries than as ad-hoc procedural code.

## Goals

- Keep `tasks.md` as the canonical source-of-truth (grepable, diffable, mergeable).
- Make validation strict, fast, and hard to bypass.
- Model the workflow as an ERD (modules -> changes -> waves -> tasks) with explicit dependency edges.
- Allow waves to depend on other waves without tasks referencing tasks in other waves.
- Add a reversible `shelved` status for tasks that are intentionally not-to-be-done.
- Produce actionable errors that point to file + line (and include remediation).
- Implement Rust-first (Rust is the source of truth). TypeScript parity can follow later if needed.

## Non-Goals

- Replace markdown task tracking with a database file stored in the repo.
- Implement Taskwarrior or remote issue tracker sync as part of this change.
- Introduce interactive TUI flows for task selection.
- Redesign the entire Ito artifact schema system.

## Key Decisions

### Decision: Use in-memory SQLite as the validation engine

We will parse `tasks.md` and `.ito/` directory structure into an in-memory SQLite database and enforce invariants using:

- table constraints (`PRIMARY KEY`, `UNIQUE`, `NOT NULL`, `CHECK`)
- foreign keys (`PRAGMA foreign_keys=ON`)
- post-load validation queries (including recursive CTEs for cycle detection)

This provides a single, declarative ground truth for validation, and it naturally supports fast readiness queries.

Implementation notes:

- **Rust**: use `rusqlite` for a consistent in-memory implementation.

### Decision: Separate dependency graphs

We will maintain two graphs:

- **Wave dependency graph**: edges `wave -> depends_on_wave` (cross-wave gating)
- **Task dependency graph**: edges `task -> depends_on_task` (within-wave only)

This enforces 'tasks should not be aware of other waves' tasks' while still allowing rich wave ordering.

### Decision: Canonical change identity is numeric-only

Change directories remain `NNN-NN_<slug>`, but identity is `NNN-NN`.

- `008-01_foo` and `008-01_bar` cannot both exist.
- The slug is required (readability) but non-identifying (metadata).

## Data Model (SQLite)

The DB is transient and rebuilt on each validation run.

### Core tables

```sql
PRAGMA foreign_keys = ON;

CREATE TABLE module (
  id_int  INTEGER PRIMARY KEY,
  id_text TEXT NOT NULL UNIQUE,   -- e.g. "008"
  dir     TEXT NOT NULL UNIQUE,
  slug    TEXT NOT NULL,
  title   TEXT NULL
);

CREATE TABLE change (
  module_id_int INTEGER NOT NULL REFERENCES module(id_int),
  change_seq_int INTEGER NOT NULL,
  id_text TEXT NOT NULL,          -- e.g. "008-01"
  slug    TEXT NOT NULL,
  dir     TEXT NOT NULL UNIQUE,
  PRIMARY KEY (module_id_int, change_seq_int),
  UNIQUE (id_text)
);

CREATE TABLE wave (
  change_id_text TEXT NOT NULL REFERENCES change(id_text),
  wave_num INTEGER NOT NULL,
  PRIMARY KEY (change_id_text, wave_num),
  CHECK (wave_num > 0)
);

CREATE TABLE wave_dep (
  change_id_text TEXT NOT NULL,
  wave_num INTEGER NOT NULL,
  depends_on_wave_num INTEGER NOT NULL,
  PRIMARY KEY (change_id_text, wave_num, depends_on_wave_num),
  FOREIGN KEY (change_id_text, wave_num) REFERENCES wave(change_id_text, wave_num),
  FOREIGN KEY (change_id_text, depends_on_wave_num) REFERENCES wave(change_id_text, wave_num),
  CHECK (wave_num <> depends_on_wave_num)
);

CREATE TABLE task (
  id TEXT PRIMARY KEY,            -- canonical, e.g. "008-02#1.3" (internal)
  change_id_text TEXT NOT NULL REFERENCES change(id_text),
  wave_num INTEGER NOT NULL,
  task_num INTEGER NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('pending','in-progress','complete','shelved')),
  updated_at TEXT NOT NULL,        -- YYYY-MM-DD
  source_path TEXT NOT NULL,
  source_line INTEGER NOT NULL,
  UNIQUE (change_id_text, wave_num, task_num),
  FOREIGN KEY (change_id_text, wave_num) REFERENCES wave(change_id_text, wave_num),
  CHECK (wave_num > 0),
  CHECK (task_num > 0)
);

CREATE TABLE task_dep (
  task_id TEXT NOT NULL REFERENCES task(id),
  dep_task_id TEXT NOT NULL REFERENCES task(id),
  PRIMARY KEY (task_id, dep_task_id),
  CHECK (task_id <> dep_task_id)
);
```

### Relational checks implemented as queries

Some invariants are easiest as post-load queries:

- **Within-wave task deps**
  - error if `task.wave_num <> dep.wave_num`
- **No deps on shelved tasks**
  - error if `task.status <> 'shelved'` and `dep.status = 'shelved'`
- **Cycle detection**
  - recursive CTE for both `task_dep` and `wave_dep`

Cycle detection sketch:

```sql
WITH RECURSIVE
  walk(task_id, dep_task_id, path) AS (
    SELECT task_id, dep_task_id, task_id || '->' || dep_task_id
    FROM task_dep
    UNION ALL
    SELECT w.task_id, d.dep_task_id, w.path || '->' || d.dep_task_id
    FROM walk w
    JOIN task_dep d ON d.task_id = w.dep_task_id
    WHERE instr(w.path, d.dep_task_id) = 0
  )
SELECT * FROM walk WHERE task_id = dep_task_id;
```

## Parsing and Canonicalization

### tasks.md format extensions

We keep the current enhanced format (waves + task blocks) and add three changes:

1. **Wave dependency line**

Under each `## Wave N` header, add a single structured line:

```md
## Wave 2
- **Depends On**: Wave 1, Wave 3
```

This is intentionally easy to parse and diff.

2. **New status value: shelved**

Status line remains:

```md
- **Status**: [ ] pending
- **Status**: [>] in-progress
- **Status**: [x] complete
- **Status**: [-] shelved
```

The bracket marker is cosmetic except for `[x] complete`; the validator keys off the label, but also validates the marker set.

3. **Updated At field**

Each task includes a required timestamp field:

```md
- **Updated At**: 2026-01-28
```

This field is updated on every status transition and uses `YYYY-MM-DD` to keep diffs minimal.

### Canonical identity rules

- **Module ID**: derived from module directory prefix (`008_*` => `id_int=8`, `id_text="008"`).
- **Change ID**: derived from change directory prefix (`008-01_*` => `module_id_int=8`, `change_seq_int=1`, `id_text="008-01"`).
- **Task identity**: internal canonical id is `"<change-id>#<wave>.<task>"` to avoid collisions; external display remains `wave.task` (e.g. `1.2`).

## Validation + Readiness Semantics

### Wave completion

A wave is considered complete when **all tasks in the wave are either `complete` or `shelved`**.

### Wave unlocking

A wave is unlocked when all waves it depends on are complete.

### Task readiness

A task is ready when:

- its wave is unlocked
- it is `pending`
- all within-wave dependencies are `complete` (or `shelved` is disallowed as a dependency)

### Shelving semantics

- Shelved tasks do not block progress.
- Shelving is reversible (`shelved -> pending`).
- Non-shelved tasks cannot depend on shelved tasks (validation error).

## Integration Points

### Rust (primary)

- Extend `ito-rs/crates/ito-workflow/src/tasks.rs`:
  - parse wave `Depends On`
  - add `shelved` status
  - add `Updated At` parsing + writing
  - enforce within-wave deps
  - update readiness/wave gating semantics
- Extend `ito-rs/crates/ito-cli/src/main.rs`:
  - add `tasks shelve` and `tasks unshelve`
  - treat validation issues as blocking errors across tasks subcommands
  - surface repo integrity validation via `ito validate`

### TypeScript (follow-up)

- If TypeScript CLI parity is still needed, port the same semantics in a later change.

## Testing Strategy

- Unit tests (Rust):
  - tasks parser: status parsing, wave deps parsing, updated-at parsing, source location accuracy
  - relational validator: duplicate change IDs, dangling deps, cross-wave deps (wave only), cycles
- Fixture-based tests:
  - `.ito/changes/008-01_foo` + `.ito/changes/008-01_bar` duplicate detection
  - malformed tasks.md cases (missing fields, invalid status)
- Integration tests (Rust CLI):
  - `ito tasks start/next/shelve/unshelve` behavior on valid + invalid task sets
  - `ito validate` outputs errors with actionable remediation

## Risks / Trade-offs

- **Dependency weight**: SQLite engines add size/complexity.
  - Mitigation: keep SQLite usage isolated to validation; keep the DB strictly in-memory.
- **TS/Rust drift**: two implementations must remain consistent.
  - Mitigation: treat specs as the contract; add shared fixtures; add parity tests.
- **Format churn**: existing repos may have cross-wave task deps.
  - Mitigation: produce clear error messages; optionally add a migration helper later.

## What NOT to Change

- Do not store a database file in the repo.
- Do not change `.ito/` directory layout.
- Do not make task files less grepable (avoid deeply nested YAML/JSON blobs).
- Do not allow task dependencies to reference other waves.
