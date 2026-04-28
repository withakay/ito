---
title: Audit Mirror Concurrency and Temp Naming
summary: Audit mirror sync uses unique temp worktrees/orphan branch names with pid, timestamp, and an atomic counter; it dedupes JSONL, truncates by age and count, and retries push or ref updates once on conflict.
tags: []
related: []
keywords: []
createdAt: '2026-04-28T16:56:31.527Z'
updatedAt: '2026-04-28T16:56:31.527Z'
---
## Reason
Document audit mirror worktree naming, branch conflict handling, and JSONL merge behavior from mirror.rs

## Raw Concept
**Task:**
Document the audit mirror synchronization flow and its concurrency protections in mirror.rs

**Changes:**
- Added unique temp worktree naming with pid, timestamp, and atomic counter
- Added unique orphan branch naming with pid, timestamp, and atomic counter
- Implemented JSONL merge deduplication and adjacent reconciled aggregation
- Added log truncation by age and max event count
- Added retry handling for push conflicts and ref update conflicts

**Files:**
- ito-rs/crates/ito-core/src/audit/mirror.rs

**Flow:**
detect git worktree -> create temp worktree -> fetch/checkout branch or orphan -> merge JSONL -> stage/commit -> push or update ref -> retry on conflict

**Timestamp:** 2026-04-28

**Patterns:**
- `ito-audit-mirror-{pid}-{nanos}-{sequence}` - Temporary audit mirror worktree directory naming
- `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}` - Temporary orphan branch naming

## Narrative
### Structure
The mirror code is split into sync, append, merge, truncation, Git command helpers, and cleanup logic. It maintains an atomic sequence counter to avoid temp-name collisions when multiple tests or processes hit the same SystemTime tick.

### Dependencies
Depends on Git worktrees, branch refs, and serde_json parsing of AuditEvent JSONL entries. Conflict detection is based on Git output such as non-fast-forward, cannot lock ref, or reference already exists.

### Highlights
The implementation avoids collisions in .git/worktrees metadata by making worktree basenames unique, which is important during parallel audit branch writes. It also keeps the mirror bounded by age and count so the branch does not grow without limit.

### Rules
Only runs inside a Git worktree. On missing remote branch, create an orphan branch. If push returns non-fast-forward, refetch and retry once. If ref update reports a conflict, return a retryable conflict result once, then fail.

### Examples
Example failure messages include: internal audit branch unavailable outside a git worktree; failed to update internal audit branch due to concurrent writes; audit mirror push to branch failed due to a remote conflict.

## Facts
- **temp_worktree_naming**: Temporary audit mirror worktree paths include the process ID, a SystemTime-based timestamp, and a process-local atomic counter. [project]
- **orphan_branch_naming**: Temporary orphan branch names include the process ID, a SystemTime-based timestamp, and a process-local atomic counter. [project]
- **audit_mirror_scope**: Audit mirror sync is best-effort and only runs inside a Git worktree. [project]
- **mirror_update_strategy**: Mirror updates use detached worktrees and fall back to an orphan branch when the remote branch is missing. [project]
- **jsonl_merge_behavior**: Merged audit JSONL dedupes identical lines, preserves order, and collapses adjacent equivalent reconciled events by incrementing count. [project]
- **mirror_retention_policy**: Mirror logs are truncated to events within 30 days of the newest event and capped at 1000 events. [project]
- **conflict_retry_policy**: Internal branch appends retry once on conflict; audit mirror pushes retry after a non-fast-forward by refetching and merging again. [project]
