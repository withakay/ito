# Tasks for: 024-03_add-backend-project-bootstrap

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-03_add-backend-project-bootstrap
ito tasks next 024-03_add-backend-project-bootstrap
ito tasks start 024-03_add-backend-project-bootstrap 1.1
ito tasks complete 024-03_add-backend-project-bootstrap 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add health/version endpoint to backend API

- **Files**: `ito-rs/crates/ito-web/`
- **Dependencies**: None
- **Action**: Implement a non-authenticated health endpoint that reports API version.
- **Verify**: `cargo test -p ito-web`
- **Done When**: Health endpoint returns success and includes API version identifier.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 1.2: Add token introspection endpoint

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`
- **Dependencies**: Task 1.1
- **Action**: Implement authenticated introspection endpoint that returns `project_id` bound to token.
- **Verify**: `cargo test -p ito-web`
- **Done When**: Valid token returns correct project identity; invalid token returns 401.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update OpenAPI documentation and add integration tests

- **Files**: `ito-rs/crates/ito-web/`, `docs/`
- **Dependencies**: None
- **Action**: Add OpenAPI entries for bootstrap endpoints and integration tests for auth success/failure.
- **Verify**: `make check`
- **Done When**: OpenAPI and tests cover health/version and whoami behaviors.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending
