---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

<!-- ITO:START -->

# Using Git Worktrees

## Overview

Git worktrees create isolated workspaces that share the same repository, allowing work on multiple branches simultaneously.


**Configured strategy:** `bare_control_siblings`
**Directory name:** `ito-worktrees`
**Default branch:** `main`
**Integration mode:** `commit_pr`

## Worktree Location


Worktrees live under the bare/control layout:

```bash
../
|-- main/
`-- ito-worktrees/<change-name>/
```

Create a worktree:

```bash
mkdir -p "../ito-worktrees"
git worktree add "../ito-worktrees/<change-name>" -b <change-name>
```


Do NOT ask the user where to create worktrees.

## Path Helpers

If you need absolute paths (for logs, scripts, or agent instructions), use:

- `ito path project-root`
- `ito path worktree-root`
- `ito path worktrees-root`
- `ito path worktree --main|--branch <name>|--change <id>`

## Safety Checks

- Ensure the parent directory for the worktree exists (create it if needed).
- Run a clean baseline build/test in the new worktree so new failures are attributable.
- Do not proceed if baseline tests fail without explicitly calling that out.

## Cleanup

After the branch is merged:

```bash
git worktree remove "<worktree-path>" 2>/dev/null || true
git branch -d "<branch-name>" 2>/dev/null || true
git worktree prune
```



## Integration

**Called by:**
- Any workflow that needs isolated workspace

<!-- ITO:END -->
