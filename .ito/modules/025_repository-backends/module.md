# Repository Backends

## Purpose
Define repository-backed persistence for Ito so clients can resolve active work through a runtime-selected implementation instead of assuming local markdown files. This module covers filesystem and remote repository implementations, lifecycle-aware change/spec access, and the agent-facing workflow changes needed to make non-filesystem mode practical.

## Scope
- change-repository
- task-repository
- module-repository
- spec-repository
- repository-runtime-selection
- backend-client-runtime
- cli-list
- cli-show
- cli-validate
- cli-module
- cli-tasks
- cli-grep
- cli-archive
- cli-spec
- backend-agent-instructions
- agent-instructions
- config
- coordination-worktree
- coordination-worktree-migration
- change-coordination-branch
- cli-init
- ito-config-crate
- cascading-config

## Changes
- [x] 025-01_wire-change-repository-backends
- [x] 025-02_wire-task-repository-backends
- [x] 025-03_wire-module-repository-backends
- [x] 025-04_add-repository-runtime-factory
- [x] 025-05_mirror-specs-and-archives-to-backend
- [x] 025-06_improve-agent-backend-workflows
- [x] 025-07_add-local-sqlite-repository-mode
- [x] 025-08_coordination-worktree-storage
- [x] 025-09_add-worktree-sync-command
- [ ] 025-10_repository-backed-artifact-mutations
- [ ] 025-11_repository-backed-artifact-mutations
