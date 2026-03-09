# Repository Backends

## Purpose
Define repository-backed persistence for Ito so clients can resolve active work through a runtime-selected implementation instead of assuming local markdown files. This module covers filesystem and remote repository implementations, lifecycle-aware change/spec access, and the agent-facing workflow changes needed to make non-filesystem mode practical.

## Implementation Preferences
- Prefer traits in `ito-domain`, concrete adapters/services in `ito-core`, and thin composition/formatting in `ito-cli` and backend HTTP handlers.
- Prefer dedicated test files over inline test modules for substantive coverage so repository behavior, transport behavior, and composition tests stay readable and easy to parallelize.

## Parallel Plan
- `025-04_add-repository-runtime-factory` is the foundation. Do this first far enough to lock the repository bundle/factory shape, mode names, and shared error/result conventions.
- After that checkpoint, `025-01_wire-change-repository-backends`, `025-02_wire-task-repository-backends`, and `025-03_wire-module-repository-backends` can proceed in parallel because they target separate repository contracts.
- `025-07_add-local-sqlite-repository-mode` can start once `025-04` has stabilized the composition shape; it should coordinate with the repository-track work so SQLite adapters follow the same shared contracts.
- `025-05_mirror-specs-and-archives-to-backend` should start after `025-01` has settled the lifecycle-aware `ChangeRepository` direction and `025-04` has settled runtime/factory composition.
- `025-06_improve-agent-backend-workflows` should follow once the main command/repository surfaces from `025-01`, `025-02`, `025-03`, and `025-05` are concrete enough to document accurately.

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

## Changes
- [x] 025-01_wire-change-repository-backends
- [x] 025-02_wire-task-repository-backends
- [x] 025-03_wire-module-repository-backends
- [x] 025-04_add-repository-runtime-factory
- [x] 025-05_mirror-specs-and-archives-to-backend
- [x] 025-06_improve-agent-backend-workflows
- [x] 025-07_add-local-sqlite-repository-mode
