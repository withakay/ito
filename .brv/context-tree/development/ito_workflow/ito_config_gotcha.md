---
createdAt: '2026-05-13T17:39:06.664Z'
keywords: []
related: [development/ito_workflow/coordination_branch_setup.md, development/ito_workflow/coordination_symlink_repair_and_sync.md]
summary: Global ito config command manages ~/.config/ito/config.json, while repo-local .ito/config.json must set coordination_branch enabled with worktree storage; the coordination worktree uses embedded storage to avoid self-symlink validation failures.
tags: []
title: Ito Config Gotcha
updatedAt: '2026-05-13T17:39:06.664Z'
---
## Reason
Document repo-local vs global config behavior and coordination worktree storage rules

## Raw Concept
**Task:**
Document the difference between global Ito config management and repo-local effective project config, including coordination worktree storage requirements.

**Changes:**
- Clarified that ito config reads and writes the global user config at ~/.config/ito/config.json
- Specified that normal repo worktrees should set changes.coordination_branch.enabled=true, name=ito/internal/changes, and storage=worktree
- Specified that the coordination worktree at ~/.local/share/ito/withakay/ito should keep enabled=true and name=ito/internal/changes but use storage=embedded

**Files:**
- .ito/config.json

**Flow:**
global config command -> ~/.config/ito/config.json; repo-local effective config -> .ito/config.json; coordination worktree storage -> embedded to avoid self-symlink validation failures

**Author:** Ito config documentation

## Narrative
### Structure
The config command is a global CLI for reading and writing user settings, but repository behavior depends on the repo-local .ito/config.json effective configuration.

### Dependencies
Normal worktrees rely on worktree-backed coordination branch storage, while the coordination worktree itself must use embedded storage because it is the storage target.

### Highlights
The command supports path, list, get, set, unset, schema, and help. The coordination worktree at ~/.local/share/ito/withakay/ito is the storage target and must not validate itself as a self-symlinked worktree.

### Rules
For normal worktrees in this repo, .ito/config.json should explicitly set changes.coordination_branch.enabled=true, name=ito/internal/changes, and storage=worktree. The coordination worktree at ~/.local/share/ito/withakay/ito should keep enabled=true and name=ito/internal/changes but use storage=embedded to avoid self-symlink validation failures.

### Examples
ito config path; ito config get defaults.schema; ito config set defaults.schema "spec-driven"
