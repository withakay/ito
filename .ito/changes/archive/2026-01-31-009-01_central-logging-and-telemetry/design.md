## Context

Ito execution can fail in ways that are hard to debug from a single CLI output. Additionally, without instrumentation it is difficult to know which CLI entrypoints are used in practice (to prioritize improvements and identify dead code paths). We want a local-only logging and telemetry foundation that is useful for debugging while being privacy-preserving by default.

## Goals / Non-Goals

**Goals:**

- Central, per-user logging location with structured (machine-readable) events.
- Group logs by project without storing raw absolute paths.
- Group logs by session to correlate a sequence of commands.
- Provide a `ito stats` command that summarizes usage locally and can include unused commands.
- Best-effort logging: logging failures must not break command execution.

**Non-Goals:**

- Network telemetry/reporting.
- Capturing full CLI arguments or environment by default.
- Windows-specific log directory conventions (can be added later).

## Decisions

### Decision: Structured JSONL event logs

Write one JSON object per line (JSONL) for append-friendly logging, easy parsing, and robust partial writes.

### Decision: Central log directory

Use Ito's per-user config directory and add `logs/` beneath it.

- Linux (XDG): `~/.config/ito/logs`
- macOS: use the platform config dir (documented), with a stable `ito/logs` subdirectory

### Decision: Privacy-preserving project identifier

Derive `project_id` as a salted hash of the canonical project root path:

- Salt is generated once and stored in the per-user config dir (e.g. `telemetry_salt`).
- Hash uses a stable algorithm (e.g. SHA-256) and is encoded (hex/base32).
- Logs store only `project_id`, not the raw path.

This avoids embedding the full path while still allowing grouping.

### Decision: Session identity and persistence

Create `session_id` at the start of a project session and persist it in the project's `.ito/` directory (e.g. `.ito/session.json`).

- Session id is time-based (start timestamp) plus randomness for uniqueness.
- If `.ito/` is not present, use a process-scoped session id.

### Decision: CLI entrypoint auditing

Each CLI entrypoint is assigned a stable `command_id` string (e.g. `ito.init`, `ito.proposal.create`). Execution events record `command_id` and outcome.

`ito stats` enumerates the known `command_id` list (from the CLI definition) so it can show both used and unused commands.

### Decision: `command_id` format and known ids

Treat `command_id` as an API.

- Format: `ito.<segment>(.<segment>...)?`
- Allowed characters per segment: `a-z0-9_` (hyphens are normalized to `_`).
- Segments are derived from the CLI tokens: top-level command + any subcommand tokens.

Known ids (from `ito-rs/crates/ito-cli/src/main.rs`):

- `ito.create.module`
- `ito.create.change`
- `ito.new.change`
- `ito.init`
- `ito.update`
- `ito.list`
- `ito.plan.init`
- `ito.plan.status`
- `ito.state.show`
- `ito.state.decision`
- `ito.state.blocker`
- `ito.state.note`
- `ito.state.focus`
- `ito.state.question`
- `ito.tasks.init`
- `ito.tasks.status`
- `ito.tasks.next`
- `ito.tasks.start`
- `ito.tasks.complete`
- `ito.tasks.shelve`
- `ito.tasks.unshelve`
- `ito.tasks.add`
- `ito.tasks.show`
- `ito.workflow.init`
- `ito.workflow.list`
- `ito.workflow.show`
- `ito.status`
- `ito.templates`
- `ito.instructions`
- `ito.agent`
- `ito.x_instructions`
- `ito.show`
- `ito.validate`
- `ito.ralph`
- `ito.loop`

Notes:

- `ito templates` and `ito x-templates` are aliases today; both map to `ito.templates`.
- `ito loop` is a deprecated alias for `ito ralph`; it still has its own `command_id` so we can measure deprecation usage.

### Decision: Execution event schema (v1)

Execution logs use JSONL: one JSON object per line.

Each command emits two events:

- `command_start` at the beginning of execution.
- `command_end` at the end of execution.

Common fields:

- `event_version`: integer schema version (start with `1`).
- `event_id`: unique identifier (UUIDv4).
- `timestamp`: RFC 3339 UTC timestamp of when the event was recorded.
- `event_type`: `command_start` | `command_end`.
- `ito_version`: CLI version string.
- `command_id`: stable id (see above).
- `session_id`: stable within a project session.
- `project_id`: salted hash of the project root.
- `pid`: process id.

End-event fields:

- `outcome`: `success` | `error`.
- `duration_ms`: integer milliseconds from command start to end.

Non-goals for v1:

- Logging full argv, raw absolute paths, or environment variables.

Example (end event):

```json
{"event_version":1,"event_id":"b5400d1a-6c4c-4e6d-ae78-7f8f22a8a0dd","timestamp":"2026-01-31T17:14:02Z","event_type":"command_end","ito_version":"0.0.0","command_id":"ito.tasks.status","session_id":"01JH...","project_id":"c6a8...","pid":12345,"outcome":"success","duration_ms":42}
```

### Decision: On-disk layout and file naming

Log root: Ito per-user config directory, with a `logs/` child.

- Root: `<config_dir>/ito/logs/`
- Schema/versioning: `<config_dir>/ito/logs/execution/v1/`
- Grouping: per-project, per-session file

Layout:

- `<config_dir>/ito/logs/execution/v1/projects/<project_id>/sessions/<session_id>.jsonl`

File naming rules:

- `<project_id>`: lowercase hex string.
- `<session_id>`: opaque, url-safe string.
- Files are append-only; a partial final line is permitted and should be ignored by readers.

### Decision: Project hashing and salt storage

`project_id` is computed from the canonical project root path using a per-user random salt.

- Salt file: `<config_dir>/ito/telemetry_salt` (32 random bytes, created on first use)
- Hash: `sha256(salt || 0x00 || canonical_project_root_utf8)` encoded as lowercase hex
- The raw project path MUST NOT be written to logs by default

## Risks / Trade-offs

- Local data growth: logs can grow unbounded if unmanaged.
  - Mitigation: retention policy (time- or size-based) and/or user-invoked cleanup.
- Privacy: even hashed paths can leak information in limited cases.
  - Mitigation: use a per-user salt; do not log args/paths by default.
- Behavioral drift: keeping `command_id` stable over time requires discipline.
  - Mitigation: treat `command_id` as an API and add CI checks/tests.

## Migration Plan

1. Add `ito-logging` crate with event schema and file writing.
1. Integrate logging into `ito-rs` entrypoints and ensure failures are best-effort.
1. Add session/project id logic and state storage under `.ito/`.
1. Add `ito stats` and document usage.
1. Add basic retention/cleanup behavior and tests.

## Open Questions

- Should retention be purely time-based, size-based, or both?
- Should `ito stats` be a stable public command or namespaced (e.g. `ito debug stats`)?
