---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

# Using Git Worktrees

## Overview

Git worktrees create isolated workspaces that share the same repository, allowing work on multiple branches simultaneously.

**Important:** Worktree layout and integration strategy are **config-driven** and can be different per developer.

## Source Of Truth

Always retrieve the exact strategy + commands for the current repo before creating a worktree:

```bash
ito agent instruction worktrees
```

Follow the printed instructions exactly.

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
