## Context

Ito's backend client currently has no proactive connectivity or auth check. The `BackendRuntime` struct resolves configuration and the server exposes `/api/v1/health` and `/api/v1/ready`, but nothing ties these together for the user. Worse, five callsites silently swallow backend failures and fall back to local storage when `backend.enabled=true` but the config is broken or the server is unreachable. There is also no client-side way to generate a project-scoped token from the HMAC seed, forcing manual computation.

## Goals / Non-Goals

**Goals:**

- `ito backend status` — validate config, connectivity, and auth in one command
- `ito backend generate-token` — derive project-scoped tokens from a seed
- Auth verify endpoint — let the server confirm "yes, this token is valid for this project"
- Strict validation — when `backend.enabled=true`, incomplete config is a hard error everywhere
- Visible warnings — replace all silent backend fallbacks with user-facing warnings
- Token security guidance — warn about committing secrets, recommend env vars

**Non-Goals:**

- Local cache / offline sync (future work)
- Implementing `ito backend import/export` (separate changes)
- Continuous health monitoring or watch mode
- Changing the token derivation algorithm (HMAC-SHA256 is fine)

## Decisions

### Health-check + auth verify client lives in `ito-core`

A new `ito-core::backend_health` module will expose `check_backend_health(runtime: &BackendRuntime) -> BackendHealthStatus`. This struct will contain health, readiness, auth verification results, and any errors. The CLI handler stays thin — parse args, call core, format output.

**Alternative**: Inline HTTP calls in CLI handler. Rejected — violates the layered architecture convention.

### Use `ureq` for HTTP calls

The existing `HttpEventIngestClient` already uses `ureq`. The health-check and auth verify calls will use the same blocking HTTP approach. No async runtime is needed for these synchronous operations.

### `ito backend` subcommand group with `BackendAction` enum

Introduce `Backend` as a top-level clap subcommand. `BackendAction` enum starts with `Status` and `GenerateToken`, with room for `Import`, `Export`, etc.

### `ito backend generate-token` is interactive when needed

Resolution order for org/repo: **env var > flag > config > interactive prompt**. This matches the general principle that env vars are the strongest override (for CI/automation), flags are for one-off use, and config is the default. When neither is available, the command interactively prompts and offers to save the values to project config.

The `--seed` flag follows the same pattern: env var `ITO_BACKEND_TOKEN_SEED` > `--seed` flag > global config `backendServer.auth.tokenSeed`.

Output is the 64-character hex token (HMAC-SHA256), printed to stdout so it can be piped or captured. Guidance is printed to stderr.

### Auth verify endpoint: `GET /api/v1/projects/{org}/{repo}/auth/verify`

This goes through the existing auth middleware (so no special auth logic needed), then returns a minimal JSON payload with the token scope. This keeps it simple — the middleware already does the hard work of token validation.

Response for valid admin token: `{"valid": true, "scope": "admin"}`
Response for valid project token: `{"valid": true, "scope": "project", "org": "...", "repo": "..."}`
Invalid tokens get 401 from the middleware before reaching the handler.

**Alternative**: `POST /api/v1/auth/verify` with credentials in the body. Rejected — would require a separate auth flow outside the middleware. The project-scoped path naturally leverages existing infrastructure.

### Exit codes for `ito backend status`

- Exit 0: backend disabled (informational), or backend enabled + config valid + healthy + ready + auth verified
- Exit non-zero: any validation failure (config incomplete, unreachable, unhealthy, not ready, auth failed)

This lets scripts use `ito backend status && ito tasks sync push ...` as a gate.

### Strict validation: fix all silent fallback sites

Five callsites currently swallow backend errors:

| Location | Current behavior | New behavior |
|---|---|---|
| `tasks.rs` `sync_after_mutation` | `Err(_) => return` (silent) | `eprintln!` warning with error detail |
| `util.rs` `forward_events_if_backend` | `resolve_backend_runtime` `Err` → silent return | `eprintln!` warning |
| `util.rs` `forward_events_if_backend` | config parse error → `tracing::warn` only | `eprintln!` warning |
| `grep.rs` `materialize_backend_artifacts` | `resolve_backend_runtime` `Err` → silent return | `eprintln!` warning |
| `backend_change_repository.rs` | `list_changes()` `Err` → `NotFound` | `tracing::warn!` (domain layer, no stderr access) |

The key distinction: **config errors** (missing token/org/repo) should be loud warnings. **Runtime errors** (server down) should also be warnings but the command should still complete (best-effort sync is still useful — you just need to know it failed).

### Token security warnings

When `ito backend status` detects that `backend.token` is set in a file that is NOT gitignored (specifically `.ito/config.json`), it emits a warning recommending env var or `.ito/config.local.json`. This check is heuristic — we check if the resolved config source is a file path that matches `.ito/config.json` and is not in `.gitignore`. If we can't determine the source, we skip the warning.

## Risks / Trade-offs

- **Network timeout on unreachable server**: Use a short timeout (5s) for health/ready/auth-verify, independent of the configured `timeout_ms` for data operations, so `ito backend status` stays responsive.
- **Feature gating**: The `ito backend` CLI subcommand group is gated on `#[cfg(feature = "backend")]`, consistent with `serve-api`.
- **Breaking best-effort behavior**: Making backend failures visible (warnings instead of silent) may surface issues users didn't know they had. This is intentional — silent degradation was hiding real problems. The warnings don't block commands, just inform.
- **Interactive prompts in generate-token**: May not work well in CI. The `--org`/`--repo`/`--seed` flags and env vars provide non-interactive alternatives.
