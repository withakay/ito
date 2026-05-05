---
title: Coordination Symlink Repair and Sync
summary: Coordination worktree init/sync now repairs missing links and broken-but-correct symlinks, rejects ambiguous duplicate or wrong-target state, and treats missing remote configuration as non-fatal after local repair.
tags: []
related: [development/source_guides/source_guide_workflow.md]
keywords: []
createdAt: '2026-04-29T19:38:05.093Z'
updatedAt: '2026-04-29T19:38:05.093Z'
---
## Reason
Document coordination worktree symlink repair and sync behavior from coordination.rs and coordination_worktree.rs

## Raw Concept
**Task:**
Document coordination worktree symlink wiring, health checks, teardown, and sync behavior

**Changes:**
- Treat missing `.ito/` links, correct symlinks whose targets are missing, and empty generated `.ito/` directories as safe during init/sync
- Fail explicitly on wrong symlink targets and non-empty duplicate `.ito/` directories to avoid silently merging ambiguous state
- Wire symlinks before health checks during sync
- Treat missing origin/remote configuration as non-fatal `RateLimited` after local repair

**Files:**
- ito-rs/crates/ito-core/src/coordination.rs
- ito-rs/crates/ito-core/src/coordination_worktree.rs
- ito-rs/crates/ito-core/src/coordination_tests.rs
- ito-rs/crates/ito-core/src/coordination_worktree_tests.rs

**Flow:**
provision/init -> resolve worktree path -> create or reuse worktree -> wire `.ito` symlinks -> update `.gitignore` -> health check -> fetch -> fast-forward -> rate-limit check -> auto-commit -> push -> persist sync state

**Timestamp:** 2026-04-29

**Patterns:**
- `^# Ito coordination worktree symlinks$` - Gitignore marker block added for coordination symlinks

## Narrative
### Structure
coordination.rs owns symlink creation, repair, teardown, and health classification for `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit`. coordination_worktree.rs owns worktree provisioning, auto-commit, sync state persistence, fetch/fast-forward handling, and push orchestration.

### Dependencies
Uses `lexical_normalize` for path comparison, git worktree commands for lifecycle management, `CoordinationStorage` to distinguish embedded versus worktree mode, and shared git metadata for sync-rate limiting.

### Highlights
Correct symlinks are repaired by recreating the target directory when the target was removed; wrong targets and non-empty directories fail with explicit remediation. Sync now fetches first, fast-forwards the local branch, and falls back to `RateLimited` when origin is missing or not configured after local repair.

### Rules
Already a correct symlink — ensure the target exists, then skip. Wrong symlink — fail with explicit actual/expected target guidance. Real directory that is empty — remove it and create the symlink. Real directory with content — fail so duplicate state is not merged implicitly. Does not exist — create the symlink directly. Messages follow the What / Why / How pattern so that both humans and AI agents can act on them immediately.

### Examples
If `.ito/changes` points at the wrong worktree path, the code returns a process error telling the user to delete or move the wrong symlink and rerun `ito init`. If the coordination worktree is not yet created but storage is worktree mode, provisioning creates the worktree, wires symlinks, and then updates `.gitignore`.
