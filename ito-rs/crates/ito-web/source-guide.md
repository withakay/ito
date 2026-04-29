# Source Guide: ito-web

## Responsibility
`ito-web` is the browser UI adapter for browsing and editing Ito projects. It owns HTTP routing, authentication, frontend asset serving, and WebSocket terminal behavior while delegating business logic to `ito-core`.

## Entry Points
- `src/lib.rs`: public `ServeConfig` and `serve` export.
- `src/main.rs`: standalone development binary.
- `src/server.rs`: server/router setup.
- `src/api.rs`: web API routes.
- `src/frontend.rs`: frontend asset serving.
- `src/terminal.rs`: terminal/WebSocket integration.
- `src/auth.rs`: web auth helpers.

## Design
- Adapter layer: no duplicate Ito workflow semantics.
- Browser interactions should call core use-cases or repository APIs, not mutate `.ito` files ad hoc.
- Terminal features bridge interactive workflows but must keep process/session cleanup clear.

## Flow
1. `ServeConfig` identifies root/bind/port.
2. Server starts routes and frontend serving.
3. API/terminal handlers call `ito-core` or process utilities.
4. Browser receives JSON, static assets, or terminal stream events.

## Integration
- Can be launched through `ito-cli` or directly via `cargo run -p ito-web`.
- Shares config and core behavior with the CLI.

## Gotchas
- Keep browser auth/session logic separate from backend API auth unless intentionally unified.
- Terminal processes need careful cleanup to avoid orphaned sessions.

## Tests
- Targeted: `cargo test -p ito-web`.
- For UI-affecting changes, add browser or API-level verification where available.
