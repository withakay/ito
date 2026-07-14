# Runtime And Storage

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-13
source_refs:
  - docs/ito/specs/repository-runtime-selection/spec.md
  - docs/ito/specs/change-repository/spec.md
  - docs/ito/specs/task-repository/spec.md
  - docs/ito/specs/spec-repository/spec.md
  - docs/ito/specs/backend-client-runtime/spec.md
  - docs/ito/specs/coordination-worktree/spec.md
  - docs/ito/specs/worktree-lifecycle/spec.md
  - .ito/changes/031-01_migrate-coordination-state-to-main/specs/coordination-main-migration/spec.md
  - .ito/changes/031-01_migrate-coordination-state-to-main/specs/coordination-worktree-migration/spec.md
  - .ito/changes/031-01_migrate-coordination-state-to-main/demos/031-01-migrate-to-main.md
known_gaps:
  - Does not yet summarize every backend API spec.
```

Ito has moved toward repository-backed runtime abstractions so command handlers
can read and mutate artifacts without hard-coding filesystem layout. Filesystem,
SQLite, and remote-backed modes should expose consistent repositories and
mutation services.

## Key Concepts

- Repository runtime selection decides whether operations use local markdown,
  SQLite-backed storage, or remote backend services.
- Coordination worktree storage shares change, module, spec, workflow, and
  audit state across change worktrees through `.ito/*` runtime links, but is a
  legacy layout rather than the default direction.
- Worktree lifecycle is Worktrunk-backed for creation/switching while retaining
  Ito's configured `ito-worktrees/<change-id>` path convention.

## Legacy Coordination Bridge

Ito classifies coordination evidence before command dispatch. Reads remain
available with a warning in legacy or ambiguous state; mutations fail closed
before sync, repository construction, filesystem writes, worktree creation, or
network calls. `ito agent instruction migrate-to-main` is the always-available,
agent-driven bridge: inventory and hash first, stop on destination conflicts,
materialize verified real directories, switch configuration to embedded and
disabled, validate, review, and integrate to main. The source coordination
worktree remains untouched as rollback evidence.

## Review Notes

When changing runtime behavior, check both read and mutation paths. Tests should
cover filesystem mode and at least one non-filesystem mode when behavior is
shared through repository contracts.
