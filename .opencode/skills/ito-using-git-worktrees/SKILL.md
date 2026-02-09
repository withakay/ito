---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

# Using Git Worktrees


## Overview

Git worktrees create isolated workspaces sharing the same repository, allowing work on multiple branches simultaneously without switching.

**Strategy:** `bare_control_siblings`
**Directory name:** `ito-worktrees`
**Default branch:** `main`

**Announce at start:** "I'm using the using-git-worktrees skill to set up an isolated workspace."


## Workspace Layout

This project uses a bare/control repo layout with worktrees as siblings:

```
<project>/                              # bare/control repo
├── .bare/                              # git object store
├── .git                                # gitdir pointer
├── main/               # main branch worktree
└── ito-worktrees/              # Ito-managed change worktrees
    └── <branch-name>/                  # one worktree per change
```

## Creating a Worktree

```bash
# 1. Ensure main worktree exists
if [ ! -d "main" ]; then
  git worktree add main
fi

# 2. Create the worktree directory and worktree
mkdir -p "ito-worktrees"
git worktree add "ito-worktrees/$BRANCH_NAME" -b "$BRANCH_NAME"

# 3. Move into the worktree
cd "ito-worktrees/$BRANCH_NAME"
```

The worktree directory is `ito-worktrees/` inside the bare repo root. Do NOT use any other location. Do NOT ask the user where to create worktrees.


## Post-Creation Setup

After creating a worktree, run project setup:

```bash
# Node.js
if [ -f package.json ]; then npm install; fi

# Rust
if [ -f Cargo.toml ]; then cargo build; fi

# Python
if [ -f requirements.txt ]; then pip install -r requirements.txt; fi
if [ -f pyproject.toml ]; then poetry install; fi

# Go
if [ -f go.mod ]; then go mod download; fi
```

Then verify a clean baseline:

```bash
# Run project-appropriate test command
npm test / cargo test / pytest / go test ./...
```

**If tests fail:** Report failures, ask whether to proceed or investigate.
**If tests pass:** Report ready.

## Cleanup

After the change branch is merged:

```bash
# Remove the worktree
git worktree remove "$BRANCH_NAME" 2>/dev/null || true

# Delete the local branch
git branch -d "$BRANCH_NAME" 2>/dev/null || true

# Prune stale worktree metadata
git worktree prune
```

**Safety notes:**
- `git worktree remove` will refuse to remove a worktree with uncommitted changes
- `git branch -d` will refuse to delete an unmerged branch
- Run `git worktree list` to verify cleanup was successful

## Common Mistakes

### Skipping gitignore verification (checkout_subdir only)

- **Problem:** Worktree contents get tracked, pollute git status
- **Fix:** Always ensure `.ito-worktrees/` is in `.gitignore` before creating worktrees

### Proceeding with failing tests

- **Problem:** Can't distinguish new bugs from pre-existing issues
- **Fix:** Report failures, get explicit permission to proceed

### Hardcoding setup commands

- **Problem:** Breaks on projects using different tools
- **Fix:** Auto-detect from project files (package.json, Cargo.toml, etc.)

## Red Flags

**Never:**
- Create worktree in an unexpected location
- Skip baseline test verification
- Proceed with failing tests without asking

**Always:**
- Use the configured directory: `ito-worktrees`
- Run project setup after creating worktree
- Verify clean test baseline


## Integration

**Called by:**
- **ito-brainstorming** - REQUIRED when design is approved and implementation follows
- **ito-subagent-driven-development** - REQUIRED before executing any tasks
- **ito-apply-change-proposal** - REQUIRED before executing any tasks
- Any skill needing isolated workspace

**Pairs with:**
- **ito-finishing-a-development-branch** - REQUIRED for cleanup after work complete
