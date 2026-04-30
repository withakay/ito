# ito-web — L3 Adapter

Feature-gated web server (axum): REST API, auth, WebSocket terminal emulation, frontend asset serving. **All business logic in ito-core.**
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Structure
```
src/
├── main.rs    lib.rs    api.rs    auth.rs    frontend.rs    server.rs    terminal.rs
```

## Dependencies
|ito-core (required edge) |ito-templates

## Feature Gating
Pulled in by ito-cli via `web` feature (default: enabled). `--no-default-features` = CLI-only binary.

## Constraints
**MUST NOT:** be depended on by ito-domain or ito-core (enforced by arch-guardrails) | contain business logic | define domain types/repo impls
**MUST:** depend on ito-core (required edge) | act as web adapter only (routing, auth, presentation)

## Gotchas
|keep browser auth/session separate from backend API auth unless intentionally unified
|terminal processes need cleanup to avoid orphaned sessions

## Quality
```bash
make check && make test && make arch-guardrails
```
|rust-quality-checker: style |rust-code-reviewer: verify no business logic in API handlers (should call ito-core and serialize results)
