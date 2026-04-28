---
title: Worktree Validation Flow
summary: ito worktree validate now uses machine-readable statuses, hard-fails main/control checkouts, warns on non-main mismatches, and matches exact change-id prefixes to avoid false positives.
tags: []
related: []
keywords: []
createdAt: '2026-04-26T20:51:02.365Z'
updatedAt: '2026-04-26T20:51:02.365Z'
---
## Reason
Document the dedicated read-only worktree validation behavior for change work

## Raw Concept
**Task:**
Document the dedicated read-only worktree validation flow for change work.

**Changes:**
- Added a dedicated read-only worktree validation flow
- Distinguished hard failures on main/control checkouts from advisory mismatches outside main
- Added machine-readable status output for OpenCode pre-tool hooks
- Changed matching to exact change-id prefixes to avoid false positives

**Flow:**
validate worktree -> emit machine-readable status -> hard-fail if main/control checkout -> otherwise return advisory mismatch guidance -> match against exact change-id prefixes

**Timestamp:** 2026-04-26

## Narrative
### Structure
The validation command now separates dangerous main/control checkout cases from recoverable mismatches, which lets pre-tool hooks block only the unsafe scenario while still guiding users elsewhere.

### Dependencies
OpenCode pre-tool hooks depend on a machine-readable status so they can gate execution correctly.

### Highlights
Exact prefix matching prevents false positives, including suffix worktrees such as `<change>-review`.

## Facts
- **worktree_validation_cli**: The CLI command `ito worktree validate --change <id> [--json]` now supports a read-only worktree validation flow for change work. [project]
- **main_control_checkout_policy**: Main/control checkouts are treated as hard failures. [project]
- **non_main_mismatch_policy**: Mismatches outside main are advisory and include recovery guidance. [project]
- **change_id_matching**: Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`, instead of arbitrary substrings. [project]
