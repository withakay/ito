## Context

Ito currently reads a minimal project config from `<repo-root>/ito.json` and a minimal global config from the per-user config dir. As we add more config (agent settings, serve settings), a single file is too limiting and encourages drift between tools.

## Goals / Non-Goals

**Goals:**

- Provide a deterministic cascading config stack for project config.
- Provide clear merge semantics that are easy to test.
- Avoid hardcoding `.ito/` in config logic (respect `projectPath`).

**Non-Goals:**

- Implement the `ito agent-config` CLI group (this change focuses on config resolution).
- Introduce a new config format (remain JSON for now).

## Decisions

### Decision: JSON object deep-merge

Merge strategy:

- objects: recursively merge
- scalars: later overrides earlier
- arrays: later replaces earlier

### Decision: Ito directory naming cannot depend on `<itoDir>/config.json`

`projectPath` (the Ito working directory name) is resolved from repo-level config and/or global config.

We do not allow `<itoDir>/config.json` to influence `projectPath` because it would create a resolution cycle.

### Decision: Environment override via `PROJECT_DIR`

If `PROJECT_DIR` is set, Ito loads `$PROJECT_DIR/config.json` as the highest-precedence project config source.

## Risks / Trade-offs

- Multiple config files can be confusing; mitigate with deterministic precedence and future "explain" output.
- Deep-merge semantics must be stable over time; treat merge rules as part of the public behavior.

## Migration Plan

1. Implement loader + tests.
1. Update docs/specs.
1. Add feature consumers (e.g. serve) in follow-up changes.
