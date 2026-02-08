---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

# Using Git Worktrees
{% if enabled %}

## Overview

Git worktrees create isolated workspaces sharing the same repository, allowing work on multiple branches simultaneously without switching.

**Strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`

**Announce at start:** "I'm using the using-git-worktrees skill to set up an isolated workspace."
{% if strategy == "checkout_subdir" %}

## Workspace Layout

Worktrees live in a hidden subdirectory inside the checkout:

```
<project>/                          # {{ default_branch }} branch checkout
├── .git/
├── src/
└── .{{ layout_dir_name }}/         # gitignored worktree directory
    └── <branch-name>/              # one worktree per change
```

## Creating a Worktree

```bash
# 1. Ensure .{{ layout_dir_name }}/ is gitignored
grep -qxF '.{{ layout_dir_name }}/' .gitignore 2>/dev/null || echo '.{{ layout_dir_name }}/' >> .gitignore

# 2. Create worktree with new branch
git worktree add ".{{ layout_dir_name }}/$BRANCH_NAME" -b "$BRANCH_NAME"

# 3. Move into the worktree
cd ".{{ layout_dir_name }}/$BRANCH_NAME"
```

The worktree directory is `.{{ layout_dir_name }}/` inside the project root. Do NOT use any other location. Do NOT ask the user where to create worktrees.
{% elif strategy == "checkout_siblings" %}

## Workspace Layout

Worktrees live in a sibling directory next to the project checkout:

```
~/Code/
├── <project>/                              # {{ default_branch }} branch checkout
│   ├── .git/
│   └── src/
└── <project>-{{ layout_dir_name }}/        # sibling worktree directory
    └── <branch-name>/                      # one worktree per change
```

## Creating a Worktree

```bash
# 1. Determine sibling directory
PROJECT_NAME=$(basename "$(pwd)")
WORKTREE_BASE="../${PROJECT_NAME}-{{ layout_dir_name }}"
mkdir -p "$WORKTREE_BASE"

# 2. Create worktree with new branch
git worktree add "${WORKTREE_BASE}/$BRANCH_NAME" -b "$BRANCH_NAME"

# 3. Move into the worktree
cd "${WORKTREE_BASE}/$BRANCH_NAME"
```

The worktree directory is `../<project>-{{ layout_dir_name }}/`. Do NOT use any other location. Do NOT ask the user where to create worktrees.
{% elif strategy == "bare_control_siblings" %}

## Workspace Layout

This project uses a bare/control repo layout with worktrees as siblings:

```
<project>/                              # bare/control repo
├── .bare/                              # git object store
├── .git                                # gitdir pointer
├── {{ default_branch }}/               # main branch worktree
└── {{ layout_dir_name }}/              # Ito-managed change worktrees
    └── <branch-name>/                  # one worktree per change
```

## Creating a Worktree

```bash
# 1. Ensure main worktree exists
if [ ! -d "{{ default_branch }}" ]; then
  git worktree add {{ default_branch }}
fi

# 2. Create the worktree directory and worktree
mkdir -p "{{ layout_dir_name }}"
git worktree add "{{ layout_dir_name }}/$BRANCH_NAME" -b "$BRANCH_NAME"

# 3. Move into the worktree
cd "{{ layout_dir_name }}/$BRANCH_NAME"
```

The worktree directory is `{{ layout_dir_name }}/` inside the bare repo root. Do NOT use any other location. Do NOT ask the user where to create worktrees.
{% endif %}

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
- **Fix:** Always ensure `.{{ layout_dir_name }}/` is in `.gitignore` before creating worktrees

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
- Use the configured directory: `{{ layout_dir_name }}`
- Run project setup after creating worktree
- Verify clean test baseline
{% else %}

## Overview

Git worktrees are **not configured** for this project. Do NOT create git worktrees unless the user explicitly requests it. Work in the current checkout.

To enable worktrees, run `ito init` and follow the worktree wizard, or set configuration directly:

```bash
ito config set worktrees.enabled true
ito config set worktrees.strategy checkout_subdir
ito update
```
{% endif %}

## Integration

**Called by:**
- **ito-brainstorming** - REQUIRED when design is approved and implementation follows
- **ito-subagent-driven-development** - REQUIRED before executing any tasks
- **ito-apply-change-proposal** - REQUIRED before executing any tasks
- Any skill needing isolated workspace

**Pairs with:**
- **ito-finishing-a-development-branch** - REQUIRED for cleanup after work complete
