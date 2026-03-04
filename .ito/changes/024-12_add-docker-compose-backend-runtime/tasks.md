# Tasks for: 024-12_add-docker-compose-backend-runtime

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status changes
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-12_add-docker-compose-backend-runtime
ito tasks next 024-12_add-docker-compose-backend-runtime
ito tasks start 024-12_add-docker-compose-backend-runtime 1.1
ito tasks complete 024-12_add-docker-compose-backend-runtime 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add Docker Compose backend runtime assets

- **Files**: `docker-compose.backend.yml`, `.env.example` (or backend runtime env sample), `ito-rs/**` (if backend container build wiring is needed)
- **Dependencies**: None
- **Action**: Add a compose definition that starts the backend service with local-testing defaults, including container lifecycle settings and required environment variables.
- **Verify**: `docker compose -f docker-compose.backend.yml config`
- **Done When**: Compose config validates successfully and includes a runnable backend service definition.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

### Task 1.2: Add compose health-check workflow for backend readiness

- **Files**: `docker-compose.backend.yml`, backend runtime docs (path to be finalized during implementation)
- **Dependencies**: Task 1.1
- **Action**: Define and document the health verification path used after compose startup (container healthcheck and/or host health endpoint command).
- **Verify**: `docker compose -f docker-compose.backend.yml up -d && curl -fsS http://127.0.0.1:9010/api/v1/health && docker compose -f docker-compose.backend.yml down`
- **Done When**: Developers can run a deterministic command sequence to confirm backend readiness and tear down cleanly.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Document local backend runtime usage and scope

- **Files**: `docs/backend-client-mode.md` (or equivalent backend docs), `README.md` (if linking entrypoint)
- **Dependencies**: None
- **Action**: Document compose startup/shutdown usage, required env inputs, health verification, and explicit non-goals (Homebrew/systemd deferred).
- **Verify**: `rg -n "docker compose|Homebrew|systemd|health" docs README.md`
- **Done When**: Runtime documentation is discoverable and clearly states current scope and follow-up areas.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

### Task 2.2: Validate proposal artifacts and task plan integrity

- **Files**: `.ito/changes/024-12_add-docker-compose-backend-runtime/**`
- **Dependencies**: Task 2.1
- **Action**: Run strict Ito validation and fix any proposal/spec/tasks formatting issues.
- **Verify**: `ito validate 024-12_add-docker-compose-backend-runtime --strict`
- **Done When**: Change validates cleanly in strict mode with no schema or delta errors.
- **Updated At**: 2026-03-02
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Proposal Review

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Review proposal, design, and spec deltas with stakeholders before implementation starts.
- **Done When**: Change proposal is approved for implementation.
- **Updated At**: 2026-03-02
- **Status**: [ ] pending
