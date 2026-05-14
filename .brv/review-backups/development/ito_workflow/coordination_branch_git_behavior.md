---
title: Coordination Branch Git Behavior
summary: Coordination branch setup uses empty-tree root commits for missing remote branches, rejects empty command output, validates branch names, and reserves changes in a temporary detached worktree.
tags: []
related: [development/ito_workflow/coordination_branch_setup.md, development/ito_workflow/coordination_symlink_repair_and_sync.md]
keywords: []
createdAt: '2026-05-13T18:51:45.443Z'
updatedAt: '2026-05-13T18:51:45.443Z'
---
## Reason
Document coordination branch bootstrapping and reservation rules from git.rs

## Raw Concept
**Task:**
Document coordination branch initialization, fetch/push classification, and reservation flow behavior in git.rs

**Changes:**
- Added empty-tree-based coordination branch bootstrap for missing remote branches
- Rejected empty commit-tree and mktree output
- Defined coordination git error classification and reservation worktree flow

**Files:**
- ito-rs/crates/ito-core/src/git.rs

**Flow:**
detect missing remote branch -> create empty tree -> create root commit -> push init refspec; otherwise fetch and reserve via detached temp worktree

**Timestamp:** 2026-05-13

**Author:** ByteRover

**Patterns:**
- `^# Ito coordination worktree symlinks$` - Gitignore marker block for coordination symlinks

## Narrative
### Structure
The module exposes fetch, push, reservation, and branch-setup helpers plus core wrappers, with dedicated cleanup for temporary worktrees.

### Dependencies
Relies on git worktree checks, remote fetch/push commands, branch-name validation, and temporary worktree cleanup.

### Highlights
Branch setup returns Ready when the remote branch already exists and Created when it must be initialized. Push failures are classified for non-fast-forward, protected branch, remote rejected, remote missing, remote not configured, and command failures.

### Rules
git commit-tree for initialization must not include -p
The pushed ref must be <commit_hash>:refs/heads/<coordination-branch>
Initialization commit must be a root commit

### Examples
Branch bootstrap flow: fetch origin/<branch> -> if missing create empty tree commit -> trim stdout -> reject empty hash -> push init refspec.

## Facts
- **coordination_branch_bootstrap**: Coordination branch initialization must not use the caller’s HEAD. [convention]
- **coordination_branch_init_flow**: When origin/<coordination-branch> is missing, the branch is initialized by creating an empty tree with git mktree, creating a root commit with git commit-tree without -p, and pushing that commit to origin/<coordination-branch>. [project]
- **empty_hash_rejection**: Empty stdout from git mktree and git commit-tree must be rejected so a blank hash is never pushed. [convention]
- **reservation_branch_safety**: Reservation flows must ensure, fetch, and checkout the coordination branch before committing metadata to avoid leaking implementation history from the caller’s HEAD. [convention]
- **coordination_git_error_kinds**: The coordination git helper classifies remote missing, remote not configured, non-fast-forward, protected branch, remote rejected, and generic command failures. [project]
