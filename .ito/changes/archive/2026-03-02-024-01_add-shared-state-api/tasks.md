## 1. Scaffold ito-backend crate

- [x] 1.1 Create `ito-rs/crates/ito-backend/` directory structure with `src/lib.rs` and `Cargo.toml`
- [x] 1.2 Add `ito-backend` to workspace `Cargo.toml` members and `workspace.dependencies`
- [x] 1.3 Configure dependencies: `axum`, `tokio`, `tower-http`, `serde_json`, `ito-core`, `ito-domain`, `ito-config`
- [x] 1.4 Add `#![warn(missing_docs)]` and module documentation
- [x] 1.5 Verify `make build` succeeds with the new crate

## 2. Implement shared application state

- [x] 2.1 Define `AppState` struct holding repository instances, project root, and ito path
- [x] 2.2 Implement `AppState::new()` constructor that builds repositories from a project root path
- [x] 2.3 Write unit tests for `AppState` construction

## 3. Implement health and readiness endpoints

- [x] 3.1 Create `GET /api/v1/health` endpoint returning `{"status": "ok"}`
- [x] 3.2 Create `GET /api/v1/ready` endpoint that checks `.ito/` directory existence
- [x] 3.3 Write integration tests for health and readiness endpoints

## 4. Implement authentication middleware

- [x] 4.1 Create bearer token authentication middleware (extract from `Authorization` header)
- [x] 4.2 Support deterministic token generation (SHA-256 of hostname + project root + salt) as default
- [x] 4.3 Support explicit token override via configuration
- [x] 4.4 Exempt `/api/v1/health` and `/api/v1/ready` from authentication
- [x] 4.5 Return 401 Unauthorized with structured error for invalid/missing tokens
- [x] 4.6 Write tests for auth middleware (valid token, invalid token, missing token, exempt paths)

## 5. Implement change API endpoints

- [x] 5.1 Create `GET /api/v1/changes` endpoint returning list of `ChangeSummary`
- [x] 5.2 Create `GET /api/v1/changes/{change_id}` endpoint returning full `Change`
- [x] 5.3 Implement 404 error handling for non-existent changes
- [x] 5.4 Create `GET /api/v1/changes/{change_id}/tasks` endpoint returning task list with progress
- [x] 5.5 Write integration tests for all change endpoints (happy path and error cases)

## 6. Implement module API endpoints

- [x] 6.1 Create `GET /api/v1/modules` endpoint returning list of `ModuleSummary`
- [x] 6.2 Create `GET /api/v1/modules/{module_id}` endpoint returning full `Module`
- [x] 6.3 Implement 404 error handling for non-existent modules
- [x] 6.4 Write integration tests for module endpoints

## 7. Implement structured error responses

- [x] 7.1 Define `ApiError` type with `error` message and `code` fields
- [x] 7.2 Implement `IntoResponse` for `ApiError` to produce JSON error bodies
- [x] 7.3 Map `CoreError` and `DomainError` variants to appropriate HTTP status codes
- [x] 7.4 Write tests for error response format

## 8. Implement server bootstrap and router assembly

- [x] 8.1 Create `BackendConfig` struct (bind address, port, token, project root)
- [x] 8.2 Implement `serve()` async function that assembles routes, middleware, and starts the server
- [x] 8.3 Add CORS middleware with configurable allowed origins
- [x] 8.4 Write integration test that starts server and makes a full request cycle

## 9. Add backend configuration to ItoConfig

- [x] 9.1 Add `BackendConfig` section to config types (`url`, `token`, `enabled`)
- [x] 9.2 Add serde/schemars annotations for JSON schema generation
- [x] 9.3 Set defaults (`enabled: false`, `url: http://127.0.0.1:9010`)
- [x] 9.4 Write tests for config loading with backend settings

## 10. Add CLI serve-api subcommand

- [x] 10.1 Add `serve-api` subcommand to `ito-cli` (feature-gated behind `backend` feature)
- [x] 10.2 Support `--bind`, `--port`, and `--token` CLI arguments
- [x] 10.3 Resolve project root and construct `BackendConfig`
- [x] 10.4 Output listening address and token to stderr on startup
- [x] 10.5 Write CLI integration test for `serve-api` subcommand

## 11. Architecture and quality verification

- [x] 11.1 Run `make arch-guardrails` and verify no violations
- [x] 11.2 Run `make check` (fmt + clippy)
- [x] 11.3 Run `make test` and verify all tests pass
- [x] 11.4 Run `make docs` and verify documentation builds cleanly
- [x] 11.5 Validate change: `ito validate 024-01 --strict`
