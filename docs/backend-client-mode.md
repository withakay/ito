# Backend Client Mode

Backend client mode enables multiple agents to coordinate through a shared backend API instead of relying solely on filesystem and git synchronization. When enabled, agents can claim changes, synchronize artifacts, and avoid conflicting edits.

## Prerequisites

- A running Ito backend server (see below for local Docker Compose setup)
- A valid bearer token (set via environment variable or config)

### Running the Backend Locally (Docker Compose)

For local development and testing, you can run the backend using Docker Compose:

```bash
# Copy the example environment file
cp .env.backend.example .env.backend

# Edit the token values (required for auth)
# At minimum, set ITO_BACKEND_ADMIN_TOKEN to a secure random value
$EDITOR .env.backend

# Start the backend
docker compose -f docker-compose.backend.yml up -d

# Verify the backend is healthy
curl http://127.0.0.1:9010/api/v1/health
# Expected: {"status":"ok"}

# Or use the helper script
./scripts/backend-health.sh
```

To stop the backend:

```bash
docker compose -f docker-compose.backend.yml down
```

The Docker Compose setup:
- Builds the `ito` binary from source with the `backend` feature
- Exposes the API on `http://127.0.0.1:9010` (configurable via `ITO_BACKEND_PORT`)
- Persists data in a Docker volume (`ito-backend-data`)
- Includes a container healthcheck for orchestration

**Configuration:**

| Variable | Default | Description |
|----------|---------|-------------|
| `ITO_BACKEND_PORT` | `9010` | Port exposed on the host |
| `ITO_BACKEND_ADMIN_TOKEN` | `dev-admin-token` | Admin bearer token (full access) |
| `ITO_BACKEND_TOKEN_SEED` | `dev-token-seed` | Seed for deriving per-project tokens |

**Scope note:** This Docker Compose runtime is intended for local testing and development only. For production or long-running self-hosted deployments, see the Homebrew and systemd service options (change `024-13`).

## Enabling Backend Mode

Add the following to your project or global config:

```json
{
  "backend": {
    "enabled": true,
    "url": "https://your-backend.example.com"
  }
}
```

Set your token via environment variable (default: `ITO_BACKEND_TOKEN`):

```bash
export ITO_BACKEND_TOKEN="your-token-here"
```

Or set it directly in config (less recommended for security):

```json
{
  "backend": {
    "enabled": true,
    "url": "https://your-backend.example.com",
    "token": "your-token-here"
  }
}
```

## Commands

All backend coordination commands live under `ito tasks`.

### Claim a change

Acquire an exclusive lease on a change so other agents know you are working on it:

```bash
ito tasks claim <change-id>
```

On success, prints the holder identity and lease timestamp. If another agent already holds the lease, returns a conflict error with the current holder's identity.

### Release a change

Release your lease when you are done or need to hand off:

```bash
ito tasks release <change-id>
```

### Allocate work

Ask the backend to assign the next available change based on priority and current leases:

```bash
ito tasks allocate
```

Returns the allocated change ID and claims it in one atomic operation. If no work is available, prints a message indicating the queue is empty.

### Sync artifacts

Pull the latest artifacts from the backend to your local workspace:

```bash
ito tasks sync pull <change-id>
```

Push your local changes to the backend:

```bash
ito tasks sync push <change-id>
```

## Automatic Sync After Mutations

When backend mode is enabled, task mutation commands (`start`, `complete`, `shelve`, `unshelve`, `add`) automatically attempt a best-effort push to the backend after the local operation succeeds. If the push fails (network issue, backend unavailable), the local operation still succeeds and a warning is printed.

## Conflict Handling

### Lease conflicts

If you try to claim a change that another agent holds:

```text
Error: Change "024-02_add-cli-backend-client" is currently claimed by agent "agent-7b3f"
  (claimed at 2026-02-28T14:30:00Z)
  Hint: Ask the holder to release it, or wait for the lease to expire.
```

### Stale revision conflicts

If you try to push artifacts but your local revision is behind the backend:

```text
Error: Push rejected — your local revision (rev-5) is behind the backend (rev-7).
  Hint: Run `ito tasks sync pull <change-id>` to fetch the latest, then retry your push.
```

The push is rejected safely; no data is lost. Pull first, resolve any local conflicts, then push again.

## Backup Snapshots

Before overwriting local artifacts during a pull, Ito creates a timestamped backup snapshot in the configured backup directory (default: `~/.ito/backups`). This ensures you can recover previous local state if needed.

Backup files are named with the change ID and timestamp:

```text
~/.ito/backups/<change-id>-<timestamp>/
```

## Failure Recovery

### Backend unavailable

If the backend is unreachable, all backend commands fail with a clear error. Local filesystem operations continue to work normally. You can:

1. Check your network connectivity
2. Verify the backend URL in your config
3. Disable backend mode temporarily: `ito config set backend.enabled false`
4. Continue working in filesystem-only mode

### Token issues

If authentication fails:

1. Verify the token environment variable is set: `echo $ITO_BACKEND_TOKEN`
2. Check the `token_env_var` config value matches your variable name
3. Ensure the token is valid and not expired

### Retry behavior

Transient failures (HTTP 429, 502, 503, 504) are automatically retried up to `max_retries` times. Each retry uses an idempotency key to prevent duplicate operations.

Non-retriable failures (4xx client errors other than 429) fail immediately with a diagnostic message.

## Configuration Reference

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `backend.enabled` | bool | `false` | Enable backend API integration |
| `backend.url` | string | `http://127.0.0.1:9010` | Base URL for the backend API |
| `backend.token` | string | (none) | Explicit bearer token |
| `backend.token_env_var` | string | `ITO_BACKEND_TOKEN` | Env var holding the bearer token |
| `backend.backup_dir` | string | `~/.ito/backups` | Directory for backup snapshots |
| `backend.timeout_ms` | u64 | `30000` | Request timeout in milliseconds |
| `backend.max_retries` | u32 | `3` | Max retry attempts for transient failures |

## Filesystem vs Backend Mode

| Aspect | Filesystem mode | Backend mode |
|--------|----------------|--------------|
| Change ownership | Implicit (git branch) | Explicit lease (claim/release) |
| Artifact sync | Git push/pull | `ito tasks sync push/pull` |
| Conflict detection | Git merge conflicts | Revision-based optimistic concurrency |
| Multi-agent safety | Manual coordination | Automatic via leases |
| Offline support | Full | Degrades to filesystem mode |

## Current Limitations

- Backend endpoints for lease and sync operations are defined by `024-01_add-shared-state-api` and may not be fully deployed yet. Until then, commands return a "not yet available" stub error.
- Real-time push notifications (websocket) are not supported; agents must poll or use explicit sync commands.
- The allocation algorithm is server-side; client-side priority hints are not yet supported.
