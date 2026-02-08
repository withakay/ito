# ito-web — Layer 3 (Adapter)

Feature-gated web server (`axum`) for browsing and editing Ito projects. Provides REST API, authentication, frontend asset serving, and terminal emulation via WebSocket.

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Serve a web UI for Ito projects. Like `ito-cli`, this is an **adapter** — all business logic lives in `ito-core`. This crate owns HTTP routing, auth, WebSocket terminal emulation, and frontend asset serving.

## Structure

```
src/
├── main.rs          # Standalone binary for development
├── lib.rs           # pub use server::{ServeConfig, serve}
├── api.rs           # REST API endpoints
├── auth.rs          # Authentication
├── frontend.rs      # Frontend asset serving
├── server.rs        # Server setup, ServeConfig, serve()
└── terminal.rs      # Terminal emulation via WebSocket + portable-pty
```

## Workspace Dependencies

- `ito-core` — **required edge** (all business logic)
- `ito-templates` — template access

## Feature Gating

This crate is not built by default in isolation — it's pulled in by `ito-cli` via the `web` feature flag (default: enabled). Building `ito-cli` with `--no-default-features` produces a CLI-only binary without the web server.

## Architectural Constraints

### MUST NOT

- Be depended on by `ito-domain` or `ito-core` (enforced by `make arch-guardrails`)
- Contain business logic — delegates everything to `ito-core`
- Define domain types or repository implementations

### MUST

- Depend on `ito-core` (required edge, enforced by guardrails)
- Act as a web adapter only — HTTP routing, auth, presentation
- Handle all async/WebSocket concerns internally

## Quality Checks

```bash
make check              # fmt + clippy
make test               # all workspace tests
make arch-guardrails    # verify no reverse dependencies from core/domain
```

Use the `rust-quality-checker` subagent for style compliance. Use the `rust-code-reviewer` subagent to verify no business logic has crept into API handlers — they should call `ito-core` functions and serialize the results.
