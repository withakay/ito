# Tasks for: 024-17_backend-status-command

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-17_backend-status-command
ito tasks next 024-17_backend-status-command
ito tasks start 024-17_backend-status-command 1.1
ito tasks complete 024-17_backend-status-command 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add auth verify endpoint to backend server

- **Files**: `ito-rs/crates/ito-backend/src/api.rs`
- **Dependencies**: None
- **Action**: Add `GET /api/v1/projects/{org}/{repo}/auth/verify` handler. It passes through the existing auth middleware. Handler returns `{"valid": true, "scope": "admin"}` or `{"valid": true, "scope": "project", "org": "...", "repo": "..."}` based on the `TokenScope` from auth. Invalid tokens get 401 from middleware before reaching the handler.
- **Verify**: `cargo test -p ito-backend` passes; manual curl test with valid/invalid tokens
- **Done When**: Auth verify endpoint returns correct scope info for admin tokens, project tokens, and rejects invalid tokens with 401
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 1.2: Add BackendHealthStatus struct and health-check function in ito-core

- **Files**: `ito-rs/crates/ito-core/src/backend_health.rs`, `ito-rs/crates/ito-core/src/lib.rs`
- **Dependencies**: None
- **Action**: Create a `BackendHealthStatus` struct with fields: `server_reachable`, `server_healthy`, `server_ready`, `server_version`, `ready_reason`, `auth_verified`, `token_scope`, `error`. Implement `check_backend_health(runtime: &BackendRuntime) -> BackendHealthStatus` that calls `/api/v1/health`, `/api/v1/ready`, and `/api/v1/projects/{org}/{repo}/auth/verify` using `ureq` with a 5-second timeout. Export from `ito-core`.
- **Verify**: `cargo test -p ito-core` passes; unit tests cover healthy, unhealthy, unreachable, and auth-failed scenarios
- **Done When**: Health-check function returns correct status for all scenarios including auth verification
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 1.3: Add `ito backend` subcommand group and actions to CLI

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**: Add a `Backend` variant to the top-level `Command` enum (gated on `#[cfg(feature = "backend")]`). Define `BackendAction` enum with `Status { json: bool }` and `GenerateToken { seed: Option<String>, org: Option<String>, repo: Option<String> }`. Wire into clap derive. Add help text explaining token resolution order, env var recommendations, and security best practices.
- **Verify**: `cargo build -p ito-cli --features backend` compiles; `ito backend --help` shows both subcommands with security guidance
- **Done When**: `ito backend status`, `ito backend status --json`, and `ito backend generate-token` are recognized as valid CLI commands with descriptive help
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement `backend status` command handler

- **Files**: `ito-rs/crates/ito-cli/src/commands/backend.rs`, `ito-rs/crates/ito-cli/src/commands/mod.rs`
- **Dependencies**: None
- **Action**: Create handler that: (1) loads cascading config, (2) checks `backend.enabled`, (3) validates all required fields are present, (4) resolves `BackendRuntime`, (5) calls core health-check function, (6) checks if `backend.token` is in a non-gitignored file and emits security warning, (7) formats human-readable or JSON output, (8) sets exit code per design. Human output should show: enabled/disabled, URL, config fields (present/missing), server health/ready/version, auth verified/failed with scope.
- **Verify**: `cargo build -p ito-cli --features backend`; manual test against running backend and with no backend
- **Done When**: `ito backend status` reports correct state for all scenarios (disabled, incomplete config, unreachable, healthy, auth failed, auth verified)
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 2.2: Implement `backend generate-token` command handler

- **Files**: `ito-rs/crates/ito-cli/src/commands/backend.rs`
- **Dependencies**: None
- **Action**: Implement handler that: (1) resolves seed from env `ITO_BACKEND_TOKEN_SEED` > `--seed` flag > global config `backendServer.auth.tokenSeed`, (2) resolves org/repo from env > flag > project config > interactive prompt, (3) if prompted interactively, offers to save values to project config, (4) calls `derive_project_token(seed, org, repo)`, (5) prints the 64-char hex token to stdout, guidance to stderr.
- **Verify**: `cargo build -p ito-cli --features backend`; generate token and verify it matches manual HMAC computation
- **Done When**: Token generation works with all input combinations (config, flags, env, interactive)
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 2.3: Fix silent backend fallback in tasks.rs sync_after_mutation

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: None
- **Action**: In `sync_after_mutation()`, replace `Err(_) => return` with `Err(e) => { eprintln!("Warning: backend sync failed: {e}"); return; }`. Also replace the `try_backend_runtime()` function's silent JSON parse error swallowing with a visible warning.
- **Verify**: `cargo build -p ito-cli --features backend`; test with broken config shows warning
- **Done When**: Backend sync failures produce visible warnings on stderr
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 2.4: Fix silent backend fallback in util.rs forward_events_if_backend

- **Files**: `ito-rs/crates/ito-cli/src/util.rs`
- **Dependencies**: None
- **Action**: Replace silent returns when `resolve_backend_runtime` fails or config parsing fails with `eprintln!` warnings showing the error detail.
- **Verify**: `cargo build -p ito-cli --features backend`; test with broken config shows warning
- **Done When**: Event forwarding failures produce visible warnings on stderr
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 2.5: Fix silent backend fallback in grep.rs materialize_backend_artifacts

- **Files**: `ito-rs/crates/ito-cli/src/app/grep.rs`
- **Dependencies**: None
- **Action**: Replace `tracing::debug` and silent return when config parsing or runtime resolution fails with `eprintln!` warning.
- **Verify**: `cargo build -p ito-cli --features backend`
- **Done When**: Backend artifact materialization failures produce visible warnings on stderr
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Write integration tests for backend status command

- **Files**: `ito-rs/crates/ito-cli/tests/` or within existing test infrastructure
- **Dependencies**: None
- **Action**: Write integration tests covering: (1) backend disabled shows disabled message and exits 0, (2) backend enabled but incomplete config shows errors and exits non-zero, (3) backend enabled but unreachable shows error, (4) JSON output has all required fields, (5) token security warning when token is in non-gitignored config.
- **Verify**: `cargo test -p ito-cli --features backend` passes
- **Done When**: All spec scenarios for `ito backend status` have corresponding test coverage
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 3.2: Write tests for generate-token and auth verify

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-backend/tests/`
- **Dependencies**: None
- **Action**: Write tests covering: (1) token generation matches `derive_project_token` output, (2) auth verify endpoint returns correct scope for admin and project tokens, (3) auth verify returns 401 for invalid tokens, (4) seed resolution precedence (env > flag > config).
- **Verify**: `cargo test -p ito-cli --features backend && cargo test -p ito-backend`
- **Done When**: Token generation and auth verify have comprehensive test coverage
- **Updated At**: 2026-03-07
- **Status**: [ ] pending

### Task 3.3: Write tests for silent fallback fixes

- **Files**: `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**: Write tests verifying that when `backend.enabled=true` but config is broken, the relevant callsites (sync_after_mutation, forward_events_if_backend, materialize_backend_artifacts) emit visible warnings rather than silently falling back. Use stderr capture or similar test infrastructure.
- **Verify**: `cargo test -p ito-cli --features backend`
- **Done When**: Silent fallback regression tests exist for all fixed callsites
- **Updated At**: 2026-03-07
- **Status**: [ ] pending
