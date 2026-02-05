# Local telemetry (execution logs)

Ito records **local-only** execution logs to help understand command usage and support
`ito stats`.

## What gets logged

Each command emits structured JSONL events:

- `command_start`
- `command_end`

Fields include (v1):

- `timestamp` (UTC)
- `command_id` (stable id like `ito.tasks.status`)
- `session_id` (stable for a project session)
- `project_id` (salted hash; does not include raw paths)
- `outcome` (`success` | `error`) and `duration_ms` on end events

Non-goals for v1:

- No raw argv
- No absolute paths
- No environment variables

## Where logs are stored

Logs are written under the per-user Ito config directory:

- `<config_dir>/ito/logs/execution/v1/projects/<project_id>/sessions/<session_id>.jsonl`

On macOS and Linux (default):

- `<config_dir>` is `~/.config`

On Windows (default):

- `<config_dir>` is `%APPDATA%`

## How `project_id` is derived

`project_id` is computed as a salted hash of the canonical project root path.

- Salt file: `<config_dir>/ito/telemetry_salt` (32 random bytes)
- Hash: `sha256(salt || 0x00 || canonical_project_root_utf8)` encoded as lowercase hex

The raw project path is not written to logs by default.

## How `session_id` is derived

For projects with a `.ito/` directory, a stable session id is persisted at:

- `.ito/session.json`

## Opt-out

Set `ITO_DISABLE_LOGGING=1` to disable writing execution logs.
