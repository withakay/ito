## Context

The v1 backend API is project-scoped, and tokens are also project-scoped. In practice, clients frequently start with only a base URL plus a bearer token. Requiring manual project ID configuration up front is error-prone.

This change adds a small bootstrap/introspection surface so clients can validate credentials and learn their effective project scope from the backend.

## Goals / Non-Goals

**Goals:**

- Provide a backend endpoint that validates a token and returns the associated project identity.
- Provide a backend health/version endpoint suitable for clients to gate backend mode.

**Non-Goals:**

- Admin workflows for creating projects or issuing tokens.
- Multi-project tokens or fine-grained RBAC.

## Decisions

- Decision: Add non-project-scoped introspection endpoints under `/v1/`.
  - Rationale: allows bootstrap without prior project ID knowledge while preserving project scoping for stateful endpoints.

- Decision: Introspection responses return the authoritative project ID bound to the token.
  - Rationale: prevents clients from accidentally writing under the wrong project scope.

## Risks / Trade-offs

- [Bootstrap endpoint leaks metadata] -> Return minimal data (project_id and token scope only).
- [Clients over-rely on bootstrap] -> Keep project-scoped endpoints unchanged; bootstrap is a convenience, not a bypass.

## Migration Plan

1. Add `/v1/health` (or equivalent) and `/v1/auth/whoami` endpoints.
2. Update OpenAPI to include these endpoints.
3. Add integration tests for valid/invalid tokens and returned project identity.
