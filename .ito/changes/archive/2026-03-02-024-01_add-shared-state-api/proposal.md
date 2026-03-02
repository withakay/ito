# Change: Add Shared State API

## Why

Multiple AI harness instances working on the same project currently rely on git-only synchronization, which is brittle and leads to race conditions, stale reads, and conflicting writes when two agents modify overlapping change/task state. A lightweight backend service with a well-defined HTTP API provides authoritative state coordination, enabling concurrent harness sessions to safely read and mutate project state without git merge conflicts.

## What Changes

- Introduce a new `ito-backend` crate (Layer 3 adapter, like `ito-web`) that hosts an HTTP API for reading Ito project state (changes, tasks, modules)
- Define a `backend-state-api` capability covering RESTful read endpoints (GET) for changes, tasks, and modules
- Add token-based authentication for backend API access (reusing patterns from `ito-web`)
- Expose change, task, and module state via JSON over HTTP, backed by the existing `ChangeRepository`, `TaskRepository`, and `ModuleRepository` domain ports
- Add a `backend` configuration section to `ItoConfig` for backend URL, auth token, and connection settings
- Wire the backend into `ito-cli` via a new `ito serve-api` subcommand that starts the backend service

## Capabilities

### New Capabilities

- `backend-state-api`: RESTful HTTP API for reading Ito project state (changes, tasks, modules). Provides JSON GET endpoints backed by domain repository ports. Runs as a standalone service or embedded in the CLI.

### Modified Capabilities

- `config`: Add `backend` configuration section for backend URL, auth token, and connection preferences

## Impact

- **New crate**: `ito-backend` under `ito-rs/crates/` (Layer 3, depends on `ito-core`)
- **Workspace**: Add new member to `Cargo.toml` workspace
- **Architecture**: New Layer 3 adapter alongside `ito-cli` and `ito-web`. Respects existing onion architecture and dependency rules.
- **Dependencies**: `axum`, `tokio`, `tower-http` (already in workspace via `ito-web`), `serde_json`
- **Configuration**: New `backend` key in `ItoConfig` schema
- **CLI**: New `ito serve-api` subcommand
- **No breaking changes**: This is purely additive; existing CLI and web functionality remain unchanged
