---
related: [development/ito_workflow/audit_mirror_concurrency_and_temp_naming.md, development/ito_workflow/ddd_discovery_workflow.md, development/ito_workflow/coordination_symlink_repair_and_sync.md]
---
# Topic: ito_workflow

## Overview
Covers Ito's tracked-main workflow, legacy coordination migration, worktree safety, and the remaining audit-mirror subsystem. Canonical Ito changes, specs, modules, workflows, and audit artifacts are tracked under `.ito/` on `main`. The published `docs/ito` mirror and its configurable publication path are retired; consumers read the tracked `.ito` tree directly.

## Key Concepts
- tracked `.ito` authority on `main`
- retired `docs/ito` publication mirror
- main-first proposal integration and implementation readiness
- explicit migration from legacy coordination storage
- worktree validation and safe Git behavior
- audit mirroring as a separate best-effort subsystem
