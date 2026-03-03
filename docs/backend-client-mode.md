# Backend Client Mode

Backend client mode enables multiple agents to coordinate through a shared backend API instead of relying solely on filesystem and git synchronization. When enabled, agents can claim changes, synchronize artifacts, and avoid conflicting edits.

## Prerequisites

- A running Ito backend server (see below for local runtime options)
- A valid bearer token (set via environment variable or config)

### Running the Backend Locally

Ito provides several options for running the backend locally:

| Runtime | Platform | Best For |
|---------|----------|----------|
| Docker Compose | macOS, Linux | Containerized testing, CI |
| Homebrew service | macOS | Long-running development |
| systemd service | Linux | Long-running development, self-hosted |

#### Docker Compose (All Platforms)

```bash
# Start the backend
docker compose -f docker-compose.backend.yml up -d

# Verify health
curl http://127.0.0.1:9010/api/v1/health

# Stop the backend
docker compose -f docker-compose.backend.yml down
```

See `docker-compose.backend.yml` and `.env.backend.example` for configuration.

#### Homebrew Service (macOS)

For long-running development on macOS, you can run the backend as a Homebrew-managed service:

```bash
# Install the plist
mkdir -p ~/Library/LaunchAgents
cp services/com.withakay.ito.backend.plist ~/Library/LaunchAgents/

# Edit the plist to set your admin token
# Replace the empty string after <key>ITO_BACKEND_ADMIN_TOKEN</key> with your token
$EDITOR ~/Library/LaunchAgents/com.withakay.ito.backend.plist

# Load and start the service
launchctl load ~/Library/LaunchAgents/com.withakay.ito.backend.plist

# Verify the backend is running
curl http://127.0.0.1:9010/api/v1/health
```

Service management commands:

```bash
# Check service status
launchctl list | grep ito.backend

# Stop the service
launchctl unload ~/Library/LaunchAgents/com.withakay.ito.backend.plist

# View logs
tail -f /tmp/ito-backend.log
```

#### systemd Service (Linux)

For Linux systems with systemd, you can run the backend as a user or system service:

**User service (recommended for development):**

```bash
# Install the unit file
mkdir -p ~/.config/systemd/user/
cp services/ito-backend.service ~/.config/systemd/user/

# Edit to set your admin token
$EDITOR ~/.config/systemd/user/ito-backend.service

# Enable and start
systemctl --user daemon-reload
systemctl --user enable --now ito-backend

# Verify health
curl http://127.0.0.1:9010/api/v1/health
```

**System service (for shared/self-hosted deployments):**

```bash
# Install as root
sudo cp services/ito-backend.service /etc/systemd/system/
sudo $EDITOR /etc/systemd/system/ito-backend.service  # Set tokens
sudo systemctl daemon-reload
sudo systemctl enable --now ito-backend
```

Service management commands:

```bash
# Check status
systemctl --user status ito-backend

# View logs
journalctl --user -u ito-backend -f

# Stop the service
systemctl --user stop ito-backend
```

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
