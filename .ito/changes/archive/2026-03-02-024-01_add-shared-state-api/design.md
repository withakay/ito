# Design: Add Shared State API

## Context

Ito currently relies entirely on the local filesystem for state. Multiple AI harness sessions (e.g., two OpenCode instances or an OpenCode + Claude Code pair) working on the same project can clobber each other's task progress, proposal edits, or audit events because there is no coordination layer. Git merge is too coarse and too slow for real-time coordination.

The existing `ito-web` crate provides a file-browser/editor/terminal UI but no domain-aware API. This change introduces a separate `ito-backend` crate that exposes project state via a proper REST API, laying the foundation for multi-agent coordination.

## Goals / Non-Goals

### Goals

- Provide a read-only-first HTTP API for changes, tasks, and modules (write operations will follow in subsequent changes)
- Reuse existing domain repository ports (`ChangeRepository`, `TaskRepository`, `ModuleRepository`)
- Follow the established onion architecture (Layer 3 adapter)
- Use axum (already in the workspace) for consistency with `ito-web`
- Support token-based authentication
- Make the backend startable via `ito serve-api`
- Add backend configuration to the config schema

### Non-Goals

- Real-time push notifications (WebSocket/SSE) -- deferred to 024-04
- Write/mutation endpoints (including task status updates) -- deferred to 024-02
- Backend-backed CLI client (the CLI still uses local filesystem) -- deferred to 024-02
- Multi-project support (one backend instance serves one project)
- Database storage (this change uses the filesystem repositories directly)
- Deployment, containerization, or cloud hosting

## Decisions

### New crate: `ito-backend`

A dedicated `ito-backend` crate under `ito-rs/crates/` at Layer 3.

**Why not extend `ito-web`?** `ito-web` is a file-browser/editor UI with an embedded frontend. The backend API has different concerns (domain-aware JSON endpoints, no frontend, different auth model for machine-to-machine communication). Keeping them separate follows the Single Responsibility Principle and avoids coupling frontend deployment with API deployment.

**Alternatives considered:**
- Extend `ito-web` with API routes: Rejected because it mixes UI and API concerns and complicates deployment
- Standalone service outside the workspace: Rejected because it loses access to the shared domain/core crates

### Axum with shared state

Use `axum` with `State<Arc<AppState>>` where `AppState` holds the repository instances and configuration. The repositories are constructed once at startup and shared across request handlers.

```rust
struct AppState {
    change_repo: ChangeRepository,
    task_repo: TaskRepository,
    module_repo: ModuleRepository,
    project_root: PathBuf,
    ito_path: PathBuf,
}
```

### JSON serialization of domain types

Domain types (`Change`, `ChangeSummary`, `Module`, `TaskItem`, etc.) already derive `Serialize`. API responses wrap them in a consistent envelope or return them directly.

### Authentication: Bearer tokens

Reuse the deterministic token generation pattern from `ito-web` (SHA-256 of hostname + project root + salt) as the default, with an explicit `--token` override. Machine-to-machine clients pass `Authorization: Bearer <token>`.

### Versioned API prefix

All endpoints live under `/api/v1/` to support future breaking changes without disrupting existing clients.

### Default port: 9010

Use port 9010 (one above `ito-web`'s 9009) to avoid conflicts when both services run simultaneously.

## Risks / Trade-offs

- **Risk**: Filesystem repositories are not thread-safe for concurrent writes.
  **Mitigation**: This change focuses on read endpoints. Write safety is addressed in 024-02 (change-leasing) and 024-05 (change-sync).

- **Risk**: Adding a new crate increases compilation time.
  **Mitigation**: The crate is small and shares most dependencies with `ito-web` (already compiled). Feature-gate the `backend` feature in `ito-cli` similar to the `web` feature.

- **Trade-off**: Separate crate vs. extending `ito-web` adds one more workspace member but provides cleaner separation of concerns.

## Migration Plan

No migration needed. This is purely additive:
1. New `ito-backend` crate added to workspace
2. New `serve-api` command added to CLI (feature-gated)
3. New `backend` config section (optional, defaults to disabled)
4. No existing behavior changes

## Open Questions

- Should the backend serve both read and write operations in this first change, or start read-only? **Decision: Start read-only; write operations follow in 024-02.**
- Should the backend support CORS for browser-based clients? **Decision: Yes, with configurable allowed origins, defaulting to same-origin.**
