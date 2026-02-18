---
name: ito-path
description: Use when you need stable absolute paths (project/worktree/.ito/worktrees) without embedding machine-specific paths into committed files
---

# Ito Path Helpers

## Overview

Use `ito path ...` to resolve filesystem paths at runtime.

This is the preferred way to:

- print absolute paths in agent-facing instructions
- make scripts robust across different checkout locations
- avoid embedding machine-specific absolute paths in committed files

## Commands

```bash
# Stable root shared across worktrees
ito path project-root

# Current working worktree root (fails in bare repos)
ito path worktree-root

# Absolute path to the `.ito` directory
ito path ito-root

# Worktrees root (fails if worktrees disabled)
ito path worktrees-root

# A specific worktree directory
ito path worktree --main
ito path worktree --branch <name>
ito path worktree --change <change-id>

# Bundle of roots
ito path roots
ito path roots --json
```

## Key Distinctions

- `project-root` is stable across linked worktrees.
- `worktree-root` is the current working tree root.
- `ito-root` is the `.ito` directory for the current invocation.

In a bare/control repo directory (no working tree), Ito should be run from a worktree. `ito path worktree-root` exits non-zero and prints a helpful error.

## JSON Output

```bash
ito path project-root --json
```

Returns:

```json
{ "path": "/abs/path" }
```

## Portability Rule

Committed files (templates/skills) should use repo-relative paths.

If an absolute path is needed at runtime (for agent output or scripts), instruct the caller to use `ito path ...`.
