## Why

Backend mode needs a predictable bootstrap path so clients can verify connectivity and authorization, discover their effective project scope, and avoid manual misconfiguration that leads to cross-project writes.

## What Changes

- Add backend endpoints for project/token introspection so a client can validate credentials and discover project identity.
- Add a minimal bootstrap workflow that does not require clients to know the project ID upfront.

## Capabilities

### New Capabilities

- `backend-project-bootstrap`: Backend support for project identity discovery and bootstrap validation.

### Modified Capabilities

- (none)

## Impact

- **Affected APIs**: Adds a small set of non-project-scoped endpoints under `/v1/` for bootstrap and introspection.
- **Security**: Reduces risk of mis-scoped tokens by letting the backend assert the token's project scope.
- **Client UX**: Enables future CLI conveniences like "auto-resolve project id" without weakening auth.
