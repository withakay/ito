## Why

Ito lacks a centralized, session-oriented execution log that makes debugging failures and understanding real-world usage straightforward. Adding structured local logs enables better diagnostics today and unlocks privacy-preserving local telemetry (command usage) without requiring any network reporting.

## What Changes

- Add a Rust logging crate used by `ito-rs` entrypoints to record structured execution events to a central per-user log directory.
- Introduce a privacy-preserving project identifier for grouping logs by project without recording the full working directory path.
- Add session-based grouping so logs can be correlated across multiple commands within a project session.
- Add a local-only stats/metrics command to summarize command usage (which commands are used and which are not) from the recorded logs.
- Document how to locate logs, interpret them, and disable logging/telemetry.

## Capabilities

### New Capabilities

- `execution-logs`: Structured, centralized, session-oriented local logging for Ito command execution.
- `ito-stats`: Local-only usage metrics and auditing derived from execution logs.

### Modified Capabilities

<!-- None -->

## Impact

- `ito-rs/`: add a new crate and integrate it into CLI entrypoints.
- Data on disk: new per-user log directory under Ito's config location and a small per-project session state file under `.ito/`.
- CLI surface: add a `ito stats` (or similarly named) command for viewing local usage metrics.
- Documentation: add guidance for debugging via logs and for opting out.
