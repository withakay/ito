---
createdAt: '2026-04-27T22:08:14.176Z'
keywords: []
related: [development/ito_workflow/audit_mirror_concurrency_and_temp_naming.md, development/ito_workflow/worktree_validation_flow.md]
summary: Ito publish mirrors coordination-backed state into a read-only docs/ito tree with safe path resolution, drift detection, and symlink skipping.
tags: []
title: Published Ito Mirror
updatedAt: '2026-04-27T22:08:14.176Z'
---
## Reason
Document the published mirror implementation and safety constraints

## Raw Concept
**Task:**
Document the published Ito mirror implementation and its safety checks

**Changes:**
- Added safe path resolution for published mirror configuration
- Documented read-only mirror generation layout
- Captured drift detection and replacement behavior in the publish CLI

**Files:**
- docs/ito
- ito publish CLI

**Flow:**
configure mirror path -> validate path -> generate read-only mirror -> compare for drift -> replace mirror from coordination state

**Timestamp:** 2026-04-27

## Narrative
### Structure
The implementation centers on a configurable published mirror path, a renderer that emits a read-only docs tree, and a publish CLI that reconciles generated output with the existing mirror.

### Dependencies
Depends on cascading configuration, coordination-backed Ito state, and the publish command to keep the mirror aligned.

### Highlights
The mirror is designed to be safe to resolve, deterministic to generate, and suitable for consumption in plain GitHub/main checkouts without exposing writable coordination state.

## Facts
- **published_mirror_path**: The published mirror path is configured via changes.published_mirror.path and defaults to docs/ito. [project]
- **mirror_path_validation**: Mirror path resolution rejects empty paths, absolute paths, parent traversal, and project-root-only paths. [project]
- **mirror_output_layout**: The core renderer writes generated read-only output under README.md, changes/active, changes/archive, and specs. [project]
- **symlink_handling**: The renderer skips symlinks when generating the mirror. [project]
- **publish_cli_flow**: The ito publish CLI loads cascading config, detects drift by comparing the existing mirror against freshly generated output, and replaces the mirror from coordination-backed Ito state. [project]
- **source_of_truth**: Coordination state remains the writable source of truth while docs/ito is generated as readable output for plain GitHub/main checkouts. [project]
