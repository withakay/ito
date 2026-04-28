---
title: Repo-level Ito Refresh Audit
summary: 'Ito refresh audit: managed assets live under ito-rs/crates/ito-templates/assets/skills and assets/commands plus ito-project-setup; no ito-* orphans remained and rerun was idempotent.'
tags: []
related: []
keywords: []
createdAt: '2026-04-28T04:26:52.131Z'
updatedAt: '2026-04-28T04:26:52.131Z'
---
## Reason
Document managed harness asset scope, orphan checks, and idempotent rerun result for Ito repo refreshes

## Raw Concept
**Task:**
Document the repo-level Ito refresh audit and scope rules for managed harness assets.

**Changes:**
- Confirmed managed harness asset sources for Ito refreshes
- Verified no ito-* orphan skills or commands remained after update
- Verified rerunning the update was idempotent with an unchanged git diff hash

**Files:**
- ito-rs/crates/ito-templates/assets/skills
- ito-rs/crates/ito-templates/assets/commands
- .opencode/commands/compare-workflow-tool.md

**Flow:**
refresh harness assets -> audit for ito-* orphans -> skip user-owned entries -> rerun ito init --update --tools all -> confirm unchanged git diff hash

**Timestamp:** 2026-04-28

**Author:** repository audit

## Narrative
### Structure
This note captures the scope of repo-level Ito refreshes, separating managed harness assets from user-owned entries that must not be touched.

### Dependencies
The audit depends on the refreshed harness state produced by ito init --update --tools all and on git diff comparison for idempotence verification.

### Highlights
No ito-* orphan skills or commands were found, and the rerun produced an unchanged diff hash, confirming the refresh is stable.

### Rules
Current harness audit found no ito-* orphan skills or commands after ito init --update --tools all; non-Ito entries like .claude/skills/byterover* and .opencode/commands/compare-workflow-tool.md remain user-owned and must be skipped.

### Examples
Managed assets are expected under ito-rs/crates/ito-templates/assets/skills and assets/commands, with ito-project-setup included as the default project command.

## Facts
- **managed_harness_assets**: For repo-level Ito refreshes, managed harness assets come from ito-rs/crates/ito-templates/assets/skills, ito-rs/crates/ito-templates/assets/commands, and the default project command ito-project-setup. [project]
- **orphan_assets_status**: Current harness audit found no ito-* orphan skills or commands after ito init --update --tools all. [project]
- **user_owned_entries**: Non-Ito entries like .claude/skills/byterover* and .opencode/commands/compare-workflow-tool.md remain user-owned and must be skipped. [project]
- **refresh_idempotence**: Re-running ito init --update --tools all from the refreshed state was idempotent. [project]
- **diff_hash_stability**: The git diff hash was unchanged before and after the rerun. [project]
