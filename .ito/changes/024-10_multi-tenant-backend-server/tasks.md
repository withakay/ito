# Tasks for: 024-10_multi-tenant-backend-server

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-10_multi-tenant-backend-server
ito tasks next 024-10_multi-tenant-backend-server
ito tasks start 024-10_multi-tenant-backend-server 1.1
ito tasks complete 024-10_multi-tenant-backend-server 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add YAML front matter parsing/writing utilities

- **Files**: `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-domain/`
- **Dependencies**: None
- **Action**:
  - Implement front matter detection (`---` / `---` at file start)
  - Parse YAML into a typed metadata struct (or safe map)
  - Preserve exact markdown body
  - Implement write/update helpers that update `updated_at` and set `created_at` on first write
- **Verify**: `cd ito-rs && cargo test -p ito-core front_matter`
- **Done When**:
  - Unit tests cover: no front matter, valid front matter, invalid front matter, roundtrip
  - No behavior change for artifacts without front matter
- **Updated At**: 2026-03-01
- **Status**: [x] complete

### Task 1.2: Apply front matter support to module and change filesystem repositories

- **Files**: `ito-rs/crates/ito-core/src/module_repository.rs`, `ito-rs/crates/ito-core/src/change_repository.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - Load metadata from front matter if present
  - Validate optional `change_id`/`module_id` integrity fields if present
  - Prefer front matter timestamps for created/updated where available
- **Verify**: `cd ito-rs && cargo test -p ito-core module_repository change_repository`
- **Done When**:
  - Repositories accept existing markdown files unchanged
  - Metadata validation errors are well-formed and tested
- **Updated At**: 2026-03-01
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement project-scoped routing for backend state API

- **Files**: `ito-rs/crates/ito-backend/src/api.rs`, `ito-rs/crates/ito-backend/src/server.rs`
- **Dependencies**: None
- **Action**:
  - Replace single-project routes with `/api/v1/projects/{org}/{repo}/...`
  - Update handlers to accept `{org, repo}` and resolve project storage before constructing repositories
- **Verify**: `cd ito-rs && cargo test -p ito-backend`
- **Done When**:
  - Old single-project routes are removed
  - New routes cover: changes list/get/tasks, modules list/get, events ingest
- **Updated At**: 2026-03-01
- **Status**: [>] in-progress

### Task 2.2: Implement admin + derived project token authentication

- **Files**: `ito-rs/crates/ito-backend/src/auth.rs`
- **Dependencies**: Task 2.1
- **Action**:
  - Add config-driven admin token(s)
  - Add derived token validation: `HMAC-SHA256(seed, "{org}/{repo}")`
  - Keep `/health` and `/ready` unauthenticated
- **Verify**: `cd ito-rs && cargo test -p ito-backend auth`
- **Done When**:
  - Tests cover: admin token, project token match, mismatch forbidden
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 2.3: Implement org/repo allowlist enforcement

- **Files**: `ito-rs/crates/ito-backend/src/`
- **Dependencies**: Task 2.1
- **Action**:
  - Add allowlist checks for org and repo before serving project routes
- **Verify**: `cd ito-rs && cargo test -p ito-backend allowlist`
- **Done When**:
  - Disallowed org/repo requests are rejected
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add domain port for backend project store resolution

- **Files**: `ito-rs/crates/ito-domain/src/`, `ito-rs/crates/ito-core/src/`
- **Dependencies**: None
- **Action**:
  - Define a domain-level abstraction for resolving `{org, repo}` to project storage
  - Keep the interface small and backend-oriented
- **Verify**: `cd ito-rs && cargo test -p ito-domain`
- **Done When**:
  - `ito-backend` can be wired without directly depending on filesystem paths
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 3.2: Implement filesystem project store (default)

- **Files**: `ito-rs/crates/ito-core/src/`
- **Dependencies**: Task 3.1
- **Action**:
  - Resolve project `.ito` path under `<dataDir>/projects/{org}/{repo}/.ito`
  - Create missing directories on first write (and/or on first access per decision)
- **Verify**: `cd ito-rs && cargo test -p ito-core fs_project_store`
- **Done When**:
  - Backend can serve multiple projects from one instance using filesystem store
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 3.3: Implement SQLite project store proof-of-concept

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-core/Cargo.toml`
- **Dependencies**: Task 3.1
- **Action**:
  - Implement the same port using SQLite (schema for modules/changes/tasks/specs/audit as needed)
  - Ensure created/updated timestamps and revision metadata are stored
- **Verify**: `cd ito-rs && cargo test -p ito-core sqlite_project_store`
- **Done When**:
  - Basic parity tests pass for read/write scenarios
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 3.4: Wire backend server config to select store implementation

- **Files**: `ito-rs/crates/ito-backend/src/server.rs`, `ito-rs/crates/ito-cli/src/commands/serve_api.rs`, `ito-rs/crates/ito-config/src/config/types.rs`
- **Dependencies**: Task 3.2, Task 3.3
- **Action**:
  - Add `backendServer.*` config models + schema
  - Add env/args overrides for server settings
  - Add minimal HTTP server settings (max body size; optional CORS origins)
  - Select fs/sqlite store at runtime (including sqlite db path)
- **Verify**: `cd ito-rs && cargo test -p ito-config -p ito-cli -p ito-backend`
- **Done When**:
  - `ito serve-api` starts a multi-tenant server using configured store
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Update backend client event forwarding to use project-scoped event ingest endpoint

- **Files**: `ito-rs/crates/ito-cli/src/util.rs`, `ito-rs/crates/ito-core/src/backend_client.rs`, `ito-rs/crates/ito-config/src/config/types.rs`
- **Dependencies**: None
- **Action**:
  - Add backend client config keys `backend.project.org` and `backend.project.repo`
  - Add env var overrides `ITO_BACKEND_PROJECT_ORG` / `ITO_BACKEND_PROJECT_REPO`
  - Update HTTP event ingest URL to `POST /api/v1/projects/{org}/{repo}/events`
- **Verify**: `cd ito-rs && cargo test -p ito-cli`
- **Done When**:
  - Backend event forwarding works with the new multi-tenant backend routing
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 4.2: Add end-to-end tests for multi-tenant routing + auth

- **Files**: `ito-rs/crates/ito-backend/tests/`
- **Dependencies**: None
- **Action**:
  - Test two projects `{org}/{repo}` in one server instance
  - Test admin token vs derived token
  - Test allowlist enforcement
- **Verify**: `cd ito-rs && cargo test -p ito-backend`
- **Done When**:
  - Tests demonstrate serving 2+ projects without git checkouts
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 4.3: Update agent instructions + embedded skills/commands for backend mode

- **Files**:
  - `ito-rs/crates/ito-cli/src/app/instructions.rs`
  - `ito-rs/crates/ito-templates/assets/instructions/agent/bootstrap.md.j2`
  - `ito-rs/crates/ito-templates/assets/instructions/agent/` (new `backend.md.j2`)
  - `ito-rs/crates/ito-templates/assets/skills/ito-workflow/SKILL.md`
  - `ito-rs/crates/ito-templates/assets/commands/ito.md`
- **Dependencies**: None
- **Action**:
  - Add a new instruction artifact: `ito agent instruction backend`
  - Ensure it documents: org/repo routing, client config keys, server config keys, and token model
  - Update bootstrap output to reference `ito agent instruction backend`
  - Update relevant skills/commands to reference backend instructions as the source of truth
- **Verify**: `cd ito-rs && cargo test -p ito-cli -p ito-templates`
- **Done When**:
  - Agents can discover backend configuration/usage guidance via `ito agent instruction backend`
- **Updated At**: 2026-02-28
- **Status**: [ ] pending
