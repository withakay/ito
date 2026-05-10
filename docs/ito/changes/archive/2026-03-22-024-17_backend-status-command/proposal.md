## Why

When `backend.enabled=true`, Ito must be able to validate that the backend configuration is complete, the token is valid, and the server is reachable. Today there is no way to do this proactively — backend issues are only discovered reactively when operations fail. Worse, five callsites in the CLI silently swallow backend failures and fall back to local storage, making users believe they're syncing when they're not. There is also no client-side way to generate a project-scoped token from a seed, forcing manual HMAC computation.

## What Changes

- **`ito backend` subcommand group**: Introduce a top-level `ito backend` namespace for all backend client operations
- **`ito backend status`**: Validates config completeness, pings `/api/v1/health` and `/api/v1/ready`, and verifies token validity against a new server-side auth verify endpoint. Supports `--json`.
- **`ito backend generate-token`**: Derives a project-scoped token from a seed. Interactive prompts for org/repo if not configured. Accepts `--seed`, `--org`, `--repo` flags (env vars override flags, flags override config). Optionally writes resolved org/repo back to project config.
- **Auth verify endpoint**: Add `GET /api/v1/projects/{org}/{repo}/auth/verify` to the backend server. Returns token scope info on 200, or 401 if invalid. Used by `ito backend status` to confirm end-to-end auth.
- **Strict backend validation**: When `backend.enabled=true`, all callsites that currently silently fall back to local storage SHALL emit visible errors or warnings. If the configuration is incomplete (missing token, org, or repo), this is a hard error — not a silent degradation.
- **Token security warnings**: Warn users if `backend.token` is set directly in `.ito/config.json` (which is not gitignored) rather than via env var or `.ito/config.local.json`. Emphasize env var usage in help text and error messages.
- **Core health-check function**: Reusable function in `ito-core` for health, readiness, and auth verification.

## Capabilities

### New Capabilities

- `backend-status-check`: Client-side validation of backend configuration, connectivity, and auth. Includes the `ito backend status` and `ito backend generate-token` commands, the core health-check client, and the server-side auth verify endpoint.

### Modified Capabilities

- `backend-client-runtime`: Tighten validation — when `backend.enabled=true`, incomplete config (missing token, org, repo) MUST be a hard error in all callsites, not a silent fallback.
- `backend-event-forwarding`: Silent swallowing of backend errors in `forward_events_if_backend()` MUST be replaced with visible warnings.

## Impact

- Affected code: `ito-cli` (new `backend` subcommand group, fix 5 silent-fallback callsites), `ito-core` (health-check client, stricter validation), `ito-backend` (new auth verify endpoint)
- Affected specs: New `backend-status-check`, modified `backend-client-runtime`, modified `backend-event-forwarding`
- **Behavioral change**: Commands that previously silently fell back to local mode when backend config was broken will now emit warnings or errors. This is intentional — silent degradation was hiding real problems.
- No data model or storage changes
