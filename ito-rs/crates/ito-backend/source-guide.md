# Source Guide: ito-backend

## Responsibility
`ito-backend` is the multi-tenant HTTP API adapter for Ito project state. It exposes project-scoped REST routes, authentication, and server state while delegating business behavior to `ito-core`.

## Entry Points
- `src/lib.rs`: public `serve` function and config/auth re-exports.
- `src/server.rs`: server startup and router wiring.
- `src/api.rs`: REST route handlers under `/api/v1/projects/{org}/{repo}/`.
- `src/auth.rs`: admin token and derived project-token authentication.
- `src/state.rs`: shared backend application state.

## Design
- Adapter layer only: keep business rules in `ito-core` and contracts in `ito-domain`.
- Routes are org/repo scoped and enforce allowlist/auth before state access.
- JSON is the boundary format.

## Flow
1. `serve` builds server state and HTTP routes.
2. Requests pass auth/project-scope checks.
3. Handlers delegate repository or sync operations into core/backend adapters.
4. Responses serialize domain/backend DTOs as JSON.

## Integration
- Uses `ito_config::types::BackendServerConfig`.
- Shares DTOs and ports through `ito-domain` and `ito-core`.

## Gotchas
- Route changes affect clients and agent instructions.
- Auth errors should avoid leaking project existence beyond intended scope.

## Tests
- Targeted: `cargo test -p ito-backend`.
- End-to-end backend behavior may be covered through CLI/backend client tests.
