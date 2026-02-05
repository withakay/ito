# Interface/Trait-Based Task System for Ito (Taskwarrior Backend Exploration)

## Goal

Design an interface/trait-based task system for Ito that:

- Preserves today’s per-change `tasks.md` workflow as the default.
- Allows alternative backends (notably Taskwarrior) without rewriting the rest of the workflow engine.
- Works in both **TypeScript** (current CLI) and **Rust** (ito-rs).

## Where Ito Is Today

- Ito’s “apply” workflow expects a tracking file (often `tasks.md`) and computes progress + blocked/ready/all-done state.
- TypeScript supports a rich “enhanced tasks” format (waves, statuses, dependency validation, diagnostics).
- Rust currently models tasks in apply instructions but lacks parity with enhanced parsing.

Practical implication: before Taskwarrior, **Rust needs enhanced parsing parity**, or we accept backend divergence.

## Proposed Ito Task Model (Backend-Agnostic)

Keep the model close to the existing enhanced template, but explicit and serializable.

### Core Types

**TaskStatus** (Ito-level):

- `pending`
- `in_progress`
- `complete`
- `blocked` (optional: derived from deps)
- `cancelled` (optional)

**TaskRef** (stable identifier):

- `backend`: string (e.g. `markdown`, `taskwarrior`)
- `id`: string (markdown: `"1"`, taskwarrior: UUID)

**Task**:

- `ref: TaskRef`
- `title: string`
- `status: TaskStatus`
- `dependencies: TaskRef[] | string[]` (see mapping note below)
- `files?: string[]`
- `action?: string`
- `verify?: string`
- `doneWhen?: string`
- `metadata?: Record<string, string>` (extensible)

### Dependency Representation

Ito today often expresses dependencies as human-oriented identifiers (“Task 3”, artifact IDs, etc.). A backend abstraction needs a normalized form.

Recommended compromise:

- Internally, support **two dependency lanes**:
  - `dependsOn: TaskRef[]` (true task-to-task dependencies)
  - `dependsOnText: string[]` (freeform refs: artifact IDs, notes)

This avoids lying about what Taskwarrior can or cannot represent.

## Backend Interface (TypeScript)

Suggested interface surface (minimal, fits current needs and future growth):

```ts
export type TaskBackendId = 'markdown' | 'taskwarrior' | (string & {});

export interface TaskRef {
  backend: TaskBackendId;
  id: string;
}

export type TaskStatus = 'pending' | 'in_progress' | 'complete' | 'cancelled';

export interface ItoTask {
  ref: TaskRef;
  title: string;
  status: TaskStatus;
  dependsOn?: TaskRef[];
  dependsOnText?: string[];
  files?: string[];
  action?: string;
  verify?: string;
  doneWhen?: string;
  metadata?: Record<string, string>;
}

export interface TaskScope {
  repoRoot: string;
  changeId?: string;
  moduleId?: string;
}

export interface TaskBackend {
  readonly id: TaskBackendId;

  list(scope: TaskScope): Promise<ItoTask[]>;
  get(scope: TaskScope, ref: TaskRef): Promise<ItoTask | null>;

  create(scope: TaskScope, input: Omit<ItoTask, 'ref'>): Promise<TaskRef>;
  update(scope: TaskScope, ref: TaskRef, patch: Partial<Omit<ItoTask, 'ref'>>): Promise<void>;

  setStatus(scope: TaskScope, ref: TaskRef, status: TaskStatus): Promise<void>;
}
```

Notes:

- The `scope` object is key: Taskwarrior queries need a filter key; markdown needs a file path.
- Keep `update` permissive; backends can implement partial support and surface capability flags later.

## Backend Trait (Rust)

Parallel trait in Rust, keeping async-friendly signatures:

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TaskRef {
  pub backend: String,
  pub id: String,
}

#[derive(Clone, Debug)]
pub enum TaskStatus { Pending, InProgress, Complete, Cancelled }

#[derive(Clone, Debug)]
pub struct ItoTask {
  pub reference: TaskRef,
  pub title: String,
  pub status: TaskStatus,
  pub depends_on: Vec<TaskRef>,
  pub depends_on_text: Vec<String>,
  pub files: Vec<String>,
  pub action: Option<String>,
  pub verify: Option<String>,
  pub done_when: Option<String>,
  pub metadata: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct TaskScope {
  pub repo_root: std::path::PathBuf,
  pub change_id: Option<String>,
  pub module_id: Option<String>,
}

#[async_trait::async_trait]
pub trait TaskBackend {
  fn id(&self) -> &str;

  async fn list(&self, scope: &TaskScope) -> miette::Result<Vec<ItoTask>>;
  async fn get(&self, scope: &TaskScope, reference: &TaskRef) -> miette::Result<Option<ItoTask>>;

  async fn create(&self, scope: &TaskScope, task: &ItoTask) -> miette::Result<TaskRef>;
  async fn update(&self, scope: &TaskScope, reference: &TaskRef, patch: &serde_json::Value)
    -> miette::Result<()>;

  async fn set_status(&self, scope: &TaskScope, reference: &TaskRef, status: TaskStatus)
    -> miette::Result<()>;
}
```

Implementation detail: in Rust, a JSON patch (`serde_json::Value`) can keep the trait surface stable while the Ito task struct evolves.

## Markdown Backend (Default)

Implementation approach:

- TS: wrap existing `parseTasksTrackingFile(...)` and `updateEnhancedTaskStatusInMarkdown(...)`.
- Rust: implement enhanced parsing to match `src/core/templates/tasks-template.ts`.

Design choice:

- Treat `tasks.md` as the source-of-truth.
- Task IDs stay as they are in the file (“1”, “2”, …).

## Taskwarrior Backend

### What Taskwarrior Provides

- `task export` produces JSON for tasks matching a filter.
- `task import` can ingest JSON tasks.
- `task modify`, `start`, `stop`, `done` allow updates.
- `depends` is a native attribute for task-to-task blocking.
- UDAs (User Defined Attributes) allow structured custom metadata (but require local config).

### Mapping Ito Scope → Taskwarrior Filter

You need a stable way to find “the tasks for this Ito change”. Options:

1. **Project-based** (zero config):

   - `project:ito.<module>.<change>`

1. **Tag-based** (zero config, can get messy):

   - tag `+ito` plus tag `+change_006-08_port-plan-tasks-workflow-state`

1. **UDA-based** (best structure, requires config):

   - `uda.ito_change.type string`
   - `uda.ito_repo.type string`
   - filter `ito_change:<id> ito_repo:<name>`

Recommendation for a first prototype:

- Use **project-based** scoping for correctness with minimal setup.
- Optionally add `+ito` tag for easy ad-hoc filtering.

### Mapping Fields

- `title` → Taskwarrior `description`
- `status`:
  - `pending` → pending (default)
  - `in_progress` → `task <id> start` (Taskwarrior “active”)
  - `complete` → `task <id> done`
  - `cancelled` → `task <id> delete` (or `modify status:deleted`), but beware semantics
- `dependsOn`:
  - If all dependencies are Taskwarrior-backed, map to native `depends` (UUIDs)
  - Otherwise store in `dependsOnText` (annotation or metadata)
- `files/action/verify/doneWhen`:
  - Store as a single “payload” annotation block (works without UDAs)
  - Or store as UDAs for structured reporting (requires configuration)

### Minimal Command Set (Backend Implementation)

- List tasks for a change:
  - `task project:ito.<module>.<change> export`
- Create a task:
  - `task add <desc> project:ito.<module>.<change> +ito`
- Mark in progress:
  - `task uuid:<uuid> start`
- Mark complete:
  - `task uuid:<uuid> done`
- Update description/tags/project:
  - `task uuid:<uuid> modify ...`

### ID Strategy

- Use Taskwarrior `uuid` as `TaskRef.id`.
- Avoid relying on Taskwarrior numeric IDs (they are not stable across exports/imports).

### “Payload” Encoding Option

To avoid requiring UDAs, store a structured payload in an annotation:

```
[ito]
files:
- src/foo.ts
action: ...
verify: ...
done_when: ...
depends_on_text:
- Artifact: spec.md
```

Backend reads this annotation to reconstruct the richer Ito task object.

Trade-off: annotations are not strongly structured, but they are portable and require no user config.

## UX Implications for Ito

Once a backend exists, new CLI affordances become possible without changing the rest of Ito:

- `ito tasks list --change <id> [--backend taskwarrior]`
- `ito tasks add --change <id> --title "..."`
- `ito tasks start|done --ref <backend:id>`
- `ito apply --change <id>` can compute progress from the backend instead of directly parsing `tasks.md`.

## Key Pitfalls (Taskwarrior)

- **User configuration burden**: UDAs require `task config ...` and are local; missing UDAs become “orphaned” metadata.
- **Semantic mismatch**: Taskwarrior has `deleted` and `completed`; Ito may want `cancelled` and `blocked` (blocked is usually derived).
- **Sync assumptions**: `task sync` requires a configured server; don’t bake sync into core workflow.
- **Concurrency**: reading/exporting while tasks are modified can race; prefer “export → compute → operate by uuid”.
- **Cross-platform**: relying on the `task` binary and parsing its output means careful error handling and diagnostics.

## Recommended Architecture (Incremental)

1. **Parity first**: implement enhanced task parsing in Rust to match TS.
1. **Backend abstraction**: introduce `TaskBackend` (TS) and `TaskBackend` trait (Rust) with Markdown backend.
1. **Taskwarrior prototype**:
   - scope via `project:` convention
   - UUID-based refs
   - annotations as payload (no UDAs required)
1. **Optional UDA upgrade path**:
   - add a `ito init --taskwarrior` helper that prints (does not auto-apply) the `task config uda.*` commands.

## Open Questions

- Should Ito treat `tasks.md` as canonical forever, or can backends become canonical and generate `tasks.md` as a view?
- Do we need task dependencies to reference artifacts (not just tasks)? If yes, how should they affect readiness?
- Should backend selection be per-repo (`.ito.yaml`) or per-change (`.ito/changes/<id>/.ito.yaml`)?
