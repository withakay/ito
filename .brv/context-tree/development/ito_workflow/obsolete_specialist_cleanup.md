---
title: Obsolete Specialist Cleanup
summary: Installer pre-cleans obsolete ito-orchestrator specialist assets on update and forceful init/reinstall paths, including broken symlinks, while preserving orchestrator coordinator assets.
tags: []
related: [development/ito_workflow/worktree_validation_flow.md, development/release_workflow/release_workflow.md]
keywords: []
createdAt: '2026-04-29T07:19:12.767Z'
updatedAt: '2026-04-29T07:19:12.767Z'
---
## Reason
Document cleanup rules for renamed orchestrator assets during install and init flows

## Raw Concept
**Task:**
Document obsolete specialist agent cleanup behavior for the ito-orchestrator asset rename migration

**Changes:**
- Added cleanup on forceful reinstall/init paths in addition to update flows
- Added harness-level pre-pass removal of legacy assets before writing new assets
- Added symlink_metadata-based removal of broken legacy symlinks
- Documented obsolete specialist asset paths renamed from ito-orchestrator-* to ito-*

**Files:**
- ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs
- ito-rs/crates/ito-core/src/installers/agents_cleanup.rs

**Flow:**
detect update/force path -> pre-clean obsolete assets -> remove files or symlinks -> prune empty directories -> write new harness assets

**Timestamp:** 2026-04-29T07:19:06.632Z

## Narrative
### Structure
The cleanup logic lives in install_agent_templates() and remove_obsolete_specialist_agent(), with prune_empty_agent_dirs() removing empty legacy directories afterward.

### Dependencies
The migration depends on symlink_metadata to distinguish broken symlinks and on installer mode/options to decide when cleanup runs.

### Highlights
Legacy specialist assets removed include ito-orchestrator-planner/researcher/reviewer/worker markdown and SKILL.md files. Coordinator assets such as ito-orchestrator.md and ito-orchestrator-workflow remain intentionally excluded.

### Rules
Cleanup is triggered when mode == InstallMode::Update, or opts.update, or opts.force. Plain init keeps untouched user files in place.

### Examples
Tests verify init --update --tools all and init --force --tools all remove obsolete specialist/orchestrator assets, preserve installed specialist assets, and keep coordinator assets installed.

## Facts
- **cleanup_triggers**: Obsolete specialist agent cleanup runs on update flows and on forceful reinstall/init paths. [project]
- **cleanup_strategy**: The installer performs a harness-level pre-pass that removes legacy assets before writing new assets. [project]
- **symlink_cleanup**: Cleanup uses symlink_metadata so broken legacy symlinks are removed rather than skipped. [project]
- **init_behavior**: Plain init keeps untouched user files in place. [project]
