# Runtime And Storage

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-13
source_refs:
  - .ito/specs/repository-runtime-selection/spec.md
  - .ito/specs/change-repository/spec.md
  - .ito/specs/task-repository/spec.md
  - .ito/specs/spec-repository/spec.md
  - .ito/specs/backend-client-runtime/spec.md
  - .ito/specs/change-coordination-branch/spec.md
  - .ito/specs/worktree-lifecycle/spec.md
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
- Coordination worktree storage is a legacy, experimental compatibility mode;
  tracked `.ito` directories on main are authoritative by default.
- New configurations default coordination to disabled with embedded,
  main-tracked storage. Experimental features add availability without
  activating either backend or coordination behavior.
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
