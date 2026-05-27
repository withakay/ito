# Runtime And Storage

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-05-27
source_refs:
  - docs/ito/specs/repository-runtime-selection/spec.md
  - docs/ito/specs/change-repository/spec.md
  - docs/ito/specs/task-repository/spec.md
  - docs/ito/specs/spec-repository/spec.md
  - docs/ito/specs/backend-client-runtime/spec.md
  - docs/ito/specs/coordination-worktree/spec.md
  - docs/ito/specs/worktree-lifecycle/spec.md
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
  audit state across change worktrees through `.ito/*` runtime links.
- Worktree lifecycle is Worktrunk-backed for creation/switching while retaining
  Ito's configured `ito-worktrees/<change-id>` path convention.

## Review Notes

When changing runtime behavior, check both read and mutation paths. Tests should
cover filesystem mode and at least one non-filesystem mode when behavior is
shared through repository contracts.
